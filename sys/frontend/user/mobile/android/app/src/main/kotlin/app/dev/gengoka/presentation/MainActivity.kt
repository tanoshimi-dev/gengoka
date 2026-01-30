package app.dev.gengoka.presentation

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import app.dev.gengoka.presentation.navigation.BottomNavBar
import app.dev.gengoka.presentation.navigation.NavGraph
import app.dev.gengoka.presentation.navigation.Screen
import app.dev.gengoka.presentation.theme.GengokTheme
import dagger.hilt.android.AndroidEntryPoint

@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        setContent {
            GengokTheme {
                val navController = rememberNavController()
                val navBackStackEntry by navController.currentBackStackEntryAsState()
                val currentRoute = navBackStackEntry?.destination?.route

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
                        modifier = if (showBottomNav) Modifier.padding(innerPadding) else Modifier
                    )
                }
            }
        }
    }
}
