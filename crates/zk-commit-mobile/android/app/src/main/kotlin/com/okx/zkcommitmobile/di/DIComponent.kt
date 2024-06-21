package com.okx.zkcommitmobile.di

import android.content.Context
import androidx.core.net.toUri
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewmodel.initializer
import androidx.lifecycle.viewmodel.viewModelFactory
import com.okx.wallet.connect.mobilesdk.api.OKX_WALLET_GP_PACKAGE_NAME
import com.okx.wallet.connect.mobilesdk.api.OkxWcmSDK
import com.okx.wallet.connect.mobilesdk.core.message.SourceType
import com.okx.zkcommitmobile.DepositViewModel
import com.okx.zkcommitmobile.OKXWalletConnectManager
import java.net.URL
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory

interface DIComponent {
    val json: Json
    val retrofit: Retrofit
    val okHttpClient: OkHttpClient
    val okxWcmSDK: OkxWcmSDK
    val okxWalletConnectManager: OKXWalletConnectManager
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
    override val okxWcmSDK by lazy {
        OkxWcmSDK.instanceOf(
            appContext = applicationContext,
            name = "ZkCommitMobile",
            iconUrl = URL("https://avatars.githubusercontent.com/u/37784886"),
            url = URL("https://react-app.walletconnect.com"),
            domain = "zkcommitmobile://zkcommitmobile.okx.com/okx_result".toUri(),
            sourceType = SourceType.NativeDapp,
            okxWalletPackageName = OKX_WALLET_GP_PACKAGE_NAME
        )
    }
    override val okxWalletConnectManager by lazy {
        OKXWalletConnectManager(json, okxWcmSDK)
    }

    override val viewModelFactory by lazy {
        viewModelFactory {
            initializer { DepositViewModel(okxWalletConnectManager) }
        }
    }
}
