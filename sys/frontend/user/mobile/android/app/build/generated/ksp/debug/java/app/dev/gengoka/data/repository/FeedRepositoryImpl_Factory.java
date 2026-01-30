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
public final class FeedRepositoryImpl_Factory implements Factory<FeedRepositoryImpl> {
  private final Provider<GengokApi> apiProvider;

  public FeedRepositoryImpl_Factory(Provider<GengokApi> apiProvider) {
    this.apiProvider = apiProvider;
  }

  @Override
  public FeedRepositoryImpl get() {
    return newInstance(apiProvider.get());
  }

  public static FeedRepositoryImpl_Factory create(Provider<GengokApi> apiProvider) {
    return new FeedRepositoryImpl_Factory(apiProvider);
  }

  public static FeedRepositoryImpl newInstance(GengokApi api) {
    return new FeedRepositoryImpl(api);
  }
}
