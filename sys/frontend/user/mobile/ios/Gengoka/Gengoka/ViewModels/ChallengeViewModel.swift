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
            // The backend returns an array of DailyChallenge objects
            let dailyChallenges: [DailyChallenge] = try await apiClient.request(.dailyChallenges)
            
            #if DEBUG
            print("✅ Successfully decoded \(dailyChallenges.count) daily challenges")
            print("📋 Looking for category: \(category.id)")
            for dc in dailyChallenges {
                print("  - Challenge: \(dc.challenge.id), Category: \(dc.challenge.categoryId), Name: \(dc.categoryName)")
            }
            #endif
            
            // Try to find a challenge for the selected category
            challenge = dailyChallenges.first { $0.challenge.categoryId == category.id }?.challenge 
                ?? dailyChallenges.first?.challenge
            
            #if DEBUG
            if let challenge = challenge {
                print("✅ Selected challenge: \(challenge.prompt)")
            } else {
                print("⚠️ No challenge found")
            }
            #endif
        } catch {
            #if DEBUG
            print("❌ Failed to load challenge: \(error)")
            print("❌ Error details: \(error.localizedDescription)")
            #endif
            self.error = error
            challenge = nil
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
            challenge = nil
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
            result = nil
        }

        isSubmitting = false
    }

    func reset() {
        answerText = ""
        result = nil
        error = nil
    }
}
