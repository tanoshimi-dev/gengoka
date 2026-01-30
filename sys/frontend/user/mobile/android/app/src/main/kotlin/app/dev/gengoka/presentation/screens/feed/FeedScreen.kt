package app.dev.gengoka.presentation.screens.feed

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import app.dev.gengoka.presentation.components.*
import app.dev.gengoka.presentation.theme.*

@Composable
fun FeedScreen(
    onUserClick: (userId: String) -> Unit,
    onAnswerClick: (answerId: String) -> Unit,
    viewModel: FeedViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsState()
    val listState = rememberLazyListState()

    // Load more when reaching end
    val shouldLoadMore = remember {
        derivedStateOf {
            val lastVisibleItem = listState.layoutInfo.visibleItemsInfo.lastOrNull()
            lastVisibleItem?.index != null && lastVisibleItem.index >= uiState.items.size - 3
        }
    }

    LaunchedEffect(shouldLoadMore.value) {
        if (shouldLoadMore.value && uiState.hasMore && !uiState.isLoadingMore) {
            viewModel.loadMore()
        }
    }

    GradientBackground {
        Column(
            modifier = Modifier.fillMaxSize()
        ) {
            // Header
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(20.dp),
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Spacer(modifier = Modifier.height(20.dp))
                Text(
                    text = "みんなの回答",
                    style = MaterialTheme.typography.headlineMedium,
                    color = TextWhite
                )
                Spacer(modifier = Modifier.height(16.dp))

                // Filter tabs
                val filterTabs = buildList {
                    add(FilterTab("all", "すべて"))
                    add(FilterTab("following", "フォロー中"))
                    uiState.categories.forEach { category ->
                        add(FilterTab("category_${category.id}", category.name))
                    }
                }

                FilterTabs(
                    tabs = filterTabs,
                    selectedTabId = uiState.selectedFilter,
                    onTabSelected = viewModel::selectFilter
                )
            }

            // Feed list
            if (uiState.isLoading && uiState.items.isEmpty()) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    LoadingIndicator(message = "読み込み中...")
                }
            } else {
                LazyColumn(
                    state = listState,
                    modifier = Modifier
                        .fillMaxSize()
                        .padding(horizontal = 20.dp),
                    contentPadding = PaddingValues(bottom = 100.dp),
                    verticalArrangement = Arrangement.spacedBy(16.dp)
                ) {
                    items(uiState.items, key = { it.id }) { answer ->
                        FeedCard(
                            answer = answer,
                            onUserClick = { onUserClick(answer.user.id) },
                            onLikeClick = { viewModel.toggleLike(answer.id) },
                            onCommentClick = { onAnswerClick(answer.id) }
                        )
                    }

                    if (uiState.isLoadingMore) {
                        item {
                            Box(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(16.dp),
                                contentAlignment = Alignment.Center
                            ) {
                                CircularProgressIndicator(
                                    color = TextWhite,
                                    strokeWidth = 2.dp,
                                    modifier = Modifier.size(24.dp)
                                )
                            }
                        }
                    }
                }
            }
        }

        // Error handling
        uiState.error?.let { error ->
            Snackbar(
                modifier = Modifier
                    .padding(16.dp)
                    .align(Alignment.BottomCenter),
                action = {
                    TextButton(onClick = viewModel::clearError) {
                        Text("閉じる")
                    }
                }
            ) {
                Text(error)
            }
        }
    }
}
