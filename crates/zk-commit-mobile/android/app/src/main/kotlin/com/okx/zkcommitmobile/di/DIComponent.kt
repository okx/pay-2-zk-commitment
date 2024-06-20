package com.okx.zkcommitmobile.di

import android.content.Context
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
}
