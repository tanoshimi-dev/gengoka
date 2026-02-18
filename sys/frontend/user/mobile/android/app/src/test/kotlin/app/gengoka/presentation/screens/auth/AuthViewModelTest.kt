package app.gengoka.presentation.screens.auth

import app.gengoka.core.util.Resource
import app.gengoka.domain.repository.AuthRepository
import app.gengoka.testutil.MainDispatcherRule
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.mockk
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.advanceUntilIdle
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class AuthViewModelTest {

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private val authRepository = mockk<AuthRepository>()
    private lateinit var viewModel: AuthViewModel

    @Before
    fun setup() {
        viewModel = AuthViewModel(authRepository)
    }

    @Test
    fun `initial state is login mode with no loading or error`() {
        val state = viewModel.uiState.value
        assertFalse(state.isLoading)
        assertFalse(state.isSocialLoading)
        assertNull(state.error)
        assertTrue(state.isLoginMode)
    }

    @Test
    fun `login success clears loading`() = runTest {
        coEvery { authRepository.login("test@example.com", "password") } returns Resource.Success(Unit)

        viewModel.login("test@example.com", "password")
        advanceUntilIdle()

        assertFalse(viewModel.uiState.value.isLoading)
        assertNull(viewModel.uiState.value.error)
    }

    @Test
    fun `login error sets error message`() = runTest {
        coEvery { authRepository.login("test@example.com", "wrong") } returns Resource.Error("認証に失敗しました")

        viewModel.login("test@example.com", "wrong")
        advanceUntilIdle()

        assertFalse(viewModel.uiState.value.isLoading)
        assertEquals("認証に失敗しました", viewModel.uiState.value.error)
    }

    @Test
    fun `login with blank email does nothing`() = runTest {
        viewModel.login("", "password")
        advanceUntilIdle()

        assertFalse(viewModel.uiState.value.isLoading)
        coVerify(exactly = 0) { authRepository.login(any(), any()) }
    }

    @Test
    fun `login with blank password does nothing`() = runTest {
        viewModel.login("test@example.com", "")
        advanceUntilIdle()

        assertFalse(viewModel.uiState.value.isLoading)
        coVerify(exactly = 0) { authRepository.login(any(), any()) }
    }

    @Test
    fun `register success clears loading`() = runTest {
        coEvery { authRepository.register("test@example.com", "password", "Test") } returns Resource.Success(Unit)

        viewModel.register("test@example.com", "password", "Test")
        advanceUntilIdle()

        assertFalse(viewModel.uiState.value.isLoading)
        assertNull(viewModel.uiState.value.error)
    }

    @Test
    fun `register error sets error message`() = runTest {
        coEvery { authRepository.register("test@example.com", "password", "Test") } returns Resource.Error("メール既に登録済み")

        viewModel.register("test@example.com", "password", "Test")
        advanceUntilIdle()

        assertEquals("メール既に登録済み", viewModel.uiState.value.error)
    }

    @Test
    fun `register with blank fields does nothing`() = runTest {
        viewModel.register("", "password", "Test")
        advanceUntilIdle()
        coVerify(exactly = 0) { authRepository.register(any(), any(), any()) }

        viewModel.register("test@example.com", "", "Test")
        advanceUntilIdle()
        coVerify(exactly = 0) { authRepository.register(any(), any(), any()) }

        viewModel.register("test@example.com", "password", "")
        advanceUntilIdle()
        coVerify(exactly = 0) { authRepository.register(any(), any(), any()) }
    }

    @Test
    fun `toggleMode switches between login and register`() {
        assertTrue(viewModel.uiState.value.isLoginMode)

        viewModel.toggleMode()
        assertFalse(viewModel.uiState.value.isLoginMode)

        viewModel.toggleMode()
        assertTrue(viewModel.uiState.value.isLoginMode)
    }

    @Test
    fun `toggleMode clears error`() = runTest {
        coEvery { authRepository.login(any(), any()) } returns Resource.Error("エラー")
        viewModel.login("test@example.com", "password")
        advanceUntilIdle()
        assertEquals("エラー", viewModel.uiState.value.error)

        viewModel.toggleMode()
        assertNull(viewModel.uiState.value.error)
    }

    @Test
    fun `clearError removes error message`() = runTest {
        coEvery { authRepository.login(any(), any()) } returns Resource.Error("エラー")
        viewModel.login("test@example.com", "password")
        advanceUntilIdle()
        assertEquals("エラー", viewModel.uiState.value.error)

        viewModel.clearError()
        assertNull(viewModel.uiState.value.error)
    }

    @Test
    fun `login trims email`() = runTest {
        coEvery { authRepository.login("test@example.com", "password") } returns Resource.Success(Unit)

        viewModel.login("  test@example.com  ", "password")
        advanceUntilIdle()

        coVerify { authRepository.login("test@example.com", "password") }
    }
}
