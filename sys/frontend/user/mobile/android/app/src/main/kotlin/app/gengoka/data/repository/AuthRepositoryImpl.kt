package app.gengoka.data.repository

import app.gengoka.core.network.TokenManager
import app.gengoka.core.util.Resource
import app.gengoka.core.util.safeApiCall
import app.gengoka.data.api.GengokApi
import app.gengoka.data.dto.LinkAccountRequestDto
import app.gengoka.data.dto.LoginRequestDto
import app.gengoka.data.dto.LogoutRequestDto
import app.gengoka.data.dto.RefreshRequestDto
import app.gengoka.data.dto.RegisterRequestDto
import app.gengoka.data.dto.SocialLoginRequestDto
import app.gengoka.data.mapper.toDomain
import app.gengoka.domain.model.LinkedSocialAccount
import app.gengoka.domain.repository.AuthRepository
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

    override suspend fun socialLogin(
        provider: String,
        idToken: String?,
        accessToken: String?
    ): Resource<Unit> {
        return safeApiCall {
            val response = api.socialLogin(
                SocialLoginRequestDto(
                    provider = provider,
                    idToken = idToken,
                    accessToken = accessToken,
                    deviceInfo = "Android"
                )
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

    override suspend fun getLinkedAccounts(): Resource<List<LinkedSocialAccount>> {
        return safeApiCall {
            api.getLinkedAccounts().data.map { it.toDomain() }
        }
    }

    override suspend fun linkAccount(
        provider: String,
        idToken: String?,
        accessToken: String?
    ): Resource<LinkedSocialAccount> {
        return safeApiCall {
            val response = api.linkAccount(
                LinkAccountRequestDto(
                    provider = provider,
                    idToken = idToken,
                    accessToken = accessToken
                )
            )
            response.data.toDomain()
        }
    }

    override suspend fun unlinkAccount(provider: String): Resource<Unit> {
        return safeApiCall {
            api.unlinkAccount(provider)
        }
    }
}
