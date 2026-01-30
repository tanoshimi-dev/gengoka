//
//  ChallengeViewModel.swift
//  Gengoka
//

import Foundation

@Observable
final class ChallengeViewModel {
    var challenge: Challenge?
    var answerText = ""
    var isPublic = true
    var isLoading = false
    var isSubmitting = false
    var error: Error?
    var result: AnswerResult?

    private let apiClient = APIClient.shared

    var characterCount: Int {
        answerText.count
    }

    var isValidLength: Bool {
        guard let challenge = challenge else { return false }
        return characterCount >= challenge.minCharacters && characterCount <= challenge.maxCharacters
    }

    var canSubmit: Bool {
        isValidLength && !isSubmitting && !answerText.isEmpty
    }

    var characterCountColor: CharacterCountColor {
        guard let challenge = challenge else { return .normal }
        if characterCount < challenge.minCharacters {
            return .warning
        } else if characterCount > challenge.maxCharacters {
            return .error
        }
        return .valid
    }

    enum CharacterCountColor {
        case normal, warning, valid, error
    }

    func loadChallenge(for category: Category) async {
        isLoading = true
        error = nil

        do {
            let challenges: [Challenge] = try await apiClient.request(.dailyChallenges)
            challenge = challenges.first { $0.categoryId == category.id } ?? challenges.first
        } catch {
            self.error = error
            // Use mock for development
            challenge = Challenge.mock
        }

        isLoading = false
    }

    func loadChallenge(id: UUID) async {
        isLoading = true
        error = nil

        do {
            challenge = try await apiClient.request(.challenge(id: id))
        } catch {
            self.error = error
            challenge = Challenge.mock
        }

        isLoading = false
    }

    func submitAnswer() async {
        guard let challenge = challenge, canSubmit else { return }

        isSubmitting = true
        error = nil

        do {
            let submission = AnswerSubmission(content: answerText, isPublic: isPublic)
            result = try await apiClient.request(.submitAnswer(challengeId: challenge.id), body: submission)
        } catch {
            self.error = error
            // Mock result for development
            result = AnswerResult(
                answer: Answer(
                    id: UUID(),
                    challengeId: challenge.id,
                    userId: AuthService.shared.userId,
                    content: answerText,
                    score: 85,
                    feedback: "素晴らしい回答です！文法も正確で、表現も自然です。",
                    isPublic: isPublic,
                    likeCount: 0,
                    commentCount: 0,
                    createdAt: Date()
                ),
                scoringDetails: ScoringDetails(
                    grammarScore: 90,
                    creativityScore: 80,
                    relevanceScore: 85,
                    overallScore: 85,
                    feedback: "素晴らしい回答です！文法も正確で、表現も自然です。",
                    improvements: ["より具体的な描写を加えると、さらに良くなります。"]
                )
            )
        }

        isSubmitting = false
    }

    func reset() {
        answerText = ""
        result = nil
        error = nil
    }
}
