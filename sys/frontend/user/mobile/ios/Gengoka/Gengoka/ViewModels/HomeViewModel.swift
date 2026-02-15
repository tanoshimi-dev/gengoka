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
    private var isInitialLoadComplete = false
    private var currentLoadTask: Task<Void, Never>? // Track the current load task
    
    func loadData() async {
        // Cancel any existing load task
        currentLoadTask?.cancel()
        
        // Prevent duplicate initial loads
        guard !isLoading else {
            print("⚠️ Already loading, skipping duplicate loadData() call")
            return
        }
        
        // Create new task and store reference
        currentLoadTask = Task {
            await performLoad()
        }
        
        await currentLoadTask?.value
    }
    
    private func performLoad() async {
        isLoading = true
        error = nil

        // Load categories first (needed for daily challenges)
        var loadedCategories: [Category] = []
        do {
            print("📡 Loading categories...")
            loadedCategories = try await apiClient.request(.categories)
            
            // Check if cancelled
            guard !Task.isCancelled else {
                print("⚠️ Load cancelled during categories")
                isLoading = false
                return
            }
            
            categories = loadedCategories
            print("✅ Categories loaded: \(loadedCategories.count) items")
        } catch {
            print("❌ Categories error: \(error)")
            
            // Don't show cancellation errors to the user
            if !isCancellationError(error) {
                self.error = error
            }
            
            categories = Category.mockCategories
            loadedCategories = categories
        }

        // Load user stats
        do {
            print("📡 Loading user stats...")
            let loadedStats: UserStats = try await apiClient.request(.userStats)
            
            guard !Task.isCancelled else {
                print("⚠️ Load cancelled during user stats")
                isLoading = false
                return
            }
            
            userStats = loadedStats
            print("✅ User stats loaded")
        } catch {
            print("❌ User stats error: \(error)")
            userStats = UserStats.mock
            
            if self.error == nil && !isCancellationError(error) {
                self.error = error
            }
        }

        // Load daily challenges
        do {
            print("📡 Loading daily challenges...")
            let loadedDaily: [DailyChallenge] = try await apiClient.request(.dailyChallenges)
            
            guard !Task.isCancelled else {
                print("⚠️ Load cancelled during daily challenges")
                isLoading = false
                return
            }
            
            dailyChallenges = loadedDaily
            print("✅ Daily challenges loaded: \(loadedDaily.count) items")
        } catch {
            print("❌ Daily challenges error: \(error)")
            dailyChallenges = []

            if self.error == nil && !isCancellationError(error) {
                self.error = error
            }
        }

        isLoading = false
        isInitialLoadComplete = true
    }
    
    // Helper to check if error is a cancellation error
    private func isCancellationError(_ error: Error) -> Bool {
        if error.isCancellationError {
            print("⚠️ Request was cancelled, ignoring error")
            return true
        }
        return false
    }

    func refresh() async {
        // Simply reload data - the task cancellation will be handled automatically
        await loadData()
    }
}
