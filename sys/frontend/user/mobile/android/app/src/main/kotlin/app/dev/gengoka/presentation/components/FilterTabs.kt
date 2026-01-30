package app.dev.gengoka.presentation.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import app.dev.gengoka.presentation.theme.PrimaryPurple
import app.dev.gengoka.presentation.theme.SurfaceWhite
import app.dev.gengoka.presentation.theme.TextWhite
import app.dev.gengoka.presentation.theme.TextWhiteAlpha70

data class FilterTab(
    val id: String,
    val label: String
)

@Composable
fun FilterTabs(
    tabs: List<FilterTab>,
    selectedTabId: String,
    onTabSelected: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    Row(
        modifier = modifier
            .horizontalScroll(rememberScrollState())
            .padding(vertical = 4.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        tabs.forEach { tab ->
            val isSelected = tab.id == selectedTabId
            Box(
                modifier = Modifier
                    .clip(RoundedCornerShape(20.dp))
                    .background(
                        if (isSelected) SurfaceWhite
                        else SurfaceWhite.copy(alpha = 0.2f)
                    )
                    .clickable { onTabSelected(tab.id) }
                    .padding(horizontal = 16.dp, vertical = 8.dp)
            ) {
                Text(
                    text = tab.label,
                    style = MaterialTheme.typography.labelLarge,
                    color = if (isSelected) PrimaryPurple else TextWhite
                )
            }
        }
    }
}
