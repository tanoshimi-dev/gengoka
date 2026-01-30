package app.dev.gengoka.presentation.screens.profile

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.AnswerWithDetails
import app.dev.gengoka.domain.model.UserProfile
import app.dev.gengoka.domain.repository.UserRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class ProfileUiState(
    val profile: UserProfile? = null,
    val answers: List<AnswerWithDetails> = emptyList(),
    val isLoading: Boolean = false,
    val isLoadingAnswers: Boolean = false,
    val error: String? = null,
    val selectedTab: Int = 0
)

@HiltViewModel
class ProfileViewModel @Inject constructor(
    savedStateHandle: SavedStateHandle,
    private val userRepository: UserRepository
) : ViewModel() {

    private val userId: String = savedStateHandle["userId"] ?: ""

    private val _uiState = MutableStateFlow(ProfileUiState())
    val uiState: StateFlow<ProfileUiState> = _uiState.asStateFlow()

    init {
        loadProfile()
        loadAnswers()
    }

    private fun loadProfile() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }

            when (val result = userRepository.getUser(userId)) {
                is Resource.Success -> {
                    _uiState.update { it.copy(profile = result.data, isLoading = false) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(error = result.message, isLoading = false) }
                }
                is Resource.Loading -> {}
            }
        }
    }

    private fun loadAnswers() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoadingAnswers = true) }

            when (val result = userRepository.getUserAnswers(userId)) {
                is Resource.Success -> {
                    _uiState.update { it.copy(answers = result.data, isLoadingAnswers = false) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(isLoadingAnswers = false) }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun toggleFollow() {
        val profile = _uiState.value.profile ?: return
        val isFollowing = profile.isFollowing

        viewModelScope.launch {
            // Optimistic update
            _uiState.update { state ->
                state.copy(
                    profile = state.profile?.copy(
                        isFollowing = !isFollowing,
                        followerCount = if (isFollowing) profile.followerCount - 1 else profile.followerCount + 1
                    )
                )
            }

            // API call
            val result = if (isFollowing) {
                userRepository.unfollowUser(userId)
            } else {
                userRepository.followUser(userId)
            }

            // Revert on error
            if (result is Resource.Error) {
                _uiState.update { state ->
                    state.copy(profile = profile)
                }
            }
        }
    }

    fun selectTab(index: Int) {
        _uiState.update { it.copy(selectedTab = index) }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
