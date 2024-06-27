package com.okx.zkcommitmobile.benchmark

import androidx.benchmark.junit4.BenchmarkRule
import androidx.benchmark.junit4.measureRepeated
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.okx.zkcommitmobile.uniffi.AmountSecretPairing
import com.okx.zkcommitmobile.uniffi.generateProofOfClaim
import com.okx.zkcommitmobile.uniffi.setupCommitment
import java.io.File
import java.security.SecureRandom
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Benchmark, which will execute on an Android device.
 *
 * The body of [BenchmarkRule.measureRepeated] is measured in a loop, and Studio will
 * output the result. Modify your code to see how it affects performance.
 */
@RunWith(AndroidJUnit4::class)
class ZkCommitMobileBenchmark {
    companion object {
        private val secureRandom = SecureRandom().apply { setSeed(0) }
        private val count = 65536
        private val amountSecretPairings = List(count) {
            AmountSecretPairing(
                amount = (secureRandom.nextInt(10) + 1).toULong(),
                secret = secureRandom.nextLong().toULong()
            )
        }
        private val commitmentTree = setupCommitment(amountSecretPairings)
    }

    @get:Rule
    val benchmarkRule = BenchmarkRule()

    @Test
    fun benchmarkSetupCommitment() {
        benchmarkRule.measureRepeated {
            setupCommitment(amountSecretPairings)
        }
    }

    @Test
    fun benchmarkGenerateProofOfClaim() {
        val context = InstrumentationRegistry.getInstrumentation().context
        benchmarkRule.measureRepeated {
            val index = 0
            generateProofOfClaim(
                amount = amountSecretPairings[index].amount,
                secret = amountSecretPairings[index].secret,
                index = index,
                commitmentTree = commitmentTree,
                path = File(context.getExternalFilesDir(null), "proof-0-$index").absolutePath
            )
        }
    }
}
