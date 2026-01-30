//
//  BackButton.swift
//  Gengoka
//

import SwiftUI

struct BackButton: View {
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        Button(action: { dismiss() }) {
            Image(systemName: "chevron.left")
                .font(.system(size: 18, weight: .semibold))
                .foregroundColor(AppColors.textPrimary)
                .frame(width: 44, height: 44)
                .background(Color.white.opacity(0.9))
                .clipShape(Circle())
                .shadow(color: .black.opacity(0.1), radius: 4, x: 0, y: 2)
        }
    }
}

#Preview {
    BackButton()
        .padding()
        .background(AppColors.backgroundGradient)
}
