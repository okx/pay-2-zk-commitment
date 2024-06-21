package com.okx.zkcommitmobile

import android.app.Application
import com.okx.zkcommitmobile.di.diComponent
import com.walletconnect.android.Core
import com.walletconnect.android.CoreClient
import com.walletconnect.android.relay.ConnectionType
import com.walletconnect.web3.modal.client.Modal
import com.walletconnect.web3.modal.client.Web3Modal
import com.walletconnect.web3.modal.presets.Web3ModalChainsPresets
import timber.log.Timber

private const val WALLET_CONNECT_TAG = "WalletConnect"

class ZkCommitMobileApp : Application() {
    companion object {
        private val WalletConnectLog = Timber.tag(WALLET_CONNECT_TAG)
    }

    override fun onCreate() {
        super.onCreate()
        Timber.plant(Timber.DebugTree())

        val connectionType = ConnectionType.AUTOMATIC
        val projectId =
            "a69b818a7043b77cf9edf14b5af4faf7" // Get Project ID at https://cloud.walletconnect.com/
        val relayUrl = "relay.walletconnect.com"
        val serverUrl = "wss://$relayUrl?projectId=$projectId"
        val appMetaData = Core.Model.AppMetaData(
            name = "ZK Commit Mobile",
            description = "ZK Commit Mobile",
            url = "kotlin.walletconnect.com",
            icons = listOf("https://avatars.githubusercontent.com/u/120148534"),
            redirect = "zkcommitmobile://result"
        )

        CoreClient.initialize(
            relayServerUrl = serverUrl,
            connectionType = connectionType,
            application = this,
            metaData = appMetaData,
            onError = { WalletConnectLog.e(it.throwable, "CoreClient initialization failed") }
        )

        Web3Modal.initialize(
            init = Modal.Params.Init(
                core = CoreClient,
                recommendedWalletsIds = listOf(
                    "971e689d0a5be527bac79629b4ee9b925e82208e5168b733496a09c0faed0709"
                )
            ),
            onSuccess = {
                // Callback will be called if initialization is successful
                WalletConnectLog.i("Web3Modal initialized")
                Web3Modal.setChains(Web3ModalChainsPresets.ethChains.values.toList())
                Web3Modal.setDelegate(diComponent.walletConnectManager)
            },
            onError = { error ->
                // Error will be thrown if there's an issue during initialization
                WalletConnectLog.e(error.throwable, "Web3Modal initialization failed")
            }
        )
    }
}
