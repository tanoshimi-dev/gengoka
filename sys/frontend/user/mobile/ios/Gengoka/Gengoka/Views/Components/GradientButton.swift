//
//  GradientButton.swift
//  Gengoka
//

import SwiftUI

struct GradientButton: View {
    let title: String
    let action: () -> Void
    var isEnabled: Bool = true
    var isLoading: Bool = false

    var body: some View {
        Button(action: action) {
            ZStack {
                if isLoading {
                    ProgressView()
                        .progressViewStyle(CircularProgressViewStyle(tint: .white))
                } else {
                    Text(title)
                        .font(.headline)
                        .fontWeight(.semibold)
                }
            }
            .foregroundColor(.white)
            .frame(maxWidth: .infinity)
            .frame(height: 54)
            .background(
                Group {
                    if isEnabled {
                        AppColors.primaryGradient
                    } else {
                        LinearGradient(
                            colors: [Color.gray.opacity(0.5), Color.gray.opacity(0.5)],
                            startPoint: .topLeading,
                            endPoint: .bottomTrailing
                        )
                    }
                }
            )
            .cornerRadius(16)
            .shadow(color: isEnabled ? AppColors.primaryGradientStart.opacity(0.4) : .clear, radius: 8, x: 0, y: 4)
        }
        .disabled(!isEnabled || isLoading)
    }
}

struct SecondaryButton: View {
    let title: String
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.headline)
                .fontWeight(.medium)
                .foregroundColor(AppColors.primaryGradientStart)
                .frame(maxWidth: .infinity)
                .frame(height: 54)
                .background(Color.white)
                .cornerRadius(16)
                .overlay(
                    RoundedRectangle(cornerRadius: 16)
                        .stroke(AppColors.primaryGradientStart, lineWidth: 2)
                )
        }
    }
}

#Preview {
    VStack(spacing: 20) {
        GradientButton(title: "送信する", action: {})
        GradientButton(title: "送信中...", action: {}, isLoading: true)
        GradientButton(title: "送信できません", action: {}, isEnabled: false)
        SecondaryButton(title: "キャンセル", action: {})
    }
    .padding()
}
