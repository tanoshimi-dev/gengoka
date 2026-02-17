//
//  SocialAuthService.swift
//  Gengoka
//

import Foundation
import AuthenticationServices

enum SocialAuthProvider: String {
    case google
    case apple
    case line
}

struct SocialAuthResult {
    let provider: SocialAuthProvider
    let idToken: String?
    let accessToken: String?
}

// MARK: - Apple Sign-In Delegate

class AppleSignInDelegate: NSObject, ASAuthorizationControllerDelegate {
    private var continuation: CheckedContinuation<SocialAuthResult, Error>?

    func signIn() async throws -> SocialAuthResult {
        return try await withCheckedThrowingContinuation { continuation in
            self.continuation = continuation

            let provider = ASAuthorizationAppleIDProvider()
            let request = provider.createRequest()
            request.requestedScopes = [.fullName, .email]

            let controller = ASAuthorizationController(authorizationRequests: [request])
            controller.delegate = self
            controller.performRequests()
        }
    }

    func authorizationController(controller: ASAuthorizationController, didCompleteWithAuthorization authorization: ASAuthorization) {
        guard let credential = authorization.credential as? ASAuthorizationAppleIDCredential,
              let identityTokenData = credential.identityToken,
              let idToken = String(data: identityTokenData, encoding: .utf8) else {
            continuation?.resume(throwing: SocialAuthError.invalidCredential)
            continuation = nil
            return
        }

        let result = SocialAuthResult(
            provider: .apple,
            idToken: idToken,
            accessToken: nil
        )

        continuation?.resume(returning: result)
        continuation = nil
    }

    func authorizationController(controller: ASAuthorizationController, didCompleteWithError error: Error) {
        if let authError = error as? ASAuthorizationError, authError.code == .canceled {
            continuation?.resume(throwing: SocialAuthError.cancelled)
        } else {
            continuation?.resume(throwing: SocialAuthError.providerError(error.localizedDescription))
        }
        continuation = nil
    }
}

// MARK: - Social Auth Errors

enum SocialAuthError: LocalizedError {
    case cancelled
    case invalidCredential
    case providerError(String)
    case notConfigured(String)

    var errorDescription: String? {
        switch self {
        case .cancelled:
            return nil // Don't show error for user cancellation
        case .invalidCredential:
            return "認証情報の取得に失敗しました"
        case .providerError(let message):
            return "認証エラー: \(message)"
        case .notConfigured(let provider):
            return "\(provider)の設定が完了していません"
        }
    }
}
