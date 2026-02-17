package app.gengoka.presentation.screens.myprofile

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChevronRight
import androidx.compose.material.icons.filled.Description
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material.icons.automirrored.filled.HelpOutline
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Link
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.automirrored.filled.Logout
import androidx.compose.material.icons.filled.Notifications
import androidx.compose.material.icons.automirrored.filled.ShowChart
import androidx.compose.material.icons.filled.TaskAlt
import androidx.compose.material.icons.filled.Whatshot
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import app.gengoka.core.util.formatCount
import app.gengoka.domain.model.UserProfile
import app.gengoka.domain.model.UserStats
import app.gengoka.presentation.components.UserAvatar
import app.gengoka.presentation.theme.BackgroundLightGradientEnd
import app.gengoka.presentation.theme.BackgroundLightGradientStart
import app.gengoka.presentation.theme.BorderLight
import app.gengoka.presentation.theme.ErrorRed
import app.gengoka.presentation.theme.PrimaryPurple
import app.gengoka.presentation.theme.StatsGreen
import app.gengoka.presentation.theme.StatsOrange
import app.gengoka.presentation.theme.SurfaceGray
import app.gengoka.presentation.theme.SurfaceWhite
import app.gengoka.presentation.theme.TextDarkPrimary
import app.gengoka.presentation.theme.TextPrimary
import app.gengoka.presentation.theme.TextSecondary
import app.gengoka.presentation.theme.TextTertiary
import app.gengoka.presentation.theme.WarningYellow

@Composable
fun MyProfileScreen(
    onAnswerClick: (answerId: String) -> Unit,
    onEditProfileClick: () -> Unit,
    onLinkedAccountsClick: () -> Unit = {},
    viewModel: MyProfileViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsState()

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(
                Brush.verticalGradient(
                    colors = listOf(BackgroundLightGradientStart, BackgroundLightGradientEnd)
                )
            )
    ) {
        if (uiState.isLoading && uiState.profile == null) {
            Box(
                modifier = Modifier.fillMaxSize(),
                contentAlignment = Alignment.Center
            ) {
                CircularProgressIndicator(color = PrimaryPurple)
            }
        } else {
            Column(
                modifier = Modifier
                    .fillMaxSize()
                    .verticalScroll(rememberScrollState())
                    .padding(16.dp)
            ) {
                Spacer(modifier = Modifier.height(48.dp))

                // Title
                Text(
                    text = "マイページ",
                    style = MaterialTheme.typography.headlineLarge,
                    fontWeight = FontWeight.Bold,
                    color = TextDarkPrimary
                )

                Spacer(modifier = Modifier.height(20.dp))

                uiState.profile?.let { profile ->
                    // Header Card
                    HeaderSection(
                        profile = profile,
                        onEditClick = onEditProfileClick
                    )

                    Spacer(modifier = Modifier.height(16.dp))

                    // Stats Section
                    uiState.stats?.let { stats ->
                        StatsSection(stats = stats)
                        Spacer(modifier = Modifier.height(16.dp))
                    }

                    // Social Section
                    SocialSection(profile = profile)

                    Spacer(modifier = Modifier.height(16.dp))

                    // Settings Section
                    SettingsSection(
                        onLinkedAccountsClick = onLinkedAccountsClick,
                        onLogout = viewModel::logout
                    )
                }

                Spacer(modifier = Modifier.height(100.dp))
            }
        }
    }
}

