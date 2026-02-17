//
//  AuthModels.swift
//  Gengoka
//

import Foundation

// MARK: - Request Models

struct LoginRequest: Codable {
    let email: String
    let password: String
}

struct RegisterRequest: Codable {
    let email: String
    let password: String
    let username: String
    let name: String?  // Backend expects "name"
    
    enum CodingKeys: String, CodingKey {
        case email
        case password
        case username
        case name
    }
}

// MARK: - Response Models

struct AuthResponse: Codable {
    let token: String
    let refreshToken: String
    let expiresIn: Int?
    let user: AuthUser
    
    enum CodingKeys: String, CodingKey {
        case token = "access_token"  // Backend uses "access_token"
        case refreshToken = "refresh_token"
        case expiresIn = "expires_in"
        case user
    }
}

struct AuthUser: Codable, Identifiable {
    let id: UUID
    let email: String?
    let username: String?
    let name: String?
    let avatar: String?
    let bio: String?
    let createdAt: Date?
    
    enum CodingKeys: String, CodingKey {
        case id
        case email
        case username
        case name
        case avatar
        case bio
        case createdAt = "created_at"
    }
    
    // Computed properties for compatibility
    var displayName: String? {
        name ?? username ?? email?.components(separatedBy: "@").first
    }
    
    var avatarUrl: String? {
        avatar
    }
}

struct SocialLoginRequest: Codable {
    let provider: String
    let idToken: String?
    let accessToken: String?
    let deviceInfo: String?

    enum CodingKeys: String, CodingKey {
        case provider
        case idToken = "id_token"
        case accessToken = "access_token"
        case deviceInfo = "device_info"
    }
}

struct LinkAccountRequest: Codable {
    let provider: String
    let idToken: String?
    let accessToken: String?

    enum CodingKeys: String, CodingKey {
        case provider
        case idToken = "id_token"
        case accessToken = "access_token"
    }
}

struct LinkedSocialAccount: Codable, Identifiable {
    let provider: String
    let providerEmail: String?
    let providerName: String?
    let linkedAt: Date

    var id: String { provider }

    enum CodingKeys: String, CodingKey {
        case provider
        case providerEmail = "provider_email"
        case providerName = "provider_name"
        case linkedAt = "linked_at"
    }
}

struct RefreshTokenRequest: Codable {
    let refreshToken: String
    
    enum CodingKeys: String, CodingKey {
        case refreshToken = "refresh_token"
    }
}

// MARK: - Validation Helpers

extension RegisterRequest {
    func validate() -> String? {
        if email.isEmpty {
            return "メールアドレスを入力してください"
        }
        
        if !isValidEmail(email) {
            return "有効なメールアドレスを入力してください"
        }
        
        if username.isEmpty {
            return "ユーザー名を入力してください"
        }
        
        if username.count < 3 {
            return "ユーザー名は3文字以上必要です"
        }
        
        if username.count > 20 {
            return "ユーザー名は20文字以内にしてください"
        }
        
        // Username must be alphanumeric and underscores only
        let usernameRegex = "^[a-zA-Z0-9_]+$"
        if !NSPredicate(format: "SELF MATCHES %@", usernameRegex).evaluate(with: username) {
            return "ユーザー名は英数字とアンダースコアのみ使用できます"
        }
        
        if password.isEmpty {
            return "パスワードを入力してください"
        }
        
        if password.count < 8 {
            return "パスワードは8文字以上必要です"
        }
        
        return nil
    }
    
    private func isValidEmail(_ email: String) -> Bool {
        let emailRegex = "[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,64}"
        return NSPredicate(format: "SELF MATCHES %@", emailRegex).evaluate(with: email)
    }
}

extension LoginRequest {
    func validate() -> String? {
        if email.isEmpty {
            return "メールアドレスを入力してください"
        }
        
        if password.isEmpty {
            return "パスワードを入力してください"
        }
        
        return nil
    }
}

