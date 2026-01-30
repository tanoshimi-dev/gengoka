package app.dev.gengoka.core.network;

import dagger.internal.DaggerGenerated;
import dagger.internal.Factory;
import dagger.internal.QualifierMetadata;
import dagger.internal.ScopeMetadata;
import javax.annotation.processing.Generated;
import javax.inject.Provider;

@ScopeMetadata
@QualifierMetadata
@DaggerGenerated
@Generated(
    value = "dagger.internal.codegen.ComponentProcessor",
    comments = "https://dagger.dev"
)
@SuppressWarnings({
    "unchecked",
    "rawtypes",
    "KotlinInternal",
    "KotlinInternalInJava"
})
public final class AuthInterceptor_Factory implements Factory<AuthInterceptor> {
  private final Provider<UserIdProvider> userIdProvider;

  public AuthInterceptor_Factory(Provider<UserIdProvider> userIdProvider) {
    this.userIdProvider = userIdProvider;
  }

  @Override
  public AuthInterceptor get() {
    return newInstance(userIdProvider.get());
  }

  public static AuthInterceptor_Factory create(Provider<UserIdProvider> userIdProvider) {
    return new AuthInterceptor_Factory(userIdProvider);
  }

  public static AuthInterceptor newInstance(UserIdProvider userIdProvider) {
    return new AuthInterceptor(userIdProvider);
  }
}
