# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

Benchmarks:

* **Serialize**: serialize data into a buffer
* **Access**: accesses a buffer as structured data (zero-copy deserialization libraries only)
* **Update**: updates a buffer as structured data (zero-copy deserialization libraries only)
* **Deserialize**: deserializes a buffer into a normal rust object
* **Size**: the size of the buffer when serialized

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

To do:
- [ ] Add Navigate benchmark that crawls through some fields of accessed data for ZCD frameworks
- [ ] Automate statistics collection and summation


## Footnotes:

\* *abomonation requires a mutable backing to access data*

† *abomonation does not support buffer mutation*

‡ *do not provide deserialization capabilities, but the user can write their own*

§ *supports buffer mutation, but not in the rust implementation*

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

| Format / Lib  | Serialize | Access        | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|---------------|-----------|---------------|-----------|-----------|
| abomonation   | 332.02 us | 39.825 us*    | ---†      | ---‡          | 1705800   | 516849    |
| bincode       | 659.60 us | n/a           | n/a       | 4.8506 ms     | 1045784   | 374305    |
| capnp         | 2.7824 ms | 271.12 ns     | ---§      | ---‡          | 1843240   | 537966    |
| cbor          | 2.1755 ms | n/a           | n/a       | 10.127 ms     | 1407835   | 407372    |
| flatbuffers   | 2.9893 ms | 3.1654 ns     | ---§      | ---‡          | 1276368   | 469962    |
| postcard      | 768.04 us | n/a           | n/a       | 4.9998 ms     | 765778    | 312739    |
| rkyv          | 473.81 us | 1.4828 ns     | 75.384 us | 3.5601 ms     | 1065784   | 459664    |
| serde_json    | 4.9167 ms | n/a           | n/a       | 11.427 ms     | 1827461   | 474358    |

### Comparison

Relative to best. Higher is better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|-----------|-----------|---------------|-----------|-----------|
| abomonation   | 100%      | <0.01%    | ---       | ---           | 44.89%    | 60.51%    |
| bincode       | 50.34%    | n/a       | n/a       | 73.40%        | 73.23%    | 83.55%    |
| capnp         | 11.93%    | 0.55%     | ---       | ---           | 41.55%    | 58.13%    |
| cbor          | 15.26%    | n/a       | n/a       | 35.15%        | 54.39%    | 76.77%    |
| flatbuffers   | 11.11%    | 46.84%    | ---       | ---           | 60.00%    | 66.55%    |
| postcard      | 43.23%    | n/a       | n/a       | 71.20%        | 100%      | 100%      |
| rkyv          | 70.07%    | 100%      | 100%      | 100%          | 71.85%    | 68.04%    |
| serde_json    | 6.75%     | n/a       | n/a       | 31.12%        | 41.90%    | 65.93%    |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three
vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

| Format / Lib  | Serialize | Access        | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|---------------|-----------|---------------|-----------|-----------|
| abomonation   | 455.51 us | 2.4902 ns*    | ---†      | ---‡          | 6000024   | 5380836   |
| bincode       | 6.0056 ms | n/a           | n/a       | 12.903 ms     | 6000008   | 5380823   |
| capnp         | 16.239 ms | 260.16 ns     | ---§      | ---‡          | 16000056  | 6780527   |
| cbor          | 43.186 ms | n/a           | n/a       | 73.951 ms     | 13122324  | 7527423   |
| flatbuffers   | 2.0310 ms | 3.0985 ns     | ---§      | ---‡          | 6000024   | 5380800   |
| postcard      | 7.0779 ms | n/a           | n/a       | 10.640 ms     | 6000003   | 5380817   |
| rkyv          | 1.2467 ms | 1.4633 ns     | 6.6647 us | 1.9545 ms     | 6000008   | 5380822   |
| serde_json    | 109.93 ms | n/a           | n/a       | 93.071 ms     | 26192883  | 9612105   |

### Comparison

Relative to best. Higher is better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|-----------|-----------|---------------|-----------|-----------|
| abomonation   | 100%      | 58.76%    | ---       | ---           | >99.99%   | >99.99%   |
| bincode       | 7.58%     | n/a       | n/a       | 15.15%        | >99.99%   | >99.99%   |
| capnp         | 2.81%     | 0.56%     | ---       | ---           | 37.50%    | 79.36%    |
| cbor          | 1.05%     | n/a       | n/a       | 2.64%         | 45.72%    | 71.48%    |
| flatbuffers   | 22.43%    | 47.23%    | ---       | ---           | >99.99%   | 100%      |
| postcard      | 6.44%     | n/a       | n/a       | 18.37%        | 100%      | >99.99%   |
| rkyv          | 36.54%    | 100%      | 100%      | 100%          | >99.99%   | >99.99%   |
| serde_json    | 0.41%     | n/a       | n/a       | 2.10%         | 22.91%    | 55.98%    |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

| Format / Lib  | Serialize | Access        | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|---------------|-----------|---------------|-----------|-----------|
| abomonation   | 410.58 us | 43.130 us*    | ---†      | ---‡          | 1290592   | 389753    |
| bincode       | 853.95 us | n/a           | n/a       | 3.5182 ms     | 569975    | 240897    |
| capnp         | 891.27 us | 245.87 ns     | ---§      | ---‡          | 835784    | 342099    |
| cbor          | 2.7159 ms | n/a           | n/a       | 9.4193 ms     | 1109821   | 347562    |
| flatbuffers   | 40.017 ms | 3.0424 ns     | ---§      | ---‡          | 845264    | 349208    |
| postcard      | 816.24 us | n/a           | n/a       | 3.9900 ms     | 356311    | 213270    |
| rkyv          | 910.82 us | 1.4372 ns     | 6.6647 us | 2.5161 ms     | 725176    | 346116    |
| serde_json    | 4.8651 ms | n/a           | n/a       | 10.836 ms     | 1623197   | 472162    |

### Comparison

Relative to best. Higher is better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|-----------|-----------|---------------|-----------|-----------|
| abomonation   | 100%      | <0.01%    | ---       | ---           | 27.61%    | 54.72%    |
| bincode       | 48.08%    | n/a       | n/a       | 71.52%        | 62.51%    | 88.53%    |
| capnp         | 46.07%    | 0.58%     | ---       | ---           | 42.63%    | 62.34%    |
| cbor          | 15.12%    | n/a       | n/a       | 26.71%        | 32.11%    | 61.36%    |
| flatbuffers   | 0.01%     | 47.24%    | ---       | ---           | 42.15%    | 61.07%    |
| postcard      | 50.30%    | n/a       | n/a       | 63.06%        | 100%      | 100%      |
| rkyv          | 45.07%    | 100%      | 100%      | 100%          | 49.13%    | 61.62%    |
| serde_json    | 8.44%     | n/a       | n/a       | 23.22%        | 21.95%    | 45.17%    |
