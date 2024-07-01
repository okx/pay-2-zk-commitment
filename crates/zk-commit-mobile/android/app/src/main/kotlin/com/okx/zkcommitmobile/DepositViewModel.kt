@file:OptIn(ExperimentalStdlibApi::class)

package com.okx.zkcommitmobile

import androidx.compose.runtime.mutableStateListOf
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.okx.zkcommitmobile.data.Deposit
import com.okx.zkcommitmobile.data.DepositWithCommitment
import com.okx.zkcommitmobile.data.EthSendTransaction
import com.okx.zkcommitmobile.data.GetAccountState
import com.okx.zkcommitmobile.network.ZkCommitService
import com.okx.zkcommitmobile.network.wrapGroth16
import com.okx.zkcommitmobile.uniffi.AmountSecretPairing
import com.okx.zkcommitmobile.uniffi.generateProofOfClaim
import com.okx.zkcommitmobile.uniffi.getClaimTokenCallData
import com.okx.zkcommitmobile.uniffi.setupCommitment
import com.walletconnect.web3.modal.client.Modal
import com.walletconnect.web3.modal.client.Web3Modal
import com.walletconnect.web3.modal.client.models.request.Request
import com.walletconnect.web3.modal.client.models.request.SentRequestResult
import java.io.File
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
import kotlin.time.measureTimedValue
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.withContext
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import timber.log.Timber

class DepositViewModel(
    private val walletConnectManager: WalletConnectManager,
    private val zkCommitService: ZkCommitService,
    private val json: Json,
    private val defaultDispatcher: CoroutineDispatcher
) : ViewModel() {
    companion object {
        private val Log = Timber.tag("DepositViewModel")
    }

    private val _deposits = mutableStateListOf<DepositWithCommitment>()
    val deposits: List<DepositWithCommitment> get() = _deposits

    val getAccountState get() = walletConnectManager.getAccountState

    private val _messages = Channel<String>(capacity = Channel.UNLIMITED)
    val messages get() = _messages.receiveAsFlow()

    fun createDeposit(deposit: Deposit) {
        viewModelScope.launch {
            val (commitmentTree, duration) = measureTimedValue {
                withContext(defaultDispatcher) {
                    setupCommitment(
                        deposit.distributions.map {
                            AmountSecretPairing(amount = it.amount, secret = it.secret)
                        }
                    )
                }
            }
            Log.i("Setup commitment duration=$duration")
            Log.i("Commitment Tree: depth=${commitmentTree.depth}")
            val commitmentRoot = commitmentTree.commitmentRoot
            Log.i("Commitment Tree: root=${commitmentRoot.toHexString()}")
            if (commitmentTree.commitmentTree.size < 10) {
                val treeInHex = commitmentTree.commitmentTree.map { it.toHexString() }
                Log.i("Commitment Tree: tree=$treeInHex")
            } else {
                Log.i("Commitment Tree: tree size=${commitmentTree.commitmentTree.size}")
            }
            _deposits += DepositWithCommitment(deposit, commitmentTree)
            _messages.send(
                "Deposit created: duration=$duration, root=${commitmentRoot.toHexString()}"
            )
        }
    }

    fun claim(id: String, index: Int, proofFile: File) {
        viewModelScope.launch {
            val from = (getAccountState as? GetAccountState.Loaded)?.account?.address
            if (from == null) {
                Log.e("Failed to claim: Account not found")
                _messages.send("Failed to claim: Account not found")
                return@launch
            }

            val depositIndex = _deposits.indexOfFirst { it.deposit.id == id }
            val (deposit, commitmentTree) = _deposits[depositIndex]
            withContext(defaultDispatcher) {
                runCatching {
                    measureTimedValue {
                        generateProofOfClaim(
                            amount = deposit.distributions[index].amount,
                            secret = deposit.distributions[index].secret,
                            index = index,
                            commitmentTree = commitmentTree,
                            path = proofFile.absolutePath
                        )
                    }
                }
            }.onSuccess { (_, duration) ->
                Log.i("Generate proof of claim duration=$duration")
                Log.i("Proof: proof size=${proofFile.length()}")
            }.onFailure {
                Log.e(it, "Failed to generate proof of claim")
                _messages.send("Failed to generate proof of claim: ${it.localizedMessage}")
                return@launch
            }.mapCatching {
                zkCommitService.wrapGroth16(proofFile)
            }.onFailure {
                Log.e(it, "Failed to wrap Groth16")
                _messages.send("Failed to wrap Groth16: ${it.localizedMessage}")
                return@launch
            }.mapCatching { pwi ->
                withContext(defaultDispatcher) { getClaimTokenCallData(pwi).toHexString() }
            }.onFailure {
                Log.e(it, "Failed to get claim token call data")
                _messages.send("Failed to get claim token call data: ${it.localizedMessage}")
                return@launch
            }.mapCatching { data ->
                withContext(defaultDispatcher) {
                    json.encodeToString(
                        listOf(
                            EthSendTransaction(
                                from = from,
                                // TODO: Smart Contract Address
                                to = "0xTODO",
                                data = if (data.startsWith("0x")) data else "0x$data"
                            )
                        )
                    )
                }
            }.onFailure {
                Log.e(it, "Failed to encode EthSendTransaction")
                _messages.send("Failed to encode EthSendTransaction: ${it.localizedMessage}")
                return@launch
            }.mapCatching { params ->
                val request = Request("eth_sendTransaction", params)
                val sentRequestResult =
                    request(request) as? SentRequestResult.WalletConnect ?: return@launch
                walletConnectManager.sessionRequestResponse.first {
                    it.result.id == sentRequestResult.requestId &&
                        it.topic == sentRequestResult.sessionTopic &&
                        it.method == request.method
                }
            }.onFailure {
                Log.e(it, "Failed to call eth_sendTransaction")
                _messages.send("Failed to call eth_sendTransaction: ${it.localizedMessage}")
                return@launch
            }.mapCatching { response ->
                when (val responseResult = response.result) {
                    is Modal.Model.JsonRpcResponse.JsonRpcResult -> {
                        Log.i("Transaction sent: result=${responseResult.result}")
                        _messages.send("Transaction sent: result=${responseResult.result}")
                        _deposits[depositIndex] = DepositWithCommitment(
                            deposit = deposit.withClaimed(index = index),
                            commitmentTree = commitmentTree
                        )
                    }

                    is Modal.Model.JsonRpcResponse.JsonRpcError -> {
                        Log.e(
                            "Failed to send transaction: code=${responseResult.code}, message=${responseResult.message}"
                        )
                        _messages.send(
                            "Failed to send transaction: code=${responseResult.code}, message=${responseResult.message}"
                        )
                    }
                }
            }
        }
    }

    private suspend fun request(request: Request): SentRequestResult =
        suspendCancellableCoroutine { cont ->
            Web3Modal.request(
                request = request,
                onSuccess = { result -> cont.resume(result) },
                onError = { cont.resumeWithException(it) }
            )
        }

    fun getAccount() = walletConnectManager.getAccount()

    fun disconnect() {
        Web3Modal.disconnect(
            onSuccess = {
                _messages.trySend("Disconnected")
                getAccount()
            },
            onError = { throwable: Throwable ->
                Log.e(throwable, "Failed to disconnect")
                _messages.trySend("Failed to disconnect: ${throwable.message}")
            }
        )
    }

    private fun Deposit.withClaimed(index: Int) = copy(
        distributions = distributions.mapIndexed { i, distribution ->
            if (i == index) {
                distribution.copy(claimed = true)
            } else {
                distribution
            }
        }
    )
}
