//
//  HomeViewModel.swift
//  Gengoka
//

import Foundation

@Observable
final class HomeViewModel {
    var categories: [Category] = []
    var dailyChallenges: [DailyChallenge] = []
    var userStats: UserStats?
    var isLoading = false
    var error: Error?

    private let apiClient = APIClient.shared

    func loadData() async {
        isLoading = true
        error = nil

        do {
            async let categoriesTask: [Category] = apiClient.request(.categories)
            async let statsTask: UserStats = apiClient.request(.userStats)
            async let dailyTask: [DailyChallenge] = apiClient.request(.dailyChallenges)

            let (loadedCategories, loadedStats, loadedDaily) = try await (categoriesTask, statsTask, dailyTask)

            categories = loadedCategories
            userStats = loadedStats
            dailyChallenges = loadedDaily
        } catch {
            self.error = error
            // Use mock data for development
            categories = Category.mockCategories
            userStats = UserStats.mock
        }

        isLoading = false
    }

    func refresh() async {
        await loadData()
    }
}
