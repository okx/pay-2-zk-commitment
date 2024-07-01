package com.okx.zkcommitmobile.network

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import com.okx.zkcommitmobile.data.BaseUrlKey
import com.okx.zkcommitmobile.data.DEFAULT_BASE_URL
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import okhttp3.Interceptor
import okhttp3.Response

class ReplaceUrlInterceptor(private val preferences: DataStore<Preferences>) : Interceptor {
    companion object {
        const val BASE_URL_PLACEHOLDER = "http://base.url.placeholder/"
    }

    override fun intercept(chain: Interceptor.Chain): Response {
        val request = chain.request()
        val url = request.url.toString()
        if (!url.startsWith(BASE_URL_PLACEHOLDER)) return chain.proceed(request)
        val baseUrl =
            runBlocking { preferences.data.first()[BaseUrlKey] }.takeIf { !it.isNullOrBlank() }
                ?: DEFAULT_BASE_URL
        return chain.proceed(
            request.newBuilder().url(url.replaceFirst(BASE_URL_PLACEHOLDER, baseUrl)).build()
        )
    }
}
