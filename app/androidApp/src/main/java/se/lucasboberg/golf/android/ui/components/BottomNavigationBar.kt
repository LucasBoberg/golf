package se.lucasboberg.golf.android.ui.components

import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.res.painterResource
import androidx.navigation.NavController
import androidx.navigation.compose.currentBackStackEntryAsState
import se.lucasboberg.golf.android.Screen
import se.lucasboberg.golf.android.utils.BottomNavItem

@Composable
fun BottomNavigationBar(
    navController: NavController
) {
    val navBackStackEntry by navController.currentBackStackEntryAsState()

    val items = listOf(
        BottomNavItem.Rounds,
        BottomNavItem.Friends,
        BottomNavItem.Profile
    )

    val screensWithBottomNav = listOf(
        Screen.RoundsScreen.route,
        Screen.FriendsScreen.route,
        Screen.ProfileScreen.route
    )
    if (navBackStackEntry?.destination?.route in screensWithBottomNav) {
        NavigationBar {
            val currentRoute = navBackStackEntry?.destination?.route
            items.forEach { item ->
                NavigationBarItem(
                    icon = {
                        Icon(
                            painterResource(id = item.icon),
                            contentDescription = item.label
                        )
                    },
                    label = { Text(text = item.label) },
                    selected = currentRoute == item.route,
                    onClick = {
                        navController.navigate(item.route) {
                            navController.graph.startDestinationRoute?.let { route ->
                                popUpTo(route) {
                                    saveState = true
                                }
                            }
                            launchSingleTop = true
                            restoreState = true
                        }
                    }
                )
            }
        }
    }
}
