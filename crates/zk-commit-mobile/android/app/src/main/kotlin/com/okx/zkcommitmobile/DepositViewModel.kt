@file:OptIn(ExperimentalStdlibApi::class)

package com.okx.zkcommitmobile

import androidx.compose.runtime.mutableStateListOf
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.okx.zkcommitmobile.data.Deposit
import com.okx.zkcommitmobile.data.DepositWithCommitment
import com.okx.zkcommitmobile.uniffi.AmountSecretPairing
import com.okx.zkcommitmobile.uniffi.generateProofOfClaim
import com.okx.zkcommitmobile.uniffi.setupCommitment
import kotlin.time.measureTimedValue
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import timber.log.Timber

class DepositViewModel : ViewModel() {
    companion object {
        private const val TAG = "DepositViewModel"
    }

    private val _deposits = mutableStateListOf<DepositWithCommitment>()
    val deposits: List<DepositWithCommitment> get() = _deposits

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
            val logger = Timber.tag(TAG)
            logger.i("Setup commitment duration=$duration")
            logger.i("Commitment Tree: depth=${commitmentTree.depth}")
            logger.i("Commitment Tree: root=${commitmentTree.commitmentRoot.toHexString()}")
            if (commitmentTree.commitmentTree.size < 10) {
                val treeInHex = commitmentTree.commitmentTree.map { it.toHexString() }
                logger.i("Commitment Tree: tree=$treeInHex")
            } else {
                logger.i("Commitment Tree: tree size=${commitmentTree.commitmentTree.size}")
            }
            _deposits += DepositWithCommitment(deposit, commitmentTree)
            _messages.send(
                "Deposit created: duration=$duration, root=${commitmentTree.commitmentRoot.toHexString()}"
            )
        }
    }

    fun claim(id: String, index: Int) {
        viewModelScope.launch {
            val depositIndex = _deposits.indexOfFirst { it.deposit.id == id }
            val (deposit, commitmentTree) = _deposits[depositIndex]
            val logger = Timber.tag(TAG)
            withContext(Dispatchers.Default) {
                runCatching {
                    measureTimedValue {
                        generateProofOfClaim(
                            amount = deposit.distributions[index].amount,
                            secret = deposit.distributions[index].secret,
                            index = index,
                            commitmentTree = commitmentTree
                        )
                    }
                }
            }.onSuccess { (proof, duration) ->
                logger.i("Generate proof of claim duration=$duration")
                logger.i("Proof: amount=${proof.amount}")
                logger.i("Proof: proof size=${proof.proof.size}")
                logger.i("Proof: publicInputs=${proof.publicInputs}")
                _deposits[depositIndex] = DepositWithCommitment(
                    deposit = deposit.copy(
                        distributions = deposit.distributions.mapIndexed { i, distribution ->
                            if (i == index) distribution.copy(claimed = true) else distribution
                        }
                    ),
                    commitmentTree = commitmentTree
                )
                _messages.send(
                    "Claimed: duration=$duration, amount=${proof.amount}, proof size=${proof.proof.size}"
                )
            }.onFailure {
                logger.e(it, "Failed to generate proof of claim")
                _messages.send("Failed to generate proof of claim: ${it.message}")
            }
        }
    }
}
