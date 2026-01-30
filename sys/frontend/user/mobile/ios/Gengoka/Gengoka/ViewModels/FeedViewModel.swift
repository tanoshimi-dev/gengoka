//
//  FeedViewModel.swift
//  Gengoka
//

import Foundation

@Observable
final class FeedViewModel {
    var feedItems: [FeedItem] = []
    var currentFilter: FeedFilter = .all
    var isLoading = false
    var isLoadingMore = false
    var error: Error?
    var hasMore = true
    private var currentPage = 1

    private let apiClient = APIClient.shared

    func loadFeed() async {
        isLoading = true
        error = nil
        currentPage = 1

        do {
            let response: PaginatedResponse<FeedItem> = try await apiClient.request(.feed(page: 1, filter: currentFilter))
            feedItems = response.items
            hasMore = response.hasMore
        } catch {
            self.error = error
            // Use mock data for development
            feedItems = createMockFeedItems()
        }

        isLoading = false
    }

    func loadMore() async {
        guard !isLoadingMore && hasMore else { return }

        isLoadingMore = true
        currentPage += 1

        do {
            let response: PaginatedResponse<FeedItem> = try await apiClient.request(.feed(page: currentPage, filter: currentFilter))
            feedItems.append(contentsOf: response.items)
            hasMore = response.hasMore
        } catch {
            currentPage -= 1
        }

        isLoadingMore = false
    }

    func setFilter(_ filter: FeedFilter) async {
        guard filter != currentFilter else { return }
        currentFilter = filter
        await loadFeed()
    }

    func toggleLike(for item: FeedItem) async {
        guard let index = feedItems.firstIndex(where: { $0.id == item.id }) else { return }

        let endpoint: APIEndpoint = item.isLiked
            ? .unlikeAnswer(id: item.answer.id)
            : .likeAnswer(id: item.answer.id)

        // Optimistic update
        let newLikeCount = item.isLiked ? item.answer.likeCount - 1 : item.answer.likeCount + 1
        let updatedAnswer = Answer(
            id: item.answer.id,
            challengeId: item.answer.challengeId,
            userId: item.answer.userId,
            content: item.answer.content,
            score: item.answer.score,
            feedback: item.answer.feedback,
            isPublic: item.answer.isPublic,
            likeCount: newLikeCount,
            commentCount: item.answer.commentCount,
            createdAt: item.answer.createdAt
        )
        feedItems[index] = FeedItem(
            id: item.id,
            answer: updatedAnswer,
            challenge: item.challenge,
            user: item.user,
            isLiked: !item.isLiked
        )

        do {
            try await apiClient.requestVoid(endpoint)
        } catch {
            // Revert on error
            feedItems[index] = item
        }
    }

    func refresh() async {
        await loadFeed()
    }

    private func createMockFeedItems() -> [FeedItem] {
        (0..<5).map { i in
            FeedItem(
                id: UUID(),
                answer: Answer(
                    id: UUID(),
                    challengeId: UUID(),
                    userId: UUID(),
                    content: "今日の朝ごはんは、ふわふわの卵焼きと白いご飯、そして温かい味噌汁でした。シンプルだけど、心が落ち着く朝食です。",
                    score: 85 + i,
                    feedback: nil,
                    isPublic: true,
                    likeCount: 10 + i * 3,
                    commentCount: 2 + i,
                    createdAt: Date().addingTimeInterval(-Double(i * 3600))
                ),
                challenge: Challenge(
                    id: UUID(),
                    categoryId: UUID(),
                    prompt: "今日の朝ごはんを50文字以内で説明してください。",
                    description: nil,
                    minCharacters: 20,
                    maxCharacters: 50,
                    difficulty: .easy,
                    createdAt: Date()
                ),
                user: User(
                    id: UUID(),
                    username: "user_\(i)",
                    displayName: ["田中太郎", "佐藤花子", "鈴木一郎", "高橋美咲", "伊藤健太"][i],
                    avatarUrl: nil,
                    bio: nil,
                    level: 10 + i,
                    totalScore: 5000 + i * 1000,
                    streakDays: 5 + i,
                    followerCount: 50 + i * 10,
                    followingCount: 30 + i * 5,
                    answerCount: 100 + i * 20,
                    createdAt: Date()
                ),
                isLiked: i % 2 == 0
            )
        }
    }
}
