# Copilot Troubleshooting Instruction

Use this file as a reusable prompt template when asking Copilot to investigate Android / Gradle / Kotlin / KSP / Hilt build issues in this project.

Project root:
`/Users/mitakik/dev/vscode_prj/gengoka/sys/frontend/user/mobile/android`

---

## 1. Copy-paste prompt template

Copy the following prompt and replace the placeholders.

```md
You are troubleshooting an Android Gradle project.

Project root:
`/Users/mitakik/dev/vscode_prj/gengoka/sys/frontend/user/mobile/android`

Current issue:
[PASTE THE ERROR MESSAGE HERE]

Context:
- Failing command or IDE action: [e.g. Gradle Sync / :app:kspDebugKotlin / assembleDebug]
- What changed recently: [optional]
- Suspected area: [optional: Gradle plugin / dependency / AGP / Kotlin / KSP / Hilt / signing / manifest / resource / test]
- Environment: macOS, zsh shell

Please investigate and fix the issue with the following rules:

1. First reproduce the problem from CLI instead of trusting IDE hints.
2. Prefer Gradle stacktraces and actual command output over generic IDE suggestions about cache corruption.
3. Identify:
   - direct cause
   - root cause
   - contributing / related causes
4. Inspect all relevant files before editing. At minimum check as needed:
   - `build.gradle.kts`
   - `app/build.gradle.kts`
   - `settings.gradle.kts`
   - `gradle.properties`
   - `gradle/wrapper/gradle-wrapper.properties`
5. For plugin-related errors, verify version alignment among:
   - Android Gradle Plugin
   - Gradle Wrapper
   - Kotlin plugins
   - KSP
   - Hilt
6. For Kotlin/KSP/Hilt issues, verify that:
   - Kotlin plugin versions are aligned
   - KSP version matches the Kotlin line
   - Hilt plugin version and Hilt library version are aligned
   - Hilt and KSP are declared in the same scope if required
7. Make the smallest safe fix first. Do not refactor unrelated code.
8. After every code change, validate edited files and then run Gradle verification again.
9. Do not stop at “configuration fixed”; also verify the relevant compile/task phase succeeds.
10. At the end, provide a short report with:
   - what failed
   - why it failed
   - what was changed
   - what commands were run
   - what passed after the fix
   - remaining risks / follow-up items

Expected workflow:
- Reproduce with CLI
- Read relevant build files
- Form a hypothesis
- Apply minimal fix
- Re-run Gradle verification
- Summarize findings

Preferred verification commands (adjust if necessary):
```zsh
cd "/Users/mitakik/dev/vscode_prj/gengoka/sys/frontend/user/mobile/android"
./gradlew --stop
./gradlew help --stacktrace --no-daemon
./gradlew :app:kspDebugKotlin :app:compileDebugKotlin --stacktrace --no-daemon
```

If the issue is not KSP-related, choose equivalent verification commands for the failing area.
```

---

## 2. Stronger version for recurring Gradle/plugin trouble

Use this when the problem looks related to Gradle plugin loading, Kotlin, KSP, Hilt, or AGP compatibility.

```md
Investigate this Android Gradle failure as a plugin compatibility and classpath problem first, not as a cache problem.

Requirements:
- Reproduce from CLI with `./gradlew --stop` and `./gradlew help --stacktrace --no-daemon`.
- Read the relevant Gradle files before editing anything.
- Trace plugin declarations at both root and module scope.
- Check compatibility between:
  - Gradle Wrapper
  - Android Gradle Plugin
  - Kotlin Android / Compose / Serialization plugins
  - KSP
  - Hilt plugin and Hilt dependencies
- Separate:
  - immediate error
  - root cause
  - related causes
- Apply the minimum safe fix.
- Re-run the failing Gradle phase and one downstream task.
- Report the final version matrix if versions were changed.

Current issue:
[PASTE ERROR HERE]
```

---

## 3. Project-specific troubleshooting rules

These rules come from previous incidents in this repository.

1. Do not assume IDE cache corruption is the real cause.
   - Always confirm with CLI stacktrace first.

2. For this project, plugin/version drift is a real risk.
   - Check root `build.gradle.kts`
   - Check module `app/build.gradle.kts`
   - Check `gradle.properties`

3. This project has used AGP compatibility flags such as:
   - `android.builtInKotlin=false`
   - `android.newDsl=false`
   - `android.r8.optimizedResourceShrinking=false`

   Do not remove these casually. If you change them, verify the build immediately.

4. For KSP/Hilt failures, verify plugin declaration scope carefully.
   - If Hilt and KSP need to see each other’s task classes, keep them in a compatible scope.

5. When changing plugin versions, verify the full chain.
   - Kotlin Android / Serialization / Compose versions should stay aligned.
   - KSP should match the Kotlin line.
   - Hilt plugin version and Hilt dependency version should stay aligned.

6. Validation must be task-specific.
   - For configuration errors: `./gradlew help --stacktrace --no-daemon`
   - For KSP issues: `./gradlew :app:kspDebugKotlin --stacktrace --no-daemon`
   - For Kotlin compile issues: `./gradlew :app:compileDebugKotlin --stacktrace --no-daemon`

---

## 4. Minimal incident report template

Ask Copilot to end with this structure:

```md
## Summary
- Issue:
- Status:

## Direct cause
-

## Root cause
-

## Related / contributing causes
-

## Changes made
-

## Validation
- Commands run:
- Results:

## Remaining risks
-

## Recommended next actions
-
```

---

## 5. Optional: short prompt

If you want a shorter reusable prompt, use this:

```md
Troubleshoot this Android Gradle issue in `/Users/mitakik/dev/vscode_prj/gengoka/sys/frontend/user/mobile/android`.

Error:
[PASTE ERROR]

Rules:
- Reproduce from CLI first.
- Read relevant Gradle files before editing.
- Prefer stacktrace evidence over IDE cache suggestions.
- Distinguish direct cause, root cause, and related causes.
- Make the smallest safe fix.
- Re-validate with Gradle after every change.
- End with a short report including commands run and final status.
```

---

## 6. Notes for future maintenance

If this file becomes a standard team prompt, consider also adding:

- `gradle/libs.versions.toml` for centralized version management
- CI checks for:
  - `./gradlew help --stacktrace --no-daemon`
  - `./gradlew :app:kspDebugKotlin --stacktrace --no-daemon`
  - `./gradlew :app:compileDebugKotlin --stacktrace --no-daemon`
- a team playbook file such as `task/troubleshooting-playbook.md`

