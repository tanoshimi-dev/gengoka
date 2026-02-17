package app.gengoka.domain.repository

import app.gengoka.core.util.Resource
import app.gengoka.domain.model.CommentWithUser

interface CommentRepository {
    suspend fun getAnswerComments(
        answerId: String,
        page: Int? = null,
        pageSize: Int? = null
    ): Resource<List<CommentWithUser>>
    suspend fun createComment(answerId: String, content: String): Resource<CommentWithUser>
    suspend fun deleteComment(id: String): Resource<Unit>
}
