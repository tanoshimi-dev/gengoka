package app.gengoka.data.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class SocialLoginRequestDto(
    val provider: String,
    @SerialName("id_token")
    val idToken: String? = null,
    @SerialName("access_token")
    val accessToken: String? = null,
    @SerialName("device_info")
    val deviceInfo: String? = null
)
