//
//  AuthService.swift
//  Gengoka
//

import Foundation

@Observable
final class AuthService {
    static let shared = AuthService()

    private let userIdKey = "gengoka_user_id"
    private(set) var userId: UUID

    private init() {
        if let storedId = UserDefaults.standard.string(forKey: userIdKey),
           let uuid = UUID(uuidString: storedId) {
            self.userId = uuid
        } else {
            let newId = UUID()
            UserDefaults.standard.set(newId.uuidString, forKey: userIdKey)
            self.userId = newId
        }
    }

    func resetUserId() {
        let newId = UUID()
        UserDefaults.standard.set(newId.uuidString, forKey: userIdKey)
        self.userId = newId
    }
}
