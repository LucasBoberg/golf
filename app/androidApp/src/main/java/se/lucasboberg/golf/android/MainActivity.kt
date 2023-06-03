package se.lucasboberg.golf.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.material.*
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import androidx.navigation.compose.rememberNavController
import se.lucasboberg.golf.ApiClient
import com.google.accompanist.systemuicontroller.rememberSystemUiController
import se.lucasboberg.golf.android.ui.components.BottomNavigationBar
import se.lucasboberg.golf.android.ui.theme.GolfTheme
import se.lucasboberg.golf.utils.Settings

class MainActivity : ComponentActivity() {
    @OptIn(ExperimentalMaterial3Api::class)
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val settings = Settings(applicationContext)
        val apiClient = ApiClient(settings)

        WindowCompat.setDecorFitsSystemWindows(window, false)
        setContent {
            val systemUiController = rememberSystemUiController()
            val useDarkIcons = !isSystemInDarkTheme()
            val navController = rememberNavController()

            GolfTheme {
                SideEffect {
                    systemUiController.setSystemBarsColor(
                        color = Color.Transparent,
                        darkIcons = useDarkIcons
                    )
                }
                androidx.compose.material3.Scaffold(
                    topBar = {
                        androidx.compose.material3.TopAppBar(
                            title = { androidx.compose.material3.Text(text = "") },
                            modifier = Modifier.height(0.dp)
                        )
                    },
                    bottomBar = {
                        BottomNavigationBar(navController)
                    }
                ) { innerPadding ->
                    Box(
                        modifier = Modifier.imePadding().navigationBarsPadding()
                            .background(color = MaterialTheme.colorScheme.surface).fillMaxSize()
                    ) {
                        Navigation(navController)
                    }
                }
            }
        }
    }
}
