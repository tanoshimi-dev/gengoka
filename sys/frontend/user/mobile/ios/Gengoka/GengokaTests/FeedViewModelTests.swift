//
//  FeedViewModelTests.swift
//  GengokaTess
//

import XCTest
@testable import Gengoka

final class FeedViewModelTests: XCTestCase {

    private var mockAPIClient: MockAPIClient!
    private var viewModel: FeedViewModel!

    override func setUp() async throws {
        mockAPIClient = MockAPIClient()
        viewModel = FeedViewModel(apiClient: mockAPIClient)
    }

    // MARK: - Helpers

    private func makeFeedItem(isLiked: Bool = false, likeCount: Int = 5) -> FeedItem {
        let challengeId = UUID()
        let userId = UUID()
        let answer = Answer(
            id: UUID(),
            challengeId: challengeId,
            userId: userId,
            content: "テスト回答",
            score: 80,
            isPublic: true,
            likeCount: likeCount,
            createdAt: Date()
        )
        let challenge = Challenge(
            categoryId: UUID(),
            prompt: "テストお題",
            maxCharacters: 100
        )
        let user = User(
            id: userId,
            username: "testuser",
            displayName: "テストユーザー",
            avatarUrl: nil,
            bio: nil,
            level: 1,
            totalScore: 100,
            streakDays: 1,
            followerCount: 10,
            followingCount: 5,
            answerCount: 20,
            createdAt: Date()
        )
        return FeedItem(
            id: answer.id,
            answer: answer,
            challenge: challenge,
            user: user,
            isLiked: isLiked
        )
    }

    private func makeFeedResponse(items: [FeedItem], hasMore: Bool = false, page: Int = 1) -> FeedResponse<FeedItem> {
        // We construct this manually since FeedResponse needs Pagination
        // Since FeedResponse is Codable, we encode/decode to create it
        // But simpler: just use the struct directly
        let pagination = FeedResponse<FeedItem>.Pagination(
            page: page,
            pageSize: 20,
            total: items.count,
            totalPages: 1,
            hasMore: hasMore
        )
        return FeedResponse(data: items, pagination: pagination)
    }

    // MARK: - Tests

    func testLoadFeedSuccess() async {
        // Arrange
        let items = [makeFeedItem(), makeFeedItem()]
        let response = makeFeedResponse(items: items, hasMore: true)
        await mockAPIClient.setResponse(response, for: "/feed?page=1&filter=all")

        // Act
        await viewModel.loadFeed()

        // Assert
        XCTAssertEqual(viewModel.feedItems.count, 2)
        XCTAssertTrue(viewModel.hasMore)
        XCTAssertFalse(viewModel.isLoading)
    }

    func testLoadFeedError() async {
        // Arrange — no response set, will throw

        // Act
        await viewModel.loadFeed()

        // Assert
        XCTAssertTrue(viewModel.feedItems.isEmpty)
        XCTAssertFalse(viewModel.hasMore)
        XCTAssertNotNil(viewModel.error)
    }

    func testLoadMoreAppends() async {
        // Arrange — first page
        let firstPageItems = [makeFeedItem()]
        let firstResponse = makeFeedResponse(items: firstPageItems, hasMore: true)
        await mockAPIClient.setResponse(firstResponse, for: "/feed?page=1&filter=all")
        await viewModel.loadFeed()
        XCTAssertEqual(viewModel.feedItems.count, 1)

        // Arrange — second page
        let secondPageItems = [makeFeedItem(), makeFeedItem()]
        let secondResponse = makeFeedResponse(items: secondPageItems, hasMore: false, page: 2)
        await mockAPIClient.setResponse(secondResponse, for: "/feed?page=2&filter=all")

        // Act
        await viewModel.loadMore()

        // Assert
        XCTAssertEqual(viewModel.feedItems.count, 3)
        XCTAssertFalse(viewModel.hasMore)
    }

    func testSetFilterReloads() async {
        // Arrange
        let items = [makeFeedItem()]
        let response = makeFeedResponse(items: items)
        await mockAPIClient.setResponse(response, for: "/feed?page=1&filter=following")

        // Act
        await viewModel.setFilter(.following)

        // Assert
        XCTAssertEqual(viewModel.currentFilter, .following)
        XCTAssertEqual(viewModel.feedItems.count, 1)
    }

    func testToggleLikeOptimistic() async {
        // Arrange — load feed first
        let item = makeFeedItem(isLiked: false, likeCount: 5)
        let response = makeFeedResponse(items: [item])
        await mockAPIClient.setResponse(response, for: "/feed?page=1&filter=all")
        await viewModel.loadFeed()

        // Set up void response for like endpoint
        // (requestVoid doesn't need a response, just no error)

        // Act
        await viewModel.toggleLike(for: viewModel.feedItems[0])

        // Assert — optimistic update
        XCTAssertTrue(viewModel.feedItems[0].isLiked)
        XCTAssertEqual(viewModel.feedItems[0].answer.likeCount, 6)
    }

    func testToggleLikeRevertsOnError() async {
        // Arrange — load feed first
        let item = makeFeedItem(isLiked: false, likeCount: 5)
        let response = makeFeedResponse(items: [item])
        await mockAPIClient.setResponse(response, for: "/feed?page=1&filter=all")
        await viewModel.loadFeed()

        // Set error for like endpoint
        let answerId = viewModel.feedItems[0].answer.id
        await mockAPIClient.setError(
            NetworkError.httpError(500),
            for: "/answers/\(answerId.uuidString)/like"
        )

        // Act
        await viewModel.toggleLike(for: viewModel.feedItems[0])

        // Assert — should revert
        XCTAssertFalse(viewModel.feedItems[0].isLiked)
        XCTAssertEqual(viewModel.feedItems[0].answer.likeCount, 5)
    }
}
