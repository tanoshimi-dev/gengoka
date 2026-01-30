package app.dev.gengoka.domain.repository

import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.ChallengeWithCategory

interface ChallengeRepository {
    suspend fun getDailyChallenges(): Resource<List<ChallengeWithCategory>>
    suspend fun getChallenge(id: String): Resource<ChallengeWithCategory>
    suspend fun getChallengesByCategory(
        categoryId: String,
        page: Int? = null,
        pageSize: Int? = null
    ): Resource<List<ChallengeWithCategory>>
}
