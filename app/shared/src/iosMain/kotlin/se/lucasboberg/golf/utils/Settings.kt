package se.lucasboberg.golf.utils

import com.liftric.kvault.KVault

actual class Settings {
    private val store: KVault = KVault()

    actual fun get(key: String): String? {
        return store.string(key)
    }

    actual fun set(key: String, value: String) {
        store.set(key, value)
    }

    actual fun remove(key: String) {
        store.deleteObject(key)
    }
}