package se.lucasboberg.golf.android.ui.viewmodels

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import se.lucasboberg.golf.ApiClient
import se.lucasboberg.golf.android.utils.NetworkConnectivityObserver
import se.lucasboberg.golf.utils.Settings

class SignInViewModel(application: Application) : AndroidViewModel(application) {
    private val settings = Settings(application.applicationContext)
    private val apiClient = ApiClient(settings)

    val connectivityObserver = NetworkConnectivityObserver(application.applicationContext)

    private var _user = MutableStateFlow<SignInResponse?>(null)
    val user = _user.asStateFlow()

    private var _signedIn = MutableStateFlow(false)
    val signedIn = _signedIn.asStateFlow()

    private var _error = MutableStateFlow<String>("")
    val error = _error.asStateFlow()

    fun signIn(email: String) {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                _error.value = ""
                _user.value = apiClient.signIn(email)
            } catch (e: Exception) {
                _error.value = e.message ?: "Something went wrong"
            }
        }
    }

    fun clearError() {
        _error.value = ""
    }
}