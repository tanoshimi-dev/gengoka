package app.dev.gengoka.data.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class AnswerDto(
    val id: String,
    @SerialName("challenge_id")
    val challengeId: String,
    @SerialName("user_id")
    val userId: String,
    val content: String,
    val score: Int? = null,
    @SerialName("ai_feedback")
    val aiFeedback: AiFeedbackDto? = null,
    @SerialName("like_count")
    val likeCount: Int,
    @SerialName("comment_count")
    val commentCount: Int,
    @SerialName("view_count")
    val viewCount: Int,
    val status: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String
)

@Serializable
data class AnswerWithUserDto(
    val id: String,
    @SerialName("challenge_id")
    val challengeId: String,
    @SerialName("user_id")
    val userId: String,
    val content: String,
    val score: Int? = null,
    @SerialName("ai_feedback")
    val aiFeedback: AiFeedbackDto? = null,
    @SerialName("like_count")
    val likeCount: Int,
    @SerialName("comment_count")
    val commentCount: Int,
    @SerialName("view_count")
    val viewCount: Int,
    val status: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String,
    val user: UserSummaryDto,
    @SerialName("is_liked")
    val isLiked: Boolean
)

@Serializable
data class AnswerWithDetailsDto(
    val id: String,
    @SerialName("challenge_id")
    val challengeId: String,
    @SerialName("user_id")
    val userId: String,
    val content: String,
    val score: Int? = null,
    @SerialName("ai_feedback")
    val aiFeedback: AiFeedbackDto? = null,
    @SerialName("like_count")
    val likeCount: Int,
    @SerialName("comment_count")
    val commentCount: Int,
    @SerialName("view_count")
    val viewCount: Int,
    val status: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String,
    val user: UserSummaryDto,
    val challenge: ChallengeDto,
    @SerialName("is_liked")
    val isLiked: Boolean
)

@Serializable
data class AiFeedbackDto(
    val score: Int,
    @SerialName("good_points")
    val goodPoints: String,
    val improvement: String,
    @SerialName("example_answer")
    val exampleAnswer: String
)

@Serializable
data class CreateAnswerRequest(
    val content: String
)
