package app.dev.gengoka.presentation.screens.home

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import app.dev.gengoka.presentation.components.*
import app.dev.gengoka.presentation.theme.*

@Composable
fun HomeScreen(
    onCategoryClick: (categoryId: String, categoryName: String) -> Unit,
    viewModel: HomeViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsState()

    GradientBackground {
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(horizontal = 20.dp),
            contentPadding = PaddingValues(bottom = 100.dp)
        ) {
            // Header
            item {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(vertical = 40.dp),
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Text(
                        text = "ゲンゴカ",
                        style = MaterialTheme.typography.headlineLarge,
                        color = TextWhite
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(
                        text = "言語化力を鍛えるトレーニング",
                        style = MaterialTheme.typography.bodyMedium,
                        color = TextWhiteAlpha85
                    )
                }
            }

            // Categories card
            item {
                Surface(
                    modifier = Modifier
                        .fillMaxWidth()
                        .clip(RoundedCornerShape(20.dp)),
                    color = SurfaceWhite,
                    shadowElevation = 10.dp
                ) {
                    Column(
                        modifier = Modifier.padding(24.dp),
                        verticalArrangement = Arrangement.spacedBy(12.dp)
                    ) {
                        if (uiState.isLoading) {
                            Box(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .height(200.dp),
                                contentAlignment = Alignment.Center
                            ) {
                                CircularProgressIndicator(color = PrimaryPurple)
                            }
                        } else {
                            uiState.categories.forEach { category ->
                                CategoryCard(
                                    category = category,
                                    onClick = { onCategoryClick(category.id, category.name) }
                                )
                            }
                        }
                    }
                }
            }

            // Stats bar
            item {
                Spacer(modifier = Modifier.height(24.dp))
                StatsBar(
                    stats = listOf(
                        StatItem(uiState.todayAnswers.toString(), "今日の回答"),
                        StatItem(uiState.averageScore.toString(), "平均スコア"),
                        StatItem(uiState.streak.toString(), "連続日数")
                    )
                )
            }
        }

        // Error snackbar
        uiState.error?.let { error ->
            Snackbar(
                modifier = Modifier
                    .padding(16.dp)
                    .align(Alignment.BottomCenter),
                action = {
                    TextButton(onClick = { viewModel.clearError() }) {
                        Text("閉じる")
                    }
                }
            ) {
                Text(error)
            }
        }
    }
}
