package app.gengoka.data.repository

import app.gengoka.core.util.Resource
import app.gengoka.core.util.safeApiCall
import app.gengoka.data.api.GengokApi
import app.gengoka.data.mapper.toChallengeWithCategory
import app.gengoka.data.mapper.toDomain
import app.gengoka.domain.model.ChallengeWithCategory
import app.gengoka.domain.repository.ChallengeRepository
import javax.inject.Inject

class ChallengeRepositoryImpl @Inject constructor(
    private val api: GengokApi
) : ChallengeRepository {

    override suspend fun getDailyChallenges(): Resource<List<ChallengeWithCategory>> = safeApiCall {
        api.getDailyChallenges().data.map { it.toChallengeWithCategory() }
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
