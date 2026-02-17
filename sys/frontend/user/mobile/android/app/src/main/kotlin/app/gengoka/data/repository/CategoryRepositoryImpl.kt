package app.gengoka.data.repository

import app.gengoka.core.util.Resource
import app.gengoka.core.util.safeApiCall
import app.gengoka.data.api.GengokApi
import app.gengoka.data.mapper.toDomain
import app.gengoka.domain.model.Category
import app.gengoka.domain.repository.CategoryRepository
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
