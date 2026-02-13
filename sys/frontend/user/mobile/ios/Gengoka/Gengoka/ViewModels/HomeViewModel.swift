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

        // Load categories first (needed for daily challenges)
        var loadedCategories: [Category] = []
        do {
            print("📡 Loading categories...")
            loadedCategories = try await apiClient.request(.categories)
            categories = loadedCategories
            print("✅ Categories loaded: \(loadedCategories.count) items")
        } catch {
            print("❌ Categories error: \(error)")
            categories = Category.mockCategories
            loadedCategories = categories
            self.error = error
        }

        // Load user stats
        do {
            print("📡 Loading user stats...")
            let loadedStats: UserStats = try await apiClient.request(.userStats)
            userStats = loadedStats
            print("✅ User stats loaded")
        } catch {
            print("❌ User stats error: \(error)")
            userStats = UserStats.mock
            if self.error == nil {
                self.error = error
            }
        }

        // Load daily challenges (try both formats)
        do {
            print("📡 Loading daily challenges...")
            
            // Try to load as DailyChallenge first (iOS format with nested structure)
            do {
                let loadedDaily: [DailyChallenge] = try await apiClient.request(.dailyChallenges)
                dailyChallenges = loadedDaily
                print("✅ Daily challenges loaded (iOS format): \(loadedDaily.count) items")
            } catch let dailyError {
                // If that fails, try loading as Challenge array (Android format)
                print("⚠️ iOS format failed, trying Android format...")
                let challenges: [Challenge] = try await apiClient.request(.dailyChallenges)
                
                // Convert to DailyChallenge with category names
                dailyChallenges = challenges.map { challenge in
                    let categoryName = loadedCategories.first { $0.id == challenge.categoryId }?.name ?? "未分類"
                    return DailyChallenge(
                        challenge: challenge,
                        categoryName: categoryName,
                        isCompleted: false
                    )
                }
                print("✅ Daily challenges loaded (Android format): \(challenges.count) items")
            }
        } catch {
            print("❌ Daily challenges error: \(error)")
            dailyChallenges = []
            if self.error == nil {
                self.error = error
            }
        }

        isLoading = false
    }

    func refresh() async {
        await loadData()
    }
}
