package app.dev.gengoka.data.mapper

import app.dev.gengoka.data.dto.*
import app.dev.gengoka.domain.model.*

fun CategoryDto.toDomain() = Category(
    id = id,
    name = name,
    description = description,
    icon = iconName,
    color = colorHex,
    challengeCount = challengeCount
)

fun ChallengeDto.toDomain() = Challenge(
    id = id,
    categoryId = categoryId,
    title = title,
    description = description,
    charLimit = charLimit,
    releaseDate = releaseDate,
    answerCount = answerCount
)

fun AnswerDto.toDomain() = Answer(
    id = id,
    challengeId = challengeId,
    userId = userId,
    content = content,
    score = score,
    aiFeedback = aiFeedback?.toDomain(),
    likeCount = likeCount,
    commentCount = commentCount,
    viewCount = viewCount,
    createdAt = createdAt
)

fun DailyChallengeResponseDto.toDomain() = DailyChallenge(
    challenge = challenge.toDomain(),
    categoryName = categoryName,
    isCompleted = isCompleted,
    userAnswer = userAnswer?.toDomain()
)

fun DailyChallengeResponseDto.toChallengeWithCategory() = ChallengeWithCategory(
    id = challenge.id,
    categoryId = challenge.categoryId,
    title = challenge.title,
    description = challenge.description,
    charLimit = challenge.charLimit,
    releaseDate = challenge.releaseDate,
    answerCount = challenge.answerCount,
    category = Category(
        id = challenge.categoryId,
        name = categoryName,
        description = null,
        icon = null,
        color = null
    ),
    isCompleted = isCompleted,
    userAnswer = userAnswer?.toDomain()
)

fun ChallengeWithCategoryDto.toDomain() = ChallengeWithCategory(
    id = id,
    categoryId = categoryId,
    title = title,
    description = description,
    charLimit = charLimit,
    releaseDate = releaseDate,
    answerCount = answerCount,
    category = category?.toDomain()
)

fun AiFeedbackDto.toDomain() = AiFeedback(
    score = score,
    goodPoints = goodPoints,
    improvement = improvement,
    exampleAnswer = exampleAnswer
)

fun UserSummaryDto.toDomain() = UserSummary(
    id = id,
    name = name,
    avatar = avatar
)

fun UserDto.toDomain() = User(
    id = id,
    email = email,
    name = name,
    avatar = avatar,
    bio = bio,
    totalLikes = totalLikes
)

fun UserProfileDto.toDomain() = UserProfile(
    id = id,
    name = name,
    avatar = avatar,
    bio = bio,
    totalLikes = totalLikes,
    answerCount = answerCount,
    followerCount = followerCount,
    followingCount = followingCount,
    isFollowing = isFollowing
)

fun AnswerWithUserDto.toDomain() = AnswerWithUser(
    id = id,
    challengeId = challengeId,
    userId = userId,
    content = content,
    score = score,
    aiFeedback = aiFeedback?.toDomain(),
    likeCount = likeCount,
    commentCount = commentCount,
    viewCount = viewCount,
    createdAt = createdAt,
    user = user.toDomain(),
    isLiked = isLiked
)

fun AnswerWithDetailsDto.toDomain() = AnswerWithDetails(
    id = id,
    challengeId = challengeId,
    userId = userId,
    content = content,
    score = score,
    aiFeedback = aiFeedback?.toDomain(),
    likeCount = likeCount,
    commentCount = commentCount,
    viewCount = viewCount,
    createdAt = createdAt,
    user = user.toDomain(),
    challenge = challenge.toDomain(),
    isLiked = isLiked
)

fun UserStatsDto.toDomain() = UserStats(
    totalChallenges = totalChallenges,
    completedToday = completedToday,
    currentStreak = currentStreak,
    bestStreak = bestStreak,
    averageScore = averageScore
)

fun CommentWithUserDto.toDomain() = CommentWithUser(
    id = id,
    answerId = answerId,
    userId = userId,
    content = content,
    createdAt = createdAt,
    user = user.toDomain()
)
