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

internal abstract class CargoUniFFITestTask @Inject constructor(
    private val fs: FileSystemOperations,
    private val exec: ExecOperations
) : DefaultTask() {
    @get:Console
    abstract val rustCrateDir: DirectoryProperty

    @get:Console
    abstract val release: Property<Boolean>

    @get:Console
    abstract val testLibraryFile: RegularFileProperty

    @get:OutputDirectory
    abstract val testJniLibsDir: DirectoryProperty

    init {
        outputs.upToDateWhen { false }
    }

    @TaskAction
    fun copyTestJniLibs() {
        fs.delete { delete(testJniLibsDir) }
        exec.exec {
            workingDir(rustCrateDir)
            val commandLine = buildList {
                add("cargo")
                add("build")
                if (release.get()) {
                    add("--release")
                }
            }
            commandLine(commandLine)
        }
        fs.copy {
            from(testLibraryFile)
            into(testJniLibsDir)
        }
    }
}
