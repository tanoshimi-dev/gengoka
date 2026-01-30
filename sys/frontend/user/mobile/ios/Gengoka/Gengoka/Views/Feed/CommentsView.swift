//
//  CommentsView.swift
//  Gengoka
//

import SwiftUI

struct CommentsView: View {
    let answerId: UUID
    @State private var comments: [Comment] = []
    @State private var newComment = ""
    @State private var isLoading = false
    @State private var isSubmitting = false
    @Environment(\.dismiss) private var dismiss

    private let apiClient = APIClient.shared

    var body: some View {
        NavigationStack {
            VStack(spacing: 0) {
                if isLoading {
                    LoadingView(message: "コメントを読み込み中...")
                } else if comments.isEmpty {
                    emptyState
                } else {
                    commentsList
                }

                inputSection
            }
            .background(Color(.systemGroupedBackground))
            .navigationTitle("コメント")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("閉じる") {
                        dismiss()
                    }
                }
            }
        }
        .task {
            await loadComments()
        }
    }

    private var emptyState: some View {
        VStack(spacing: 16) {
            Image(systemName: "bubble.left.and.bubble.right")
                .font(.system(size: 48))
                .foregroundColor(AppColors.textTertiary)

            Text("まだコメントはありません")
                .font(.subheadline)
                .foregroundColor(AppColors.textSecondary)

            Text("最初のコメントを書いてみましょう！")
                .font(.caption)
                .foregroundColor(AppColors.textTertiary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }

    private var commentsList: some View {
        ScrollView {
            LazyVStack(spacing: 0) {
                ForEach(comments) { comment in
                    CommentRow(comment: comment)
                    Divider()
                        .padding(.leading, 60)
                }
            }
            .padding(.vertical)
        }
    }

    private var inputSection: some View {
        VStack(spacing: 0) {
            Divider()

            HStack(spacing: 12) {
                TextField("コメントを入力...", text: $newComment)
                    .textFieldStyle(.plain)
                    .padding(12)
                    .background(Color(.systemGray6))
                    .cornerRadius(20)

                Button(action: {
                    Task { await submitComment() }
                }) {
                    if isSubmitting {
                        ProgressView()
                            .frame(width: 40, height: 40)
                    } else {
                        Image(systemName: "arrow.up.circle.fill")
                            .font(.system(size: 32))
                            .foregroundColor(newComment.isEmpty ? AppColors.textTertiary : AppColors.primaryGradientStart)
                    }
                }
                .disabled(newComment.isEmpty || isSubmitting)
            }
            .padding(.horizontal)
            .padding(.vertical, 12)
            .background(Color(.systemBackground))
        }
    }

    private func loadComments() async {
        isLoading = true

        do {
            comments = try await apiClient.request(.comments(answerId: answerId))
        } catch {
            // Mock comments
            comments = [
                Comment(id: UUID(), answerId: answerId, userId: UUID(), content: "素敵な表現ですね！", user: .mock, createdAt: Date()),
                Comment(id: UUID(), answerId: answerId, userId: UUID(), content: "参考になります！", user: .mock, createdAt: Date().addingTimeInterval(-3600))
            ]
        }

        isLoading = false
    }

    private func submitComment() async {
        guard !newComment.isEmpty else { return }

        isSubmitting = true

        do {
            let submission = CommentSubmission(content: newComment)
            let comment: Comment = try await apiClient.request(.addComment(answerId: answerId), body: submission)
            comments.insert(comment, at: 0)
            newComment = ""
        } catch {
            // Add mock comment
            let mockComment = Comment(
                id: UUID(),
                answerId: answerId,
                userId: AuthService.shared.userId,
                content: newComment,
                user: nil,
                createdAt: Date()
            )
            comments.insert(mockComment, at: 0)
            newComment = ""
        }

        isSubmitting = false
    }
}

struct CommentRow: View {
    let comment: Comment

    var body: some View {
        HStack(alignment: .top, spacing: 12) {
            Circle()
                .fill(AppColors.primaryGradient)
                .frame(width: 36, height: 36)
                .overlay(
                    Text(String((comment.user?.displayName ?? "U").prefix(1)))
                        .font(.subheadline)
                        .foregroundColor(.white)
                )

            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(comment.user?.displayName ?? "ユーザー")
                        .font(.subheadline)
                        .fontWeight(.medium)
                        .foregroundColor(AppColors.textPrimary)

                    Text(timeAgo(from: comment.createdAt))
                        .font(.caption)
                        .foregroundColor(AppColors.textTertiary)
                }

                Text(comment.content)
                    .font(.subheadline)
                    .foregroundColor(AppColors.textPrimary)
            }

            Spacer()
        }
        .padding(.horizontal)
        .padding(.vertical, 12)
    }

    private func timeAgo(from date: Date) -> String {
        let interval = Date().timeIntervalSince(date)

        if interval < 60 {
            return "たった今"
        } else if interval < 3600 {
            return "\(Int(interval / 60))分前"
        } else if interval < 86400 {
            return "\(Int(interval / 3600))時間前"
        } else {
            return "\(Int(interval / 86400))日前"
        }
    }
}

#Preview {
    CommentsView(answerId: UUID())
}
