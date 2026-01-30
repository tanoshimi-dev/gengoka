//
//  Answer.swift
//  Gengoka
//

import Foundation

struct Answer: Identifiable, Codable, Equatable {
    let id: UUID
    let challengeId: UUID
    let userId: UUID
    let content: String
    let score: Int?
    let feedback: String?
    let isPublic: Bool
    let likeCount: Int
    let commentCount: Int
    let createdAt: Date

    enum CodingKeys: String, CodingKey {
        case id
        case challengeId = "challenge_id"
        case userId = "user_id"
        case content
        case score
        case feedback
        case isPublic = "is_public"
        case likeCount = "like_count"
        case commentCount = "comment_count"
        case createdAt = "created_at"
    }
}

struct AnswerSubmission: Codable {
    let content: String
    let isPublic: Bool

    enum CodingKeys: String, CodingKey {
        case content
        case isPublic = "is_public"
    }
}

struct AnswerResult: Codable, Equatable {
    let answer: Answer
    let scoringDetails: ScoringDetails?

    enum CodingKeys: String, CodingKey {
        case answer
        case scoringDetails = "scoring_details"
    }
}

struct ScoringDetails: Codable, Equatable {
    let grammarScore: Int
    let creativityScore: Int
    let relevanceScore: Int
    let overallScore: Int
    let feedback: String
    let improvements: [String]?

    enum CodingKeys: String, CodingKey {
        case grammarScore = "grammar_score"
        case creativityScore = "creativity_score"
        case relevanceScore = "relevance_score"
        case overallScore = "overall_score"
        case feedback
        case improvements
    }
}

struct FeedItem: Identifiable, Codable {
    let id: UUID
    let answer: Answer
    let challenge: Challenge
    let user: User
    let isLiked: Bool

    enum CodingKeys: String, CodingKey {
        case id
        case answer
        case challenge
        case user
        case isLiked = "is_liked"
    }
}

extension Answer {
    static let mock = Answer(
        id: UUID(),
        challengeId: UUID(),
        userId: UUID(),
        content: "今朝は納豆ご飯と味噌汁を食べました。シンプルだけど栄養満点で美味しかったです。",
        score: 85,
        feedback: "素晴らしい表現です！シンプルながら情景が浮かぶ文章ですね。",
        isPublic: true,
        likeCount: 12,
        commentCount: 3,
        createdAt: Date()
    )
}
