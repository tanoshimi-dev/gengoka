//
//  MainTabView.swift
//  Gengoka
//

import SwiftUI

struct MainTabView: View {
    @State private var selectedTab = 0

    var body: some View {
        TabView(selection: $selectedTab) {
            HomeView()
                .tabItem {
                    Label("ホーム", systemImage: "house.fill")
                }
                .tag(0)

            FeedView()
                .tabItem {
                    Label("タイムライン", systemImage: "bubble.left.and.bubble.right.fill")
                }
                .tag(1)

            MyProfileView()
                .tabItem {
                    Label("マイページ", systemImage: "person.fill")
                }
                .tag(2)
        }
        .tint(AppColors.primaryGradientStart)
    }
}

#Preview {
    MainTabView()
}
