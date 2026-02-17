package app.gengoka.domain.repository

import app.gengoka.core.util.Resource
import app.gengoka.domain.model.AnswerWithDetails
import app.gengoka.domain.model.AnswerWithUser

interface AnswerRepository {
    suspend fun getAnswer(id: String): Resource<AnswerWithDetails>
    suspend fun createAnswer(challengeId: String, content: String): Resource<AnswerWithUser>
    suspend fun deleteAnswer(id: String): Resource<Unit>
    suspend fun likeAnswer(id: String): Resource<Unit>
    suspend fun unlikeAnswer(id: String): Resource<Unit>
    suspend fun getChallengeAnswers(
        challengeId: String,
        page: Int? = null,
        pageSize: Int? = null,
        sort: String? = null
    ): Resource<List<AnswerWithUser>>
}
