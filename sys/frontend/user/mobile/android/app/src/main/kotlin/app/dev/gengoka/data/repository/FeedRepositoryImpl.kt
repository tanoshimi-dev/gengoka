package app.dev.gengoka.data.repository

import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.core.util.safeApiCall
import app.dev.gengoka.data.api.GengokApi
import app.dev.gengoka.data.mapper.toDomain
import app.dev.gengoka.domain.model.AnswerWithDetails
import app.dev.gengoka.domain.repository.FeedRepository
import javax.inject.Inject

class FeedRepositoryImpl @Inject constructor(
    private val api: GengokApi
) : FeedRepository {

    override suspend fun getFeed(
        page: Int?,
        pageSize: Int?,
        filter: String?,
        categoryId: String?
    ): Resource<List<AnswerWithDetails>> = safeApiCall {
        api.getFeed(page, pageSize, filter, categoryId).data.map { it.toDomain() }
    }

    override suspend fun getTrending(
        page: Int?,
        pageSize: Int?
    ): Resource<List<AnswerWithDetails>> = safeApiCall {
        api.getTrending(page, pageSize).data.map { it.toDomain() }
    }
}
