//
//  APITestView.swift
//  Gengoka
//
//  デバッグ用のAPI テストビュー
//

#if DEBUG
import SwiftUI

struct APITestView: View {
    @State private var testResults: [String] = []
    @State private var isRunning = false
    
    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 16) {
                    Button {
                        Task {
                            await runTests()
                        }
                    } label: {
                        Label("API テストを実行", systemImage: "play.circle.fill")
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.blue)
                            .foregroundColor(.white)
                            .cornerRadius(12)
                    }
                    .disabled(isRunning)
                    
                    Button {
                        testResults.removeAll()
                        APIDebugHelper.printExpectedFormat()
                    } label: {
                        Label("期待されるフォーマットを表示", systemImage: "doc.text")
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.green)
                            .foregroundColor(.white)
                            .cornerRadius(12)
                    }
                    
                    if isRunning {
                        ProgressView("テスト実行中...")
                            .padding()
                    }
                    
                    if !testResults.isEmpty {
                        VStack(alignment: .leading, spacing: 8) {
                            Text("テスト結果:")
                                .font(.headline)
                                .padding(.top)
                            
                            ForEach(testResults.indices, id: \.self) { index in
                                Text(testResults[index])
                                    .font(.system(.caption, design: .monospaced))
                                    .padding(8)
                                    .frame(maxWidth: .infinity, alignment: .leading)
                                    .background(Color.gray.opacity(0.1))
                                    .cornerRadius(8)
                            }
                        }
                    }
                    
                    Divider()
                        .padding(.vertical)
                    
                    VStack(alignment: .leading, spacing: 12) {
                        Text("現在のユーザーID")
                            .font(.headline)
                        
                        Text(AuthService.shared.userId.uuidString)
                            .font(.system(.caption, design: .monospaced))
                            .padding(8)
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .background(Color.gray.opacity(0.1))
                            .cornerRadius(8)
                        
                        Button {
                            AuthService.shared.resetUserId()
                        } label: {
                            Label("ユーザーIDをリセット", systemImage: "arrow.clockwise")
                                .font(.caption)
                        }
                    }
                    
                    Divider()
                        .padding(.vertical)
                    
                    VStack(alignment: .leading, spacing: 12) {
                        Text("バックエンドチェックリスト")
                            .font(.headline)
                        
                        ChecklistItem(
                            text: "サーバーが起動している",
                            detail: "http://localhost:8080"
                        )
                        
                        ChecklistItem(
                            text: "APIレスポンスがAPIResponse形式",
                            detail: "{\"success\":true,\"data\":{...}}"
                        )
                        
                        ChecklistItem(
                            text: "JSONキーがsnake_case",
                            detail: "category_id, created_at など"
                        )
                        
                        ChecklistItem(
                            text: "日付がISO8601形式",
                            detail: "2026-02-12T12:00:00Z"
                        )
                    }
                }
                .padding()
            }
            .navigationTitle("API テスト")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
    
    private func runTests() async {
        isRunning = true
        testResults.removeAll()
        testResults.append("🔍 API テストを開始...")
        
        await APIDebugHelper.testEndpoints()
        
        testResults.append("✅ テスト完了")
        testResults.append("📋 詳細はXcodeのコンソールを確認してください")
        
        isRunning = false
    }
}

struct ChecklistItem: View {
    let text: String
    let detail: String
    
    var body: some View {
        HStack(alignment: .top, spacing: 8) {
            Image(systemName: "checkmark.circle")
                .foregroundColor(.green)
            
            VStack(alignment: .leading, spacing: 4) {
                Text(text)
                    .font(.subheadline)
                
                Text(detail)
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
        }
        .padding(8)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color.gray.opacity(0.05))
        .cornerRadius(8)
    }
}

#Preview {
    APITestView()
}
#endif
