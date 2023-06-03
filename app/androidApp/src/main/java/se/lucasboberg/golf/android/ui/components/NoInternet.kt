package se.lucasboberg.golf.android.ui.components

import androidx.compose.foundation.layout.*
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import se.lucasboberg.golf.android.R

@Composable
fun NoInternet(modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .fillMaxSize()
    ) {
        Column(
            modifier = Modifier
                .align(Alignment.Center)
                .padding(16.dp),
        ) {
            Icon(
                painter = painterResource(id = R.drawable.wifi_off),
                contentDescription = "No internet connection",
                modifier = Modifier
                    .size(108.dp)
                    .align(Alignment.CenterHorizontally)
            )
            Text(
                text = "No internet connection",
                fontSize = 18.sp,
                fontWeight = FontWeight.SemiBold,
            )
        }
    }
}

@Preview
@Composable
fun NoInternetPreview() {
    NoInternet()
}