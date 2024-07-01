package com.okx.zkcommitmobile.data

import android.content.Context
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore

val Context.preferences by preferencesDataStore("zk-commit-mobile")
val BaseUrlKey = stringPreferencesKey("baseUrl")
const val DEFAULT_BASE_URL = "http://10.20.88.157:8080/"
