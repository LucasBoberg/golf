package se.lucasboberg.golf.utils

expect class Settings {
    fun get(key: String): String?
    fun set(key: String, value: String)
    fun remove(key: String)
}
