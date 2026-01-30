package app.dev.gengoka.domain.repository

import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.AnswerWithDetails

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
