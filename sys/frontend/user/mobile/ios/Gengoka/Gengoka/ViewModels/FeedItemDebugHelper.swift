//
//  FeedItemDebugHelper.swift
//  Gengoka
//
//  Helper for debugging feed item decoding issues
//

import Foundation

#if DEBUG
struct FeedItemDebugHelper {
    
    /// Test decoding a sample feed response
    static func testFeedDecoding() async {
        print("\n🔍 Testing Feed Endpoint Decoding...")
        print("=" + String(repeating: "=", count: 60))
        
        do {
            // Make raw request to get JSON data
            let url = URL(string: "http://localhost:8080/api/v1/feed?page=1&filter=all")!
            var request = URLRequest(url: url)
            request.setValue("application/json", forHTTPHeaderField: "Content-Type")
            
            // Add auth headers
            if let token = AuthService.shared.authToken {
                request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
            }
            request.setValue(AuthService.shared.userId.uuidString, forHTTPHeaderField: "User-ID")
            
            let (data, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                print("❌ Invalid response type")
                return
            }
            
            print("✅ HTTP Status: \(httpResponse.statusCode)")
            print()
            
            // Print raw JSON
            if let jsonString = String(data: data, encoding: .utf8) {
                print("📄 Raw JSON Response:")
                print(String(repeating: "-", count: 60))
                
                // Try to pretty print
                if let jsonObject = try? JSONSerialization.jsonObject(with: data),
                   let prettyData = try? JSONSerialization.data(withJSONObject: jsonObject, options: .prettyPrinted),
                   let prettyString = String(data: prettyData, encoding: .utf8) {
                    print(prettyString)
                } else {
                    print(jsonString)
                }
                print(String(repeating: "-", count: 60))
                print()
            }
            
            // Try to decode the structure step by step
            print("🔍 Attempting to decode structure...")
            
            let decoder = JSONDecoder()
            decoder.dateDecodingStrategy = .iso8601
            
            // First, try to decode the outer FeedResponse wrapper
            do {
                // Try direct FeedResponse decoding first
                struct DebugFeedResponse: Codable {
                    let data: [DebugFeedItem]
                    let pagination: FeedResponse<FeedItem>.Pagination
                }
                
                struct DebugFeedItem: Codable {
                    let keys: [String]
                    
                    init(from decoder: Decoder) throws {
                        let container = try decoder.container(keyedBy: DynamicCodingKeys.self)
                        keys = container.allKeys.map { $0.stringValue }
                    }
                    
                    struct DynamicCodingKeys: CodingKey {
                        var stringValue: String
                        var intValue: Int? { nil }
                        init?(stringValue: String) { self.stringValue = stringValue }
                        init?(intValue: Int) { return nil }
                    }
                }
                
                let debugResponse = try decoder.decode(DebugFeedResponse.self, from: data)
                print("✅ FeedResponse wrapper decoded successfully")
                print("   Total items in data array: \(debugResponse.data.count)")
                print("   Pagination - Page: \(debugResponse.pagination.page), Total: \(debugResponse.pagination.total)")
                print()
                
                if let firstItem = debugResponse.data.first {
                    print("📋 Keys in first feed item:")
                    for key in firstItem.keys.sorted() {
                        print("   - \(key)")
                    }
                    print()
                }
                
            } catch {
                print("❌ Failed to decode FeedResponse wrapper: \(error)")
            }
            
            // Now try the actual FeedItem decoding
            print("🔍 Attempting full FeedItem decoding...")
            do {
                let fullResponse = try decoder.decode(FeedResponse<FeedItem>.self, from: data)
                print("✅ Full decoding successful!")
                print("   Feed items: \(fullResponse.data.count)")
                if let first = fullResponse.data.first {
                    print("   First item:")
                    print("     - Answer ID: \(first.answer.id)")
                    print("     - Content: \(first.answer.content.prefix(50))...")
                    print("     - Challenge: \(first.challenge.prompt)")
                    print("     - User: \(first.user.displayName ?? first.user.username)")
                }
            } catch {
                print("❌ Full decoding failed: \(error)")
                if let decodingError = error as? DecodingError {
                    printDecodingError(decodingError)
                }
            }
            
        } catch {
            print("❌ Network error: \(error)")
        }
        
        print("=" + String(repeating: "=", count: 60))
        print()
    }
    
    private static func printDecodingError(_ error: DecodingError) {
        switch error {
        case .keyNotFound(let key, let context):
            print("   Missing key: '\(key.stringValue)'")
            print("   Path: \(context.codingPath.map { $0.stringValue }.joined(separator: " → "))")
            print("   Description: \(context.debugDescription)")
        case .typeMismatch(let type, let context):
            print("   Type mismatch: expected \(type)")
            print("   Path: \(context.codingPath.map { $0.stringValue }.joined(separator: " → "))")
            print("   Description: \(context.debugDescription)")
        case .valueNotFound(let type, let context):
            print("   Missing value: expected \(type)")
            print("   Path: \(context.codingPath.map { $0.stringValue }.joined(separator: " → "))")
            print("   Description: \(context.debugDescription)")
        case .dataCorrupted(let context):
            print("   Data corrupted")
            print("   Path: \(context.codingPath.map { $0.stringValue }.joined(separator: " → "))")
            print("   Description: \(context.debugDescription)")
        @unknown default:
            print("   Unknown error: \(error)")
        }
    }
    
    /// Quick command to run from the console
    static func run() {
        Task {
            await testFeedDecoding()
        }
    }
}
#endif
