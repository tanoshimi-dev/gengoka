package app.dev.gengoka.domain.model

data class Comment(
    val id: String,
    val answerId: String,
    val userId: String,
    val content: String,
    val createdAt: String
)

data class CommentWithUser(
    val id: String,
    val answerId: String,
    val userId: String,
    val content: String,
    val createdAt: String,
    val user: UserSummary
)
