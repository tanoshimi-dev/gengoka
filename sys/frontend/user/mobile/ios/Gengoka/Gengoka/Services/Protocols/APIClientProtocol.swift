//
//  APIClientProtocol.swift
//  Gengoka
//

import Foundation

protocol APIClientProtocol: Sendable {
    func request<T: Codable>(_ endpoint: APIEndpoint) async throws -> T
    func request<T: Codable, B: Codable>(_ endpoint: APIEndpoint, body: B) async throws -> T
    func requestVoid(_ endpoint: APIEndpoint) async throws
    func requestVoid<B: Codable>(_ endpoint: APIEndpoint, body: B) async throws
}
