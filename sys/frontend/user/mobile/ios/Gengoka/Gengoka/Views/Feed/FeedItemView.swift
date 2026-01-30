//
//  FeedItemView.swift
//  Gengoka
//

import SwiftUI

struct FeedItemView: View {
    let item: FeedItem
    let onLike: () -> Void
    let onComment: () -> Void
    let onUserTap: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            headerSection
            challengeSection
            answerSection
            scoreSection
            actionsSection
        }
        .padding(16)
        .background(Color.white)
        .cornerRadius(20)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private var headerSection: some View {
        Button(action: onUserTap) {
            HStack(spacing: 12) {
                Circle()
                    .fill(AppColors.primaryGradient)
                    .frame(width: 44, height: 44)
                    .overlay(
                        Text(String(item.user.displayName.prefix(1)))
                            .font(.headline)
                            .foregroundColor(.white)
                    )

                VStack(alignment: .leading, spacing: 2) {
                    Text(item.user.displayName)
                        .font(.subheadline)
                        .fontWeight(.semibold)
                        .foregroundColor(AppColors.textPrimary)

                    HStack(spacing: 4) {
                        Image(systemName: "star.fill")
                            .font(.caption2)
                            .foregroundColor(AppColors.warning)
                        Text("Lv.\(item.user.level)")
                            .font(.caption)
                            .foregroundColor(AppColors.textSecondary)

                        Text("・")
                            .foregroundColor(AppColors.textTertiary)

                        Text(timeAgo(from: item.answer.createdAt))
                            .font(.caption)
                            .foregroundColor(AppColors.textTertiary)
                    }
                }

                Spacer()
            }
        }
        .buttonStyle(.plain)
    }

    private var challengeSection: some View {
        HStack(spacing: 8) {
            Image(systemName: "quote.opening")
                .font(.caption)
                .foregroundColor(AppColors.primaryGradientStart)

            Text(item.challenge.prompt)
                .font(.caption)
                .foregroundColor(AppColors.textSecondary)
                .lineLimit(1)
        }
        .padding(10)
        .background(Color(.systemGray6))
        .cornerRadius(10)
    }

    private var answerSection: some View {
        Text(item.answer.content)
            .font(.body)
            .foregroundColor(AppColors.textPrimary)
            .lineLimit(4)
            .fixedSize(horizontal: false, vertical: true)
    }

    private var scoreSection: some View {
        Group {
            if let score = item.answer.score {
                HStack(spacing: 8) {
                    Image(systemName: "checkmark.seal.fill")
                        .font(.caption)
                        .foregroundColor(scoreColor(score))

                    Text("\(score)点")
                        .font(.caption)
                        .fontWeight(.medium)
                        .foregroundColor(scoreColor(score))
                }
                .padding(.horizontal, 10)
                .padding(.vertical, 6)
                .background(scoreColor(score).opacity(0.1))
                .cornerRadius(8)
            }
        }
    }

    private var actionsSection: some View {
        HStack(spacing: 24) {
            Button(action: onLike) {
                HStack(spacing: 6) {
                    Image(systemName: item.isLiked ? "heart.fill" : "heart")
                        .font(.system(size: 18))
                        .foregroundColor(item.isLiked ? AppColors.error : AppColors.textSecondary)

                    Text("\(item.answer.likeCount)")
                        .font(.subheadline)
                        .foregroundColor(AppColors.textSecondary)
                }
            }

            Button(action: onComment) {
                HStack(spacing: 6) {
                    Image(systemName: "bubble.left")
                        .font(.system(size: 18))
                        .foregroundColor(AppColors.textSecondary)

                    Text("\(item.answer.commentCount)")
                        .font(.subheadline)
                        .foregroundColor(AppColors.textSecondary)
                }
            }

            Spacer()

            Button(action: {}) {
                Image(systemName: "square.and.arrow.up")
                    .font(.system(size: 18))
                    .foregroundColor(AppColors.textSecondary)
            }
        }
    }

    private func scoreColor(_ score: Int) -> Color {
        switch score {
        case 90...100: return AppColors.success
        case 70..<90: return AppColors.primaryGradientStart
        default: return AppColors.warning
        }
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
    FeedItemView(
        item: FeedItem(
            id: UUID(),
            answer: Answer.mock,
            challenge: Challenge.mock,
            user: User.mock,
            isLiked: false
        ),
        onLike: {},
        onComment: {},
        onUserTap: {}
    )
    .padding()
    .background(Color(.systemGray6))
}
