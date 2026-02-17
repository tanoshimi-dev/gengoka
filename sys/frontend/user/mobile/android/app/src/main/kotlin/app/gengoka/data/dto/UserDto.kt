package app.gengoka.data.dto

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class UserDto(
    val id: String,
    val email: String? = null,
    val name: String,
    val avatar: String? = null,
    val bio: String? = null,
    @SerialName("total_likes")
    val totalLikes: Int,
    val status: String,
    @SerialName("created_at")
    val createdAt: String,
    @SerialName("updated_at")
    val updatedAt: String
)

@Serializable
data class UserProfileDto(
    val id: String,
    val name: String,
    val avatar: String? = null,
    val bio: String? = null,
    @SerialName("total_likes")
    val totalLikes: Int,
    @SerialName("answer_count")
    val answerCount: Long,
    @SerialName("follower_count")
    val followerCount: Long,
    @SerialName("following_count")
    val followingCount: Long,
    @SerialName("is_following")
    val isFollowing: Boolean
)

@Serializable
data class UserSummaryDto(
    val id: String,
    val name: String,
    val avatar: String? = null
)

@Serializable
data class CreateUserRequest(
    val email: String? = null,
    val name: String,
    val avatar: String? = null,
    val bio: String? = null
)

@Serializable
data class UpdateUserRequest(
    val name: String? = null,
    val avatar: String? = null,
    val bio: String? = null
)

@Serializable
data class UserStatsDto(
    @SerialName("total_challenges")
    val totalChallenges: Int = 0,
    @SerialName("completed_today")
    val completedToday: Int = 0,
    @SerialName("current_streak")
    val currentStreak: Int = 0,
    @SerialName("best_streak")
    val bestStreak: Int = 0,
    @SerialName("average_score")
    val averageScore: Double = 0.0
)
