package app.gengoka.core.network

import okhttp3.Interceptor
import okhttp3.Response
import javax.inject.Inject

class AuthInterceptor @Inject constructor(
    private val tokenManager: TokenManager,
    private val userIdProvider: UserIdProvider
) : Interceptor {

    override fun intercept(chain: Interceptor.Chain): Response {
        val originalRequest = chain.request()
        val requestBuilder = originalRequest.newBuilder()
            .header("Content-Type", "application/json")

        val accessToken = tokenManager.getAccessToken()
        if (accessToken != null) {
            requestBuilder.header("Authorization", "Bearer $accessToken")
            tokenManager.getUserId()?.let { userId ->
                requestBuilder.header("X-User-ID", userId)
            }
        } else {
            requestBuilder.header("X-User-ID", userIdProvider.getUserId())
        }

        return chain.proceed(requestBuilder.build())
    }
}
