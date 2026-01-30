package app.dev.gengoka.core.di;

import app.dev.gengoka.core.network.AuthInterceptor;
import app.dev.gengoka.core.network.UserIdProvider;
import dagger.internal.DaggerGenerated;
import dagger.internal.Factory;
import dagger.internal.Preconditions;
import dagger.internal.QualifierMetadata;
import dagger.internal.ScopeMetadata;
import javax.annotation.processing.Generated;
import javax.inject.Provider;

@ScopeMetadata("javax.inject.Singleton")
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
public final class NetworkModule_ProvideAuthInterceptorFactory implements Factory<AuthInterceptor> {
  private final Provider<UserIdProvider> userIdProvider;

  public NetworkModule_ProvideAuthInterceptorFactory(Provider<UserIdProvider> userIdProvider) {
    this.userIdProvider = userIdProvider;
  }

  @Override
  public AuthInterceptor get() {
    return provideAuthInterceptor(userIdProvider.get());
  }

  public static NetworkModule_ProvideAuthInterceptorFactory create(
      Provider<UserIdProvider> userIdProvider) {
    return new NetworkModule_ProvideAuthInterceptorFactory(userIdProvider);
  }

  public static AuthInterceptor provideAuthInterceptor(UserIdProvider userIdProvider) {
    return Preconditions.checkNotNullFromProvides(NetworkModule.INSTANCE.provideAuthInterceptor(userIdProvider));
  }
}
