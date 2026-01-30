//
//  StatsBarView.swift
//  Gengoka
//

import SwiftUI

struct StatsBarView: View {
    let stats: UserStats

    var body: some View {
        HStack(spacing: 0) {
            StatItem(
                icon: "flame.fill",
                value: "\(stats.currentStreak)",
                label: "連続",
                color: .orange
            )

            Divider()
                .frame(height: 40)

            StatItem(
                icon: "checkmark.circle.fill",
                value: "\(stats.completedToday)",
                label: "今日",
                color: AppColors.success
            )

            Divider()
                .frame(height: 40)

            StatItem(
                icon: "star.fill",
                value: String(format: "%.0f", stats.averageScore),
                label: "平均点",
                color: AppColors.warning
            )

            Divider()
                .frame(height: 40)

            StatItem(
                icon: "doc.text.fill",
                value: "\(stats.totalChallenges)",
                label: "総数",
                color: AppColors.primaryGradientStart
            )
        }
        .padding(.vertical, 16)
        .background(Color.white)
        .cornerRadius(16)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }
}

struct StatItem: View {
    let icon: String
    let value: String
    let label: String
    let color: Color

    var body: some View {
        VStack(spacing: 6) {
            HStack(spacing: 4) {
                Image(systemName: icon)
                    .font(.system(size: 14))
                    .foregroundColor(color)

                Text(value)
                    .font(.title3)
                    .fontWeight(.bold)
                    .foregroundColor(AppColors.textPrimary)
            }

            Text(label)
                .font(.caption2)
                .foregroundColor(AppColors.textSecondary)
        }
        .frame(maxWidth: .infinity)
    }
}

#Preview {
    StatsBarView(stats: UserStats.mock)
        .padding()
        .background(Color(.systemGray6))
}
