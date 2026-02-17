package app.gengoka.presentation.screens.auth

import android.content.Context
import android.content.Intent
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.gengoka.core.auth.GoogleAuthHelper
import app.gengoka.core.auth.LineAuthHelper
import app.gengoka.core.util.Resource
import app.gengoka.domain.repository.AuthRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class AuthUiState(
    val isLoading: Boolean = false,
    val isSocialLoading: Boolean = false,
    val error: String? = null,
    val isLoginMode: Boolean = true
)

@HiltViewModel
class AuthViewModel @Inject constructor(
    private val authRepository: AuthRepository
) : ViewModel() {

    private val _uiState = MutableStateFlow(AuthUiState())
    val uiState: StateFlow<AuthUiState> = _uiState.asStateFlow()

    fun login(email: String, password: String) {
        if (email.isBlank() || password.isBlank()) return
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            when (val result = authRepository.login(email.trim(), password)) {
                is Resource.Success -> {
                    _uiState.update { it.copy(isLoading = false) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(isLoading = false, error = result.message) }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun register(email: String, password: String, name: String) {
        if (email.isBlank() || password.isBlank() || name.isBlank()) return
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            when (val result = authRepository.register(email.trim(), password, name.trim())) {
                is Resource.Success -> {
                    _uiState.update { it.copy(isLoading = false) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(isLoading = false, error = result.message) }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun socialLogin(provider: String, context: Context) {
        viewModelScope.launch {
            _uiState.update { it.copy(isSocialLoading = true, error = null) }
            try {
                val idToken: String?
                val accessToken: String?
                when (provider) {
                    "google" -> {
                        idToken = GoogleAuthHelper.signIn(context)
                        accessToken = null
                    }
                    else -> {
                        idToken = null
                        accessToken = null
                    }
                }
                completeSocialLogin(provider, idToken, accessToken)
            } catch (e: Exception) {
                _uiState.update {
                    it.copy(isSocialLoading = false, error = e.message ?: "ソーシャルログインに失敗しました")
                }
            }
        }
    }

    fun handleLineLoginResult(data: Intent?) {
        viewModelScope.launch {
            try {
                val accessToken = LineAuthHelper.getAccessTokenFromResult(data)
                completeSocialLogin("line", null, accessToken)
            } catch (e: Exception) {
                _uiState.update {
                    it.copy(isSocialLoading = false, error = e.message ?: "LINEログインに失敗しました")
                }
            }
        }
    }

    fun cancelLineLogin() {
        _uiState.update { it.copy(isSocialLoading = false) }
    }

    fun setSocialLoading(loading: Boolean) {
        _uiState.update { it.copy(isSocialLoading = loading, error = null) }
    }

    private suspend fun completeSocialLogin(provider: String, idToken: String?, accessToken: String?) {
        when (val result = authRepository.socialLogin(provider, idToken, accessToken)) {
            is Resource.Success -> {
                _uiState.update { it.copy(isSocialLoading = false) }
            }
            is Resource.Error -> {
                _uiState.update { it.copy(isSocialLoading = false, error = result.message) }
            }
            is Resource.Loading -> {}
        }
    }

    fun toggleMode() {
        _uiState.update { it.copy(isLoginMode = !it.isLoginMode, error = null) }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
