package app.gengoka.testutil

import app.gengoka.domain.model.AiFeedback
import app.gengoka.domain.model.Answer
import app.gengoka.domain.model.AnswerWithDetails
import app.gengoka.domain.model.AnswerWithUser
import app.gengoka.domain.model.Category
import app.gengoka.domain.model.Challenge
import app.gengoka.domain.model.ChallengeWithCategory
import app.gengoka.domain.model.UserProfile
import app.gengoka.domain.model.UserStats
import app.gengoka.domain.model.UserSummary

object TestDataFactory {

    fun createCategory(
        id: String = "cat-1",
        name: String = "状況描写",
        description: String? = "その場にいない人に伝える",
        challengeCount: Long = 10,
        charLimit: Int = 200
    ) = Category(
        id = id,
        name = name,
        description = description,
        icon = null,
        color = null,
        challengeCount = challengeCount,
        charLimit = charLimit,
        sortOrder = 0
    )

    fun createCategories(count: Int = 3): List<Category> =
        (1..count).map { createCategory(id = "cat-$it", name = "Category $it") }

    fun createChallenge(
        id: String = "ch-1",
        categoryId: String = "cat-1",
        title: String = "テストチャレンジ",
        charLimit: Int = 200,
        answerCount: Int = 5
    ) = Challenge(
        id = id,
        categoryId = categoryId,
        title = title,
        description = "テスト説明文",
        charLimit = charLimit,
        releaseDate = "2024-01-01",
        answerCount = answerCount
    )

    fun createChallengeWithCategory(
        id: String = "ch-1",
        categoryId: String = "cat-1",
        title: String = "テストチャレンジ",
        charLimit: Int = 200,
        isCompleted: Boolean = false,
        userAnswer: Answer? = null
    ) = ChallengeWithCategory(
        id = id,
        categoryId = categoryId,
        title = title,
        description = "テスト説明文",
        charLimit = charLimit,
        releaseDate = "2024-01-01",
        answerCount = 5,
        category = createCategory(id = categoryId),
        isCompleted = isCompleted,
        userAnswer = userAnswer
    )

    fun createDailyChallenges(count: Int = 3): List<ChallengeWithCategory> =
        (1..count).map {
            createChallengeWithCategory(
                id = "ch-$it",
                categoryId = "cat-$it",
                isCompleted = it == 1
            )
        }

    fun createUserSummary(
        id: String = "user-1",
        name: String = "テストユーザー"
    ) = UserSummary(
        id = id,
        name = name,
        avatar = null
    )

    fun createUserProfile(
        id: String = "user-1",
        name: String = "テストユーザー",
        isFollowing: Boolean = false,
        followerCount: Long = 100,
        followingCount: Long = 50
    ) = UserProfile(
        id = id,
        name = name,
        avatar = null,
        bio = "テスト自己紹介",
        totalLikes = 42,
        answerCount = 10,
        followerCount = followerCount,
        followingCount = followingCount,
        isFollowing = isFollowing
    )

    fun createUserStats(
        totalChallenges: Int = 30,
        completedToday: Int = 2,
        currentStreak: Int = 5,
        bestStreak: Int = 10,
        averageScore: Double = 78.5
    ) = UserStats(
        totalChallenges = totalChallenges,
        completedToday = completedToday,
        currentStreak = currentStreak,
        bestStreak = bestStreak,
        averageScore = averageScore
    )

    fun createAiFeedback(
        score: Int = 85,
        goodPoints: String = "良い表現です",
        improvement: String = "もう少し具体的に",
        exampleAnswer: String = "模範回答の例"
    ) = AiFeedback(
        score = score,
        goodPoints = goodPoints,
        improvement = improvement,
        exampleAnswer = exampleAnswer
    )

    fun createAnswer(
        id: String = "ans-1",
        challengeId: String = "ch-1",
        userId: String = "user-1",
        content: String = "テスト回答内容",
        score: Int? = 85
    ) = Answer(
        id = id,
        challengeId = challengeId,
        userId = userId,
        content = content,
        score = score,
        aiFeedback = createAiFeedback(),
        likeCount = 3,
        commentCount = 1,
        viewCount = 10,
        createdAt = "2024-01-01T00:00:00Z"
    )

    fun createAnswerWithUser(
        id: String = "ans-1",
        challengeId: String = "ch-1",
        content: String = "テスト回答内容",
        isLiked: Boolean = false
    ) = AnswerWithUser(
        id = id,
        challengeId = challengeId,
        userId = "user-1",
        content = content,
        score = 85,
        aiFeedback = createAiFeedback(),
        likeCount = 3,
        commentCount = 1,
        viewCount = 10,
        createdAt = "2024-01-01T00:00:00Z",
        user = createUserSummary(),
        isLiked = isLiked
    )

    fun createAnswerWithDetails(
        id: String = "ans-1",
        challengeId: String = "ch-1",
        isLiked: Boolean = false,
        likeCount: Int = 3
    ) = AnswerWithDetails(
        id = id,
        challengeId = challengeId,
        userId = "user-1",
        content = "テスト回答内容",
        score = 85,
        aiFeedback = createAiFeedback(),
        likeCount = likeCount,
        commentCount = 1,
        viewCount = 10,
        createdAt = "2024-01-01T00:00:00Z",
        user = createUserSummary(),
        challenge = createChallenge(id = challengeId),
        isLiked = isLiked
    )

    fun createFeedItems(count: Int = 5): List<AnswerWithDetails> =
        (1..count).map { createAnswerWithDetails(id = "ans-$it") }
}
