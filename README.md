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

## Last updated: 2024-1-6 17:59:33

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 431.85 µs | <span title="unvalidated">*1.4564 ms\**</span> | 1705800 | 520075 | 413435 |
| [alkahest 0.1.5][alkahest] | 192.01 µs | † | 1045784 | 454157 | 389424 |
| [bincode 2.0.0-rc][bincode] | 214.70 µs | 2.4164 ms | 741295 | 303944 | 257153 |
| [bincode 1.3.3][bincode1] | 522.83 µs | 1.9999 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.5.0][bitcode] | 367.72 µs | 2.1733 ms | 703664 | 317711 | 273622 |
| [borsh 1.3.0][borsh] | 544.59 µs | 2.1676 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 2.2021 ms | 7.9124 ms | 1924682 | 532821 | 376270 |
| [capnp 0.18.11][capnp] | 603.18 µs | † | 1443216 | 513986 | 428649 |
| [cbor4ii 0.3.2][cbor4ii] | 903.35 µs | 4.7859 ms | 1407835 | 403440 | 324081 |
| [ciborium 0.2.1][ciborium] | 4.1146 ms | 10.843 ms | 1407835 | 403440 | 324081 |
| [databuf 0.5.0][databuf] | 273.40 µs | 2.0456 ms | 765778 | 311715 | 264630 |
| [dlhn 0.1.6][dlhn] | 784.79 µs | 2.4130 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.4649 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.2243 ms | 2.5811 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.6419 ms | 4.4101 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.35][nanoserde] | 281.51 µs | 2.0566 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 671.21 µs | 2.2498 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.8][postcard] | 420.80 µs | 2.2531 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 2.2387 ms | 6.5078 ms | 971922 | 372513 | 304122 |
| [prost 0.12.3][prost] | <span title="encode">*460.64 µs\**</span> <span title="populate + encode">*2.0537 ms\**</span> | 2.7789 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.43][rkyv] | 217.42 µs | <span title="unvalidated">*1.4464 ms\**</span> <span title="validated upfront with error">*1.9597 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.3691 ms | 3.5583 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 14.355 ms | 16.287 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.2][savefile] | 203.80 µs | 2.1253 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 664.63 µs | 2.1913 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0076 ms | 4.9323 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.111][serde_json] | 3.8404 ms | 5.5338 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.13.6][simd-json] | 2.0578 ms | 4.6324 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 196.67 µs | 1.7510 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.081 µs\**</span> | <span title="unvalidated">*37.408 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8561 ns\**</span> | <span title="validated on-demand with panic">*24.779 µs\**</span> | ‡ |
| [capnp 0.18.11][capnp] | <span title="validated on-demand with error">*79.895 ns\**</span> | <span title="validated on-demand with error">*180.49 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4730 ns\**</span> <span title="validated upfront with error">*1.9150 ms\**</span> | <span title="unvalidated">*48.887 µs\**</span> <span title="validated upfront with error">*1.9367 ms\**</span> | ‡ |
| [rkyv 0.7.43][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*503.56 µs\**</span> | <span title="unvalidated">*10.525 µs\**</span> <span title="validated upfront with error">*515.33 µs\**</span> | 9.7220 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 44.46% | <span title="unvalidated">*99.31%\**</span> | 41.25% | 51.56% | 55.13% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.04% | 58.53% |
| [bincode 2.0.0-rc][bincode] | 89.43% | 59.86% | 94.92% | 88.22% | 88.64% |
| [bincode 1.3.3][bincode1] | 36.73% | 72.32% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.5.0][bitcode] | 52.22% | 66.55% | 100.00% | 84.40% | 83.31% |
| [borsh 1.3.0][borsh] | 35.26% | 66.73% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.72% | 18.28% | 36.56% | 50.32% | 60.58% |
| [capnp 0.18.11][capnp] | 31.83% | † | 48.76% | 52.17% | 53.18% |
| [cbor4ii 0.3.2][cbor4ii] | 21.26% | 30.22% | 49.98% | 66.46% | 70.34% |
| [ciborium 0.2.1][ciborium] | 4.67% | 13.34% | 49.98% | 66.46% | 70.34% |
| [databuf 0.5.0][databuf] | 70.23% | 70.71% | 91.89% | 86.02% | 86.14% |
| [dlhn 0.1.6][dlhn] | 24.47% | 59.94% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 13.11% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 15.68% | 56.04% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.40% | 32.80% | 85.95% | 80.63% | 79.84% |
| [nanoserde 0.1.35][nanoserde] | 68.21% | 70.33% | 67.29% | 71.86% | 73.12% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 28.61% | 64.29% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.8][postcard] | 45.63% | 64.20% | 97.06% | 88.67% | 89.83% |
| [pot 3.0.0][pot] | 8.58% | 22.23% | 72.40% | 71.98% | 74.95% |
| [prost 0.12.3][prost] | <span title="encode">*41.68%\**</span> <span title="populate + encode">*9.35%\**</span> | 52.05% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.43][rkyv] | 88.31% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.81%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.2][rmp-serde] | 14.02% | 40.65% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.1][ron] | 1.34% | 8.88% | 43.77% | 59.70% | 65.18% |
| [savefile 0.16.2][savefile] | 94.21% | 68.06% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 28.89% | 66.01% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 9.56% | 29.33% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.111][serde_json] | 5.00% | 26.14% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.13.6][simd-json] | 9.33% | 31.22% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 97.63% | 82.60% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*28.14%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.65%\**</span> | <span title="validated on-demand with panic">*42.48%\**</span> | ‡ |
| [capnp 0.18.11][capnp] | <span title="validated on-demand with error">*1.55%\**</span> | <span title="validated on-demand with error">*5.83%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.53%\**</span> <span title="validated upfront with error">*0.54%\**</span> | ‡ |
| [rkyv 0.7.43][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.04%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 660.18 µs | <span title="unvalidated">*261.47 µs\**</span> | 6000024 | 5378513 | 5345889 |
| [alkahest 0.1.5][alkahest] | 149.30 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 2.0.0-rc][bincode] | 428.11 µs | 826.38 µs | 6000005 | 5378497 | 5345897 |
| [bincode 1.3.3][bincode1] | 4.8033 ms | 3.9906 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.5.0][bitcode] | 3.3422 ms | 6.0858 ms | 4688054 | 4688491 | 4688168 |
| [borsh 1.3.0][borsh] | 6.2013 ms | 4.3544 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 41.196 ms | 87.025 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.18.11][capnp] | 5.5292 ms | † | 14000088 | 7130367 | 6051062 |
| [cbor4ii 0.3.2][cbor4ii] | 9.9601 ms | 47.122 ms | 13125016 | 7524114 | 6757967 |
| [ciborium 0.2.1][ciborium] | 70.853 ms | 103.14 ms | 13122324 | 7524660 | 6759658 |
| [databuf 0.5.0][databuf] | 2.4019 ms | 5.3454 ms | 6000003 | 5378495 | 5345900 |
| [dlhn 0.1.6][dlhn] | 7.5774 ms | 5.2017 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 642.34 µs | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 21.449 ms | 8.8336 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 132.30 ms | 27.298 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.35][nanoserde] | 1.3858 ms | 898.78 µs | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 5.1524 ms | 3.9645 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.8][postcard] | 507.29 µs | 1.3038 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 36.480 ms | 74.126 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.12.3][prost] | <span title="encode">*7.0122 ms\**</span> <span title="populate + encode">*8.1612 ms\**</span> | 14.194 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.43][rkyv] | 191.88 µs | <span title="unvalidated">*147.75 µs\**</span> <span title="validated upfront with error">*147.79 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 13.723 ms | 18.928 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 171.40 ms | 252.59 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.2][savefile] | 1.6026 ms | 260.35 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.3620 ms | 5.0178 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 35.809 ms | 42.495 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.111][serde_json] | 89.806 ms | 80.048 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.13.6][simd-json] | 54.639 ms | 73.179 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 1.5915 ms | 260.27 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1629 ns\**</span> | <span title="unvalidated">*142.18 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8561 ns\**</span> | <span title="validated on-demand with panic">*77.377 µs\**</span> | ‡ |
| [capnp 0.18.11][capnp] | <span title="validated on-demand with error">*104.89 ns\**</span> | <span title="validated on-demand with error">*2.2404 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4741 ns\**</span> <span title="validated upfront with error">*37.759 ns\**</span> | <span title="unvalidated">*54.104 µs\**</span> <span title="validated upfront with error">*77.459 µs\**</span> | ‡ |
| [rkyv 0.7.43][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*6.1914 ns\**</span> | <span title="unvalidated">*46.097 µs\**</span> <span title="validated upfront with error">*38.713 µs\**</span> | 106.13 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 22.62% | <span title="unvalidated">*56.51%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 78.13% | 87.17% | 87.70% |
| [bincode 2.0.0-rc][bincode] | 34.87% | 17.88% | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode1] | 3.11% | 3.70% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.5.0][bitcode] | 4.47% | 2.43% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 2.41% | 3.39% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.36% | 0.17% | 20.37% | 50.89% | 62.53% |
| [capnp 0.18.11][capnp] | 2.70% | † | 33.49% | 65.75% | 77.48% |
| [cbor4ii 0.3.2][cbor4ii] | 1.50% | 0.31% | 35.72% | 62.31% | 69.37% |
| [ciborium 0.2.1][ciborium] | 0.21% | 0.14% | 35.73% | 62.31% | 69.36% |
| [databuf 0.5.0][databuf] | 6.22% | 2.76% | 78.13% | 87.17% | 87.70% |
| [dlhn 0.1.6][dlhn] | 1.97% | 2.84% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 23.24% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 0.70% | 1.67% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.11% | 0.54% | 57.70% | 72.20% | 73.40% |
| [nanoserde 0.1.35][nanoserde] | 10.77% | 16.44% | 78.13% | 87.17% | 87.70% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.90% | 3.73% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.8][postcard] | 29.43% | 11.33% | 78.13% | 87.17% | 87.70% |
| [pot 3.0.0][pot] | 0.41% | 0.20% | 46.31% | 68.80% | 68.42% |
| [prost 0.12.3][prost] | <span title="encode">*2.13%\**</span> <span title="populate + encode">*1.83%\**</span> | 1.04% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.43][rkyv] | 77.81% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.97%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.2][rmp-serde] | 1.09% | 0.78% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.2][savefile] | 9.32% | 56.75% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 2.35% | 2.94% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.42% | 0.35% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.111][serde_json] | 0.17% | 0.18% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.13.6][simd-json] | 0.27% | 0.20% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 9.38% | 56.77% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.19%\**</span> | <span title="unvalidated">*27.23%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.64%\**</span> | <span title="validated on-demand with panic">*50.03%\**</span> | ‡ |
| [capnp 0.18.11][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.73%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*3.28%\**</span> | <span title="unvalidated">*71.55%\**</span> <span title="validated upfront with error">*49.98%\**</span> | ‡ |
| [rkyv 0.7.43][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*19.98%\**</span> | <span title="unvalidated">*83.98%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 189.66 µs | <span title="unvalidated">*1.3198 ms\**</span> | 1290592 | 396648 | 340139 |
| [alkahest 0.1.5][alkahest] | 220.76 µs | † | 667570 | 325484 | 320452 |
| [bincode 2.0.0-rc][bincode] | 283.96 µs | 2.0588 ms | 367413 | 221291 | 206273 |
| [bincode 1.3.3][bincode1] | 573.69 µs | 1.7915 ms | 569975 | 240525 | 232423 |
| [bitcode 0.5.0][bitcode] | 289.62 µs | 1.8916 ms | 322798 | 214279 | 201247 |
| [borsh 1.3.0][borsh] | 550.34 µs | 1.8177 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 2.8848 ms | 9.0810 ms | 1619653 | 502185 | 328399 |
| [capnp 0.18.11][capnp] | 456.14 µs | † | 803896 | 335606 | 280851 |
| [cbor4ii 0.3.2][cbor4ii] | 792.47 µs | 4.7986 ms | 1109831 | 344745 | 274514 |
| [ciborium 0.2.1][ciborium] | 3.8102 ms | 9.3868 ms | 1109821 | 344751 | 274526 |
| [databuf 0.5.0][databuf] | 326.43 µs | 1.7422 ms | 356311 | 213062 | 198488 |
| [dlhn 0.1.6][dlhn] | 800.19 µs | 2.6113 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.3525 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 965.00 µs | 2.8479 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3845 ms | 4.0811 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.35][nanoserde] | 291.58 µs | 1.8941 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 664.56 µs | 1.9929 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.8][postcard] | 433.07 µs | 2.0056 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 2.2298 ms | 6.0924 ms | 599125 | 299158 | 247693 |
| [prost 0.12.3][prost] | <span title="encode">*995.48 µs\**</span> <span title="populate + encode">*2.7022 ms\**</span> | 3.4607 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.43][rkyv] | 301.11 µs | <span title="unvalidated">*1.2669 ms\**</span> <span title="validated upfront with error">*1.7583 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.4643 ms | 3.0767 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 8.3345 ms | 17.874 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.2][savefile] | 220.77 µs | 1.8269 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 739.41 µs | 2.2384 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 1.8077 ms | 4.7880 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.111][serde_json] | 3.7686 ms | 6.6920 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.13.6][simd-json] | 2.2204 ms | 4.5624 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 274.39 µs | 1.6557 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.519 µs\**</span> | <span title="unvalidated">*37.583 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8554 ns\**</span> | <span title="validated on-demand with panic">*4.6647 µs\**</span> | ‡ |
| [capnp 0.18.11][capnp] | <span title="validated on-demand with error">*80.238 ns\**</span> | <span title="validated on-demand with error">*581.51 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4758 ns\**</span> <span title="validated upfront with error">*2.1466 ms\**</span> | <span title="unvalidated">*1.3731 µs\**</span> <span title="validated upfront with error">*2.1468 ms\**</span> | ‡ |
| [rkyv 0.7.43][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*488.37 µs\**</span> | <span title="unvalidated">*163.49 ns\**</span> <span title="validated upfront with error">*496.53 µs\**</span> | 916.53 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.99%\**</span> | 25.01% | 53.69% | 58.35% |
| [alkahest 0.1.5][alkahest] | 85.91% | † | 48.35% | 65.43% | 61.94% |
| [bincode 2.0.0-rc][bincode] | 66.79% | 61.54% | 87.86% | 96.24% | 96.23% |
| [bincode 1.3.3][bincode1] | 33.06% | 70.72% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.5.0][bitcode] | 65.49% | 66.98% | 100.00% | 99.39% | 98.63% |
| [borsh 1.3.0][borsh] | 34.46% | 69.70% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.57% | 13.95% | 19.93% | 42.41% | 60.44% |
| [capnp 0.18.11][capnp] | 41.58% | † | 40.15% | 63.46% | 70.67% |
| [cbor4ii 0.3.2][cbor4ii] | 23.93% | 26.40% | 29.09% | 61.78% | 72.31% |
| [ciborium 0.2.1][ciborium] | 4.98% | 13.50% | 29.09% | 61.78% | 72.30% |
| [databuf 0.5.0][databuf] | 58.10% | 72.72% | 90.59% | 99.96% | 100.00% |
| [dlhn 0.1.6][dlhn] | 23.70% | 48.52% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 5.66% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 19.65% | 44.49% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.52% | 31.04% | 71.77% | 84.37% | 85.88% |
| [nanoserde 0.1.35][nanoserde] | 65.05% | 66.89% | 56.83% | 88.77% | 85.40% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 28.54% | 63.57% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.8][postcard] | 43.79% | 63.17% | 87.84% | 95.97% | 95.73% |
| [pot 3.0.0][pot] | 8.51% | 20.79% | 53.88% | 71.19% | 80.13% |
| [prost 0.12.3][prost] | <span title="encode">*19.05%\**</span> <span title="populate + encode">*7.02%\**</span> | 36.61% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.43][rkyv] | 62.99% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.05%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.2][rmp-serde] | 12.95% | 41.18% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.1][ron] | 2.28% | 7.09% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.2][savefile] | 85.91% | 69.35% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 25.65% | 56.60% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 10.49% | 26.46% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.111][serde_json] | 5.03% | 18.93% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.13.6][simd-json] | 8.54% | 27.77% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 69.12% | 76.52% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.44%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*3.50%\**</span> | ‡ |
| [capnp 0.18.11][capnp] | <span title="validated on-demand with error">*1.54%\**</span> | <span title="validated on-demand with error">*28.11%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.91%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.43][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 492.86 µs | <span title="unvalidated">*2.3171 ms\**</span> | 2984682 | 1408278 | 1273666 |
| [alkahest 0.1.5][alkahest] | 745.34 µs | † | 1863391 | 1234113 | 1202345 |
| [bincode 2.0.0-rc][bincode] | 733.76 µs | 3.6634 ms | 1372381 | 1091486 | 1037296 |
| [bincode 1.3.3][bincode1] | 3.7602 ms | 3.9136 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.5.0][bitcode] | 1.0885 ms | 3.2854 ms | 870693 | 866738 | 870720 |
| [borsh 1.3.0][borsh] | 2.9755 ms | 2.5663 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 24.443 ms | 49.830 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.18.11][capnp] | 2.1578 ms | † | 2664040 | 1511895 | 1212087 |
| [cbor4ii 0.3.2][cbor4ii] | 4.2188 ms | 18.567 ms | 5878791 | 1655835 | 1431390 |
| [ciborium 0.2.1][ciborium] | 24.209 ms | 47.457 ms | 5878653 | 1655791 | 1431560 |
| [databuf 0.5.0][databuf] | 1.7706 ms | 3.7469 ms | 1288257 | 1037579 | 984337 |
| [dlhn 0.1.6][dlhn] | 5.1918 ms | 6.6967 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 4.8966 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 2.2453 ms | 4.5244 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.800 ms | 17.922 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.35][nanoserde] | 1.3006 ms | 3.0069 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.0333 ms | 3.0252 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.8][postcard] | 1.8409 ms | 3.9292 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 13.203 ms | 30.876 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.12.3][prost] | <span title="encode">*4.1153 ms\**</span> <span title="populate + encode">*8.2012 ms\**</span> | 9.3302 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.43][rkyv] | 1.2869 ms | <span title="unvalidated">*2.1725 ms\**</span> <span title="validated upfront with error">*2.7760 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 10.388 ms | 11.628 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 38.632 ms | 99.438 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.2][savefile] | 1.0287 ms | 2.6657 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.9025 ms | 4.4623 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 9.6509 ms | 20.785 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.111][serde_json] | 20.324 ms | 28.784 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.13.6][simd-json] | 11.396 ms | 26.158 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 718.66 µs | 2.5019 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.080 µs\**</span> | <span title="unvalidated">*67.483 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8562 ns\**</span> | <span title="validated on-demand with panic">*628.00 ns\**</span> | ‡ |
| [capnp 0.18.11][capnp] | <span title="validated on-demand with error">*80.046 ns\**</span> | <span title="validated on-demand with error">*728.61 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4731 ns\**</span> <span title="validated upfront with error">*5.0862 ms\**</span> | <span title="unvalidated">*2.6564 µs\**</span> <span title="validated upfront with error">*5.0945 ms\**</span> | ‡ |
| [rkyv 0.7.43][rkyv] | <span title="unvalidated">*1.2374 ns\**</span> <span title="validated upfront with error">*599.81 µs\**</span> | <span title="unvalidated">*356.94 ns\**</span> <span title="validated upfront with error">*604.01 µs\**</span> | 625.21 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.76%\**</span> | 29.17% | 61.55% | 68.36% |
| [alkahest 0.1.5][alkahest] | 66.13% | † | 46.73% | 70.23% | 72.42% |
| [bincode 2.0.0-rc][bincode] | 67.17% | 59.30% | 63.44% | 79.41% | 83.94% |
| [bincode 1.3.3][bincode1] | 13.11% | 55.51% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.5.0][bitcode] | 45.28% | 66.13% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 16.56% | 84.65% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 2.02% | 4.36% | 8.68% | 30.59% | 54.39% |
| [capnp 0.18.11][capnp] | 22.84% | † | 32.68% | 57.33% | 71.84% |
| [cbor4ii 0.3.2][cbor4ii] | 11.68% | 11.70% | 14.81% | 52.34% | 60.83% |
| [ciborium 0.2.1][ciborium] | 2.04% | 4.58% | 14.81% | 52.35% | 60.82% |
| [databuf 0.5.0][databuf] | 27.84% | 57.98% | 67.59% | 83.53% | 88.46% |
| [dlhn 0.1.6][dlhn] | 9.49% | 32.44% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 10.07% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 21.95% | 48.02% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.60% | 12.12% | 50.37% | 69.47% | 70.60% |
| [nanoserde 0.1.35][nanoserde] | 37.89% | 72.25% | 49.18% | 78.20% | 84.54% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 16.25% | 71.81% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.8][postcard] | 26.77% | 55.29% | 68.04% | 81.90% | 85.64% |
| [pot 3.0.0][pot] | 3.73% | 7.04% | 34.21% | 59.88% | 68.65% |
| [prost 0.12.3][prost] | <span title="encode">*11.98%\**</span> <span title="populate + encode">*6.01%\**</span> | 23.28% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.43][rkyv] | 38.30% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.26%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.2][rmp-serde] | 4.74% | 18.68% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.1][ron] | 1.28% | 2.18% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.2][savefile] | 47.91% | 81.50% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 10.05% | 48.69% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.11% | 10.45% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.111][serde_json] | 2.43% | 7.55% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.13.6][simd-json] | 4.32% | 8.31% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 68.58% | 86.83% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.53%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*56.84%\**</span> | ‡ |
| [capnp 0.18.11][capnp] | <span title="validated on-demand with error">*1.55%\**</span> | <span title="validated on-demand with error">*48.99%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.44%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.43][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.5.0
[borsh]: https://crates.io/crates/borsh/1.3.0
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.18.11
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.2
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.35
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.9
[postcard]: https://crates.io/crates/postcard/1.0.8
[pot]: https://crates.io/crates/pot/3.0.0
[prost]: https://crates.io/crates/prost/0.12.3
[rkyv]: https://crates.io/crates/rkyv/0.7.43
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.16.2
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.111
[simd-json]: https://crates.io/crates/simd-json/0.13.6
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
