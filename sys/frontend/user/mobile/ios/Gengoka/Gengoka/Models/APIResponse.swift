//
//  APIResponse.swift
//  Gengoka
//

import Foundation

struct APIResponse<T: Codable>: Codable {
    let success: Bool
    let data: T?
    let message: String?
    let error: APIError?
}

struct APIError: Codable, Error {
    let code: String
    let message: String
}

struct PaginatedResponse<T: Codable>: Codable {
    let items: [T]
    let total: Int
    let page: Int
    let perPage: Int
    let hasMore: Bool

    enum CodingKeys: String, CodingKey {
        case items
        case total
        case page
        case perPage = "per_page"
        case hasMore = "has_more"
    }
}

struct EmptyResponse: Codable {}
