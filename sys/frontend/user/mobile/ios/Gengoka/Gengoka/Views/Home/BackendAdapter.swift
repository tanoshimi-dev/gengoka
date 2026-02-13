//
//  BackendAdapter.swift
//  Gengoka
//
//  Adapter to work with existing Android backend
//

import Foundation

/// Helper to adapt the iOS app to work with the existing Android backend
struct BackendAdapter {
    
    /// Convert raw challenges from backend to DailyChallenge format
    static func adaptDailyChallenges(
        _ challenges: [Challenge],
        categories: [Category]
    ) -> [DailyChallenge] {
        challenges.map { challenge in
            let categoryName = categories.first { $0.id == challenge.categoryId }?.name ?? ""
            return DailyChallenge(
                challenge: challenge,
                categoryName: categoryName,
                isCompleted: false // Backend should provide this
            )
        }
    }
    
    /// Try to decode challenges in multiple formats
    static func decodeDailyChallenges(from data: Data) throws -> [DailyChallenge] {
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        
        // First, try to unwrap from APIResponse
        if let apiResponse = try? decoder.decode(APIResponse<[DailyChallenge]>.self, from: data),
           let dailyChallenges = apiResponse.data {
            return dailyChallenges
        }
        
        // Try direct array of DailyChallenge
        if let dailyChallenges = try? decoder.decode([DailyChallenge].self, from: data) {
            return dailyChallenges
        }
        
        // Try APIResponse with Challenge array (Android format)
        if let apiResponse = try? decoder.decode(APIResponse<[Challenge]>.self, from: data),
           let challenges = apiResponse.data {
            // Convert to DailyChallenge format
            return challenges.map { challenge in
                DailyChallenge(
                    challenge: challenge,
                    categoryName: "", // Will be populated later
                    isCompleted: false
                )
            }
        }
        
        // Try direct array of Challenge
        if let challenges = try? decoder.decode([Challenge].self, from: data) {
            return challenges.map { challenge in
                DailyChallenge(
                    challenge: challenge,
                    categoryName: "",
                    isCompleted: false
                )
            }
        }
        
        throw NetworkError.decodingError
    }
}
