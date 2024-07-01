@file:OptIn(ExperimentalMaterial3Api::class)

package com.okx.zkcommitmobile.ui

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.LargeTopAppBar
import androidx.compose.material3.ListItem
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.nestedscroll.nestedScroll
import androidx.compose.ui.platform.LocalContext
import androidx.datastore.preferences.core.edit
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import com.okx.zkcommitmobile.data.BaseUrlKey
import com.okx.zkcommitmobile.data.DEFAULT_BASE_URL
import com.okx.zkcommitmobile.data.preferences
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.launch
import kotlinx.serialization.Serializable

@Serializable
data object SettingsScreen

@Composable
fun SettingsScreen(modifier: Modifier = Modifier, onNavigateUp: () -> Unit = {}) {
    val scrollBehavior = TopAppBarDefaults.exitUntilCollapsedScrollBehavior()
    Scaffold(
        modifier = modifier.nestedScroll(scrollBehavior.nestedScrollConnection),
        topBar = {
            LargeTopAppBar(
                title = { Text(text = "Settings") },
                navigationIcon = {
                    IconButton(onClick = onNavigateUp) {
                        Icon(Icons.AutoMirrored.Default.ArrowBack, contentDescription = null)
                    }
                },
                scrollBehavior = scrollBehavior
            )
        }
    ) { innerPaddings ->
        LazyColumn(contentPadding = innerPaddings) {
            item { BaseUrlItem() }
        }
    }
}

@Composable
private fun BaseUrlItem(modifier: Modifier = Modifier) {
    val context = LocalContext.current
    val baseUrlFlow = remember(context) { context.preferences.data.map { it[BaseUrlKey] } }
    val baseUrl by baseUrlFlow.collectAsStateWithLifecycle(null)
    var showDialog by remember { mutableStateOf(false) }
    ListItem(
        headlineContent = { Text(text = "Base URL") },
        modifier = modifier.clickable { showDialog = true },
        supportingContent = {
            Text(
                text = baseUrl.let {
                    if (it.isNullOrBlank()) "$DEFAULT_BASE_URL (Default)" else it
                }
            )
        }
    )

    if (showDialog) {
        BaseUrlDialog(initialBaseUrl = baseUrl) { showDialog = false }
    }
}

@Composable
private fun BaseUrlDialog(
    initialBaseUrl: String?,
    modifier: Modifier = Modifier,
    onDismissRequest: () -> Unit = {}
) {
    val coroutineScope = rememberCoroutineScope()
    val context = LocalContext.current
    var baseUrl by remember { mutableStateOf(initialBaseUrl) }
    AlertDialog(
        onDismissRequest = onDismissRequest,
        confirmButton = {
            TextButton(
                onClick = {
                    coroutineScope.launch {
                        context.preferences.edit {
                            var url = baseUrl
                            if (url.isNullOrBlank()) {
                                it.remove(BaseUrlKey)
                                return@edit
                            }

                            if (!url.startsWith("http://", ignoreCase = true)) {
                                url = "http://$url"
                            }
                            if (!url.endsWith("/")) {
                                url += "/"
                            }
                            it[BaseUrlKey] = url
                        }
                        onDismissRequest()
                    }
                }
            ) {
                Text(text = "Save")
            }
        },
        modifier = modifier,
        dismissButton = { TextButton(onClick = onDismissRequest) { Text(text = "Cancel") } },
        title = { Text(text = "Base URL") },
        text = {
            OutlinedTextField(
                value = baseUrl.orEmpty(),
                onValueChange = { baseUrl = it },
                modifier = Modifier.fillMaxWidth(),
                placeholder = { Text("E.g. $DEFAULT_BASE_URL") }
            )
        }
    )
}
