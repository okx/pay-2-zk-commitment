@file:OptIn(ExperimentalCoroutinesApi::class)

package com.okx.zkcommitmobile.di

import android.content.Context
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewmodel.initializer
import androidx.lifecycle.viewmodel.viewModelFactory
import com.okx.zkcommitmobile.DepositViewModel
import com.okx.zkcommitmobile.WalletConnectManager
import kotlinx.coroutines.CoroutineExceptionHandler
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.SupervisorJob
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory
import timber.log.Timber

interface DIComponent {
    val json: Json
    val retrofit: Retrofit
    val okHttpClient: OkHttpClient
    val walletConnectManager: WalletConnectManager

    val viewModelFactory: ViewModelProvider.Factory

    companion object {
        @Volatile
        private var instance: DIComponent? = null

        fun getInstance(context: Context): DIComponent = instance ?: synchronized(this) {
            instance ?: DIComponentImpl(context).also { instance = it }
        }
    }
}

val Context.diComponent get() = DIComponent.getInstance(this)

class DIComponentImpl(context: Context) : DIComponent {
    private val applicationContext = context.applicationContext
    override val json by lazy { Json { ignoreUnknownKeys = true } }
    override val retrofit by lazy {
        Retrofit.Builder()
            .baseUrl("https://www.example.com/")
            .client(okHttpClient)
            .addConverterFactory(json.asConverterFactory("application/json".toMediaType()))
            .build()
    }
    override val okHttpClient by lazy {
        OkHttpClient.Builder()
            .addInterceptor(HttpLoggingInterceptor().setLevel(HttpLoggingInterceptor.Level.BODY))
            .build()
    }
    override val walletConnectManager by lazy {
        WalletConnectManager(
            applicationScope = CoroutineScope(
                SupervisorJob() + Dispatchers.Main.immediate +
                    CoroutineExceptionHandler { _, throwable ->
                        Timber.e(throwable)
                    }
            ),
            ioDispatcher = Dispatchers.IO.limitedParallelism(64)
        )
    }

    override val viewModelFactory by lazy {
        viewModelFactory {
            initializer { DepositViewModel(walletConnectManager) }
        }
    }
}
