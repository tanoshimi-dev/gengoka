package app.dev.gengoka.core.network

import okhttp3.Interceptor
import okhttp3.Response
import javax.inject.Inject

class AuthInterceptor @Inject constructor(
    private val userIdProvider: UserIdProvider
) : Interceptor {

    override fun intercept(chain: Interceptor.Chain): Response {
        val originalRequest = chain.request()
        val userId = userIdProvider.getUserId()

        val newRequest = originalRequest.newBuilder()
            .header("X-User-ID", userId)
            .header("Content-Type", "application/json")
            .build()

        return chain.proceed(newRequest)
    }
}
