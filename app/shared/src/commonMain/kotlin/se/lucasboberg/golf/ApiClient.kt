package se.lucasboberg.golf

import io.github.aakira.napier.DebugAntilog
import io.github.aakira.napier.Napier
import io.ktor.client.HttpClient
import io.ktor.client.call.body
import io.ktor.client.plugins.auth.Auth
import io.ktor.client.plugins.auth.providers.BearerTokens
import io.ktor.client.plugins.auth.providers.bearer
import io.ktor.client.plugins.contentnegotiation.ContentNegotiation
import io.ktor.client.plugins.logging.LogLevel
import io.ktor.client.plugins.logging.Logger
import io.ktor.client.plugins.logging.Logging
import io.ktor.client.request.post
import io.ktor.client.request.setBody
import io.ktor.http.ContentType
import io.ktor.http.contentType
import io.ktor.serialization.kotlinx.json.json
import kotlinx.coroutines.MainScope
import se.lucasboberg.golf.data.AuthResponse
import se.lucasboberg.golf.data.RefreshInput
import se.lucasboberg.golf.data.SignInInput
import se.lucasboberg.golf.data.SignUpInput
import se.lucasboberg.golf.data.User
import se.lucasboberg.golf.utils.Settings

class ApiClient(settings: Settings) {
    private val scope = MainScope()

    private val client = HttpClient {
        install(Logging) {
            logger = object: Logger {
                override fun log(message: String) {
                    Napier.v(message, null, "HTTP Client")
                }
            }
            level = LogLevel.ALL
        }
        install(ContentNegotiation) {
            json()
        }
        install(Auth) {
            bearer {
                loadTokens { BearerTokens(settings.get("token") ?: "", settings.get("refreshToken") ?: "") }
                refreshTokens {
                    val response = client.post("${BuildKonfig.BASE_URL}/auth/refresh") {
                        contentType(ContentType.Application.Json)
                        setBody(RefreshInput(settings.get("refreshToken") ?: ""))
                    }.body<AuthResponse>()
                    settings.set("token", response.token)
                    settings.set("refreshToken", response.refreshToken)
                    BearerTokens(response.token, response.refreshToken)
                }
            }
        }.also { Napier.base(DebugAntilog()) }
    }

    suspend fun signIn(email: String, password: String): Result<AuthResponse> {
        return try {
            val response = client.post("${BuildKonfig.BASE_URL}/auth/sign-in") {
                contentType(ContentType.Application.Json)
                setBody(SignInInput(email, password))
            }.body<AuthResponse>()
            Result.success(response)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun signUp(
        email: String,
        password: String,
        firstName: String,
        lastName: String,
        hcp: Float
    ): Result<User> {
        return try {
            val response = client.post("${BuildKonfig.BASE_URL}/auth/sign-up") {
                contentType(ContentType.Application.Json)
                setBody(SignUpInput(email, password, firstName, lastName, hcp))
            }.body<User>()
            Result.success(response)
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
}