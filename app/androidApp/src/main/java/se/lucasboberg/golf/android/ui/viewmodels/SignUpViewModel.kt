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

class SignUpViewModel(application: Application) : AndroidViewModel(application) {
    private val settings = Settings(application.applicationContext)
    private val apiClient = ApiClient(settings)

    val connectivityObserver = NetworkConnectivityObserver(application.applicationContext)

    private var _name = MutableStateFlow<String?>(null)
    val name = _name.asStateFlow()

    private var _user = MutableStateFlow<User?>(null)
    val user = _user.asStateFlow()

    private var _signedIn = MutableStateFlow(false)
    val signedIn = _signedIn.asStateFlow()

    private var _error = MutableStateFlow<String>("")
    val error = _error.asStateFlow()

    private var _loading = MutableStateFlow(false)
    val loading = _loading.asStateFlow()

    fun setName(firstName: String, lastName: String) {
        _name.value = "$firstName $lastName"
    }

    fun signUp(email: String, firstName: String, lastName: String) {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                _error.value = ""
                _user.value = apiClient.signUp(email, firstName, lastName)
            } catch (e: Exception) {
                _error.value = e.message ?: "Something went wrong"
            }
        }
    }

    fun verifyCode(email: String, code: String) {
        viewModelScope.launch(Dispatchers.IO) {
            try {
                _error.value = ""
                val authResponse = apiClient.verifyCode(email, code)
                settings.set("token", authResponse.token)
                settings.set("refreshToken", authResponse.refreshToken)
                _signedIn.value = true
            } catch (e: Exception) {
                _error.value = "Invalid code"
            }
        }
    }

    fun changeButtonStatus(status: Boolean){
        _loading.compareAndSet(!status, status)
    }

    fun clearError() {
        _error.value = ""
    }
}