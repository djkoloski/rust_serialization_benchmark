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

## Last updated: 2023-9-6 12:8:26

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 242.85 µs | <span title="unvalidated">*2.4152 ms\**</span> | 1705800 | 530419 | 403367 |
| [alkahest 0.1.5][alkahest] | 229.96 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 591.86 µs | 3.2996 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.4.0][bitcode] | 495.07 µs | 3.4693 ms | 703664 | 317711 | 273622 |
| [borsh 0.10.3][borsh] | 491.27 µs | 3.4357 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 2.7687 ms | 10.304 ms | 1924682 | 532821 | 376270 |
| [capnp 0.16.1][capnp] | 705.18 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 3.5254 ms | 11.092 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 802.66 µs | 4.0869 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.8153 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.7396 ms | 4.0859 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.1921 ms | 5.8464 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.35][nanoserde] | 323.45 µs | 3.3149 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 631.52 µs | 3.6102 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.7][postcard] | 412.28 µs | 3.5978 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 3.0368 ms | 8.5209 ms | 971922 | 372513 | 304122 |
| [prost 0.11.9][prost] | <span title="encode">*565.64 µs\**</span> <span title="populate + encode">*3.2990 ms\**</span> | 4.1358 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 311.34 µs | <span title="unvalidated">*2.4982 ms\**</span> <span title="validated upfront with error">*3.5208 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.4161 ms | 4.7744 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 20.805 ms | 23.854 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.2][savefile] | 242.52 µs | 3.2516 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 812.88 µs | 3.3555 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0946 ms | 6.8775 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.105][serde_json] | 4.4715 ms | 8.6854 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.3071 ms | 6.0737 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 284.37 µs | 2.8472 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*38.759 µs\**</span> | <span title="unvalidated">*64.057 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6119 ns\**</span> | <span title="validated on-demand with panic">*56.929 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*106.82 ns\**</span> | <span title="validated on-demand with error">*390.96 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2815 ns\**</span> <span title="validated upfront with error">*2.1994 ms\**</span> | <span title="unvalidated">*103.07 µs\**</span> <span title="validated upfront with error">*2.2939 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5263 ns\**</span> <span title="validated upfront with error">*1.0090 ms\**</span> | <span title="unvalidated">*16.407 µs\**</span> <span title="validated upfront with error">*1.0243 ms\**</span> | 23.450 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 94.69% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 50.55% | 56.51% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 38.85% | 73.20% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.4.0][bitcode] | 46.45% | 69.62% | 100.00% | 84.40% | 83.31% |
| [borsh 0.10.3][borsh] | 46.81% | 70.30% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.31% | 23.44% | 36.56% | 50.32% | 60.58% |
| [capnp 0.16.1][capnp] | 32.61% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 6.52% | 21.77% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 28.65% | 59.10% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 12.67% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 13.22% | 59.11% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.81% | 41.31% | 85.95% | 80.63% | 79.84% |
| [nanoserde 0.1.35][nanoserde] | 71.10% | 72.86% | 67.29% | 71.86% | 73.12% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 36.41% | 66.90% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.7][postcard] | 55.78% | 67.13% | 97.06% | 88.67% | 89.83% |
| [pot 3.0.0][pot] | 7.57% | 28.34% | 72.40% | 71.98% | 74.95% |
| [prost 0.11.9][prost] | <span title="encode">*40.65%\**</span> <span title="populate + encode">*6.97%\**</span> | 58.40% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 73.86% | <span title="unvalidated">*96.68%\**</span> <span title="validated upfront with error">*68.60%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.2][rmp-serde] | 16.24% | 50.59% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.1][ron] | 1.11% | 10.12% | 43.77% | 59.70% | 65.18% |
| [savefile 0.16.2][savefile] | 94.82% | 74.28% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 28.29% | 71.98% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 10.98% | 35.12% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.105][serde_json] | 5.14% | 27.81% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.9.2][simd-json] | 9.97% | 39.76% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 80.87% | 84.83% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*25.61%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.44%\**</span> | <span title="validated on-demand with panic">*28.82%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.43%\**</span> | <span title="validated on-demand with error">*4.20%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.51%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*15.92%\**</span> <span title="validated upfront with error">*0.72%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.60%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 474.80 µs | <span title="unvalidated">*471.11 µs\**</span> | 6000024 | 5378513 | 5345890 |
| [alkahest 0.1.5][alkahest] | 477.37 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 4.9524 ms | 5.3177 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.4.0][bitcode] | 5.0741 ms | 8.6807 ms | 4688054 | 4688491 | 4688168 |
| [borsh 0.10.3][borsh] | 5.7739 ms | 7.4393 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 54.702 ms | 108.20 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.16.1][capnp] | 10.560 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 64.955 ms | 99.168 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 7.5648 ms | 9.9538 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.0684 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 20.121 ms | 8.4935 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 199.51 ms | 38.775 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.35][nanoserde] | 1.9673 ms | 1.2476 ms | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 5.3204 ms | 4.8881 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.7][postcard] | 1.0983 ms | 1.9068 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 50.342 ms | 85.308 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.11.9][prost] | <span title="encode">*11.821 ms\**</span> <span title="populate + encode">*14.701 ms\**</span> | 17.081 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 518.61 µs | <span title="unvalidated">*471.25 µs\**</span> <span title="validated upfront with error">*470.17 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 18.919 ms | 20.729 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 223.66 ms | 398.39 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.2][savefile] | 472.51 µs | 468.37 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.3209 ms | 5.2021 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 38.991 ms | 57.509 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.105][serde_json] | 101.55 ms | 87.895 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.9.2][simd-json] | 66.823 ms | 132.04 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 467.29 µs | 467.72 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.4783 ns\**</span> | <span title="unvalidated">*255.28 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6132 ns\**</span> | <span title="validated on-demand with panic">*83.727 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*178.36 ns\**</span> | <span title="validated on-demand with error">*5.6734 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2717 ns\**</span> <span title="validated upfront with error">*43.497 ns\**</span> | <span title="unvalidated">*83.777 µs\**</span> <span title="validated upfront with error">*83.837 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5230 ns\**</span> <span title="validated upfront with error">*13.178 ns\**</span> | <span title="unvalidated">*47.141 µs\**</span> <span title="validated upfront with error">*47.081 µs\**</span> | 243.72 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 98.42% | <span title="unvalidated">*99.28%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 97.89% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 9.44% | 8.80% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.4.0][bitcode] | 9.21% | 5.39% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 8.09% | 6.29% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.85% | 0.43% | 20.37% | 50.89% | 62.53% |
| [capnp 0.16.1][capnp] | 4.43% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.72% | 0.47% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 6.18% | 4.70% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 43.74% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 2.32% | 5.51% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.23% | 1.21% | 57.70% | 72.20% | 73.40% |
| [nanoserde 0.1.35][nanoserde] | 23.75% | 37.49% | 78.13% | 87.17% | 87.70% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 8.78% | 9.57% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.7][postcard] | 42.55% | 24.53% | 78.13% | 87.17% | 87.70% |
| [pot 3.0.0][pot] | 0.93% | 0.55% | 46.31% | 68.80% | 68.42% |
| [prost 0.11.9][prost] | <span title="encode">*3.95%\**</span> <span title="populate + encode">*3.18%\**</span> | 2.74% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 90.10% | <span title="unvalidated">*99.25%\**</span> <span title="validated upfront with error">*99.48%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.2][rmp-serde] | 2.47% | 2.26% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.1][ron] | 0.21% | 0.12% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.2][savefile] | 98.90% | 99.86% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 7.39% | 8.99% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 1.20% | 0.81% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.105][serde_json] | 0.46% | 0.53% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.70% | 0.35% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 100.00% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.45%\**</span> | <span title="unvalidated">*18.44%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.28%\**</span> | <span title="validated on-demand with panic">*56.23%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.85%\**</span> | <span title="validated on-demand with error">*0.83%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.55%\**</span> <span title="validated upfront with error">*3.50%\**</span> | <span title="unvalidated">*56.20%\**</span> <span title="validated upfront with error">*56.16%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*11.56%\**</span> | <span title="unvalidated">*99.87%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 241.38 µs | <span title="unvalidated">*1.9967 ms\**</span> | 1290592 | 395756 | 333564 |
| [alkahest 0.1.5][alkahest] | 290.98 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 669.73 µs | 2.6020 ms | 569975 | 240525 | 232423 |
| [bitcode 0.4.0][bitcode] | 409.43 µs | 2.7052 ms | 322798 | 214279 | 201247 |
| [borsh 0.10.3][borsh] | 580.43 µs | 2.7355 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 3.4984 ms | 11.684 ms | 1619653 | 502185 | 328399 |
| [capnp 0.16.1][capnp] | 596.03 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.5043 ms | 10.266 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 858.61 µs | 3.7220 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.4304 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.2201 ms | 3.9914 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.5180 ms | 5.0801 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.35][nanoserde] | 348.05 µs | 2.7233 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 661.85 µs | 2.8281 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.7][postcard] | 482.16 µs | 2.8611 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 3.1095 ms | 7.7676 ms | 599125 | 299158 | 247693 |
| [prost 0.11.9][prost] | <span title="encode">*1.3080 ms\**</span> <span title="populate + encode">*3.8248 ms\**</span> | 4.8362 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 412.74 µs | <span title="unvalidated">*1.9005 ms\**</span> <span title="validated upfront with error">*2.6777 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.5917 ms | 4.0363 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 10.978 ms | 23.847 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.2][savefile] | 299.22 µs | 2.6685 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 927.78 µs | 3.2126 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0706 ms | 6.8532 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.105][serde_json] | 4.4635 ms | 9.8849 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.5356 ms | 5.7210 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 403.80 µs | 2.4165 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.071 µs\**</span> | <span title="unvalidated">*62.814 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6127 ns\**</span> | <span title="validated on-demand with panic">*7.0659 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*106.87 ns\**</span> | <span title="validated on-demand with error">*647.19 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2711 ns\**</span> <span title="validated upfront with error">*2.2910 ms\**</span> | <span title="unvalidated">*2.6751 µs\**</span> <span title="validated upfront with error">*2.3096 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5241 ns\**</span> <span title="validated upfront with error">*763.90 µs\**</span> | <span title="unvalidated">*190.16 ns\**</span> <span title="validated upfront with error">*763.87 µs\**</span> | 1.5989 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.18%\**</span> | 25.01% | 53.81% | 59.51% |
| [alkahest 0.1.5][alkahest] | 82.95% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 36.04% | 73.04% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.4.0][bitcode] | 58.96% | 70.25% | 100.00% | 99.39% | 98.63% |
| [borsh 0.10.3][borsh] | 41.59% | 69.48% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.90% | 16.27% | 19.93% | 42.41% | 60.44% |
| [capnp 0.16.1][capnp] | 40.50% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.89% | 18.51% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 28.11% | 51.06% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 7.04% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 19.78% | 47.61% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.21% | 37.41% | 71.77% | 84.37% | 85.88% |
| [nanoserde 0.1.35][nanoserde] | 69.35% | 69.79% | 56.83% | 88.77% | 85.40% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 36.47% | 67.20% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.7][postcard] | 50.06% | 66.43% | 87.84% | 95.97% | 95.73% |
| [pot 3.0.0][pot] | 7.76% | 24.47% | 53.88% | 71.19% | 80.13% |
| [prost 0.11.9][prost] | <span title="encode">*18.45%\**</span> <span title="populate + encode">*6.31%\**</span> | 39.30% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 58.48% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*70.98%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.2][rmp-serde] | 15.16% | 47.09% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.1][ron] | 2.20% | 7.97% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.2][savefile] | 80.67% | 71.22% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 26.02% | 59.16% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.66% | 27.73% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.105][serde_json] | 5.41% | 19.23% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.9.2][simd-json] | 9.52% | 33.22% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 59.78% | 78.65% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.30%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.33%\**</span> | <span title="validated on-demand with panic">*2.69%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.43%\**</span> | <span title="validated on-demand with error">*29.38%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.59%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.11%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 583.32 µs | <span title="unvalidated">*3.3207 ms\**</span> | 2984682 | 1449887 | 1311484 |
| [alkahest 0.1.5][alkahest] | 1.0025 ms | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 3.5429 ms | 6.0405 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.5618 ms | 4.8114 ms | 870693 | 866738 | 870720 |
| [borsh 0.10.3][borsh] | 2.5970 ms | 5.3904 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 29.379 ms | 63.054 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.16.1][capnp] | 2.5502 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 21.653 ms | 45.725 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.0547 ms | 11.560 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.3229 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.1942 ms | 8.6879 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 44.200 ms | 20.167 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.35][nanoserde] | 1.3730 ms | 4.9099 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 2.8326 ms | 4.5063 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.7][postcard] | 2.1629 ms | 5.8238 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 17.815 ms | 38.223 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.11.9][prost] | <span title="encode">*5.3680 ms\**</span> <span title="populate + encode">*11.554 ms\**</span> | 11.782 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.5915 ms | <span title="unvalidated">*3.0997 ms\**</span> <span title="validated upfront with error">*3.9759 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 11.254 ms | 14.097 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 46.862 ms | 132.18 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.2][savefile] | 1.0982 ms | 3.9823 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.9221 ms | 6.6685 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 10.841 ms | 27.882 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.105][serde_json] | 25.721 ms | 42.877 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.9.2][simd-json] | 13.342 ms | 40.602 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 987.11 µs | 3.7029 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*120.11 µs\**</span> | <span title="unvalidated">*120.29 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6114 ns\**</span> | <span title="validated on-demand with panic">*792.95 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*106.80 ns\**</span> | <span title="validated on-demand with error">*1.8237 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2715 ns\**</span> <span title="validated upfront with error">*5.8505 ms\**</span> | <span title="unvalidated">*5.4581 µs\**</span> <span title="validated upfront with error">*4.8011 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5225 ns\**</span> <span title="validated upfront with error">*813.90 µs\**</span> | <span title="unvalidated">*397.04 ns\**</span> <span title="validated upfront with error">*817.51 µs\**</span> | 770.53 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.34%\**</span> | 29.17% | 59.78% | 66.39% |
| [alkahest 0.1.5][alkahest] | 58.19% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 16.46% | 51.32% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.4.0][bitcode] | 37.35% | 64.42% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 22.46% | 57.50% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 1.99% | 4.92% | 8.68% | 30.59% | 54.39% |
| [capnp 0.16.1][capnp] | 22.87% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.69% | 6.78% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 9.63% | 26.81% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 10.96% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 13.91% | 35.68% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.32% | 15.37% | 50.37% | 69.47% | 70.60% |
| [nanoserde 0.1.35][nanoserde] | 42.49% | 63.13% | 49.18% | 78.20% | 84.54% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 20.59% | 68.79% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.7][postcard] | 26.97% | 53.22% | 68.04% | 81.90% | 85.64% |
| [pot 3.0.0][pot] | 3.27% | 8.11% | 34.21% | 59.88% | 68.65% |
| [prost 0.11.9][prost] | <span title="encode">*10.87%\**</span> <span title="populate + encode">*5.05%\**</span> | 26.31% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 36.65% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.96%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.2][rmp-serde] | 5.18% | 21.99% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.1][ron] | 1.24% | 2.35% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.2][savefile] | 53.12% | 77.84% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 11.85% | 46.48% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.38% | 11.12% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.105][serde_json] | 2.27% | 7.23% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.37% | 7.63% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 59.09% | 83.71% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.33%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.30%\**</span> | <span title="validated on-demand with panic">*50.07%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.43%\**</span> | <span title="validated on-demand with error">*21.77%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.54%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.27%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.4.0
[borsh]: https://crates.io/crates/borsh/0.10.3
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.16.1
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.35
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.5
[postcard]: https://crates.io/crates/postcard/1.0.7
[pot]: https://crates.io/crates/pot/3.0.0
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.16.2
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.105
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
