package com.okx.zkcommitmobile.data

import com.walletconnect.web3.modal.client.models.Account

sealed interface GetAccountState {
    data object Loading : GetAccountState
    data class Loaded(val account: Account?) : GetAccountState
}
