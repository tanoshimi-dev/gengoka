# Phase 2 Step 3: iOS Test Infrastructure

## Summary

Built iOS test infrastructure with protocol-based dependency injection and 39 unit tests covering models, ViewModels, and AuthService.

## Production Code Changes

### New Files (2)
- `Services/Protocols/APIClientProtocol.swift` — Protocol defining all APIClient methods (`Sendable`)
- `Services/Protocols/AuthServiceProtocol.swift` — Protocol defining all AuthService public API (`AnyObject`)

### Modified Files (7)
- `Services/APIClient.swift` — Added `APIClientProtocol` conformance (1 line)
- `Services/AuthService.swift` — Added `AuthServiceProtocol` conformance (1 line)
- `ViewModels/HomeViewModel.swift` — DI init with `any APIClientProtocol` (default: `APIClient.shared`)
- `ViewModels/ChallengeViewModel.swift` — DI init with `any APIClientProtocol`
- `ViewModels/FeedViewModel.swift` — DI init with `any APIClientProtocol`
- `ViewModels/ProfileViewModel.swift` — DI init with `any APIClientProtocol` + `any AuthServiceProtocol`; replaced `AuthService.shared` references with injected `authService`
- `Views/Profile/LinkedAccountsView.swift` — DI init for `LinkedAccountsViewModel`; replaced `APIClient.shared` method calls with injected `apiClient`

### Design Decisions
- Used `any Protocol` existential types (not generics) to preserve `@Observable` compatibility
- Default parameter values ensure zero breaking changes to existing code
- `MockAPIClient` is an `actor` to match `APIClient`'s `Sendable` conformance
- `MockAuthService` is a `class` to match `AuthService`'s `@Observable` pattern

## Test Files

### Test Target: `GengokaTess`

| File | Tests | What It Tests |
|------|-------|---------------|
| `Mocks/MockAPIClient.swift` | — | Mock implementation with response/error dictionaries |
| `Mocks/MockAuthService.swift` | — | Mock with tracking flags and error simulation |
| `AuthModelsTests.swift` | 8 | LoginRequest/RegisterRequest validation |
| `HomeViewModelTests.swift` | 5 | Data loading, error handling, loading state |
| `ChallengeViewModelTests.swift` | 8 | Character count, validation, submission |
| `FeedViewModelTests.swift` | 6 | Feed loading, pagination, optimistic likes |
| `ProfileViewModelTests.swift` | 6 | Profile loading, isCurrentUser, follow toggle |
| `AuthServiceTests.swift` | 6 | Logout, initial state, validation, userId fallback |
| **Total** | **39** | |

## Xcode Setup Instructions

### 1. Create Test Target
1. Open `Gengoka.xcodeproj` in Xcode
2. **File → New → Target...**
3. Select **Unit Testing Bundle**
4. Name: `GengokaTess`
5. Ensure **Host Application** = `Gengoka`
6. Click **Finish**

### 2. Add Protocol Files to Main Target
1. In the Project Navigator, right-click `Services` group
2. **Add Files to "Gengoka"...**
3. Navigate to `Services/Protocols/` and select both files:
   - `APIClientProtocol.swift`
   - `AuthServiceProtocol.swift`
4. Ensure **Target Membership** = `Gengoka` (main target)
5. Click **Add**

### 3. Add Test Files to Test Target
1. In the Project Navigator, right-click the `GengokaTess` group
2. **Add Files to "Gengoka"...**
3. Navigate to `GengokaTess/` and select all files and the `Mocks` folder
4. Ensure **Target Membership** = `GengokaTess` (test target only)
5. Click **Add**

### 4. Run Tests
- **Cmd+U** to run all tests
- Or click the diamond icon next to each test class/method

## Verification Checklist
- [ ] Protocol files added to main Gengoka target
- [ ] `APIClient` and `AuthService` conformance compiles
- [ ] ViewModels still work with default parameters (no behavioral change)
- [ ] Mock files added to GengokaTess target only
- [ ] Test files added to GengokaTess target only
- [ ] All 39 tests pass via Cmd+U
