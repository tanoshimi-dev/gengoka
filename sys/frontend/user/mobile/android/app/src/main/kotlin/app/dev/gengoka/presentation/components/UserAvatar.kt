package app.dev.gengoka.presentation.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import app.dev.gengoka.presentation.theme.BackgroundGradientEnd
import app.dev.gengoka.presentation.theme.BackgroundGradientStart
import app.dev.gengoka.presentation.theme.TextWhite
import coil.compose.AsyncImage

@Composable
fun UserAvatar(
    name: String,
    avatarUrl: String?,
    modifier: Modifier = Modifier,
    size: Dp = 44.dp,
    onClick: (() -> Unit)? = null
) {
    val initial = name.firstOrNull()?.toString() ?: "?"

    Box(
        modifier = modifier
            .size(size)
            .clip(CircleShape)
            .background(
                brush = Brush.linearGradient(
                    colors = listOf(BackgroundGradientStart, BackgroundGradientEnd)
                )
            )
            .then(
                if (onClick != null) Modifier.clickable(onClick = onClick)
                else Modifier
            ),
        contentAlignment = Alignment.Center
    ) {
        if (avatarUrl != null) {
            AsyncImage(
                model = avatarUrl,
                contentDescription = name,
                contentScale = ContentScale.Crop,
                modifier = Modifier.size(size)
            )
        } else {
            Text(
                text = initial,
                style = when {
                    size >= 80.dp -> MaterialTheme.typography.headlineLarge
                    size >= 44.dp -> MaterialTheme.typography.titleMedium
                    else -> MaterialTheme.typography.labelMedium
                },
                color = TextWhite
            )
        }
    }
}
