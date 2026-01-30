//
//  Challenge.swift
//  Gengoka
//

import Foundation

struct Challenge: Identifiable, Codable {
    let id: UUID
    let categoryId: UUID
    let prompt: String
    let description: String?
    let minCharacters: Int
    let maxCharacters: Int
    let difficulty: Difficulty
    let createdAt: Date

    enum Difficulty: String, Codable {
        case easy
        case medium
        case hard
    }

    enum CodingKeys: String, CodingKey {
        case id
        case categoryId = "category_id"
        case prompt
        case description
        case minCharacters = "min_characters"
        case maxCharacters = "max_characters"
        case difficulty
        case createdAt = "created_at"
    }
}

struct DailyChallenge: Codable {
    let challenge: Challenge
    let categoryName: String
    let isCompleted: Bool

    enum CodingKeys: String, CodingKey {
        case challenge
        case categoryName = "category_name"
        case isCompleted = "is_completed"
    }
}

extension Challenge {
    static let mock = Challenge(
        id: UUID(),
        categoryId: UUID(),
        prompt: "今日の朝ごはんを50文字以内で説明してください。",
        description: "シンプルな文章で、何を食べたか伝えましょう。",
        minCharacters: 20,
        maxCharacters: 50,
        difficulty: .easy,
        createdAt: Date()
    )
}
