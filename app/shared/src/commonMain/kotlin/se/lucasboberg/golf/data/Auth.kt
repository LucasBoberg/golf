package se.lucasboberg.golf.data

import kotlinx.serialization.Serializable


@Serializable
data class SignUpInput(
    val email: String,
    val password: String,
    val firstName: String,
    val lastName: String,
    val hcp: Float
)

@Serializable
data class SignInInput(
    val email: String,
    val password: String
)

@Serializable
data class RefreshInput(
    val refreshToken: String
)

@Serializable
data class AuthResponse(
    val token: String,
    val refreshToken: String
)