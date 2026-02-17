//
//  SocialAuthService.swift
//  Gengoka
//

import Foundation
import AuthenticationServices
import GoogleSignIn
import LineSDK
import UIKit

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

// MARK: - Google Sign-In

class GoogleSignInService {
    static func signIn() async throws -> SocialAuthResult {
        guard let rootViewController = await UIApplication.shared.connectedScenes
            .compactMap({ $0 as? UIWindowScene })
            .flatMap({ $0.windows })
            .first(where: { $0.isKeyWindow })?.rootViewController else {
            throw SocialAuthError.providerError("UIViewControllerが見つかりません")
        }

        return try await withCheckedThrowingContinuation { continuation in
            Task { @MainActor in
                GIDSignIn.sharedInstance.signIn(withPresenting: rootViewController) { result, error in
                    if let error = error as NSError?,
                       error.domain == "com.google.GIDSignIn",
                       error.code == -5 { // GIDSignInErrorCodeCanceled
                        continuation.resume(throwing: SocialAuthError.cancelled)
                        return
                    }

                    if let error = error {
                        continuation.resume(throwing: SocialAuthError.providerError(error.localizedDescription))
                        return
                    }

                    guard let user = result?.user,
                          let idToken = user.idToken?.tokenString else {
                        continuation.resume(throwing: SocialAuthError.invalidCredential)
                        return
                    }

                    let accessToken = user.accessToken.tokenString
                    let authResult = SocialAuthResult(
                        provider: .google,
                        idToken: idToken,
                        accessToken: accessToken
                    )
                    continuation.resume(returning: authResult)
                }
            }
        }
    }
}

// MARK: - LINE Sign-In

class LineSignInService {
    static func signIn() async throws -> SocialAuthResult {
        return try await withCheckedThrowingContinuation { continuation in
            LoginManager.shared.login(permissions: [.profile], in: nil) { result in
                switch result {
                case .success(let loginResult):
                    let token = loginResult.accessToken.value
                    let authResult = SocialAuthResult(
                        provider: .line,
                        idToken: nil,
                        accessToken: token
                    )
                    continuation.resume(returning: authResult)
                case .failure(let error):
                    if error.errorCode == 3003 {
                        // User cancelled
                        continuation.resume(throwing: SocialAuthError.cancelled)
                    } else {
                        continuation.resume(throwing: SocialAuthError.providerError(error.localizedDescription))
                    }
                }
            }
        }
    }
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
