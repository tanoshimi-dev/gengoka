package app.dev.gengoka.presentation.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import app.dev.gengoka.presentation.screens.challenge.ChallengeScreen
import app.dev.gengoka.presentation.screens.feed.FeedScreen
import app.dev.gengoka.presentation.screens.home.HomeScreen
import app.dev.gengoka.presentation.screens.myprofile.MyProfileScreen
import app.dev.gengoka.presentation.screens.profile.ProfileScreen
import app.dev.gengoka.presentation.screens.result.ResultScreen
import androidx.compose.ui.Modifier

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
        // Home Screen
        composable(Screen.Home.route) {
            HomeScreen(
                onCategoryClick = { categoryId, categoryName ->
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
                }
            )
        }
    }
}
