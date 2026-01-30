package app.dev.gengoka.data.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class ChallengeDto(
    val id: String,
    @SerialName("category_id")
    val categoryId: String,
    val title: String,
    val description: String? = null,
    @SerialName("char_limit")
    val charLimit: Int,
    @SerialName("release_date")
    val releaseDate: String? = null,
    @SerialName("answer_count")
    val answerCount: Int,
    val status: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String
)

@Serializable
data class ChallengeWithCategoryDto(
    val id: String,
    @SerialName("category_id")
    val categoryId: String,
    val title: String,
    val description: String? = null,
    @SerialName("char_limit")
    val charLimit: Int,
    @SerialName("release_date")
    val releaseDate: String? = null,
    @SerialName("answer_count")
    val answerCount: Int,
    val status: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String,
    val category: CategoryDto
)
