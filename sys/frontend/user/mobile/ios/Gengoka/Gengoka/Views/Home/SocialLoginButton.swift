//
//  SocialLoginButton.swift
//  Gengoka
//

import SwiftUI

struct SocialLoginButton: View {
    let title: String
    let iconName: String
    let backgroundColor: Color
    let foregroundColor: Color
    var borderColor: Color? = nil
    let isLoading: Bool
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            HStack(spacing: 12) {
                if isLoading {
                    ProgressView()
                        .tint(foregroundColor)
                        .frame(width: 20, height: 20)
                } else {
                    Image(systemName: iconName)
                        .font(.system(size: 18, weight: .medium))
                        .frame(width: 20, height: 20)
                }

                Text(title)
                    .fontWeight(.medium)
                    .font(.subheadline)
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 14)
            .background(backgroundColor)
            .foregroundColor(foregroundColor)
            .cornerRadius(12)
            .overlay(
                RoundedRectangle(cornerRadius: 12)
                    .stroke(borderColor ?? Color.clear, lineWidth: 1)
            )
        }
        .disabled(isLoading)
        .opacity(isLoading ? 0.6 : 1.0)
    }
}
