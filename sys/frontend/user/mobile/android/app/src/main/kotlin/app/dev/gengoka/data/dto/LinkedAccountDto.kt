package app.dev.gengoka.data.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class LinkedSocialAccountDto(
    val provider: String,
    @SerialName("provider_email")
    val providerEmail: String? = null,
    @SerialName("provider_name")
    val providerName: String? = null,
    @SerialName("linked_at")
    val linkedAt: String
)

@Serializable
data class LinkAccountRequestDto(
    val provider: String,
    @SerialName("id_token")
    val idToken: String? = null,
    @SerialName("access_token")
    val accessToken: String? = null
)
