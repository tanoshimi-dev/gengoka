//
//  GengokaApp.swift
//  Gengoka
//
//  Created by MITAKI KEIJI on 2026/01/30.
//

import SwiftUI
import LineSDK

class AppDelegate: NSObject, UIApplicationDelegate {
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        LoginManager.shared.setup(channelID: "2009155107", universalLinkURL: nil)
        return true
    }

    func application(
        _ app: UIApplication,
        open url: URL,
        options: [UIApplication.OpenURLOptionsKey: Any] = [:]
    ) -> Bool {
        return LoginManager.shared.application(app, open: url, options: options)
    }
}

@main
struct GengokaApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
