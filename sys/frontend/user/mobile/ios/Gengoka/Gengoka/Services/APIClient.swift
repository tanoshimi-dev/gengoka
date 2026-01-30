//
//  APIClient.swift
//  Gengoka
//

import Foundation

actor APIClient {
    static let shared = APIClient()

    private let baseURL = "http://localhost:8080/api/v1"
    private let session: URLSession
    private let decoder: JSONDecoder
    private let encoder: JSONEncoder

    private init() {
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 30
        config.timeoutIntervalForResource = 60
        self.session = URLSession(configuration: config)

        self.decoder = JSONDecoder()
        self.decoder.dateDecodingStrategy = .iso8601

        self.encoder = JSONEncoder()
        self.encoder.dateEncodingStrategy = .iso8601
    }

    private func buildRequest(for endpoint: APIEndpoint, body: Data? = nil) -> URLRequest {
        let url = URL(string: baseURL + endpoint.path)!
        var request = URLRequest(url: url)
        request.httpMethod = endpoint.method.rawValue
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue(AuthService.shared.userId.uuidString, forHTTPHeaderField: "User-ID")

        if let body = body {
            request.httpBody = body
        }

        return request
    }

    func request<T: Codable>(_ endpoint: APIEndpoint) async throws -> T {
        let request = buildRequest(for: endpoint)
        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw NetworkError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            if let apiError = try? decoder.decode(APIResponse<EmptyResponse>.self, from: data),
               let error = apiError.error {
                throw NetworkError.serverError(error.message)
            }
            throw NetworkError.httpError(httpResponse.statusCode)
        }

        do {
            let apiResponse = try decoder.decode(APIResponse<T>.self, from: data)
            if let responseData = apiResponse.data {
                return responseData
            } else if let error = apiResponse.error {
                throw NetworkError.serverError(error.message)
            } else {
                throw NetworkError.decodingError
            }
        } catch let error as NetworkError {
            throw error
        } catch {
            // Try direct decoding as fallback
            do {
                return try decoder.decode(T.self, from: data)
            } catch {
                throw NetworkError.decodingError
            }
        }
    }

    func request<T: Codable, B: Codable>(_ endpoint: APIEndpoint, body: B) async throws -> T {
        let bodyData = try encoder.encode(body)
        let request = buildRequest(for: endpoint, body: bodyData)
        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw NetworkError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            if let apiError = try? decoder.decode(APIResponse<EmptyResponse>.self, from: data),
               let error = apiError.error {
                throw NetworkError.serverError(error.message)
            }
            throw NetworkError.httpError(httpResponse.statusCode)
        }

        do {
            let apiResponse = try decoder.decode(APIResponse<T>.self, from: data)
            if let responseData = apiResponse.data {
                return responseData
            } else if let error = apiResponse.error {
                throw NetworkError.serverError(error.message)
            } else {
                throw NetworkError.decodingError
            }
        } catch let error as NetworkError {
            throw error
        } catch {
            do {
                return try decoder.decode(T.self, from: data)
            } catch {
                throw NetworkError.decodingError
            }
        }
    }

    func requestVoid(_ endpoint: APIEndpoint) async throws {
        let request = buildRequest(for: endpoint)
        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw NetworkError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            if let apiError = try? decoder.decode(APIResponse<EmptyResponse>.self, from: data),
               let error = apiError.error {
                throw NetworkError.serverError(error.message)
            }
            throw NetworkError.httpError(httpResponse.statusCode)
        }
    }

    func requestVoid<B: Codable>(_ endpoint: APIEndpoint, body: B) async throws {
        let bodyData = try encoder.encode(body)
        let request = buildRequest(for: endpoint, body: bodyData)
        let (data, response) = try await session.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse else {
            throw NetworkError.invalidResponse
        }

        guard (200...299).contains(httpResponse.statusCode) else {
            if let apiError = try? decoder.decode(APIResponse<EmptyResponse>.self, from: data),
               let error = apiError.error {
                throw NetworkError.serverError(error.message)
            }
            throw NetworkError.httpError(httpResponse.statusCode)
        }
    }
}

enum NetworkError: LocalizedError {
    case invalidResponse
    case httpError(Int)
    case decodingError
    case serverError(String)

    var errorDescription: String? {
        switch self {
        case .invalidResponse:
            return "サーバーからの応答が無効です"
        case .httpError(let code):
            return "HTTPエラー: \(code)"
        case .decodingError:
            return "データの解析に失敗しました"
        case .serverError(let message):
            return message
        }
    }
}
