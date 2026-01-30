//
//  FeedView.swift
//  Gengoka
//

import SwiftUI

struct FeedView: View {
    @State private var viewModel = FeedViewModel()
    @State private var selectedItem: FeedItem?
    @State private var showComments = false
    @State private var selectedUserId: UUID?
    @State private var showProfile = false

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                filterBar

                if viewModel.isLoading && viewModel.feedItems.isEmpty {
                    LoadingView(message: "フィードを読み込み中...")
                } else if viewModel.feedItems.isEmpty {
                    emptyState
                } else {
                    feedList
                }
            }
            .background(AppColors.backgroundGradient.ignoresSafeArea())
            .navigationTitle("タイムライン")
            .navigationBarTitleDisplayMode(.large)
            .refreshable {
                await viewModel.refresh()
            }
            .sheet(isPresented: $showComments) {
                if let item = selectedItem {
                    CommentsView(answerId: item.answer.id)
                        .presentationDetents([.medium, .large])
                }
            }
            .navigationDestination(isPresented: $showProfile) {
                if let userId = selectedUserId {
                    ProfileView(userId: userId)
                }
            }
        }
        .task {
            await viewModel.loadFeed()
        }
    }

    private var filterBar: some View {
        ScrollView(.horizontal, showsIndicators: false) {
            HStack(spacing: 12) {
                FilterChip(
                    title: "すべて",
                    isSelected: viewModel.currentFilter == .all
                ) {
                    Task { await viewModel.setFilter(.all) }
                }

                FilterChip(
                    title: "フォロー中",
                    isSelected: viewModel.currentFilter == .following
                ) {
                    Task { await viewModel.setFilter(.following) }
                }

                FilterChip(
                    title: "人気",
                    isSelected: viewModel.currentFilter == .popular
                ) {
                    Task { await viewModel.setFilter(.popular) }
                }
            }
            .padding(.horizontal)
            .padding(.vertical, 12)
        }
        .background(Color(.systemBackground))
    }

    private var emptyState: some View {
        VStack(spacing: 16) {
            Image(systemName: "doc.text.magnifyingglass")
                .font(.system(size: 48))
                .foregroundColor(AppColors.textTertiary)

            Text("投稿がありません")
                .font(.headline)
                .foregroundColor(AppColors.textPrimary)

            Text("みんなの投稿がここに表示されます")
                .font(.subheadline)
                .foregroundColor(AppColors.textSecondary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    private var feedList: some View {
        ScrollView {
            LazyVStack(spacing: 16) {
                ForEach(viewModel.feedItems) { item in
                    FeedItemView(
                        item: item,
                        onLike: {
                            Task { await viewModel.toggleLike(for: item) }
                        },
                        onComment: {
                            selectedItem = item
                            showComments = true
                        },
                        onUserTap: {
                            selectedUserId = item.user.id
                            showProfile = true
                        }
                    )
                    .onAppear {
                        if item.id == viewModel.feedItems.last?.id {
                            Task { await viewModel.loadMore() }
                        }
                    }
                }

                if viewModel.isLoadingMore {
                    ProgressView()
                        .padding()
                }
            }
            .padding()
        }
    }
}

struct FilterChip: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.subheadline)
                .fontWeight(isSelected ? .semibold : .regular)
                .foregroundColor(isSelected ? .white : AppColors.textPrimary)
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(
                    Group {
                        if isSelected {
                            AppColors.primaryGradient
                        } else {
                            Color(.systemGray6)
                        }
                    }
                )
                .cornerRadius(20)
        }
    }
}

#Preview {
    FeedView()
}
