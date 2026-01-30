//
//  ResultView.swift
//  Gengoka
//

import SwiftUI

struct ResultView: View {
    let result: AnswerResult
    let challenge: Challenge
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                scoreSection

                feedbackSection

                if let details = result.scoringDetails {
                    detailsSection(details)
                }

                answerSection

                actionsSection
            }
            .padding()
        }
        .background(AppColors.backgroundGradient.ignoresSafeArea())
        .navigationBarTitleDisplayMode(.inline)
        .navigationBarBackButtonHidden(true)
        .toolbar {
            ToolbarItem(placement: .navigationBarLeading) {
                Button(action: { navigateToHome() }) {
                    Image(systemName: "xmark")
                        .font(.system(size: 16, weight: .semibold))
                        .foregroundColor(AppColors.textPrimary)
                        .frame(width: 44, height: 44)
                        .background(Color.white.opacity(0.9))
                        .clipShape(Circle())
                        .shadow(color: .black.opacity(0.1), radius: 4, x: 0, y: 2)
                }
            }
            ToolbarItem(placement: .principal) {
                Text("結果")
                    .font(.headline)
            }
        }
    }

    private var scoreSection: some View {
        VStack(spacing: 16) {
            ZStack {
                Circle()
                    .stroke(Color(.systemGray5), lineWidth: 12)
                    .frame(width: 140, height: 140)

                Circle()
                    .trim(from: 0, to: CGFloat(result.answer.score ?? 0) / 100)
                    .stroke(
                        AppColors.primaryGradient,
                        style: StrokeStyle(lineWidth: 12, lineCap: .round)
                    )
                    .frame(width: 140, height: 140)
                    .rotationEffect(.degrees(-90))

                VStack(spacing: 4) {
                    Text("\(result.answer.score ?? 0)")
                        .font(.system(size: 48, weight: .bold))
                        .foregroundColor(AppColors.textPrimary)

                    Text("点")
                        .font(.subheadline)
                        .foregroundColor(AppColors.textSecondary)
                }
            }

            Text(scoreMessage)
                .font(.title3)
                .fontWeight(.semibold)
                .foregroundColor(AppColors.textPrimary)
        }
        .padding(24)
        .frame(maxWidth: .infinity)
        .background(Color.white)
        .cornerRadius(24)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private var scoreMessage: String {
        let score = result.answer.score ?? 0
        switch score {
        case 90...100: return "素晴らしい！完璧です！"
        case 80..<90: return "とても良くできました！"
        case 70..<80: return "良い回答です！"
        case 60..<70: return "もう少しで良くなります！"
        default: return "がんばりましょう！"
        }
    }

    private var feedbackSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Image(systemName: "bubble.left.fill")
                    .foregroundColor(AppColors.primaryGradientStart)
                Text("フィードバック")
                    .font(.headline)
                    .foregroundColor(AppColors.textPrimary)
            }

            Text(result.answer.feedback ?? result.scoringDetails?.feedback ?? "フィードバックはありません")
                .font(.body)
                .foregroundColor(AppColors.textSecondary)
                .fixedSize(horizontal: false, vertical: true)
        }
        .padding(20)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.white)
        .cornerRadius(20)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private func detailsSection(_ details: ScoringDetails) -> some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("採点詳細")
                .font(.headline)
                .foregroundColor(AppColors.textPrimary)

            VStack(spacing: 12) {
                ScoreBar(label: "文法", score: details.grammarScore, color: AppColors.primaryGradientStart)
                ScoreBar(label: "創造性", score: details.creativityScore, color: AppColors.secondaryGradient)
                ScoreBar(label: "関連性", score: details.relevanceScore, color: AppColors.success)
            }

            if let improvements = details.improvements, !improvements.isEmpty {
                VStack(alignment: .leading, spacing: 8) {
                    Text("改善ポイント")
                        .font(.subheadline)
                        .fontWeight(.medium)
                        .foregroundColor(AppColors.textPrimary)

                    ForEach(improvements, id: \.self) { improvement in
                        HStack(alignment: .top, spacing: 8) {
                            Image(systemName: "lightbulb.fill")
                                .font(.caption)
                                .foregroundColor(AppColors.warning)
                            Text(improvement)
                                .font(.caption)
                                .foregroundColor(AppColors.textSecondary)
                        }
                    }
                }
                .padding(.top, 8)
            }
        }
        .padding(20)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.white)
        .cornerRadius(20)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private var answerSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Image(systemName: "doc.text.fill")
                    .foregroundColor(AppColors.primaryGradientStart)
                Text("あなたの回答")
                    .font(.headline)
                    .foregroundColor(AppColors.textPrimary)
            }

            Text(result.answer.content)
                .font(.body)
                .foregroundColor(AppColors.textPrimary)
                .padding(16)
                .frame(maxWidth: .infinity, alignment: .leading)
                .background(Color(.systemGray6))
                .cornerRadius(12)
        }
        .padding(20)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.white)
        .cornerRadius(20)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private var actionsSection: some View {
        VStack(spacing: 12) {
            GradientButton(title: "次のチャレンジへ", action: {
                navigateToHome()
            })

            SecondaryButton(title: "結果をシェア", action: {
                shareResult()
            })
        }
    }

    private func navigateToHome() {
        dismiss()
        dismiss()
    }

    private func shareResult() {
        // Share functionality
    }
}

struct ScoreBar: View {
    let label: String
    let score: Int
    let color: LinearGradient

    init(label: String, score: Int, color: Color) {
        self.label = label
        self.score = score
        self.color = LinearGradient(colors: [color, color.opacity(0.7)], startPoint: .leading, endPoint: .trailing)
    }

    init(label: String, score: Int, color: LinearGradient) {
        self.label = label
        self.score = score
        self.color = color
    }

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text(label)
                    .font(.caption)
                    .foregroundColor(AppColors.textSecondary)
                Spacer()
                Text("\(score)点")
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(AppColors.textPrimary)
            }

            GeometryReader { geometry in
                ZStack(alignment: .leading) {
                    Rectangle()
                        .fill(Color(.systemGray5))
                        .frame(height: 8)
                        .cornerRadius(4)

                    Rectangle()
                        .fill(color)
                        .frame(width: geometry.size.width * CGFloat(score) / 100, height: 8)
                        .cornerRadius(4)
                }
            }
            .frame(height: 8)
        }
    }
}

#Preview {
    NavigationStack {
        ResultView(
            result: AnswerResult(
                answer: Answer.mock,
                scoringDetails: ScoringDetails(
                    grammarScore: 90,
                    creativityScore: 80,
                    relevanceScore: 85,
                    overallScore: 85,
                    feedback: "素晴らしい回答です！",
                    improvements: ["より具体的な描写を加えると良いです"]
                )
            ),
            challenge: Challenge.mock
        )
    }
}
