package app.gengoka.presentation.screens.linkedaccounts

import android.content.Context
import android.content.Intent
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.gengoka.core.auth.GoogleAuthHelper
import app.gengoka.core.auth.LineAuthHelper
import app.gengoka.core.util.Resource
import app.gengoka.domain.model.LinkedSocialAccount
import app.gengoka.domain.repository.AuthRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class LinkedAccountsUiState(
    val accounts: List<LinkedSocialAccount> = emptyList(),
    val isLoading: Boolean = false,
    val isLinking: Boolean = false,
    val error: String? = null,
    val unlinkTarget: String? = null
)

@HiltViewModel
class LinkedAccountsViewModel @Inject constructor(
    private val authRepository: AuthRepository
) : ViewModel() {

    private val _uiState = MutableStateFlow(LinkedAccountsUiState())
    val uiState: StateFlow<LinkedAccountsUiState> = _uiState.asStateFlow()

    init {
        loadAccounts()
    }

    fun loadAccounts() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }
            when (val result = authRepository.getLinkedAccounts()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(isLoading = false, accounts = result.data) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(isLoading = false, error = result.message) }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun linkAccount(provider: String, context: Context) {
        viewModelScope.launch {
            _uiState.update { it.copy(isLinking = true, error = null) }
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
                completeLinkAccount(provider, idToken, accessToken)
            } catch (e: Exception) {
                _uiState.update {
                    it.copy(isLinking = false, error = e.message ?: "アカウント連携に失敗しました")
                }
            }
        }
    }

    fun handleLineLoginResult(data: Intent?) {
        viewModelScope.launch {
            try {
                val accessToken = LineAuthHelper.getAccessTokenFromResult(data)
                completeLinkAccount("line", null, accessToken)
            } catch (e: Exception) {
                _uiState.update {
                    it.copy(isLinking = false, error = e.message ?: "LINE連携に失敗しました")
                }
            }
        }
    }

    fun cancelLineLogin() {
        _uiState.update { it.copy(isLinking = false) }
    }

    fun setLinking(linking: Boolean) {
        _uiState.update { it.copy(isLinking = linking, error = null) }
    }

    private suspend fun completeLinkAccount(provider: String, idToken: String?, accessToken: String?) {
        when (val result = authRepository.linkAccount(provider, idToken, accessToken)) {
            is Resource.Success -> {
                _uiState.update {
                    it.copy(
                        isLinking = false,
                        accounts = it.accounts + result.data
                    )
                }
            }
            is Resource.Error -> {
                _uiState.update { it.copy(isLinking = false, error = result.message) }
            }
            is Resource.Loading -> {}
        }
    }

    fun showUnlinkDialog(provider: String) {
        _uiState.update { it.copy(unlinkTarget = provider) }
    }

    fun dismissUnlinkDialog() {
        _uiState.update { it.copy(unlinkTarget = null) }
    }

    fun confirmUnlink() {
        val provider = _uiState.value.unlinkTarget ?: return
        viewModelScope.launch {
            _uiState.update { it.copy(unlinkTarget = null, error = null) }
            when (val result = authRepository.unlinkAccount(provider)) {
                is Resource.Success -> {
                    _uiState.update {
                        it.copy(accounts = it.accounts.filter { a -> a.provider != provider })
                    }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(error = result.message) }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
