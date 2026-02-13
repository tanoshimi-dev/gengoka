package app.dev.gengoka.data.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class CategoryDto(
    val id: String,
    val name: String,
    val description: String? = null,
    @SerialName("icon_name")
    val iconName: String? = null,
    @SerialName("color_hex")
    val colorHex: String? = null,
    @SerialName("challenge_count")
    val challengeCount: Long = 0
)
