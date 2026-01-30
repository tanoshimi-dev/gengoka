package app.dev.gengoka.core.di;

import app.dev.gengoka.data.api.GengokApi;
import dagger.internal.DaggerGenerated;
import dagger.internal.Factory;
import dagger.internal.Preconditions;
import dagger.internal.QualifierMetadata;
import dagger.internal.ScopeMetadata;
import javax.annotation.processing.Generated;
import javax.inject.Provider;
import retrofit2.Retrofit;

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
public final class NetworkModule_ProvideGengokApiFactory implements Factory<GengokApi> {
  private final Provider<Retrofit> retrofitProvider;

  public NetworkModule_ProvideGengokApiFactory(Provider<Retrofit> retrofitProvider) {
    this.retrofitProvider = retrofitProvider;
  }

  @Override
  public GengokApi get() {
    return provideGengokApi(retrofitProvider.get());
  }

  public static NetworkModule_ProvideGengokApiFactory create(Provider<Retrofit> retrofitProvider) {
    return new NetworkModule_ProvideGengokApiFactory(retrofitProvider);
  }

  public static GengokApi provideGengokApi(Retrofit retrofit) {
    return Preconditions.checkNotNullFromProvides(NetworkModule.INSTANCE.provideGengokApi(retrofit));
  }
}
