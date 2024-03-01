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

## Last updated: 2024-3-1 3:45:34

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 393.20 µs | <span title="unvalidated">*1.4639 ms\**</span> | 1705800 | 520081 | 413498 |
| [alkahest 0.1.5][alkahest] | 194.89 µs | † | 1045784 | 454157 | 389424 |
| [bincode 2.0.0-rc][bincode] | 211.85 µs | 2.4572 ms | 741295 | 303944 | 257153 |
| [bincode 1.3.3][bincode1] | 526.99 µs | 1.9938 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.6.0-alpha.2][bitcode] | 138.80 µs | 1.4476 ms | 703710 | 288826 | 229755 |
| [borsh 1.3.0][borsh] | 541.94 µs | 2.2235 ms | 885780 | 362204 | 286514 |
| [bson 2.9.0][bson] | 2.2601 ms | 7.3480 ms | 1924682 | 532821 | 376270 |
| [capnp 0.18.13][capnp] | 479.06 µs | † | 1443216 | 513986 | 428649 |
| [cbor4ii 0.3.2][cbor4ii] | 901.49 µs | 4.8897 ms | 1407835 | 403440 | 324081 |
| [ciborium 0.2.2][ciborium] | 3.9277 ms | 10.054 ms | 1407835 | 403440 | 324081 |
| [databuf 0.5.0][databuf] | 266.05 µs | 2.0342 ms | 765778 | 311715 | 264630 |
| [dlhn 0.1.6][dlhn] | 790.53 µs | 2.4411 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.5325 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.2065 ms | 2.8210 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5523 ms | 4.0755 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.37][nanoserde] | 387.11 µs | 2.0436 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 680.36 µs | 2.2352 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.8][postcard] | 418.89 µs | 2.1846 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 2.3453 ms | 6.7181 ms | 971922 | 372513 | 304122 |
| [prost 0.12.3][prost] | <span title="encode">*822.08 µs\**</span> <span title="populate + encode">*2.6974 ms\**</span> | 3.5366 ms | 884628 | 363130 | 315494 |
| [rkyv 0.7.44][rkyv] | 214.18 µs | <span title="unvalidated">*1.4431 ms\**</span> <span title="validated upfront with error">*2.0079 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.3560 ms | 3.6421 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 14.025 ms | 15.084 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.5][savefile] | 205.67 µs | 2.0701 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 688.26 µs | 2.4263 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 1.9345 ms | 5.1153 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.114][serde_json] | 3.7869 ms | 5.8364 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.13.8][simd-json] | 2.0694 ms | 4.6571 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.7][speedy] | 198.96 µs | 1.7494 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.109 µs\**</span> | <span title="unvalidated">*37.918 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8560 ns\**</span> | <span title="validated on-demand with panic">*24.790 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.349 ns\**</span> | <span title="validated on-demand with error">*171.63 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4744 ns\**</span> <span title="validated upfront with error">*1.8310 ms\**</span> | <span title="unvalidated">*50.798 µs\**</span> <span title="validated upfront with error">*1.9471 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2375 ns\**</span> <span title="validated upfront with error">*563.79 µs\**</span> | <span title="unvalidated">*10.565 µs\**</span> <span title="validated upfront with error">*554.40 µs\**</span> | 9.7890 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 35.30% | <span title="unvalidated">*98.58%\**</span> | 41.25% | 55.53% | 55.56% |
| [alkahest 0.1.5][alkahest] | 71.22% | † | 67.29% | 63.60% | 59.00% |
| [bincode 2.0.0-rc][bincode] | 65.52% | 58.73% | 94.93% | 95.03% | 89.35% |
| [bincode 1.3.3][bincode1] | 26.34% | 72.38% | 67.29% | 77.41% | 73.70% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 99.69% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 25.61% | 64.90% | 79.45% | 79.74% | 80.19% |
| [bson 2.9.0][bson] | 6.14% | 19.64% | 36.56% | 54.21% | 61.06% |
| [capnp 0.18.13][capnp] | 28.97% | † | 48.76% | 56.19% | 53.60% |
| [cbor4ii 0.3.2][cbor4ii] | 15.40% | 29.51% | 49.99% | 71.59% | 70.89% |
| [ciborium 0.2.2][ciborium] | 3.53% | 14.35% | 49.99% | 71.59% | 70.89% |
| [databuf 0.5.0][databuf] | 52.17% | 70.94% | 91.89% | 92.66% | 86.82% |
| [dlhn 0.1.6][dlhn] | 17.56% | 59.12% | 97.07% | 95.81% | 90.59% |
| [flatbuffers 23.5.26][flatbuffers] | 9.06% | † | 55.13% | 61.64% | 59.09% |
| [msgpacker 0.4.3][msgpacker] | 11.50% | 51.16% | 91.99% | 91.61% | 86.73% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.50% | 35.41% | 85.96% | 86.85% | 80.47% |
| [nanoserde 0.1.37][nanoserde] | 35.86% | 70.62% | 67.29% | 77.41% | 73.70% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.40% | 64.56% | 91.89% | 92.65% | 86.86% |
| [postcard 1.0.8][postcard] | 33.14% | 66.06% | 97.07% | 95.51% | 90.54% |
| [pot 3.0.0][pot] | 5.92% | 21.48% | 72.40% | 77.53% | 75.55% |
| [prost 0.12.3][prost] | <span title="encode">*16.88%\**</span> <span title="populate + encode">*5.15%\**</span> | 40.80% | 79.55% | 79.54% | 72.82% |
| [rkyv 0.7.44][rkyv] | 64.81% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.87%\**</span> | 69.57% | 75.24% | 68.88% |
| [rmp-serde 1.1.2][rmp-serde] | 10.24% | 39.62% | 89.64% | 88.76% | 82.58% |
| [ron 0.8.1][ron] | 0.99% | 9.57% | 43.78% | 64.30% | 65.70% |
| [savefile 0.16.5][savefile] | 67.49% | 69.71% | 67.29% | 77.40% | 73.70% |
| [serde_bare 0.5.0][serde_bare] | 20.17% | 59.48% | 91.89% | 92.66% | 86.82% |
| [serde_cbor 0.11.2][serde_cbor] | 7.17% | 28.21% | 49.99% | 71.59% | 70.89% |
| [serde_json 1.0.114][serde_json] | 3.67% | 24.73% | 38.51% | 61.38% | 63.63% |
| [simd-json 0.13.8][simd-json] | 6.71% | 30.99% | 38.51% | 61.38% | 63.63% |
| [speedy 0.8.7][speedy] | 69.76% | 82.49% | 79.45% | 79.74% | 80.19% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*27.86%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.68%\**</span> | <span title="validated on-demand with panic">*42.62%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*6.16%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.80%\**</span> <span title="validated upfront with error">*0.54%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.91%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 241.45 µs | <span title="unvalidated">*242.66 µs\**</span> | 6000024 | 5378513 | 5345890 |
| [alkahest 0.1.5][alkahest] | 151.11 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 2.0.0-rc][bincode] | 421.53 µs | 824.79 µs | 6000005 | 5378497 | 5345897 |
| [bincode 1.3.3][bincode1] | 5.1226 ms | 4.5051 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.6.0-alpha.2][bitcode] | 1.4656 ms | 618.13 µs | 6000006 | 5182295 | 4923880 |
| [borsh 1.3.0][borsh] | 6.1277 ms | 4.5499 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.9.0][bson] | 44.070 ms | 78.679 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.18.13][capnp] | 6.3762 ms | † | 14000088 | 7130367 | 6051062 |
| [cbor4ii 0.3.2][cbor4ii] | 10.528 ms | 46.911 ms | 13125016 | 7524114 | 6757967 |
| [ciborium 0.2.2][ciborium] | 66.558 ms | 100.35 ms | 13122324 | 7524660 | 6759658 |
| [databuf 0.5.0][databuf] | 2.4025 ms | 5.2896 ms | 6000003 | 5378495 | 5345900 |
| [dlhn 0.1.6][dlhn] | 7.1967 ms | 5.8424 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 659.53 µs | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 21.105 ms | 9.0175 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.84 ms | 26.472 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.37][nanoserde] | 1.0603 ms | 900.69 µs | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 5.1546 ms | 4.1208 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.8][postcard] | 476.48 µs | 1.5345 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 40.406 ms | 72.472 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.12.3][prost] | <span title="encode">*7.0164 ms\**</span> <span title="populate + encode">*8.3076 ms\**</span> | 13.488 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.44][rkyv] | 184.88 µs | <span title="unvalidated">*150.12 µs\**</span> <span title="validated upfront with error">*149.78 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 13.576 ms | 18.967 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 175.02 ms | 260.56 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.5][savefile] | 240.56 µs | 244.37 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.2446 ms | 3.8079 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 35.978 ms | 41.863 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.114][serde_json] | 91.241 ms | 71.799 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.13.8][simd-json] | 54.059 ms | 72.869 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.7][speedy] | 240.27 µs | 240.72 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1643 ns\**</span> | <span title="unvalidated">*141.53 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8586 ns\**</span> | <span title="validated on-demand with panic">*77.379 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*107.93 ns\**</span> | <span title="validated on-demand with error">*2.1914 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4738 ns\**</span> <span title="validated upfront with error">*37.436 ns\**</span> | <span title="unvalidated">*54.161 µs\**</span> <span title="validated upfront with error">*77.448 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2362 ns\**</span> <span title="validated upfront with error">*6.2180 ns\**</span> | <span title="unvalidated">*45.933 µs\**</span> <span title="validated upfront with error">*38.703 µs\**</span> | 107.63 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 62.58% | <span title="unvalidated">*61.72%\**</span> | 100.00% | 96.35% | 92.11% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% |
| [bincode 2.0.0-rc][bincode] | 35.85% | 18.16% | 100.00% | 96.35% | 92.11% |
| [bincode 1.3.3][bincode1] | 2.95% | 3.32% | 100.00% | 96.35% | 92.11% |
| [bitcode 0.6.0-alpha.2][bitcode] | 10.31% | 24.23% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 2.47% | 3.29% | 100.00% | 96.35% | 92.11% |
| [bson 2.9.0][bson] | 0.34% | 0.19% | 26.07% | 56.26% | 65.67% |
| [capnp 0.18.13][capnp] | 2.37% | † | 42.86% | 72.68% | 81.37% |
| [cbor4ii 0.3.2][cbor4ii] | 1.44% | 0.32% | 45.71% | 68.88% | 72.86% |
| [ciborium 0.2.2][ciborium] | 0.23% | 0.15% | 45.72% | 68.87% | 72.84% |
| [databuf 0.5.0][databuf] | 6.29% | 2.83% | 100.00% | 96.35% | 92.11% |
| [dlhn 0.1.6][dlhn] | 2.10% | 2.56% | 100.00% | 96.35% | 92.11% |
| [flatbuffers 23.5.26][flatbuffers] | 22.91% | † | 100.00% | 96.35% | 92.11% |
| [msgpacker 0.4.3][msgpacker] | 0.72% | 1.66% | 80.00% | 85.54% | 81.87% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.57% | 73.85% | 79.81% | 77.09% |
| [nanoserde 0.1.37][nanoserde] | 14.25% | 16.63% | 100.00% | 96.35% | 92.11% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.93% | 3.63% | 100.00% | 96.35% | 92.11% |
| [postcard 1.0.8][postcard] | 31.71% | 9.76% | 100.00% | 96.35% | 92.11% |
| [pot 3.0.0][pot] | 0.37% | 0.21% | 59.27% | 76.05% | 71.86% |
| [prost 0.12.3][prost] | <span title="encode">*2.15%\**</span> <span title="populate + encode">*1.82%\**</span> | 1.11% | 68.57% | 77.75% | 76.67% |
| [rkyv 0.7.44][rkyv] | 81.73% | <span title="unvalidated">*99.77%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% |
| [rmp-serde 1.1.2][rmp-serde] | 1.11% | 0.79% | 73.85% | 79.79% | 77.04% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% |
| [savefile 0.16.5][savefile] | 62.82% | 61.29% | 100.00% | 96.35% | 92.11% |
| [serde_bare 0.5.0][serde_bare] | 2.42% | 3.93% | 100.00% | 96.35% | 92.11% |
| [serde_cbor 0.11.2][serde_cbor] | 0.42% | 0.36% | 45.72% | 68.87% | 72.84% |
| [serde_json 1.0.114][serde_json] | 0.17% | 0.21% | 22.91% | 54.17% | 57.34% |
| [simd-json 0.13.8][simd-json] | 0.28% | 0.21% | 22.91% | 54.17% | 57.34% |
| [speedy 0.8.7][speedy] | 62.89% | 62.22% | 100.00% | 96.35% | 92.11% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.12%\**</span> | <span title="unvalidated">*27.35%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.51%\**</span> | <span title="validated on-demand with panic">*50.02%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.15%\**</span> | <span title="validated on-demand with error">*1.77%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*3.30%\**</span> | <span title="unvalidated">*71.46%\**</span> <span title="validated upfront with error">*49.97%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*19.88%\**</span> | <span title="unvalidated">*84.26%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 190.16 µs | <span title="unvalidated">*1.3071 ms\**</span> | 1290592 | 396682 | 340686 |
| [alkahest 0.1.5][alkahest] | 220.50 µs | † | 667570 | 325484 | 320452 |
| [bincode 2.0.0-rc][bincode] | 285.30 µs | 2.0600 ms | 367413 | 221291 | 206273 |
| [bincode 1.3.3][bincode1] | 581.23 µs | 1.8147 ms | 569975 | 240525 | 232423 |
| [bitcode 0.6.0-alpha.2][bitcode] | 125.32 µs | 1.2680 ms | 327688 | 200947 | 182736 |
| [borsh 1.3.0][borsh] | 549.50 µs | 1.8204 ms | 446595 | 234236 | 210008 |
| [bson 2.9.0][bson] | 2.9252 ms | 8.2596 ms | 1619653 | 502185 | 328399 |
| [capnp 0.18.13][capnp] | 451.92 µs | † | 803896 | 335606 | 280851 |
| [cbor4ii 0.3.2][cbor4ii] | 797.45 µs | 4.6426 ms | 1109831 | 344745 | 274514 |
| [ciborium 0.2.2][ciborium] | 3.7946 ms | 9.5416 ms | 1109821 | 344751 | 274526 |
| [databuf 0.5.0][databuf] | 317.74 µs | 1.7362 ms | 356311 | 213062 | 198488 |
| [dlhn 0.1.6][dlhn] | 803.64 µs | 2.5550 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.3519 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 937.26 µs | 2.7712 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2704 ms | 3.9217 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.37][nanoserde] | 285.37 µs | 1.8761 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 664.50 µs | 2.0179 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.8][postcard] | 430.15 µs | 1.9498 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 2.3493 ms | 6.0385 ms | 599125 | 299158 | 247693 |
| [prost 0.12.3][prost] | <span title="encode">*1.0029 ms\**</span> <span title="populate + encode">*2.6840 ms\**</span> | 3.6142 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.44][rkyv] | 301.46 µs | <span title="unvalidated">*1.2539 ms\**</span> <span title="validated upfront with error">*1.7697 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.4479 ms | 2.9719 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 8.2640 ms | 17.247 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.5][savefile] | 220.47 µs | 1.8313 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 734.38 µs | 2.2120 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 1.7730 ms | 4.7247 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.114][serde_json] | 3.8082 ms | 6.6363 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.13.8][simd-json] | 2.2286 ms | 4.5570 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.7][speedy] | 276.05 µs | 1.6429 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.627 µs\**</span> | <span title="unvalidated">*38.068 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8563 ns\**</span> | <span title="validated on-demand with panic">*4.7609 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.312 ns\**</span> | <span title="validated on-demand with error">*441.27 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4744 ns\**</span> <span title="validated upfront with error">*2.2037 ms\**</span> | <span title="unvalidated">*1.3779 µs\**</span> <span title="validated upfront with error">*2.2076 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2371 ns\**</span> <span title="validated upfront with error">*506.39 µs\**</span> | <span title="unvalidated">*239.01 ns\**</span> <span title="validated upfront with error">*508.00 µs\**</span> | 957.14 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 65.90% | <span title="unvalidated">*95.93%\**</span> | 25.39% | 50.66% | 53.64% |
| [alkahest 0.1.5][alkahest] | 56.83% | † | 49.09% | 61.74% | 57.02% |
| [bincode 2.0.0-rc][bincode] | 43.93% | 60.87% | 89.19% | 90.81% | 88.59% |
| [bincode 1.3.3][bincode1] | 21.56% | 69.10% | 57.49% | 83.55% | 78.62% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 98.89% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 22.81% | 68.88% | 73.37% | 85.79% | 87.01% |
| [bson 2.9.0][bson] | 4.28% | 15.18% | 20.23% | 40.01% | 55.64% |
| [capnp 0.18.13][capnp] | 27.73% | † | 40.76% | 59.88% | 65.07% |
| [cbor4ii 0.3.2][cbor4ii] | 15.72% | 27.01% | 29.53% | 58.29% | 66.57% |
| [ciborium 0.2.2][ciborium] | 3.30% | 13.14% | 29.53% | 58.29% | 66.56% |
| [databuf 0.5.0][databuf] | 39.44% | 72.22% | 91.97% | 94.31% | 92.06% |
| [dlhn 0.1.6][dlhn] | 15.59% | 49.08% | 89.41% | 91.09% | 88.84% |
| [flatbuffers 23.5.26][flatbuffers] | 3.74% | † | 38.82% | 58.13% | 62.15% |
| [msgpacker 0.4.3][msgpacker] | 13.37% | 45.25% | 83.75% | 84.83% | 82.88% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.38% | 31.97% | 72.86% | 79.60% | 79.07% |
| [nanoserde 0.1.37][nanoserde] | 43.91% | 66.84% | 57.69% | 83.75% | 78.62% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 18.86% | 62.14% | 91.97% | 94.35% | 92.05% |
| [postcard 1.0.8][postcard] | 29.13% | 64.31% | 89.17% | 90.55% | 88.13% |
| [pot 3.0.0][pot] | 5.33% | 20.77% | 54.69% | 67.17% | 73.78% |
| [prost 0.12.3][prost] | <span title="encode">*12.50%\**</span> <span title="populate + encode">*4.67%\**</span> | 34.69% | 54.91% | 65.82% | 67.85% |
| [rkyv 0.7.44][rkyv] | 41.57% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*70.85%\**</span> | 54.89% | 79.12% | 82.80% |
| [rmp-serde 1.1.2][rmp-serde] | 8.66% | 42.19% | 77.19% | 81.95% | 80.79% |
| [ron 0.8.1][ron] | 1.52% | 7.27% | 22.36% | 46.20% | 53.22% |
| [savefile 0.16.5][savefile] | 56.84% | 68.47% | 57.79% | 83.95% | 78.76% |
| [serde_bare 0.5.0][serde_bare] | 17.06% | 56.69% | 91.97% | 94.31% | 92.06% |
| [serde_cbor 0.11.2][serde_cbor] | 7.07% | 26.54% | 29.53% | 58.29% | 66.56% |
| [serde_json 1.0.114][serde_json] | 3.29% | 18.89% | 20.19% | 43.07% | 50.81% |
| [simd-json 0.13.8][simd-json] | 5.62% | 27.52% | 20.19% | 43.07% | 50.81% |
| [speedy 0.8.7][speedy] | 45.40% | 76.32% | 72.89% | 85.52% | 86.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.63%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.64%\**</span> | <span title="validated on-demand with panic">*5.02%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*54.16%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.35%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 491.94 µs | <span title="unvalidated">*2.3323 ms\**</span> | 2984682 | 1406317 | 1264707 |
| [alkahest 0.1.5][alkahest] | 748.57 µs | † | 1863391 | 1234113 | 1202345 |
| [bincode 2.0.0-rc][bincode] | 701.89 µs | 3.5223 ms | 1372381 | 1091486 | 1037296 |
| [bincode 1.3.3][bincode1] | 3.7117 ms | 3.9158 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.6.0-alpha.2][bitcode] | 704.98 µs | 2.3735 ms | 948499 | 857321 | 837658 |
| [borsh 1.3.0][borsh] | 2.8849 ms | 2.5492 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.9.0][bson] | 22.024 ms | 44.388 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.18.13][capnp] | 2.1883 ms | † | 2664040 | 1511895 | 1212087 |
| [cbor4ii 0.3.2][cbor4ii] | 4.2958 ms | 17.417 ms | 5878791 | 1655835 | 1431390 |
| [ciborium 0.2.2][ciborium] | 23.457 ms | 46.589 ms | 5878653 | 1655791 | 1431560 |
| [databuf 0.5.0][databuf] | 1.8045 ms | 3.5770 ms | 1288257 | 1037579 | 984337 |
| [dlhn 0.1.6][dlhn] | 5.1777 ms | 7.1219 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.2353 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 1.8843 ms | 4.5293 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 31.260 ms | 15.681 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.37][nanoserde] | 1.3791 ms | 2.9930 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.0479 ms | 3.0536 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.8][postcard] | 1.8338 ms | 3.9373 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 14.145 ms | 31.749 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.12.3][prost] | <span title="encode">*4.0489 ms\**</span> <span title="populate + encode">*8.1123 ms\**</span> | 8.8186 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.44][rkyv] | 1.2787 ms | <span title="unvalidated">*2.1664 ms\**</span> <span title="validated upfront with error">*2.7568 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 10.168 ms | 11.664 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 37.105 ms | 96.533 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.5][savefile] | 1.0104 ms | 2.6238 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.9328 ms | 4.4175 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 9.3731 ms | 21.870 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.114][serde_json] | 21.157 ms | 30.502 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.13.8][simd-json] | 11.488 ms | 26.352 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.7][speedy] | 711.16 µs | 2.5048 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*65.715 µs\**</span> | <span title="unvalidated">*66.659 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8556 ns\**</span> | <span title="validated on-demand with panic">*628.42 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.376 ns\**</span> | <span title="validated on-demand with error">*718.28 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4744 ns\**</span> <span title="validated upfront with error">*5.3888 ms\**</span> | <span title="unvalidated">*2.6538 µs\**</span> <span title="validated upfront with error">*5.3962 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2365 ns\**</span> <span title="validated upfront with error">*572.55 µs\**</span> | <span title="unvalidated">*352.87 ns\**</span> <span title="validated upfront with error">*575.30 µs\**</span> | 625.53 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.89%\**</span> | 31.78% | 60.96% | 66.23% |
| [alkahest 0.1.5][alkahest] | 65.72% | † | 50.90% | 69.47% | 69.67% |
| [bincode 2.0.0-rc][bincode] | 70.09% | 61.51% | 69.11% | 78.55% | 80.75% |
| [bincode 1.3.3][bincode1] | 13.25% | 55.32% | 52.37% | 76.87% | 81.67% |
| [bitcode 0.6.0-alpha.2][bitcode] | 69.78% | 91.27% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 17.05% | 84.98% | 63.82% | 79.21% | 82.65% |
| [bson 2.9.0][bson] | 2.23% | 4.88% | 9.46% | 30.26% | 52.33% |
| [capnp 0.18.13][capnp] | 22.48% | † | 35.60% | 56.71% | 69.11% |
| [cbor4ii 0.3.2][cbor4ii] | 11.45% | 12.44% | 16.13% | 51.78% | 58.52% |
| [ciborium 0.2.2][ciborium] | 2.10% | 4.65% | 16.13% | 51.78% | 58.51% |
| [databuf 0.5.0][databuf] | 27.26% | 60.56% | 73.63% | 82.63% | 85.10% |
| [dlhn 0.1.6][dlhn] | 9.50% | 30.42% | 74.12% | 81.49% | 82.03% |
| [flatbuffers 23.5.26][flatbuffers] | 9.40% | † | 41.72% | 60.87% | 67.80% |
| [msgpacker 0.4.3][msgpacker] | 26.11% | 47.83% | 66.61% | 75.95% | 75.45% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.57% | 13.82% | 54.87% | 68.72% | 67.92% |
| [nanoserde 0.1.37][nanoserde] | 35.67% | 72.38% | 53.57% | 77.35% | 81.33% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 16.14% | 70.95% | 73.63% | 82.49% | 84.91% |
| [postcard 1.0.8][postcard] | 26.83% | 55.02% | 74.12% | 81.01% | 82.39% |
| [pot 3.0.0][pot] | 3.48% | 6.82% | 37.27% | 59.23% | 66.04% |
| [prost 0.12.3][prost] | <span title="encode">*12.15%\**</span> <span title="populate + encode">*6.06%\**</span> | 24.57% | 52.16% | 65.56% | 66.15% |
| [rkyv 0.7.44][rkyv] | 38.47% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.58%\**</span> | 46.75% | 64.21% | 72.28% |
| [rmp-serde 1.1.2][rmp-serde] | 4.84% | 18.57% | 55.67% | 69.59% | 69.79% |
| [ron 0.8.1][ron] | 1.33% | 2.24% | 11.19% | 39.31% | 46.95% |
| [savefile 0.16.5][savefile] | 48.69% | 82.57% | 54.19% | 77.82% | 81.50% |
| [serde_bare 0.5.0][serde_bare] | 9.97% | 49.04% | 73.63% | 82.63% | 85.10% |
| [serde_cbor 0.11.2][serde_cbor] | 5.25% | 9.91% | 16.13% | 51.78% | 58.51% |
| [serde_json 1.0.114][serde_json] | 2.33% | 7.10% | 10.34% | 36.73% | 46.52% |
| [simd-json 0.13.8][simd-json] | 4.28% | 8.22% | 10.34% | 36.73% | 46.52% |
| [speedy 0.8.7][speedy] | 69.17% | 86.49% | 61.31% | 78.40% | 82.65% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.53%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.64%\**</span> | <span title="validated on-demand with panic">*56.15%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*49.13%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.30%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.0-alpha.2
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
[serde_json]: https://crates.io/crates/serde_json/1.0.114
[simd-json]: https://crates.io/crates/simd-json/0.13.8
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
