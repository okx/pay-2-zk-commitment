package com.okx.zkcommitmobile.data

import com.okx.zkcommitmobile.uniffi.CommitmentTree

data class Deposit(val id: String, val distributions: List<Distribution>) {
    val totalAmount get() = distributions.sumOf { it.amount }
}

data class Distribution(val claimed: Boolean, val amount: ULong, val secret: ULong)

data class DepositWithCommitment(val deposit: Deposit, val commitmentTree: CommitmentTree)
