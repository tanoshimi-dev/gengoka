//
//  Category.swift
//  Gengoka
//

import Foundation

struct Category: Identifiable, Codable, Hashable {
    let id: UUID
    let name: String
    let description: String
    let iconName: String
    let colorHex: String
    let challengeCount: Int

    enum CodingKeys: String, CodingKey {
        case id
        case name
        case description
        case iconName = "icon_name"
        case colorHex = "color_hex"
        case challengeCount = "challenge_count"
    }
}

extension Category {
    static let mockCategories: [Category] = [
        Category(id: UUID(), name: "日常会話", description: "毎日使える表現を学ぼう", iconName: "message.fill", colorHex: "#667eea", challengeCount: 25),
        Category(id: UUID(), name: "ビジネス", description: "仕事で使える丁寧な表現", iconName: "briefcase.fill", colorHex: "#764ba2", challengeCount: 30),
        Category(id: UUID(), name: "旅行", description: "旅先で役立つフレーズ", iconName: "airplane", colorHex: "#f093fb", challengeCount: 20),
        Category(id: UUID(), name: "文化", description: "日本の文化を表現しよう", iconName: "theatermasks.fill", colorHex: "#4facfe", challengeCount: 15),
        Category(id: UUID(), name: "クリエイティブ", description: "自由な発想で表現しよう", iconName: "paintbrush.fill", colorHex: "#43e97b", challengeCount: 18)
    ]
}
