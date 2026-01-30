package app.dev.gengoka.presentation.navigation

sealed class Screen(val route: String) {
    data object Home : Screen("home")
    data object Challenge : Screen("challenge/{categoryId}/{categoryName}") {
        fun createRoute(categoryId: String, categoryName: String) =
            "challenge/$categoryId/$categoryName"
    }
    data object Result : Screen("result/{answerId}") {
        fun createRoute(answerId: String) = "result/$answerId"
    }
    data object Feed : Screen("feed")
    data object Profile : Screen("profile/{userId}") {
        fun createRoute(userId: String) = "profile/$userId"
    }
    data object MyProfile : Screen("myprofile")
}

sealed class BottomNavItem(
    val route: String,
    val title: String,
    val icon: String
) {
    data object Home : BottomNavItem(Screen.Home.route, "ホーム", "home")
    data object Feed : BottomNavItem(Screen.Feed.route, "タイムライン", "feed")
    data object MyProfile : BottomNavItem(Screen.MyProfile.route, "マイページ", "person")
}
