package se.lucasboberg.golf.android.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import se.lucasboberg.golf.android.Screen
import se.lucasboberg.golf.android.ui.components.NoInternet
import se.lucasboberg.golf.android.ui.viewmodels.SignInViewModel
import se.lucasboberg.golf.android.utils.ConnectivityObserver

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SignInScreen(navController: NavController) {
    val viewModel = viewModel<SignInViewModel>()
    val connectivityStatus by viewModel.connectivityObserver
        .observe()
        .collectAsStateWithLifecycle(
            ConnectivityObserver.Status.Available
        )
    var email by remember { mutableStateOf("") }
    var otpCode by remember { mutableStateOf("") }
    val user by viewModel.user.collectAsStateWithLifecycle()
    val signedIn by viewModel.signedIn.collectAsStateWithLifecycle()
    val error by viewModel.error.collectAsStateWithLifecycle()
    LaunchedEffect(signedIn) {
        signedIn.let {
            if (it) {
                navController.navigate(Screen.RoundsScreen.route) {
                    popUpTo(Screen.SignInScreen.route) { inclusive = true }
                }
            }
        }
    }
    if (connectivityStatus == ConnectivityObserver.Status.Available) {
        TopAppBar(
            title = { Text(text = "") },
            navigationIcon = {
                IconButton(onClick = { navController.popBackStack() }) {
                    Icon(
                        imageVector = Icons.Default.ArrowBack,
                        contentDescription = "Back"
                    )
                }
            }
        )
        Box(
            contentAlignment = Alignment.Center,
            modifier = Modifier
                .fillMaxSize()
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
            ) {
                if (user == null) {
                    Text(
                        text = "Sign In",
                        style = MaterialTheme.typography.headlineLarge,
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 32.dp, vertical = 16.dp),
                        fontSize = 32.sp
                    )
                    OutlinedTextField(
                        value = email,
                        onValueChange = {
                            email = it
                            viewModel.clearError()
                        },
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 32.dp),
                        label = { Text(text = "Email") },
                        singleLine = true
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(
                        text = error,
                        color = MaterialTheme.colorScheme.error,
                        modifier = Modifier.padding(horizontal = 32.dp)
                    )
                    Button(
                        onClick = { viewModel.signIn(email) },
                        modifier = Modifier
                            .align(Alignment.CenterHorizontally)
                            .fillMaxWidth(1 / 2f),
                        content = { Text(text = "Sign In", fontSize = 16.sp) }
                    )
                }
            }
        }
    } else {
        NoInternet()
    }
}

@Preview
@Composable
fun ComposableSignInPreview() {
    SignInScreen(navController = rememberNavController())
}
