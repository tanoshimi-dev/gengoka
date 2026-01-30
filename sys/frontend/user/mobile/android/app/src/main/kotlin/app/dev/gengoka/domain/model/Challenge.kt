package app.dev.gengoka.domain.model

data class Challenge(
    val id: String,
    val categoryId: String,
    val title: String,
    val description: String?,
    val charLimit: Int,
    val releaseDate: String?,
    val answerCount: Int
)

data class ChallengeWithCategory(
    val id: String,
    val categoryId: String,
    val title: String,
    val description: String?,
    val charLimit: Int,
    val releaseDate: String?,
    val answerCount: Int,
    val category: Category
)
