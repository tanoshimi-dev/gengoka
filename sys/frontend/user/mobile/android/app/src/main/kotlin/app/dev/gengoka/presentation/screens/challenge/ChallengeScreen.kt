package app.dev.gengoka.presentation.screens.challenge

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import app.dev.gengoka.presentation.components.*
import app.dev.gengoka.presentation.theme.*

@Composable
fun ChallengeScreen(
    onBackClick: () -> Unit,
    onAnswerSubmitted: (answerId: String) -> Unit,
    viewModel: ChallengeViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsState()

    // Navigate to result when answer is submitted
    uiState.submittedAnswer?.let { answer ->
        onAnswerSubmitted(answer.id)
        return
    }

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
                Text("カテゴリー選択に戻る")
            }

            Spacer(modifier = Modifier.height(16.dp))

            if (uiState.isLoading) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    LoadingIndicator(message = "お題を読み込み中...")
                }
            } else {
                uiState.challenge?.let { challenge ->
                    // Challenge card
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
                            // Category badge
                            Box(
                                modifier = Modifier
                                    .clip(RoundedCornerShape(20.dp))
                                    .background(
                                        brush = Brush.linearGradient(
                                            colors = listOf(BackgroundGradientStart, BackgroundGradientEnd)
                                        )
                                    )
                                    .padding(horizontal = 14.dp, vertical = 6.dp)
                            ) {
                                Text(
                                    text = challenge.category.name,
                                    style = MaterialTheme.typography.labelMedium,
                                    color = TextWhite
                                )
                            }

                            Spacer(modifier = Modifier.height(16.dp))

                            // Challenge title
                            Text(
                                text = challenge.title,
                                style = MaterialTheme.typography.titleLarge,
                                color = TextPrimary,
                                lineHeight = MaterialTheme.typography.titleLarge.lineHeight * 1.4
                            )

                            Spacer(modifier = Modifier.height(16.dp))

                            // Character limit info
                            Row(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .clip(RoundedCornerShape(12.dp))
                                    .background(SurfaceGray)
                                    .padding(12.dp, 12.dp),
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Icon(
                                    Icons.Default.Edit,
                                    contentDescription = null,
                                    tint = TextSecondary,
                                    modifier = Modifier.size(20.dp)
                                )
                                Spacer(modifier = Modifier.width(8.dp))
                                Text(
                                    text = "${challenge.charLimit}文字以内で回答",
                                    style = MaterialTheme.typography.bodyMedium,
                                    color = TextSecondary
                                )
                            }

                            Spacer(modifier = Modifier.height(20.dp))

                            // Answer input
                            CharacterCounterTextField(
                                value = uiState.answerText,
                                onValueChange = viewModel::updateAnswerText,
                                charLimit = challenge.charLimit,
                                placeholder = "ここに回答を入力..."
                            )

                            Spacer(modifier = Modifier.height(16.dp))

                            // Submit button
                            val isValid = uiState.answerText.isNotEmpty() &&
                                uiState.answerText.length <= challenge.charLimit

                            GradientButton(
                                text = "回答を送信",
                                onClick = viewModel::submitAnswer,
                                enabled = isValid,
                                isLoading = uiState.isSubmitting
                            )
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
