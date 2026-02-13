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

// UserSummary matches the backend's simplified user object in feed
struct UserSummary: Codable {
    let id: UUID
    let name: String
    let avatar: String?
}

struct FeedItem: Identifiable, Codable {
    let id: UUID
    let answer: Answer
    let challenge: Challenge
    let user: User
    let isLiked: Bool

    enum CodingKeys: String, CodingKey {
        // Answer fields (flattened at top level)
        case id
        case challengeId = "challenge_id"
        case userId = "user_id"
        case content
        case score
        case feedback = "ai_feedback"
        case status
        case likeCount = "like_count"
        case commentCount = "comment_count"
        case viewCount = "view_count"
        case createdAt = "created_at"
        case updatedAt = "updated_at"
        
        // Nested objects
        case challenge
        case user
        case isLiked = "is_liked"
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        
        // Decode flattened answer fields
        let answerId = try container.decode(UUID.self, forKey: .id)
        let challengeId = try container.decode(UUID.self, forKey: .challengeId)
        let userId = try container.decode(UUID.self, forKey: .userId)
        let content = try container.decode(String.self, forKey: .content)
        let score = try container.decodeIfPresent(Int.self, forKey: .score)
        let likeCount = try container.decodeIfPresent(Int.self, forKey: .likeCount) ?? 0
        let commentCount = try container.decodeIfPresent(Int.self, forKey: .commentCount) ?? 0
        let createdAt = try container.decode(Date.self, forKey: .createdAt)
        
        // Decode nested challenge and user
        let challenge = try container.decode(Challenge.self, forKey: .challenge)
        let userSummary = try container.decode(UserSummary.self, forKey: .user)
        let isLiked = try container.decode(Bool.self, forKey: .isLiked)
        
        // Build Answer object from flattened fields
        self.id = answerId
        self.answer = Answer(
            id: answerId,
            challengeId: challengeId,
            userId: userId,
            content: content,
            score: score,
            feedback: nil, // ai_feedback is complex, ignore for now
            isPublic: true, // Not provided in feed, default to true
            likeCount: likeCount,
            commentCount: commentCount,
            createdAt: createdAt
        )
        
        self.challenge = challenge
        self.isLiked = isLiked
        
        // Convert UserSummary to full User object
        self.user = User(
            id: userSummary.id,
            username: userSummary.name,
            displayName: userSummary.name,
            avatarUrl: userSummary.avatar,
            bio: nil,
            level: 1,
            totalScore: 0,
            streakDays: 0,
            followerCount: 0,
            followingCount: 0,
            answerCount: 0,
            createdAt: Date()
        )
    }
    
    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        
        // Encode flattened answer fields
        try container.encode(answer.id, forKey: .id)
        try container.encode(answer.challengeId, forKey: .challengeId)
        try container.encode(answer.userId, forKey: .userId)
        try container.encode(answer.content, forKey: .content)
        try container.encodeIfPresent(answer.score, forKey: .score)
        try container.encode(answer.likeCount, forKey: .likeCount)
        try container.encode(answer.commentCount, forKey: .commentCount)
        try container.encode(answer.createdAt, forKey: .createdAt)
        
        // Encode nested objects
        try container.encode(challenge, forKey: .challenge)
        
        let userSummary = UserSummary(
            id: user.id,
            name: user.displayName ?? user.username,
            avatar: user.avatarUrl
        )
        try container.encode(userSummary, forKey: .user)
        try container.encode(isLiked, forKey: .isLiked)
    }
    
    init(id: UUID, answer: Answer, challenge: Challenge, user: User, isLiked: Bool) {
        self.id = id
        self.answer = answer
        self.challenge = challenge
        self.user = user
        self.isLiked = isLiked
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
