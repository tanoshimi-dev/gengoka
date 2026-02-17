package app.gengoka.data.repository

import app.gengoka.core.util.Resource
import app.gengoka.core.util.safeApiCall
import app.gengoka.data.api.GengokApi
import app.gengoka.data.dto.CreateCommentRequest
import app.gengoka.data.mapper.toDomain
import app.gengoka.domain.model.CommentWithUser
import app.gengoka.domain.repository.CommentRepository
import javax.inject.Inject

class CommentRepositoryImpl @Inject constructor(
    private val api: GengokApi
) : CommentRepository {

    override suspend fun getAnswerComments(
        answerId: String,
        page: Int?,
        pageSize: Int?
    ): Resource<List<CommentWithUser>> = safeApiCall {
        api.getAnswerComments(answerId, page, pageSize).data.map { it.toDomain() }
    }

    override suspend fun createComment(
        answerId: String,
        content: String
    ): Resource<CommentWithUser> = safeApiCall {
        api.createComment(answerId, CreateCommentRequest(content)).data.toDomain()
    }

    override suspend fun deleteComment(id: String): Resource<Unit> = safeApiCall {
        api.deleteComment(id)
        Unit
    }
}
