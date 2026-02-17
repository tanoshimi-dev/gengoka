package app.gengoka.presentation

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import app.gengoka.core.network.TokenManager
import app.gengoka.presentation.navigation.BottomNavBar
import app.gengoka.presentation.navigation.NavGraph
import app.gengoka.presentation.navigation.Screen
import app.gengoka.presentation.theme.GengokTheme
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

@AndroidEntryPoint
class MainActivity : ComponentActivity() {

    @Inject
    lateinit var tokenManager: TokenManager

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        setContent {
            GengokTheme {
                val navController = rememberNavController()
                val navBackStackEntry by navController.currentBackStackEntryAsState()
                val currentRoute = navBackStackEntry?.destination?.route
                val isAuthenticated by tokenManager.isAuthenticated.collectAsState()

                val startDestination = if (isAuthenticated) Screen.Home.route else Screen.Auth.route

                // React to auth state changes
                LaunchedEffect(isAuthenticated) {
                    val currentDest = navController.currentDestination?.route
                    if (isAuthenticated && currentDest == Screen.Auth.route) {
                        navController.navigate(Screen.Home.route) {
                            popUpTo(Screen.Auth.route) { inclusive = true }
                        }
                    } else if (!isAuthenticated && currentDest != Screen.Auth.route) {
                        navController.navigate(Screen.Auth.route) {
                            popUpTo(0) { inclusive = true }
                        }
                    }
                }

                // Routes where bottom nav should be shown
                val mainRoutes = listOf(
                    Screen.Home.route,
                    Screen.Feed.route,
                    Screen.MyProfile.route
                )
                val showBottomNav = mainRoutes.contains(currentRoute)

                Scaffold(
                    modifier = Modifier.fillMaxSize(),
                    bottomBar = {
                        if (showBottomNav) {
                            BottomNavBar(navController = navController)
                        }
                    }
                ) { innerPadding ->
                    NavGraph(
                        navController = navController,
                        startDestination = startDestination,
                        modifier = if (showBottomNav) Modifier.padding(innerPadding) else Modifier
                    )
                }
            }
        }
    }
}
