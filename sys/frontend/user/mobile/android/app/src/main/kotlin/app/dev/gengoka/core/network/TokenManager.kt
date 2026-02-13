package app.dev.gengoka.core.network

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import app.dev.gengoka.data.dto.AuthTokensDto
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.runBlocking
import javax.inject.Inject
import javax.inject.Singleton

@Singleton
class TokenManager @Inject constructor(
    private val dataStore: DataStore<Preferences>
) {
    private val accessTokenKey = stringPreferencesKey("access_token")
    private val refreshTokenKey = stringPreferencesKey("refresh_token")
    private val userIdKey = stringPreferencesKey("auth_user_id")
    private val userNameKey = stringPreferencesKey("auth_user_name")
    private val userAvatarKey = stringPreferencesKey("auth_user_avatar")

    private var cachedAccessToken: String? = null
    private var cachedRefreshToken: String? = null
    private var cachedUserId: String? = null
    private var cachedUserName: String? = null
    private var cachedUserAvatar: String? = null

    private val _isAuthenticated = MutableStateFlow(false)
    val isAuthenticated: StateFlow<Boolean> = _isAuthenticated.asStateFlow()

    init {
        runBlocking {
            val prefs = dataStore.data.first()
            cachedAccessToken = prefs[accessTokenKey]
            cachedRefreshToken = prefs[refreshTokenKey]
            cachedUserId = prefs[userIdKey]
            cachedUserName = prefs[userNameKey]
            cachedUserAvatar = prefs[userAvatarKey]
            _isAuthenticated.value = cachedAccessToken != null
        }
    }

    fun getAccessToken(): String? = cachedAccessToken

    fun getRefreshToken(): String? = cachedRefreshToken

    fun getUserId(): String? = cachedUserId

    fun getUserName(): String? = cachedUserName

    suspend fun saveTokens(tokens: AuthTokensDto) {
        cachedAccessToken = tokens.accessToken
        cachedRefreshToken = tokens.refreshToken
        cachedUserId = tokens.user.id
        cachedUserName = tokens.user.name
        cachedUserAvatar = tokens.user.avatar

        dataStore.edit { prefs ->
            prefs[accessTokenKey] = tokens.accessToken
            prefs[refreshTokenKey] = tokens.refreshToken
            prefs[userIdKey] = tokens.user.id
            prefs[userNameKey] = tokens.user.name
            tokens.user.avatar?.let { prefs[userAvatarKey] = it }
                ?: prefs.remove(userAvatarKey)
        }

        _isAuthenticated.value = true
    }

    suspend fun clearTokens() {
        cachedAccessToken = null
        cachedRefreshToken = null
        cachedUserId = null
        cachedUserName = null
        cachedUserAvatar = null

        dataStore.edit { prefs ->
            prefs.remove(accessTokenKey)
            prefs.remove(refreshTokenKey)
            prefs.remove(userIdKey)
            prefs.remove(userNameKey)
            prefs.remove(userAvatarKey)
        }

        _isAuthenticated.value = false
    }
}
