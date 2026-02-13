package app.dev.gengoka.presentation.screens.home

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.Category
import app.dev.gengoka.domain.model.ChallengeWithCategory
import app.dev.gengoka.domain.model.UserStats
import app.dev.gengoka.domain.repository.CategoryRepository
import app.dev.gengoka.domain.repository.ChallengeRepository
import app.dev.gengoka.domain.repository.UserRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class HomeUiState(
    val categories: List<Category> = emptyList(),
    val dailyChallenges: List<ChallengeWithCategory> = emptyList(),
    val isLoading: Boolean = false,
    val isRefreshing: Boolean = false,
    val error: String? = null,
    val userStats: UserStats = UserStats(
        totalChallenges = 0,
        completedToday = 0,
        currentStreak = 0,
        bestStreak = 0,
        averageScore = 0.0
    )
) {
    val incompleteCount: Int
        get() = dailyChallenges.count { !it.isCompleted }
}

@HiltViewModel
class HomeViewModel @Inject constructor(
    private val categoryRepository: CategoryRepository,
    private val challengeRepository: ChallengeRepository,
    private val userRepository: UserRepository
) : ViewModel() {

    private val _uiState = MutableStateFlow(HomeUiState())
    val uiState: StateFlow<HomeUiState> = _uiState.asStateFlow()

    init {
        loadData()
    }

    fun loadData() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }

            // Load categories
            when (val result = categoryRepository.getCategories()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(categories = result.data) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(error = result.message) }
                }
                is Resource.Loading -> {}
            }

            // Load daily challenges
            when (val result = challengeRepository.getDailyChallenges()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(dailyChallenges = result.data) }
                }
                is Resource.Error -> {}
                is Resource.Loading -> {}
            }

            // Load user stats
            when (val result = userRepository.getUserStats()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(userStats = result.data) }
                }
                is Resource.Error -> {}
                is Resource.Loading -> {}
            }

            _uiState.update { it.copy(isLoading = false) }
        }
    }

    fun refresh() {
        viewModelScope.launch {
            _uiState.update { it.copy(isRefreshing = true) }

            when (val result = categoryRepository.getCategories()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(categories = result.data) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(error = result.message) }
                }
                is Resource.Loading -> {}
            }

            when (val result = challengeRepository.getDailyChallenges()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(dailyChallenges = result.data) }
                }
                is Resource.Error -> {}
                is Resource.Loading -> {}
            }

            when (val result = userRepository.getUserStats()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(userStats = result.data) }
                }
                is Resource.Error -> {}
                is Resource.Loading -> {}
            }

            _uiState.update { it.copy(isRefreshing = false) }
        }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
