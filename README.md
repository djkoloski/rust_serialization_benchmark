# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

Benchmarks:

* **Serialize**: serialize data into a buffer
* **Access**: accesses a buffer as structured data (zero-copy deserialization libraries only)
* **Update**: updates a buffer as structured data (zero-copy deserialization libraries only)
* **Deserialize**: deserializes a buffer into a normal rust object
* **Size**: the size of the buffer when serialized

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

To do:
- [ ] Add Navigate benchmark that crawls through some fields of accessed data for ZCD frameworks
- [ ] Automate statistics collection and summation

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is always better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      |
|---------------|-----------|-----------|-----------|---------------|-----------|
| abomonation   | 332.02 us | 39.825 us | ---*      | ---†          | 1705800   |
| bincode       | 659.60 us | n/a       | n/a       | 4.8506 ms     | 1045784   |
| capnp         | 2.7824 ms | 271.12 ns | ---‡      | ---†          | 1843240   |
| flatbuffers   | 2.9893 ms | 3.1654 ns | ---‡      | ---†          | 1276368   |
| postcard      | 768.04 us | n/a       | n/a       | 4.9998 ms     | 765778    |
| rkyv          | 473.81 us | 1.4828 ns | 75.384 us | 3.5601 ms     | 1065784   |
| serde_json    | 4.9167 ms | n/a       | n/a       | 11.427 ms     | 1827461   |

\* *abomonation does not support buffer mutation*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *supports buffer mutation, but not in the rust implementation*

### Comparison

Relative to best. Higher is always better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      |
|---------------|-----------|-----------|-----------|---------------|-----------|
| abomonation   | 100%      | <0.01%    | ---       | ---           | 44.89%    |
| bincode       | 50.34%    | n/a       | n/a       | 73.40%        | 73.23%    |
| capnp         | 11.93%    | 0.55%     | ---       | ---           | 41.55%    |
| flatbuffers   | 11.11%    | 46.84%    | ---       | ---           | 60.00%    |
| postcard      | 43.23%    | n/a       | n/a       | 71.20%        | 100%      |
| rkyv          | 70.07%    | 100%      | 100%      | 100%          | 71.85%    |
| serde_json    | 6.75%     | n/a       | n/a       | 31.12%        | 41.90%    |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is always better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      |
|---------------|-----------|-----------|-----------|---------------|-----------|
| abomonation   | 410.58 us | 43.130 us | ---*      | ---†          | 1290592   |
| bincode       | 853.95 us | n/a       | n/a       | 3.5182 ms     | 569975    |
| capnp         | 891.27 us | 245.87 ns | ---‡      | ---†          | 835784    |
| flatbuffers   | 40.017 ms | 3.0424 ns | ---‡      | ---†          | 845264    |
| postcard      | 816.24 us | n/a       | n/a       | 3.9900 ms     | 356311    |
| rkyv          | 910.82 us | 1.4372 ns | 6.6647 us | 2.5161 ms     | 725176    |
| serde_json    | 4.8651 ms | n/a       | n/a       | 10.836 ms     | 1623197   |

\* *abomonation does not support buffer mutation*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *supports buffer mutation, but not in the rust implementation*

### Comparison

Relative to best. Higher is always better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      |
|---------------|-----------|-----------|-----------|---------------|-----------|
| abomonation   | 100%      | <0.01%    | ---       | ---           | 27.61%    |
| bincode       | 48.08%    | n/a       | n/a       | 71.52%        | 62.51%    |
| capnp         | 46.07%    | 0.58%     | ---       | ---           | 42.63%    |
| flatbuffers   | 0.01%     | 47.24%    | ---       | ---           | 42.15%    |
| postcard      | 50.30%    | n/a       | n/a       | 63.06%        | 100%      |
| rkyv          | 45.07%    | 100%      | 100%      | 100%          | 49.13%    |
| serde_json    | 8.44%     | n/a       | n/a       | 23.22%        | 21.95%    |
