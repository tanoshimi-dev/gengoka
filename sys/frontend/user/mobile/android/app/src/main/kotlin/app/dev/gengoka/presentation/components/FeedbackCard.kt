package app.dev.gengoka.presentation.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import app.dev.gengoka.presentation.theme.*

enum class FeedbackType {
    GOOD,
    IMPROVEMENT,
    EXAMPLE
}

@Composable
fun FeedbackCard(
    type: FeedbackType,
    title: String,
    content: String,
    modifier: Modifier = Modifier
) {
    val (backgroundColor, borderColor, titleColor) = when (type) {
        FeedbackType.GOOD -> Triple(FeedbackGoodBackground, FeedbackGoodBorder, FeedbackGoodText)
        FeedbackType.IMPROVEMENT -> Triple(FeedbackImprovementBackground, FeedbackImprovementBorder, FeedbackImprovementText)
        FeedbackType.EXAMPLE -> Triple(FeedbackExampleBackground, FeedbackExampleBorder, FeedbackExampleText)
    }

    Box(
        modifier = modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(12.dp))
            .background(backgroundColor)
            .drawBehind {
                drawLine(
                    color = borderColor,
                    start = Offset(0f, 0f),
                    end = Offset(0f, size.height),
                    strokeWidth = 4.dp.toPx()
                )
            }
            .padding(16.dp)
    ) {
        Column {
            Text(
                text = title,
                style = MaterialTheme.typography.titleSmall,
                color = titleColor
            )
            Spacer(modifier = Modifier.height(8.dp))
            Text(
                text = content,
                style = MaterialTheme.typography.bodyMedium,
                color = TextPrimary,
                lineHeight = MaterialTheme.typography.bodyLarge.lineHeight
            )
        }
    }
}
