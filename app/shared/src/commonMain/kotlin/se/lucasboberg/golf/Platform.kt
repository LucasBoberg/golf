package se.lucasboberg.golf

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform