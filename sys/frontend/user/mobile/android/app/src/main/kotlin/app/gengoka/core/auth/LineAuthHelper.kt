package app.gengoka.core.auth

import android.content.Context
import android.content.Intent
import com.linecorp.linesdk.Scope
import com.linecorp.linesdk.auth.LineAuthenticationParams
import com.linecorp.linesdk.auth.LineLoginApi

object LineAuthHelper {

    private const val CHANNEL_ID = "2009155107"

    fun getLoginIntent(context: Context): Intent {
        val params = LineAuthenticationParams.Builder()
            .scopes(listOf(Scope.PROFILE))
            .build()
        return LineLoginApi.getLoginIntent(context, CHANNEL_ID, params)
    }

    fun getAccessTokenFromResult(data: Intent?): String {
        val result = LineLoginApi.getLoginResultFromIntent(data)
        if (!result.isSuccess) {
            throw Exception("LINEログインに失敗しました: ${result.responseCode}")
        }
        return result.lineCredential?.accessToken?.tokenString
            ?: throw Exception("LINEアクセストークンの取得に失敗しました")
    }
}
