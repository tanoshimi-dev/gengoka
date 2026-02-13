//
//  APIEndpoints.swift
//  Gengoka
//

import Foundation

enum APIEndpoint {
    // Auth endpoints
    case login
    case register
    case refreshToken
    
    // App endpoints
    case categories
    case dailyChallenges
    case challenge(id: UUID)
    case submitAnswer(challengeId: UUID)
    case feed(page: Int, filter: FeedFilter)
    case user(id: UUID)
    case currentUser
    case userStats
    case followUser(id: UUID)
    case unfollowUser(id: UUID)
    case likeAnswer(id: UUID)
    case unlikeAnswer(id: UUID)
    case comments(answerId: UUID)
    case addComment(answerId: UUID)

    var path: String {
        switch self {
        case .login:
            return "/auth/login"
        case .register:
            return "/auth/register"
        case .refreshToken:
            return "/auth/refresh"
        case .categories:
            return "/categories"
        case .dailyChallenges:
            return "/challenges/daily"
        case .challenge(let id):
            return "/challenges/\(id.uuidString)"
        case .submitAnswer(let challengeId):
            return "/challenges/\(challengeId.uuidString)/answers"
        case .feed(let page, let filter):
            return "/feed?page=\(page)&filter=\(filter.rawValue)"
        case .user(let id):
            return "/users/\(id.uuidString)"
        case .currentUser:
            return "/users/me"
        case .userStats:
            return "/users/me/stats"
        case .followUser(let id):
            return "/users/\(id.uuidString)/follow"
        case .unfollowUser(let id):
            return "/users/\(id.uuidString)/follow"
        case .likeAnswer(let id):
            return "/answers/\(id.uuidString)/like"
        case .unlikeAnswer(let id):
            return "/answers/\(id.uuidString)/like"
        case .comments(let answerId):
            return "/answers/\(answerId.uuidString)/comments"
        case .addComment(let answerId):
            return "/answers/\(answerId.uuidString)/comments"
        }
    }

    var method: HTTPMethod {
        switch self {
        case .categories, .dailyChallenges, .challenge, .feed, .user, .currentUser, .userStats, .comments:
            return .get
        case .login, .register, .refreshToken, .submitAnswer, .followUser, .likeAnswer, .addComment:
            return .post
        case .unfollowUser, .unlikeAnswer:
            return .delete
        }
    }
}

enum HTTPMethod: String {
    case get = "GET"
    case post = "POST"
    case put = "PUT"
    case delete = "DELETE"
}

enum FeedFilter: String {
    case all
    case following
    case popular
}
