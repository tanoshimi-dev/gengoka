package app.dev.gengoka.presentation.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import app.dev.gengoka.presentation.theme.TextWhite
import app.dev.gengoka.presentation.theme.TextWhiteAlpha70

data class StatItem(
    val value: String,
    val label: String
)

@Composable
fun StatsBar(
    stats: List<StatItem>,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(16.dp))
            .background(TextWhite.copy(alpha = 0.1f))
            .padding(16.dp),
        horizontalArrangement = Arrangement.SpaceAround
    ) {
        stats.forEach { stat ->
            Column(
                horizontalAlignment = Alignment.CenterHorizontally
            ) {
                Text(
                    text = stat.value,
                    style = MaterialTheme.typography.headlineMedium,
                    color = TextWhite
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = stat.label,
                    style = MaterialTheme.typography.labelSmall,
                    color = TextWhiteAlpha70
                )
            }
        }
    }
}
