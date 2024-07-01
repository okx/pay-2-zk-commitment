package com.okx.zkcommitmobile

import javax.inject.Inject
import org.gradle.api.DefaultTask
import org.gradle.api.file.DirectoryProperty
import org.gradle.api.file.FileSystemOperations
import org.gradle.api.file.RegularFileProperty
import org.gradle.api.provider.Property
import org.gradle.api.tasks.Console
import org.gradle.api.tasks.OutputDirectory
import org.gradle.api.tasks.TaskAction
import org.gradle.process.ExecOperations

internal abstract class CargoUniFFITask @Inject constructor(
    private val fs: FileSystemOperations,
    private val exec: ExecOperations
) : DefaultTask() {

    @get:Console
    abstract val rustCrateDir: DirectoryProperty

    @get:Console
    abstract val release: Property<Boolean>

    @get:Console
    abstract val libraryFile: RegularFileProperty

    @get:OutputDirectory
    abstract val jniLibsDir: DirectoryProperty

    @get:OutputDirectory
    abstract val generatedSourcesDir: DirectoryProperty

    init {
        outputs.upToDateWhen { false }
    }

    @TaskAction
    fun buildUniFFI() {
        fs.delete { delete(jniLibsDir, generatedSourcesDir) }
        exec.exec {
            workingDir(rustCrateDir)
            environment("RUSTFLAGS", "-C target-feature=+neon")
            val cargoNdkBuild = buildList {
                add("cargo")
                add("ndk")
                add("-o")
                add(jniLibsDir.asFile.get().path)
                for (target in Target.values()) {
                    add("-t")
                    add(target.android)
                }
                if (!release.get()) {
                    add("--no-strip")
                }
                add("build")
                if (release.get()) {
                    add("--release")
                }
            }
            println("Running: ${cargoNdkBuild.joinToString(" ")}")
            commandLine(cargoNdkBuild)
        }
        exec.exec {
            workingDir(rustCrateDir)
            commandLine(
                "cargo",
                "run",
                "--bin",
                "uniffi-bindgen",
                "generate",
                "--library",
                libraryFile.asFile.get().path,
                "--language",
                "kotlin",
                "--out-dir",
                generatedSourcesDir.asFile.get().path
            )
        }
    }
}
