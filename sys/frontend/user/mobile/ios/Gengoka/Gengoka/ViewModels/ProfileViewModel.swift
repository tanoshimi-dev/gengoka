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

    private let apiClient = APIClient.shared

    var isCurrentUser: Bool {
        user?.id == AuthService.shared.userId
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

        do {
            async let userTask: User = apiClient.request(.currentUser)
            async let statsTask: UserStats = apiClient.request(.userStats)

            let (loadedUser, loadedStats) = try await (userTask, statsTask)
            user = loadedUser
            stats = loadedStats
        } catch {
            self.error = error
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
