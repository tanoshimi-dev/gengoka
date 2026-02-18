package app.gengoka.presentation.screens.feed

import app.gengoka.core.util.Resource
import app.gengoka.domain.repository.AnswerRepository
import app.gengoka.domain.repository.CategoryRepository
import app.gengoka.domain.repository.FeedRepository
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
class FeedViewModelTest {

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private val feedRepository = mockk<FeedRepository>()
    private val categoryRepository = mockk<CategoryRepository>()
    private val answerRepository = mockk<AnswerRepository>()

    private fun createViewModel(): FeedViewModel {
        return FeedViewModel(feedRepository, categoryRepository, answerRepository)
    }

    @Test
    fun `init loads feed and categories`() = runTest {
        val feedItems = TestDataFactory.createFeedItems(3)
        val categories = TestDataFactory.createCategories(2)
        coEvery { feedRepository.getFeed(page = 1, pageSize = 20, filter = null, categoryId = null) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(categories)

        val viewModel = createViewModel()
        advanceUntilIdle()

        val state = viewModel.uiState.value
        assertEquals(3, state.items.size)
        assertEquals(2, state.categories.size)
        assertFalse(state.isLoading)
        assertEquals(2, state.page)
    }

    @Test
    fun `init handles feed error`() = runTest {
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Error("Network error")
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        assertEquals("Network error", viewModel.uiState.value.error)
        assertFalse(viewModel.uiState.value.isLoading)
    }

    @Test
    fun `hasMore is false when fewer than 20 items returned`() = runTest {
        val feedItems = TestDataFactory.createFeedItems(5)
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        assertFalse(viewModel.uiState.value.hasMore)
    }

    @Test
    fun `hasMore is true when 20 items returned`() = runTest {
        val feedItems = (1..20).map { TestDataFactory.createAnswerWithDetails(id = "ans-$it") }
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        assertTrue(viewModel.uiState.value.hasMore)
    }

    @Test
    fun `selectFilter resets page and reloads with new filter`() = runTest {
        val feedItems = TestDataFactory.createFeedItems(3)
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.selectFilter("following")
        advanceUntilIdle()

        assertEquals("following", viewModel.uiState.value.selectedFilter)
        coVerify { feedRepository.getFeed(page = 1, pageSize = 20, filter = "following", categoryId = null) }
    }

    @Test
    fun `selectFilter with same filter does nothing`() = runTest {
        val feedItems = TestDataFactory.createFeedItems(3)
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.selectFilter("all") // same as default
        advanceUntilIdle()

        // Only 1 getFeed call from init, not 2
        coVerify(exactly = 1) { feedRepository.getFeed(page = 1, pageSize = 20, filter = null, categoryId = null) }
    }

    @Test
    fun `selectFilter with category passes categoryId`() = runTest {
        val feedItems = TestDataFactory.createFeedItems(3)
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.selectFilter("category_cat-1")
        advanceUntilIdle()

        coVerify { feedRepository.getFeed(page = 1, pageSize = 20, filter = null, categoryId = "cat-1") }
    }

    @Test
    fun `toggleLike optimistically updates liked state`() = runTest {
        val feedItems = listOf(
            TestDataFactory.createAnswerWithDetails(id = "ans-1", isLiked = false, likeCount = 5)
        )
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())
        coEvery { answerRepository.likeAnswer("ans-1") } returns Resource.Success(Unit)

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.toggleLike("ans-1")
        advanceUntilIdle()

        val item = viewModel.uiState.value.items.first()
        assertTrue(item.isLiked)
        assertEquals(6, item.likeCount)
    }

    @Test
    fun `toggleLike reverts on API error`() = runTest {
        val feedItems = listOf(
            TestDataFactory.createAnswerWithDetails(id = "ans-1", isLiked = false, likeCount = 5)
        )
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())
        coEvery { answerRepository.likeAnswer("ans-1") } returns Resource.Error("Network error")

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.toggleLike("ans-1")
        advanceUntilIdle()

        val item = viewModel.uiState.value.items.first()
        assertFalse(item.isLiked)
        assertEquals(5, item.likeCount)
    }

    @Test
    fun `toggleLike unlike decrements count`() = runTest {
        val feedItems = listOf(
            TestDataFactory.createAnswerWithDetails(id = "ans-1", isLiked = true, likeCount = 5)
        )
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Success(feedItems)
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())
        coEvery { answerRepository.unlikeAnswer("ans-1") } returns Resource.Success(Unit)

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.toggleLike("ans-1")
        advanceUntilIdle()

        val item = viewModel.uiState.value.items.first()
        assertFalse(item.isLiked)
        assertEquals(4, item.likeCount)
    }

    @Test
    fun `clearError removes error message`() = runTest {
        coEvery { feedRepository.getFeed(any(), any(), any(), any()) } returns Resource.Error("Error")
        coEvery { categoryRepository.getCategories() } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()
        assertEquals("Error", viewModel.uiState.value.error)

        viewModel.clearError()
        assertNull(viewModel.uiState.value.error)
    }
}
