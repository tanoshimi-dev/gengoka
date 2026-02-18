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

    private let apiClient: any APIClientProtocol

    init(apiClient: any APIClientProtocol = APIClient.shared) {
        self.apiClient = apiClient
    }

    func loadFeed() async {
        isLoading = true
        error = nil
        currentPage = 1

        do {
            // Backend returns FeedResponse with data array and pagination object (NOT wrapped in APIResponse)
            let response: FeedResponse<FeedItem> = try await apiClient.request(.feed(page: 1, filter: currentFilter))
            feedItems = response.data
            hasMore = response.pagination.hasMore
            
            #if DEBUG
            print("✅ Successfully loaded \(response.data.count) feed items")
            print("📄 Page: \(response.pagination.page), Total: \(response.pagination.total), Has More: \(response.pagination.hasMore)")
            if let first = response.data.first {
                print("📝 First item - Answer ID: \(first.answer.id), Challenge: \(first.challenge.prompt)")
            } else {
                print("⚠️ No feed items in response - backend may not have any public answers yet")
            }
            #endif
        } catch {
            #if DEBUG
            print("❌ Failed to load feed: \(error)")
            print("❌ Error details: \(error.localizedDescription)")
            if let decodingError = error as? DecodingError {
                switch decodingError {
                case .keyNotFound(let key, let context):
                    print("❌ Key '\(key.stringValue)' not found at path: \(context.codingPath.map { $0.stringValue })")
                    print("   Debug description: \(context.debugDescription)")
                case .typeMismatch(let type, let context):
                    print("❌ Type mismatch for \(type) at path: \(context.codingPath.map { $0.stringValue })")
                    print("   Debug description: \(context.debugDescription)")
                case .valueNotFound(let type, let context):
                    print("❌ Value not found for \(type) at path: \(context.codingPath.map { $0.stringValue })")
                    print("   Debug description: \(context.debugDescription)")
                case .dataCorrupted(let context):
                    print("❌ Data corrupted at path: \(context.codingPath.map { $0.stringValue })")
                    print("   Debug description: \(context.debugDescription)")
                @unknown default:
                    print("❌ Unknown decoding error")
                }
            }
            print("\n💡 TIP: Check the console for the raw JSON response from APIClient")
            print("💡 TIP: Make sure you have at least one public answer in the backend database")
            #endif
            self.error = error
            feedItems = []
            hasMore = false
        }

        isLoading = false
    }

    func loadMore() async {
        guard !isLoadingMore && hasMore else { return }

        isLoadingMore = true
        currentPage += 1

        do {
            // Backend returns FeedResponse
            let response: FeedResponse<FeedItem> = try await apiClient.request(.feed(page: currentPage, filter: currentFilter))
            feedItems.append(contentsOf: response.data)
            hasMore = response.pagination.hasMore
            
            #if DEBUG
            print("✅ Loaded \(response.data.count) more feed items (page \(currentPage))")
            #endif
        } catch {
            currentPage -= 1
            hasMore = false
            
            #if DEBUG
            print("❌ Failed to load more feed items: \(error)")
            #endif
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
            isLiked: !item.isLiked,
            hasUserAnswered: item.hasUserAnswered
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
}
