//
//  MockAuthService.swift
//  GengokaTess
//

import Foundation
@testable import Gengoka

final class MockAuthService: AuthServiceProtocol {
    var currentUser: AuthUser?
    var isAuthenticated = false
    var authToken: String?
    var userId = UUID()

    // Tracking
    var loginCalled = false
    var registerCalled = false
    var socialLoginCalled = false
    var logoutCalled = false
    var refreshTokenCalled = false

    // Error simulation
    var loginError: Error?
    var registerError: Error?
    var socialLoginError: Error?
    var refreshTokenError: Error?

    func login(email: String, password: String) async throws {
        loginCalled = true
        if let error = loginError {
            throw error
        }
        isAuthenticated = true
        authToken = "mock-token"
    }

    func register(email: String, password: String, username: String, name: String?) async throws {
        registerCalled = true
        if let error = registerError {
            throw error
        }
        isAuthenticated = true
        authToken = "mock-token"
    }

    func socialLogin(result: SocialAuthResult) async throws {
        socialLoginCalled = true
        if let error = socialLoginError {
            throw error
        }
        isAuthenticated = true
        authToken = "mock-token"
    }

    func logout() {
        logoutCalled = true
        isAuthenticated = false
        authToken = nil
        currentUser = nil
    }

    func refreshToken() async throws {
        refreshTokenCalled = true
        if let error = refreshTokenError {
            throw error
        }
    }

    func reset() {
        currentUser = nil
        isAuthenticated = false
        authToken = nil
        userId = UUID()
        loginCalled = false
        registerCalled = false
        socialLoginCalled = false
        logoutCalled = false
        refreshTokenCalled = false
        loginError = nil
        registerError = nil
        socialLoginError = nil
        refreshTokenError = nil
    }
}
