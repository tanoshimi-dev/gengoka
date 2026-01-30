package app.dev.gengoka.core.network;

import androidx.datastore.core.DataStore;
import androidx.datastore.preferences.core.Preferences;
import dagger.internal.DaggerGenerated;
import dagger.internal.Factory;
import dagger.internal.QualifierMetadata;
import dagger.internal.ScopeMetadata;
import javax.annotation.processing.Generated;
import javax.inject.Provider;

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
public final class UserIdProvider_Factory implements Factory<UserIdProvider> {
  private final Provider<DataStore<Preferences>> dataStoreProvider;

  public UserIdProvider_Factory(Provider<DataStore<Preferences>> dataStoreProvider) {
    this.dataStoreProvider = dataStoreProvider;
  }

  @Override
  public UserIdProvider get() {
    return newInstance(dataStoreProvider.get());
  }

  public static UserIdProvider_Factory create(Provider<DataStore<Preferences>> dataStoreProvider) {
    return new UserIdProvider_Factory(dataStoreProvider);
  }

  public static UserIdProvider newInstance(DataStore<Preferences> dataStore) {
    return new UserIdProvider(dataStore);
  }
}
