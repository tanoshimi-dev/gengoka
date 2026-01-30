package app.dev.gengoka.data.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class CommentDto(
    val id: String,
    @SerialName("answer_id")
    val answerId: String,
    @SerialName("user_id")
    val userId: String,
    val content: String,
    val status: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String
)

@Serializable
data class CommentWithUserDto(
    val id: String,
    @SerialName("answer_id")
    val answerId: String,
    @SerialName("user_id")
    val userId: String,
    val content: String,
    val status: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String,
    val user: UserSummaryDto
)

@Serializable
data class CreateCommentRequest(
    val content: String
)
