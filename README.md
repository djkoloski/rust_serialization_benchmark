# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

These benchmarks are still being developed. Known issues:

- Flatbuffers serialization speed is really low. The implementation has to have some problem
  somewhere.
- Only the `minecraft_savedata` dataset has been tested, which doesn't show how formats compare for
  small payloads like messages.

## `minecraft_savedata`

### Raw data

For operations, time per iteration; for size, bytes. Lower is always better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      |
|---------------|-----------|-----------|-----------|---------------|-----------|
| bincode       | 871.63 us | n/a       | 4.5623 ms | 3.5914 ms     | 569975    |
| flatbuffers   | 40.718 ms | 3.0615 ns | n/a       | n/a           | 845264    |
| rkyv          | 883.87 us | 1.4159 ns | 6.6869 us | 2.6380 ms     | 725176    |

### Comparison

Relative to best. For operations, higher is better; for size, lower is better.

| Format / Lib  | Serialize all | Access    | Update all    | Deserialize all   | Size (bytes)  |
|---------------|---------------|-----------|---------------|-------------------|---------------|
| bincode       | 100%          | n/a       | 0.11%         | 73.23%            | 100%          |
| flatbuffers   | 2.14%         | 46.25%    | n/a           | n/a               | 148.30%       |
| rkyv          | 98.62%        | 100%      | 100%          | 100%              | 127.23%       |
