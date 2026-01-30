//
//  HomeView.swift
//  Gengoka
//

import SwiftUI

struct HomeView: View {
    @State private var viewModel = HomeViewModel()
    @State private var selectedCategory: Category?
    @State private var showChallenge = false

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    headerSection

                    if let stats = viewModel.userStats {
                        StatsBarView(stats: stats)
                            .padding(.horizontal)
                    }

                    dailyChallengeSection

                    categoriesSection
                }
                .padding(.vertical)
            }
            .background(AppColors.backgroundGradient.ignoresSafeArea())
            .refreshable {
                await viewModel.refresh()
            }
            .navigationDestination(isPresented: $showChallenge) {
                if let category = selectedCategory {
                    ChallengeView(category: category)
                }
            }
            .overlay {
                if viewModel.isLoading && viewModel.categories.isEmpty {
                    LoadingView()
                }
            }
        }
        .task {
            await viewModel.loadData()
        }
    }

    private var headerSection: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("おはようございます")
                .font(.subheadline)
                .foregroundColor(AppColors.textSecondary)

            Text("今日も言葉を磨こう")
                .font(.title)
                .fontWeight(.bold)
                .foregroundColor(AppColors.textPrimary)
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(.horizontal)
    }

    private var dailyChallengeSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("今日のチャレンジ")
                    .font(.headline)
                    .foregroundColor(AppColors.textPrimary)

                Spacer()

                if !viewModel.dailyChallenges.isEmpty {
                    Text("\(viewModel.dailyChallenges.filter { !$0.isCompleted }.count)個未完了")
                        .font(.caption)
                        .foregroundColor(AppColors.textSecondary)
                }
            }
            .padding(.horizontal)

            if viewModel.dailyChallenges.isEmpty {
                DailyChallengePlaceholder()
                    .padding(.horizontal)
            } else {
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 12) {
                        ForEach(viewModel.dailyChallenges, id: \.challenge.id) { daily in
                            DailyChallengeCard(dailyChallenge: daily) {
                                // Navigate to challenge
                            }
                        }
                    }
                    .padding(.horizontal)
                }
            }
        }
    }

    private var categoriesSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("カテゴリーを選択")
                .font(.headline)
                .foregroundColor(AppColors.textPrimary)
                .padding(.horizontal)

            LazyVGrid(columns: [
                GridItem(.flexible(), spacing: 12),
                GridItem(.flexible(), spacing: 12)
            ], spacing: 12) {
                ForEach(viewModel.categories) { category in
                    CategoryCardView(category: category) {
                        selectedCategory = category
                        showChallenge = true
                    }
                }
            }
            .padding(.horizontal)
        }
    }
}

struct DailyChallengeCard: View {
    let dailyChallenge: DailyChallenge
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(alignment: .leading, spacing: 8) {
                HStack {
                    Text(dailyChallenge.categoryName)
                        .font(.caption)
                        .fontWeight(.medium)
                        .foregroundColor(.white)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 4)
                        .background(Color.white.opacity(0.2))
                        .cornerRadius(8)

                    Spacer()

                    if dailyChallenge.isCompleted {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundColor(.white)
                    }
                }

                Text(dailyChallenge.challenge.prompt)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
                    .lineLimit(2)
                    .multilineTextAlignment(.leading)

                Spacer()

                HStack {
                    Text("\(dailyChallenge.challenge.minCharacters)-\(dailyChallenge.challenge.maxCharacters)文字")
                        .font(.caption2)
                        .foregroundColor(.white.opacity(0.8))

                    Spacer()

                    Image(systemName: "arrow.right")
                        .font(.caption)
                        .foregroundColor(.white)
                }
            }
            .padding(16)
            .frame(width: 200, height: 140)
            .background(AppColors.primaryGradient)
            .cornerRadius(16)
        }
        .buttonStyle(ScaleButtonStyle())
    }
}

struct DailyChallengePlaceholder: View {
    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: "sparkles")
                .font(.largeTitle)
                .foregroundColor(AppColors.primaryGradientStart)

            Text("今日のチャレンジを取得中...")
                .font(.subheadline)
                .foregroundColor(AppColors.textSecondary)
        }
        .frame(maxWidth: .infinity)
        .frame(height: 140)
        .background(Color.white)
        .cornerRadius(16)
    }
}

#Preview {
    HomeView()
}
