package app.dev.gengoka.presentation.screens.feed

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.AnswerWithDetails
import app.dev.gengoka.domain.model.Category
import app.dev.gengoka.domain.repository.AnswerRepository
import app.dev.gengoka.domain.repository.CategoryRepository
import app.dev.gengoka.domain.repository.FeedRepository
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

data class FeedUiState(
    val items: List<AnswerWithDetails> = emptyList(),
    val categories: List<Category> = emptyList(),
    val selectedFilter: String = "all",
    val isLoading: Boolean = false,
    val isLoadingMore: Boolean = false,
    val error: String? = null,
    val page: Int = 1,
    val hasMore: Boolean = true
)

@HiltViewModel
class FeedViewModel @Inject constructor(
    private val feedRepository: FeedRepository,
    private val categoryRepository: CategoryRepository,
    private val answerRepository: AnswerRepository
) : ViewModel() {

    private val _uiState = MutableStateFlow(FeedUiState())
    val uiState: StateFlow<FeedUiState> = _uiState.asStateFlow()

    init {
        loadCategories()
        loadFeed()
    }

    private fun loadCategories() {
        viewModelScope.launch {
            when (val result = categoryRepository.getCategories()) {
                is Resource.Success -> {
                    _uiState.update { it.copy(categories = result.data) }
                }
                else -> {}
            }
        }
    }

    fun loadFeed(refresh: Boolean = false) {
        if (_uiState.value.isLoading) return

        viewModelScope.launch {
            val page = if (refresh) 1 else _uiState.value.page
            _uiState.update {
                it.copy(
                    isLoading = refresh || page == 1,
                    isLoadingMore = !refresh && page > 1,
                    error = null
                )
            }

            val filter = _uiState.value.selectedFilter
            val categoryId = if (filter.startsWith("category_")) filter.removePrefix("category_") else null

            when (val result = feedRepository.getFeed(
                page = page,
                pageSize = 20,
                filter = if (filter == "following") "following" else null,
                categoryId = categoryId
            )) {
                is Resource.Success -> {
                    val newItems = if (refresh || page == 1) {
                        result.data
                    } else {
                        _uiState.value.items + result.data
                    }
                    _uiState.update {
                        it.copy(
                            items = newItems,
                            page = page + 1,
                            hasMore = result.data.size >= 20,
                            isLoading = false,
                            isLoadingMore = false
                        )
                    }
                }
                is Resource.Error -> {
                    _uiState.update {
                        it.copy(
                            error = result.message,
                            isLoading = false,
                            isLoadingMore = false
                        )
                    }
                }
                is Resource.Loading -> {}
            }
        }
    }

    fun loadMore() {
        if (!_uiState.value.hasMore || _uiState.value.isLoadingMore) return
        loadFeed(refresh = false)
    }

    fun selectFilter(filterId: String) {
        if (filterId == _uiState.value.selectedFilter) return
        _uiState.update { it.copy(selectedFilter = filterId, page = 1, items = emptyList()) }
        loadFeed(refresh = true)
    }

    fun toggleLike(answerId: String) {
        viewModelScope.launch {
            val answer = _uiState.value.items.find { it.id == answerId } ?: return@launch
            val isLiked = answer.isLiked

            // Optimistic update
            _uiState.update { state ->
                state.copy(
                    items = state.items.map {
                        if (it.id == answerId) {
                            it.copy(
                                isLiked = !isLiked,
                                likeCount = if (isLiked) it.likeCount - 1 else it.likeCount + 1
                            )
                        } else it
                    }
                )
            }

            // API call
            val result = if (isLiked) {
                answerRepository.unlikeAnswer(answerId)
            } else {
                answerRepository.likeAnswer(answerId)
            }

            // Revert on error
            if (result is Resource.Error) {
                _uiState.update { state ->
                    state.copy(
                        items = state.items.map {
                            if (it.id == answerId) {
                                it.copy(
                                    isLiked = isLiked,
                                    likeCount = answer.likeCount
                                )
                            } else it
                        }
                    )
                }
            }
        }
    }

    fun clearError() {
        _uiState.update { it.copy(error = null) }
    }
}
