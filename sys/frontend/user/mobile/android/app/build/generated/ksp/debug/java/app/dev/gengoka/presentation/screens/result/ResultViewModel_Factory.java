package app.dev.gengoka.presentation.screens.result;

import androidx.lifecycle.SavedStateHandle;
import app.dev.gengoka.domain.repository.AnswerRepository;
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
public final class ResultViewModel_Factory implements Factory<ResultViewModel> {
  private final Provider<SavedStateHandle> savedStateHandleProvider;

  private final Provider<AnswerRepository> answerRepositoryProvider;

  public ResultViewModel_Factory(Provider<SavedStateHandle> savedStateHandleProvider,
      Provider<AnswerRepository> answerRepositoryProvider) {
    this.savedStateHandleProvider = savedStateHandleProvider;
    this.answerRepositoryProvider = answerRepositoryProvider;
  }

  @Override
  public ResultViewModel get() {
    return newInstance(savedStateHandleProvider.get(), answerRepositoryProvider.get());
  }

  public static ResultViewModel_Factory create(Provider<SavedStateHandle> savedStateHandleProvider,
      Provider<AnswerRepository> answerRepositoryProvider) {
    return new ResultViewModel_Factory(savedStateHandleProvider, answerRepositoryProvider);
  }

  public static ResultViewModel newInstance(SavedStateHandle savedStateHandle,
      AnswerRepository answerRepository) {
    return new ResultViewModel(savedStateHandle, answerRepository);
  }
}
