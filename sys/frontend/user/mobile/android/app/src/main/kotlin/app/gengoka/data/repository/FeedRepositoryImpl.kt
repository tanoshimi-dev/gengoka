package app.gengoka.data.repository

import app.gengoka.core.util.Resource
import app.gengoka.core.util.safeApiCall
import app.gengoka.data.api.GengokApi
import app.gengoka.data.mapper.toDomain
import app.gengoka.domain.model.AnswerWithDetails
import app.gengoka.domain.repository.FeedRepository
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
