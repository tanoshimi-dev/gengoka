package app.gengoka.presentation.screens.auth

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsNotEnabled
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import app.gengoka.presentation.theme.GengokTheme
import org.junit.Rule
import org.junit.Test

class RegisterScreenTest {

    @get:Rule
    val composeTestRule = createComposeRule()

    private fun setUpRegisterScreen(
        isLoading: Boolean = false,
        isSocialLoading: Boolean = false,
        error: String? = null,
        onRegister: (String, String, String) -> Unit = { _, _, _ -> },
        onSocialLogin: (String, android.content.Context) -> Unit = { _, _ -> },
        onSwitchToLogin: () -> Unit = {}
    ) {
        composeTestRule.setContent {
            GengokTheme {
                RegisterScreen(
                    isLoading = isLoading,
                    isSocialLoading = isSocialLoading,
                    error = error,
                    onRegister = onRegister,
                    onSocialLogin = onSocialLogin,
                    onSwitchToLogin = onSwitchToLogin
                )
            }
        }
    }

    @Test
    fun registerScreenDisplaysAllElements() {
        setUpRegisterScreen()

        composeTestRule.onNodeWithText("新規登録").assertIsDisplayed()
        composeTestRule.onNodeWithText("メールアドレス").assertIsDisplayed()
        composeTestRule.onNodeWithText("表示名").assertIsDisplayed()
        composeTestRule.onNodeWithText("パスワード").assertIsDisplayed()
        composeTestRule.onNodeWithText("パスワード(確認)").assertIsDisplayed()
        composeTestRule.onNodeWithText("アカウントを作成").assertIsDisplayed()
        composeTestRule.onNodeWithText("Googleで登録").assertIsDisplayed()
        composeTestRule.onNodeWithText("LINEで登録").assertIsDisplayed()
    }

    @Test
    fun registerScreenShowsErrorMessage() {
        setUpRegisterScreen(error = "メール既に登録済み")

        composeTestRule.onNodeWithText("メール既に登録済み").assertIsDisplayed()
    }

    @Test
    fun registerButtonDisabledWhenFieldsEmpty() {
        setUpRegisterScreen()

        composeTestRule.onNodeWithText("アカウントを作成").assertIsNotEnabled()
    }

    @Test
    fun switchToLoginCallsCallback() {
        var switched = false
        setUpRegisterScreen(onSwitchToLogin = { switched = true })

        composeTestRule.onNodeWithText("ログイン").performClick()

        assert(switched)
    }
}
