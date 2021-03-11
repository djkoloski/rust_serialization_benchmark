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
| abomonation   | 296.40 us | 35.883 us*    | †         | ‡             | 1705800   | 507151    |
| bincode       | 599.80 us | n/a           | n/a       | 4.1668 ms     | 1045784   | 374305    |
| capnp         | 1.6796 ms | 234.41 ns     | §         | ‡             | 1843240   | 537966    |
| cbor          | 1.9819 ms | n/a           | n/a       | 8.7133 ms     | 1407835   | 407372    |
| flatbuffers   | 2.6301 ms | 2.9371 ns     | §         | ‡             | 1276368   | 469962    |
| postcard      | 707.77 us | n/a           | n/a       | 4.5096 ms     | 765778    | 312739    |
| prost         | 5.2709 ms | n/a           | n/a       | 5.0111 ms     | 764951    | 269811    |
| rkyv          | 425.09 us | 1.3604 ns     | 67.754 us | 3.1887 ms     | 1065784   | 333895    |
| serde_json    | 4.3752 ms | n/a           | n/a       | 9.7127 ms     | 1827461   | 474358    |

### Comparison

Relative to best. Higher is better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|-----------|-----------|---------------|-----------|-----------|
| abomonation   | 100.00%   | 0.00%     |           |               | 44.84%    | 53.20%    |
| bincode       | 49.42%    |           |           | 76.53%        | 73.15%    | 72.08%    |
| capnp         | 17.65%    | 0.58%     |           |               | 41.50%    | 50.15%    |
| cbor          | 14.96%    |           |           | 36.60%        | 54.34%    | 66.23%    |
| flatbuffers   | 11.27%    | 46.32%    |           |               | 59.93%    | 57.41%    |
| postcard      | 41.88%    |           |           | 70.71%        | 99.89%    | 86.27%    |
| prost         | 5.62%     |           |           | 63.63%        | 100.00%   | 100.00%   |
| rkyv          | 69.73%    | 100.00%   | 100.00%   | 100.00%       | 71.77%    | 80.81%    |
| serde_json    | 6.77%     |           |           | 32.83%        | 41.86%    | 56.88%    |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three
vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

| Format / Lib  | Serialize | Access        | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|---------------|-----------|---------------|-----------|-----------|
| abomonation   | 453.63 us | 2.3861 ns*    | †         | ‡             | 6000024   | 5380837   |
| bincode       | 5.4933 ms | n/a           | n/a       | 12.416 ms     | 6000008   | 5380823   |
| capnp         | 15.660 ms | 237.09 ns     | §         | ‡             | 16000056  | 6780527   |
| cbor          | 39.159 ms | n/a           | n/a       | 69.602 ms     | 13122324  | 7527423   |
| flatbuffers   | 1.9309 ms | 2.9659 ns     | §         | ‡             | 6000024   | 5380800   |
| postcard      | 6.3688 ms | n/a           | n/a       | 8.3112 ms     | 6000003   | 5380817   |
| prost         | 32.684 ms | n/a           | n/a       | 20.402 ms     | 8750000   | 6683814   |
| rkyv          | 1.1510 ms | 1.3862 ns     | 651.45 us | 1.8980 ms     | 6000008   | 4263104   |
| serde_json    | 105.75 ms | n/a           | n/a       | 84.321 ms     | 26192883  | 9612105   |

### Comparison

Relative to best. Higher is better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|-----------|-----------|---------------|-----------|-----------|
| abomonation   | 100.00%   | 58.09%    |           |               | 100.00%   | 79.23%    |
| bincode       | 8.26%     |           |           | 15.29%        | 100.00%   | 79.23%    |
| capnp         | 2.90%     | 0.58%     |           |               | 37.50%    | 62.87%    |
| cbor          | 1.16%     |           |           | 2.73%         | 45.72%    | 56.63%    |
| flatbuffers   | 23.49%    | 46.74%    |           |               | 100.00%   | 79.23%    |
| postcard      | 7.12%     |           |           | 22.84%        | 100.00%   | 79.23%    |
| prost         | 1.39%     |           |           | 9.30%         | 68.57%    | 63.78%    |
| rkyv          | 39.41%    | 100.00%   | 100.00%   | 100.00%       | 100.00%   | 100.00%   |
| serde_json    | 0.43%     |           |           | 2.25%         | 22.91%    | 44.35%    |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

| Format / Lib  | Serialize | Access        | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|---------------|-----------|---------------|-----------|-----------|
| abomonation   | 380.52 us | 41.372 us*    | †         | ‡             | 1290592   | 392506    |
| bincode       | 813.03 us | n/a           | n/a       | 3.4005 ms     | 569975    | 240897    |
| capnp         | 862.48 us | 239.69 ns     | §         | ‡             | 835784    | 342099    |
| cbor          | 2.4551 ms | n/a           | n/a       | 8.6514 ms     | 1109821   | 347562    |
| flatbuffers   | 38.945 ms | 2.9586 ns     | §         | ‡             | 849472    | 349208    |
| postcard      | 807.62 us | n/a           | n/a       | 3.7966 ms     | 356311    | 213270    |
| prost         | 5.9403 ms | n/a           | n/a       | 4.9223 ms     | 596811    | 306728    |
| rkyv          | 891.03 us | 1.3783 ns     | 6.5060 us | 2.3929 ms     | 725176    | 334238    |
| serde_json    | 4.6727 ms | n/a           | n/a       | 10.725 ms     | 1623197   | 472162    |

### Comparison

Relative to best. Higher is better.

| Format / Lib  | Serialize | Access    | Update    | Deserialize   | Size      | Zlib      |
|---------------|-----------|-----------|-----------|---------------|-----------|-----------|
| abomonation   | 100.00%   | 0.00%     |           |               | 27.61%    | 54.34%    |
| bincode       | 46.80%    |           |           | 70.37%        | 62.51%    | 88.53%    |
| capnp         | 44.12%    | 0.58%     |           |               | 42.63%    | 62.34%    |
| cbor          | 15.50%    |           |           | 27.66%        | 32.11%    | 61.36%    |
| flatbuffers   | 0.98%     | 46.59%    |           |               | 41.94%    | 61.07%    |
| postcard      | 47.12%    |           |           | 63.03%        | 100.00%   | 100.00%   |
| prost         | 6.41%     |           |           | 48.61%        | 59.70%    | 69.53%    |
| rkyv          | 42.71%    | 100.00%   | 100.00%   | 100.00%       | 49.13%    | 63.81%    |
| serde_json    | 8.14%     |           |           | 22.31%        | 21.95%    | 45.17%    |
