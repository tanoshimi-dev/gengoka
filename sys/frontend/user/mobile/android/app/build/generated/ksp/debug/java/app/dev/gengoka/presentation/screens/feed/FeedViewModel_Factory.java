package app.dev.gengoka.presentation.screens.feed;

import app.dev.gengoka.domain.repository.AnswerRepository;
import app.dev.gengoka.domain.repository.CategoryRepository;
import app.dev.gengoka.domain.repository.FeedRepository;
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
public final class FeedViewModel_Factory implements Factory<FeedViewModel> {
  private final Provider<FeedRepository> feedRepositoryProvider;

  private final Provider<CategoryRepository> categoryRepositoryProvider;

  private final Provider<AnswerRepository> answerRepositoryProvider;

  public FeedViewModel_Factory(Provider<FeedRepository> feedRepositoryProvider,
      Provider<CategoryRepository> categoryRepositoryProvider,
      Provider<AnswerRepository> answerRepositoryProvider) {
    this.feedRepositoryProvider = feedRepositoryProvider;
    this.categoryRepositoryProvider = categoryRepositoryProvider;
    this.answerRepositoryProvider = answerRepositoryProvider;
  }

  @Override
  public FeedViewModel get() {
    return newInstance(feedRepositoryProvider.get(), categoryRepositoryProvider.get(), answerRepositoryProvider.get());
  }

  public static FeedViewModel_Factory create(Provider<FeedRepository> feedRepositoryProvider,
      Provider<CategoryRepository> categoryRepositoryProvider,
      Provider<AnswerRepository> answerRepositoryProvider) {
    return new FeedViewModel_Factory(feedRepositoryProvider, categoryRepositoryProvider, answerRepositoryProvider);
  }

  public static FeedViewModel newInstance(FeedRepository feedRepository,
      CategoryRepository categoryRepository, AnswerRepository answerRepository) {
    return new FeedViewModel(feedRepository, categoryRepository, answerRepository);
  }
}
