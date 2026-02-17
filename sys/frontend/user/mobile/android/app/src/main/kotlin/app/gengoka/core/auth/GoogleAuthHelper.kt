package app.gengoka.core.auth

import android.content.Context
import androidx.credentials.CredentialManager
import androidx.credentials.GetCredentialRequest
import androidx.credentials.exceptions.NoCredentialException
import com.google.android.libraries.identity.googleid.GetGoogleIdOption
import com.google.android.libraries.identity.googleid.GetSignInWithGoogleOption
import com.google.android.libraries.identity.googleid.GoogleIdTokenCredential

object GoogleAuthHelper {

    private const val WEB_CLIENT_ID =
        "274572916893-pdq3jq3551lojpl5d8qru8v1sndiq18j.apps.googleusercontent.com"

    suspend fun signIn(context: Context): String {
        val credentialManager = CredentialManager.create(context)

        // First try GetGoogleIdOption (works for returning users with saved credentials)
        try {
            val googleIdOption = GetGoogleIdOption.Builder()
                .setFilterByAuthorizedAccounts(false)
                .setServerClientId(WEB_CLIENT_ID)
                .build()

            val request = GetCredentialRequest.Builder()
                .addCredentialOption(googleIdOption)
                .build()

            val result = credentialManager.getCredential(context, request)
            val credential = GoogleIdTokenCredential.createFrom(result.credential.data)
            return credential.idToken
        } catch (e: NoCredentialException) {
            // Fall through to Sign In With Google button flow
        }

        // Fallback: GetSignInWithGoogleOption (always shows the Google Sign-In bottom sheet)
        val signInOption = GetSignInWithGoogleOption.Builder(WEB_CLIENT_ID)
            .build()

        val request = GetCredentialRequest.Builder()
            .addCredentialOption(signInOption)
            .build()

        val result = credentialManager.getCredential(context, request)
        val credential = GoogleIdTokenCredential.createFrom(result.credential.data)
        return credential.idToken
    }
}
