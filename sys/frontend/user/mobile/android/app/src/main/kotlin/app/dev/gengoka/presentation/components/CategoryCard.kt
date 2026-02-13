package app.dev.gengoka.presentation.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import app.dev.gengoka.domain.model.Category
import app.dev.gengoka.presentation.theme.*

@Composable
fun CategoryCard(
    category: Category,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    val accentColor = getCategoryAccentColor(category.name)
    val icon = getCategoryIcon(category.name)

    Surface(
        modifier = modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
        shape = RoundedCornerShape(20.dp),
        color = SurfaceWhite,
        shadowElevation = 2.dp
    ) {
        Column(
            modifier = Modifier.padding(16.dp)
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.Top
            ) {
                Box(
                    modifier = Modifier
                        .size(48.dp)
                        .clip(RoundedCornerShape(14.dp))
                        .background(
                            brush = Brush.linearGradient(
                                colors = listOf(
                                    accentColor,
                                    accentColor.copy(alpha = 0.7f)
                                )
                            )
                        ),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = icon,
                        contentDescription = null,
                        tint = SurfaceWhite,
                        modifier = Modifier.size(24.dp)
                    )
                }

                Box(
                    modifier = Modifier
                        .clip(RoundedCornerShape(12.dp))
                        .background(accentColor.copy(alpha = 0.12f))
                        .padding(horizontal = 8.dp, vertical = 4.dp)
                ) {
                    Text(
                        text = "${category.challengeCount}問",
                        style = MaterialTheme.typography.labelSmall.copy(
                            fontWeight = FontWeight.Medium,
                            fontSize = 11.sp
                        ),
                        color = accentColor
                    )
                }
            }

            Spacer(modifier = Modifier.height(12.dp))

            Text(
                text = category.name,
                style = MaterialTheme.typography.titleSmall.copy(
                    fontWeight = FontWeight.Bold
                ),
                color = TextDarkPrimary
            )

            Spacer(modifier = Modifier.height(4.dp))

            Text(
                text = category.getDisplayDescription(),
                style = MaterialTheme.typography.bodySmall,
                color = TextTertiary,
                maxLines = 2,
                overflow = TextOverflow.Ellipsis
            )
        }
    }
}

private fun getCategoryAccentColor(name: String): Color {
    return when (name) {
        "状況描写" -> CategorySituationAccent
        "要約力" -> CategorySummaryAccent
        "感性の言語化" -> CategoryEmotionAccent
        "言い換え" -> CategoryRephraseAccent
        "概念説明" -> CategoryExplainAccent
        else -> PrimaryPurple
    }
}

private fun getCategoryIcon(name: String): ImageVector {
    return when (name) {
        "状況描写" -> Icons.Default.LocationOn
        "要約力" -> Icons.Default.MenuBook
        "感性の言語化" -> Icons.Default.Mood
        "言い換え" -> Icons.Default.SyncAlt
        "概念説明" -> Icons.Default.Lightbulb
        else -> Icons.Default.Quiz
    }
}
