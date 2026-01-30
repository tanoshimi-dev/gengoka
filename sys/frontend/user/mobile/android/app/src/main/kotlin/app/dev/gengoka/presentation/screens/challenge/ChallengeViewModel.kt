package app.dev.gengoka.presentation.screens.challenge

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.AnswerWithUser
import app.dev.gengoka.domain.model.ChallengeWithCategory
import app.dev.gengoka.domain.repository.AnswerRepository
import app.dev.gengoka.domain.repository.ChallengeRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class ChallengeUiState(
    val challenge: ChallengeWithCategory? = null,
    val answerText: String = "",
    val isLoading: Boolean = false,
    val isSubmitting: Boolean = false,
    val error: String? = null,
    val submittedAnswer: AnswerWithUser? = null
)

@HiltViewModel
class ChallengeViewModel @Inject constructor(
    savedStateHandle: SavedStateHandle,
    private val challengeRepository: ChallengeRepository,
    private val answerRepository: AnswerRepository
) : ViewModel() {

    private val categoryId: String = savedStateHandle["categoryId"] ?: ""
    private val categoryName: String = savedStateHandle["categoryName"] ?: ""

    private val _uiState = MutableStateFlow(ChallengeUiState())
    val uiState: StateFlow<ChallengeUiState> = _uiState.asStateFlow()

    init {
        loadChallenge()
    }

    private fun loadChallenge() {
        viewModelScope.launch {
            _uiState.update { it.copy(isLoading = true, error = null) }

            when (val result = challengeRepository.getChallengesByCategory(categoryId, 1, 1)) {
                is Resource.Success -> {
                    val challenge = result.data.firstOrNull()
                    _uiState.update { it.copy(challenge = challenge, isLoading = false) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(error = result.message, isLoading = false) }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun updateAnswerText(text: String) {
        _uiState.update { it.copy(answerText = text) }
    }

    fun submitAnswer() {
        val challenge = _uiState.value.challenge ?: return
        val content = _uiState.value.answerText

        if (content.isEmpty() || content.length > challenge.charLimit) return

        viewModelScope.launch {
            _uiState.update { it.copy(isSubmitting = true, error = null) }

            when (val result = answerRepository.createAnswer(challenge.id, content)) {
                is Resource.Success -> {
                    _uiState.update { it.copy(submittedAnswer = result.data, isSubmitting = false) }
                }
                is Resource.Error -> {
                    _uiState.update { it.copy(error = result.message, isSubmitting = false) }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun retryChallenge() {
        _uiState.update { it.copy(answerText = "", submittedAnswer = null) }
        loadChallenge()
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
