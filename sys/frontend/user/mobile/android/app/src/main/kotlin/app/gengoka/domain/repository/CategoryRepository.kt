package app.gengoka.domain.repository

import app.gengoka.core.util.Resource
import app.gengoka.domain.model.Category

interface CategoryRepository {
    suspend fun getCategories(): Resource<List<Category>>
    suspend fun getCategory(id: String): Resource<Category>
}
