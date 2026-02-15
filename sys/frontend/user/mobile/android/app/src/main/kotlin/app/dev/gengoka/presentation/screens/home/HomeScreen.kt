package app.dev.gengoka.presentation.screens.home

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowForward
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import app.dev.gengoka.domain.model.ChallengeWithCategory
import app.dev.gengoka.presentation.components.*
import app.dev.gengoka.presentation.theme.*

@Composable
fun HomeScreen(
    onCategoryClick: (categoryId: String, categoryName: String) -> Unit,
    onDailyChallengeClick: (challengeId: String, categoryId: String, categoryName: String) -> Unit = { _, _, _ -> },
    viewModel: HomeViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsState()

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(
                brush = Brush.verticalGradient(
                    colors = listOf(
                        BackgroundLightGradientStart,
                        BackgroundLightGradientEnd
                    )
                )
            )
    ) {
        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = PaddingValues(bottom = 100.dp)
        ) {
            // Header
            item {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 20.dp)
                        .padding(top = 56.dp, bottom = 8.dp)
                ) {
                    Text(
                        text = "おはようございます",
                        style = MaterialTheme.typography.bodyMedium,
                        color = TextSecondary
                    )
                    Spacer(modifier = Modifier.height(4.dp))
                    Text(
                        text = "今日も言葉を磨こう",
                        style = MaterialTheme.typography.headlineMedium.copy(
                            fontWeight = FontWeight.Bold
                        ),
                        color = TextDarkPrimary
                    )
                }
            }

            // Stats Bar
            item {
                StatsBar(
                    stats = listOf(
                        StatItemData(
                            icon = Icons.Default.LocalFireDepartment,
                            value = uiState.userStats.currentStreak.toString(),
                            label = "連続",
                            color = StatsOrange
                        ),
                        StatItemData(
                            icon = Icons.Default.CheckCircle,
                            value = uiState.userStats.completedToday.toString(),
                            label = "今日",
                            color = StatsGreen
                        ),
                        StatItemData(
                            icon = Icons.Default.Star,
                            value = "%.0f".format(uiState.userStats.averageScore),
                            label = "平均点",
                            color = StatsYellow
                        ),
                        StatItemData(
                            icon = Icons.Default.Description,
                            value = uiState.userStats.totalChallenges.toString(),
                            label = "総数",
                            color = StatsBlue
                        )
                    ),
                    modifier = Modifier.padding(horizontal = 20.dp, vertical = 12.dp)
                )
            }

            // Daily Challenges Section
            if (uiState.dailyChallenges.isNotEmpty()) {
                item {
                    DailyChallengesSection(
                        challenges = uiState.dailyChallenges,
                        incompleteCount = uiState.incompleteCount,
                        onChallengeClick = { challenge ->
                            val categoryId = challenge.category?.id ?: challenge.categoryId
                            val categoryName = challenge.category?.name ?: ""
                            onDailyChallengeClick(challenge.id, categoryId, categoryName)
                        }
                    )
                }
            }

            // Categories Section Header
            item {
                Text(
                    text = "カテゴリーを選択",
                    style = MaterialTheme.typography.titleMedium.copy(
                        fontWeight = FontWeight.Bold
                    ),
                    color = TextDarkPrimary,
                    modifier = Modifier.padding(horizontal = 20.dp, vertical = 12.dp)
                )
            }

            // Loading
            if (uiState.isLoading) {
                item {
                    Box(
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(200.dp),
                        contentAlignment = Alignment.Center
                    ) {
                        CircularProgressIndicator(color = PrimaryPurple)
                    }
                }
            }

            // Categories 2-column grid
            val categories = uiState.categories
            val rows = categories.chunked(2)
            items(rows.size) { rowIndex ->
                val rowItems = rows[rowIndex]
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 20.dp)
                        .padding(bottom = 12.dp),
                    horizontalArrangement = Arrangement.spacedBy(12.dp)
                ) {
                    rowItems.forEach { category ->
                        CategoryCard(
                            category = category,
                            onClick = { onCategoryClick(category.id, category.name) },
                            modifier = Modifier.weight(1f)
                        )
                    }
                    if (rowItems.size == 1) {
                        Spacer(modifier = Modifier.weight(1f))
                    }
                }
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

