package com.okx.zkcommitmobile

import android.content.Context
import com.okx.wallet.connect.mobilesdk.api.OkxWcmSDK
import com.okx.wallet.connect.mobilesdk.api.request.RequestType
import com.okx.wallet.connect.mobilesdk.api.request.Web3JsonRPC
import com.okx.wallet.connect.mobilesdk.api.response.Account
import com.okx.wallet.connect.mobilesdk.api.response.ActionResult
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.serialization.json.Json
import timber.log.Timber
import kotlin.coroutines.resume

class OKXWalletConnectManager(private val json: Json, private val okxWcmSDK: OkxWcmSDK) {
    companion object {
        private const val TAG = "OKXWalletConnectManager"
    }

    private val requestAccountAction
        get() = Web3JsonRPC.RequestAccounts(chainIds = listOf("1"), timeToLive = 25200).action()

    suspend fun getAccount(context: Context): Account? {
        if (okxWcmSDK.isConnected()) {
            val result = suspendCancellableCoroutine { cont ->
                okxWcmSDK.sendRequest(
                    uiContext = context,
                    request = RequestType.Request(requestAccountAction)
                ) { cont.resume(it) }
            }.onFailure { Timber.tag(TAG).e(it, "Failed to get account") }.getOrNull()
                ?: return null
            return when (result) {
                is ActionResult.Result -> {
                    Timber.tag(TAG).i("Action result: ${result.value}")
                    kotlin.runCatching { json.decodeFromString<Account>(result.value) }
                        .onFailure { Timber.tag(TAG).e(it, "Failed to decode account") }
                        .getOrNull()
                }

                is ActionResult.Error -> {
                    Timber.tag(TAG)
                        .e("Action result: code=${result.code}, message=${result.message}")
                    null
                }
            }
        }

        return suspendCancellableCoroutine { cont ->
            okxWcmSDK.initiateHandshake(
                uiContext = context,
                initialAction = requestAccountAction
            ) { _, account -> cont.resume(account) }
        }
    }
}
