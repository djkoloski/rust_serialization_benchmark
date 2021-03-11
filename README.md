# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

Benchmarks:

* **Serialize**: serialize data into a buffer
* **Access**: accesses a buffer as structured data (zero-copy deserialization libraries only)
* **Read**: runs through a buffer and reads fields out of it (zero-copy deserialization libraries only)
* **Update**: updates a buffer as structured data (zero-copy deserialization libraries only)
* **Deserialize**: deserializes a buffer into a normal rust object
* **Size**: the size of the buffer when serialized

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

| Format / Lib | Serialize | Access | Read | Update | Deserialize | Size | Size (zlib) |
|---|---|---|---|---|---|---|---|
| abomonation | 315.13 us | 36.773 us* | 58.999 us* | † | ‡ | 1705800 | 507971 |
| bincode | 640.51 us |  |  |  | 4.2787 ms | 1045784 | 374305 |
| capnp | 1.8558 ms | 259.95 ns | 711.84 us | § | ‡ | 1843240 | 537966 |
| cbor | 1.9698 ms |  |  |  | 8.9702 ms | 1407835 | 407372 |
| flatbuffers | 2.6780 ms | 2.9815 ns | 162.95 us | § | ‡ | 1276368 | 469962 |
| postcard | 714.70 us |  |  |  | 4.4387 ms | 765778 | 312739 |
| prost | 5.4927 ms |  |  |  | 5.1024 ms | 764951 | 269811 |
| rkyv | 422.92 us | 1.3616 ns | 18.962 us | 71.321 us | 3.2492 ms | 1065784 | 333895 |
| serde_json | 4.4054 ms |  |  |  | 10.148 ms | 1827461 | 474358 |

### Comparison

Relative to best. Higher is better.

| Format / Lib | Serialize | Access | Read | Update | Deserialize | Size | Size (zlib) |
|---|---|---|---|---|---|---|---|
| abomonation | 100.00% | 0.00%* | 32.14%* | † | ‡ | 44.84% | 53.12% |
| bincode | 49.20% |  |  |  | 75.94% | 73.15% | 72.08% |
| capnp | 16.98% | 0.52% | 2.66% | § | ‡ | 41.50% | 50.15% |
| cbor | 16.00% |  |  |  | 36.22% | 54.34% | 66.23% |
| flatbuffers | 11.77% | 45.67% | 11.64% | § | ‡ | 59.93% | 57.41% |
| postcard | 44.09% |  |  |  | 73.20% | 99.89% | 86.27% |
| prost | 5.74% |  |  |  | 63.68% | 100.00% | 100.00% |
| rkyv | 74.51% | 100.00% | 100.00% | 100.00% | 100.00% | 71.77% | 80.81% |
| serde_json | 7.15% |  |  |  | 32.02% | 41.86% | 56.88% |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three
vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

| Format / Lib | Serialize | Access | Read | Update | Deserialize | Size | Size (zlib) |
|---|---|---|---|---|---|---|---|
| abomonation | 430.61 us | 2.4135 ns* | 177.87 us* | † | ‡ | 6000024 | 5380836 |
| bincode | 7.0288 ms |  |  |  | 12.294 ms | 6000008 | 5380823 |
| capnp | 15.854 ms | 247.35 ns | 8.9442 ms | § | ‡ | 16000056 | 6780527 |
| cbor | 43.109 ms |  |  |  | 70.247 ms | 13122324 | 7527423 |
| flatbuffers | 1.9518 ms | 2.9588 ns | 152.39 us | § | ‡ | 6000024 | 5380800 |
| postcard | 6.6844 ms |  |  |  | 8.9408 ms | 6000003 | 5380817 |
| prost | 34.037 ms |  |  |  | 20.232 ms | 8750000 | 6683814 |
| rkyv | 1.1217 ms | 1.4006 ns | 172.20 us | 649.18 us | 1.9594 ms | 6000008 | 4263104 |
| serde_json | 105.86 ms |  |  |  | 83.016 ms | 26192883 | 9612105 |

### Comparison

Relative to best. Higher is better.

| Format / Lib | Serialize | Access | Read | Update | Deserialize | Size | Size (zlib) |
|---|---|---|---|---|---|---|---|
| abomonation | 100.00% | 58.03%* | 85.67%* | † | ‡ | 100.00% | 79.23% |
| bincode | 6.13% |  |  |  | 15.94% | 100.00% | 79.23% |
| capnp | 2.72% | 0.57% | 1.70% | § | ‡ | 37.50% | 62.87% |
| cbor | 1.00% |  |  |  | 2.79% | 45.72% | 56.63% |
| flatbuffers | 22.06% | 47.34% | 100.00% | § | ‡ | 100.00% | 79.23% |
| postcard | 6.44% |  |  |  | 21.92% | 100.00% | 79.23% |
| prost | 1.27% |  |  |  | 9.68% | 68.57% | 63.78% |
| rkyv | 38.39% | 100.00% | 88.50% | 100.00% | 100.00% | 100.00% | 100.00% |
| serde_json | 0.41% |  |  |  | 2.36% | 22.91% | 44.35% |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

| Format / Lib | Serialize | Access | Read | Update | Deserialize | Size | Size (zlib) |
|---|---|---|---|---|---|---|---|
| abomonation | 368.23 us | 40.823 us* | 41.413 us* | † | ‡ | 1290592 | 393696 |
| bincode | 806.73 us |  |  |  | 3.4132 ms | 569975 | 240897 |
| capnp | 863.41 us | 256.55 ns | 5.3431 us | § | ‡ | 835784 | 342099 |
| cbor | 2.4356 ms |  |  |  | 8.8797 ms | 1109821 | 347562 |
| flatbuffers | 38.683 ms | 2.9212 ns | 3.9676 us | § | ‡ | 849472 | 349208 |
| postcard | 774.37 us |  |  |  | 3.7533 ms | 356311 | 213270 |
| prost | 5.8678 ms |  |  |  | 5.4083 ms | 596811 | 306728 |
| rkyv | 843.80 us | 1.3837 ns | 282.88 ns | 6.5422 us | 2.4810 ms | 725176 | 334238 |
| serde_json | 4.3501 ms |  |  |  | 10.699 ms | 1623197 | 472162 |

### Comparison

Relative to best. Higher is better.

| Format / Lib | Serialize | Access | Read | Update | Deserialize | Size | Size (zlib) |
|---|---|---|---|---|---|---|---|
| abomonation | 100.00% | 0.00%* | 0.68%* | † | ‡ | 27.61% | 54.17% |
| bincode | 45.64% |  |  |  | 72.69% | 62.51% | 88.53% |
| capnp | 42.65% | 0.54% | 5.29% | § | ‡ | 42.63% | 62.34% |
| cbor | 15.12% |  |  |  | 27.94% | 32.11% | 61.36% |
| flatbuffers | 0.95% | 47.37% | 7.13% | § | ‡ | 41.94% | 61.07% |
| postcard | 47.55% |  |  |  | 66.10% | 100.00% | 100.00% |
| prost | 6.28% |  |  |  | 45.87% | 59.70% | 69.53% |
| rkyv | 43.64% | 100.00% | 100.00% | 100.00% | 100.00% | 49.13% | 63.81% |
| serde_json | 8.46% |  |  |  | 23.19% | 21.95% | 45.17% |

## Footnotes:

\* *abomonation requires a mutable backing to access data*

† *abomonation does not support buffer mutation*

‡ *do not provide deserialization capabilities, but the user can write their own*

§ *supports buffer mutation, but not in the rust implementation*
