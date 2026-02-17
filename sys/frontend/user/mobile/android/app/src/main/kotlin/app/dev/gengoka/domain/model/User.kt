package app.dev.gengoka.domain.model

data class User(
    val id: String,
    val email: String?,
    val name: String,
    val avatar: String?,
    val bio: String?,
    val totalLikes: Int
)

data class UserProfile(
    val id: String,
    val name: String,
    val avatar: String?,
    val bio: String?,
    val totalLikes: Int,
    val answerCount: Long,
    val followerCount: Long,
    val followingCount: Long,
    val isFollowing: Boolean
)

data class UserSummary(
    val id: String,
    val name: String,
    val avatar: String?
) {
    fun getInitial(): String = name.firstOrNull()?.toString() ?: "?"
}

data class UserStats(
    val totalChallenges: Int,
    val completedToday: Int,
    val currentStreak: Int,
    val bestStreak: Int,
    val averageScore: Double
)

data class LinkedSocialAccount(
    val provider: String,
    val providerEmail: String?,
    val providerName: String?,
    val linkedAt: String
)
