package app.dev.gengoka.core.di

import app.dev.gengoka.core.network.AuthInterceptor
import app.dev.gengoka.core.network.TokenManager
import app.dev.gengoka.core.network.UserIdProvider
import app.dev.gengoka.data.api.GengokApi
import app.dev.gengoka.data.dto.RefreshRequestDto
import com.jakewharton.retrofit2.converter.kotlinx.serialization.asConverterFactory
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import okhttp3.Authenticator
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import java.util.concurrent.TimeUnit
import javax.inject.Qualifier
import javax.inject.Singleton

@Qualifier
@Retention(AnnotationRetention.BINARY)
annotation class AuthRetrofit

@Module
@InstallIn(SingletonComponent::class)
object NetworkModule {

    private const val BASE_URL = "http://10.0.2.2:8080/api/v1/"

    @Provides
    @Singleton
    fun provideJson(): Json = Json {
        ignoreUnknownKeys = true
        coerceInputValues = true
        isLenient = true
    }

    @Provides
    @Singleton
    fun provideAuthInterceptor(
        tokenManager: TokenManager,
        userIdProvider: UserIdProvider
    ): AuthInterceptor {
        return AuthInterceptor(tokenManager, userIdProvider)
    }

    @Provides
    @Singleton
    @AuthRetrofit
    fun provideAuthOkHttpClient(): OkHttpClient {
        val loggingInterceptor = HttpLoggingInterceptor().apply {
            level = HttpLoggingInterceptor.Level.BODY
        }
        return OkHttpClient.Builder()
            .addInterceptor(loggingInterceptor)
            .connectTimeout(30, TimeUnit.SECONDS)
            .readTimeout(30, TimeUnit.SECONDS)
            .writeTimeout(30, TimeUnit.SECONDS)
            .build()
    }

    @Provides
    @Singleton
    @AuthRetrofit
    fun provideAuthRetrofit(@AuthRetrofit okHttpClient: OkHttpClient, json: Json): Retrofit {
        return Retrofit.Builder()
            .baseUrl(BASE_URL)
            .client(okHttpClient)
            .addConverterFactory(json.asConverterFactory("application/json".toMediaType()))
            .build()
    }

    @Provides
    @Singleton
    @AuthRetrofit
    fun provideAuthGengokApi(@AuthRetrofit retrofit: Retrofit): GengokApi {
        return retrofit.create(GengokApi::class.java)
    }

    @Provides
    @Singleton
    fun provideTokenAuthenticator(
        tokenManager: TokenManager,
        @AuthRetrofit authApi: GengokApi
    ): Authenticator {
        return Authenticator { _, response ->
            if (response.request.header("Authorization") == null) {
                return@Authenticator null
            }

            val refreshToken = tokenManager.getRefreshToken() ?: return@Authenticator null

            try {
                val tokenResponse = runBlocking {
                    authApi.refreshToken(RefreshRequestDto(refreshToken = refreshToken))
                }
                runBlocking { tokenManager.saveTokens(tokenResponse.data) }

                response.request.newBuilder()
                    .header("Authorization", "Bearer ${tokenResponse.data.accessToken}")
                    .build()
            } catch (_: Exception) {
                runBlocking { tokenManager.clearTokens() }
                null
            }
        }
    }

    @Provides
    @Singleton
    fun provideOkHttpClient(
        authInterceptor: AuthInterceptor,
        authenticator: Authenticator
    ): OkHttpClient {
        val loggingInterceptor = HttpLoggingInterceptor().apply {
            level = HttpLoggingInterceptor.Level.BODY
        }

        return OkHttpClient.Builder()
            .addInterceptor(authInterceptor)
            .addInterceptor(loggingInterceptor)
            .authenticator(authenticator)
            .connectTimeout(30, TimeUnit.SECONDS)
            .readTimeout(30, TimeUnit.SECONDS)
            .writeTimeout(30, TimeUnit.SECONDS)
            .build()
    }

    @Provides
    @Singleton
    fun provideRetrofit(okHttpClient: OkHttpClient, json: Json): Retrofit {
        return Retrofit.Builder()
            .baseUrl(BASE_URL)
            .client(okHttpClient)
            .addConverterFactory(json.asConverterFactory("application/json".toMediaType()))
            .build()
    }

    @Provides
    @Singleton
    fun provideGengokApi(retrofit: Retrofit): GengokApi {
        return retrofit.create(GengokApi::class.java)
    }
}
