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

// Feed-specific paginated response that matches backend structure
struct FeedResponse<T: Codable>: Codable {
    let data: [T]
    let pagination: Pagination
    
    struct Pagination: Codable {
        let page: Int
        let pageSize: Int
        let total: Int
        let totalPages: Int
        let hasMore: Bool
        
        enum CodingKeys: String, CodingKey {
            case page
            case pageSize = "page_size"
            case total
            case totalPages = "total_pages"
            case hasMore = "has_more"
        }
    }
}

struct EmptyResponse: Codable {}
