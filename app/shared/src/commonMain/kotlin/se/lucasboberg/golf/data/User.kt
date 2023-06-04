package se.lucasboberg.golf.data

import kotlinx.serialization.Serializable

@Serializable
data class User(
    val id: String,
    val email: String,
    val firstName: String,
    val lastName: String,
    val hcp: Float,
    val createdAt: String,
    val updatedAt: String
)
