@file:OptIn(ExperimentalMaterial3Api::class)

package com.okx.zkcommitmobile.ui

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.updateTransition
import androidx.compose.animation.expandVertically
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.shrinkVertically
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.ime
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListState
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ArrowDropDown
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.LargeTopAppBar
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.okx.zkcommitmobile.data.Deposit
import com.okx.zkcommitmobile.data.Distribution
import com.okx.zkcommitmobile.ui.theme.ZkCommitMobileTheme
import java.security.SecureRandom
import kotlinx.coroutines.launch
import kotlinx.serialization.Serializable

@Serializable
data object CreateDepositScreen

@Composable
fun CreateDepositScreen(
    modifier: Modifier = Modifier,
    onNavigateUp: () -> Unit = {},
    onCreate: (deposit: Deposit) -> Unit = {}
) {
    val scrollBehavior = TopAppBarDefaults.exitUntilCollapsedScrollBehavior()
    Scaffold(
        modifier = modifier.nestedScroll(scrollBehavior.nestedScrollConnection),
        topBar = {
            LargeTopAppBar(
                title = { Text(text = "Create A Deposit") },
                navigationIcon = {
                    IconButton(onClick = onNavigateUp) {
                        Icon(Icons.AutoMirrored.Default.ArrowBack, contentDescription = null)
                    }
                },
                scrollBehavior = scrollBehavior
            )
        },
        contentWindowInsets = WindowInsets.ime
    ) { innerPaddings ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPaddings)
                .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            val distributions = remember { mutableStateListOf<Distribution>() }
            val coroutineScope = rememberCoroutineScope()
            val lazyListState = rememberLazyListState()
            val secureRandom = remember { SecureRandom() }
            var isCreateClaimerExpanded by remember { mutableStateOf(true) }
            CreateDepositContent(
                expanded = isCreateClaimerExpanded,
                secureRandom = secureRandom,
                onAddDistribution = {
                    distributions += it
                    coroutineScope.launch {
                        lazyListState.animateScrollToItem(distributions.lastIndex)
                    }
                },
                onClickHeader = { isCreateClaimerExpanded = !isCreateClaimerExpanded }
            )
            GenerateClaimersContent(
                expanded = !isCreateClaimerExpanded,
                onGenerate = { count ->
                    val generated = List(count) {
                        Distribution(
                            claimed = false,
                            amount = (secureRandom.nextInt(10) + 1).toULong(),
                            secret = secureRandom.nextLong().toULong()
                        )
                    }
                    distributions.clear()
                    distributions += generated
                },
                onClickHeader = { isCreateClaimerExpanded = !isCreateClaimerExpanded }
            )
            val createEnabled by remember {
                derivedStateOf { distributions.size.countOneBits() == 1 }
            }
            Button(
                onClick = {
                    onCreate(
                        Deposit(
                            id = secureRandom.nextInt().toUInt().toString(),
                            distributions = distributions.toList()
                        )
                    )
                },
                modifier = Modifier.fillMaxWidth(),
                enabled = createEnabled
            ) {
                Text(text = "Create")
            }
            ClaimersListContent(
                distributions = distributions,
                lazyListState = lazyListState,
                modifier = Modifier.weight(1f)
            )
        }
    }
}

@Preview
@Composable
private fun CreateDepositScreenPreview() {
    ZkCommitMobileTheme {
        CreateDepositScreen()
    }
}

@Composable
fun ClaimersListContent(
    distributions: List<Distribution>,
    lazyListState: LazyListState,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        Text(text = "Claimers List", style = MaterialTheme.typography.titleLarge)
        LazyColumn(state = lazyListState) {
            itemsIndexed(distributions) { index, distribution ->
                ListItem(
                    headlineContent = { Text(text = "Amount: ${distribution.amount}") },
                    modifier = Modifier.animateItem(),
                    overlineContent = { Text(text = "Index: $index") },
                    supportingContent = { Text(text = "Secret: ${distribution.secret}") }
                )
            }
        }
    }
}

@Preview
@Composable
private fun ClaimersListContentPreview() {
    ZkCommitMobileTheme {
        ClaimersListContent(
            distributions = listOf(
                Distribution(claimed = false, amount = 1UL, secret = 1UL),
                Distribution(claimed = false, amount = 2UL, secret = 2UL)
            ),
            lazyListState = rememberLazyListState()
        )
    }
}

