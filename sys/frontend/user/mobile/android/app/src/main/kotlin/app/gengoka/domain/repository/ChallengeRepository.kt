package app.gengoka.domain.repository

import app.gengoka.core.util.Resource
import app.gengoka.domain.model.ChallengeWithCategory

interface ChallengeRepository {
    suspend fun getDailyChallenges(): Resource<List<ChallengeWithCategory>>
    suspend fun getChallenge(id: String): Resource<ChallengeWithCategory>
    suspend fun getChallengesByCategory(
        categoryId: String,
        page: Int? = null,
        pageSize: Int? = null
    ): Resource<List<ChallengeWithCategory>>
}
