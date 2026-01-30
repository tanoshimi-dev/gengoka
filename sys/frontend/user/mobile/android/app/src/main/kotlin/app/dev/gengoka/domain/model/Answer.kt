package app.dev.gengoka.domain.model

data class Answer(
    val id: String,
    val challengeId: String,
    val userId: String,
    val content: String,
    val score: Int?,
    val aiFeedback: AiFeedback?,
    val likeCount: Int,
    val commentCount: Int,
    val viewCount: Int,
    val createdAt: String
)

data class AnswerWithUser(
    val id: String,
    val challengeId: String,
    val userId: String,
    val content: String,
    val score: Int?,
    val aiFeedback: AiFeedback?,
    val likeCount: Int,
    val commentCount: Int,
    val viewCount: Int,
    val createdAt: String,
    val user: UserSummary,
    val isLiked: Boolean
)

data class AnswerWithDetails(
    val id: String,
    val challengeId: String,
    val userId: String,
    val content: String,
    val score: Int?,
    val aiFeedback: AiFeedback?,
    val likeCount: Int,
    val commentCount: Int,
    val viewCount: Int,
    val createdAt: String,
    val user: UserSummary,
    val challenge: Challenge,
    val isLiked: Boolean
)

data class AiFeedback(
    val score: Int,
    val goodPoints: String,
    val improvement: String,
    val exampleAnswer: String
)
