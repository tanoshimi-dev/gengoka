//
//  User.swift
//  Gengoka
//

import Foundation

struct User: Identifiable, Codable, Hashable {
    let id: UUID
    let username: String
    let displayName: String
    let avatarUrl: String?
    let bio: String?
    let level: Int
    let totalScore: Int
    let streakDays: Int
    let followerCount: Int
    let followingCount: Int
    let answerCount: Int
    let createdAt: Date

    enum CodingKeys: String, CodingKey {
        case id
        case username
        case displayName = "display_name"
        case avatarUrl = "avatar_url"
        case bio
        case level
        case totalScore = "total_score"
        case streakDays = "streak_days"
        case followerCount = "follower_count"
        case followingCount = "following_count"
        case answerCount = "answer_count"
        case createdAt = "created_at"
    }
}

struct UserProfile: Codable {
    let user: User
    let isFollowing: Bool
    let recentAnswers: [Answer]

    enum CodingKeys: String, CodingKey {
        case user
        case isFollowing = "is_following"
        case recentAnswers = "recent_answers"
    }
}

struct UserStats: Codable {
    let totalChallenges: Int
    let completedToday: Int
    let currentStreak: Int
    let bestStreak: Int
    let averageScore: Double

    enum CodingKeys: String, CodingKey {
        case totalChallenges = "total_challenges"
        case completedToday = "completed_today"
        case currentStreak = "current_streak"
        case bestStreak = "best_streak"
        case averageScore = "average_score"
    }
}

extension User {
    static let mock = User(
        id: UUID(),
        username: "tanaka_user",
        displayName: "田中太郎",
        avatarUrl: nil,
        bio: "言葉を学ぶのが大好きです！毎日チャレンジ中 📚",
        level: 15,
        totalScore: 12500,
        streakDays: 7,
        followerCount: 128,
        followingCount: 56,
        answerCount: 234,
        createdAt: Date()
    )
}

extension UserStats {
    static let mock = UserStats(
        totalChallenges: 234,
        completedToday: 2,
        currentStreak: 7,
        bestStreak: 21,
        averageScore: 82.5
    )
}
