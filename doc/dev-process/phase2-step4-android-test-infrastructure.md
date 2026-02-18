# Phase 2 Step 4: Android Test Infrastructure

## Summary
Established comprehensive test infrastructure for the Android app, creating 10 new test files with ~50 tests covering data layer, ViewModels, and Compose UI screens.

## Changes Made

### Build Configuration
- **`app/build.gradle.kts`**: Added test dependencies:
  - `io.mockk:mockk:1.13.9` - Kotlin-first mocking library
  - `kotlinx-coroutines-test:1.8.0` - Coroutine test utilities
  - `androidx.arch.core:core-testing:2.2.0` - Architecture component testing
  - `hilt-android-testing:2.50` + KSP compiler for instrumented tests

### Test Utilities (2 files)
| File | Purpose |
|------|---------|
| `testutil/MainDispatcherRule.kt` | JUnit rule replacing `Dispatchers.Main` with `StandardTestDispatcher` for ViewModel tests |
| `testutil/TestDataFactory.kt` | Factory functions for all domain models (Category, Challenge, Answer, User, etc.) |

### Unit Tests (6 files, ~42 tests)
| File | Tests | Coverage |
|------|-------|----------|
| `AuthRepositoryImplTest.kt` | 8 | Login/register/socialLogin token saving, refreshToken, logout clears tokens on API error |
| `AuthViewModelTest.kt` | 9 | Login/register flows, blank field validation, toggleMode, clearError, email trimming |
| `HomeViewModelTest.kt` | 5 | Multi-repo data loading, error handling, incompleteCount, refresh, clearError |
| `ChallengeViewModelTest.kt` | 9 | Challenge loading by categoryId, fallback, answer submission, charLimit validation, retry |
| `FeedViewModelTest.kt` | 10 | Feed loading, pagination (hasMore), filter selection, category filter, optimistic like toggle with revert |
| `ProfileViewModelTest.kt` | 7 | Profile + answers loading, optimistic follow/unfollow with revert, selectTab, clearError |

### Compose UI Tests (2 files, ~8 tests)
| File | Tests | Coverage |
|------|-------|----------|
| `LoginScreenTest.kt` | 4 | Element display, error message, button disabled state, navigation callback |
| `RegisterScreenTest.kt` | 4 | Element display, error message, button disabled state, navigation callback |

## Key Patterns Used

### ViewModel Test Pattern
All ViewModel tests follow the same structure:
1. `MainDispatcherRule` replaces `Dispatchers.Main`
2. MockK mocks for repository interfaces
3. Mocks configured **before** ViewModel construction (handles `init` block)
4. `advanceUntilIdle()` to process coroutines
5. Assert on `uiState.value`

### Optimistic Update Testing (Feed, Profile)
1. Set up initial state with known values
2. Trigger action (toggleLike/toggleFollow)
3. Verify immediate UI update (optimistic)
4. For error case: verify revert to original state

### Repository Test Pattern
1. Mock API responses with `coEvery`
2. Verify `Resource.Success`/`Resource.Error` mapping
3. Verify side effects (token saving/clearing)

## Test Counts
- **Unit tests**: ~42
- **UI tests**: ~8
- **Total**: ~50

## How to Run

### Unit Tests
```bash
cd sys/frontend/user/mobile/android
./gradlew test
```

Run a specific test class:
```bash
./gradlew test --tests "app.gengoka.presentation.screens.auth.AuthViewModelTest"
```

### Instrumented Tests (Compose UI)
Requires a connected device or running emulator:
```bash
cd sys/frontend/user/mobile/android
./gradlew connectedAndroidTest
```

Run a specific instrumented test class:
```bash
./gradlew connectedAndroidTest -Pandroid.testInstrumentationRunnerArguments.class=app.gengoka.presentation.screens.auth.LoginScreenTest
```

### Test Reports
- Unit test report: `app/build/reports/tests/testDebugUnitTest/index.html`
- Instrumented test report: `app/build/reports/androidTests/connected/index.html`
