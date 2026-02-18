//
//  ProfileViewModel.swift
//  Gengoka
//

import Foundation

@Observable
final class ProfileViewModel {
    var user: User?
    var stats: UserStats?
    var recentAnswers: [Answer] = []
    var isFollowing = false
    var isLoading = false
    var isLoadingStats = false
    var error: Error?

    private let apiClient: any APIClientProtocol
    private let authService: any AuthServiceProtocol

    init(apiClient: any APIClientProtocol = APIClient.shared,
         authService: any AuthServiceProtocol = AuthService.shared) {
        self.apiClient = apiClient
        self.authService = authService
    }

    var isCurrentUser: Bool {
        user?.id == authService.userId
    }

    func loadProfile(userId: UUID) async {
        isLoading = true
        error = nil

        do {
            let profile: UserProfile = try await apiClient.request(.user(id: userId))
            user = profile.user
            isFollowing = profile.isFollowing
            recentAnswers = profile.recentAnswers
        } catch {
            self.error = error
            // Use mock data
            user = User.mock
            recentAnswers = [Answer.mock]
        }

        isLoading = false
    }

    func loadCurrentUser() async {
        isLoading = true
        error = nil

        // First, check if we have authenticated user data
        if let authUser = authService.currentUser {
            // Convert AuthUser to User for display
            user = User(
                id: authUser.id,
                username: authUser.username ?? "user",
                displayName: authUser.displayName ?? authUser.username ?? "User",
                avatarUrl: authUser.avatarUrl,
                bio: authUser.bio,
                level: 1,  // Default values since auth doesn't have these
                totalScore: 0,
                streakDays: 0,
                followerCount: 0,
                followingCount: 0,
                answerCount: 0,
                createdAt: authUser.createdAt ?? Date()
            )
            
            // Try to load stats separately
            do {
                stats = try await apiClient.request(.userStats)
            } catch {
                stats = UserStats(
                    totalChallenges: 0,
                    completedToday: 0,
                    currentStreak: 0,
                    bestStreak: 0,
                    averageScore: 0
                )
            }
            
            isLoading = false
            return
        }

        // Fallback: Try to load from API
        do {
            async let userTask: User = apiClient.request(.currentUser)
            async let statsTask: UserStats = apiClient.request(.userStats)

            let (loadedUser, loadedStats) = try await (userTask, statsTask)
            user = loadedUser
            stats = loadedStats
        } catch {
            self.error = error
            // Last resort: use mock data
            user = User.mock
            stats = UserStats.mock
        }

        isLoading = false
    }

    func toggleFollow() async {
        guard let userId = user?.id, !isCurrentUser else { return }

        let wasFollowing = isFollowing
        isFollowing.toggle()

        // Update follower count optimistically
        if var currentUser = user {
            let newCount = wasFollowing ? currentUser.followerCount - 1 : currentUser.followerCount + 1
            user = User(
                id: currentUser.id,
                username: currentUser.username,
                displayName: currentUser.displayName,
                avatarUrl: currentUser.avatarUrl,
                bio: currentUser.bio,
                level: currentUser.level,
                totalScore: currentUser.totalScore,
                streakDays: currentUser.streakDays,
                followerCount: newCount,
                followingCount: currentUser.followingCount,
                answerCount: currentUser.answerCount,
                createdAt: currentUser.createdAt
            )
        }

        do {
            if wasFollowing {
                try await apiClient.requestVoid(.unfollowUser(id: userId))
            } else {
                try await apiClient.requestVoid(.followUser(id: userId))
            }
        } catch {
            // Revert on error
            isFollowing = wasFollowing
            if var currentUser = user {
                let newCount = wasFollowing ? currentUser.followerCount + 1 : currentUser.followerCount - 1
                user = User(
                    id: currentUser.id,
                    username: currentUser.username,
                    displayName: currentUser.displayName,
                    avatarUrl: currentUser.avatarUrl,
                    bio: currentUser.bio,
                    level: currentUser.level,
                    totalScore: currentUser.totalScore,
                    streakDays: currentUser.streakDays,
                    followerCount: newCount,
                    followingCount: currentUser.followingCount,
                    answerCount: currentUser.answerCount,
                    createdAt: currentUser.createdAt
                )
            }
        }
    }
}
