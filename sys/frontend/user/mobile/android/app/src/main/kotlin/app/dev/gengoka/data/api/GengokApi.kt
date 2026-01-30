package app.dev.gengoka.data.api

import app.dev.gengoka.data.dto.*
import retrofit2.http.*

interface GengokApi {

    // Categories
    @GET("categories")
    suspend fun getCategories(): ApiResponse<List<CategoryDto>>

    @GET("categories/{id}")
    suspend fun getCategory(@Path("id") id: String): ApiResponse<CategoryDto>

    @GET("categories/{id}/challenges")
    suspend fun getChallengesByCategory(
        @Path("id") categoryId: String,
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null
    ): ApiResponse<List<ChallengeWithCategoryDto>>

    // Challenges
    @GET("challenges")
    suspend fun getChallenges(
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null
    ): ApiResponse<List<ChallengeDto>>

    @GET("challenges/daily")
    suspend fun getDailyChallenges(): ApiResponse<List<ChallengeWithCategoryDto>>

    @GET("challenges/{id}")
    suspend fun getChallenge(@Path("id") id: String): ApiResponse<ChallengeWithCategoryDto>

    @GET("challenges/{id}/answers")
    suspend fun getChallengeAnswers(
        @Path("id") challengeId: String,
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null,
        @Query("sort") sort: String? = null
    ): ApiResponse<List<AnswerWithUserDto>>

    @POST("challenges/{id}/answers")
    suspend fun createAnswer(
        @Path("id") challengeId: String,
        @Body request: CreateAnswerRequest
    ): ApiResponse<AnswerWithUserDto>

    // Answers
    @GET("answers/{id}")
    suspend fun getAnswer(@Path("id") id: String): ApiResponse<AnswerWithDetailsDto>

    @DELETE("answers/{id}")
    suspend fun deleteAnswer(@Path("id") id: String): ApiResponse<Unit?>

    @POST("answers/{id}/like")
    suspend fun likeAnswer(@Path("id") id: String): ApiResponse<Unit?>

    @DELETE("answers/{id}/like")
    suspend fun unlikeAnswer(@Path("id") id: String): ApiResponse<Unit?>

    @GET("answers/{id}/comments")
    suspend fun getAnswerComments(
        @Path("id") answerId: String,
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null
    ): ApiResponse<List<CommentWithUserDto>>

    @POST("answers/{id}/comments")
    suspend fun createComment(
        @Path("id") answerId: String,
        @Body request: CreateCommentRequest
    ): ApiResponse<CommentWithUserDto>

    // Comments
    @DELETE("comments/{id}")
    suspend fun deleteComment(@Path("id") id: String): ApiResponse<Unit?>

    // Users
    @POST("users")
    suspend fun createUser(@Body request: CreateUserRequest): ApiResponse<UserDto>

    @GET("users/{id}")
    suspend fun getUser(@Path("id") id: String): ApiResponse<UserProfileDto>

    @PUT("users/{id}")
    suspend fun updateUser(
        @Path("id") id: String,
        @Body request: UpdateUserRequest
    ): ApiResponse<UserDto>

    @GET("users/{id}/answers")
    suspend fun getUserAnswers(
        @Path("id") userId: String,
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null
    ): ApiResponse<List<AnswerWithDetailsDto>>

    @POST("users/{id}/follow")
    suspend fun followUser(@Path("id") id: String): ApiResponse<Unit?>

    @DELETE("users/{id}/follow")
    suspend fun unfollowUser(@Path("id") id: String): ApiResponse<Unit?>

    @GET("users/{id}/followers")
    suspend fun getFollowers(
        @Path("id") userId: String,
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null
    ): ApiResponse<List<UserSummaryDto>>

    @GET("users/{id}/following")
    suspend fun getFollowing(
        @Path("id") userId: String,
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null
    ): ApiResponse<List<UserSummaryDto>>

    // Feed & Rankings
    @GET("feed")
    suspend fun getFeed(
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null,
        @Query("filter") filter: String? = null,
        @Query("category_id") categoryId: String? = null
    ): ApiResponse<List<AnswerWithDetailsDto>>

    @GET("trending")
    suspend fun getTrending(
        @Query("page") page: Int? = null,
        @Query("page_size") pageSize: Int? = null
    ): ApiResponse<List<AnswerWithDetailsDto>>

    @GET("rankings/daily")
    suspend fun getDailyRanking(): ApiResponse<List<AnswerWithUserDto>>

    @GET("rankings/weekly")
    suspend fun getWeeklyRanking(): ApiResponse<List<AnswerWithUserDto>>

    @GET("rankings/all-time")
    suspend fun getAllTimeRanking(): ApiResponse<List<AnswerWithUserDto>>
}
