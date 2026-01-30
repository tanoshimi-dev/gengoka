package app.dev.gengoka.presentation.components

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import app.dev.gengoka.domain.model.Category
import app.dev.gengoka.presentation.theme.*

@Composable
fun CategoryCard(
    category: Category,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    val backgroundColor = getCategoryColor(category.name)
    val icon = getCategoryIcon(category.name)

    Row(
        modifier = modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(16.dp))
            .border(2.dp, BorderLight, RoundedCornerShape(16.dp))
            .background(SurfaceWhite)
            .clickable(onClick = onClick)
            .padding(horizontal = 20.dp, vertical = 18.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Box(
            modifier = Modifier
                .size(48.dp)
                .clip(RoundedCornerShape(12.dp))
                .background(backgroundColor),
            contentAlignment = Alignment.Center
        ) {
            Icon(
                imageVector = icon,
                contentDescription = null,
                tint = TextPrimary,
                modifier = Modifier.size(24.dp)
            )
        }

        Spacer(modifier = Modifier.width(16.dp))

        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = category.name,
                style = MaterialTheme.typography.titleMedium,
                color = TextPrimary
            )
            Spacer(modifier = Modifier.height(4.dp))
            Text(
                text = category.getDisplayDescription(),
                style = MaterialTheme.typography.bodySmall,
                color = TextTertiary
            )
        }

        Spacer(modifier = Modifier.width(12.dp))

        Box(
            modifier = Modifier
                .clip(RoundedCornerShape(20.dp))
                .background(SurfaceGray)
                .padding(horizontal = 12.dp, vertical = 6.dp)
        ) {
            Text(
                text = "${category.charLimit}文字",
                style = MaterialTheme.typography.labelSmall,
                color = TextSecondary
            )
        }
    }
}

private fun getCategoryColor(name: String): Color {
    return when (name) {
        "状況描写" -> CategorySituation
        "要約力" -> CategorySummary
        "感性の言語化" -> CategoryEmotion
        "言い換え" -> CategoryRephrase
        "概念説明" -> CategoryExplain
        else -> SurfaceGray
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
