@file:OptIn(ExperimentalMaterial3Api::class)

package com.okx.zkcommitmobile.ui

import androidx.compose.animation.AnimatedContent
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowForward
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExtendedFloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.LargeTopAppBar
import androidx.compose.material3.ListItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.tooling.preview.Preview
import com.okx.zkcommitmobile.data.Deposit
import com.okx.zkcommitmobile.data.Distribution
import com.okx.zkcommitmobile.ui.theme.ZkCommitMobileTheme
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.emptyFlow
import kotlinx.serialization.Serializable

@Serializable
data object DepositScreen

@Composable
fun DepositScreen(
    deposits: List<Deposit>,
    messages: Flow<String>,
    modifier: Modifier = Modifier,
    onCreateDeposit: () -> Unit = {},
    onClaim: (deposit: Deposit) -> Unit = {}
) {
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(Unit) { messages.collect { message -> snackbarHostState.showSnackbar(message) } }
    val scrollBehavior = TopAppBarDefaults.exitUntilCollapsedScrollBehavior()
    Scaffold(
        modifier = modifier.nestedScroll(scrollBehavior.nestedScrollConnection),
        topBar = {
            LargeTopAppBar(
                title = { Text(text = "Deposit") },
                scrollBehavior = scrollBehavior
            )
        },
        snackbarHost = { SnackbarHost(hostState = snackbarHostState) },
        floatingActionButton = {
            ExtendedFloatingActionButton(
                text = { Text(text = "Create deposit") },
                icon = { Icon(Icons.Default.Add, contentDescription = null) },
                onClick = onCreateDeposit
            )
        }
    ) { innerPaddings ->
        AnimatedContent(deposits.isEmpty(), label = "deposits") { isEmpty ->
            if (isEmpty) {
                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(innerPaddings),
                    contentAlignment = Alignment.Center
                ) {
                    Text(text = "No deposits")
                }
            } else {
                LazyColumn(modifier = Modifier.fillMaxSize(), contentPadding = innerPaddings) {
                    items(deposits) { deposit ->
                        DepositItem(
                            deposit = deposit,
                            modifier = Modifier.animateItem(),
                            onClick = onClaim
                        )
                    }
                }
            }
        }
    }
}

@Composable
fun DepositItem(
    deposit: Deposit,
    modifier: Modifier = Modifier,
    onClick: (deposit: Deposit) -> Unit = {}
) {
    ListItem(
        headlineContent = { Text(text = "ID: ${deposit.id}") },
        modifier = modifier.clickable { onClick(deposit) },
        supportingContent = { Text(text = "Total amount: ${deposit.totalAmount}") },
        trailingContent = {
            Icon(Icons.AutoMirrored.Default.ArrowForward, contentDescription = null)
        }
    )
}

@Preview
@Composable
private fun DepositScreenPreview() {
    ZkCommitMobileTheme {
        DepositScreen(
            deposits = listOf(
                Deposit(
                    id = "1",
                    distributions = listOf(
                        Distribution(claimed = false, amount = 1UL, secret = 1UL)
                    )
                ),
                Deposit(
                    id = "2",
                    distributions = listOf(
                        Distribution(claimed = false, amount = 1UL, secret = 1UL),
                        Distribution(claimed = true, amount = 2UL, secret = 2UL)
                    )
                )
            ),
            messages = emptyFlow()
        )
    }
}
