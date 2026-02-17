//
//  RegisterView.swift
//  Gengoka
//

import SwiftUI
import AuthenticationServices

struct RegisterView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var email = ""
    @State private var username = ""
    @State private var displayName = ""
    @State private var password = ""
    @State private var confirmPassword = ""
    @State private var isLoading = false
    @State private var isSocialLoading = false
    @State private var errorMessage: String?
    @State private var showPassword = false
    @State private var showConfirmPassword = false
    @FocusState private var focusedField: Field?

    let onRegisterSuccess: () -> Void
    let switchToLogin: () -> Void
    
    enum Field {
        case email
        case username
        case displayName
        case password
        case confirmPassword
    }
    
    var body: some View {
        ScrollView {
            VStack(spacing: 32) {
                // Header
                VStack(spacing: 16) {
                    Image(systemName: "person.badge.plus.fill")
                        .font(.system(size: 64))
                        .foregroundStyle(AppColors.primaryGradient)
                        .padding(.top, 20)
                    
                    Text("新規登録")
                        .font(.largeTitle)
                        .fontWeight(.bold)
                        .foregroundColor(AppColors.textPrimary)
                    
                    Text("アカウントを作成して始めましょう")
                        .font(.subheadline)
                        .foregroundColor(AppColors.textSecondary)
                }
                .padding(.bottom, 8)
                
                // Form
                VStack(spacing: 20) {
                    // Email Field
                    VStack(alignment: .leading, spacing: 8) {
                        Text("メールアドレス")
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(AppColors.textPrimary)
                        
                        HStack {
                            Image(systemName: "envelope.fill")
                                .foregroundColor(AppColors.textSecondary)
                                .frame(width: 20)
                            
                            TextField("example@email.com", text: $email)
                                .textContentType(.emailAddress)
                                .keyboardType(.emailAddress)
                                .autocapitalization(.none)
                                .autocorrectionDisabled()
                                .focused($focusedField, equals: .email)
                                .submitLabel(.next)
                                .onSubmit {
                                    focusedField = .username
                                }
                        }
                        .padding()
                        .background(Color.white)
                        .cornerRadius(12)
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .stroke(focusedField == .email ? Color.blue : Color.gray.opacity(0.2), lineWidth: 1)
                        )
                    }
                    
                    // Username Field
                    VStack(alignment: .leading, spacing: 8) {
                        Text("ユーザー名")
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(AppColors.textPrimary)
                        
                        HStack {
                            Image(systemName: "at")
                                .foregroundColor(AppColors.textSecondary)
                                .frame(width: 20)
                            
                            TextField("username", text: $username)
                                .textContentType(.username)
                                .autocapitalization(.none)
                                .autocorrectionDisabled()
                                .focused($focusedField, equals: .username)
                                .submitLabel(.next)
                                .onSubmit {
                                    focusedField = .displayName
                                }
                        }
                        .padding()
                        .background(Color.white)
                        .cornerRadius(12)
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .stroke(focusedField == .username ? Color.blue : Color.gray.opacity(0.2), lineWidth: 1)
                        )
                        
                        Text("英数字とアンダースコアのみ、3-20文字")
                            .font(.caption)
                            .foregroundColor(AppColors.textSecondary)
                    }
                    
                    // Display Name Field (Optional)
                    VStack(alignment: .leading, spacing: 8) {
                        HStack {
                            Text("表示名")
                                .font(.subheadline)
                                .fontWeight(.medium)
                                .foregroundColor(AppColors.textPrimary)
                            
                            Text("(任意)")
                                .font(.caption)
                                .foregroundColor(AppColors.textSecondary)
                        }
                        
                        HStack {
                            Image(systemName: "person.fill")
                                .foregroundColor(AppColors.textSecondary)
                                .frame(width: 20)
                            
                            TextField("太郎", text: $displayName)
                                .textContentType(.name)
                                .focused($focusedField, equals: .displayName)
                                .submitLabel(.next)
                                .onSubmit {
                                    focusedField = .password
                                }
                        }
                        .padding()
                        .background(Color.white)
                        .cornerRadius(12)
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .stroke(focusedField == .displayName ? Color.blue : Color.gray.opacity(0.2), lineWidth: 1)
                        )
                    }
                    
                    // Password Field
                    VStack(alignment: .leading, spacing: 8) {
                        Text("パスワード")
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(AppColors.textPrimary)
                        
                        HStack {
                            Image(systemName: "lock.fill")
                                .foregroundColor(AppColors.textSecondary)
                                .frame(width: 20)
                            
                            if showPassword {
                                TextField("パスワードを入力", text: $password)
                                    .textContentType(.newPassword)
                                    .autocapitalization(.none)
                                    .autocorrectionDisabled()
                                    .focused($focusedField, equals: .password)
                                    .submitLabel(.next)
                                    .onSubmit {
                                        focusedField = .confirmPassword
                                    }
                            } else {
                                SecureField("パスワードを入力", text: $password)
                                    .textContentType(.newPassword)
                                    .autocapitalization(.none)
                                    .autocorrectionDisabled()
                                    .focused($focusedField, equals: .password)
                                    .submitLabel(.next)
                                    .onSubmit {
                                        focusedField = .confirmPassword
                                    }
                            }
                            
                            Button {
                                showPassword.toggle()
                            } label: {
                                Image(systemName: showPassword ? "eye.slash.fill" : "eye.fill")
                                    .foregroundColor(AppColors.textSecondary)
                            }
                        }
                        .padding()
                        .background(Color.white)
                        .cornerRadius(12)
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .stroke(focusedField == .password ? Color.blue : Color.gray.opacity(0.2), lineWidth: 1)
                        )
                        
                        Text("8文字以上")
                            .font(.caption)
                            .foregroundColor(AppColors.textSecondary)
                    }
                    
                    // Confirm Password Field
                    VStack(alignment: .leading, spacing: 8) {
                        Text("パスワード(確認)")
                            .font(.subheadline)
                            .fontWeight(.medium)
                            .foregroundColor(AppColors.textPrimary)
                        
                        HStack {
                            Image(systemName: "lock.fill")
                                .foregroundColor(AppColors.textSecondary)
                                .frame(width: 20)
                            
                            if showConfirmPassword {
                                TextField("パスワードを再入力", text: $confirmPassword)
                                    .textContentType(.newPassword)
                                    .autocapitalization(.none)
                                    .autocorrectionDisabled()
                                    .focused($focusedField, equals: .confirmPassword)
                                    .submitLabel(.go)
                                    .onSubmit {
                                        handleRegister()
                                    }
                            } else {
                                SecureField("パスワードを再入力", text: $confirmPassword)
                                    .textContentType(.newPassword)
                                    .autocapitalization(.none)
                                    .autocorrectionDisabled()
                                    .focused($focusedField, equals: .confirmPassword)
                                    .submitLabel(.go)
                                    .onSubmit {
                                        handleRegister()
                                    }
                            }
                            
                            Button {
                                showConfirmPassword.toggle()
                            } label: {
                                Image(systemName: showConfirmPassword ? "eye.slash.fill" : "eye.fill")
                                    .foregroundColor(AppColors.textSecondary)
                            }
                        }
                        .padding()
                        .background(Color.white)
                        .cornerRadius(12)
                        .overlay(
                            RoundedRectangle(cornerRadius: 12)
                                .stroke(focusedField == .confirmPassword ? Color.blue : Color.gray.opacity(0.2), lineWidth: 1)
                        )
                    }
                    
                    // Error Message
                    if let errorMessage = errorMessage {
                        HStack(spacing: 8) {
                            Image(systemName: "exclamationmark.triangle.fill")
                                .foregroundColor(.red)
                            Text(errorMessage)
                                .font(.subheadline)
                                .foregroundColor(.red)
                        }
                        .padding()
                        .frame(maxWidth: .infinity, alignment: .leading)
                        .background(Color.red.opacity(0.1))
                        .cornerRadius(12)
                    }
                    
                    // Register Button
                    Button {
                        handleRegister()
                    } label: {
                        HStack {
                            if isLoading {
                                ProgressView()
                                    .tint(.white)
                            } else {
                                Text("アカウントを作成")
                                    .fontWeight(.semibold)
                            }
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(AppColors.primaryGradient)
                        .foregroundColor(.white)
                        .cornerRadius(12)
                    }
                    .disabled(isLoading || !isFormValid)
                    .opacity(isLoading || !isFormValid ? 0.6 : 1.0)
                    
                    // Terms of Service
                    Text("登録することで、[利用規約](https://example.com)と[プライバシーポリシー](https://example.com)に同意したものとみなされます")
                        .font(.caption)
                        .foregroundColor(AppColors.textSecondary)
                        .multilineTextAlignment(.center)
                }
                .padding(.horizontal, 24)
                
                // Social Login Section
                VStack(spacing: 16) {
                    HStack {
                        Rectangle()
                            .fill(Color.gray.opacity(0.3))
                            .frame(height: 1)
                        Text("または")
                            .font(.caption)
                            .foregroundColor(AppColors.textSecondary)
                        Rectangle()
                            .fill(Color.gray.opacity(0.3))
                            .frame(height: 1)
                    }
                    .padding(.horizontal, 24)

                    VStack(spacing: 12) {
                        SocialLoginButton(
                            title: "Googleで登録",
                            iconName: "g.circle.fill",
                            backgroundColor: .white,
                            foregroundColor: AppColors.textPrimary,
                            borderColor: Color.gray.opacity(0.3),
                            isLoading: isSocialLoading
                        ) {
                            handleSocialLogin(provider: .google)
                        }

                        SocialLoginButton(
                            title: "Appleで登録",
                            iconName: "apple.logo",
                            backgroundColor: .black,
                            foregroundColor: .white,
                            isLoading: isSocialLoading
                        ) {
                            handleSocialLogin(provider: .apple)
                        }

                        SocialLoginButton(
                            title: "LINEで登録",
                            iconName: "message.fill",
                            backgroundColor: Color(hex: "#06C755"),
                            foregroundColor: .white,
                            isLoading: isSocialLoading
                        ) {
                            handleSocialLogin(provider: .line)
                        }
                    }
                    .padding(.horizontal, 24)
                }
                .disabled(isLoading || isSocialLoading)

                // Login Link
                VStack(spacing: 12) {
                    Rectangle()
                        .fill(Color.gray.opacity(0.2))
                        .frame(height: 1)
                        .padding(.horizontal, 24)

                    HStack {
                        Text("すでにアカウントをお持ちですか？")
                            .font(.subheadline)
                            .foregroundColor(AppColors.textSecondary)

                        Button {
                            switchToLogin()
                        } label: {
                            Text("ログイン")
                                .font(.subheadline)
                                .fontWeight(.semibold)
                                .foregroundColor(AppColors.primaryGradientStart)
                        }
                    }
                }
                .padding(.top, 8)
                
                Spacer()
            }
        }
        .background(AppColors.backgroundGradient.ignoresSafeArea())
        .onAppear {
            focusedField = .email
        }
    }
    
    // MARK: - Social Login

    private func handleSocialLogin(provider: SocialAuthProvider) {
        errorMessage = nil
        focusedField = nil
        isSocialLoading = true

        Task {
            do {
                let result: SocialAuthResult
                switch provider {
                case .apple:
                    let delegate = AppleSignInDelegate()
                    result = try await delegate.signIn()
                case .google:
                    result = try await GoogleSignInService.signIn()
                case .line:
                    result = try await LineSignInService.signIn()
                }

                try await AuthService.shared.socialLogin(result: result)
                await MainActor.run {
                    isSocialLoading = false
                    onRegisterSuccess()
                }
            } catch let error as SocialAuthError where error.errorDescription == nil {
                await MainActor.run { isSocialLoading = false }
            } catch {
                await MainActor.run {
                    isSocialLoading = false
                    if let socialError = error as? SocialAuthError {
                        errorMessage = socialError.localizedDescription
                    } else if let networkError = error as? NetworkError {
                        errorMessage = networkError.localizedDescription
                    } else {
                        errorMessage = "ソーシャルログインに失敗しました"
                    }
                }
            }
        }
    }

    private var isFormValid: Bool {
        !email.isEmpty && !username.isEmpty && !password.isEmpty && !confirmPassword.isEmpty
    }
    
    private func handleRegister() {
        errorMessage = nil
        focusedField = nil
        
        // Check password match
        if password != confirmPassword {
            errorMessage = "パスワードが一致しません"
            return
        }
        
        // Validate
        let request = RegisterRequest(
            email: email,
            password: password,
            username: username,
            name: displayName.isEmpty ? nil : displayName
        )
        
        if let error = request.validate() {
            errorMessage = error
            return
        }
        
        isLoading = true
        
        Task {
            do {
                try await AuthService.shared.register(
                    email: email,
                    password: password,
                    username: username,
                    name: displayName.isEmpty ? nil : displayName
                )
                await MainActor.run {
                    isLoading = false
                    onRegisterSuccess()
                }
            } catch {
                await MainActor.run {
                    isLoading = false
                    if let authError = error as? AuthError {
                        errorMessage = authError.localizedDescription
                    } else if let networkError = error as? NetworkError {
                        errorMessage = networkError.localizedDescription
                    } else {
                        errorMessage = "登録に失敗しました: \(error.localizedDescription)"
                    }
                }
            }
        }
    }
}

#Preview {
    RegisterView(
        onRegisterSuccess: {},
        switchToLogin: {}
    )
}