@Composable
private fun HeaderSection(
    profile: UserProfile,
    onEditClick: () -> Unit
) {
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(4.dp, RoundedCornerShape(24.dp)),
        shape = RoundedCornerShape(24.dp),
        color = SurfaceWhite
    ) {
        Column(
            modifier = Modifier.padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            UserAvatar(
                name = profile.name,
                avatarUrl = profile.avatar,
                size = 100.dp
            )

            Spacer(modifier = Modifier.height(16.dp))

            Text(
                text = profile.name,
                style = MaterialTheme.typography.headlineSmall,
                fontWeight = FontWeight.Bold,
                color = TextPrimary
            )

            profile.bio?.let { bio ->
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = bio,
                    style = MaterialTheme.typography.bodySmall,
                    color = TextSecondary
                )
            }

            Spacer(modifier = Modifier.height(12.dp))

            // Score badge
            Row(
                modifier = Modifier
                    .background(
                        PrimaryPurple.copy(alpha = 0.15f),
                        RoundedCornerShape(12.dp)
                    )
                    .padding(horizontal = 10.dp, vertical = 6.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = "\u2605",
                    color = PrimaryPurple,
                    fontSize = 14.sp
                )
                Spacer(modifier = Modifier.width(4.dp))
                Text(
                    text = "${profile.totalLikes}pt",
                    style = MaterialTheme.typography.labelMedium,
                    fontWeight = FontWeight.Medium,
                    color = TextPrimary
                )
            }

            Spacer(modifier = Modifier.height(16.dp))

            // Edit profile button
            Surface(
                modifier = Modifier.clickable(onClick = onEditClick),
                shape = RoundedCornerShape(16.dp),
                color = SurfaceWhite,
                border = androidx.compose.foundation.BorderStroke(1.dp, PrimaryPurple)
            ) {
                Row(
                    modifier = Modifier.padding(horizontal = 20.dp, vertical = 10.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Icon(
                        Icons.Filled.Edit,
                        contentDescription = null,
                        tint = PrimaryPurple,
                        modifier = Modifier.size(16.dp)
                    )
                    Spacer(modifier = Modifier.width(6.dp))
                    Text(
                        text = "プロフィールを編集",
                        style = MaterialTheme.typography.bodySmall,
                        fontWeight = FontWeight.Medium,
                        color = PrimaryPurple
                    )
                }
            }
        }
    }
}

@Composable
private fun StatsSection(stats: UserStats) {
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(4.dp, RoundedCornerShape(20.dp)),
        shape = RoundedCornerShape(20.dp),
        color = SurfaceWhite
    ) {
        Column(modifier = Modifier.padding(20.dp)) {
            Text(
                text = "学習統計",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold,
                color = TextPrimary
            )

            Spacer(modifier = Modifier.height(16.dp))

            // 2x2 grid
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                StatCard(
                    icon = Icons.Filled.Whatshot,
                    iconColor = StatsOrange,
                    value = "${stats.currentStreak}日",
                    label = "連続チャレンジ",
                    subLabel = "最高: ${stats.bestStreak}日",
                    modifier = Modifier.weight(1f)
                )
                StatCard(
                    icon = Icons.Filled.TaskAlt,
                    iconColor = StatsGreen,
                    value = "${stats.completedToday}",
                    label = "今日の完了",
                    subLabel = null,
                    modifier = Modifier.weight(1f)
                )
            }

            Spacer(modifier = Modifier.height(12.dp))

            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                StatCard(
                    icon = Icons.AutoMirrored.Filled.ShowChart,
                    iconColor = PrimaryPurple,
                    value = String.format("%.1f", stats.averageScore),
                    label = "平均スコア",
                    subLabel = null,
                    modifier = Modifier.weight(1f)
                )
                StatCard(
                    icon = Icons.Filled.Description,
                    iconColor = WarningYellow,
                    value = "${stats.totalChallenges}",
                    label = "総チャレンジ数",
                    subLabel = null,
                    modifier = Modifier.weight(1f)
                )
            }
        }
    }
}

@Composable
private fun StatCard(
    icon: ImageVector,
    iconColor: Color,
    value: String,
    label: String,
    subLabel: String?,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .background(SurfaceGray, RoundedCornerShape(16.dp))
            .padding(16.dp)
    ) {
        Icon(
            icon,
            contentDescription = null,
            tint = iconColor,
            modifier = Modifier.size(24.dp)
        )

        Spacer(modifier = Modifier.height(8.dp))

        Text(
            text = value,
            style = MaterialTheme.typography.headlineSmall,
            fontWeight = FontWeight.Bold,
            color = TextPrimary
        )

        Text(
            text = label,
            style = MaterialTheme.typography.labelSmall,
            color = TextSecondary
        )

        if (subLabel != null) {
            Text(
                text = subLabel,
                style = MaterialTheme.typography.labelSmall,
                fontSize = 10.sp,
                color = TextTertiary
            )
        }
    }
}

