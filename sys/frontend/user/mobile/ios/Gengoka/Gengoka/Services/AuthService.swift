//
//  AuthService.swift
//  Gengoka
//

import Foundation
import SwiftUI

@Observable
final class AuthService {
    static let shared = AuthService()

    // MARK: - Storage Keys
    private let tokenKey = "gengoka_auth_token"
    private let refreshTokenKey = "gengoka_refresh_token"
    private let userKey = "gengoka_user"
    
    // MARK: - Published State
    private(set) var currentUser: AuthUser?
    private(set) var isAuthenticated = false
    private(set) var authToken: String?
    
    // MARK: - Computed Properties
    var userId: UUID {
        currentUser?.id ?? UUID() // Fallback for legacy code
    }

    private init() {
        loadStoredAuth()
    }
    
    // MARK: - Authentication Methods
    
    func login(email: String, password: String) async throws {
        let request = LoginRequest(email: email, password: password)
        
        if let error = request.validate() {
            throw AuthError.validationError(error)
        }
        
        let response: AuthResponse = try await APIClient.shared.request(
            .login,
            body: request
        )
        
        saveAuth(response)
    }
    
    func register(email: String, password: String, username: String, name: String?) async throws {
        let request = RegisterRequest(
            email: email,
            password: password,
            username: username,
            name: name
        )
        
        if let error = request.validate() {
            throw AuthError.validationError(error)
        }
        
        let response: AuthResponse = try await APIClient.shared.request(
            .register,
            body: request
        )
        
        saveAuth(response)
    }
    
    func logout() {
        clearAuth()
    }
    
    func refreshToken() async throws {
        guard let refreshToken = UserDefaults.standard.string(forKey: refreshTokenKey) else {
            throw AuthError.notAuthenticated
        }
        
        let request = RefreshTokenRequest(refreshToken: refreshToken)
        
        let response: AuthResponse = try await APIClient.shared.request(
            .refreshToken,
            body: request
        )
        
        saveAuth(response)
    }
    
    // MARK: - Storage Methods
    
    private func saveAuth(_ response: AuthResponse) {
        UserDefaults.standard.set(response.token, forKey: tokenKey)
        UserDefaults.standard.set(response.refreshToken, forKey: refreshTokenKey)
        
        if let encoded = try? JSONEncoder().encode(response.user) {
            UserDefaults.standard.set(encoded, forKey: userKey)
        }
        
        authToken = response.token
        currentUser = response.user
        isAuthenticated = true
    }
    
    private func loadStoredAuth() {
        guard let token = UserDefaults.standard.string(forKey: tokenKey),
              let userData = UserDefaults.standard.data(forKey: userKey),
              let user = try? JSONDecoder().decode(AuthUser.self, from: userData) else {
            isAuthenticated = false
            return
        }
        
        authToken = token
        currentUser = user
        isAuthenticated = true
    }
    
    private func clearAuth() {
        UserDefaults.standard.removeObject(forKey: tokenKey)
        UserDefaults.standard.removeObject(forKey: refreshTokenKey)
        UserDefaults.standard.removeObject(forKey: userKey)
        
        authToken = nil
        currentUser = nil
        isAuthenticated = false
    }
    
    // MARK: - Legacy Support (for backward compatibility)
    
    func resetUserId() {
        // Keep for backward compatibility, but doesn't do anything in JWT mode
        clearAuth()
    }
}
// MARK: - Auth Errors

enum AuthError: LocalizedError {
    case notAuthenticated
    case validationError(String)
    case tokenExpired
    
    var errorDescription: String? {
        switch self {
        case .notAuthenticated:
            return "認証されていません"
        case .validationError(let message):
            return message
        case .tokenExpired:
            return "セッションが期限切れです。再度ログインしてください"
        }
    }
}

