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
public final class AnswerRepositoryImpl_Factory implements Factory<AnswerRepositoryImpl> {
  private final Provider<GengokApi> apiProvider;

  public AnswerRepositoryImpl_Factory(Provider<GengokApi> apiProvider) {
    this.apiProvider = apiProvider;
  }

  @Override
  public AnswerRepositoryImpl get() {
    return newInstance(apiProvider.get());
  }

  public static AnswerRepositoryImpl_Factory create(Provider<GengokApi> apiProvider) {
    return new AnswerRepositoryImpl_Factory(apiProvider);
  }

  public static AnswerRepositoryImpl newInstance(GengokApi api) {
    return new AnswerRepositoryImpl(api);
  }
}
