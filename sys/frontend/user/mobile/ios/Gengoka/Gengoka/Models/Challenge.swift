//
//  Challenge.swift
//  Gengoka
//

import Foundation

struct Challenge: Identifiable, Codable {
    let id: UUID
    let categoryId: UUID
    let prompt: String  // Maps to "title" from backend
    let description: String?
    let minCharacters: Int  // Derived from char_limit
    let maxCharacters: Int  // Same as char_limit
    let difficulty: Difficulty
    let createdAt: Date
    
    // Additional backend fields (optional)
    let releaseDate: String?
    let answerCount: Int?
    let status: String?
    let updatedAt: Date?

    enum Difficulty: String, Codable {
        case easy
        case medium
        case hard
    }

    enum CodingKeys: String, CodingKey {
        case id
        case categoryId = "category_id"
        case title  // Backend uses "title"
        case description
        case charLimit = "char_limit"  // Backend uses "char_limit"
        case releaseDate = "release_date"
        case answerCount = "answer_count"
        case status
        case createdAt = "created_at"
        case updatedAt = "updated_at"
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        
        #if DEBUG
        print("🔍 Decoding Challenge with available keys: \(container.allKeys)")
        #endif
        
        id = try container.decode(UUID.self, forKey: .id)
        categoryId = try container.decode(UUID.self, forKey: .categoryId)
        
        // Map "title" to "prompt"
        prompt = try container.decode(String.self, forKey: .title)
        description = try container.decodeIfPresent(String.self, forKey: .description)
        
        // Map "char_limit" to both min and max
        let charLimit = try container.decode(Int.self, forKey: .charLimit)
        minCharacters = 0  // No minimum specified, so default to 0
        maxCharacters = charLimit
        
        // Default difficulty to easy since backend doesn't provide it
        difficulty = .easy
        
        createdAt = try container.decode(Date.self, forKey: .createdAt)
        
        // Optional backend fields
        releaseDate = try container.decodeIfPresent(String.self, forKey: .releaseDate)
        answerCount = try container.decodeIfPresent(Int.self, forKey: .answerCount)
        status = try container.decodeIfPresent(String.self, forKey: .status)
        updatedAt = try container.decodeIfPresent(Date.self, forKey: .updatedAt)
    }
    
    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        
        try container.encode(id, forKey: .id)
        try container.encode(categoryId, forKey: .categoryId)
        try container.encode(prompt, forKey: .title)
        try container.encodeIfPresent(description, forKey: .description)
        try container.encode(maxCharacters, forKey: .charLimit)
        try container.encode(createdAt, forKey: .createdAt)
        try container.encodeIfPresent(releaseDate, forKey: .releaseDate)
        try container.encodeIfPresent(answerCount, forKey: .answerCount)
        try container.encodeIfPresent(status, forKey: .status)
        try container.encodeIfPresent(updatedAt, forKey: .updatedAt)
    }
    
    // Convenience initializer for creating mock data
    init(
        id: UUID = UUID(),
        categoryId: UUID,
        prompt: String,
        description: String? = nil,
        minCharacters: Int = 0,
        maxCharacters: Int,
        difficulty: Difficulty = .easy,
        createdAt: Date = Date(),
        releaseDate: String? = nil,
        answerCount: Int? = nil,
        status: String? = nil,
        updatedAt: Date? = nil
    ) {
        self.id = id
        self.categoryId = categoryId
        self.prompt = prompt
        self.description = description
        self.minCharacters = minCharacters
        self.maxCharacters = maxCharacters
        self.difficulty = difficulty
        self.createdAt = createdAt
        self.releaseDate = releaseDate
        self.answerCount = answerCount
        self.status = status
        self.updatedAt = updatedAt
    }
}

struct DailyChallenge: Codable {
    let challenge: Challenge
    let categoryName: String
    let isCompleted: Bool
    let userAnswer: Answer?  // ← User's existing answer if already completed

    enum CodingKeys: String, CodingKey {
        case challenge
        case categoryName = "category_name"
        case isCompleted = "is_completed"
        case userAnswer = "user_answer"
    }
    
    // Initialize from a Challenge directly (for backend compatibility)
    init(challenge: Challenge, categoryName: String = "", isCompleted: Bool = false, userAnswer: Answer? = nil) {
        self.challenge = challenge
        self.categoryName = categoryName
        self.isCompleted = isCompleted
        self.userAnswer = userAnswer
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        self.challenge = try container.decode(Challenge.self, forKey: .challenge)
        self.categoryName = try container.decode(String.self, forKey: .categoryName)
        self.isCompleted = try container.decode(Bool.self, forKey: .isCompleted)
        self.userAnswer = try container.decodeIfPresent(Answer.self, forKey: .userAnswer)
    }
}

extension Challenge {
    static let mock = Challenge(
        categoryId: UUID(),
        prompt: "今日の朝ごはんを50文字以内で説明してください。",
        description: "シンプルな文章で、何を食べたか伝えましょう。",
        maxCharacters: 50,
        difficulty: .easy
    )
}
