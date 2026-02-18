//
//  ChallengeViewModel.swift
//  Gengoka
//

import Foundation

@Observable
final class ChallengeViewModel {
    var challenge: Challenge?
    var existingAnswer: Answer?  // ← User's existing answer if already completed
    var answerText = ""
    var isPublic = true
    var isLoading = false
    var isSubmitting = false
    var error: Error?
    var result: AnswerResult?

    private let apiClient: any APIClientProtocol

    init(apiClient: any APIClientProtocol = APIClient.shared) {
        self.apiClient = apiClient
    }
    
    /// True if user has already answered this challenge
    var isAlreadyAnswered: Bool {
        existingAnswer != nil
    }

    var characterCount: Int {
        answerText.count
    }

    var isValidLength: Bool {
        guard let challenge = challenge else { return false }
        return characterCount >= challenge.minCharacters && characterCount <= challenge.maxCharacters
    }

    var canSubmit: Bool {
        !isAlreadyAnswered && isValidLength && !isSubmitting && !answerText.isEmpty
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
                print("  - Challenge: \(dc.challenge.id), Category: \(dc.challenge.categoryId), Name: \(dc.categoryName), Completed: \(dc.isCompleted)")
            }
            #endif
            
            // Try to find a challenge for the selected category
            if let dailyChallenge = dailyChallenges.first(where: { $0.challenge.categoryId == category.id }) 
                ?? dailyChallenges.first {
                challenge = dailyChallenge.challenge
                existingAnswer = dailyChallenge.userAnswer
                
                // Pre-fill the answer if already completed
                if let answer = dailyChallenge.userAnswer {
                    answerText = answer.content
                    isPublic = answer.isPublic
                }
                
                #if DEBUG
                if let challenge = challenge {
                    print("✅ Selected challenge:")
                    print("  📝 Prompt: \(challenge.prompt)")
                    print("  🆔 ID: \(challenge.id)")
                    print("  📁 Category ID: \(challenge.categoryId)")
                    print("  📏 Characters: \(challenge.minCharacters)-\(challenge.maxCharacters)")
                    print("  ⚡️ Difficulty: \(challenge.difficulty.rawValue)")
                    print("  📅 Created: \(challenge.createdAt)")
                    if let desc = challenge.description {
                        print("  📄 Description: \(desc)")
                    }
                    if let releaseDate = challenge.releaseDate {
                        print("  🗓️ Release Date: \(releaseDate)")
                    }
                    if let answerCount = challenge.answerCount {
                        print("  💬 Answer Count: \(answerCount)")
                    }
                    if let status = challenge.status {
                        print("  🔖 Status: \(status)")
                    }
                    
                    if let answer = existingAnswer {
                        print("⚠️ User has already answered this challenge (Answer ID: \(answer.id))")
                    }
                } else {
                    print("⚠️ No challenge found")
                }
                #endif
            }
        } catch {
            #if DEBUG
            print("❌ Failed to load challenge: \(error)")
            print("❌ Error details: \(error.localizedDescription)")
            #endif
            self.error = error
            challenge = nil
            existingAnswer = nil
        }

        isLoading = false
    }

    func loadChallenge(id: UUID) async {
        isLoading = true
        error = nil

        do {
            challenge = try await apiClient.request(.challenge(id: id))
            
            // Try to get user's existing answer for this challenge
            do {
                existingAnswer = try await apiClient.request(.challengeAnswer(challengeId: id))
                
                // Pre-fill if already answered
                if let answer = existingAnswer {
                    answerText = answer.content
                    isPublic = answer.isPublic
                    
                    #if DEBUG
                    print("⚠️ User has already answered this challenge (Answer ID: \(answer.id))")
                    #endif
                }
            } catch {
                // No answer found or endpoint doesn't exist - user hasn't answered
                existingAnswer = nil
                
                #if DEBUG
                print("ℹ️ No existing answer found for challenge \(id)")
                #endif
            }
        } catch {
            self.error = error
            challenge = nil
            existingAnswer = nil
        }

        isLoading = false
    }

    func submitAnswer() async {
        guard let challenge = challenge, canSubmit else { return }

        isSubmitting = true
        error = nil

        do {
            let submission = AnswerSubmission(content: answerText, isPublic: isPublic)
            
            #if DEBUG
            print("📤 Submitting answer for challenge: \(challenge.id)")
            print("   Content: \(answerText)")
            print("   Is Public: \(isPublic)")
            #endif
            
            let response: AnswerResult = try await apiClient.request(.submitAnswer(challengeId: challenge.id), body: submission)
            result = response
            
            #if DEBUG
            print("✅ Answer submitted successfully!")
            print("   Answer ID: \(response.answer.id)")
            if let scoring = response.scoringDetails {
                print("   Score: \(scoring.overallScore)")
            }
            #endif
        } catch {
            #if DEBUG
            print("❌ Failed to submit answer: \(error)")
            if let networkError = error as? NetworkError {
                print("   Network error: \(networkError.errorDescription ?? "Unknown")")
            }
            #endif
            self.error = error
            result = nil
        }

        isSubmitting = false
    }

    func reset() {
        answerText = ""
        result = nil
        error = nil
        existingAnswer = nil
    }
}
