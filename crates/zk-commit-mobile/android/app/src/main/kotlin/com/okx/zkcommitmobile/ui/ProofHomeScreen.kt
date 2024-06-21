@file:OptIn(ExperimentalComposeUiApi::class, ExperimentalMaterial3Api::class)

package com.okx.zkcommitmobile.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.testTag
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import androidx.compose.ui.unit.dp
import com.okx.zkcommitmobile.uniffi.ProofResult
import com.okx.zkcommitmobile.uniffi.fibonacci
import com.walletconnect.web3.modal.ui.components.internal.Web3ModalComponent
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import timber.log.Timber

private const val TAG = "ProofHomeScreen"

private sealed interface ProofState {
    data object Idle : ProofState

    data object Proving : ProofState

    data class Proved(val result: ProofResult) : ProofState
}

@Composable
fun ProofHomeScreen(modifier: Modifier = Modifier) {
    Scaffold(modifier = modifier.semantics { testTagsAsResourceId = true }) { innerPadding ->
        val coroutineScope = rememberCoroutineScope()
        val bottomSheetState =
            rememberModalBottomSheetState(skipPartiallyExpanded = true)
        var showBottomSheet by remember { mutableStateOf(false) }
        Column(
            modifier =
            Modifier
                .fillMaxSize()
                .padding(innerPadding),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            var proofState by remember { mutableStateOf<ProofState>(ProofState.Idle) }
            Button(
                onClick = {
                    coroutineScope.launch {
                        proofState = ProofState.Proving
                        val result = withContext(Dispatchers.Default) { fibonacci() }
                        Timber.tag(TAG).i("Fibonacci result: $result")
                        proofState = ProofState.Proved(result)
                    }
                },
                modifier = Modifier.testTag("calculateFibonacci")
            ) {
                Text("Calculate Fibonacci")
            }
            Button(onClick = { showBottomSheet = true }) {
                Text("Wallet Connect")
            }

            AnimatedContent(
                targetState = proofState,
                modifier = Modifier.padding(horizontal = 16.dp),
                label = "proofState",
                contentAlignment = Alignment.Center
            ) { state ->
                when (state) {
                    ProofState.Idle -> Unit
                    ProofState.Proving ->
                        CircularProgressIndicator(modifier = Modifier.size(48.dp))

                    is ProofState.Proved -> {
                        val result = state.result
                        Text(
                            text =
                            "100th Fibonacci number mod |F| " +
                                "(starting with ${result.firstInput}, " +
                                "${result.secondInput}) is: " +
                                "${result.output}, time taken: ${result.duration}",
                            modifier = Modifier.testTag("fibonacciResult")
                        )
                    }
                }
            }
        }

        if (showBottomSheet) {
            ModalBottomSheet(
                onDismissRequest = { showBottomSheet = false },
                sheetState = bottomSheetState,
                dragHandle = null
            ) {
                Web3ModalComponent(
                    shouldOpenChooseNetwork = false,
                    closeModal = {
                        coroutineScope
                            .launch { bottomSheetState.hide() }
                            .invokeOnCompletion {
                                if (!bottomSheetState.isVisible) {
                                    showBottomSheet = false
                                }
                            }
                    }
                )
            }
        }
    }
}
