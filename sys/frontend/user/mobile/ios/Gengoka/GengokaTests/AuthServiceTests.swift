//
//  AuthServiceTests.swift
//  GengokaTess
//

import XCTest
@testable import Gengoka

final class AuthServiceTests: XCTestCase {

    // MARK: - Logout

    func testLogoutClearsState() {
        // Use a MockAuthService to test logout behavior
        let service = MockAuthService()
        service.isAuthenticated = true
        service.authToken = "test-token"
        service.currentUser = AuthUser(
            id: UUID(),
            email: "test@example.com",
            username: "testuser",
            name: "Test",
            avatar: nil,
            bio: nil,
            createdAt: Date()
        )

        // Act
        service.logout()

        // Assert
        XCTAssertFalse(service.isAuthenticated)
        XCTAssertNil(service.authToken)
        XCTAssertNil(service.currentUser)
        XCTAssertTrue(service.logoutCalled)
    }

    // MARK: - Initial State

    func testInitialStateNotAuthenticated() {
        let service = MockAuthService()

        XCTAssertFalse(service.isAuthenticated)
        XCTAssertNil(service.authToken)
        XCTAssertNil(service.currentUser)
    }

    // MARK: - Login Validation

    func testLoginValidationRejectsEmpty() {
        // Test LoginRequest validation directly
        let emptyEmail = LoginRequest(email: "", password: "password123")
        XCTAssertNotNil(emptyEmail.validate())

        let emptyPassword = LoginRequest(email: "test@example.com", password: "")
        XCTAssertNotNil(emptyPassword.validate())
    }

    // MARK: - Register Validation

    func testRegisterValidationRejectsShortPassword() {
        let request = RegisterRequest(
            email: "test@example.com",
            password: "short",
            username: "testuser",
            name: nil
        )
        let error = request.validate()
        XCTAssertNotNil(error)
        XCTAssertTrue(error!.contains("8"))
    }

    func testRegisterValidationRejectsInvalidUsername() {
        let request = RegisterRequest(
            email: "test@example.com",
            password: "password123",
            username: "user@invalid!",
            name: nil
        )
        XCTAssertNotNil(request.validate())
    }

    // MARK: - UserId Fallback

    func testUserIdFallback() {
        let service = MockAuthService()
        service.currentUser = nil

        // userId should still return a valid UUID (not crash)
        let id = service.userId
        XCTAssertNotEqual(id, UUID(uuidString: "00000000-0000-0000-0000-000000000000"))
    }
}
