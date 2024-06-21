package com.okx.zkcommitmobile

import org.gradle.api.file.DirectoryProperty
import org.gradle.api.provider.Property

interface UniFFIExtension {
    val rustRootProjectDir: DirectoryProperty
    val rustCrateDir: DirectoryProperty
    val libraryName: Property<String>
}
