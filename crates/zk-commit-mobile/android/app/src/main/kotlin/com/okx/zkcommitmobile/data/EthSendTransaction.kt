package com.okx.zkcommitmobile.data

import kotlinx.serialization.Serializable

@Serializable
data class EthSendTransaction(val from: String, val to: String, val data: String)
