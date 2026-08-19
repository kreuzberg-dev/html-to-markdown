package io.xberg.android

import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import org.junit.Assert.assertEquals
import org.junit.Test

class HtmlToMarkdownScaffoldTest {
    // Round-trips the generated `ImageDimensions` data class through the Jackson mapping the
    // JNI bridge marshals values with: it fails to compile if the generated constructor
    // loses a parameter or changes a type, and fails at runtime if the class stops being
    // serializable or stops rebuilding an equal value. It proves nothing about the native
    // library -- no tier here loads it, deliberately; see the note on the emitter. Seeded
    // once and never regenerated over, so replace it with a real suite. ~keep
    @Test
    fun imageDimensionsRoundTripsThroughItsGeneratedJsonMapping() {
        val original = ImageDimensions(1, 1)
        val mapper = jacksonObjectMapper()
        val json = mapper.writeValueAsString(original)
        assertEquals(original, mapper.readValue(json, ImageDimensions::class.java))
    }
}
