package se.lucasboberg.golf.android

sealed class Screen(val route: String) {
    object WelcomeScreen : Screen("welcome_screen")
    object SignUpScreen : Screen("sign_up_screen")
    object SignInScreen : Screen("sign_in_screen")
    object RoundsScreen : Screen("rounds_screen")
    object FriendsScreen : Screen("friends_screen")
    object ProfileScreen : Screen("profile_screen")

    object RoundScreen : Screen("round_screen") {
        fun withArgs(roundId: String): String {
            return "$route/$roundId"
        }
    }

    fun withArgs(vararg args: String): String {
        return buildString {
            append(route)
            args.forEach { arg ->
                append("/$arg")
            }
        }
    }
}