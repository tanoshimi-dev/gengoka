package app.dev.gengoka.presentation.components

import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.unit.dp
import app.dev.gengoka.presentation.theme.*

@Composable
fun CharacterCounterTextField(
    value: String,
    onValueChange: (String) -> Unit,
    charLimit: Int,
    modifier: Modifier = Modifier,
    placeholder: String = "ここに回答を入力..."
) {
    val isOverLimit = value.length > charLimit
    val borderColor = if (isOverLimit) ErrorRed else BorderLight
    val focusedBorderColor = if (isOverLimit) ErrorRed else PrimaryPurple

    Box(modifier = modifier) {
        BasicTextField(
            value = value,
            onValueChange = onValueChange,
            textStyle = MaterialTheme.typography.bodyLarge.copy(color = TextPrimary),
            cursorBrush = SolidColor(PrimaryPurple),
            modifier = Modifier
                .fillMaxWidth()
                .heightIn(min = 120.dp)
                .border(2.dp, borderColor, RoundedCornerShape(12.dp))
                .padding(16.dp)
                .padding(bottom = 24.dp),
            decorationBox = { innerTextField ->
                Box {
                    if (value.isEmpty()) {
                        Text(
                            text = placeholder,
                            style = MaterialTheme.typography.bodyLarge,
                            color = TextTertiary
                        )
                    }
                    innerTextField()
                }
            }
        )

        Text(
            text = "${value.length}/$charLimit",
            style = MaterialTheme.typography.bodyMedium,
            color = if (isOverLimit) ErrorRed else TextTertiary,
            modifier = Modifier
                .align(Alignment.BottomEnd)
                .padding(16.dp)
        )
    }
}
