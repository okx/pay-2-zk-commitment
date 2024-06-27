@file:OptIn(ExperimentalStdlibApi::class)

package com.okx.zkcommitmobile

import androidx.compose.runtime.mutableStateListOf
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.okx.zkcommitmobile.data.Deposit
import com.okx.zkcommitmobile.data.DepositWithCommitment
import com.okx.zkcommitmobile.network.ZkCommitService
import com.okx.zkcommitmobile.network.wrapGroth16
import com.okx.zkcommitmobile.uniffi.AmountSecretPairing
import com.okx.zkcommitmobile.uniffi.generateProofOfClaim
import com.okx.zkcommitmobile.uniffi.getClaimTokenCallData
import com.okx.zkcommitmobile.uniffi.setupCommitment
import com.walletconnect.web3.modal.client.Web3Modal
import java.io.File
import kotlin.time.measureTimedValue
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import timber.log.Timber

class DepositViewModel(
    private val walletConnectManager: WalletConnectManager,
    private val zkCommitService: ZkCommitService
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
                withContext(Dispatchers.Default) {
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
            val depositIndex = _deposits.indexOfFirst { it.deposit.id == id }
            val (deposit, commitmentTree) = _deposits[depositIndex]
            withContext(Dispatchers.Default) {
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
            }.onSuccess { (proof, duration) ->
                Log.i("Generate proof of claim duration=$duration")
                Log.i("Proof: proof size=${proofFile.length()}")
            }.onFailure {
                Log.e(it, "Failed to generate proof of claim")
                _messages.send("Failed to generate proof of claim: ${it.localizedMessage}")
            }.mapCatching {
                zkCommitService.wrapGroth16(proofFile)
            }.onFailure {
                Log.e(it, "Failed to wrap Groth16")
                _messages.send("Failed to wrap Groth16: ${it.localizedMessage}")
            }.mapCatching { pwi ->
                withContext(Dispatchers.Default) { getClaimTokenCallData(pwi) }
            }.onSuccess {
                _deposits[depositIndex] = DepositWithCommitment(
                    deposit = deposit.copy(
                        distributions = deposit.distributions.mapIndexed { i, distribution ->
                            if (i == index) distribution.copy(claimed = true) else distribution
                        }
                    ),
                    commitmentTree = commitmentTree
                )
                val calldata = it.toHexString()
                Log.i("Claim token call data: $calldata")
                _messages.send("Claim token call data: $calldata")
            }.onFailure {
                Log.e(it, "Failed to get claim token call data")
                _messages.send("Failed to get claim token call data: ${it.localizedMessage}")
            }
        }
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
}
