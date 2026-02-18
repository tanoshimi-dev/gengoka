//
//  ProfileViewModelTests.swift
//  GengokaTess
//

import XCTest
@testable import Gengoka

final class ProfileViewModelTests: XCTestCase {

    private var mockAPIClient: MockAPIClient!
    private var mockAuthService: MockAuthService!
    private var viewModel: ProfileViewModel!

    override func setUp() async throws {
        mockAPIClient = MockAPIClient()
        mockAuthService = MockAuthService()
        viewModel = ProfileViewModel(apiClient: mockAPIClient, authService: mockAuthService)
    }

    // MARK: - Helpers

    private func makeUser(id: UUID = UUID()) -> User {
        User(
            id: id,
            username: "testuser",
            displayName: "テストユーザー",
            avatarUrl: nil,
            bio: "テストプロフィール",
            level: 5,
            totalScore: 1000,
            streakDays: 3,
            followerCount: 50,
            followingCount: 30,
            answerCount: 100,
            createdAt: Date()
        )
    }

    // MARK: - Tests

    func testLoadProfileSuccess() async {
        // Arrange
        let userId = UUID()
        let user = makeUser(id: userId)
        let profile = UserProfile(
            user: user,
            isFollowing: true,
            recentAnswers: [Answer.mock]
        )
        await mockAPIClient.setResponse(profile, for: "/users/\(userId.uuidString)")

        // Act
        await viewModel.loadProfile(userId: userId)

        // Assert
        XCTAssertNotNil(viewModel.user)
        XCTAssertEqual(viewModel.user?.id, userId)
        XCTAssertTrue(viewModel.isFollowing)
        XCTAssertFalse(viewModel.isLoading)
    }

    func testIsCurrentUserTrue() async {
        // Arrange
        let userId = UUID()
        mockAuthService.userId = userId
        let user = makeUser(id: userId)
        let profile = UserProfile(user: user, isFollowing: false, recentAnswers: [])
        await mockAPIClient.setResponse(profile, for: "/users/\(userId.uuidString)")

        // Act
        await viewModel.loadProfile(userId: userId)

        // Assert
        XCTAssertTrue(viewModel.isCurrentUser)
    }

    func testIsCurrentUserFalse() async {
        // Arrange
        mockAuthService.userId = UUID()  // Different ID
        let otherUserId = UUID()
        let user = makeUser(id: otherUserId)
        let profile = UserProfile(user: user, isFollowing: false, recentAnswers: [])
        await mockAPIClient.setResponse(profile, for: "/users/\(otherUserId.uuidString)")

        // Act
        await viewModel.loadProfile(userId: otherUserId)

        // Assert
        XCTAssertFalse(viewModel.isCurrentUser)
    }

    func testToggleFollowOptimistic() async {
        // Arrange
        let otherUserId = UUID()
        mockAuthService.userId = UUID()  // Different user
        let user = makeUser(id: otherUserId)
        let profile = UserProfile(user: user, isFollowing: false, recentAnswers: [])
        await mockAPIClient.setResponse(profile, for: "/users/\(otherUserId.uuidString)")
        await viewModel.loadProfile(userId: otherUserId)

        let originalFollowerCount = viewModel.user!.followerCount

        // Act
        await viewModel.toggleFollow()

        // Assert
        XCTAssertTrue(viewModel.isFollowing)
        XCTAssertEqual(viewModel.user!.followerCount, originalFollowerCount + 1)
    }

    func testToggleFollowRevertsOnError() async {
        // Arrange
        let otherUserId = UUID()
        mockAuthService.userId = UUID()
        let user = makeUser(id: otherUserId)
        let profile = UserProfile(user: user, isFollowing: false, recentAnswers: [])
        await mockAPIClient.setResponse(profile, for: "/users/\(otherUserId.uuidString)")
        await viewModel.loadProfile(userId: otherUserId)

        let originalFollowerCount = viewModel.user!.followerCount

        // Set error for follow endpoint
        await mockAPIClient.setError(
            NetworkError.httpError(500),
            for: "/users/\(otherUserId.uuidString)/follow"
        )

        // Act
        await viewModel.toggleFollow()

        // Assert — should revert
        XCTAssertFalse(viewModel.isFollowing)
        XCTAssertEqual(viewModel.user!.followerCount, originalFollowerCount)
    }

    func testLoadCurrentUser() async {
        // Arrange
        let userId = UUID()
        mockAuthService.userId = userId
        mockAuthService.currentUser = AuthUser(
            id: userId,
            email: "test@example.com",
            username: "testuser",
            name: "テストユーザー",
            avatar: nil,
            bio: "テスト",
            createdAt: Date()
        )

        let stats = UserStats(
            totalChallenges: 50,
            completedToday: 2,
            currentStreak: 5,
            bestStreak: 10,
            averageScore: 80.0
        )
        await mockAPIClient.setResponse(stats, for: "/users/me/stats")

        // Act
        await viewModel.loadCurrentUser()

        // Assert
        XCTAssertNotNil(viewModel.user)
        XCTAssertEqual(viewModel.user?.id, userId)
        XCTAssertNotNil(viewModel.stats)
        XCTAssertFalse(viewModel.isLoading)
    }
}
