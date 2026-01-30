package app.dev.gengoka.data.repository

import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.core.util.safeApiCall
import app.dev.gengoka.data.api.GengokApi
import app.dev.gengoka.data.mapper.toDomain
import app.dev.gengoka.domain.model.Category
import app.dev.gengoka.domain.repository.CategoryRepository
import javax.inject.Inject

class CategoryRepositoryImpl @Inject constructor(
    private val api: GengokApi
) : CategoryRepository {

    override suspend fun getCategories(): Resource<List<Category>> = safeApiCall {
        api.getCategories().data.map { it.toDomain() }
    }

    override suspend fun getCategory(id: String): Resource<Category> = safeApiCall {
        api.getCategory(id).data.toDomain()
    }
}
