package app.dev.gengoka.data.repository

import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.core.util.safeApiCall
import app.dev.gengoka.data.api.GengokApi
import app.dev.gengoka.data.mapper.toDomain
import app.dev.gengoka.domain.model.ChallengeWithCategory
import app.dev.gengoka.domain.repository.ChallengeRepository
import javax.inject.Inject

class ChallengeRepositoryImpl @Inject constructor(
    private val api: GengokApi
) : ChallengeRepository {

    override suspend fun getDailyChallenges(): Resource<List<ChallengeWithCategory>> = safeApiCall {
        api.getDailyChallenges().data.map { it.toDomain() }
    }

    override suspend fun getChallenge(id: String): Resource<ChallengeWithCategory> = safeApiCall {
        api.getChallenge(id).data.toDomain()
    }

    override suspend fun getChallengesByCategory(
        categoryId: String,
        page: Int?,
        pageSize: Int?
    ): Resource<List<ChallengeWithCategory>> = safeApiCall {
        api.getChallengesByCategory(categoryId, page, pageSize).data.map { it.toDomain() }
    }
}
