package com.okx.zkcommitmobile.core

import com.okx.zkcommitmobile.uniffi.fibonacci
import org.junit.Assert.assertEquals
import org.junit.Test

/**
 * Example local unit test, which will execute on the development machine (host).
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
class ExampleUnitTest {
    @Test
    fun addition_isCorrect() {
        assertEquals(4, 2 + 2)
    }

    @Test
    fun fibonacci_isCorrect() {
        val result = fibonacci()
        assertEquals(3736710860384812976UL, result.output)
    }
}
