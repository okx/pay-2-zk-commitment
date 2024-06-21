@file:OptIn(ExperimentalMaterial3Api::class)

package com.okx.zkcommitmobile.ui

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
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
data class ClaimListScreen(val id: String)

@Composable
fun ClaimListScreen(
    deposit: Deposit,
    messages: Flow<String>,
    modifier: Modifier = Modifier,
    onNavigateUp: () -> Unit = {},
    onClaim: (deposit: Deposit, index: Int, distribution: Distribution) -> Unit = { _, _, _ -> }
) {
    val snackbarHostState = remember { SnackbarHostState() }
    LaunchedEffect(Unit) { messages.collect { message -> snackbarHostState.showSnackbar(message) } }
    val scrollBehavior = TopAppBarDefaults.exitUntilCollapsedScrollBehavior()
    Scaffold(
        modifier = modifier.nestedScroll(scrollBehavior.nestedScrollConnection),
        topBar = {
            LargeTopAppBar(
                title = { Text(text = "Claim list") },
                navigationIcon = {
                    IconButton(onClick = onNavigateUp) {
                        Icon(Icons.AutoMirrored.Default.ArrowBack, contentDescription = null)
                    }
                },
                scrollBehavior = scrollBehavior
            )
        },
        snackbarHost = { SnackbarHost(hostState = snackbarHostState) }
    ) { innerPaddings ->
        LazyColumn(contentPadding = innerPaddings) {
            itemsIndexed(deposit.distributions) { index, distribution ->
                ListItem(
                    headlineContent = { Text(text = "Amount: ${distribution.amount}") },
                    modifier = Modifier.animateItem(),
                    overlineContent = { Text(text = "ID: $index") },
                    supportingContent = { Text(text = "Secret: ${distribution.secret}") },
                    trailingContent = if (distribution.claimed) {
                        null
                    } else {
                        {
                            Button(onClick = { onClaim(deposit, index, distribution) }) {
                                Text(text = "Claim")
                            }
                        }
                    }
                )
            }
        }
    }
}

@Preview
@Composable
private fun ClaimListScreenPreview() {
    ZkCommitMobileTheme {
        ClaimListScreen(
            deposit = Deposit(
                id = "2",
                distributions = listOf(
                    Distribution(claimed = false, amount = 1UL, secret = 1UL),
                    Distribution(claimed = true, amount = 2UL, secret = 2UL)
                )
            ),
            messages = emptyFlow()
        )
    }
}
