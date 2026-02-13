//
//  LoginView.swift
//  Gengoka
//

import SwiftUI

struct LoginView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var email = ""
    @State private var password = ""
    @State private var isLoading = false
    @State private var errorMessage: String?
    @State private var showPassword = false
    @FocusState private var focusedField: Field?
    
    let onLoginSuccess: () -> Void
    let switchToRegister: () -> Void
    
    enum Field {
        case email
        case password
    }
    
    var body: some View {
        ScrollView {
            VStack(spacing: 32) {
                // Header
                VStack(spacing: 16) {
                    Image(systemName: "text.bubble.fill")
                        .font(.system(size: 72))
                        .foregroundStyle(AppColors.primaryGradient)
                        .padding(.top, 40)
                    
                    Text("Gengoka")
                        .font(.largeTitle)
                        .fontWeight(.bold)
                        .foregroundColor(AppColors.textPrimary)
                    
                    Text("言葉を磨く、表現を学ぶ")
                        .font(.subheadline)
                        .foregroundColor(AppColors.textSecondary)
                }
                .padding(.bottom, 16)
                
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
                                    focusedField = .password
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
                                    .textContentType(.password)
                                    .autocapitalization(.none)
                                    .autocorrectionDisabled()
                                    .focused($focusedField, equals: .password)
                                    .submitLabel(.go)
                                    .onSubmit {
                                        handleLogin()
                                    }
                            } else {
                                SecureField("パスワードを入力", text: $password)
                                    .textContentType(.password)
                                    .autocapitalization(.none)
                                    .autocorrectionDisabled()
                                    .focused($focusedField, equals: .password)
                                    .submitLabel(.go)
                                    .onSubmit {
                                        handleLogin()
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
                    
                    // Login Button
                    Button {
                        handleLogin()
                    } label: {
                        HStack {
                            if isLoading {
                                ProgressView()
                                    .tint(.white)
                            } else {
                                Text("ログイン")
                                    .fontWeight(.semibold)
                            }
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(AppColors.primaryGradient)
                        .foregroundColor(.white)
                        .cornerRadius(12)
                    }
                    .disabled(isLoading || email.isEmpty || password.isEmpty)
                    .opacity(isLoading || email.isEmpty || password.isEmpty ? 0.6 : 1.0)
                    
                    // Forgot Password
                    Button {
                        // TODO: Implement forgot password
                    } label: {
                        Text("パスワードをお忘れですか？")
                            .font(.subheadline)
                            .foregroundColor(AppColors.primaryGradientStart)
                    }
                }
                .padding(.horizontal, 24)
                
                // Register Link
                VStack(spacing: 12) {
                    Rectangle()
                        .fill(Color.gray.opacity(0.2))
                        .frame(height: 1)
                        .padding(.horizontal, 24)
                    
                    HStack {
                        Text("アカウントをお持ちでないですか？")
                            .font(.subheadline)
                            .foregroundColor(AppColors.textSecondary)
                        
                        Button {
                            switchToRegister()
                        } label: {
                            Text("新規登録")
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
    
    private func handleLogin() {
        errorMessage = nil
        focusedField = nil
        
        // Validate
        let request = LoginRequest(email: email, password: password)
        if let error = request.validate() {
            errorMessage = error
            return
        }
        
        isLoading = true
        
        Task {
            do {
                try await AuthService.shared.login(email: email, password: password)
                await MainActor.run {
                    isLoading = false
                    onLoginSuccess()
                }
            } catch {
                await MainActor.run {
                    isLoading = false
                    if let authError = error as? AuthError {
                        errorMessage = authError.localizedDescription
                    } else if let networkError = error as? NetworkError {
                        errorMessage = networkError.localizedDescription
                    } else {
                        errorMessage = "ログインに失敗しました: \(error.localizedDescription)"
                    }
                }
            }
        }
    }
}

#Preview {
    LoginView(
        onLoginSuccess: {},
        switchToRegister: {}
    )
}
