package app.gengoka.presentation.navigation

import android.app.Activity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.animation.AnimatedContent
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import app.gengoka.core.auth.LineAuthHelper
import app.gengoka.presentation.screens.auth.AuthViewModel
import app.gengoka.presentation.screens.auth.LoginScreen
import app.gengoka.presentation.screens.auth.RegisterScreen
import app.gengoka.presentation.screens.challenge.ChallengeScreen
import app.gengoka.presentation.screens.feed.FeedScreen
import app.gengoka.presentation.screens.home.HomeScreen
import app.gengoka.presentation.screens.linkedaccounts.LinkedAccountsScreen
import app.gengoka.presentation.screens.myprofile.MyProfileScreen
import app.gengoka.presentation.screens.profile.ProfileScreen
import app.gengoka.presentation.screens.result.ResultScreen

@Composable
fun NavGraph(
    navController: NavHostController,
    modifier: Modifier = Modifier,
    startDestination: String = Screen.Home.route
) {
    NavHost(
        navController = navController,
        startDestination = startDestination
    ) {
        // Auth Screen
        composable(Screen.Auth.route) {
            val viewModel: AuthViewModel = hiltViewModel()
            val uiState by viewModel.uiState.collectAsState()
            val context = LocalContext.current

            val lineLoginLauncher = rememberLauncherForActivityResult(
                contract = ActivityResultContracts.StartActivityForResult()
            ) { result ->
                if (result.resultCode == Activity.RESULT_OK) {
                    viewModel.handleLineLoginResult(result.data)
                } else {
                    viewModel.cancelLineLogin()
                }
            }

            AnimatedContent(
                targetState = uiState.isLoginMode,
                label = "auth_mode"
            ) { isLogin ->
                if (isLogin) {
                    LoginScreen(
                        isLoading = uiState.isLoading,
                        isSocialLoading = uiState.isSocialLoading,
                        error = uiState.error,
                        onLogin = { email, password -> viewModel.login(email, password) },
                        onSocialLogin = { provider, ctx ->
                            when (provider) {
                                "line" -> {
                                    viewModel.setSocialLoading(true)
                                    lineLoginLauncher.launch(LineAuthHelper.getLoginIntent(ctx))
                                }
                                else -> viewModel.socialLogin(provider, ctx)
                            }
                        },
                        onSwitchToRegister = { viewModel.toggleMode() }
                    )
                } else {
                    RegisterScreen(
                        isLoading = uiState.isLoading,
                        isSocialLoading = uiState.isSocialLoading,
                        error = uiState.error,
                        onRegister = { email, password, name ->
                            viewModel.register(email, password, name)
                        },
                        onSocialLogin = { provider, ctx ->
                            when (provider) {
                                "line" -> {
                                    viewModel.setSocialLoading(true)
                                    lineLoginLauncher.launch(LineAuthHelper.getLoginIntent(ctx))
                                }
                                else -> viewModel.socialLogin(provider, ctx)
                            }
                        },
                        onSwitchToLogin = { viewModel.toggleMode() }
                    )
                }
            }
        }

        // Home Screen
        composable(Screen.Home.route) {
            HomeScreen(
                onCategoryClick = { categoryId, categoryName ->
                    navController.navigate(Screen.Challenge.createRoute(categoryId, categoryName))
                },
                onDailyChallengeClick = { _, categoryId, categoryName ->
                    navController.navigate(Screen.Challenge.createRoute(categoryId, categoryName))
                }
            )
        }

        // Challenge Screen
        composable(
            route = Screen.Challenge.route,
            arguments = listOf(
                navArgument("categoryId") { type = NavType.StringType },
                navArgument("categoryName") { type = NavType.StringType }
            )
        ) {
            ChallengeScreen(
                onBackClick = { navController.popBackStack() },
                onAnswerSubmitted = { answerId ->
                    navController.navigate(Screen.Result.createRoute(answerId)) {
                        popUpTo(Screen.Home.route)
                    }
                }
            )
        }

        // Result Screen
        composable(
            route = Screen.Result.route,
            arguments = listOf(
                navArgument("answerId") { type = NavType.StringType }
            )
        ) {
            ResultScreen(
                onBackClick = {
                    navController.navigate(Screen.Home.route) {
                        popUpTo(Screen.Home.route) { inclusive = true }
                    }
                },
                onRetryClick = { categoryId, categoryName ->
                    navController.navigate(Screen.Challenge.createRoute(categoryId, categoryName)) {
                        popUpTo(Screen.Home.route)
                    }
                }
            )
        }

        // Feed Screen
        composable(Screen.Feed.route) {
            FeedScreen(
                onUserClick = { userId ->
                    navController.navigate(Screen.Profile.createRoute(userId))
                },
                onAnswerClick = { answerId ->
                    navController.navigate(Screen.Result.createRoute(answerId))
                }
            )
        }

        // Profile Screen
        composable(
            route = Screen.Profile.route,
            arguments = listOf(
                navArgument("userId") { type = NavType.StringType }
            )
        ) {
            ProfileScreen(
                onBackClick = { navController.popBackStack() },
                onAnswerClick = { answerId ->
                    navController.navigate(Screen.Result.createRoute(answerId))
                }
            )
        }

        // My Profile Screen
        composable(Screen.MyProfile.route) {
            MyProfileScreen(
                onAnswerClick = { answerId ->
                    navController.navigate(Screen.Result.createRoute(answerId))
                },
                onEditProfileClick = {
                    // TODO: Navigate to edit profile screen
                },
                onLinkedAccountsClick = {
                    navController.navigate(Screen.LinkedAccounts.route)
                }
            )
        }

        // Linked Accounts Screen
        composable(Screen.LinkedAccounts.route) {
            LinkedAccountsScreen(
                onBackClick = { navController.popBackStack() }
            )
        }
    }
}
