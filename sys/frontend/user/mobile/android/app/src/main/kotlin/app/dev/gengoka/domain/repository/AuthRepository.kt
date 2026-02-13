package app.dev.gengoka.domain.repository

import app.dev.gengoka.core.util.Resource

interface AuthRepository {
    suspend fun login(email: String, password: String): Resource<Unit>
    suspend fun register(email: String, password: String, name: String): Resource<Unit>
    suspend fun refreshToken(): Resource<Unit>
    suspend fun logout(): Resource<Unit>
}
