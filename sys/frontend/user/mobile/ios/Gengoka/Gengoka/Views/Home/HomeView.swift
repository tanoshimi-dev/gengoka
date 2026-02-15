//
//  HomeView.swift
//  Gengoka
//

import SwiftUI

struct HomeView: View {
    @State private var viewModel = HomeViewModel()
    @State private var selectedCategory: Category?
    @State private var showChallenge = false
    @State private var showErrorAlert = false
    @State private var loadTrigger = UUID() // Used to control when to reload

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    #if DEBUG
                    debugSection
                    #endif
                    
                    headerSection
                        .transition(.move(edge: .top).combined(with: .opacity))

                    if let stats = viewModel.userStats {
                        StatsBarView(stats: stats)
                            .padding(.horizontal)
                            .transition(.move(edge: .leading).combined(with: .opacity))
                    }

                    dailyChallengeSection
                        .transition(.move(edge: .leading).combined(with: .opacity))

                    categoriesSection
                        .transition(.move(edge: .bottom).combined(with: .opacity))
                }
                .padding(.vertical)
                .animation(.easeInOut(duration: 0.3), value: viewModel.isLoading)
            }
            .background(AppColors.backgroundGradient.ignoresSafeArea())
            .refreshable {
                loadTrigger = UUID() // Trigger a fresh load
            }
            .navigationDestination(isPresented: $showChallenge) {
                if let category = selectedCategory {
                    ChallengeView(category: category)
                }
            }
            .overlay {
                if viewModel.isLoading && viewModel.categories.isEmpty {
                    LoadingView()
                        .transition(.opacity)
                }
            }
            .alert("エラーが発生しました", isPresented: $showErrorAlert) {
                Button("OK", role: .cancel) {
                    showErrorAlert = false
                }
                Button("再試行") {
                    Task {
                        await viewModel.refresh()
                    }
                }
                #if DEBUG
                Button("詳細を表示") {
                    if let error = viewModel.error {
                        print("📋 Full error details:")
                        print(error)
                        dump(error)
                    }
                }
                #endif
            } message: {
                if let error = viewModel.error {
                    #if DEBUG
                    Text("\(error.localizedDescription)\n\nデバッグ情報: \(String(describing: error))")
                    #else
                    Text(error.localizedDescription)
                    #endif
                }
            }
        }
        .task(id: loadTrigger) {
            await viewModel.loadData()
        }
        .onChange(of: viewModel.error != nil) { oldValue, newValue in
            if newValue {
                showErrorAlert = true
            }
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
    
    #if DEBUG
    private var debugSection: some View {
        VStack(spacing: 8) {
            HStack {
                Image(systemName: "ladybug.fill")
                    .foregroundColor(.orange)
                Text("デバッグモード")
                    .font(.caption)
                    .fontWeight(.bold)
                Spacer()
                Button("APIテスト") {
                    Task {
                        await APIDebugHelper.testEndpoints()
                    }
                }
                .font(.caption)
                .buttonStyle(.bordered)
            }
            
            if let error = viewModel.error {
                Text("エラー: \(error.localizedDescription)")
                    .font(.caption2)
                    .foregroundColor(.red)
                    .frame(maxWidth: .infinity, alignment: .leading)
            }
        }
        .padding()
        .background(Color.orange.opacity(0.1))
        .cornerRadius(8)
        .padding(.horizontal)
    }
    #endif

    private var dailyChallengeSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Label("今日のチャレンジ", systemImage: "flame.fill")
                    .font(.headline)
                    .foregroundColor(AppColors.textPrimary)
                    .symbolRenderingMode(.multicolor)

                Spacer()

                if !viewModel.dailyChallenges.isEmpty {
                    let incompleteCount = viewModel.dailyChallenges.filter { !$0.isCompleted }.count
                    
                    HStack(spacing: 4) {
                        if incompleteCount > 0 {
                            Circle()
                                .fill(Color.orange)
                                .frame(width: 8, height: 8)
                        }
                        Text("\(incompleteCount)個未完了")
                            .font(.caption)
                            .foregroundColor(AppColors.textSecondary)
                    }
                }
            }
            .padding(.horizontal)
            .accessibilityElement(children: .combine)

            if viewModel.isLoading && viewModel.dailyChallenges.isEmpty {
                DailyChallengePlaceholder()
                    .padding(.horizontal)
            } else if viewModel.dailyChallenges.isEmpty {
                EmptyDailyChallengeView()
                    .padding(.horizontal)
            } else {
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack(spacing: 12) {
                        ForEach(viewModel.dailyChallenges, id: \.challenge.id) { daily in
                            DailyChallengeCard(dailyChallenge: daily) {
                                selectedCategory = viewModel.categories.first { $0.id == daily.challenge.categoryId }
                                showChallenge = true
                            }
                        }
                    }
                    .padding(.horizontal)
                }
                .scrollClipDisabled()
            }
        }
    }

    private var categoriesSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("カテゴリーを選択")
                .font(.headline)
                .foregroundColor(AppColors.textPrimary)
                .padding(.horizontal)

            if viewModel.categories.isEmpty && !viewModel.isLoading {
                EmptyCategoriesView()
                    .padding()
            } else {
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
                            .imageScale(.large)
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
                    HStack(spacing: 4) {
                        Image(systemName: "text.alignleft")
                            .font(.caption2)
                        // Better display for char_limit format
                        if dailyChallenge.challenge.minCharacters > 0 {
                            Text("\(dailyChallenge.challenge.minCharacters)-\(dailyChallenge.challenge.maxCharacters)文字")
                                .font(.caption2)
                        } else {
                            Text("\(dailyChallenge.challenge.maxCharacters)文字以内")
                                .font(.caption2)
                        }
                    }
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
            .shadow(color: .black.opacity(0.1), radius: 8, x: 0, y: 4)
        }
        .buttonStyle(ScaleButtonStyle())
        .accessibilityLabel("\(dailyChallenge.categoryName): \(dailyChallenge.challenge.prompt)")
        .accessibilityHint(dailyChallenge.isCompleted ? "完了済み" : "タップしてチャレンジを開始")
    }
}

