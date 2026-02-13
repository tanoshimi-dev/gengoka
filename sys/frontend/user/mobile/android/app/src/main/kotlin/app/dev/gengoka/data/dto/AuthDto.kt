package app.dev.gengoka.data.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class LoginRequestDto(
    val email: String,
    val password: String,
    @SerialName("device_info")
    val deviceInfo: String? = null
)

@Serializable
data class RegisterRequestDto(
    val email: String,
    val password: String,
    val name: String,
    @SerialName("device_info")
    val deviceInfo: String? = null
)

@Serializable
data class RefreshRequestDto(
    @SerialName("refresh_token")
    val refreshToken: String
)

@Serializable
data class LogoutRequestDto(
    @SerialName("refresh_token")
    val refreshToken: String? = null
)

@Serializable
data class AuthTokensDto(
    @SerialName("access_token")
    val accessToken: String,
    @SerialName("refresh_token")
    val refreshToken: String,
    @SerialName("expires_in")
    val expiresIn: Int? = null,
    val user: UserSummaryDto
)
