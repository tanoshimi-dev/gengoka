package app.dev.gengoka.presentation.screens.result

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
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
fun ResultScreen(
    onBackClick: () -> Unit,
    onRetryClick: (categoryId: String, categoryName: String) -> Unit,
    viewModel: ResultViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsState()

    GradientBackground {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(20.dp)
        ) {
            // Back button
            TextButton(
                onClick = onBackClick,
                colors = ButtonDefaults.textButtonColors(contentColor = TextWhite)
            ) {
                Icon(Icons.Default.ArrowBack, contentDescription = null)
                Spacer(modifier = Modifier.width(6.dp))
                Text("ホームに戻る")
            }

            Spacer(modifier = Modifier.height(16.dp))

            if (uiState.isLoading) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    LoadingIndicator(message = "結果を読み込み中...")
                }
            } else {
                uiState.answer?.let { answer ->
                    Column(
                        modifier = Modifier
                            .fillMaxSize()
                            .verticalScroll(rememberScrollState())
                    ) {
                        // Result card
                        Surface(
                            modifier = Modifier
                                .fillMaxWidth()
                                .clip(RoundedCornerShape(20.dp)),
                            color = SurfaceWhite,
                            shadowElevation = 10.dp
                        ) {
                            Column(
                                modifier = Modifier.padding(24.dp)
                            ) {
                                // Score display
                                Box(
                                    modifier = Modifier
                                        .fillMaxWidth()
                                        .padding(vertical = 20.dp),
                                    contentAlignment = Alignment.Center
                                ) {
                                    ScoreDisplay(
                                        score = answer.score ?: 0
                                    )
                                }

                                // User's answer section
                                Text(
                                    text = "あなたの回答",
                                    style = MaterialTheme.typography.titleSmall,
                                    color = TextTertiary
                                )
                                Spacer(modifier = Modifier.height(8.dp))
                                Box(
                                    modifier = Modifier
                                        .fillMaxWidth()
                                        .clip(RoundedCornerShape(12.dp))
                                        .background(SurfaceGray)
                                        .padding(16.dp)
                                ) {
                                    Text(
                                        text = answer.content,
                                        style = MaterialTheme.typography.bodyLarge,
                                        color = TextPrimary
                                    )
                                }

                                // AI Feedback
                                answer.aiFeedback?.let { feedback ->
                                    Spacer(modifier = Modifier.height(16.dp))

                                    FeedbackCard(
                                        type = FeedbackType.GOOD,
                                        title = "良かった点",
                                        content = feedback.goodPoints
                                    )

                                    Spacer(modifier = Modifier.height(12.dp))

                                    FeedbackCard(
                                        type = FeedbackType.IMPROVEMENT,
                                        title = "改善ポイント",
                                        content = feedback.improvement
                                    )

                                    Spacer(modifier = Modifier.height(12.dp))

                                    FeedbackCard(
                                        type = FeedbackType.EXAMPLE,
                                        title = "お手本回答",
                                        content = feedback.exampleAnswer
                                    )
                                }

                                Spacer(modifier = Modifier.height(24.dp))

                                // Action buttons
                                Row(
                                    modifier = Modifier.fillMaxWidth(),
                                    horizontalArrangement = Arrangement.spacedBy(12.dp)
                                ) {
                                    OutlinedButton(
                                        onClick = onBackClick,
                                        modifier = Modifier.weight(1f),
                                        shape = RoundedCornerShape(12.dp),
                                        colors = ButtonDefaults.outlinedButtonColors(
                                            contentColor = TextSecondary
                                        )
                                    ) {
                                        Text(
                                            text = "別のお題へ",
                                            modifier = Modifier.padding(vertical = 8.dp)
                                        )
                                    }

                                    Button(
                                        onClick = {
                                            onRetryClick(
                                                answer.challenge.categoryId,
                                                answer.challenge.title
                                            )
                                        },
                                        modifier = Modifier.weight(1f),
                                        shape = RoundedCornerShape(12.dp),
                                        colors = ButtonDefaults.buttonColors(
                                            containerColor = PrimaryPurple
                                        )
                                    ) {
                                        Text(
                                            text = "もう一度挑戦",
                                            modifier = Modifier.padding(vertical = 8.dp)
                                        )
                                    }
                                }
                            }
                        }

                        Spacer(modifier = Modifier.height(100.dp))
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
