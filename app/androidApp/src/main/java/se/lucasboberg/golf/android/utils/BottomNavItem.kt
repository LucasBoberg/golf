package se.lucasboberg.golf.android.utils

import androidx.annotation.DrawableRes
import se.lucasboberg.golf.android.R
import se.lucasboberg.golf.android.Screen

sealed class BottomNavItem(
    val route: String,
    @DrawableRes val icon: Int,
    val label: String
) {
    object Rounds : BottomNavItem(
        Screen.RoundsScreen.route,
        R.drawable.golf,
        "Rounds")
    object Friends : BottomNavItem(
        Screen.FriendsScreen.route, R.drawable.friends,
        "Friends")
    object Profile : BottomNavItem(
        Screen.ProfileScreen.route, R.drawable.profile,
        "Profile")
}
