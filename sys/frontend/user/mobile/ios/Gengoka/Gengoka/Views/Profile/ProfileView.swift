//
//  ProfileView.swift
//  Gengoka
//

import SwiftUI

struct ProfileView: View {
    let userId: UUID
    @State private var viewModel = ProfileViewModel()
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                if let user = viewModel.user {
                    headerSection(user)
                    statsSection(user)

                    if !viewModel.recentAnswers.isEmpty {
                        answersSection
                    }
                }
            }
            .padding()
        }
        .background(AppColors.backgroundGradient.ignoresSafeArea())
        .navigationBarTitleDisplayMode(.inline)
        .navigationBarBackButtonHidden(true)
        .toolbar {
            ToolbarItem(placement: .navigationBarLeading) {
                BackButton()
            }
        }
        .overlay {
            if viewModel.isLoading {
                LoadingView(message: "プロフィールを読み込み中...")
            }
        }
        .task {
            await viewModel.loadProfile(userId: userId)
        }
    }

    private func headerSection(_ user: User) -> some View {
        VStack(spacing: 16) {
            Circle()
                .fill(AppColors.primaryGradient)
                .frame(width: 100, height: 100)
                .overlay(
                    Text(String(user.displayName.prefix(1)))
                        .font(.system(size: 40, weight: .bold))
                        .foregroundColor(.white)
                )

            VStack(spacing: 4) {
                Text(user.displayName)
                    .font(.title2)
                    .fontWeight(.bold)
                    .foregroundColor(AppColors.textPrimary)

                Text("@\(user.username)")
                    .font(.subheadline)
                    .foregroundColor(AppColors.textSecondary)
            }

            if let bio = user.bio {
                Text(bio)
                    .font(.subheadline)
                    .foregroundColor(AppColors.textSecondary)
                    .multilineTextAlignment(.center)
                    .padding(.horizontal)
            }

            HStack(spacing: 8) {
                LevelBadge(level: user.level)

                HStack(spacing: 4) {
                    Image(systemName: "flame.fill")
                        .foregroundColor(.orange)
                    Text("\(user.streakDays)日連続")
                        .font(.caption)
                        .foregroundColor(AppColors.textSecondary)
                }
                .padding(.horizontal, 10)
                .padding(.vertical, 6)
                .background(Color(.systemGray6))
                .cornerRadius(12)
            }

            if !viewModel.isCurrentUser {
                followButton
            }
        }
        .padding(24)
        .frame(maxWidth: .infinity)
        .background(Color.white)
        .cornerRadius(24)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private var followButton: some View {
        Button(action: {
            Task { await viewModel.toggleFollow() }
        }) {
            HStack {
                Image(systemName: viewModel.isFollowing ? "checkmark" : "plus")
                Text(viewModel.isFollowing ? "フォロー中" : "フォローする")
            }
            .font(.subheadline)
            .fontWeight(.semibold)
            .foregroundColor(viewModel.isFollowing ? AppColors.textPrimary : .white)
            .padding(.horizontal, 24)
            .padding(.vertical, 12)
            .background(
                Group {
                    if viewModel.isFollowing {
                        Color(.systemGray6)
                    } else {
                        AppColors.primaryGradient
                    }
                }
            )
            .cornerRadius(20)
        }
    }

    private func statsSection(_ user: User) -> some View {
        HStack(spacing: 0) {
            ProfileStatItem(value: "\(user.followerCount)", label: "フォロワー")
            Divider().frame(height: 40)
            ProfileStatItem(value: "\(user.followingCount)", label: "フォロー中")
            Divider().frame(height: 40)
            ProfileStatItem(value: "\(user.answerCount)", label: "回答数")
        }
        .padding(.vertical, 16)
        .background(Color.white)
        .cornerRadius(16)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private var answersSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("最近の回答")
                .font(.headline)
                .foregroundColor(AppColors.textPrimary)

            ForEach(viewModel.recentAnswers) { answer in
                AnswerCard(answer: answer)
            }
        }
    }
}

struct ProfileStatItem: View {
    let value: String
    let label: String

    var body: some View {
        VStack(spacing: 4) {
            Text(value)
                .font(.title2)
                .fontWeight(.bold)
                .foregroundColor(AppColors.textPrimary)

            Text(label)
                .font(.caption)
                .foregroundColor(AppColors.textSecondary)
        }
        .frame(maxWidth: .infinity)
    }
}

struct LevelBadge: View {
    let level: Int

    var body: some View {
        HStack(spacing: 4) {
            Image(systemName: "star.fill")
                .foregroundColor(AppColors.warning)
            Text("Lv.\(level)")
                .font(.caption)
                .fontWeight(.semibold)
                .foregroundColor(AppColors.textPrimary)
        }
        .padding(.horizontal, 10)
        .padding(.vertical, 6)
        .background(AppColors.warning.opacity(0.15))
        .cornerRadius(12)
    }
}

struct AnswerCard: View {
    let answer: Answer

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(answer.content)
                .font(.subheadline)
                .foregroundColor(AppColors.textPrimary)
                .lineLimit(3)

            HStack {
                if let score = answer.score {
                    HStack(spacing: 4) {
                        Image(systemName: "checkmark.seal.fill")
                            .font(.caption)
                            .foregroundColor(AppColors.success)
                        Text("\(score)点")
                            .font(.caption)
                            .foregroundColor(AppColors.textSecondary)
                    }
                }

                Spacer()

                HStack(spacing: 12) {
                    HStack(spacing: 4) {
                        Image(systemName: "heart.fill")
                            .font(.caption)
                            .foregroundColor(AppColors.error)
                        Text("\(answer.likeCount)")
                            .font(.caption)
                            .foregroundColor(AppColors.textSecondary)
                    }

                    HStack(spacing: 4) {
                        Image(systemName: "bubble.left.fill")
                            .font(.caption)
                            .foregroundColor(AppColors.primaryGradientStart)
                        Text("\(answer.commentCount)")
                            .font(.caption)
                            .foregroundColor(AppColors.textSecondary)
                    }
                }
            }
        }
        .padding(16)
        .background(Color.white)
        .cornerRadius(16)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }
}

#Preview {
    NavigationStack {
        ProfileView(userId: UUID())
    }
}
