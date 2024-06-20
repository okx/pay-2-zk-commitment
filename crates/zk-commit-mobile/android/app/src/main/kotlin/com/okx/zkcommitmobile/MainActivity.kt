@file:OptIn(ExperimentalComposeUiApi::class)

package com.okx.zkcommitmobile

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.ui.ExperimentalComposeUiApi
import androidx.compose.ui.Modifier
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.semantics.testTagsAsResourceId
import com.okx.zkcommitmobile.di.diComponent
import com.okx.zkcommitmobile.ui.DepositApp
import com.okx.zkcommitmobile.ui.theme.ZkCommitMobileTheme
import timber.log.Timber

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            ZkCommitMobileTheme {
                DepositApp(modifier = Modifier.semantics { testTagsAsResourceId = true })
            }
        }
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        Timber.tag("MainActivity").i("onNewIntent: $intent")
        intent.data?.let { diComponent.okxWcmSDK.handleResponse(it) }
    }
}
