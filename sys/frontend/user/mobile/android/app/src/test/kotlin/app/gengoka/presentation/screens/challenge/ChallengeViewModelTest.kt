package app.gengoka.presentation.screens.challenge

import androidx.lifecycle.SavedStateHandle
import app.gengoka.core.util.Resource
import app.gengoka.domain.repository.AnswerRepository
import app.gengoka.domain.repository.ChallengeRepository
import app.gengoka.testutil.MainDispatcherRule
import app.gengoka.testutil.TestDataFactory
import io.mockk.coEvery
import io.mockk.coVerify
import io.mockk.mockk
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.advanceUntilIdle
import kotlinx.coroutines.test.runTest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class ChallengeViewModelTest {

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private val challengeRepository = mockk<ChallengeRepository>()
    private val answerRepository = mockk<AnswerRepository>()

    private fun createViewModel(
        categoryId: String = "cat-1",
        categoryName: String = "状況描写"
    ): ChallengeViewModel {
        val savedStateHandle = SavedStateHandle(
            mapOf("categoryId" to categoryId, "categoryName" to categoryName)
        )
        return ChallengeViewModel(savedStateHandle, challengeRepository, answerRepository)
    }

    @Test
    fun `init loads challenge matching categoryId`() = runTest {
        val challenges = listOf(
            TestDataFactory.createChallengeWithCategory(id = "ch-1", categoryId = "cat-1"),
            TestDataFactory.createChallengeWithCategory(id = "ch-2", categoryId = "cat-2")
        )
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(challenges)

        val viewModel = createViewModel(categoryId = "cat-2")
        advanceUntilIdle()

        assertEquals("ch-2", viewModel.uiState.value.challenge?.id)
        assertFalse(viewModel.uiState.value.isLoading)
    }

    @Test
    fun `init falls back to first challenge when categoryId not found`() = runTest {
        val challenges = listOf(
            TestDataFactory.createChallengeWithCategory(id = "ch-1", categoryId = "cat-1")
        )
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(challenges)

        val viewModel = createViewModel(categoryId = "nonexistent")
        advanceUntilIdle()

        assertEquals("ch-1", viewModel.uiState.value.challenge?.id)
    }

    @Test
    fun `init sets error on load failure`() = runTest {
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Error("Network error")

        val viewModel = createViewModel()
        advanceUntilIdle()

        assertEquals("Network error", viewModel.uiState.value.error)
        assertFalse(viewModel.uiState.value.isLoading)
    }

    @Test
    fun `updateAnswerText updates text in state`() = runTest {
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.updateAnswerText("新しい回答")
        assertEquals("新しい回答", viewModel.uiState.value.answerText)
    }

    @Test
    fun `submitAnswer succeeds with valid content`() = runTest {
        val challenge = TestDataFactory.createChallengeWithCategory(charLimit = 200)
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(listOf(challenge))
        val answerWithUser = TestDataFactory.createAnswerWithUser()
        coEvery { answerRepository.createAnswer("ch-1", "テスト回答") } returns Resource.Success(answerWithUser)

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.updateAnswerText("テスト回答")
        viewModel.submitAnswer()
        advanceUntilIdle()

        assertNotNull(viewModel.uiState.value.submittedAnswer)
        assertFalse(viewModel.uiState.value.isSubmitting)
    }

    @Test
    fun `submitAnswer does nothing with empty content`() = runTest {
        val challenge = TestDataFactory.createChallengeWithCategory()
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(listOf(challenge))

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.updateAnswerText("")
        viewModel.submitAnswer()
        advanceUntilIdle()

        assertNull(viewModel.uiState.value.submittedAnswer)
        coVerify(exactly = 0) { answerRepository.createAnswer(any(), any()) }
    }

    @Test
    fun `submitAnswer does nothing when content exceeds charLimit`() = runTest {
        val challenge = TestDataFactory.createChallengeWithCategory(charLimit = 5)
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(listOf(challenge))

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.updateAnswerText("123456") // exceeds limit of 5
        viewModel.submitAnswer()
        advanceUntilIdle()

        assertNull(viewModel.uiState.value.submittedAnswer)
        coVerify(exactly = 0) { answerRepository.createAnswer(any(), any()) }
    }

    @Test
    fun `submitAnswer error sets error message`() = runTest {
        val challenge = TestDataFactory.createChallengeWithCategory(charLimit = 200)
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(listOf(challenge))
        coEvery { answerRepository.createAnswer(any(), any()) } returns Resource.Error("送信エラー")

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.updateAnswerText("テスト回答")
        viewModel.submitAnswer()
        advanceUntilIdle()

        assertEquals("送信エラー", viewModel.uiState.value.error)
        assertNull(viewModel.uiState.value.submittedAnswer)
    }

    @Test
    fun `retryChallenge resets answer and reloads`() = runTest {
        val challenge = TestDataFactory.createChallengeWithCategory()
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(listOf(challenge))

        val viewModel = createViewModel()
        advanceUntilIdle()
        viewModel.updateAnswerText("some text")

        viewModel.retryChallenge()
        advanceUntilIdle()

        assertEquals("", viewModel.uiState.value.answerText)
        assertNull(viewModel.uiState.value.submittedAnswer)
    }

    @Test
    fun `clearError removes error message`() = runTest {
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Error("Error")

        val viewModel = createViewModel()
        advanceUntilIdle()
        assertEquals("Error", viewModel.uiState.value.error)

        viewModel.clearError()
        assertNull(viewModel.uiState.value.error)
    }
}
