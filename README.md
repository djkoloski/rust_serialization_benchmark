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

## Last updated: 2023-8-30 22:37:21

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 257.09 µs | <span title="unvalidated">*2.4212 ms\**</span> | 1705800 | 530424 | 403625 |
| [alkahest 0.1.5][alkahest] | 228.14 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 590.37 µs | 3.3056 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.4.0][bitcode] | 486.80 µs | 3.4599 ms | 703664 | 317711 | 273622 |
| [borsh 0.10.3][borsh] | 477.42 µs | 3.3958 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 2.7285 ms | 10.291 ms | 1924682 | 532821 | 376270 |
| [capnp 0.16.1][capnp] | 741.66 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 3.7592 ms | 11.202 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 787.48 µs | 4.0392 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.9642 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.7484 ms | 3.8619 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.9420 ms | 5.6248 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.33][nanoserde] | 321.83 µs | 3.3271 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 629.62 µs | 3.6261 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.6][postcard] | 410.46 µs | 3.5036 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 2.9945 ms | 8.2727 ms | 971922 | 372513 | 304122 |
| [prost 0.11.9][prost] | <span title="encode">*609.09 µs\**</span> <span title="populate + encode">*3.2274 ms\**</span> | 4.1908 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 316.64 µs | <span title="unvalidated">*2.4899 ms\**</span> <span title="validated upfront with error">*3.4536 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.3813 ms | 4.6170 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 20.703 ms | 23.852 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.2][savefile] | 243.23 µs | 3.2713 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 751.52 µs | 3.2947 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0589 ms | 6.9100 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.105][serde_json] | 4.3145 ms | 8.6280 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.2252 ms | 5.9450 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 273.33 µs | 2.8605 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*38.540 µs\**</span> | <span title="unvalidated">*62.898 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6124 ns\**</span> | <span title="validated on-demand with panic">*35.240 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*106.94 ns\**</span> | <span title="validated on-demand with error">*383.19 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2713 ns\**</span> <span title="validated upfront with error">*2.1946 ms\**</span> | <span title="unvalidated">*110.32 µs\**</span> <span title="validated upfront with error">*2.2921 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5232 ns\**</span> <span title="validated upfront with error">*946.75 µs\**</span> | <span title="unvalidated">*16.221 µs\**</span> <span title="validated upfront with error">*962.77 µs\**</span> | 23.452 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 88.74% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 50.55% | 56.47% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 38.64% | 73.25% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.4.0][bitcode] | 46.87% | 69.98% | 100.00% | 84.40% | 83.31% |
| [borsh 0.10.3][borsh] | 47.79% | 71.30% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.36% | 23.53% | 36.56% | 50.32% | 60.58% |
| [capnp 0.16.1][capnp] | 30.76% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 6.07% | 21.61% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 28.97% | 59.94% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 11.61% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 13.05% | 62.69% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.87% | 43.05% | 85.95% | 80.63% | 79.84% |
| [nanoserde 0.1.33][nanoserde] | 70.89% | 72.77% | 67.29% | 71.86% | 73.12% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 36.23% | 66.77% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.6][postcard] | 55.58% | 69.11% | 97.06% | 88.67% | 89.83% |
| [pot 3.0.0][pot] | 7.62% | 29.27% | 72.40% | 71.98% | 74.95% |
| [prost 0.11.9][prost] | <span title="encode">*37.46%\**</span> <span title="populate + encode">*7.07%\**</span> | 57.77% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 72.05% | <span title="unvalidated">*97.24%\**</span> <span title="validated upfront with error">*70.11%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.2][rmp-serde] | 16.52% | 52.44% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.1][ron] | 1.10% | 10.15% | 43.77% | 59.70% | 65.18% |
| [savefile 0.16.2][savefile] | 93.80% | 74.01% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 30.36% | 73.49% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.08% | 35.04% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.105][serde_json] | 5.29% | 28.06% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.25% | 40.73% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 83.47% | 84.64% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*25.79%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.31%\**</span> | <span title="validated on-demand with panic">*46.03%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.42%\**</span> | <span title="validated on-demand with error">*4.23%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.56%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*14.70%\**</span> <span title="validated upfront with error">*0.71%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.68%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 466.48 µs | <span title="unvalidated">*465.27 µs\**</span> | 6000024 | 5378513 | 5345891 |
| [alkahest 0.1.5][alkahest] | 466.15 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 4.7113 ms | 5.2551 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.4.0][bitcode] | 5.0578 ms | 8.5304 ms | 4688054 | 4688491 | 4688168 |
| [borsh 0.10.3][borsh] | 5.7734 ms | 7.3894 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 51.236 ms | 109.20 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.16.1][capnp] | 10.554 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 78.589 ms | 97.990 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 7.5400 ms | 10.280 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 961.78 µs | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 19.608 ms | 8.8097 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 193.60 ms | 36.839 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.33][nanoserde] | 1.8894 ms | 1.8446 ms | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 3.7563 ms | 4.7951 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.6][postcard] | 843.38 µs | 1.8062 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 50.849 ms | 93.231 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.11.9][prost] | <span title="encode">*11.094 ms\**</span> <span title="populate + encode">*13.337 ms\**</span> | 17.334 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 502.89 µs | <span title="unvalidated">*483.31 µs\**</span> <span title="validated upfront with error">*475.75 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 17.038 ms | 20.559 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 222.70 ms | 390.60 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.2][savefile] | 473.32 µs | 473.78 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.2842 ms | 5.5974 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 46.721 ms | 57.100 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.105][serde_json] | 106.48 ms | 91.679 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.9.2][simd-json] | 64.187 ms | 117.42 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 465.13 µs | 464.98 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.4775 ns\**</span> | <span title="unvalidated">*255.83 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6113 ns\**</span> | <span title="validated on-demand with panic">*83.678 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*178.73 ns\**</span> | <span title="validated on-demand with error">*5.5293 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2854 ns\**</span> <span title="validated upfront with error">*44.171 ns\**</span> | <span title="unvalidated">*83.718 µs\**</span> <span title="validated upfront with error">*83.808 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5236 ns\**</span> <span title="validated upfront with error">*13.166 ns\**</span> | <span title="unvalidated">*47.388 µs\**</span> <span title="validated upfront with error">*46.919 µs\**</span> | 244.29 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 99.71% | <span title="unvalidated">*99.94%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 99.78% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 9.87% | 8.85% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.4.0][bitcode] | 9.20% | 5.45% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 8.06% | 6.29% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.91% | 0.43% | 20.37% | 50.89% | 62.53% |
| [capnp 0.16.1][capnp] | 4.41% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.59% | 0.47% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 6.17% | 4.52% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 48.36% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 2.37% | 5.28% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.24% | 1.26% | 57.70% | 72.20% | 73.40% |
| [nanoserde 0.1.33][nanoserde] | 24.62% | 25.21% | 78.13% | 87.17% | 87.70% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 12.38% | 9.70% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.6][postcard] | 55.15% | 25.74% | 78.13% | 87.17% | 87.70% |
| [pot 3.0.0][pot] | 0.91% | 0.50% | 46.31% | 68.80% | 68.42% |
| [prost 0.11.9][prost] | <span title="encode">*4.19%\**</span> <span title="populate + encode">*3.49%\**</span> | 2.68% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 92.49% | <span title="unvalidated">*96.21%\**</span> <span title="validated upfront with error">*97.74%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.2][rmp-serde] | 2.73% | 2.26% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.1][ron] | 0.21% | 0.12% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.2][savefile] | 98.27% | 98.14% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 7.40% | 8.31% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 1.00% | 0.81% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.105][serde_json] | 0.44% | 0.51% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.72% | 0.40% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 100.00% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.50%\**</span> | <span title="unvalidated">*18.34%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.35%\**</span> | <span title="validated on-demand with panic">*56.07%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.85%\**</span> | <span title="validated on-demand with error">*0.85%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.37%\**</span> <span title="validated upfront with error">*3.45%\**</span> | <span title="unvalidated">*56.04%\**</span> <span title="validated upfront with error">*55.98%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*11.57%\**</span> | <span title="unvalidated">*99.01%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 241.16 µs | <span title="unvalidated">*2.0085 ms\**</span> | 1290592 | 395720 | 333379 |
| [alkahest 0.1.5][alkahest] | 293.57 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 658.10 µs | 2.5689 ms | 569975 | 240525 | 232423 |
| [bitcode 0.4.0][bitcode] | 394.11 µs | 2.7325 ms | 322798 | 214279 | 201247 |
| [borsh 0.10.3][borsh] | 578.10 µs | 2.7273 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 3.4902 ms | 11.772 ms | 1619653 | 502185 | 328399 |
| [capnp 0.16.1][capnp] | 601.11 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.6639 ms | 10.326 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 856.86 µs | 3.7333 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.4524 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.3020 ms | 3.9600 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.5176 ms | 5.0821 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.33][nanoserde] | 340.03 µs | 3.5041 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 657.21 µs | 2.8352 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.6][postcard] | 493.67 µs | 2.8110 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 3.0614 ms | 7.6498 ms | 599125 | 299158 | 247693 |
| [prost 0.11.9][prost] | <span title="encode">*1.2847 ms\**</span> <span title="populate + encode">*3.8154 ms\**</span> | 4.8545 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 412.74 µs | <span title="unvalidated">*1.9007 ms\**</span> <span title="validated upfront with error">*2.6759 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.5446 ms | 4.0580 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 11.004 ms | 23.965 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.2][savefile] | 300.43 µs | 2.6325 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 851.80 µs | 3.2443 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0786 ms | 6.7715 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.105][serde_json] | 4.4819 ms | 9.8748 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.5003 ms | 5.7777 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 374.14 µs | 2.4393 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*60.143 µs\**</span> | <span title="unvalidated">*61.916 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6117 ns\**</span> | <span title="validated on-demand with panic">*7.1903 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*107.01 ns\**</span> | <span title="validated on-demand with error">*978.86 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2715 ns\**</span> <span title="validated upfront with error">*2.2870 ms\**</span> | <span title="unvalidated">*2.5691 µs\**</span> <span title="validated upfront with error">*2.3084 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5232 ns\**</span> <span title="validated upfront with error">*747.51 µs\**</span> | <span title="unvalidated">*189.15 ns\**</span> <span title="validated upfront with error">*747.99 µs\**</span> | 1.5795 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*94.63%\**</span> | 25.01% | 53.82% | 59.54% |
| [alkahest 0.1.5][alkahest] | 82.15% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 36.64% | 73.99% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.4.0][bitcode] | 61.19% | 69.56% | 100.00% | 99.39% | 98.63% |
| [borsh 0.10.3][borsh] | 41.72% | 69.69% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.91% | 16.15% | 19.93% | 42.41% | 60.44% |
| [capnp 0.16.1][capnp] | 40.12% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.58% | 18.41% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 28.14% | 50.91% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 6.99% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 18.52% | 48.00% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.21% | 37.40% | 71.77% | 84.37% | 85.88% |
| [nanoserde 0.1.33][nanoserde] | 70.92% | 54.24% | 56.83% | 88.77% | 85.40% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 36.69% | 67.04% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.6][postcard] | 48.85% | 67.62% | 87.84% | 95.97% | 95.73% |
| [pot 3.0.0][pot] | 7.88% | 24.85% | 53.88% | 71.19% | 80.13% |
| [prost 0.11.9][prost] | <span title="encode">*18.77%\**</span> <span title="populate + encode">*6.32%\**</span> | 39.15% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 58.43% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.03%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.2][rmp-serde] | 15.61% | 46.84% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.1][ron] | 2.19% | 7.93% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.2][savefile] | 80.27% | 72.20% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 28.31% | 58.59% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.60% | 28.07% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.105][serde_json] | 5.38% | 19.25% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.9.2][simd-json] | 9.65% | 32.90% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 64.46% | 77.92% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.31%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.32%\**</span> | <span title="validated on-demand with panic">*2.63%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.42%\**</span> | <span title="validated on-demand with error">*19.32%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.56%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.36%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 635.54 µs | <span title="unvalidated">*3.3604 ms\**</span> | 2984682 | 1449713 | 1311463 |
| [alkahest 0.1.5][alkahest] | 1.0057 ms | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 3.5797 ms | 5.8350 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.6263 ms | 4.8113 ms | 870693 | 866738 | 870720 |
| [borsh 0.10.3][borsh] | 2.7999 ms | 5.5231 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 27.739 ms | 61.619 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.16.1][capnp] | 2.5253 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 23.232 ms | 45.855 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.1088 ms | 11.269 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 6.0051 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.1496 ms | 8.2614 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 43.139 ms | 20.141 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.33][nanoserde] | 1.3345 ms | 6.9380 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 2.8904 ms | 4.3004 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.6][postcard] | 2.0645 ms | 5.3431 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 17.301 ms | 38.298 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.11.9][prost] | <span title="encode">*5.2510 ms\**</span> <span title="populate + encode">*11.514 ms\**</span> | 11.755 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.5475 ms | <span title="unvalidated">*3.0791 ms\**</span> <span title="validated upfront with error">*3.9889 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 9.8566 ms | 13.923 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 46.597 ms | 131.20 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.2][savefile] | 1.0951 ms | 3.8861 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.8627 ms | 6.5728 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 11.242 ms | 27.644 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.105][serde_json] | 25.893 ms | 42.176 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.9.2][simd-json] | 13.130 ms | 40.488 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 1.0282 ms | 3.5836 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*120.22 µs\**</span> | <span title="unvalidated">*121.77 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6115 ns\**</span> | <span title="validated on-demand with panic">*790.36 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*107.03 ns\**</span> | <span title="validated on-demand with error">*1.8201 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2706 ns\**</span> <span title="validated upfront with error">*4.9009 ms\**</span> | <span title="unvalidated">*5.0958 µs\**</span> <span title="validated upfront with error">*6.6065 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5233 ns\**</span> <span title="validated upfront with error">*861.27 µs\**</span> | <span title="unvalidated">*385.85 ns\**</span> <span title="validated upfront with error">*859.68 µs\**</span> | 769.13 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*91.63%\**</span> | 29.17% | 59.79% | 66.39% |
| [alkahest 0.1.5][alkahest] | 63.19% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 17.75% | 52.77% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.4.0][bitcode] | 39.08% | 64.00% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 22.70% | 55.75% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 2.29% | 5.00% | 8.68% | 30.59% | 54.39% |
| [capnp 0.16.1][capnp] | 25.17% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.74% | 6.71% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 10.40% | 27.32% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 10.58% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 15.32% | 37.27% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.47% | 15.29% | 50.37% | 69.47% | 70.60% |
| [nanoserde 0.1.33][nanoserde] | 47.62% | 44.38% | 49.18% | 78.20% | 84.54% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 21.99% | 71.60% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.6][postcard] | 30.78% | 57.63% | 68.04% | 81.90% | 85.64% |
| [pot 3.0.0][pot] | 3.67% | 8.04% | 34.21% | 59.88% | 68.65% |
| [prost 0.11.9][prost] | <span title="encode">*12.10%\**</span> <span title="populate + encode">*5.52%\**</span> | 26.19% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 41.07% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.19%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.2][rmp-serde] | 6.45% | 22.12% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.1][ron] | 1.36% | 2.35% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.2][savefile] | 58.03% | 79.23% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 13.07% | 46.85% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.65% | 11.14% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.105][serde_json] | 2.45% | 7.30% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.84% | 7.60% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 61.81% | 85.92% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.32%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.33%\**</span> | <span title="validated on-demand with panic">*48.82%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.42%\**</span> | <span title="validated on-demand with error">*21.20%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.58%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.57%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.04%\**</span> | 100.00% |

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
[nanoserde]: https://crates.io/crates/nanoserde/0.1.33
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.5
[postcard]: https://crates.io/crates/postcard/1.0.6
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
