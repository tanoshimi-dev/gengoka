package app.dev.gengoka.presentation.screens.profile

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
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
import app.dev.gengoka.core.util.formatCount
import app.dev.gengoka.presentation.components.*
import app.dev.gengoka.presentation.theme.*

@Composable
fun ProfileScreen(
    onBackClick: () -> Unit,
    onAnswerClick: (answerId: String) -> Unit,
    viewModel: ProfileViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsState()

    GradientBackground {
        Column(
            modifier = Modifier.fillMaxSize()
        ) {
            // Back button
            TextButton(
                onClick = onBackClick,
                colors = ButtonDefaults.textButtonColors(contentColor = TextWhite),
                modifier = Modifier.padding(start = 8.dp, top = 8.dp)
            ) {
                Icon(Icons.Default.ArrowBack, contentDescription = null)
                Spacer(modifier = Modifier.width(6.dp))
                Text("戻る")
            }

            if (uiState.isLoading) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    LoadingIndicator(message = "プロフィールを読み込み中...")
                }
            } else {
                uiState.profile?.let { profile ->
                    LazyColumn(
                        modifier = Modifier
                            .fillMaxSize()
                            .padding(horizontal = 20.dp),
                        contentPadding = PaddingValues(bottom = 100.dp)
                    ) {
                        // Profile header card
                        item {
                            Surface(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .clip(RoundedCornerShape(20.dp)),
                                color = SurfaceWhite
                            ) {
                                Column(
                                    modifier = Modifier.padding(24.dp),
                                    horizontalAlignment = Alignment.CenterHorizontally
                                ) {
                                    UserAvatar(
                                        name = profile.name,
                                        avatarUrl = profile.avatar,
                                        size = 80.dp
                                    )

                                    Spacer(modifier = Modifier.height(16.dp))

                                    Text(
                                        text = profile.name,
                                        style = MaterialTheme.typography.headlineSmall,
                                        color = TextPrimary
                                    )

                                    profile.bio?.let { bio ->
                                        Spacer(modifier = Modifier.height(8.dp))
                                        Text(
                                            text = bio,
                                            style = MaterialTheme.typography.bodyMedium,
                                            color = TextSecondary
                                        )
                                    }

                                    Spacer(modifier = Modifier.height(20.dp))

                                    // Stats
                                    Row(
                                        modifier = Modifier
                                            .fillMaxWidth()
                                            .padding(vertical = 16.dp),
                                        horizontalArrangement = Arrangement.SpaceAround
                                    ) {
                                        ProfileStat(
                                            value = profile.answerCount.formatCount(),
                                            label = "回答"
                                        )
                                        ProfileStat(
                                            value = profile.followerCount.formatCount(),
                                            label = "フォロワー"
                                        )
                                        ProfileStat(
                                            value = profile.followingCount.formatCount(),
                                            label = "フォロー中"
                                        )
                                    }

                                    Spacer(modifier = Modifier.height(20.dp))

                                    FollowButton(
                                        isFollowing = profile.isFollowing,
                                        onClick = viewModel::toggleFollow
                                    )
                                }
                            }
                        }

                        item { Spacer(modifier = Modifier.height(16.dp)) }

                        // Tabs
                        item {
                            ProfileTabs(
                                tabs = listOf("回答", "いいね"),
                                selectedIndex = uiState.selectedTab,
                                onTabSelected = viewModel::selectTab
                            )
                        }

                        item { Spacer(modifier = Modifier.height(16.dp)) }

                        // Answers list
                        if (uiState.isLoadingAnswers) {
                            item {
                                Box(
                                    modifier = Modifier
                                        .fillMaxWidth()
                                        .padding(32.dp),
                                    contentAlignment = Alignment.Center
                                ) {
                                    CircularProgressIndicator(color = PrimaryPurple)
                                }
                            }
                        } else {
                            items(uiState.answers) { answer ->
                                ProfileAnswerCard(
                                    answer = answer,
                                    onClick = { onAnswerClick(answer.id) }
                                )
                                Spacer(modifier = Modifier.height(16.dp))
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

@Composable
private fun ProfileStat(
    value: String,
    label: String
) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = value,
            style = MaterialTheme.typography.headlineSmall,
            color = TextPrimary
        )
        Spacer(modifier = Modifier.height(4.dp))
        Text(
            text = label,
            style = MaterialTheme.typography.bodySmall,
            color = TextTertiary
        )
    }
}

@Composable
private fun ProfileTabs(
    tabs: List<String>,
    selectedIndex: Int,
    onTabSelected: (Int) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(12.dp))
            .background(SurfaceWhite)
            .padding(4.dp)
    ) {
        tabs.forEachIndexed { index, tab ->
            val isSelected = index == selectedIndex
            Box(
                modifier = Modifier
                    .weight(1f)
                    .clip(RoundedCornerShape(8.dp))
                    .background(if (isSelected) PrimaryPurple else SurfaceWhite)
                    .clickable { onTabSelected(index) }
                    .padding(vertical = 12.dp),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = tab,
                    style = MaterialTheme.typography.labelLarge,
                    color = if (isSelected) TextWhite else TextTertiary
                )
            }
        }
    }
}

@Composable
private fun ProfileAnswerCard(
    answer: app.dev.gengoka.domain.model.AnswerWithDetails,
    onClick: () -> Unit
) {
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(16.dp))
            .clickable(onClick = onClick),
        color = SurfaceWhite
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            // Challenge info
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .clip(RoundedCornerShape(8.dp))
                    .background(SurfaceGray)
                    .padding(12.dp)
            ) {
                Box(
                    modifier = Modifier
                        .clip(RoundedCornerShape(10.dp))
                        .background(PrimaryPurple)
                        .padding(horizontal = 8.dp, vertical = 2.dp)
                ) {
                    Text(
                        text = answer.challenge.title.take(15),
                        style = MaterialTheme.typography.labelSmall,
                        color = TextWhite
                    )
                }
                Spacer(modifier = Modifier.height(6.dp))
                Text(
                    text = answer.challenge.title,
                    style = MaterialTheme.typography.bodySmall,
                    color = TextSecondary,
                    maxLines = 2
                )
            }

            Spacer(modifier = Modifier.height(12.dp))

            Text(
                text = answer.content,
                style = MaterialTheme.typography.bodyLarge,
                color = TextPrimary
            )

            Spacer(modifier = Modifier.height(12.dp))

            Row(
                verticalAlignment = Alignment.CenterVertically
            ) {
                answer.score?.let { score ->
                    ScoreText(score = score)
                    Spacer(modifier = Modifier.width(8.dp))
                    Text("・", color = TextTertiary)
                    Spacer(modifier = Modifier.width(8.dp))
                }
                Text(
                    text = app.dev.gengoka.core.util.DateUtils.formatRelativeTime(answer.createdAt),
                    style = MaterialTheme.typography.bodySmall,
                    color = TextTertiary
                )
            }
        }
    }
}