@Composable
private fun DailyChallengesSection(
    challenges: List<ChallengeWithCategory>,
    incompleteCount: Int,
    onChallengeClick: (ChallengeWithCategory) -> Unit
) {
    Column(
        modifier = Modifier.padding(vertical = 8.dp)
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 20.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            Icon(
                imageVector = Icons.Default.LocalFireDepartment,
                contentDescription = null,
                tint = StatsOrange,
                modifier = Modifier.size(22.dp)
            )
            Spacer(modifier = Modifier.width(6.dp))
            Text(
                text = "今日のチャレンジ",
                style = MaterialTheme.typography.titleMedium.copy(
                    fontWeight = FontWeight.Bold
                ),
                color = TextDarkPrimary
            )
            Spacer(modifier = Modifier.width(8.dp))
            if (incompleteCount > 0) {
                Box(
                    modifier = Modifier
                        .clip(RoundedCornerShape(12.dp))
                        .background(StatsOrange.copy(alpha = 0.15f))
                        .padding(horizontal = 8.dp, vertical = 2.dp)
                ) {
                    Text(
                        text = "残り$incompleteCount",
                        style = MaterialTheme.typography.labelSmall.copy(
                            fontWeight = FontWeight.Medium
                        ),
                        color = StatsOrange
                    )
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

        LazyRow(
            contentPadding = PaddingValues(horizontal = 20.dp),
            horizontalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(challenges) { challenge ->
                DailyChallengeCard(
                    challenge = challenge,
                    onClick = { onChallengeClick(challenge) }
                )
            }
        }
    }
}

@Composable
private fun DailyChallengeCard(
    challenge: ChallengeWithCategory,
    onClick: () -> Unit
) {
    val categoryName = challenge.category?.name ?: ""
    val gradientColors = getDailyChallengeGradient(categoryName)

    Surface(
        modifier = Modifier
            .width(260.dp)
            .clickable(onClick = onClick),
        shape = RoundedCornerShape(16.dp),
        shadowElevation = 2.dp
    ) {
        Box(
            modifier = Modifier
                .background(
                    brush = Brush.linearGradient(colors = gradientColors)
                )
                .padding(16.dp)
        ) {
            Column {
                // Category badge and completion indicator
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Box(
                        modifier = Modifier
                            .clip(RoundedCornerShape(8.dp))
                            .background(SurfaceWhite.copy(alpha = 0.25f))
                            .padding(horizontal = 8.dp, vertical = 4.dp)
                    ) {
                        Text(
                            text = categoryName,
                            style = MaterialTheme.typography.labelSmall.copy(
                                fontWeight = FontWeight.Medium
                            ),
                            color = SurfaceWhite
                        )
                    }

                    if (challenge.isCompleted) {
                        Icon(
                            imageVector = Icons.Default.CheckCircle,
                            contentDescription = "回答済み",
                            tint = SurfaceWhite,
                            modifier = Modifier.size(24.dp)
                        )
                    }
                }

                Spacer(modifier = Modifier.height(12.dp))

                Text(
                    text = challenge.title,
                    style = MaterialTheme.typography.titleSmall.copy(
                        fontWeight = FontWeight.Bold
                    ),
                    color = SurfaceWhite,
                    maxLines = 2
                )

                Spacer(modifier = Modifier.height(8.dp))

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = "${challenge.charLimit}文字以内",
                        style = MaterialTheme.typography.bodySmall,
                        color = SurfaceWhite.copy(alpha = 0.8f)
                    )
                    Icon(
                        imageVector = Icons.AutoMirrored.Filled.ArrowForward,
                        contentDescription = null,
                        tint = SurfaceWhite.copy(alpha = 0.8f),
                        modifier = Modifier.size(18.dp)
                    )
                }
            }
        }
    }
}

private fun getDailyChallengeGradient(categoryName: String): List<Color> {
    return when (categoryName) {
        "状況描写" -> listOf(CategorySituationAccent, CategorySituationAccent.copy(alpha = 0.8f))
        "要約力" -> listOf(CategorySummaryAccent, CategorySummaryAccent.copy(alpha = 0.8f))
        "感性の言語化" -> listOf(CategoryEmotionAccent, CategoryEmotionAccent.copy(alpha = 0.8f))
        "言い換え" -> listOf(CategoryRephraseAccent, CategoryRephraseAccent.copy(alpha = 0.8f))
        "概念説明" -> listOf(CategoryExplainAccent, CategoryExplainAccent.copy(alpha = 0.8f))
        else -> listOf(PrimaryPurple, SecondaryPurple)
    }
}
