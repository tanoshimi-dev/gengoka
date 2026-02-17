package app.gengoka.domain.repository

import app.gengoka.core.util.Resource
import app.gengoka.domain.model.AnswerWithDetails
import app.gengoka.domain.model.User
import app.gengoka.domain.model.UserProfile
import app.gengoka.domain.model.UserStats
import app.gengoka.domain.model.UserSummary

interface UserRepository {
    suspend fun getUserStats(): Resource<UserStats>
    suspend fun getUser(id: String): Resource<UserProfile>
    suspend fun createUser(
        name: String,
        email: String? = null,
        avatar: String? = null,
        bio: String? = null
    ): Resource<User>
    suspend fun updateUser(
        id: String,
        name: String? = null,
        avatar: String? = null,
        bio: String? = null
    ): Resource<User>
    suspend fun getUserAnswers(
        userId: String,
        page: Int? = null,
        pageSize: Int? = null
    ): Resource<List<AnswerWithDetails>>
    suspend fun followUser(id: String): Resource<Unit>
    suspend fun unfollowUser(id: String): Resource<Unit>
    suspend fun getFollowers(
        userId: String,
        page: Int? = null,
        pageSize: Int? = null
    ): Resource<List<UserSummary>>
    suspend fun getFollowing(
        userId: String,
        page: Int? = null,
        pageSize: Int? = null
    ): Resource<List<UserSummary>>
}
