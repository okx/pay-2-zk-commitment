package com.okx.zkcommitmobile

internal enum class Target(val android: String, val rust: String) {
    ARM64_V8A("arm64-v8a", "aarch64-linux-android"),
    X86_64("x86_64", "x86_64-linux-android")
}
