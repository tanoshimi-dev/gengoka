package app.dev.gengoka.presentation.screens.myprofile

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.dev.gengoka.core.network.UserIdProvider
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

data class MyProfileUiState(
    val profile: UserProfile? = null,
    val answers: List<AnswerWithDetails> = emptyList(),
    val isLoading: Boolean = false,
    val isLoadingAnswers: Boolean = false,
    val error: String? = null,
    val selectedTab: Int = 0
)

@HiltViewModel
class MyProfileViewModel @Inject constructor(
    private val userRepository: UserRepository,
    private val userIdProvider: UserIdProvider
) : ViewModel() {

    private val _uiState = MutableStateFlow(MyProfileUiState())
    val uiState: StateFlow<MyProfileUiState> = _uiState.asStateFlow()

    init {
        loadProfile()
    }

    fun loadProfile() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }

            val userId = userIdProvider.getUserIdAsync()

            when (val result = userRepository.getUser(userId)) {
                is Resource.Success -> {
                    _uiState.update { it.copy(profile = result.data, isLoading = false) }
                    loadAnswers(userId)
                }
                is Resource.Error -> {
                    // User might not exist yet, create default profile
                    _uiState.update {
                        it.copy(
                            profile = UserProfile(
                                id = userId,
                                name = "あなた",
                                avatar = null,
                                bio = "プロフィールを編集して自己紹介を追加しましょう",
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
        }
    }

    private fun loadAnswers(userId: String) {
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

    fun selectTab(index: Int) {
        _uiState.update { it.copy(selectedTab = index) }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
