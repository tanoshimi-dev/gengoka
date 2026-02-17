package app.gengoka.data.repository

import app.gengoka.core.util.Resource
import app.gengoka.core.util.safeApiCall
import app.gengoka.data.api.GengokApi
import app.gengoka.data.dto.CreateAnswerRequest
import app.gengoka.data.mapper.toDomain
import app.gengoka.domain.model.AnswerWithDetails
import app.gengoka.domain.model.AnswerWithUser
import app.gengoka.domain.repository.AnswerRepository
import javax.inject.Inject

class AnswerRepositoryImpl @Inject constructor(
    private val api: GengokApi
) : AnswerRepository {

    override suspend fun getAnswer(id: String): Resource<AnswerWithDetails> = safeApiCall {
        api.getAnswer(id).data.toDomain()
    }

    override suspend fun createAnswer(
        challengeId: String,
        content: String
    ): Resource<AnswerWithUser> = safeApiCall {
        api.createAnswer(challengeId, CreateAnswerRequest(content)).data.toDomain()
    }

    override suspend fun deleteAnswer(id: String): Resource<Unit> = safeApiCall {
        api.deleteAnswer(id)
        Unit
    }

    override suspend fun likeAnswer(id: String): Resource<Unit> = safeApiCall {
        api.likeAnswer(id)
        Unit
    }

    override suspend fun unlikeAnswer(id: String): Resource<Unit> = safeApiCall {
        api.unlikeAnswer(id)
        Unit
    }

    override suspend fun getChallengeAnswers(
        challengeId: String,
        page: Int?,
        pageSize: Int?,
        sort: String?
    ): Resource<List<AnswerWithUser>> = safeApiCall {
        api.getChallengeAnswers(challengeId, page, pageSize, sort).data.map { it.toDomain() }
    }
}
