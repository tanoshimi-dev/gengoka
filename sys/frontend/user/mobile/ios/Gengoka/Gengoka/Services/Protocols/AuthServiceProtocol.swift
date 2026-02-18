//
//  AuthServiceProtocol.swift
//  Gengoka
//

import Foundation

protocol AuthServiceProtocol: AnyObject {
    var currentUser: AuthUser? { get }
    var isAuthenticated: Bool { get }
    var authToken: String? { get }
    var userId: UUID { get }
    func login(email: String, password: String) async throws
    func register(email: String, password: String, username: String, name: String?) async throws
    func socialLogin(result: SocialAuthResult) async throws
    func logout()
    func refreshToken() async throws
}
