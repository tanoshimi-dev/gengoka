package app.dev.gengoka.core.network

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.runBlocking
import java.util.UUID
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class UserIdProvider @Inject constructor(
    private val dataStore: DataStore<Preferences>
) {
    private val userIdKey = stringPreferencesKey("user_id")

    private var cachedUserId: String? = null

    fun getUserId(): String {
        cachedUserId?.let { return it }

        return runBlocking {
            val stored = dataStore.data.map { it[userIdKey] }.first()
            if (stored != null) {
                cachedUserId = stored
                stored
            } else {
                val newId = UUID.randomUUID().toString()
                dataStore.edit { it[userIdKey] = newId }
                cachedUserId = newId
                newId
            }
        }
    }

    suspend fun getUserIdAsync(): String {
        cachedUserId?.let { return it }

        val stored = dataStore.data.map { it[userIdKey] }.first()
        return if (stored != null) {
            cachedUserId = stored
            stored
        } else {
            val newId = UUID.randomUUID().toString()
            dataStore.edit { it[userIdKey] = newId }
            cachedUserId = newId
            newId
        }
    }
}
