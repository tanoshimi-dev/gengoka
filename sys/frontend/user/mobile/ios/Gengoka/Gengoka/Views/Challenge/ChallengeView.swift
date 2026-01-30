//
//  ChallengeView.swift
//  Gengoka
//

import SwiftUI

struct ChallengeView: View {
    let category: Category
    @State private var viewModel = ChallengeViewModel()
    @State private var showResult = false
    @Environment(\.dismiss) private var dismiss
    @FocusState private var isTextFieldFocused: Bool

    var body: some View {
        ZStack {
            AppColors.backgroundGradient.ignoresSafeArea()

            ScrollView {
                VStack(spacing: 24) {
                    if let challenge = viewModel.challenge {
                        promptSection(challenge)
                        inputSection(challenge)
                        optionsSection
                        submitSection
                    }
                }
                .padding()
            }
        }
        .navigationBarTitleDisplayMode(.inline)
        .navigationBarBackButtonHidden(true)
        .toolbar {
            ToolbarItem(placement: .navigationBarLeading) {
                BackButton()
            }
            ToolbarItem(placement: .principal) {
                Text(category.name)
                    .font(.headline)
            }
        }
        .overlay {
            if viewModel.isLoading {
                LoadingView(message: "チャレンジを読み込み中...")
            }
            if viewModel.isSubmitting {
                LoadingOverlay()
            }
        }
        .navigationDestination(isPresented: $showResult) {
            if let result = viewModel.result {
                ResultView(result: result, challenge: viewModel.challenge!)
            }
        }
        .task {
            await viewModel.loadChallenge(for: category)
        }
        .onChange(of: viewModel.result) { _, newValue in
            if newValue != nil {
                showResult = true
            }
        }
    }

    private func promptSection(_ challenge: Challenge) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("お題")
                    .font(.caption)
                    .fontWeight(.medium)
                    .foregroundColor(.white)
                    .padding(.horizontal, 12)
                    .padding(.vertical, 6)
                    .background(AppColors.primaryGradient)
                    .cornerRadius(12)

                Spacer()

                difficultyBadge(challenge.difficulty)
            }

            Text(challenge.prompt)
                .font(.title3)
                .fontWeight(.semibold)
                .foregroundColor(AppColors.textPrimary)
                .fixedSize(horizontal: false, vertical: true)

            if let description = challenge.description {
                Text(description)
                    .font(.subheadline)
                    .foregroundColor(AppColors.textSecondary)
            }
        }
        .padding(20)
        .background(Color.white)
        .cornerRadius(20)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private func difficultyBadge(_ difficulty: Challenge.Difficulty) -> some View {
        let (text, color): (String, Color) = {
            switch difficulty {
            case .easy: return ("やさしい", AppColors.success)
            case .medium: return ("ふつう", AppColors.warning)
            case .hard: return ("むずかしい", AppColors.error)
            }
        }()

        return Text(text)
            .font(.caption)
            .fontWeight(.medium)
            .foregroundColor(color)
            .padding(.horizontal, 10)
            .padding(.vertical, 4)
            .background(color.opacity(0.15))
            .cornerRadius(8)
    }

    private func inputSection(_ challenge: Challenge) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            TextEditor(text: $viewModel.answerText)
                .focused($isTextFieldFocused)
                .frame(minHeight: 150)
                .padding(12)
                .background(Color(.systemGray6))
                .cornerRadius(12)
                .overlay(
                    Group {
                        if viewModel.answerText.isEmpty {
                            Text("ここに回答を入力してください...")
                                .foregroundColor(AppColors.textTertiary)
                                .padding(16)
                        }
                    },
                    alignment: .topLeading
                )

            HStack {
                characterCounter(challenge)

                Spacer()

                Text("\(challenge.minCharacters)〜\(challenge.maxCharacters)文字")
                    .font(.caption)
                    .foregroundColor(AppColors.textSecondary)
            }
        }
        .padding(20)
        .background(Color.white)
        .cornerRadius(20)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private func characterCounter(_ challenge: Challenge) -> some View {
        let color: Color = {
            switch viewModel.characterCountColor {
            case .normal: return AppColors.textSecondary
            case .warning: return AppColors.warning
            case .valid: return AppColors.success
            case .error: return AppColors.error
            }
        }()

        return HStack(spacing: 4) {
            Text("\(viewModel.characterCount)")
                .font(.headline)
                .fontWeight(.bold)
                .foregroundColor(color)

            Text("文字")
                .font(.caption)
                .foregroundColor(color)
        }
    }

    private var optionsSection: some View {
        HStack {
            Toggle(isOn: $viewModel.isPublic) {
                HStack(spacing: 8) {
                    Image(systemName: viewModel.isPublic ? "globe" : "lock.fill")
                        .foregroundColor(AppColors.primaryGradientStart)

                    Text(viewModel.isPublic ? "みんなに公開" : "非公開")
                        .font(.subheadline)
                        .foregroundColor(AppColors.textPrimary)
                }
            }
            .toggleStyle(SwitchToggleStyle(tint: AppColors.primaryGradientStart))
        }
        .padding(16)
        .background(Color.white)
        .cornerRadius(16)
        .shadow(color: .black.opacity(0.05), radius: 8, x: 0, y: 4)
    }

    private var submitSection: some View {
        GradientButton(
            title: "回答を送信",
            action: {
                isTextFieldFocused = false
                Task {
                    await viewModel.submitAnswer()
                }
            },
            isEnabled: viewModel.canSubmit,
            isLoading: viewModel.isSubmitting
        )
    }
}

#Preview {
    NavigationStack {
        ChallengeView(category: Category.mockCategories[0])
    }
}
