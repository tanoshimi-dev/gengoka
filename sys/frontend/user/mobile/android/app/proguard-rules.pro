# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.kts.

# Keep Kotlinx Serialization
-keepattributes *Annotation*, InnerClasses
-dontnote kotlinx.serialization.AnnotationsKt

-keepclassmembers class kotlinx.serialization.json.** {
    *** Companion;
}
-keepclasseswithmembers class kotlinx.serialization.json.** {
    kotlinx.serialization.KSerializer serializer(...);
}

-keep,includedescriptorclasses class app.dev.gengoka.**$$serializer { *; }
-keepclassmembers class app.dev.gengoka.** {
    *** Companion;
}
-keepclasseswithmembers class app.dev.gengoka.** {
    kotlinx.serialization.KSerializer serializer(...);
}
