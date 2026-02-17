package app.gengoka.domain.repository

import app.gengoka.core.util.Resource
import app.gengoka.domain.model.LinkedSocialAccount

interface AuthRepository {
    suspend fun login(email: String, password: String): Resource<Unit>
    suspend fun register(email: String, password: String, name: String): Resource<Unit>
    suspend fun socialLogin(provider: String, idToken: String?, accessToken: String?): Resource<Unit>
    suspend fun refreshToken(): Resource<Unit>
    suspend fun logout(): Resource<Unit>
    suspend fun getLinkedAccounts(): Resource<List<LinkedSocialAccount>>
    suspend fun linkAccount(provider: String, idToken: String?, accessToken: String?): Resource<LinkedSocialAccount>
    suspend fun unlinkAccount(provider: String): Resource<Unit>
}
