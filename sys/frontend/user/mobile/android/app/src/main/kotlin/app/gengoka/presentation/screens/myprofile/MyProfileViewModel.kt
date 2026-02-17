package app.gengoka.presentation.screens.myprofile

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.gengoka.core.network.TokenManager
import app.gengoka.core.util.Resource
import app.gengoka.domain.model.UserProfile
import app.gengoka.domain.model.UserStats
import app.gengoka.domain.repository.AuthRepository
import app.gengoka.domain.repository.UserRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class MyProfileUiState(
    val profile: UserProfile? = null,
    val stats: UserStats? = null,
    val isLoading: Boolean = false,
    val error: String? = null
)

@HiltViewModel
class MyProfileViewModel @Inject constructor(
    private val userRepository: UserRepository,
    private val authRepository: AuthRepository,
    private val tokenManager: TokenManager
) : ViewModel() {

    private val _uiState = MutableStateFlow(MyProfileUiState())
    val uiState: StateFlow<MyProfileUiState> = _uiState.asStateFlow()

    init {
        loadProfile()
    }

    fun loadProfile() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }

            val userId = tokenManager.getUserId()
            if (userId == null) {
                _uiState.update { it.copy(isLoading = false) }
                return@launch
            }

            when (val result = userRepository.getUser(userId)) {
                is Resource.Success -> {
                    _uiState.update { it.copy(profile = result.data, isLoading = false) }
                }
                is Resource.Error -> {
                    _uiState.update {
                        it.copy(
                            profile = UserProfile(
                                id = userId,
                                name = tokenManager.getUserName() ?: "あなた",
                                avatar = null,
                                bio = null,
                                totalLikes = 0,
                                answerCount = 0,
                                followerCount = 0,
                                followingCount = 0,
                                isFollowing = false
                            ),
                            isLoading = false
                        )
                    }
                }
                is Resource.Loading -> {}
            }

            // Load stats in parallel
            when (val statsResult = userRepository.getUserStats()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(stats = statsResult.data) }
                }
                is Resource.Error -> {
                    _uiState.update {
                        it.copy(stats = UserStats(0, 0, 0, 0, 0.0))
                    }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun logout() {
        viewModelScope.launch {
            authRepository.logout()
        }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
