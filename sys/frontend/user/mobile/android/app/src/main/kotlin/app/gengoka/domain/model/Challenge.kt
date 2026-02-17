package app.gengoka.domain.model

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
    val category: Category?,
    val isCompleted: Boolean = false,
    val userAnswer: Answer? = null
)

data class DailyChallenge(
    val challenge: Challenge,
    val categoryName: String,
    val isCompleted: Boolean,
    val userAnswer: Answer? = null
)