@Composable
fun CreateDepositContent(
    expanded: Boolean,
    secureRandom: SecureRandom,
    modifier: Modifier = Modifier,
    onAddDistribution: (distribution: Distribution) -> Unit = {},
    onClickHeader: () -> Unit = {}
) {
    Column(modifier = modifier) {
        val transition = updateTransition(expanded, label = "expanded")
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable(onClick = onClickHeader),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            val rotationZ by transition.animateFloat(label = "rotationZ") { expanded ->
                if (expanded) 0f else 270f
            }
            Icon(
                imageVector = Icons.Default.ArrowDropDown,
                contentDescription = null,
                modifier = Modifier.graphicsLayer { this.rotationZ = rotationZ }
            )
            Text(text = "Create claimer", style = MaterialTheme.typography.titleLarge)
        }

        transition.AnimatedVisibility(
            visible = { it },
            enter = fadeIn() + expandVertically(),
            exit = fadeOut() + shrinkVertically()
        ) {
            var amount by remember { mutableStateOf("") }
            val errorMessage by remember {
                derivedStateOf {
                    if (amount.isEmpty()) return@derivedStateOf null
                    val amountValue = amount.toULongOrNull()
                    if (amountValue == null ||
                        amountValue == 0UL
                    ) {
                        return@derivedStateOf "Invalid amount"
                    }
                    null
                }
            }

            Column(
                modifier = Modifier.padding(top = 8.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                OutlinedTextField(
                    value = amount,
                    onValueChange = { amount = it },
                    modifier = Modifier.fillMaxWidth(),
                    label = { Text(text = "Amount for claimer") },
                    supportingText = errorMessage?.let { { Text(text = it) } },
                    isError = errorMessage != null,
                    keyboardOptions = KeyboardOptions.Default.copy(
                        keyboardType = KeyboardType.Number
                    )
                )
                Button(
                    onClick = {
                        onAddDistribution(
                            Distribution(
                                claimed = false,
                                amount = amount.toULong(),
                                secret = secureRandom.nextLong().toULong()
                            )
                        )
                        amount = ""
                    },
                    modifier = Modifier.fillMaxWidth(),
                    enabled = amount.isNotEmpty() && errorMessage == null
                ) {
                    Text("Add claimer")
                }
            }
        }
    }
}

@Preview
@Composable
private fun CreateDepositContentPreview() {
    ZkCommitMobileTheme {
        var expanded by remember { mutableStateOf(false) }
        CreateDepositContent(
            expanded = expanded,
            secureRandom = SecureRandom(),
            onClickHeader = { expanded = !expanded }
        )
    }
}

@Composable
fun GenerateClaimersContent(
    expanded: Boolean,
    modifier: Modifier = Modifier,
    onGenerate: (count: Int) -> Unit = {},
    onClickHeader: () -> Unit = {}
) {
    Column(modifier = modifier) {
        val transition = updateTransition(expanded, label = "expanded")
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable(onClick = onClickHeader),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            val rotationZ by transition.animateFloat(label = "rotationZ") { expanded ->
                if (expanded) 0f else 270f
            }
            Icon(
                imageVector = Icons.Default.ArrowDropDown,
                contentDescription = null,
                modifier = Modifier.graphicsLayer { this.rotationZ = rotationZ }
            )
            Text(text = "Generate claimers", style = MaterialTheme.typography.titleLarge)
        }

        transition.AnimatedVisibility(
            visible = { it },
            enter = fadeIn() + expandVertically(),
            exit = fadeOut() + shrinkVertically()
        ) {
            Column(
                modifier = Modifier.padding(top = 8.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                var count by remember { mutableStateOf("") }
                val errorMessage by remember {
                    derivedStateOf {
                        if (count.isEmpty()) return@derivedStateOf null
                        val countValue =
                            count.toIntOrNull() ?: return@derivedStateOf "Invalid integer"
                        if (countValue.countOneBits() != 1) {
                            return@derivedStateOf "Must be a power of 2"
                        }
                        null
                    }
                }
                OutlinedTextField(
                    value = count,
                    onValueChange = { count = it },
                    modifier = Modifier.fillMaxWidth(),
                    label = { Text(text = "Number of claimers: ") },
                    keyboardOptions = KeyboardOptions.Default.copy(
                        keyboardType = KeyboardType.Number
                    ),
                    supportingText = errorMessage?.let { { Text(text = it) } },
                    isError = count.isNotEmpty() && errorMessage != null
                )
                Button(
                    onClick = { onGenerate(count.toInt()) },
                    modifier = Modifier.fillMaxWidth(),
                    enabled = count.isNotEmpty() && errorMessage == null
                ) {
                    Text(text = "Generate")
                }
            }
        }
    }
}

@Preview
@Composable
private fun GenerateClaimersContentPreview() {
    ZkCommitMobileTheme {
        var expanded by remember { mutableStateOf(false) }
        GenerateClaimersContent(
            expanded = expanded,
            onClickHeader = { expanded = !expanded }
        )
    }
}
