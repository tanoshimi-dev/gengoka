package app.dev.gengoka.presentation.components

import androidx.compose.foundation.layout.Column
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp
import app.dev.gengoka.presentation.theme.BackgroundGradientEnd
import app.dev.gengoka.presentation.theme.BackgroundGradientStart
import app.dev.gengoka.presentation.theme.TextTertiary

@Composable
fun ScoreDisplay(
    score: Int,
    modifier: Modifier = Modifier,
    label: String = "言語化スコア"
) {
    Column(
        modifier = modifier,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = score.toString(),
            style = TextStyle(
                brush = Brush.linearGradient(
                    colors = listOf(BackgroundGradientStart, BackgroundGradientEnd)
                ),
                fontSize = 64.sp,
                fontWeight = FontWeight.Bold
            )
        )
        Text(
            text = label,
            style = MaterialTheme.typography.bodyMedium,
            color = TextTertiary
        )
    }
}

@Composable
fun ScoreText(
    score: Int,
    modifier: Modifier = Modifier
) {
    Text(
        text = "スコア $score",
        style = TextStyle(
            brush = Brush.linearGradient(
                colors = listOf(BackgroundGradientStart, BackgroundGradientEnd)
            ),
            fontSize = 14.sp,
            fontWeight = FontWeight.Bold
        ),
        modifier = modifier
    )
}
