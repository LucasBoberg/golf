package se.lucasboberg.golf.android.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.rememberNavController
import se.labbs.app.android.Screen
import se.labbs.app.android.ui.components.NoInternet
import se.labbs.app.android.ui.components.OTPField
import se.lucasboberg.golf.android.ui.viewmodels.SignUpViewModel
import se.labbs.app.android.utils.ConnectivityObserver

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SignUpScreen(navController: NavController) {
    val viewModel = viewModel<SignUpViewModel>()
    val connectivityStatus by viewModel.connectivityObserver
        .observe()
        .collectAsStateWithLifecycle(
            ConnectivityObserver.Status.Available
        )
    var firstName by remember { mutableStateOf("") }
    var lastName by remember { mutableStateOf("") }
    var email by remember { mutableStateOf("") }
    var otpCode by remember { mutableStateOf("") }
    val name by viewModel.name.collectAsStateWithLifecycle()
    val user by viewModel.user.collectAsStateWithLifecycle()
    val signedIn by viewModel.signedIn.collectAsStateWithLifecycle()
    val error by viewModel.error.collectAsStateWithLifecycle()
    val loading by viewModel.loading.collectAsStateWithLifecycle()
    LaunchedEffect(signedIn) {
        signedIn.let {
            if (it) {
                navController.navigate(Screen.EventsScreen.route) {
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
                    if (name == null) {
                        Text(
                            text = "Sign Up",
                            style = MaterialTheme.typography.headlineLarge,
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(horizontal = 32.dp, vertical = 16.dp),
                            fontSize = 32.sp
                        )
                        OutlinedTextField(
                            value = firstName,
                            onValueChange = {
                                firstName = it
                            },
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(horizontal = 32.dp),
                            label = { Text(text = "First name") },
                            singleLine = true,
                            keyboardOptions = KeyboardOptions(
                                capitalization = KeyboardCapitalization.Words
                            )
                        )
                        OutlinedTextField(
                            value = lastName,
                            onValueChange = {
                                lastName = it
                            },
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(horizontal = 32.dp),
                            label = { Text(text = "Last name") },
                            singleLine = true,
                            keyboardOptions = KeyboardOptions(
                                capitalization = KeyboardCapitalization.Words
                            )
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                        Button(
                            onClick = { viewModel.setName(firstName, lastName) },
                            modifier = Modifier
                                .align(Alignment.CenterHorizontally)
                                .fillMaxWidth(1 / 2f),
                            content = { Text(text = "Continue", fontSize = 16.sp) }
                        )
                    } else {
                        Text(
                            text = "Hi, \n$name",
                            style = MaterialTheme.typography.headlineLarge,
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(horizontal = 32.dp),
                            fontSize = 32.sp
                        )
                        Text(
                            text = "What is your email?",
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(bottom = 16.dp, start = 32.dp, end = 32.dp, top = 4.dp),
                            fontSize = 16.sp
                        )
                        OutlinedTextField(
                            value = email,
                            onValueChange = {
                                email = it
                                viewModel.clearError()
                                viewModel.changeButtonStatus(true)
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
                            onClick = {
                                viewModel.signUp(email, firstName, lastName)
                                viewModel.changeButtonStatus(false)
                                      },
                            enabled = loading,
                            modifier = Modifier
                                .align(Alignment.CenterHorizontally)
                                .fillMaxWidth(1 / 2f),
                            content = { Text(text = "Sign Up", fontSize = 16.sp) }
                        )
                        if (!loading && !email.isEmpty() && error.isEmpty()) {
                            Box(
                                contentAlignment = Alignment.Center,
                                modifier = Modifier
                                    .align(Alignment.CenterHorizontally)
                            ) {
                                CircularProgressIndicator()
                            }
                        }
                    }
                } else {
                    Text(
                        text = "Verify",
                        style = MaterialTheme.typography.headlineLarge,
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 32.dp, vertical = 16.dp),
                        fontSize = 32.sp
                    )
                    OTPField(otpCode, onValueChange = {
                        otpCode = it
                        viewModel.clearError()
                    })
                    Text(
                        text = error,
                        color = MaterialTheme.colorScheme.error,
                        modifier = Modifier.padding(horizontal = 32.dp)
                    )
                    Button(
                        onClick = { viewModel.verifyCode(email, otpCode) },
                        modifier = Modifier
                            .align(Alignment.CenterHorizontally)
                            .fillMaxWidth(1 / 2f),
                        content = { Text(text = "Verify", fontSize = 16.sp) }
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
fun ComposableSignUpPreview() {
    SignUpScreen(navController = rememberNavController())
}
