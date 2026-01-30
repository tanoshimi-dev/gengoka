package app.dev.gengoka.presentation.screens.challenge;

import androidx.lifecycle.SavedStateHandle;
import app.dev.gengoka.domain.repository.AnswerRepository;
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
public final class ChallengeViewModel_Factory implements Factory<ChallengeViewModel> {
  private final Provider<SavedStateHandle> savedStateHandleProvider;

  private final Provider<ChallengeRepository> challengeRepositoryProvider;

  private final Provider<AnswerRepository> answerRepositoryProvider;

  public ChallengeViewModel_Factory(Provider<SavedStateHandle> savedStateHandleProvider,
      Provider<ChallengeRepository> challengeRepositoryProvider,
      Provider<AnswerRepository> answerRepositoryProvider) {
    this.savedStateHandleProvider = savedStateHandleProvider;
    this.challengeRepositoryProvider = challengeRepositoryProvider;
    this.answerRepositoryProvider = answerRepositoryProvider;
  }

  @Override
  public ChallengeViewModel get() {
    return newInstance(savedStateHandleProvider.get(), challengeRepositoryProvider.get(), answerRepositoryProvider.get());
  }

  public static ChallengeViewModel_Factory create(
      Provider<SavedStateHandle> savedStateHandleProvider,
      Provider<ChallengeRepository> challengeRepositoryProvider,
      Provider<AnswerRepository> answerRepositoryProvider) {
    return new ChallengeViewModel_Factory(savedStateHandleProvider, challengeRepositoryProvider, answerRepositoryProvider);
  }

  public static ChallengeViewModel newInstance(SavedStateHandle savedStateHandle,
      ChallengeRepository challengeRepository, AnswerRepository answerRepository) {
    return new ChallengeViewModel(savedStateHandle, challengeRepository, answerRepository);
  }
}
