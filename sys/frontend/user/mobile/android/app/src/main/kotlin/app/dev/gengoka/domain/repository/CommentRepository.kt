package app.dev.gengoka.domain.repository

import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.CommentWithUser

interface CommentRepository {
    suspend fun getAnswerComments(
        answerId: String,
        page: Int? = null,
        pageSize: Int? = null
    ): Resource<List<CommentWithUser>>
    suspend fun createComment(answerId: String, content: String): Resource<CommentWithUser>
    suspend fun deleteComment(id: String): Resource<Unit>
}
