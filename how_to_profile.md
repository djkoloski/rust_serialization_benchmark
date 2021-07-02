# How to profile these benchmarks

## Windows

We'll be profiling these benchmarks with Visual Studio. Make sure you have basic C/C++ payloads
installed for it.

1. Add the following beneath the `[[bench]]` line in `Cargo.toml`:
   ```
   [profile.bench]
   debug = true
   ```
2. Build the benchmark without running it with `cargo bench --no-run`
3. Locate the benchmark binary at `target/release/deps/bench-*.exe`. If there are multiple `bench`
   executables in `deps`, you can determine which one is the most recent by deleting all of them and
   rebuilding.
4. Start Visual Studio and open a new project. Browse to the `.exe` location and open it.
5. Right click on the `bench-*` project in the Solution Explorer and select `Properties`.
6. Set the `Arguments` parameter to `--bench <FILTER>` where `<FILTER>` is the filter you would pass
   to `cargo bench -- <FILTER>`.
7. Save the project properties and select `Debug > Performance Profiler...`.
8. Check `CPU Usage` and click `Start`. The benchmarks will run and upon finishing the data will be
   analyzed.
9. Select the areas of the diagnostics session when your benchmark is running to analyze.

## Other platforms

Know how to profile these benchmarks on other platforms? Open a pull request!