@Composable
private fun SocialSection(profile: UserProfile) {
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(4.dp, RoundedCornerShape(16.dp)),
        shape = RoundedCornerShape(16.dp),
        color = SurfaceWhite
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(vertical = 16.dp),
            horizontalArrangement = Arrangement.SpaceEvenly,
            verticalAlignment = Alignment.CenterVertically
        ) {
            SocialStatItem(
                value = profile.followerCount.formatCount(),
                label = "フォロワー"
            )
            Box(
                modifier = Modifier
                    .width(1.dp)
                    .height(40.dp)
                    .background(BorderLight)
            )
            SocialStatItem(
                value = profile.followingCount.formatCount(),
                label = "フォロー中"
            )
            Box(
                modifier = Modifier
                    .width(1.dp)
                    .height(40.dp)
                    .background(BorderLight)
            )
            SocialStatItem(
                value = profile.answerCount.formatCount(),
                label = "回答数"
            )
        }
    }
}

@Composable
private fun SocialStatItem(value: String, label: String) {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        Text(
            text = value,
            style = MaterialTheme.typography.titleLarge,
            fontWeight = FontWeight.Bold,
            color = TextPrimary
        )
        Spacer(modifier = Modifier.height(2.dp))
        Text(
            text = label,
            style = MaterialTheme.typography.labelSmall,
            color = TextSecondary
        )
    }
}

@Composable
private fun SettingsSection(
    onLinkedAccountsClick: () -> Unit,
    onLogout: () -> Unit
) {
    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(4.dp, RoundedCornerShape(16.dp)),
        shape = RoundedCornerShape(16.dp),
        color = SurfaceWhite
    ) {
        Column {
            SettingsRow(
                icon = Icons.Filled.Link,
                title = "アカウント連携",
                iconColor = PrimaryPurple,
                onClick = onLinkedAccountsClick
            )
            HorizontalDivider(
                modifier = Modifier.padding(start = 52.dp),
                color = BorderLight
            )
            SettingsRow(
                icon = Icons.Filled.Notifications,
                title = "通知設定",
                iconColor = ErrorRed,
                onClick = {}
            )
            HorizontalDivider(
                modifier = Modifier.padding(start = 52.dp),
                color = BorderLight
            )
            SettingsRow(
                icon = Icons.Filled.Lock,
                title = "プライバシー設定",
                iconColor = PrimaryPurple,
                onClick = {}
            )
            HorizontalDivider(
                modifier = Modifier.padding(start = 52.dp),
                color = BorderLight
            )
            SettingsRow(
                icon = Icons.AutoMirrored.Filled.HelpOutline,
                title = "ヘルプ",
                iconColor = StatsGreen,
                onClick = {}
            )
            HorizontalDivider(
                modifier = Modifier.padding(start = 52.dp),
                color = BorderLight
            )
            SettingsRow(
                icon = Icons.Filled.Info,
                title = "アプリについて",
                iconColor = TextSecondary,
                onClick = {}
            )
            HorizontalDivider(
                modifier = Modifier.padding(start = 52.dp),
                color = BorderLight
            )
            // Logout
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable(onClick = onLogout)
                    .padding(horizontal = 16.dp, vertical = 16.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Icon(
                    Icons.AutoMirrored.Filled.Logout,
                    contentDescription = null,
                    tint = ErrorRed,
                    modifier = Modifier.size(24.dp)
                )
                Spacer(modifier = Modifier.width(16.dp))
                Text(
                    text = "ログアウト",
                    style = MaterialTheme.typography.bodyMedium,
                    color = ErrorRed
                )
            }
        }
    }
}

@Composable
private fun SettingsRow(
    icon: ImageVector,
    title: String,
    iconColor: Color,
    onClick: () -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick)
            .padding(horizontal = 16.dp, vertical = 14.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            icon,
            contentDescription = null,
            tint = iconColor,
            modifier = Modifier.size(24.dp)
        )
        Spacer(modifier = Modifier.width(16.dp))
        Text(
            text = title,
            style = MaterialTheme.typography.bodySmall,
            color = TextPrimary,
            modifier = Modifier.weight(1f)
        )
        Icon(
            Icons.Filled.ChevronRight,
            contentDescription = null,
            tint = TextTertiary,
            modifier = Modifier.size(16.dp)
        )
    }
}
