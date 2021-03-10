# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

Benchmarks:

* **Serialize**: serialize data into a buffer
* **Access**: accesses a buffer as structured data (zero-copy deserialization libraries only)
* **Update**: updates a buffer as structured data (zero-copy deserialization libraries only)
* **Deserialize**: deserializes a buffer into a normal rust object
* **Size**: the size of the buffer when serialized

These benchmarks are still being developed. Known issues:

- Flatbuffers serialization speed is really low. The implementation has to have some problem
  somewhere.
- Only the `minecraft_savedata` dataset has been tested, which doesn't show how formats compare for
  small payloads like messages.

## `minecraft_savedata`

### Raw data

For operations, time per iteration; for size, bytes. Lower is always better.

| Format / Lib  | Serialize | Access        | Update    | Deserialize   | Size      |
|---------------|-----------|---------------|-----------|---------------|-----------|
| abomonation   | 410.58 us | 43.130 us     | ---*      | ---†          | 1290592   |
| bincode       | 853.95 us | n/a           | n/a       | 3.5182 ms     | 569975    |
| capnp         | 891.27 us | 245.87 ns     | ---‡      | ---†          | 835784    |
| flatbuffers   | 40.017 ms | 3.0424 ns     | ---‡      | ---†          | 845264    |
| postcard      | 816.24 us | n/a           | n/a       | 3.9900 ms     | 356311    |
| rkyv          | 910.82 us | 1.4372 ns     | 6.6647 us | 2.5161 ms     | 725176    |
| serde_json    | 4.8651 ms | n/a           | n/a       | 10.836 ms     | 1623197   |

\* *abomonation does not support buffer mutation*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *supports buffer mutation, but not in the rust implementation*

### Comparison

Relative to best. For operations, higher is better; for size, lower is better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      |
|---------------|-----------|-----------|-----------|---------------|-----------|
| abomonation   | 100%      | <0.01%    | ---       | ---           | 362.21%   |
| bincode       | 48.08%    | n/a       | n/a       | 71.52%        | 159.96%   |
| capnp         | 46.07%    | 0.06%     | ---       | ---           | 234.57%   |
| flatbuffers   | 0.01%     | 47.24%    | ---       | ---           | 237.23%   |
| postcard      | 50.30%    | n/a       | n/a       | 63.06%        | 100%      |
| rkyv          | 45.07%    | 100%      | 100%      | 100%          | 203.52%   |
| serde_json    | 8.44%     | n/a       | n/a       | 23.22%        | 455.56%   |
