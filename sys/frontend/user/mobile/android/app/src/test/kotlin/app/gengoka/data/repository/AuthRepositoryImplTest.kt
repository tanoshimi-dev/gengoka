package app.gengoka.data.repository

import app.gengoka.core.network.TokenManager
import app.gengoka.core.util.Resource
import app.gengoka.data.api.GengokApi
import app.gengoka.data.dto.ApiResponse
import app.gengoka.data.dto.AuthTokensDto
import app.gengoka.data.dto.LinkedSocialAccountDto
import app.gengoka.data.dto.LoginRequestDto
import app.gengoka.data.dto.RegisterRequestDto
import app.gengoka.data.dto.SocialLoginRequestDto
import app.gengoka.data.dto.UserSummaryDto
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.just
import io.mockk.mockk
import io.mockk.runs
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test

class AuthRepositoryImplTest {

    private val api = mockk<GengokApi>()
    private val tokenManager = mockk<TokenManager>()
    private lateinit var repository: AuthRepositoryImpl

    private val testTokens = AuthTokensDto(
        accessToken = "test-access-token",
        refreshToken = "test-refresh-token",
        expiresIn = 3600,
        user = UserSummaryDto(id = "user-1", name = "Test User", avatar = null)
    )

    @Before
    fun setup() {
        repository = AuthRepositoryImpl(api, tokenManager)
    }

    @Test
    fun `login success saves tokens`() = runTest {
        coEvery { api.login(any()) } returns ApiResponse(success = true, data = testTokens)
        coEvery { tokenManager.saveTokens(testTokens) } just runs

        val result = repository.login("test@example.com", "password")

        assertTrue(result is Resource.Success)
        coVerify { tokenManager.saveTokens(testTokens) }
    }

    @Test
    fun `login failure returns error`() = runTest {
        coEvery { api.login(any()) } throws RuntimeException("Invalid credentials")

        val result = repository.login("test@example.com", "wrong")

        assertTrue(result is Resource.Error)
        assertEquals("Invalid credentials", (result as Resource.Error).message)
    }

    @Test
    fun `register success saves tokens`() = runTest {
        coEvery { api.register(any()) } returns ApiResponse(success = true, data = testTokens)
        coEvery { tokenManager.saveTokens(testTokens) } just runs

        val result = repository.register("test@example.com", "password", "Test User")

        assertTrue(result is Resource.Success)
        coVerify { tokenManager.saveTokens(testTokens) }
    }

    @Test
    fun `register failure returns error`() = runTest {
        coEvery { api.register(any()) } throws RuntimeException("Email already exists")

        val result = repository.register("test@example.com", "password", "Test User")

        assertTrue(result is Resource.Error)
        assertEquals("Email already exists", (result as Resource.Error).message)
    }

    @Test
    fun `socialLogin success saves tokens`() = runTest {
        coEvery { api.socialLogin(any()) } returns ApiResponse(success = true, data = testTokens)
        coEvery { tokenManager.saveTokens(testTokens) } just runs

        val result = repository.socialLogin("google", "id-token", null)

        assertTrue(result is Resource.Success)
        coVerify {
            api.socialLogin(match {
                it.provider == "google" && it.idToken == "id-token" && it.deviceInfo == "Android"
            })
        }
        coVerify { tokenManager.saveTokens(testTokens) }
    }

    @Test
    fun `refreshToken returns error when no refresh token available`() = runTest {
        coEvery { tokenManager.getRefreshToken() } returns null

        val result = repository.refreshToken()

        assertTrue(result is Resource.Error)
        assertEquals("No refresh token available", (result as Resource.Error).message)
    }

    @Test
    fun `refreshToken success saves new tokens`() = runTest {
        coEvery { tokenManager.getRefreshToken() } returns "old-refresh-token"
        coEvery { api.refreshToken(any()) } returns ApiResponse(success = true, data = testTokens)
        coEvery { tokenManager.saveTokens(testTokens) } just runs

        val result = repository.refreshToken()

        assertTrue(result is Resource.Success)
        coVerify { tokenManager.saveTokens(testTokens) }
    }

    @Test
    fun `logout clears tokens even when API fails`() = runTest {
        coEvery { tokenManager.getRefreshToken() } returns "refresh-token"
        coEvery { api.logout(any()) } throws RuntimeException("Network error")
        coEvery { tokenManager.clearTokens() } just runs

        val result = repository.logout()

        assertTrue(result is Resource.Success)
        coVerify { tokenManager.clearTokens() }
    }
}
