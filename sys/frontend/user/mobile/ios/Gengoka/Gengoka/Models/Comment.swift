//
//  Comment.swift
//  Gengoka
//

import Foundation

struct Comment: Identifiable, Codable {
    let id: UUID
    let answerId: UUID
    let userId: UUID
    let content: String
    let user: User?
    let createdAt: Date

    enum CodingKeys: String, CodingKey {
        case id
        case answerId = "answer_id"
        case userId = "user_id"
        case content
        case user
        case createdAt = "created_at"
    }
}

struct CommentSubmission: Codable {
    let content: String
}

extension Comment {
    static let mock = Comment(
        id: UUID(),
        answerId: UUID(),
        userId: UUID(),
        content: "素敵な表現ですね！参考になります。",
        user: .mock,
        createdAt: Date()
    )
}
