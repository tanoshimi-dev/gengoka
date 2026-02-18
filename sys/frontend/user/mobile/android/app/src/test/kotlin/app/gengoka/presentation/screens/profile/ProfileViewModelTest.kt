package app.gengoka.presentation.screens.profile

import androidx.lifecycle.SavedStateHandle
import app.gengoka.core.util.Resource
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
class ProfileViewModelTest {

    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private val userRepository = mockk<UserRepository>()

    private fun createViewModel(userId: String = "user-1"): ProfileViewModel {
        val savedStateHandle = SavedStateHandle(mapOf("userId" to userId))
        return ProfileViewModel(savedStateHandle, userRepository)
    }

    @Test
    fun `init loads profile and answers`() = runTest {
        val profile = TestDataFactory.createUserProfile()
        val answers = TestDataFactory.createFeedItems(3)
        coEvery { userRepository.getUser("user-1") } returns Resource.Success(profile)
        coEvery { userRepository.getUserAnswers("user-1") } returns Resource.Success(answers)

        val viewModel = createViewModel()
        advanceUntilIdle()

        val state = viewModel.uiState.value
        assertEquals(profile, state.profile)
        assertEquals(3, state.answers.size)
        assertFalse(state.isLoading)
    }

    @Test
    fun `init handles profile error`() = runTest {
        coEvery { userRepository.getUser("user-1") } returns Resource.Error("User not found")
        coEvery { userRepository.getUserAnswers("user-1") } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        assertEquals("User not found", viewModel.uiState.value.error)
        assertNull(viewModel.uiState.value.profile)
    }

    @Test
    fun `toggleFollow optimistically updates follow state`() = runTest {
        val profile = TestDataFactory.createUserProfile(isFollowing = false, followerCount = 100)
        coEvery { userRepository.getUser("user-1") } returns Resource.Success(profile)
        coEvery { userRepository.getUserAnswers("user-1") } returns Resource.Success(emptyList())
        coEvery { userRepository.followUser("user-1") } returns Resource.Success(Unit)

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.toggleFollow()
        advanceUntilIdle()

        val updatedProfile = viewModel.uiState.value.profile!!
        assertTrue(updatedProfile.isFollowing)
        assertEquals(101, updatedProfile.followerCount)
    }

    @Test
    fun `toggleFollow unfollow decrements count`() = runTest {
        val profile = TestDataFactory.createUserProfile(isFollowing = true, followerCount = 100)
        coEvery { userRepository.getUser("user-1") } returns Resource.Success(profile)
        coEvery { userRepository.getUserAnswers("user-1") } returns Resource.Success(emptyList())
        coEvery { userRepository.unfollowUser("user-1") } returns Resource.Success(Unit)

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.toggleFollow()
        advanceUntilIdle()

        val updatedProfile = viewModel.uiState.value.profile!!
        assertFalse(updatedProfile.isFollowing)
        assertEquals(99, updatedProfile.followerCount)
    }

    @Test
    fun `toggleFollow reverts on API error`() = runTest {
        val profile = TestDataFactory.createUserProfile(isFollowing = false, followerCount = 100)
        coEvery { userRepository.getUser("user-1") } returns Resource.Success(profile)
        coEvery { userRepository.getUserAnswers("user-1") } returns Resource.Success(emptyList())
        coEvery { userRepository.followUser("user-1") } returns Resource.Error("Network error")

        val viewModel = createViewModel()
        advanceUntilIdle()

        viewModel.toggleFollow()
        advanceUntilIdle()

        val revertedProfile = viewModel.uiState.value.profile!!
        assertFalse(revertedProfile.isFollowing)
        assertEquals(100, revertedProfile.followerCount)
    }

    @Test
    fun `selectTab updates selected tab`() = runTest {
        coEvery { userRepository.getUser("user-1") } returns Resource.Success(TestDataFactory.createUserProfile())
        coEvery { userRepository.getUserAnswers("user-1") } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()

        assertEquals(0, viewModel.uiState.value.selectedTab)
        viewModel.selectTab(1)
        assertEquals(1, viewModel.uiState.value.selectedTab)
    }

    @Test
    fun `clearError removes error message`() = runTest {
        coEvery { userRepository.getUser("user-1") } returns Resource.Error("Error")
        coEvery { userRepository.getUserAnswers("user-1") } returns Resource.Success(emptyList())

        val viewModel = createViewModel()
        advanceUntilIdle()
        assertEquals("Error", viewModel.uiState.value.error)

        viewModel.clearError()
        assertNull(viewModel.uiState.value.error)
    }
}
