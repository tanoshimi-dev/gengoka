//
//  ChallengeViewModelTests.swift
//  GengokaTess
//

import XCTest
@testable import Gengoka

final class ChallengeViewModelTests: XCTestCase {

    private var mockAPIClient: MockAPIClient!
    private var viewModel: ChallengeViewModel!

    override func setUp() async throws {
        mockAPIClient = MockAPIClient()
        viewModel = ChallengeViewModel(apiClient: mockAPIClient)
    }

    // MARK: - Character Count

    func testCharacterCount() {
        viewModel.answerText = "テスト回答"
        XCTAssertEqual(viewModel.characterCount, 5)
    }

    // MARK: - Valid Length

    func testIsValidLengthWithinRange() {
        let categoryId = UUID()
        viewModel.challenge = Challenge(
            categoryId: categoryId,
            prompt: "テスト",
            minCharacters: 5,
            maxCharacters: 50,
            difficulty: .easy
        )
        viewModel.answerText = "これはテスト回答です"  // 9 characters
        XCTAssertTrue(viewModel.isValidLength)
    }

    func testIsValidLengthBelowMin() {
        let categoryId = UUID()
        viewModel.challenge = Challenge(
            categoryId: categoryId,
            prompt: "テスト",
            minCharacters: 10,
            maxCharacters: 50,
            difficulty: .easy
        )
        viewModel.answerText = "短い"  // 2 characters
        XCTAssertFalse(viewModel.isValidLength)
    }

    func testIsValidLengthAboveMax() {
        let categoryId = UUID()
        viewModel.challenge = Challenge(
            categoryId: categoryId,
            prompt: "テスト",
            minCharacters: 0,
            maxCharacters: 5,
            difficulty: .easy
        )
        viewModel.answerText = "これは長すぎる回答です"  // 10 characters
        XCTAssertFalse(viewModel.isValidLength)
    }

    // MARK: - Can Submit

    func testCanSubmitTrue() {
        let categoryId = UUID()
        viewModel.challenge = Challenge(
            categoryId: categoryId,
            prompt: "テスト",
            minCharacters: 0,
            maxCharacters: 50,
            difficulty: .easy
        )
        viewModel.answerText = "有効な回答"
        viewModel.existingAnswer = nil
        XCTAssertTrue(viewModel.canSubmit)
    }

    func testCanSubmitFalseWhenAlreadyAnswered() {
        let categoryId = UUID()
        viewModel.challenge = Challenge(
            categoryId: categoryId,
            prompt: "テスト",
            minCharacters: 0,
            maxCharacters: 50,
            difficulty: .easy
        )
        viewModel.answerText = "有効な回答"
        viewModel.existingAnswer = Answer(
            id: UUID(),
            challengeId: categoryId,
            userId: UUID(),
            content: "既存の回答",
            createdAt: Date()
        )
        XCTAssertFalse(viewModel.canSubmit)
    }

    // MARK: - Character Count Color

    func testCharacterCountColorThresholds() {
        let categoryId = UUID()
        viewModel.challenge = Challenge(
            categoryId: categoryId,
            prompt: "テスト",
            minCharacters: 5,
            maxCharacters: 10,
            difficulty: .easy
        )

        // Below min → warning
        viewModel.answerText = "短い"
        XCTAssertEqual(viewModel.characterCountColor, .warning)

        // Within range → valid
        viewModel.answerText = "ちょうど良い長さ"
        XCTAssertEqual(viewModel.characterCountColor, .valid)

        // Above max → error
        viewModel.answerText = "これは非常に長すぎる回答テキストです"
        XCTAssertEqual(viewModel.characterCountColor, .error)
    }

    // MARK: - Submit Answer

    func testSubmitAnswerSuccess() async {
        let challengeId = UUID()
        let answerId = UUID()
        let challenge = Challenge(
            id: challengeId,
            categoryId: UUID(),
            prompt: "テスト",
            minCharacters: 0,
            maxCharacters: 100,
            difficulty: .easy
        )
        viewModel.challenge = challenge
        viewModel.answerText = "テスト回答"

        let mockAnswer = Answer(
            id: answerId,
            challengeId: challengeId,
            userId: UUID(),
            content: "テスト回答",
            score: 85,
            createdAt: Date()
        )
        let mockResult = AnswerResult(answer: mockAnswer, scoringDetails: nil)

        await mockAPIClient.setResponse(
            mockResult,
            for: "/challenges/\(challengeId.uuidString)/answers"
        )

        // Act
        await viewModel.submitAnswer()

        // Assert
        XCTAssertNotNil(viewModel.result)
        XCTAssertEqual(viewModel.result?.answer.id, answerId)
        XCTAssertFalse(viewModel.isSubmitting)
    }
}
