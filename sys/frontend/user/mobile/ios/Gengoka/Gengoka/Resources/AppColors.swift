//
//  AppColors.swift
//  Gengoka
//

import SwiftUI

enum AppColors {
    static let primaryGradientStart = Color(hex: "#667eea")
    static let primaryGradientEnd = Color(hex: "#764ba2")

    static let primaryGradient = LinearGradient(
        colors: [primaryGradientStart, primaryGradientEnd],
        startPoint: .topLeading,
        endPoint: .bottomTrailing
    )

    static let secondaryGradient = LinearGradient(
        colors: [Color(hex: "#f093fb"), Color(hex: "#f5576c")],
        startPoint: .topLeading,
        endPoint: .bottomTrailing
    )

    static let backgroundGradient = LinearGradient(
        colors: [Color(hex: "#667eea").opacity(0.1), Color(hex: "#764ba2").opacity(0.1)],
        startPoint: .top,
        endPoint: .bottom
    )

    static let cardBackground = Color.white
    static let textPrimary = Color(hex: "#1a1a2e")
    static let textSecondary = Color(hex: "#666666")
    static let textTertiary = Color(hex: "#999999")

    static let success = Color(hex: "#43e97b")
    static let warning = Color(hex: "#f6d365")
    static let error = Color(hex: "#f5576c")

    static let borderColor = Color(hex: "#e0e0e0")
}

extension Color {
    init(hex: String) {
        let hex = hex.trimmingCharacters(in: CharacterSet.alphanumerics.inverted)
        var int: UInt64 = 0
        Scanner(string: hex).scanHexInt64(&int)
        let a, r, g, b: UInt64
        switch hex.count {
        case 3: // RGB (12-bit)
            (a, r, g, b) = (255, (int >> 8) * 17, (int >> 4 & 0xF) * 17, (int & 0xF) * 17)
        case 6: // RGB (24-bit)
            (a, r, g, b) = (255, int >> 16, int >> 8 & 0xFF, int & 0xFF)
        case 8: // ARGB (32-bit)
            (a, r, g, b) = (int >> 24, int >> 16 & 0xFF, int >> 8 & 0xFF, int & 0xFF)
        default:
            (a, r, g, b) = (1, 1, 1, 0)
        }
        self.init(
            .sRGB,
            red: Double(r) / 255,
            green: Double(g) / 255,
            blue: Double(b) / 255,
            opacity: Double(a) / 255
        )
    }
}
