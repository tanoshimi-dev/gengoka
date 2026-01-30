package app.dev.gengoka.data.repository;

import app.dev.gengoka.data.api.GengokApi;
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
public final class UserRepositoryImpl_Factory implements Factory<UserRepositoryImpl> {
  private final Provider<GengokApi> apiProvider;

  public UserRepositoryImpl_Factory(Provider<GengokApi> apiProvider) {
    this.apiProvider = apiProvider;
  }

  @Override
  public UserRepositoryImpl get() {
    return newInstance(apiProvider.get());
  }

  public static UserRepositoryImpl_Factory create(Provider<GengokApi> apiProvider) {
    return new UserRepositoryImpl_Factory(apiProvider);
  }

  public static UserRepositoryImpl newInstance(GengokApi api) {
    return new UserRepositoryImpl(api);
  }
}
