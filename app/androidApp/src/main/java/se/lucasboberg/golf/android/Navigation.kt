package se.lucasboberg.golf.android

import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import se.lucasboberg.golf.android.ui.screens.SignInScreen
import se.lucasboberg.golf.android.ui.screens.SignUpScreen
import se.lucasboberg.golf.android.ui.screens.WelcomeScreen
import se.lucasboberg.golf.utils.Settings

@Composable
fun Navigation(
    navController: NavHostController
) {
    val context = LocalContext.current
    val settings = Settings(context.applicationContext)

    NavHost(
        navController = navController,
        startDestination = if (settings.get("token") == null || settings.get("token") == "") Screen.WelcomeScreen.route else Screen.RoundsScreen.route
    ) {
        composable(Screen.WelcomeScreen.route) {
            WelcomeScreen(navController)
        }
        composable(Screen.SignUpScreen.route) {
            SignUpScreen(navController)
        }
        composable(Screen.SignInScreen.route) {
            SignInScreen(navController)
        }
    }
}