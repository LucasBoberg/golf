package se.lucasboberg.golf.android.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.*
import androidx.compose.runtime.Composable

private val DarkColorPalette = darkColorScheme(
    primary = Colors.Primary,
    onPrimary = Colors.onPrimary,
    secondary = Colors.Accent,
    surfaceTint = Colors.Primary,
    primaryContainer = Colors.Primary,
    secondaryContainer = Colors.Primary,
    tertiary = Colors.Accent,
    surfaceVariant = Colors.CardBackgroundDark,
    background = Colors.BackgroundDark,
    error = Colors.Error,
)

private val LightColorPalette = lightColorScheme(
    primary = Colors.Primary,
    onPrimary = Colors.onPrimary,
    secondary = Colors.Accent,
    surfaceTint = Colors.Primary,
    primaryContainer = Colors.Primary,
    secondaryContainer = Colors.Primary,
    onSecondaryContainer = Colors.onPrimary,
    tertiary = Colors.Accent,
    surfaceVariant = Colors.CardBackgroundLight,
    background = Colors.BackgroundLight,
    error = Colors.Error,
)

@Composable
fun GolfTheme(darkTheme: Boolean = isSystemInDarkTheme(), content: @Composable() () -> Unit) {
    val colors = if (darkTheme) {
        DarkColorPalette
    } else {
        LightColorPalette
    }

    MaterialTheme(
        typography = Typography,
        shapes = Shapes,
        content = content,
        colorScheme = colors
    )
}