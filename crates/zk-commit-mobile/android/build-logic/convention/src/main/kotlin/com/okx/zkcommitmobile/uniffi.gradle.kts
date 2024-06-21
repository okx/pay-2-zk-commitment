@file:Suppress("UnstableApiUsage")

package com.okx.zkcommitmobile

import com.android.build.api.dsl.CommonExtension
import com.android.build.api.variant.AndroidComponentsExtension
import com.android.build.api.variant.HasHostTests
import com.android.build.api.variant.HostTestBuilder
import com.android.build.gradle.BasePlugin

val libs = versionCatalogs.named("libs")

private fun String.toLibraryName() = replace('-', '_')

plugins.withType<BasePlugin> {
    val uniFFIExtension = extensions.create<UniFFIExtension>("uniffi")
    extensions.configure<CommonExtension<*, *, *, *, *, *>>("android") {
        compileOptions.isCoreLibraryDesugaringEnabled = true
    }
    extensions.configure<AndroidComponentsExtension<*, *, *>>("androidComponents") {
        onVariants { variant ->
            val cargoUniFFITask = tasks.register<CargoUniFFITask>(
                variant.computeTaskName("cargo", "uniFFI")
            ) {
                rustCrateDir = uniFFIExtension.rustCrateDir
                release = variant.buildType == "release"
                libraryFile = uniFFIExtension.rustRootProjectDir
                    .dir("target/${Target.values().first().rust}/${variant.buildType}")
                    .flatMap { directory ->
                        directory.file(
                            uniFFIExtension.libraryName.map {
                                "lib${it.toLibraryName()}.so"
                            }
                        )
                    }
            }
            variant.sources.jniLibs?.addGeneratedSourceDirectory(cargoUniFFITask) { it.jniLibsDir }
            variant.sources.java?.addGeneratedSourceDirectory(cargoUniFFITask) {
                it.generatedSourcesDir
            }
            tasks.all {
                when (name) {
                    variant.computeTaskName("merge", "jniLibFolders"),
                    variant.computeTaskName("compile", "kotlin"),
                    variant.computeTaskName("connected", "androidTest"),
                    variant.computeTaskName("compile", "javaWithJavac") -> {
                        dependsOn(cargoUniFFITask)
                    }
                }
            }

            val unitTest =
                (variant as? HasHostTests)?.hostTests?.get(HostTestBuilder.UNIT_TEST_TYPE)
                    ?: return@onVariants

            val cargoUniFFITestTask = tasks.register<CargoUniFFITestTask>(
                variant.computeTaskName("cargo", "uniFFITest")
            ) {
                dependsOn(cargoUniFFITask)
                rustCrateDir = cargoUniFFITask.flatMap { it.rustCrateDir }
                release = cargoUniFFITask.flatMap { it.release }
                testLibraryFile = uniFFIExtension.rustRootProjectDir
                    .dir("target/${variant.buildType}")
                    .flatMap { directory ->
                        directory.file(
                            uniFFIExtension.libraryName.map {
                                System.mapLibraryName(it.toLibraryName())
                            }
                        )
                    }
            }
            val jniLibs = unitTest.sources.jniLibs ?: return@onVariants
            jniLibs.addGeneratedSourceDirectory(cargoUniFFITestTask) { it.testJniLibsDir }
            unitTest.configureTestTask { test ->
                test.dependsOn(cargoUniFFITestTask)
                // See `addGeneratedSourceDirectory`
                val libraryPath =
                    layout.buildDirectory.dir(
                        "generated/${jniLibs.name}/${cargoUniFFITestTask.name}"
                    )
                        .map { it.asFile.absolutePath }
                test.systemProperty("jna.library.path", libraryPath.get())
            }
        }
    }
    dependencies {
        "coreLibraryDesugaring"(libs.findLibrary("desugar_jdk_libs_nio").get())
        "implementation"(libs.findLibrary("jna").get()) { artifact { type = "aar" } }
        "testImplementation"(libs.findLibrary("jna").get())
    }
}
