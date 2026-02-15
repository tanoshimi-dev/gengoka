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
    let aiFeedback: String?  // Backend uses "ai_feedback"
    let isPublic: Bool
    let likeCount: Int
    let commentCount: Int
    let viewCount: Int  // Added from backend
    let status: String  // Added from backend
    let createdAt: Date
    let updatedAt: Date  // Added from backend

    enum CodingKeys: String, CodingKey {
        case id
        case challengeId = "challenge_id"
        case userId = "user_id"
        case content
        case score
        case aiFeedback = "ai_feedback"  // Changed from "feedback"
        case isPublic = "is_public"
        case likeCount = "like_count"
        case commentCount = "comment_count"
        case viewCount = "view_count"  // Added
        case status  // Added
        case createdAt = "created_at"
        case updatedAt = "updated_at"  // Added
    }
    
    // Custom initializer for manual creation
    init(
        id: UUID,
        challengeId: UUID,
        userId: UUID,
        content: String,
        score: Int? = nil,
        feedback: String? = nil,  // Keep for backward compatibility
        aiFeedback: String? = nil,  // New parameter
        isPublic: Bool = true,
        likeCount: Int = 0,
        commentCount: Int = 0,
        viewCount: Int = 0,
        status: String = "active",
        createdAt: Date,
        updatedAt: Date? = nil
    ) {
        self.id = id
        self.challengeId = challengeId
        self.userId = userId
        self.content = content
        self.score = score
        // Prefer aiFeedback if provided, otherwise use feedback
        self.aiFeedback = aiFeedback ?? feedback
        self.isPublic = isPublic
        self.likeCount = likeCount
        self.commentCount = commentCount
        self.viewCount = viewCount
        self.status = status
        self.createdAt = createdAt
        self.updatedAt = updatedAt ?? createdAt
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        
        id = try container.decode(UUID.self, forKey: .id)
        challengeId = try container.decode(UUID.self, forKey: .challengeId)
        userId = try container.decode(UUID.self, forKey: .userId)
        content = try container.decode(String.self, forKey: .content)
        score = try container.decodeIfPresent(Int.self, forKey: .score)
        aiFeedback = try container.decodeIfPresent(String.self, forKey: .aiFeedback)
        
        // isPublic might be missing, default to true
        isPublic = try container.decodeIfPresent(Bool.self, forKey: .isPublic) ?? true
        
        likeCount = try container.decodeIfPresent(Int.self, forKey: .likeCount) ?? 0
        commentCount = try container.decodeIfPresent(Int.self, forKey: .commentCount) ?? 0
        viewCount = try container.decodeIfPresent(Int.self, forKey: .viewCount) ?? 0
        status = try container.decodeIfPresent(String.self, forKey: .status) ?? "active"
        createdAt = try container.decode(Date.self, forKey: .createdAt)
        updatedAt = try container.decodeIfPresent(Date.self, forKey: .updatedAt) ?? createdAt
    }
    
    // Convenience computed property for backward compatibility
    var feedback: String? {
        aiFeedback
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
    
    init(answer: Answer, scoringDetails: ScoringDetails? = nil) {
        self.answer = answer
        self.scoringDetails = scoringDetails
    }
    
    init(from decoder: Decoder) throws {
        // Try to decode as a full AnswerResult first
        if let container = try? decoder.container(keyedBy: CodingKeys.self),
           let answer = try? container.decode(Answer.self, forKey: .answer) {
            self.answer = answer
            self.scoringDetails = try? container.decodeIfPresent(ScoringDetails.self, forKey: .scoringDetails)
            
            #if DEBUG
            print("✅ Decoded AnswerResult with nested structure")
            #endif
        } else {
            // If that fails, the backend might return Answer directly
            #if DEBUG
            print("⚠️ Trying to decode Answer directly (no nesting)")
            #endif
            
            self.answer = try Answer(from: decoder)
            self.scoringDetails = nil
            
            #if DEBUG
            print("✅ Decoded Answer directly as AnswerResult")
            #endif
        }
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
    let hasUserAnswered: Bool  // ← NEW: Has current user answered this challenge?

    enum CodingKeys: String, CodingKey {
        // Nested format keys
        case answer
        case challenge
        case user
        case isLiked = "is_liked"
        case hasUserAnswered = "has_user_answered"
        
        // Flattened format keys (for backward compatibility)
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
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        
        #if DEBUG
        // Debug: Print all available keys
        print("🔍 FeedItem decoding - Available keys: \(container.allKeys.map { $0.stringValue })")
        #endif
        
        // Try to decode nested format first (new backend format)
        if let nestedAnswer = try? container.decode(Answer.self, forKey: .answer) {
            #if DEBUG
            print("✅ Found nested 'answer' object")
            #endif
            
            // New format: answer, challenge, user are all nested objects
            self.id = nestedAnswer.id
            self.answer = nestedAnswer
            self.challenge = try container.decode(Challenge.self, forKey: .challenge)
            
            // Decode user (might be UserSummary or full User)
            if let userSummary = try? container.decode(UserSummary.self, forKey: .user) {
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
            } else {
                self.user = try container.decode(User.self, forKey: .user)
            }
            
            self.isLiked = try container.decode(Bool.self, forKey: .isLiked)
            self.hasUserAnswered = try container.decodeIfPresent(Bool.self, forKey: .hasUserAnswered) ?? false
            
            #if DEBUG
            print("✅ Decoded FeedItem using nested format")
            #endif
        } else {
            #if DEBUG
            print("⚠️ No nested 'answer' key found, trying flattened format")
            #endif
            
            // Old format: answer fields are flattened at top level
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
            self.hasUserAnswered = try container.decodeIfPresent(Bool.self, forKey: .hasUserAnswered) ?? false
            
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
            
            #if DEBUG
            print("✅ Decoded FeedItem using flattened format")
            #endif
        }
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
        try container.encode(hasUserAnswered, forKey: .hasUserAnswered)
    }
    
    init(id: UUID, answer: Answer, challenge: Challenge, user: User, isLiked: Bool, hasUserAnswered: Bool = false) {
        self.id = id
        self.answer = answer
        self.challenge = challenge
        self.user = user
        self.isLiked = isLiked
        self.hasUserAnswered = hasUserAnswered
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
