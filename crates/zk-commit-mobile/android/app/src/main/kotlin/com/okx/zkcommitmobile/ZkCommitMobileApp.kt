package com.okx.zkcommitmobile

import android.app.Application
import com.walletconnect.android.Core
import com.walletconnect.android.CoreClient
import com.walletconnect.android.relay.ConnectionType
import com.walletconnect.web3.modal.client.Modal
import com.walletconnect.web3.modal.client.Web3Modal
import com.walletconnect.web3.modal.presets.Web3ModalChainsPresets
import timber.log.Timber

private const val WALLET_CONNECT_TAG = "WalletConnect"

class ZkCommitMobileApp : Application() {
    override fun onCreate() {
        super.onCreate()
        Timber.plant(Timber.DebugTree())
        val connectionType = ConnectionType.AUTOMATIC
        val projectId =
            "a69b818a7043b77cf9edf14b5af4faf7" // Get Project ID at https://cloud.walletconnect.com/
        val relayUrl = "relay.walletconnect.com"
        val serverUrl = "wss://$relayUrl?projectId=$projectId"
        val appMetaData =
            Core.Model.AppMetaData(
                name = "Kotlin.Web3Modal",
                description = "Kotlin Web3Modal Implementation",
                url = "kotlin.walletconnect.com",
                icons = listOf(
                    "https://raw.githubusercontent.com/WalletConnect/walletconnect-assets/master/Icon/Gradient/Icon.png"
                ),
                redirect = "kotlin-web3modal://request"
            )

        CoreClient.initialize(
            relayServerUrl = serverUrl,
            connectionType = connectionType,
            application = this,
            metaData = appMetaData,
            onError = {
                Timber.tag(WALLET_CONNECT_TAG).e(it.throwable, "CoreClient initialization failed")
            }
        )

        Web3Modal.initialize(
            init =
            Modal.Params.Init(
                core = CoreClient,
                recommendedWalletsIds = listOf(
                    "971e689d0a5be527bac79629b4ee9b925e82208e5168b733496a09c0faed0709"
                )
            ),
            onSuccess = {
                // Callback will be called if initialization is successful
                Timber.tag(WALLET_CONNECT_TAG).i("Web3Modal initialized")
                Web3Modal.setChains(Web3ModalChainsPresets.ethChains.values.toList())
                Web3Modal.setDelegate(ModalDelegate())
            },
            onError = { error ->
                // Error will be thrown if there's an issue during initialization
                Timber.tag(WALLET_CONNECT_TAG).e(error.throwable, "Web3Modal initialization failed")
            }
        )
    }
}

class ModalDelegate : Web3Modal.ModalDelegate {
    override fun onConnectionStateChange(state: Modal.Model.ConnectionState) {
        Timber.tag(WALLET_CONNECT_TAG).i("Connection state: $state")
    }

    override fun onError(error: Modal.Model.Error) {
        Timber.tag(WALLET_CONNECT_TAG).e(error.throwable, "Web3Modal error")
    }

    override fun onProposalExpired(proposal: Modal.Model.ExpiredProposal) {
        Timber.tag(WALLET_CONNECT_TAG).i("Proposal expired: $proposal")
    }

    override fun onRequestExpired(request: Modal.Model.ExpiredRequest) {
        Timber.tag(WALLET_CONNECT_TAG).i("Request expired: $request")
    }

    override fun onSessionApproved(approvedSession: Modal.Model.ApprovedSession) {
        Timber.tag(WALLET_CONNECT_TAG).i("Session approved: $approvedSession")
    }

    override fun onSessionDelete(deletedSession: Modal.Model.DeletedSession) {
        Timber.tag(WALLET_CONNECT_TAG).i("Session deleted: $deletedSession")
    }

    override fun onSessionAuthenticateResponse(
        sessionUpdateResponse: Modal.Model.SessionAuthenticateResponse
    ) {
        Timber
            .tag(WALLET_CONNECT_TAG)
            .i("Session authenticate response: $sessionUpdateResponse")
    }

    override fun onSessionEvent(sessionEvent: Modal.Model.Event) {
        Timber.tag(WALLET_CONNECT_TAG).i("Session event: $sessionEvent")
    }

    @Deprecated(
        "Use onSessionEvent(Modal.Model.Event) instead. Using both will result in duplicate events.",
        replaceWith = ReplaceWith("onEvent(event)")
    )
    override fun onSessionEvent(sessionEvent: Modal.Model.SessionEvent) {
        Timber.tag(WALLET_CONNECT_TAG).i("Session event: $sessionEvent")
    }

    override fun onSessionExtend(session: Modal.Model.Session) {
        Timber.tag(WALLET_CONNECT_TAG).i("Session extended: $session")
    }

    override fun onSessionRejected(rejectedSession: Modal.Model.RejectedSession) {
        Timber.tag(WALLET_CONNECT_TAG).i("Session rejected: $rejectedSession")
    }

    override fun onSessionRequestResponse(response: Modal.Model.SessionRequestResponse) {
        Timber.tag(WALLET_CONNECT_TAG).i("Session request response: $response")
    }

    override fun onSessionUpdate(updatedSession: Modal.Model.UpdatedSession) {
        Timber.tag(WALLET_CONNECT_TAG).i("Session updated: $updatedSession")
    }
}
