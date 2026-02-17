package app.gengoka.domain.repository

import app.gengoka.core.util.Resource
import app.gengoka.domain.model.AnswerWithDetails

interface FeedRepository {
    suspend fun getFeed(
        page: Int? = null,
        pageSize: Int? = null,
        filter: String? = null,
        categoryId: String? = null
    ): Resource<List<AnswerWithDetails>>

    suspend fun getTrending(
        page: Int? = null,
        pageSize: Int? = null
    ): Resource<List<AnswerWithDetails>>
}
