package app.dev.gengoka.presentation.screens.auth

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.repository.AuthRepository
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

    fun socialLogin(provider: String, idToken: String? = null, accessToken: String? = null) {
        viewModelScope.launch {
            _uiState.update { it.copy(isSocialLoading = true, error = null) }
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
    }

    fun toggleMode() {
        _uiState.update { it.copy(isLoginMode = !it.isLoginMode, error = null) }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
