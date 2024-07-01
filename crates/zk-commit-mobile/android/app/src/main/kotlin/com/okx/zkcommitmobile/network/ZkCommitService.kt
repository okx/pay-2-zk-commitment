package com.okx.zkcommitmobile.network

import java.io.File
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.MultipartBody
import okhttp3.RequestBody.Companion.asRequestBody
import retrofit2.http.Multipart
import retrofit2.http.POST
import retrofit2.http.Part

interface ZkCommitService {
    @Multipart
    @POST("api/v1/wrap/groth16")
    suspend fun wrapGroth16(@Part file: MultipartBody.Part): String
}

suspend fun ZkCommitService.wrapGroth16(file: File): String {
    val requestBody = file.asRequestBody("application/octet-stream".toMediaType())
    val part = MultipartBody.Part.createFormData("file", file.name, requestBody)
    return wrapGroth16(part)
}
