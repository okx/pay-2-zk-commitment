package com.okx.zkcommitmobile

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import com.okx.zkcommitmobile.data.GetAccountState
import com.walletconnect.web3.modal.client.Modal
import com.walletconnect.web3.modal.client.Web3Modal
import kotlinx.coroutines.CoroutineDispatcher
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import timber.log.Timber

class WalletConnectManager(
    private val applicationScope: CoroutineScope,
    private val ioDispatcher: CoroutineDispatcher
) : Web3Modal.ModalDelegate {
    companion object {
        private val Log = Timber.tag("WalletConnectManager")
    }

    private var _getAccountState by mutableStateOf<GetAccountState>(GetAccountState.Loading)
    val getAccountState get() = _getAccountState

    fun getAccount() {
        applicationScope.launch {
            _getAccountState =
                withContext(ioDispatcher) { GetAccountState.Loaded(Web3Modal.getAccount()) }
        }
    }

    override fun onConnectionStateChange(state: Modal.Model.ConnectionState) {
        Log.i("Connection state: $state")
    }

    override fun onError(error: Modal.Model.Error) {
        Log.e(error.throwable, "Web3Modal error")
    }

    override fun onProposalExpired(proposal: Modal.Model.ExpiredProposal) {
        Log.i("Proposal expired: $proposal")
    }

    override fun onRequestExpired(request: Modal.Model.ExpiredRequest) {
        Log.i("Request expired: $request")
    }

    override fun onSessionApproved(approvedSession: Modal.Model.ApprovedSession) {
        Log.i("Session approved: $approvedSession")
        getAccount()
    }

    override fun onSessionDelete(deletedSession: Modal.Model.DeletedSession) {
        Log.i("Session deleted: $deletedSession")
        getAccount()
    }

    override fun onSessionAuthenticateResponse(
        sessionUpdateResponse: Modal.Model.SessionAuthenticateResponse
    ) {
        Log.i("Session authenticate response: $sessionUpdateResponse")
    }

    override fun onSessionEvent(sessionEvent: Modal.Model.Event) {
        Log.i("Session event: $sessionEvent")
    }

    @Deprecated(
        "Use onSessionEvent(Modal.Model.Event) instead. Using both will result in duplicate events.",
        replaceWith = ReplaceWith("onEvent(event)")
    )
    override fun onSessionEvent(sessionEvent: Modal.Model.SessionEvent) {
        Log.i("Session event: $sessionEvent")
    }

    override fun onSessionExtend(session: Modal.Model.Session) {
        Log.i("Session extended: $session")
    }

    override fun onSessionRejected(rejectedSession: Modal.Model.RejectedSession) {
        Log.i("Session rejected: $rejectedSession")
    }

    override fun onSessionRequestResponse(response: Modal.Model.SessionRequestResponse) {
        Log.i("Session request response: $response")
    }

    override fun onSessionUpdate(updatedSession: Modal.Model.UpdatedSession) {
        Log.i("Session updated: $updatedSession")
    }
}
