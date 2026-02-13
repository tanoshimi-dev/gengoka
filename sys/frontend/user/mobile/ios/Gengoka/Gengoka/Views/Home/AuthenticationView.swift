//
//  AuthenticationView.swift
//  Gengoka
//

import SwiftUI

struct AuthenticationView: View {
    @State private var showingLogin = true
    
    var body: some View {
        ZStack {
            if showingLogin {
                LoginView(
                    onLoginSuccess: {},
                    switchToRegister: {
                        withAnimation(.easeInOut(duration: 0.3)) {
                            showingLogin = false
                        }
                    }
                )
                .transition(.asymmetric(
                    insertion: .move(edge: .leading).combined(with: .opacity),
                    removal: .move(edge: .trailing).combined(with: .opacity)
                ))
            } else {
                RegisterView(
                    onRegisterSuccess: {},
                    switchToLogin: {
                        withAnimation(.easeInOut(duration: 0.3)) {
                            showingLogin = true
                        }
                    }
                )
                .transition(.asymmetric(
                    insertion: .move(edge: .trailing).combined(with: .opacity),
                    removal: .move(edge: .leading).combined(with: .opacity)
                ))
            }
        }
    }
}

#Preview {
    AuthenticationView()
}
