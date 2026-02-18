//
//  AuthModelsTests.swift
//  GengokaTess
//

import XCTest
@testable import Gengoka

final class AuthModelsTests: XCTestCase {

    // MARK: - LoginRequest Validation

    func testLoginValidateSuccess() {
        let request = LoginRequest(email: "test@example.com", password: "password123")
        XCTAssertNil(request.validate())
    }

    func testLoginValidateEmptyEmail() {
        let request = LoginRequest(email: "", password: "password123")
        XCTAssertNotNil(request.validate())
    }

    func testLoginValidateEmptyPassword() {
        let request = LoginRequest(email: "test@example.com", password: "")
        XCTAssertNotNil(request.validate())
    }

    // MARK: - RegisterRequest Validation

    func testRegisterValidateSuccess() {
        let request = RegisterRequest(
            email: "test@example.com",
            password: "password123",
            username: "testuser",
            name: "Test User"
        )
        XCTAssertNil(request.validate())
    }

    func testRegisterValidateShortUsername() {
        let request = RegisterRequest(
            email: "test@example.com",
            password: "password123",
            username: "ab",
            name: nil
        )
        let error = request.validate()
        XCTAssertNotNil(error)
        XCTAssertTrue(error!.contains("3"))
    }

    func testRegisterValidateLongUsername() {
        let request = RegisterRequest(
            email: "test@example.com",
            password: "password123",
            username: "a_very_long_username_123",
            name: nil
        )
        let error = request.validate()
        XCTAssertNotNil(error)
        XCTAssertTrue(error!.contains("20"))
    }

    func testRegisterValidateInvalidUsernameChars() {
        let request = RegisterRequest(
            email: "test@example.com",
            password: "password123",
            username: "user@name!",
            name: nil
        )
        XCTAssertNotNil(request.validate())
    }

    func testRegisterValidateShortPassword() {
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
}
