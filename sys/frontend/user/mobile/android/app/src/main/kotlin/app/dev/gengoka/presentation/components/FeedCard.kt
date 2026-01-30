package app.dev.gengoka.presentation.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ChatBubbleOutline
import androidx.compose.material.icons.filled.Favorite
import androidx.compose.material.icons.filled.FavoriteBorder
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import app.dev.gengoka.core.util.DateUtils
import app.dev.gengoka.domain.model.AnswerWithDetails
import app.dev.gengoka.presentation.theme.*

@Composable
fun FeedCard(
    answer: AnswerWithDetails,
    onUserClick: () -> Unit,
    onLikeClick: () -> Unit,
    onCommentClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(16.dp))
            .background(SurfaceWhite)
            .padding(16.dp)
    ) {
        // Header
        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier.fillMaxWidth()
        ) {
            UserAvatar(
                name = answer.user.name,
                avatarUrl = answer.user.avatar,
                size = 44.dp,
                onClick = onUserClick
            )

            Spacer(modifier = Modifier.width(12.dp))

            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = answer.user.name,
                    style = MaterialTheme.typography.titleSmall,
                    color = TextPrimary,
                    modifier = Modifier.clickable(onClick = onUserClick)
                )
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    answer.score?.let { score ->
                        ScoreText(score = score)
                    }
                    Text(
                        text = DateUtils.formatRelativeTime(answer.createdAt),
                        style = MaterialTheme.typography.bodySmall,
                        color = TextTertiary
                    )
                }
            }
        }

        Spacer(modifier = Modifier.height(12.dp))

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
                    text = answer.challenge.title.take(20),
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

        // Answer content
        Text(
            text = answer.content,
            style = MaterialTheme.typography.bodyLarge,
            color = TextPrimary,
            modifier = Modifier.padding(vertical = 8.dp)
        )

        // Divider
        Spacer(
            modifier = Modifier
                .fillMaxWidth()
                .height(1.dp)
                .background(BorderMedium)
        )

        // Actions
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(top = 12.dp),
            horizontalArrangement = Arrangement.spacedBy(20.dp)
        ) {
            Row(
                modifier = Modifier.clickable(onClick = onLikeClick),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                Icon(
                    imageVector = if (answer.isLiked) Icons.Default.Favorite else Icons.Default.FavoriteBorder,
                    contentDescription = "Like",
                    tint = if (answer.isLiked) LikeRed else TextTertiary,
                    modifier = Modifier.size(20.dp)
                )
                Text(
                    text = answer.likeCount.toString(),
                    style = MaterialTheme.typography.bodyMedium,
                    color = if (answer.isLiked) LikeRed else TextTertiary
                )
            }

            Row(
                modifier = Modifier.clickable(onClick = onCommentClick),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(6.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.ChatBubbleOutline,
                    contentDescription = "Comment",
                    tint = TextTertiary,
                    modifier = Modifier.size(20.dp)
                )
                Text(
                    text = answer.commentCount.toString(),
                    style = MaterialTheme.typography.bodyMedium,
                    color = TextTertiary
                )
            }
        }
    }
}
