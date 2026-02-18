//
//  HomeViewModelTests.swift
//  GengokaTess
//

import XCTest
@testable import Gengoka

final class HomeViewModelTests: XCTestCase {

    private var mockAPIClient: MockAPIClient!
    private var viewModel: HomeViewModel!

    override func setUp() async throws {
        mockAPIClient = MockAPIClient()
        viewModel = HomeViewModel(apiClient: mockAPIClient)
    }

    // MARK: - Tests

    func testLoadDataSuccess() async {
        // Arrange
        let categories = [
            Gengoka.Category(
                id: UUID(),
                name: "日常会話",
                description: "毎日使える表現",
                iconName: "message.fill",
                colorHex: "#667eea",
                challengeCount: 10
            )
        ]
        let stats = UserStats(
            totalChallenges: 50,
            completedToday: 2,
            currentStreak: 5,
            bestStreak: 10,
            averageScore: 80.0
        )
        let dailyChallenges: [DailyChallenge] = []

        await mockAPIClient.setResponse(categories, for: "/categories")
        await mockAPIClient.setResponse(stats, for: "/users/me/stats")
        await mockAPIClient.setResponse(dailyChallenges, for: "/challenges/daily")

        // Act
        await viewModel.loadData()

        // Assert
        XCTAssertEqual(viewModel.categories.count, 1)
        XCTAssertNotNil(viewModel.userStats)
        XCTAssertFalse(viewModel.isLoading)
    }

    func testLoadDataAPIError() async {
        // Arrange — no responses set, will throw NetworkError.invalidResponse

        // Act
        await viewModel.loadData()

        // Assert — should fall back gracefully
        XCTAssertFalse(viewModel.isLoading)
        XCTAssertNotNil(viewModel.error)
        // Falls back to mock categories
        XCTAssertFalse(viewModel.categories.isEmpty)
    }

    func testLoadingState() async {
        // Arrange
        let categories: [Gengoka.Category] = []
        let stats = UserStats(
            totalChallenges: 0,
            completedToday: 0,
            currentStreak: 0,
            bestStreak: 0,
            averageScore: 0
        )
        let dailyChallenges: [DailyChallenge] = []

        await mockAPIClient.setResponse(categories, for: "/categories")
        await mockAPIClient.setResponse(stats, for: "/users/me/stats")
        await mockAPIClient.setResponse(dailyChallenges, for: "/challenges/daily")

        // Before load
        XCTAssertFalse(viewModel.isLoading)

        // Act
        await viewModel.loadData()

        // After load
        XCTAssertFalse(viewModel.isLoading)
    }

    func testLoadDataSetsCategories() async {
        // Arrange
        let categoryId = UUID()
        let categories = [
            Gengoka.Category(
                id: categoryId,
                name: "ビジネス",
                description: "仕事で使える表現",
                iconName: "briefcase.fill",
                colorHex: "#764ba2",
                challengeCount: 30
            )
        ]
        let stats = UserStats.mock
        let dailyChallenges: [DailyChallenge] = []

        await mockAPIClient.setResponse(categories, for: "/categories")
        await mockAPIClient.setResponse(stats, for: "/users/me/stats")
        await mockAPIClient.setResponse(dailyChallenges, for: "/challenges/daily")

        // Act
        await viewModel.loadData()

        // Assert
        XCTAssertEqual(viewModel.categories.count, 1)
        XCTAssertEqual(viewModel.categories.first?.name, "ビジネス")
        XCTAssertEqual(viewModel.categories.first?.id, categoryId)
    }

    func testLoadDataSetsUserStats() async {
        // Arrange
        let categories: [Gengoka.Category] = []
        let stats = UserStats(
            totalChallenges: 100,
            completedToday: 3,
            currentStreak: 7,
            bestStreak: 14,
            averageScore: 85.5
        )
        let dailyChallenges: [DailyChallenge] = []

        await mockAPIClient.setResponse(categories, for: "/categories")
        await mockAPIClient.setResponse(stats, for: "/users/me/stats")
        await mockAPIClient.setResponse(dailyChallenges, for: "/challenges/daily")

        // Act
        await viewModel.loadData()

        // Assert
        XCTAssertEqual(viewModel.userStats?.totalChallenges, 100)
        XCTAssertEqual(viewModel.userStats?.completedToday, 3)
        XCTAssertEqual(viewModel.userStats?.currentStreak, 7)
        XCTAssertEqual(viewModel.userStats?.averageScore, 85.5)
    }
}
