//
//  APIDebugHelper.swift
//  Gengoka
//
//  Helper for debugging API responses during development
//

import Foundation

#if DEBUG
struct APIDebugHelper {
    static func testEndpoints() async {
        print("🔍 Starting API endpoint tests...")
        
        let baseURL = "http://localhost:8080/api/v1"
        
        // Test categories endpoint
        await testEndpoint(
            url: "\(baseURL)/categories",
            name: "Categories",
            expectedType: [Category].self
        )
        
        // Test daily challenges endpoint
        await testEndpoint(
            url: "\(baseURL)/challenges/daily",
            name: "Daily Challenges",
            expectedType: [DailyChallenge].self
        )
        
        // Test user stats endpoint
        await testEndpoint(
            url: "\(baseURL)/users/me/stats",
            name: "User Stats",
            expectedType: UserStats.self
        )
    }
    
    private static func testEndpoint<T: Codable>(
        url: String,
        name: String,
        expectedType: T.Type
    ) async {
        print("\n📡 Testing \(name) endpoint: \(url)")
        
        guard let requestURL = URL(string: url) else {
            print("❌ Invalid URL")
            return
        }
        
        var request = URLRequest(url: requestURL)
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue(AuthService.shared.userId.uuidString, forHTTPHeaderField: "User-ID")
        
        do {
            let (data, response) = try await URLSession.shared.data(for: request)
            
            guard let httpResponse = response as? HTTPURLResponse else {
                print("❌ Invalid response type")
                return
            }
            
            print("✅ Status code: \(httpResponse.statusCode)")
            
            // Print raw response
            if let jsonString = String(data: data, encoding: .utf8) {
                print("📄 Raw response:")
                print(jsonString)
            }
            
            // Try to decode as APIResponse
            let decoder = JSONDecoder()
            decoder.dateDecodingStrategy = .iso8601
            
            do {
                let apiResponse = try decoder.decode(APIResponse<T>.self, from: data)
                print("✅ Successfully decoded as APIResponse<\(String(describing: T.self))>")
                print("   Success: \(apiResponse.success)")
                print("   Message: \(apiResponse.message ?? "none")")
                print("   Has data: \(apiResponse.data != nil)")
            } catch {
                print("⚠️ Failed to decode as APIResponse: \(error)")
                
                // Try direct decode
                do {
                    let directResponse = try decoder.decode(T.self, from: data)
                    print("✅ Successfully decoded directly as \(String(describing: T.self))")
                    print("   Note: Backend should wrap this in APIResponse format")
                } catch {
                    print("❌ Failed to decode directly: \(error)")
                }
            }
            
        } catch {
            print("❌ Network error: \(error)")
        }
    }
    
    /// Call this from your view to manually test endpoints
    static func printExpectedFormat() {
        print("""
        
        📋 Expected Backend Response Format:
        
        For GET /api/v1/categories:
        {
          "success": true,
          "data": [
            {
              "id": "uuid-string",
              "name": "カテゴリー名",
              "description": "説明",
              "icon_name": "icon.name",
              "color_hex": "#667eea",
              "challenge_count": 25
            }
          ],
          "message": null,
          "error": null
        }
        
        For GET /api/v1/challenges/daily:
        {
          "success": true,
          "data": [
            {
              "challenge": {
                "id": "uuid-string",
                "category_id": "uuid-string",
                "prompt": "チャレンジ内容",
                "description": "説明",
                "min_characters": 20,
                "max_characters": 50,
                "difficulty": "easy",
                "created_at": "2026-02-12T12:00:00Z"
              },
              "category_name": "カテゴリー名",
              "is_completed": false
            }
          ],
          "message": null,
          "error": null
        }
        
        For GET /api/v1/users/me/stats:
        {
          "success": true,
          "data": {
            "total_challenges": 234,
            "completed_today": 2,
            "current_streak": 7,
            "best_streak": 21,
            "average_score": 82.5
          },
          "message": null,
          "error": null
        }
        
        """)
    }
}
#endif
