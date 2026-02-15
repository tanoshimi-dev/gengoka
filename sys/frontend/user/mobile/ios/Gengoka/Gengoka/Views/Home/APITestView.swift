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
                    
                    Button {
                        Task {
                            await testFeedEndpoint()
                        }
                    } label: {
                        Label("フィードエンドポイントをテスト", systemImage: "list.bullet")
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.orange)
                            .foregroundColor(.white)
                            .cornerRadius(12)
                    }
                    .disabled(isRunning)
                    
                    Button {
                        Task {
                            await runFeedDebug()
                        }
                    } label: {
                        Label("フィードJSONを詳細表示", systemImage: "doc.text.magnifyingglass")
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.purple)
                            .foregroundColor(.white)
                            .cornerRadius(12)
                    }
                    .disabled(isRunning)
                    
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
    
    private func testFeedEndpoint() async {
        isRunning = true
        testResults.removeAll()
        testResults.append("🔍 フィードエンドポイントをテスト中...")
        
        print("\n🔍 Testing Feed Endpoint...")
        
        do {
            let response: FeedResponse<FeedItem> = try await APIClient.shared.request(.feed(page: 1, filter: .all))
            testResults.append("✅ フィードの読み込み成功!")
            testResults.append("📊 アイテム数: \(response.data.count)")
            testResults.append("📄 ページ: \(response.pagination.page)")
            testResults.append("📈 合計: \(response.pagination.total)")
            
            if let first = response.data.first {
                testResults.append("\n📝 最初のアイテム:")
                testResults.append("  Answer ID: \(first.answer.id)")
                testResults.append("  Content: \(first.answer.content.prefix(50))...")
                testResults.append("  Challenge: \(first.challenge.prompt)")
                testResults.append("  User: \(first.user.displayName ?? first.user.username)")
            }
            
            print("✅ Feed test completed successfully")
        } catch {
            testResults.append("❌ エラー: \(error.localizedDescription)")
            print("❌ Feed test failed: \(error)")
        }
        
        testResults.append("\n📋 詳細はXcodeのコンソールを確認してください")
        isRunning = false
    }
    
    private func runFeedDebug() async {
        isRunning = true
        testResults.removeAll()
        testResults.append("🔍 フィードJSONを解析中...")
        testResults.append("📋 Xcodeのコンソールを確認してください")
        
        await FeedItemDebugHelper.testFeedDecoding()
        
        testResults.append("\n✅ 解析完了!")
        testResults.append("💡 詳細な情報はXcodeのコンソールに表示されています")
        
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
