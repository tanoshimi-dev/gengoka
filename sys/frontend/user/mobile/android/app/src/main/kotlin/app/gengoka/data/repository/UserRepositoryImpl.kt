package app.gengoka.data.repository

import app.gengoka.core.util.Resource
import app.gengoka.core.util.safeApiCall
import app.gengoka.data.api.GengokApi
import app.gengoka.data.dto.CreateUserRequest
import app.gengoka.data.dto.UpdateUserRequest
import app.gengoka.data.mapper.toDomain
import app.gengoka.domain.model.AnswerWithDetails
import app.gengoka.domain.model.User
import app.gengoka.domain.model.UserProfile
import app.gengoka.domain.model.UserStats
import app.gengoka.domain.model.UserSummary
import app.gengoka.domain.repository.UserRepository
import javax.inject.Inject

class UserRepositoryImpl @Inject constructor(
    private val api: GengokApi
) : UserRepository {

    override suspend fun getUserStats(): Resource<UserStats> = safeApiCall {
        api.getUserStats().data.toDomain()
    }

    override suspend fun getUser(id: String): Resource<UserProfile> = safeApiCall {
        api.getUser(id).data.toDomain()
    }

    override suspend fun createUser(
        name: String,
        email: String?,
        avatar: String?,
        bio: String?
    ): Resource<User> = safeApiCall {
        api.createUser(CreateUserRequest(email, name, avatar, bio)).data.toDomain()
    }

    override suspend fun updateUser(
        id: String,
        name: String?,
        avatar: String?,
        bio: String?
    ): Resource<User> = safeApiCall {
        api.updateUser(id, UpdateUserRequest(name, avatar, bio)).data.toDomain()
    }

    override suspend fun getUserAnswers(
        userId: String,
        page: Int?,
        pageSize: Int?
    ): Resource<List<AnswerWithDetails>> = safeApiCall {
        api.getUserAnswers(userId, page, pageSize).data.map { it.toDomain() }
    }

    override suspend fun followUser(id: String): Resource<Unit> = safeApiCall {
        api.followUser(id)
        Unit
    }

    override suspend fun unfollowUser(id: String): Resource<Unit> = safeApiCall {
        api.unfollowUser(id)
        Unit
    }

    override suspend fun getFollowers(
        userId: String,
        page: Int?,
        pageSize: Int?
    ): Resource<List<UserSummary>> = safeApiCall {
        api.getFollowers(userId, page, pageSize).data.map { it.toDomain() }
    }

    override suspend fun getFollowing(
        userId: String,
        page: Int?,
        pageSize: Int?
    ): Resource<List<UserSummary>> = safeApiCall {
        api.getFollowing(userId, page, pageSize).data.map { it.toDomain() }
    }
}
