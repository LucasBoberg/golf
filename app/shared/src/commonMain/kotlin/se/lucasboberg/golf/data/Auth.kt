package se.lucasboberg.golf.data

import kotlinx.serialization.Serializable

@Serializable
data class RefreshInput(
    val refreshToken: String
)

@Serializable
data class AuthResponse(
    val token: String,
    val refreshToken: String
)