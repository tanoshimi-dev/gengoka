package app.dev.gengoka.presentation.screens.myprofile;

import app.dev.gengoka.core.network.UserIdProvider;
import app.dev.gengoka.domain.repository.UserRepository;
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
public final class MyProfileViewModel_Factory implements Factory<MyProfileViewModel> {
  private final Provider<UserRepository> userRepositoryProvider;

  private final Provider<UserIdProvider> userIdProvider;

  public MyProfileViewModel_Factory(Provider<UserRepository> userRepositoryProvider,
      Provider<UserIdProvider> userIdProvider) {
    this.userRepositoryProvider = userRepositoryProvider;
    this.userIdProvider = userIdProvider;
  }

  @Override
  public MyProfileViewModel get() {
    return newInstance(userRepositoryProvider.get(), userIdProvider.get());
  }

  public static MyProfileViewModel_Factory create(Provider<UserRepository> userRepositoryProvider,
      Provider<UserIdProvider> userIdProvider) {
    return new MyProfileViewModel_Factory(userRepositoryProvider, userIdProvider);
  }

  public static MyProfileViewModel newInstance(UserRepository userRepository,
      UserIdProvider userIdProvider) {
    return new MyProfileViewModel(userRepository, userIdProvider);
  }
}
