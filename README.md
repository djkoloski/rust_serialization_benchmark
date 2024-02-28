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

## Last updated: 2024-2-6 17:59:10

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 412.49 µs | <span title="unvalidated">*1.4634 ms\**</span> | 1705800 | 520079 | 413624 |
| [alkahest 0.1.5][alkahest] | 198.75 µs | † | 1045784 | 454157 | 389424 |
| [bincode 2.0.0-rc][bincode] | 204.01 µs | 2.4683 ms | 741295 | 303944 | 257153 |
| [bincode 1.3.3][bincode1] | 522.15 µs | 2.0239 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.5.0][bitcode] | 369.60 µs | 2.2337 ms | 703664 | 317711 | 273622 |
| [borsh 1.3.0][borsh] | 544.23 µs | 2.2206 ms | 885780 | 362204 | 286514 |
| [bson 2.9.0][bson] | 2.2234 ms | 7.0106 ms | 1924682 | 532821 | 376270 |
| [capnp 0.18.13][capnp] | 590.96 µs | † | 1443216 | 513986 | 428649 |
| [cbor4ii 0.3.2][cbor4ii] | 905.01 µs | 4.9817 ms | 1407835 | 403440 | 324081 |
| [ciborium 0.2.2][ciborium] | 3.8698 ms | 10.150 ms | 1407835 | 403440 | 324081 |
| [databuf 0.5.0][databuf] | 255.70 µs | 2.0341 ms | 765778 | 311715 | 264630 |
| [dlhn 0.1.6][dlhn] | 635.22 µs | 2.4176 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.3477 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.1960 ms | 2.5583 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.9543 ms | 4.1778 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.37][nanoserde] | 253.61 µs | 2.0668 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 674.76 µs | 2.2274 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.8][postcard] | 435.20 µs | 2.1404 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 2.4076 ms | 6.6797 ms | 971922 | 372513 | 304122 |
| [prost 0.12.3][prost] | <span title="encode">*823.29 µs\**</span> <span title="populate + encode">*2.3389 ms\**</span> | 3.4769 ms | 884628 | 363130 | 315494 |
| [rkyv 0.7.44][rkyv] | 214.04 µs | <span title="unvalidated">*1.4452 ms\**</span> <span title="validated upfront with error">*1.9704 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.3825 ms | 3.5453 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 14.135 ms | 16.409 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.5][savefile] | 202.44 µs | 2.0934 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 668.53 µs | 2.1683 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0089 ms | 4.8453 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.113][serde_json] | 3.6966 ms | 5.6882 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.13.8][simd-json] | 2.0944 ms | 5.2013 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.7][speedy] | 194.13 µs | 1.7587 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.905 µs\**</span> | <span title="unvalidated">*40.941 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8559 ns\**</span> | <span title="validated on-demand with panic">*24.864 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*76.284 ns\**</span> | <span title="validated on-demand with error">*169.43 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4801 ns\**</span> <span title="validated upfront with error">*1.8466 ms\**</span> | <span title="unvalidated">*50.811 µs\**</span> <span title="validated upfront with error">*1.9148 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2402 ns\**</span> <span title="validated upfront with error">*515.53 µs\**</span> | <span title="unvalidated">*10.545 µs\**</span> <span title="validated upfront with error">*526.20 µs\**</span> | 9.6463 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 47.06% | <span title="unvalidated">*98.76%\**</span> | 41.25% | 57.96% | 61.32% |
| [alkahest 0.1.5][alkahest] | 97.68% | † | 67.29% | 66.37% | 65.13% |
| [bincode 2.0.0-rc][bincode] | 95.16% | 58.55% | 94.92% | 99.18% | 98.63% |
| [bincode 1.3.3][bincode1] | 37.18% | 71.41% | 67.29% | 80.79% | 81.35% |
| [bitcode 0.5.0][bitcode] | 52.52% | 64.70% | 100.00% | 94.88% | 92.69% |
| [borsh 1.3.0][borsh] | 35.67% | 65.08% | 79.44% | 83.23% | 88.52% |
| [bson 2.9.0][bson] | 8.73% | 20.61% | 36.56% | 56.58% | 67.41% |
| [capnp 0.18.13][capnp] | 32.85% | † | 48.76% | 58.65% | 59.17% |
| [cbor4ii 0.3.2][cbor4ii] | 21.45% | 29.01% | 49.98% | 74.72% | 78.26% |
| [ciborium 0.2.2][ciborium] | 5.02% | 14.24% | 49.98% | 74.72% | 78.26% |
| [databuf 0.5.0][databuf] | 75.92% | 71.05% | 91.89% | 96.71% | 95.84% |
| [dlhn 0.1.6][dlhn] | 30.56% | 59.78% | 97.06% | 100.00% | 100.00% |
| [flatbuffers 23.5.26][flatbuffers] | 14.40% | † | 55.13% | 64.34% | 65.23% |
| [msgpacker 0.4.3][msgpacker] | 16.23% | 56.49% | 91.98% | 95.61% | 95.75% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.26% | 34.59% | 85.95% | 90.65% | 88.83% |
| [nanoserde 0.1.37][nanoserde] | 76.55% | 69.92% | 67.29% | 80.79% | 81.35% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 28.77% | 64.88% | 91.89% | 96.70% | 95.88% |
| [postcard 1.0.8][postcard] | 44.61% | 67.52% | 97.06% | 99.68% | 99.95% |
| [pot 3.0.0][pot] | 8.06% | 21.64% | 72.40% | 80.92% | 83.40% |
| [prost 0.12.3][prost] | <span title="encode">*23.58%\**</span> <span title="populate + encode">*8.30%\**</span> | 41.57% | 79.54% | 83.01% | 80.39% |
| [rkyv 0.7.44][rkyv] | 90.70% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.35%\**</span> | 69.57% | 78.53% | 76.04% |
| [rmp-serde 1.1.2][rmp-serde] | 14.04% | 40.76% | 89.64% | 92.64% | 91.16% |
| [ron 0.8.1][ron] | 1.37% | 8.81% | 43.77% | 67.11% | 72.52% |
| [savefile 0.16.5][savefile] | 95.90% | 69.04% | 67.28% | 80.79% | 81.36% |
| [serde_bare 0.5.0][serde_bare] | 29.04% | 66.65% | 91.89% | 96.71% | 95.84% |
| [serde_cbor 0.11.2][serde_cbor] | 9.66% | 29.83% | 49.98% | 74.72% | 78.26% |
| [serde_json 1.0.113][serde_json] | 5.25% | 25.41% | 38.51% | 64.06% | 70.24% |
| [simd-json 0.13.8][simd-json] | 9.27% | 27.79% | 38.51% | 64.06% | 70.24% |
| [speedy 0.8.7][speedy] | 100.00% | 82.17% | 79.44% | 83.23% | 88.52% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*25.76%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.82%\**</span> | <span title="validated on-demand with panic">*42.41%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.63%\**</span> | <span title="validated on-demand with error">*6.22%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.75%\**</span> <span title="validated upfront with error">*0.55%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.00%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 238.97 µs | <span title="unvalidated">*237.45 µs\**</span> | 6000024 | 5378513 | 5345890 |
| [alkahest 0.1.5][alkahest] | 198.11 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 2.0.0-rc][bincode] | 421.98 µs | 823.88 µs | 6000005 | 5378497 | 5345897 |
| [bincode 1.3.3][bincode1] | 4.7801 ms | 4.0723 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.5.0][bitcode] | 3.3469 ms | 6.1142 ms | 4688054 | 4688491 | 4688168 |
| [borsh 1.3.0][borsh] | 5.9735 ms | 4.5215 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.9.0][bson] | 41.847 ms | 82.314 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.18.13][capnp] | 5.3081 ms | † | 14000088 | 7130367 | 6051062 |
| [cbor4ii 0.3.2][cbor4ii] | 10.611 ms | 47.404 ms | 13125016 | 7524114 | 6757967 |
| [ciborium 0.2.2][ciborium] | 65.319 ms | 104.71 ms | 13122324 | 7524660 | 6759658 |
| [databuf 0.5.0][databuf] | 2.4066 ms | 5.3143 ms | 6000003 | 5378495 | 5345900 |
| [dlhn 0.1.6][dlhn] | 6.1386 ms | 6.3852 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 645.52 µs | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 19.561 ms | 9.6388 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 125.33 ms | 26.799 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.37][nanoserde] | 1.5480 ms | 904.59 µs | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.8467 ms | 3.9677 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.8][postcard] | 476.28 µs | 1.4674 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 39.376 ms | 73.754 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.12.3][prost] | <span title="encode">*7.0202 ms\**</span> <span title="populate + encode">*8.8683 ms\**</span> | 13.375 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.44][rkyv] | 185.47 µs | <span title="unvalidated">*199.46 µs\**</span> <span title="validated upfront with error">*198.14 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 12.461 ms | 18.934 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 170.79 ms | 258.21 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.5][savefile] | 238.48 µs | 238.51 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.0994 ms | 4.9335 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 35.447 ms | 43.303 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.113][serde_json] | 89.020 ms | 73.645 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.13.8][simd-json] | 54.133 ms | 72.827 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.7][speedy] | 238.58 µs | 241.62 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1650 ns\**</span> | <span title="unvalidated">*142.64 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8560 ns\**</span> | <span title="validated on-demand with panic">*77.345 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*109.50 ns\**</span> | <span title="validated on-demand with error">*2.1448 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4727 ns\**</span> <span title="validated upfront with error">*37.138 ns\**</span> | <span title="unvalidated">*54.193 µs\**</span> <span title="validated upfront with error">*77.508 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*6.2272 ns\**</span> | <span title="unvalidated">*46.057 µs\**</span> <span title="validated upfront with error">*38.735 µs\**</span> | 109.02 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 77.61% | <span title="unvalidated">*83.44%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 93.62% | † | 78.13% | 87.17% | 87.70% |
| [bincode 2.0.0-rc][bincode] | 43.95% | 24.05% | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode1] | 3.88% | 4.87% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.5.0][bitcode] | 5.54% | 3.24% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 3.10% | 4.38% | 78.13% | 87.17% | 87.70% |
| [bson 2.9.0][bson] | 0.44% | 0.24% | 20.37% | 50.89% | 62.53% |
| [capnp 0.18.13][capnp] | 3.49% | † | 33.49% | 65.75% | 77.48% |
| [cbor4ii 0.3.2][cbor4ii] | 1.75% | 0.42% | 35.72% | 62.31% | 69.37% |
| [ciborium 0.2.2][ciborium] | 0.28% | 0.19% | 35.73% | 62.31% | 69.36% |
| [databuf 0.5.0][databuf] | 7.71% | 3.73% | 78.13% | 87.17% | 87.70% |
| [dlhn 0.1.6][dlhn] | 3.02% | 3.10% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 28.73% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 0.95% | 2.06% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.15% | 0.74% | 57.70% | 72.20% | 73.40% |
| [nanoserde 0.1.37][nanoserde] | 11.98% | 21.90% | 78.13% | 87.17% | 87.70% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.83% | 4.99% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.8][postcard] | 38.94% | 13.50% | 78.13% | 87.17% | 87.70% |
| [pot 3.0.0][pot] | 0.47% | 0.27% | 46.31% | 68.80% | 68.42% |
| [prost 0.12.3][prost] | <span title="encode">*2.64%\**</span> <span title="populate + encode">*2.09%\**</span> | 1.48% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.44][rkyv] | 100.00% | <span title="unvalidated">*99.34%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.2][rmp-serde] | 1.49% | 1.05% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.1][ron] | 0.11% | 0.08% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.5][savefile] | 77.77% | 83.07% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 3.04% | 4.02% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.52% | 0.46% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.113][serde_json] | 0.21% | 0.27% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.13.8][simd-json] | 0.34% | 0.27% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.7][speedy] | 77.74% | 82.00% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.14%\**</span> | <span title="unvalidated">*27.16%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.65%\**</span> | <span title="validated on-demand with panic">*50.08%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.13%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*3.33%\**</span> | <span title="unvalidated">*71.48%\**</span> <span title="validated upfront with error">*49.98%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*19.86%\**</span> | <span title="unvalidated">*84.10%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 192.02 µs | <span title="unvalidated">*1.3145 ms\**</span> | 1290592 | 397021 | 340587 |
| [alkahest 0.1.5][alkahest] | 221.52 µs | † | 667570 | 325484 | 320452 |
| [bincode 2.0.0-rc][bincode] | 274.49 µs | 2.0685 ms | 367413 | 221291 | 206273 |
| [bincode 1.3.3][bincode1] | 561.87 µs | 1.7887 ms | 569975 | 240525 | 232423 |
| [bitcode 0.5.0][bitcode] | 285.78 µs | 1.8681 ms | 322798 | 214279 | 201247 |
| [borsh 1.3.0][borsh] | 525.85 µs | 1.8277 ms | 446595 | 234236 | 210008 |
| [bson 2.9.0][bson] | 2.8913 ms | 8.2514 ms | 1619653 | 502185 | 328399 |
| [capnp 0.18.13][capnp] | 470.99 µs | † | 803896 | 335606 | 280851 |
| [cbor4ii 0.3.2][cbor4ii] | 796.00 µs | 4.8538 ms | 1109831 | 344745 | 274514 |
| [ciborium 0.2.2][ciborium] | 3.6280 ms | 9.5520 ms | 1109821 | 344751 | 274526 |
| [databuf 0.5.0][databuf] | 319.10 µs | 1.7396 ms | 356311 | 213062 | 198488 |
| [dlhn 0.1.6][dlhn] | 683.74 µs | 2.5726 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.3207 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 914.87 µs | 2.8704 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3306 ms | 3.9616 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.37][nanoserde] | 288.38 µs | 1.8804 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 661.87 µs | 2.0076 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.8][postcard] | 422.09 µs | 1.9886 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 2.3508 ms | 6.3356 ms | 599125 | 299158 | 247693 |
| [prost 0.12.3][prost] | <span title="encode">*1.0447 ms\**</span> <span title="populate + encode">*2.7444 ms\**</span> | 3.5767 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.44][rkyv] | 302.47 µs | <span title="unvalidated">*1.2572 ms\**</span> <span title="validated upfront with error">*1.7566 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.4560 ms | 2.9793 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 8.1970 ms | 16.822 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.5][savefile] | 221.91 µs | 1.8129 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 680.58 µs | 2.2206 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 1.7966 ms | 4.6925 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.113][serde_json] | 3.6635 ms | 6.6790 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.13.8][simd-json] | 2.2298 ms | 4.5224 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.7][speedy] | 272.65 µs | 1.6372 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.656 µs\**</span> | <span title="unvalidated">*38.454 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8552 ns\**</span> | <span title="validated on-demand with panic">*4.5755 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*76.091 ns\**</span> | <span title="validated on-demand with error">*441.08 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4749 ns\**</span> <span title="validated upfront with error">*2.1469 ms\**</span> | <span title="unvalidated">*1.3721 µs\**</span> <span title="validated upfront with error">*2.2558 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2378 ns\**</span> <span title="validated upfront with error">*498.19 µs\**</span> | <span title="unvalidated">*163.40 ns\**</span> <span title="validated upfront with error">*493.95 µs\**</span> | 900.88 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.64%\**</span> | 25.01% | 53.64% | 58.28% |
| [alkahest 0.1.5][alkahest] | 86.68% | † | 48.35% | 65.43% | 61.94% |
| [bincode 2.0.0-rc][bincode] | 69.96% | 60.78% | 87.86% | 96.24% | 96.23% |
| [bincode 1.3.3][bincode1] | 34.18% | 70.29% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.5.0][bitcode] | 67.19% | 67.30% | 100.00% | 99.39% | 98.63% |
| [borsh 1.3.0][borsh] | 36.52% | 68.79% | 72.28% | 90.92% | 94.51% |
| [bson 2.9.0][bson] | 6.64% | 15.24% | 19.93% | 42.41% | 60.44% |
| [capnp 0.18.13][capnp] | 40.77% | † | 40.15% | 63.46% | 70.67% |
| [cbor4ii 0.3.2][cbor4ii] | 24.12% | 25.90% | 29.09% | 61.78% | 72.31% |
| [ciborium 0.2.2][ciborium] | 5.29% | 13.16% | 29.09% | 61.78% | 72.30% |
| [databuf 0.5.0][databuf] | 60.18% | 72.27% | 90.59% | 99.96% | 100.00% |
| [dlhn 0.1.6][dlhn] | 28.08% | 48.87% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 5.78% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 20.99% | 43.80% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.60% | 31.73% | 71.77% | 84.37% | 85.88% |
| [nanoserde 0.1.37][nanoserde] | 66.59% | 66.86% | 56.83% | 88.77% | 85.40% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 29.01% | 62.62% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.8][postcard] | 45.49% | 63.22% | 87.84% | 95.97% | 95.73% |
| [pot 3.0.0][pot] | 8.17% | 19.84% | 53.88% | 71.19% | 80.13% |
| [prost 0.12.3][prost] | <span title="encode">*18.38%\**</span> <span title="populate + encode">*7.00%\**</span> | 35.15% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.44][rkyv] | 63.48% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.57%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.2][rmp-serde] | 13.19% | 42.20% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.1][ron] | 2.34% | 7.47% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.5][savefile] | 86.53% | 69.35% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 28.21% | 56.62% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 10.69% | 26.79% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.113][serde_json] | 5.24% | 18.82% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.13.8][simd-json] | 8.61% | 27.80% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.7][speedy] | 70.43% | 76.79% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.42%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.72%\**</span> | <span title="validated on-demand with panic">*3.57%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.63%\**</span> | <span title="validated on-demand with error">*37.05%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.91%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 476.53 µs | <span title="unvalidated">*2.3155 ms\**</span> | 2984682 | 1410692 | 1273554 |
| [alkahest 0.1.5][alkahest] | 727.55 µs | † | 1863391 | 1234113 | 1202345 |
| [bincode 2.0.0-rc][bincode] | 696.23 µs | 3.5300 ms | 1372381 | 1091486 | 1037296 |
| [bincode 1.3.3][bincode1] | 3.6726 ms | 3.9197 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.5.0][bitcode] | 1.0915 ms | 3.2882 ms | 870693 | 866738 | 870720 |
| [borsh 1.3.0][borsh] | 2.8697 ms | 2.5394 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.9.0][bson] | 20.591 ms | 44.216 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.18.13][capnp] | 2.6748 ms | † | 2664040 | 1511895 | 1212087 |
| [cbor4ii 0.3.2][cbor4ii] | 4.3748 ms | 17.675 ms | 5878791 | 1655835 | 1431390 |
| [ciborium 0.2.2][ciborium] | 22.072 ms | 47.000 ms | 5878653 | 1655791 | 1431560 |
| [databuf 0.5.0][databuf] | 1.7838 ms | 3.6107 ms | 1288257 | 1037579 | 984337 |
| [dlhn 0.1.6][dlhn] | 4.9176 ms | 6.6456 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.0609 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 1.9131 ms | 4.5309 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 32.279 ms | 15.469 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.37][nanoserde] | 1.2905 ms | 3.0065 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.8666 ms | 2.9817 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.8][postcard] | 1.7794 ms | 3.9256 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 13.589 ms | 30.766 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.12.3][prost] | <span title="encode">*4.0968 ms\**</span> <span title="populate + encode">*8.1792 ms\**</span> | 9.3688 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.44][rkyv] | 1.1072 ms | <span title="unvalidated">*2.1630 ms\**</span> <span title="validated upfront with error">*2.7725 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 10.609 ms | 11.396 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 36.074 ms | 96.823 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.5][savefile] | 1.0051 ms | 2.6673 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.6538 ms | 4.4064 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 9.2504 ms | 20.887 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.113][serde_json] | 19.958 ms | 28.918 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.13.8][simd-json] | 11.395 ms | 25.695 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.7][speedy] | 718.98 µs | 2.5161 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.432 µs\**</span> | <span title="unvalidated">*67.406 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8559 ns\**</span> | <span title="validated on-demand with panic">*628.22 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*76.071 ns\**</span> | <span title="validated on-demand with error">*1.0164 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4726 ns\**</span> <span title="validated upfront with error">*5.0640 ms\**</span> | <span title="unvalidated">*2.7103 µs\**</span> <span title="validated upfront with error">*5.0529 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2374 ns\**</span> <span title="validated upfront with error">*624.39 µs\**</span> | <span title="unvalidated">*489.20 ns\**</span> <span title="validated upfront with error">*619.26 µs\**</span> | 502.06 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.41%\**</span> | 29.17% | 61.44% | 68.37% |
| [alkahest 0.1.5][alkahest] | 65.50% | † | 46.73% | 70.23% | 72.42% |
| [bincode 2.0.0-rc][bincode] | 68.44% | 61.27% | 63.44% | 79.41% | 83.94% |
| [bincode 1.3.3][bincode1] | 12.98% | 55.18% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.5.0][bitcode] | 43.66% | 65.78% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 16.61% | 85.18% | 58.59% | 80.08% | 85.91% |
| [bson 2.9.0][bson] | 2.31% | 4.89% | 8.68% | 30.59% | 54.39% |
| [capnp 0.18.13][capnp] | 17.82% | † | 32.68% | 57.33% | 71.84% |
| [cbor4ii 0.3.2][cbor4ii] | 10.89% | 12.24% | 14.81% | 52.34% | 60.83% |
| [ciborium 0.2.2][ciborium] | 2.16% | 4.60% | 14.81% | 52.35% | 60.82% |
| [databuf 0.5.0][databuf] | 26.71% | 59.91% | 67.59% | 83.53% | 88.46% |
| [dlhn 0.1.6][dlhn] | 9.69% | 32.55% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 9.42% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 24.91% | 47.74% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.48% | 13.98% | 50.37% | 69.47% | 70.60% |
| [nanoserde 0.1.37][nanoserde] | 36.93% | 71.94% | 49.18% | 78.20% | 84.54% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 16.62% | 72.54% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.8][postcard] | 26.78% | 55.10% | 68.04% | 81.90% | 85.64% |
| [pot 3.0.0][pot] | 3.51% | 7.03% | 34.21% | 59.88% | 68.65% |
| [prost 0.12.3][prost] | <span title="encode">*11.63%\**</span> <span title="populate + encode">*5.83%\**</span> | 23.09% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.44][rkyv] | 43.04% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.02%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.2][rmp-serde] | 4.49% | 18.98% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.1][ron] | 1.32% | 2.23% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.5][savefile] | 47.41% | 81.09% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 10.24% | 49.09% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.15% | 10.36% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.113][serde_json] | 2.39% | 7.48% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.13.8][simd-json] | 4.18% | 8.42% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.7][speedy] | 66.28% | 85.97% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.73%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*77.87%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.63%\**</span> | <span title="validated on-demand with error">*48.13%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.04%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.05%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.08%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.5.0
[borsh]: https://crates.io/crates/borsh/1.3.0
[bson]: https://crates.io/crates/bson/2.9.0
[capnp]: https://crates.io/crates/capnp/0.18.13
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.2
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.9
[postcard]: https://crates.io/crates/postcard/1.0.8
[pot]: https://crates.io/crates/pot/3.0.0
[prost]: https://crates.io/crates/prost/0.12.3
[rkyv]: https://crates.io/crates/rkyv/0.7.44
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.16.5
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.113
[simd-json]: https://crates.io/crates/simd-json/0.13.8
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
