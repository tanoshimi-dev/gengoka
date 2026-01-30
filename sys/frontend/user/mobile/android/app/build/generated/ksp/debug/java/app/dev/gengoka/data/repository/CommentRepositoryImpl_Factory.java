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
public final class CommentRepositoryImpl_Factory implements Factory<CommentRepositoryImpl> {
  private final Provider<GengokApi> apiProvider;

  public CommentRepositoryImpl_Factory(Provider<GengokApi> apiProvider) {
    this.apiProvider = apiProvider;
  }

  @Override
  public CommentRepositoryImpl get() {
    return newInstance(apiProvider.get());
  }

  public static CommentRepositoryImpl_Factory create(Provider<GengokApi> apiProvider) {
    return new CommentRepositoryImpl_Factory(apiProvider);
  }

  public static CommentRepositoryImpl newInstance(GengokApi api) {
    return new CommentRepositoryImpl(api);
  }
}
