package app.dev.gengoka.presentation.screens.home;

import app.dev.gengoka.domain.repository.CategoryRepository;
import app.dev.gengoka.domain.repository.ChallengeRepository;
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
public final class HomeViewModel_Factory implements Factory<HomeViewModel> {
  private final Provider<CategoryRepository> categoryRepositoryProvider;

  private final Provider<ChallengeRepository> challengeRepositoryProvider;

  public HomeViewModel_Factory(Provider<CategoryRepository> categoryRepositoryProvider,
      Provider<ChallengeRepository> challengeRepositoryProvider) {
    this.categoryRepositoryProvider = categoryRepositoryProvider;
    this.challengeRepositoryProvider = challengeRepositoryProvider;
  }

  @Override
  public HomeViewModel get() {
    return newInstance(categoryRepositoryProvider.get(), challengeRepositoryProvider.get());
  }

  public static HomeViewModel_Factory create(
      Provider<CategoryRepository> categoryRepositoryProvider,
      Provider<ChallengeRepository> challengeRepositoryProvider) {
    return new HomeViewModel_Factory(categoryRepositoryProvider, challengeRepositoryProvider);
  }

  public static HomeViewModel newInstance(CategoryRepository categoryRepository,
      ChallengeRepository challengeRepository) {
    return new HomeViewModel(categoryRepository, challengeRepository);
  }
}
