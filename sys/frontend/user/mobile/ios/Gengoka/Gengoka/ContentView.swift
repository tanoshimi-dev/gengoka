//
//  ContentView.swift
//  Gengoka
//
//  Created by MITAKI KEIJI on 2026/01/30.
//

import SwiftUI

struct ContentView: View {
    @State private var authService = AuthService.shared
    
    var body: some View {
        Group {
            if authService.isAuthenticated {
                MainTabView()
            } else {
                AuthenticationView()
            }
        }
        .animation(.easeInOut(duration: 0.3), value: authService.isAuthenticated)
    }
}

#Preview {
    ContentView()
}
#Preview("Authenticated") {
    let _ = {
        // Mock authenticated state for preview
        // Note: This won't actually work since AuthService is a singleton
        // but shows the intention
    }()
    ContentView()
}

