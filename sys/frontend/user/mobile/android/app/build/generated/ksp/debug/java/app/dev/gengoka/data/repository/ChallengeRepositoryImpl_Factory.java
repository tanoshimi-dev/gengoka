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
public final class ChallengeRepositoryImpl_Factory implements Factory<ChallengeRepositoryImpl> {
  private final Provider<GengokApi> apiProvider;

  public ChallengeRepositoryImpl_Factory(Provider<GengokApi> apiProvider) {
    this.apiProvider = apiProvider;
  }

  @Override
  public ChallengeRepositoryImpl get() {
    return newInstance(apiProvider.get());
  }

  public static ChallengeRepositoryImpl_Factory create(Provider<GengokApi> apiProvider) {
    return new ChallengeRepositoryImpl_Factory(apiProvider);
  }

  public static ChallengeRepositoryImpl newInstance(GengokApi api) {
    return new ChallengeRepositoryImpl(api);
  }
}