struct DailyChallengePlaceholder: View {
    @State private var isAnimating = false
    
    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: "sparkles")
                .font(.largeTitle)
                .foregroundStyle(AppColors.primaryGradient)
                .symbolEffect(.pulse, isActive: isAnimating)

            Text("今日のチャレンジを取得中...")
                .font(.subheadline)
                .foregroundColor(AppColors.textSecondary)
        }
        .frame(maxWidth: .infinity)
        .frame(height: 140)
        .background(Color.white)
        .cornerRadius(16)
        .onAppear {
            isAnimating = true
        }
    }
}

struct EmptyDailyChallengeView: View {
    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: "checkmark.circle.fill")
                .font(.system(size: 48))
                .foregroundStyle(AppColors.primaryGradient)
            
            Text("すべて完了！")
                .font(.headline)
                .foregroundColor(AppColors.textPrimary)
            
            Text("今日のチャレンジをすべてクリアしました。素晴らしい！")
                .font(.subheadline)
                .foregroundColor(AppColors.textSecondary)
                .multilineTextAlignment(.center)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 32)
        .background(Color.white)
        .cornerRadius(16)
    }
}

struct EmptyCategoriesView: View {
    var body: some View {
        VStack(spacing: 12) {
            Image(systemName: "folder.fill.badge.questionmark")
                .font(.system(size: 48))
                .foregroundColor(AppColors.textSecondary)
            
            Text("カテゴリーがありません")
                .font(.headline)
                .foregroundColor(AppColors.textPrimary)
            
            Text("引き下げて再読み込みしてください")
                .font(.subheadline)
                .foregroundColor(AppColors.textSecondary)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 40)
        .background(Color.white.opacity(0.5))
        .cornerRadius(16)
    }
}

#Preview {
    HomeView()
}
