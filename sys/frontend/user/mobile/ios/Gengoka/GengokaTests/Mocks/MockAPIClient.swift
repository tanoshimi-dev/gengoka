//
//  MockAPIClient.swift
//  GengokaTess
//

import Foundation
@testable import Gengoka

actor MockAPIClient: APIClientProtocol {
    var responses: [String: Any] = [:]
    var errors: [String: Error] = [:]
    var calledEndpoints: [APIEndpoint] = []

    func setResponse<T: Codable>(_ response: T, for path: String) {
        responses[path] = response
    }

    func setError(_ error: Error, for path: String) {
        errors[path] = error
    }

    func request<T: Codable>(_ endpoint: APIEndpoint) async throws -> T {
        calledEndpoints.append(endpoint)

        if let error = errors[endpoint.path] {
            throw error
        }

        guard let response = responses[endpoint.path] as? T else {
            throw NetworkError.invalidResponse
        }

        return response
    }

    func request<T: Codable, B: Codable>(_ endpoint: APIEndpoint, body: B) async throws -> T {
        calledEndpoints.append(endpoint)

        if let error = errors[endpoint.path] {
            throw error
        }

        guard let response = responses[endpoint.path] as? T else {
            throw NetworkError.invalidResponse
        }

        return response
    }

    func requestVoid(_ endpoint: APIEndpoint) async throws {
        calledEndpoints.append(endpoint)

        if let error = errors[endpoint.path] {
            throw error
        }
    }

    func requestVoid<B: Codable>(_ endpoint: APIEndpoint, body: B) async throws {
        calledEndpoints.append(endpoint)

        if let error = errors[endpoint.path] {
            throw error
        }
    }

    func reset() {
        responses.removeAll()
        errors.removeAll()
        calledEndpoints.removeAll()
    }
}
