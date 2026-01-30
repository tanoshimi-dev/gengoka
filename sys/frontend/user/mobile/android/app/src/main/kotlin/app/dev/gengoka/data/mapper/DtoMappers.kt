package app.dev.gengoka.data.mapper

import app.dev.gengoka.data.dto.*
import app.dev.gengoka.domain.model.*

fun CategoryDto.toDomain() = Category(
    id = id,
    name = name,
    description = description,
    icon = icon,
    color = color,
    charLimit = charLimit,
    sortOrder = sortOrder
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

fun ChallengeWithCategoryDto.toDomain() = ChallengeWithCategory(
    id = id,
    categoryId = categoryId,
    title = title,
    description = description,
    charLimit = charLimit,
    releaseDate = releaseDate,
    answerCount = answerCount,
    category = category.toDomain()
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

fun CommentWithUserDto.toDomain() = CommentWithUser(
    id = id,
    answerId = answerId,
    userId = userId,
    content = content,
    createdAt = createdAt,
    user = user.toDomain()
)
