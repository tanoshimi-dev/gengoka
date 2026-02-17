package app.dev.gengoka.presentation.screens.linkedaccounts

import androidx.compose.foundation.background
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
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.ChatBubble
import androidx.compose.material.icons.filled.Email
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
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
import androidx.hilt.navigation.compose.hiltViewModel
import app.dev.gengoka.domain.model.LinkedSocialAccount
import app.dev.gengoka.presentation.theme.BackgroundLightGradientEnd
import app.dev.gengoka.presentation.theme.BackgroundLightGradientStart
import app.dev.gengoka.presentation.theme.BorderLight
import app.dev.gengoka.presentation.theme.ErrorRed
import app.dev.gengoka.presentation.theme.PrimaryPurple
import app.dev.gengoka.presentation.theme.SurfaceWhite
import app.dev.gengoka.presentation.theme.TextDarkPrimary
import app.dev.gengoka.presentation.theme.TextPrimary
import app.dev.gengoka.presentation.theme.TextSecondary
import app.dev.gengoka.presentation.theme.TextTertiary

data class ProviderInfo(
    val id: String,
    val name: String,
    val icon: ImageVector,
    val color: Color
)

private val providers = listOf(
    ProviderInfo("google", "Google", Icons.Filled.Email, Color(0xFFDB4437)),
    ProviderInfo("line", "LINE", Icons.Filled.ChatBubble, Color(0xFF00BA34))
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LinkedAccountsScreen(
    onBackClick: () -> Unit,
    viewModel: LinkedAccountsViewModel = hiltViewModel()
) {
    val uiState by viewModel.uiState.collectAsState()

    Scaffold(
        topBar = {
            TopAppBar(
                title = {
                    Text(
                        "アカウント連携",
                        fontWeight = FontWeight.Bold,
                        color = TextDarkPrimary
                    )
                },
                navigationIcon = {
                    IconButton(onClick = onBackClick) {
                        Icon(
                            Icons.AutoMirrored.Filled.ArrowBack,
                            contentDescription = "戻る",
                            tint = TextDarkPrimary
                        )
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = Color.Transparent
                )
            )
        },
        containerColor = Color.Transparent
    ) { paddingValues ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(
                    Brush.verticalGradient(
                        colors = listOf(BackgroundLightGradientStart, BackgroundLightGradientEnd)
                    )
                )
                .padding(paddingValues)
        ) {
            if (uiState.isLoading && uiState.accounts.isEmpty()) {
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
                    // Error message
                    uiState.error?.let { error ->
                        Surface(
                            modifier = Modifier.fillMaxWidth(),
                            shape = RoundedCornerShape(12.dp),
                            color = ErrorRed.copy(alpha = 0.1f)
                        ) {
                            Text(
                                text = error,
                                color = ErrorRed,
                                style = MaterialTheme.typography.bodySmall,
                                modifier = Modifier.padding(12.dp)
                            )
                        }
                        Spacer(modifier = Modifier.height(16.dp))
                    }

                    // Provider list
                    Surface(
                        modifier = Modifier
                            .fillMaxWidth()
                            .shadow(4.dp, RoundedCornerShape(16.dp)),
                        shape = RoundedCornerShape(16.dp),
                        color = SurfaceWhite
                    ) {
                        Column {
                            providers.forEachIndexed { index, provider ->
                                val linked = uiState.accounts.find { it.provider == provider.id }
                                ProviderRow(
                                    provider = provider,
                                    linked = linked,
                                    isLinking = uiState.isLinking,
                                    onLink = { viewModel.linkAccount(provider.id) },
                                    onUnlink = { viewModel.showUnlinkDialog(provider.id) }
                                )
                                if (index < providers.lastIndex) {
                                    HorizontalDivider(
                                        modifier = Modifier.padding(start = 56.dp),
                                        color = BorderLight
                                    )
                                }
                            }
                        }
                    }

                    Spacer(modifier = Modifier.height(12.dp))

                    // Footer note
                    Text(
                        text = "連携したアカウントでログインできます。すべての連携を解除することはできません。",
                        style = MaterialTheme.typography.bodySmall,
                        color = TextTertiary,
                        modifier = Modifier.padding(horizontal = 4.dp)
                    )
                }
            }
        }
    }

    // Unlink confirmation dialog
    if (uiState.unlinkTarget != null) {
        val targetName = providers.find { it.id == uiState.unlinkTarget }?.name ?: uiState.unlinkTarget
        AlertDialog(
            onDismissRequest = { viewModel.dismissUnlinkDialog() },
            title = { Text("連携を解除しますか？") },
            text = { Text("${targetName}との連携を解除すると、このアカウントでのログインができなくなります。") },
            confirmButton = {
                TextButton(
                    onClick = { viewModel.confirmUnlink() },
                    colors = ButtonDefaults.textButtonColors(contentColor = ErrorRed)
                ) {
                    Text("解除")
                }
            },
            dismissButton = {
                TextButton(onClick = { viewModel.dismissUnlinkDialog() }) {
                    Text("キャンセル")
                }
            }
        )
    }
}

@Composable
private fun ProviderRow(
    provider: ProviderInfo,
    linked: LinkedSocialAccount?,
    isLinking: Boolean,
    onLink: () -> Unit,
    onUnlink: () -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 14.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            provider.icon,
            contentDescription = null,
            tint = provider.color,
            modifier = Modifier.size(28.dp)
        )

        Spacer(modifier = Modifier.width(12.dp))

        Column(modifier = Modifier.weight(1f)) {
            Text(
                text = provider.name,
                style = MaterialTheme.typography.bodyMedium,
                fontWeight = FontWeight.Medium,
                color = TextPrimary
            )
            Text(
                text = if (linked != null) {
                    linked.providerEmail ?: "連携済み"
                } else {
                    "未連携"
                },
                style = MaterialTheme.typography.labelSmall,
                color = if (linked != null) TextSecondary else TextTertiary
            )
        }

        if (linked != null) {
            OutlinedButton(
                onClick = onUnlink,
                colors = ButtonDefaults.outlinedButtonColors(contentColor = ErrorRed),
                border = ButtonDefaults.outlinedButtonBorder(enabled = true).copy(
                    brush = Brush.linearGradient(listOf(ErrorRed, ErrorRed))
                )
            ) {
                Text("解除", style = MaterialTheme.typography.labelSmall)
            }
        } else {
            Button(
                onClick = onLink,
                enabled = !isLinking,
                colors = ButtonDefaults.buttonColors(containerColor = PrimaryPurple)
            ) {
                if (isLinking) {
                    CircularProgressIndicator(
                        modifier = Modifier.size(16.dp),
                        color = Color.White,
                        strokeWidth = 2.dp
                    )
                } else {
                    Text("連携", style = MaterialTheme.typography.labelSmall)
                }
            }
        }
    }
}
