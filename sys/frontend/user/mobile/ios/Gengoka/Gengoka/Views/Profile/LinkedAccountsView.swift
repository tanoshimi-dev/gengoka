//
//  LinkedAccountsView.swift
//  Gengoka
//

import SwiftUI

@Observable
final class LinkedAccountsViewModel {
    var accounts: [LinkedSocialAccount] = []
    var isLoading = false
    var isLinking = false
    var error: String?
    var unlinkTarget: String?

    private let apiClient: any APIClientProtocol
    private let appleDelegate = AppleSignInDelegate()

    init(apiClient: any APIClientProtocol = APIClient.shared) {
        self.apiClient = apiClient
    }

    var allProviders: [(id: String, name: String, icon: String, color: Color)] {
        [
            ("google", "Google", "globe", .red),
            ("apple", "Apple", "apple.logo", .black),
            ("line", "LINE", "message.fill", Color(red: 0.0, green: 0.73, blue: 0.33))
        ]
    }

    func linkedAccount(for provider: String) -> LinkedSocialAccount? {
        accounts.first { $0.provider == provider }
    }

    func loadAccounts() async {
        isLoading = true
        error = nil
        do {
            let result: [LinkedSocialAccount] = try await apiClient.request(.linkedAccounts)
            accounts = result
        } catch let e {
            if !e.isCancellationError {
                self.error = e.localizedDescription
            }
        }
        isLoading = false
    }

    func linkAccount(provider: String) async {
        isLinking = true
        error = nil

        do {
            let authResult: SocialAuthResult
            switch provider {
            case "apple":
                authResult = try await appleDelegate.signIn()
            case "google":
                authResult = try await GoogleSignInService.signIn()
            case "line":
                authResult = try await LineSignInService.signIn()
            default:
                throw SocialAuthError.notConfigured(provider)
            }

            let request = LinkAccountRequest(
                provider: authResult.provider.rawValue,
                idToken: authResult.idToken,
                accessToken: authResult.accessToken
            )

            let linked: LinkedSocialAccount = try await apiClient.request(.linkAccount, body: request)
            accounts.append(linked)
        } catch let authError as SocialAuthError where authError.errorDescription == nil {
            // User cancelled — ignore
        } catch let e {
            if !e.isCancellationError {
                self.error = e.localizedDescription
            }
        }
        isLinking = false
    }

    func unlinkAccount(provider: String) async {
        error = nil
        do {
            try await apiClient.requestVoid(.unlinkAccount(provider: provider))
            accounts.removeAll { $0.provider == provider }
        } catch let e {
            if !e.isCancellationError {
                self.error = e.localizedDescription
            }
        }
        unlinkTarget = nil
    }
}

struct LinkedAccountsView: View {
    @State private var viewModel = LinkedAccountsViewModel()

    var body: some View {
        List {
            Section {
                ForEach(viewModel.allProviders, id: \.id) { provider in
                    providerRow(provider)
                }
            } header: {
                Text("ソーシャルアカウント")
            } footer: {
                Text("連携したアカウントでログインできます。すべての連携を解除することはできません。")
                    .font(.caption2)
            }

            if let error = viewModel.error {
                Section {
                    Text(error)
                        .foregroundColor(.red)
                        .font(.caption)
                }
            }
        }
        .navigationTitle("アカウント連携")
        .navigationBarTitleDisplayMode(.inline)
        .task {
            await viewModel.loadAccounts()
        }
        .overlay {
            if viewModel.isLoading && viewModel.accounts.isEmpty {
                ProgressView()
            }
        }
        .alert("連携を解除しますか？", isPresented: Binding(
            get: { viewModel.unlinkTarget != nil },
            set: { if !$0 { viewModel.unlinkTarget = nil } }
        )) {
            Button("解除", role: .destructive) {
                if let provider = viewModel.unlinkTarget {
                    Task {
                        await viewModel.unlinkAccount(provider: provider)
                    }
                }
            }
            Button("キャンセル", role: .cancel) {
                viewModel.unlinkTarget = nil
            }
        } message: {
            Text("この連携を解除すると、このアカウントでのログインができなくなります。")
        }
    }

    @ViewBuilder
    private func providerRow(_ provider: (id: String, name: String, icon: String, color: Color)) -> some View {
        let linked = viewModel.linkedAccount(for: provider.id)

        HStack(spacing: 12) {
            Image(systemName: provider.icon)
                .font(.title3)
                .foregroundColor(provider.color)
                .frame(width: 28)

            VStack(alignment: .leading, spacing: 2) {
                Text(provider.name)
                    .font(.body)
                    .foregroundColor(AppColors.textPrimary)

                if let linked = linked {
                    Text(linked.providerEmail ?? "連携済み")
                        .font(.caption)
                        .foregroundColor(AppColors.textSecondary)
                } else {
                    Text("未連携")
                        .font(.caption)
                        .foregroundColor(AppColors.textTertiary)
                }
            }

            Spacer()

            if linked != nil {
                Button("解除") {
                    viewModel.unlinkTarget = provider.id
                }
                .font(.caption)
                .foregroundColor(.red)
                .buttonStyle(.bordered)
                .controlSize(.small)
            } else {
                Button("連携") {
                    Task {
                        await viewModel.linkAccount(provider: provider.id)
                    }
                }
                .font(.caption)
                .buttonStyle(.borderedProminent)
                .controlSize(.small)
                .disabled(viewModel.isLinking)
            }
        }
        .padding(.vertical, 4)
    }
}

#Preview {
    NavigationStack {
        LinkedAccountsView()
    }
}
