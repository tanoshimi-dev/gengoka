package app.gengoka.presentation.screens.home

import app.gengoka.core.util.Resource
import app.gengoka.domain.repository.CategoryRepository
import app.gengoka.domain.repository.ChallengeRepository
import app.gengoka.domain.repository.UserRepository
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
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test

@OptIn(ExperimentalCoroutinesApi::class)
class HomeViewModelTest {

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private val categoryRepository = mockk<CategoryRepository>()
    private val challengeRepository = mockk<ChallengeRepository>()
    private val userRepository = mockk<UserRepository>()

    private fun createViewModel(): HomeViewModel {
        return HomeViewModel(categoryRepository, challengeRepository, userRepository)
    }

    @Test
    fun `init loads all data successfully`() = runTest {
        val categories = TestDataFactory.createCategories()
        val challenges = TestDataFactory.createDailyChallenges()
        val stats = TestDataFactory.createUserStats()
        coEvery { categoryRepository.getCategories() } returns Resource.Success(categories)
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(challenges)
        coEvery { userRepository.getUserStats() } returns Resource.Success(stats)

        val viewModel = createViewModel()
        advanceUntilIdle()

        val state = viewModel.uiState.value
        assertFalse(state.isLoading)
        assertEquals(categories, state.categories)
        assertEquals(challenges, state.dailyChallenges)
        assertEquals(stats, state.userStats)
        assertNull(state.error)
    }

    @Test
    fun `init handles category error`() = runTest {
        coEvery { categoryRepository.getCategories() } returns Resource.Error("Network error")
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(emptyList())
        coEvery { userRepository.getUserStats() } returns Resource.Success(TestDataFactory.createUserStats())

        val viewModel = createViewModel()
        advanceUntilIdle()

        assertEquals("Network error", viewModel.uiState.value.error)
        assertFalse(viewModel.uiState.value.isLoading)
    }

    @Test
    fun `incompleteCount returns count of uncompleted challenges`() = runTest {
        val challenges = TestDataFactory.createDailyChallenges(3) // first is completed, rest aren't
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(challenges)
        coEvery { userRepository.getUserStats() } returns Resource.Success(TestDataFactory.createUserStats())

        val viewModel = createViewModel()
        advanceUntilIdle()

        assertEquals(2, viewModel.uiState.value.incompleteCount)
    }

    @Test
    fun `refresh reloads all data`() = runTest {
        val initialCategories = TestDataFactory.createCategories(2)
        val updatedCategories = TestDataFactory.createCategories(3)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(initialCategories) andThen Resource.Success(updatedCategories)
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(emptyList())
        coEvery { userRepository.getUserStats() } returns Resource.Success(TestDataFactory.createUserStats())

        val viewModel = createViewModel()
        advanceUntilIdle()
        assertEquals(2, viewModel.uiState.value.categories.size)

        viewModel.refresh()
        advanceUntilIdle()

        assertEquals(3, viewModel.uiState.value.categories.size)
        assertFalse(viewModel.uiState.value.isRefreshing)
    }

    @Test
    fun `clearError removes error message`() = runTest {
        coEvery { categoryRepository.getCategories() } returns Resource.Error("Error")
        coEvery { challengeRepository.getDailyChallenges() } returns Resource.Success(emptyList())
        coEvery { userRepository.getUserStats() } returns Resource.Success(TestDataFactory.createUserStats())

        val viewModel = createViewModel()
        advanceUntilIdle()
        assertEquals("Error", viewModel.uiState.value.error)

        viewModel.clearError()
        assertNull(viewModel.uiState.value.error)
    }
}
