package app.dev.gengoka.presentation.screens.home

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.Category
import app.dev.gengoka.domain.model.ChallengeWithCategory
import app.dev.gengoka.domain.repository.CategoryRepository
import app.dev.gengoka.domain.repository.ChallengeRepository
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
    val error: String? = null,
    val todayAnswers: Int = 0,
    val averageScore: Int = 0,
    val streak: Int = 0
)

@HiltViewModel
class HomeViewModel @Inject constructor(
    private val categoryRepository: CategoryRepository,
    private val challengeRepository: ChallengeRepository
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
                    _uiState.update { it.copy(categories = result.data.sortedBy { c -> c.sortOrder }) }
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
                is Resource.Error -> {
                    // Non-critical error, don't update error state
                }
                is Resource.Loading -> {}
            }

            _uiState.update { it.copy(isLoading = false) }
        }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
