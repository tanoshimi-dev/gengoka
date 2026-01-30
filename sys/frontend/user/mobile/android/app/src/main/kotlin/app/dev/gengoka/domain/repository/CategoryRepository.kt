package app.dev.gengoka.domain.repository

import app.dev.gengoka.core.util.Resource
import app.dev.gengoka.domain.model.Category

interface CategoryRepository {
    suspend fun getCategories(): Resource<List<Category>>
    suspend fun getCategory(id: String): Resource<Category>
}
