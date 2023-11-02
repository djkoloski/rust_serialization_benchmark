<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

## [Interactive site](https://djkoloski.github.io/rust_serialization_benchmark/)

Calculate the number of messages per second that can be sent/received with various rust serialization frameworks and compression libraries.
[Documentation](pages/README.md)

## Format

All tests benchmark the following properties (time or size):

* **Serialize**: serialize data into a buffer
* **Deserialize**: deserializes a buffer into a normal rust object
* **Size**: the size of the buffer when serialized
* **Zlib**: the size of the buffer after zlib compression

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2023-11-2 12:2:42

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 283.21 µs | <span title="unvalidated">*3.0393 ms\**</span> | 1705800 | 530414 | 403697 |
| [alkahest 0.1.5][alkahest] | 306.92 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 558.15 µs | 3.7570 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.5.0][bitcode] | 647.95 µs | 3.8469 ms | 703664 | 317711 | 273622 |
| [borsh 1.1.1][borsh] | 559.21 µs | 4.0979 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 3.2364 ms | 12.358 ms | 1924682 | 532821 | 376270 |
| [capnp 0.18.3][capnp] | 783.14 µs | † | 1443216 | 513986 | 428649 |
| [cbor4ii 0.3.1][cbor4ii] | 933.42 µs | 8.0616 ms | 1407835 | 403440 | 324081 |
| [ciborium 0.2.1][ciborium] | 3.4895 ms | 13.494 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 954.94 µs | 4.3723 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 2.4802 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.7167 ms | 4.3503 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.6046 ms | 7.1545 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.35][nanoserde] | 366.36 µs | 3.8179 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 708.05 µs | 4.2412 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.8][postcard] | 488.72 µs | 3.8276 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 3.6423 ms | 9.8657 ms | 971922 | 372513 | 304122 |
| [prost 0.12.1][prost] | <span title="encode">*543.10 µs\**</span> <span title="populate + encode">*3.8562 ms\**</span> | 4.9595 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 387.91 µs | <span title="unvalidated">*2.8573 ms\**</span> <span title="validated upfront with error">*3.8544 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.5626 ms | 5.3907 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 20.447 ms | 26.023 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.2][savefile] | 292.18 µs | 3.9024 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 803.28 µs | 3.8633 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.2711 ms | 8.0472 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.108][serde_json] | 5.8537 ms | 9.9404 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.13.4][simd-json] | 2.7248 ms | 8.3125 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 278.69 µs | 3.1855 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*72.847 µs\**</span> | <span title="unvalidated">*109.73 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6767 ns\**</span> | <span title="validated on-demand with panic">*42.113 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*117.01 ns\**</span> | <span title="validated on-demand with error">*243.53 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.4918 ns\**</span> <span title="validated upfront with error">*2.4829 ms\**</span> | <span title="unvalidated">*99.755 µs\**</span> <span title="validated upfront with error">*2.9545 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5957 ns\**</span> <span title="validated upfront with error">*929.14 µs\**</span> | <span title="unvalidated">*22.444 µs\**</span> <span title="validated upfront with error">*939.62 µs\**</span> | 37.360 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 98.40% | <span title="unvalidated">*94.01%\**</span> | 41.25% | 50.55% | 56.46% |
| [alkahest 0.1.5][alkahest] | 90.80% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 49.93% | 76.05% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.5.0][bitcode] | 43.01% | 74.28% | 100.00% | 84.40% | 83.31% |
| [borsh 1.1.1][borsh] | 49.84% | 69.73% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.61% | 23.12% | 36.56% | 50.32% | 60.58% |
| [capnp 0.18.3][capnp] | 35.59% | † | 48.76% | 52.17% | 53.18% |
| [cbor4ii 0.3.1][cbor4ii] | 29.86% | 35.44% | 49.98% | 66.46% | 70.34% |
| [ciborium 0.2.1][ciborium] | 7.99% | 21.17% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 29.18% | 65.35% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 11.24% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 16.23% | 65.68% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.24% | 39.94% | 85.95% | 80.63% | 79.84% |
| [nanoserde 0.1.35][nanoserde] | 76.07% | 74.84% | 67.29% | 71.86% | 73.12% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 39.36% | 67.37% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.8][postcard] | 57.02% | 74.65% | 97.06% | 88.67% | 89.83% |
| [pot 3.0.0][pot] | 7.65% | 28.96% | 72.40% | 71.98% | 74.95% |
| [prost 0.12.1][prost] | <span title="encode">*51.31%\**</span> <span title="populate + encode">*7.23%\**</span> | 57.61% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 71.84% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*74.13%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.2][rmp-serde] | 17.84% | 53.00% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.1][ron] | 1.36% | 10.98% | 43.77% | 59.70% | 65.18% |
| [savefile 0.16.2][savefile] | 95.38% | 73.22% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 34.69% | 73.96% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 12.27% | 35.51% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.108][serde_json] | 4.76% | 28.74% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.13.4][simd-json] | 10.23% | 34.37% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 100.00% | 89.70% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*20.45%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*59.61%\**</span> | <span title="validated on-demand with panic">*53.29%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.36%\**</span> | <span title="validated on-demand with error">*9.22%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*45.70%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*22.50%\**</span> <span title="validated upfront with error">*0.76%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.39%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 506.13 µs | <span title="unvalidated">*487.39 µs\**</span> | 6000024 | 5378513 | 5345891 |
| [alkahest 0.1.5][alkahest] | 553.56 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 4.8224 ms | 6.2288 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.5.0][bitcode] | 5.8093 ms | 10.186 ms | 4688054 | 4688491 | 4688168 |
| [borsh 1.1.1][borsh] | 6.5473 ms | 6.4270 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 61.508 ms | 131.89 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.18.3][capnp] | 10.214 ms | † | 14000088 | 7130367 | 6051062 |
| [cbor4ii 0.3.1][cbor4ii] | 15.808 ms | 88.918 ms | 13125016 | 7524114 | 6757967 |
| [ciborium 0.2.1][ciborium] | 84.497 ms | 127.42 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 9.3574 ms | 10.486 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.4935 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 23.056 ms | 12.567 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 185.04 ms | 44.065 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.35][nanoserde] | 2.0955 ms | 1.5629 ms | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 4.9491 ms | 6.6305 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.8][postcard] | 835.24 µs | 1.9325 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 59.420 ms | 100.28 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.12.1][prost] | <span title="encode">*8.5271 ms\**</span> <span title="populate + encode">*11.327 ms\**</span> | 20.558 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 963.65 µs | <span title="unvalidated">*570.20 µs\**</span> <span title="validated upfront with error">*555.45 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 13.702 ms | 24.907 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 283.43 ms | 463.87 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.2][savefile] | 502.72 µs | 480.88 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.9023 ms | 6.2447 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 45.463 ms | 64.463 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.108][serde_json] | 129.01 ms | 104.14 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.13.4][simd-json] | 75.794 ms | 151.41 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 486.13 µs | 494.69 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*3.0190 ns\**</span> | <span title="unvalidated">*263.38 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.8504 ns\**</span> | <span title="validated on-demand with panic">*98.228 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*206.71 ns\**</span> | <span title="validated on-demand with error">*3.3902 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.6935 ns\**</span> <span title="validated upfront with error">*53.220 ns\**</span> | <span title="unvalidated">*75.183 µs\**</span> <span title="validated upfront with error">*97.619 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5953 ns\**</span> <span title="validated upfront with error">*9.3781 ns\**</span> | <span title="unvalidated">*48.824 µs\**</span> <span title="validated upfront with error">*50.751 µs\**</span> | 368.89 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 96.05% | <span title="unvalidated">*98.66%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 87.82% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 10.08% | 7.72% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.5.0][bitcode] | 8.37% | 4.72% | 100.00% | 100.00% | 100.00% |
| [borsh 1.1.1][borsh] | 7.42% | 7.48% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.79% | 0.36% | 20.37% | 50.89% | 62.53% |
| [capnp 0.18.3][capnp] | 4.76% | † | 33.49% | 65.75% | 77.48% |
| [cbor4ii 0.3.1][cbor4ii] | 3.08% | 0.54% | 35.72% | 62.31% | 69.37% |
| [ciborium 0.2.1][ciborium] | 0.58% | 0.38% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 5.20% | 4.59% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 32.55% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 2.11% | 3.83% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.26% | 1.09% | 57.70% | 72.20% | 73.40% |
| [nanoserde 0.1.35][nanoserde] | 23.20% | 30.77% | 78.13% | 87.17% | 87.70% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 9.82% | 7.25% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.8][postcard] | 58.20% | 24.88% | 78.13% | 87.17% | 87.70% |
| [pot 3.0.0][pot] | 0.82% | 0.48% | 46.31% | 68.80% | 68.42% |
| [prost 0.12.1][prost] | <span title="encode">*5.70%\**</span> <span title="populate + encode">*4.29%\**</span> | 2.34% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 50.45% | <span title="unvalidated">*84.34%\**</span> <span title="validated upfront with error">*86.57%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.2][rmp-serde] | 3.55% | 1.93% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.1][ron] | 0.17% | 0.10% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.2][savefile] | 96.70% | 100.00% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 7.04% | 7.70% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 1.07% | 0.75% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.108][serde_json] | 0.38% | 0.46% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.13.4][simd-json] | 0.64% | 0.32% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 97.21% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*52.84%\**</span> | <span title="unvalidated">*18.54%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.97%\**</span> | <span title="validated on-demand with panic">*49.70%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*0.77%\**</span> | <span title="validated on-demand with error">*1.44%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*43.19%\**</span> <span title="validated upfront with error">*3.00%\**</span> | <span title="unvalidated">*64.94%\**</span> <span title="validated upfront with error">*50.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*17.01%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*96.20%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 273.18 µs | <span title="unvalidated">*2.3302 ms\**</span> | 1290592 | 394977 | 331561 |
| [alkahest 0.1.5][alkahest] | 328.21 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 702.40 µs | 3.0510 ms | 569975 | 240525 | 232423 |
| [bitcode 0.5.0][bitcode] | 479.32 µs | 3.1413 ms | 322798 | 214279 | 201247 |
| [borsh 1.1.1][borsh] | 656.99 µs | 3.0077 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 4.2406 ms | 13.884 ms | 1619653 | 502185 | 328399 |
| [capnp 0.18.3][capnp] | 704.69 µs | † | 803896 | 335606 | 280851 |
| [cbor4ii 0.3.1][cbor4ii] | 1.0722 ms | 7.9339 ms | 1109831 | 344745 | 274514 |
| [ciborium 0.2.1][ciborium] | 4.1039 ms | 12.369 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 1.0549 ms | 4.2419 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 4.4808 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.4281 ms | 4.5268 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.3488 ms | 6.0057 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.35][nanoserde] | 422.24 µs | 3.3122 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 806.72 µs | 3.3304 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.8][postcard] | 554.16 µs | 3.1756 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 3.1897 ms | 8.9867 ms | 599125 | 299158 | 247693 |
| [prost 0.12.1][prost] | <span title="encode">*1.5108 ms\**</span> <span title="populate + encode">*4.4014 ms\**</span> | 5.7685 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 448.04 µs | <span title="unvalidated">*2.2890 ms\**</span> <span title="validated upfront with error">*3.1046 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.7813 ms | 4.6210 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 12.951 ms | 27.035 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.2][savefile] | 304.44 µs | 3.1120 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 996.59 µs | 3.8212 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.3485 ms | 7.8069 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.108][serde_json] | 5.6627 ms | 11.366 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.13.4][simd-json] | 3.1740 ms | 7.9578 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 433.49 µs | 2.9234 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*76.846 µs\**</span> | <span title="unvalidated">*81.082 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7294 ns\**</span> | <span title="validated on-demand with panic">*9.1958 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*116.34 ns\**</span> | <span title="validated on-demand with error">*728.70 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.5803 ns\**</span> <span title="validated upfront with error">*2.6682 ms\**</span> | <span title="unvalidated">*3.3089 µs\**</span> <span title="validated upfront with error">*2.6745 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5974 ns\**</span> <span title="validated upfront with error">*777.18 µs\**</span> | <span title="unvalidated">*210.89 ns\**</span> <span title="validated upfront with error">*770.84 µs\**</span> | 2.9035 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*98.23%\**</span> | 25.01% | 53.92% | 59.86% |
| [alkahest 0.1.5][alkahest] | 83.23% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 38.89% | 75.02% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.5.0][bitcode] | 56.99% | 72.87% | 100.00% | 99.39% | 98.63% |
| [borsh 1.1.1][borsh] | 41.58% | 76.10% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.44% | 16.49% | 19.93% | 42.41% | 60.44% |
| [capnp 0.18.3][capnp] | 38.77% | † | 40.15% | 63.46% | 70.67% |
| [cbor4ii 0.3.1][cbor4ii] | 25.48% | 28.85% | 29.09% | 61.78% | 72.31% |
| [ciborium 0.2.1][ciborium] | 6.66% | 18.51% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 25.90% | 53.96% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 6.10% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 19.13% | 50.57% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.72% | 38.11% | 71.77% | 84.37% | 85.88% |
| [nanoserde 0.1.35][nanoserde] | 64.70% | 69.11% | 56.83% | 88.77% | 85.40% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 33.86% | 68.73% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.8][postcard] | 49.30% | 72.08% | 87.84% | 95.97% | 95.73% |
| [pot 3.0.0][pot] | 8.56% | 25.47% | 53.88% | 71.19% | 80.13% |
| [prost 0.12.1][prost] | <span title="encode">*18.08%\**</span> <span title="populate + encode">*6.21%\**</span> | 39.68% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 60.97% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.73%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.2][rmp-serde] | 15.34% | 49.53% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.1][ron] | 2.11% | 8.47% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.2][savefile] | 89.73% | 73.55% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 27.41% | 59.90% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.63% | 29.32% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.108][serde_json] | 4.82% | 20.14% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.13.4][simd-json] | 8.61% | 28.76% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 63.02% | 78.30% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.26%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.53%\**</span> | <span title="validated on-demand with panic">*2.29%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.37%\**</span> | <span title="validated on-demand with error">*28.94%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.62%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.37%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 692.87 µs | <span title="unvalidated">*3.8866 ms\**</span> | 2984682 | 1427900 | 1300500 |
| [alkahest 0.1.5][alkahest] | 1.0023 ms | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 4.1188 ms | 6.5127 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.5.0][bitcode] | 1.6821 ms | 5.6473 ms | 870693 | 866738 | 870720 |
| [borsh 1.1.1][borsh] | 3.1786 ms | 4.5239 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 35.677 ms | 74.101 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.18.3][capnp] | 3.0645 ms | † | 2664040 | 1511895 | 1212087 |
| [cbor4ii 0.3.1][cbor4ii] | 4.8403 ms | 34.780 ms | 5878791 | 1655835 | 1431390 |
| [ciborium 0.2.1][ciborium] | 26.770 ms | 59.952 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.9975 ms | 12.714 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 6.6346 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 3.0389 ms | 10.539 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 47.643 ms | 27.976 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.35][nanoserde] | 1.4007 ms | 5.6800 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 3.4098 ms | 5.6246 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.8][postcard] | 2.4313 ms | 7.4439 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 23.032 ms | 47.389 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.12.1][prost] | <span title="encode">*6.5043 ms\**</span> <span title="populate + encode">*13.092 ms\**</span> | 14.812 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 2.0683 ms | <span title="unvalidated">*3.6737 ms\**</span> <span title="validated upfront with error">*4.8934 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 12.254 ms | 16.889 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 57.697 ms | 148.14 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.2][savefile] | 1.3489 ms | 4.7070 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 5.8960 ms | 8.2438 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 14.074 ms | 33.979 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.108][serde_json] | 33.175 ms | 50.738 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.13.4][simd-json] | 17.063 ms | 52.157 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 992.94 µs | 4.2370 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*174.92 µs\**</span> | <span title="unvalidated">*178.88 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7635 ns\**</span> | <span title="validated on-demand with panic">*908.37 ns\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*122.84 ns\**</span> | <span title="validated on-demand with error">*1.2980 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.6508 ns\**</span> <span title="validated upfront with error">*5.7142 ms\**</span> | <span title="unvalidated">*5.1875 µs\**</span> <span title="validated upfront with error">*5.8587 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6700 ns\**</span> <span title="validated upfront with error">*935.73 µs\**</span> | <span title="unvalidated">*554.82 ns\**</span> <span title="validated upfront with error">*913.47 µs\**</span> | 961.62 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*94.52%\**</span> | 29.17% | 60.70% | 66.95% |
| [alkahest 0.1.5][alkahest] | 69.13% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 16.82% | 56.41% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.5.0][bitcode] | 41.19% | 65.05% | 100.00% | 100.00% | 100.00% |
| [borsh 1.1.1][borsh] | 21.80% | 81.21% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 1.94% | 4.96% | 8.68% | 30.59% | 54.39% |
| [capnp 0.18.3][capnp] | 22.61% | † | 32.68% | 57.33% | 71.84% |
| [cbor4ii 0.3.1][cbor4ii] | 14.31% | 10.56% | 14.81% | 52.34% | 60.83% |
| [ciborium 0.2.1][ciborium] | 2.59% | 6.13% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 9.90% | 28.89% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 10.44% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 22.80% | 34.86% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.45% | 13.13% | 50.37% | 69.47% | 70.60% |
| [nanoserde 0.1.35][nanoserde] | 49.47% | 64.68% | 49.18% | 78.20% | 84.54% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 20.32% | 65.31% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.8][postcard] | 28.50% | 49.35% | 68.04% | 81.90% | 85.64% |
| [pot 3.0.0][pot] | 3.01% | 7.75% | 34.21% | 59.88% | 68.65% |
| [prost 0.12.1][prost] | <span title="encode">*10.65%\**</span> <span title="populate + encode">*5.29%\**</span> | 24.80% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 33.50% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*75.07%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.2][rmp-serde] | 5.65% | 21.75% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.1][ron] | 1.20% | 2.48% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.2][savefile] | 51.37% | 78.05% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 11.75% | 44.56% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 4.92% | 10.81% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.108][serde_json] | 2.09% | 7.24% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.13.4][simd-json] | 4.06% | 7.04% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 69.78% | 86.71% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.31%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*60.43%\**</span> | <span title="validated on-demand with panic">*61.08%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.36%\**</span> | <span title="validated on-demand with error">*42.74%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*45.74%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.70%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.5.0
[borsh]: https://crates.io/crates/borsh/1.1.1
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.18.3
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.1
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.35
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.5
[postcard]: https://crates.io/crates/postcard/1.0.8
[pot]: https://crates.io/crates/pot/3.0.0
[prost]: https://crates.io/crates/prost/0.12.1
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.16.2
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.108
[simd-json]: https://crates.io/crates/simd-json/0.13.4
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
