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
    case socialLogin
    
    // App endpoints
    case categories
    case dailyChallenges
    case challenge(id: UUID)
    case challengeAnswer(challengeId: UUID)  // Get user's answer for a challenge
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

    // Account linking
    case linkedAccounts
    case linkAccount
    case unlinkAccount(provider: String)

    var path: String {
        switch self {
        case .login:
            return "/auth/login"
        case .register:
            return "/auth/register"
        case .refreshToken:
            return "/auth/refresh"
        case .socialLogin:
            return "/auth/social"
        case .categories:
            return "/categories"
        case .dailyChallenges:
            return "/challenges/daily"
        case .challenge(let id):
            return "/challenges/\(id.uuidString)"
        case .challengeAnswer(let challengeId):
            return "/challenges/\(challengeId.uuidString)/my-answer"
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
        case .linkedAccounts:
            return "/users/me/social-accounts"
        case .linkAccount:
            return "/users/me/social-accounts"
        case .unlinkAccount(let provider):
            return "/users/me/social-accounts/\(provider)"
        }
    }

    var method: HTTPMethod {
        switch self {
        case .categories, .dailyChallenges, .challenge, .challengeAnswer, .feed, .user, .currentUser, .userStats, .comments, .linkedAccounts:
            return .get
        case .login, .register, .refreshToken, .socialLogin, .submitAnswer, .followUser, .likeAnswer, .addComment, .linkAccount:
            return .post
        case .unfollowUser, .unlikeAnswer, .unlinkAccount:
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
