//
//  MyProfileView.swift
//  Gengoka
//

import SwiftUI

struct MyProfileView: View {
    @State private var viewModel = ProfileViewModel()

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    if let user = viewModel.user {
                        headerSection(user)

                        if let stats = viewModel.stats {
                            statsSection(stats)
                        }

                        socialSection(user)
                        settingsSection
                    }
                }
                .padding()
            }
            .background(AppColors.backgroundGradient.ignoresSafeArea())
            .navigationTitle("マイページ")
            .navigationBarTitleDisplayMode(.large)
            .refreshable {
                await viewModel.loadCurrentUser()
            }
            .overlay {
                if viewModel.isLoading && viewModel.user == nil {
                    LoadingView()
                }
            }
        }
        .task {
            await viewModel.loadCurrentUser()
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
                    Image(systemName: "star.fill")
                        .foregroundColor(AppColors.primaryGradientStart)
                    Text("\(user.totalScore)pt")
                        .font(.caption)
                        .fontWeight(.medium)
                        .foregroundColor(AppColors.textPrimary)
                }
                .padding(.horizontal, 10)
                .padding(.vertical, 6)
                .background(AppColors.primaryGradientStart.opacity(0.15))
                .cornerRadius(12)
            }

            Button(action: {}) {
                HStack {
                    Image(systemName: "pencil")
                    Text("プロフィールを編集")
                }
                .font(.subheadline)
                .fontWeight(.medium)
                .foregroundColor(AppColors.primaryGradientStart)
                .padding(.horizontal, 20)
                .padding(.vertical, 10)
                .background(Color.white)
                .cornerRadius(16)
                .overlay(
                    RoundedRectangle(cornerRadius: 16)
                        .stroke(AppColors.primaryGradientStart, lineWidth: 1)
                )
            }
        }
        .padding(24)
        .frame(maxWidth: .infinity)
        .background(Color.white)
        .cornerRadius(24)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private func statsSection(_ stats: UserStats) -> some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("学習統計")
                .font(.headline)
                .foregroundColor(AppColors.textPrimary)

            LazyVGrid(columns: [
                GridItem(.flexible()),
                GridItem(.flexible())
            ], spacing: 12) {
                MyProfileStatCard(
                    icon: "flame.fill",
                    iconColor: .orange,
                    value: "\(stats.currentStreak)日",
                    label: "連続チャレンジ",
                    subLabel: "最高: \(stats.bestStreak)日"
                )

                MyProfileStatCard(
                    icon: "checkmark.circle.fill",
                    iconColor: AppColors.success,
                    value: "\(stats.completedToday)",
                    label: "今日の完了",
                    subLabel: nil
                )

                MyProfileStatCard(
                    icon: "chart.line.uptrend.xyaxis",
                    iconColor: AppColors.primaryGradientStart,
                    value: String(format: "%.1f", stats.averageScore),
                    label: "平均スコア",
                    subLabel: nil
                )

                MyProfileStatCard(
                    icon: "doc.text.fill",
                    iconColor: AppColors.warning,
                    value: "\(stats.totalChallenges)",
                    label: "総チャレンジ数",
                    subLabel: nil
                )
            }
        }
        .padding(20)
        .background(Color.white)
        .cornerRadius(20)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private func socialSection(_ user: User) -> some View {
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

    private var settingsSection: some View {
        VStack(spacing: 0) {
            SettingsRow(icon: "bell.fill", title: "通知設定", iconColor: AppColors.error)
            Divider().padding(.leading, 52)
            SettingsRow(icon: "lock.fill", title: "プライバシー設定", iconColor: AppColors.primaryGradientStart)
            Divider().padding(.leading, 52)
            SettingsRow(icon: "questionmark.circle.fill", title: "ヘルプ", iconColor: AppColors.success)
            Divider().padding(.leading, 52)
            SettingsRow(icon: "info.circle.fill", title: "アプリについて", iconColor: AppColors.textSecondary)
        }
        .background(Color.white)
        .cornerRadius(16)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }
}

struct MyProfileStatCard: View {
    let icon: String
    let iconColor: Color
    let value: String
    let label: String
    let subLabel: String?

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                Image(systemName: icon)
                    .font(.system(size: 20))
                    .foregroundColor(iconColor)
                Spacer()
            }

            Text(value)
                .font(.title2)
                .fontWeight(.bold)
                .foregroundColor(AppColors.textPrimary)

            Text(label)
                .font(.caption)
                .foregroundColor(AppColors.textSecondary)

            if let subLabel = subLabel {
                Text(subLabel)
                    .font(.caption2)
                    .foregroundColor(AppColors.textTertiary)
            }
        }
        .padding(16)
        .background(Color(.systemGray6))
        .cornerRadius(16)
    }
}

struct SettingsRow: View {
    let icon: String
    let title: String
    let iconColor: Color

    var body: some View {
        Button(action: {}) {
            HStack(spacing: 16) {
                Image(systemName: icon)
                    .font(.system(size: 20))
                    .foregroundColor(iconColor)
                    .frame(width: 28)

                Text(title)
                    .font(.subheadline)
                    .foregroundColor(AppColors.textPrimary)

                Spacer()

                Image(systemName: "chevron.right")
                    .font(.caption)
                    .foregroundColor(AppColors.textTertiary)
            }
            .padding(.horizontal, 16)
            .padding(.vertical, 14)
        }
    }
}

#Preview {
    MyProfileView()
}
