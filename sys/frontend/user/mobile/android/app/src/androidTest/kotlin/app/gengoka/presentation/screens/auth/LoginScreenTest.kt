package app.gengoka.presentation.screens.auth

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.assertIsEnabled
import androidx.compose.ui.test.assertIsNotEnabled
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextInput
import app.gengoka.presentation.theme.GengokTheme
import org.junit.Rule
import org.junit.Test

class LoginScreenTest {

    @get:Rule
    val composeTestRule = createComposeRule()

    private fun setUpLoginScreen(
        isLoading: Boolean = false,
        isSocialLoading: Boolean = false,
        error: String? = null,
        onLogin: (String, String) -> Unit = { _, _ -> },
        onSocialLogin: (String, android.content.Context) -> Unit = { _, _ -> },
        onSwitchToRegister: () -> Unit = {}
    ) {
        composeTestRule.setContent {
            GengokTheme {
                LoginScreen(
                    isLoading = isLoading,
                    isSocialLoading = isSocialLoading,
                    error = error,
                    onLogin = onLogin,
                    onSocialLogin = onSocialLogin,
                    onSwitchToRegister = onSwitchToRegister
                )
            }
        }
    }

    @Test
    fun loginScreenDisplaysAllElements() {
        setUpLoginScreen()

        composeTestRule.onNodeWithText("Gengoka").assertIsDisplayed()
        composeTestRule.onNodeWithText("メールアドレス").assertIsDisplayed()
        composeTestRule.onNodeWithText("パスワード").assertIsDisplayed()
        composeTestRule.onNodeWithText("ログイン").assertIsDisplayed()
        composeTestRule.onNodeWithText("Googleでログイン").assertIsDisplayed()
        composeTestRule.onNodeWithText("LINEでログイン").assertIsDisplayed()
        composeTestRule.onNodeWithText("新規登録").assertIsDisplayed()
    }

    @Test
    fun loginScreenShowsErrorMessage() {
        setUpLoginScreen(error = "認証に失敗しました")

        composeTestRule.onNodeWithText("認証に失敗しました").assertIsDisplayed()
    }

    @Test
    fun loginButtonDisabledWhenFieldsEmpty() {
        setUpLoginScreen()

        composeTestRule.onNodeWithText("ログイン").assertIsNotEnabled()
    }

    @Test
    fun switchToRegisterCallsCallback() {
        var switched = false
        setUpLoginScreen(onSwitchToRegister = { switched = true })

        composeTestRule.onNodeWithText("新規登録").performClick()

        assert(switched)
    }
}
