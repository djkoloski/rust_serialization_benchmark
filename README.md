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

## Last updated: 2023-11-1 13:41:22

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 296.17 µs | <span title="unvalidated">*2.8586 ms\**</span> | 1705800 | 530419 | 403673 |
| [alkahest 0.1.5][alkahest] | 314.55 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 670.57 µs | 3.6921 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.5.0][bitcode] | 661.82 µs | 4.0917 ms | 703664 | 317711 | 273622 |
| [borsh 1.1.1][borsh] | 579.71 µs | 4.0903 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 2.9524 ms | 12.022 ms | 1924682 | 532821 | 376270 |
| [capnp 0.18.3][capnp] | 711.95 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 3.2328 ms | 12.770 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 936.23 µs | 4.4048 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 2.1313 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.4349 ms | 4.3892 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 9.5019 ms | 7.0377 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.35][nanoserde] | 372.05 µs | 3.7521 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 698.93 µs | 3.9078 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.8][postcard] | 469.05 µs | 3.9931 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 3.4173 ms | 9.7295 ms | 971922 | 372513 | 304122 |
| [prost 0.12.1][prost] | <span title="encode">*592.71 µs\**</span> <span title="populate + encode">*3.6491 ms\**</span> | 4.9093 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 352.61 µs | <span title="unvalidated">*2.8178 ms\**</span> <span title="validated upfront with error">*3.8290 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.6219 ms | 5.3364 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 21.077 ms | 27.957 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.2][savefile] | 261.01 µs | 3.8350 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 789.80 µs | 3.8168 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.2909 ms | 7.7114 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.108][serde_json] | 5.0516 ms | 9.5331 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.13.4][simd-json] | 2.7362 ms | 7.7805 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 331.77 µs | 3.4068 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*46.157 µs\**</span> | <span title="unvalidated">*78.285 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.0659 ns\**</span> | <span title="validated on-demand with panic">*46.848 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*93.502 ns\**</span> | <span title="validated on-demand with error">*242.09 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.8750 ns\**</span> <span title="validated upfront with error">*2.6092 ms\**</span> | <span title="unvalidated">*101.14 µs\**</span> <span title="validated upfront with error">*2.8462 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7781 ns\**</span> <span title="validated upfront with error">*994.23 µs\**</span> | <span title="unvalidated">*19.193 µs\**</span> <span title="validated upfront with error">*1.0111 ms\**</span> | 27.539 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 88.13% | <span title="unvalidated">*98.57%\**</span> | 41.25% | 50.55% | 56.47% |
| [alkahest 0.1.5][alkahest] | 82.98% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 38.92% | 76.32% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.5.0][bitcode] | 39.44% | 68.87% | 100.00% | 84.40% | 83.31% |
| [borsh 1.1.1][borsh] | 45.02% | 68.89% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.84% | 23.44% | 36.56% | 50.32% | 60.58% |
| [capnp 0.18.3][capnp] | 36.66% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 8.07% | 22.07% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 27.88% | 63.97% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 12.25% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 18.19% | 64.20% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.75% | 40.04% | 85.95% | 80.63% | 79.84% |
| [nanoserde 0.1.35][nanoserde] | 70.15% | 75.10% | 67.29% | 71.86% | 73.12% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 37.34% | 72.11% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.8][postcard] | 55.65% | 70.57% | 97.06% | 88.67% | 89.83% |
| [pot 3.0.0][pot] | 7.64% | 28.96% | 72.40% | 71.98% | 74.95% |
| [prost 0.12.1][prost] | <span title="encode">*44.04%\**</span> <span title="populate + encode">*7.15%\**</span> | 57.40% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 74.02% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.59%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.2][rmp-serde] | 16.09% | 52.80% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.1][ron] | 1.24% | 10.08% | 43.77% | 59.70% | 65.18% |
| [savefile 0.16.2][savefile] | 100.00% | 73.48% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 33.05% | 73.83% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.39% | 36.54% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.108][serde_json] | 5.17% | 29.56% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.13.4][simd-json] | 9.54% | 36.22% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 78.67% | 82.71% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*24.52%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.00%\**</span> | <span title="validated on-demand with panic">*40.97%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.90%\**</span> | <span title="validated on-demand with error">*7.93%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*45.89%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.98%\**</span> <span title="validated upfront with error">*0.67%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.90%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 525.58 µs | <span title="unvalidated">*529.58 µs\**</span> | 6000024 | 5378513 | 5345891 |
| [alkahest 0.1.5][alkahest] | 531.90 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 6.1385 ms | 6.4461 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.5.0][bitcode] | 5.5224 ms | 9.9475 ms | 4688054 | 4688491 | 4688168 |
| [borsh 1.1.1][borsh] | 6.9712 ms | 6.7910 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 53.349 ms | 127.01 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.18.3][capnp] | 11.249 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 77.533 ms | 117.13 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 8.4180 ms | 10.913 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.0549 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 23.398 ms | 12.301 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 227.45 ms | 45.127 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.35][nanoserde] | 2.3665 ms | 1.6822 ms | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 6.3399 ms | 5.7167 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.8][postcard] | 989.97 µs | 2.3945 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 56.347 ms | 98.603 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.12.1][prost] | <span title="encode">*13.358 ms\**</span> <span title="populate + encode">*15.795 ms\**</span> | 23.629 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 583.89 µs | <span title="unvalidated">*531.33 µs\**</span> <span title="validated upfront with error">*518.48 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 13.923 ms | 25.007 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 264.27 ms | 472.22 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.2][savefile] | 531.09 µs | 526.85 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.5534 ms | 6.4065 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 46.533 ms | 71.113 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.108][serde_json] | 120.50 ms | 109.51 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.13.4][simd-json] | 77.160 ms | 132.51 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 525.44 µs | 524.68 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.9276 ns\**</span> | <span title="unvalidated">*291.36 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.0776 ns\**</span> | <span title="validated on-demand with panic">*196.32 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*170.22 ns\**</span> | <span title="validated on-demand with error">*3.4173 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.8439 ns\**</span> <span title="validated upfront with error">*50.860 ns\**</span> | <span title="unvalidated">*73.541 µs\**</span> <span title="validated upfront with error">*99.019 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.8006 ns\**</span> <span title="validated upfront with error">*11.929 ns\**</span> | <span title="unvalidated">*55.005 µs\**</span> <span title="validated upfront with error">*55.189 µs\**</span> | 270.98 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 99.97% | <span title="unvalidated">*97.90%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 98.79% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 8.56% | 8.04% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.5.0][bitcode] | 9.51% | 5.21% | 100.00% | 100.00% | 100.00% |
| [borsh 1.1.1][borsh] | 7.54% | 7.63% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.98% | 0.41% | 20.37% | 50.89% | 62.53% |
| [capnp 0.18.3][capnp] | 4.67% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.68% | 0.44% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 6.24% | 4.75% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 49.81% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 2.25% | 4.21% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.23% | 1.15% | 57.70% | 72.20% | 73.40% |
| [nanoserde 0.1.35][nanoserde] | 22.20% | 30.82% | 78.13% | 87.17% | 87.70% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 8.29% | 9.07% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.8][postcard] | 53.08% | 21.65% | 78.13% | 87.17% | 87.70% |
| [pot 3.0.0][pot] | 0.93% | 0.53% | 46.31% | 68.80% | 68.42% |
| [prost 0.12.1][prost] | <span title="encode">*3.93%\**</span> <span title="populate + encode">*3.33%\**</span> | 2.19% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 89.99% | <span title="unvalidated">*97.58%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.2][rmp-serde] | 3.77% | 2.07% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.1][ron] | 0.20% | 0.11% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.2][savefile] | 98.94% | 98.41% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 8.02% | 8.09% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 1.13% | 0.73% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.108][serde_json] | 0.44% | 0.47% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.13.4][simd-json] | 0.68% | 0.39% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 98.82% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.50%\**</span> | <span title="unvalidated">*18.88%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.51%\**</span> | <span title="validated on-demand with panic">*28.02%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.06%\**</span> | <span title="validated on-demand with error">*1.61%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.84%\**</span> <span title="validated upfront with error">*3.54%\**</span> | <span title="unvalidated">*74.80%\**</span> <span title="validated upfront with error">*55.55%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*15.09%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.67%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 279.47 µs | <span title="unvalidated">*2.3408 ms\**</span> | 1290592 | 394786 | 331392 |
| [alkahest 0.1.5][alkahest] | 351.04 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 777.73 µs | 3.0931 ms | 569975 | 240525 | 232423 |
| [bitcode 0.5.0][bitcode] | 470.40 µs | 3.2209 ms | 322798 | 214279 | 201247 |
| [borsh 1.1.1][borsh] | 688.84 µs | 3.1308 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 4.0343 ms | 13.540 ms | 1619653 | 502185 | 328399 |
| [capnp 0.18.3][capnp] | 682.54 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.9928 ms | 11.920 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 1.0193 ms | 4.1593 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 4.1043 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.4454 ms | 4.5857 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.7886 ms | 6.1819 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.35][nanoserde] | 411.66 µs | 3.2234 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 794.12 µs | 3.3600 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.8][postcard] | 578.71 µs | 3.3882 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 3.4414 ms | 8.9474 ms | 599125 | 299158 | 247693 |
| [prost 0.12.1][prost] | <span title="encode">*1.5207 ms\**</span> <span title="populate + encode">*4.5084 ms\**</span> | 5.5722 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 484.91 µs | <span title="unvalidated">*2.2250 ms\**</span> <span title="validated upfront with error">*3.1017 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.8476 ms | 4.7524 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 12.362 ms | 28.052 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.2][savefile] | 321.73 µs | 3.1018 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 970.39 µs | 3.7969 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.4532 ms | 7.9179 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.108][serde_json] | 5.3475 ms | 11.601 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.13.4][simd-json] | 3.1157 ms | 7.3361 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 429.82 µs | 2.8148 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*70.613 µs\**</span> | <span title="unvalidated">*72.761 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.0825 ns\**</span> | <span title="validated on-demand with panic">*8.6129 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*93.871 ns\**</span> | <span title="validated on-demand with error">*722.72 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.8790 ns\**</span> <span title="validated upfront with error">*2.7386 ms\**</span> | <span title="unvalidated">*2.6801 µs\**</span> <span title="validated upfront with error">*2.7514 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.8043 ns\**</span> <span title="validated upfront with error">*835.50 µs\**</span> | <span title="unvalidated">*223.55 ns\**</span> <span title="validated upfront with error">*825.69 µs\**</span> | 1.8520 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.05%\**</span> | 25.01% | 53.95% | 59.90% |
| [alkahest 0.1.5][alkahest] | 79.61% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 35.93% | 71.93% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.5.0][bitcode] | 59.41% | 69.08% | 100.00% | 99.39% | 98.63% |
| [borsh 1.1.1][borsh] | 40.57% | 71.07% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.93% | 16.43% | 19.93% | 42.41% | 60.44% |
| [capnp 0.18.3][capnp] | 40.95% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 7.00% | 18.67% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 27.42% | 53.49% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 6.81% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 19.34% | 48.52% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.18% | 35.99% | 71.77% | 84.37% | 85.88% |
| [nanoserde 0.1.35][nanoserde] | 67.89% | 69.03% | 56.83% | 88.77% | 85.40% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 35.19% | 66.22% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.8][postcard] | 48.29% | 65.67% | 87.84% | 95.97% | 95.73% |
| [pot 3.0.0][pot] | 8.12% | 24.87% | 53.88% | 71.19% | 80.13% |
| [prost 0.12.1][prost] | <span title="encode">*18.38%\**</span> <span title="populate + encode">*6.20%\**</span> | 39.93% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 57.63% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.73%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.2][rmp-serde] | 15.13% | 46.82% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.1][ron] | 2.26% | 7.93% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.2][savefile] | 86.86% | 71.73% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 28.80% | 58.60% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.39% | 28.10% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.108][serde_json] | 5.23% | 19.18% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.13.4][simd-json] | 8.97% | 30.33% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 65.02% | 79.05% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.31%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.53%\**</span> | <span title="validated on-demand with panic">*2.60%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.92%\**</span> | <span title="validated on-demand with error">*30.93%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.51%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*8.34%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 664.87 µs | <span title="unvalidated">*3.9000 ms\**</span> | 2984682 | 1427005 | 1298573 |
| [alkahest 0.1.5][alkahest] | 1.2571 ms | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 3.8584 ms | 6.4656 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.5.0][bitcode] | 1.7134 ms | 5.6703 ms | 870693 | 866738 | 870720 |
| [borsh 1.1.1][borsh] | 2.9782 ms | 4.5021 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 33.618 ms | 72.867 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.18.3][capnp] | 3.0267 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 24.877 ms | 53.666 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.7217 ms | 13.006 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 6.4173 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 2.9897 ms | 9.4501 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 51.682 ms | 26.351 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.35][nanoserde] | 1.3337 ms | 5.8811 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 3.3964 ms | 4.9870 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.8][postcard] | 2.3867 ms | 6.6644 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 21.078 ms | 42.733 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.12.1][prost] | <span title="encode">*6.3176 ms\**</span> <span title="populate + encode">*13.339 ms\**</span> | 13.470 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.9678 ms | <span title="unvalidated">*3.6525 ms\**</span> <span title="validated upfront with error">*4.7038 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 12.696 ms | 16.725 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 54.386 ms | 155.39 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.2][savefile] | 1.3310 ms | 4.7158 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 5.7692 ms | 7.7172 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 12.607 ms | 32.619 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.108][serde_json] | 30.495 ms | 49.808 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.13.4][simd-json] | 15.477 ms | 46.621 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 1.1998 ms | 4.2887 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*134.12 µs\**</span> | <span title="unvalidated">*135.67 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.0626 ns\**</span> | <span title="validated on-demand with panic">*933.53 ns\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*93.166 ns\**</span> | <span title="validated on-demand with error">*2.0752 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.8447 ns\**</span> <span title="validated upfront with error">*5.6132 ms\**</span> | <span title="unvalidated">*4.8333 µs\**</span> <span title="validated upfront with error">*5.6064 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7939 ns\**</span> <span title="validated upfront with error">*1.0260 ms\**</span> | <span title="unvalidated">*466.38 ns\**</span> <span title="validated upfront with error">*1.0287 ms\**</span> | 909.88 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.65%\**</span> | 29.17% | 60.74% | 67.05% |
| [alkahest 0.1.5][alkahest] | 52.89% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 17.23% | 56.49% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.5.0][bitcode] | 38.80% | 64.41% | 100.00% | 100.00% | 100.00% |
| [borsh 1.1.1][borsh] | 22.32% | 81.13% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 1.98% | 5.01% | 8.68% | 30.59% | 54.39% |
| [capnp 0.18.3][capnp] | 21.97% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.67% | 6.81% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 9.89% | 28.08% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 10.36% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 22.24% | 38.65% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.29% | 13.86% | 50.37% | 69.47% | 70.60% |
| [nanoserde 0.1.35][nanoserde] | 49.85% | 62.11% | 49.18% | 78.20% | 84.54% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 19.58% | 73.24% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.8][postcard] | 27.86% | 54.81% | 68.04% | 81.90% | 85.64% |
| [pot 3.0.0][pot] | 3.15% | 8.55% | 34.21% | 59.88% | 68.65% |
| [prost 0.12.1][prost] | <span title="encode">*10.52%\**</span> <span title="populate + encode">*4.98%\**</span> | 27.12% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 33.79% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.65%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.2][rmp-serde] | 5.24% | 21.84% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.1][ron] | 1.22% | 2.35% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.2][savefile] | 49.95% | 77.45% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 11.52% | 47.33% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.27% | 11.20% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.108][serde_json] | 2.18% | 7.33% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.13.4][simd-json] | 4.30% | 7.83% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 55.42% | 85.17% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.34%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.57%\**</span> | <span title="validated on-demand with panic">*49.96%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.93%\**</span> | <span title="validated on-demand with error">*22.47%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.66%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*9.65%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.5.0
[borsh]: https://crates.io/crates/borsh/1.1.1
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.18.3
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
