package app.dev.gengoka.data.repository

import app.dev.gengoka.core.network.TokenManager
import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.core.util.safeApiCall
import app.dev.gengoka.data.api.GengokApi
import app.dev.gengoka.data.dto.LoginRequestDto
import app.dev.gengoka.data.dto.LogoutRequestDto
import app.dev.gengoka.data.dto.RefreshRequestDto
import app.dev.gengoka.data.dto.RegisterRequestDto
import app.dev.gengoka.domain.repository.AuthRepository
import javax.inject.Inject

class AuthRepositoryImpl @Inject constructor(
    private val api: GengokApi,
    private val tokenManager: TokenManager
) : AuthRepository {

    override suspend fun login(email: String, password: String): Resource<Unit> {
        return safeApiCall {
            val response = api.login(LoginRequestDto(email = email, password = password))
            tokenManager.saveTokens(response.data)
        }
    }

    override suspend fun register(email: String, password: String, name: String): Resource<Unit> {
        return safeApiCall {
            val response = api.register(
                RegisterRequestDto(email = email, password = password, name = name)
            )
            tokenManager.saveTokens(response.data)
        }
    }

    override suspend fun refreshToken(): Resource<Unit> {
        val refreshToken = tokenManager.getRefreshToken()
            ?: return Resource.Error("No refresh token available")
        return safeApiCall {
            val response = api.refreshToken(RefreshRequestDto(refreshToken = refreshToken))
            tokenManager.saveTokens(response.data)
        }
    }

    override suspend fun logout(): Resource<Unit> {
        return safeApiCall {
            try {
                api.logout(LogoutRequestDto(refreshToken = tokenManager.getRefreshToken()))
            } catch (_: Exception) {
                // Ignore API errors on logout — clear tokens regardless
            }
            tokenManager.clearTokens()
        }
    }
}
