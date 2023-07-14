<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

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

## Last updated: 2023-7-14 20:32:42

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 261.53 µs | <span title="unvalidated">*2.3526 ms\**</span> | 1705800 | 530433 | 403630 |
| [alkahest 0.1.5][alkahest] | 246.06 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 630.70 µs | 3.2868 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.4.0][bitcode] | 507.26 µs | 3.2712 ms | 703664 | 317711 | 273622 |
| [borsh 0.10.3][borsh] | 490.04 µs | 3.4106 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 2.7962 ms | 9.7939 ms | 1924682 | 532821 | 376270 |
| [capnp 0.16.1][capnp] | 739.66 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 4.0753 ms | 12.730 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 822.47 µs | 3.7589 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.9297 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.7562 ms | 3.6975 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.7306 ms | 5.6064 ms | 818669 | 332556 | 285514 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 620.16 µs | 3.5382 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.4][postcard] | 399.44 µs | 3.3128 ms | 724953 | 302399 | 253747 |
| [prost 0.11.9][prost] | <span title="encode">*523.27 µs\**</span> <span title="populate + encode">*3.1808 ms\**</span> | 3.9310 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 314.95 µs | <span title="unvalidated">*2.5064 ms\**</span> <span title="validated upfront with error">*3.5274 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.7077 ms | 4.4265 ms | 784997 | 325384 | 278219 |
| [ron 0.8.0][ron] | 21.450 ms | 20.811 ms | 1607459 | 449158 | 349713 |
| [savefile 0.15.0][savefile] | 804.03 µs | 3.6032 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 757.37 µs | 3.4629 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.1130 ms | 6.3469 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.102][serde_json] | 4.6252 ms | 8.2402 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.3356 ms | 5.4329 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 253.26 µs | 2.7453 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*29.022 µs\**</span> | <span title="unvalidated">*51.500 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.8644 ns\**</span> | <span title="validated on-demand with panic">*36.225 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*116.80 ns\**</span> | <span title="validated on-demand with error">*390.39 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6803 ns\**</span> <span title="validated upfront with error">*2.2816 ms\**</span> | <span title="unvalidated">*79.887 µs\**</span> <span title="validated upfront with error">*2.3527 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2060 ns\**</span> <span title="validated upfront with error">*887.06 µs\**</span> | <span title="unvalidated">*17.650 µs\**</span> <span title="validated upfront with error">*907.45 µs\**</span> | 19.903 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 94.08% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 50.55% | 56.47% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 39.01% | 71.58% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.4.0][bitcode] | 48.51% | 71.92% | 100.00% | 84.40% | 83.31% |
| [borsh 0.10.3][borsh] | 50.21% | 68.98% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.80% | 24.02% | 36.56% | 50.32% | 60.58% |
| [capnp 0.16.1][capnp] | 33.27% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 6.04% | 18.48% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 29.92% | 62.59% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 12.75% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 14.01% | 63.63% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.82% | 41.96% | 85.95% | 80.63% | 79.84% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 39.68% | 66.49% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.4][postcard] | 61.60% | 71.02% | 97.06% | 88.67% | 89.83% |
| [prost 0.11.9][prost] | <span title="encode">*47.02%\**</span> <span title="populate + encode">*7.74%\**</span> | 59.85% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 78.13% | <span title="unvalidated">*93.86%\**</span> <span title="validated upfront with error">*66.70%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 14.41% | 53.15% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.0][ron] | 1.15% | 11.30% | 43.77% | 59.70% | 65.18% |
| [savefile 0.15.0][savefile] | 30.60% | 65.29% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 32.49% | 67.94% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.65% | 37.07% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.102][serde_json] | 5.32% | 28.55% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.54% | 43.30% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 97.16% | 85.70% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*34.27%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*42.10%\**</span> | <span title="validated on-demand with panic">*48.72%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.03%\**</span> | <span title="validated on-demand with error">*4.52%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*22.09%\**</span> <span title="validated upfront with error">*0.75%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.95%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 445.36 µs | <span title="unvalidated">*446.57 µs\**</span> | 6000024 | 5378513 | 5345891 |
| [alkahest 0.1.5][alkahest] | 445.90 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 4.1640 ms | 6.9686 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.4.0][bitcode] | 4.8936 ms | 9.7754 ms | 4688054 | 4688491 | 4688168 |
| [borsh 0.10.3][borsh] | 5.3201 ms | 5.0561 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 51.645 ms | 108.79 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.16.1][capnp] | 8.7939 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 89.032 ms | 123.86 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 5.6163 ms | 10.198 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.2427 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 23.655 ms | 10.858 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 213.44 ms | 36.068 ms | 8125037 | 6493484 | 6386940 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 3.8243 ms | 5.9187 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.4][postcard] | 583.18 µs | 1.7188 ms | 6000003 | 5378495 | 5345900 |
| [prost 0.11.9][prost] | <span title="encode">*10.674 ms\**</span> <span title="populate + encode">*13.017 ms\**</span> | 16.259 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 543.46 µs | <span title="unvalidated">*449.99 µs\**</span> <span title="validated upfront with error">*458.29 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 18.538 ms | 25.105 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.0][ron] | 230.05 ms | 362.79 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.15.0][savefile] | 456.06 µs | 455.14 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.3299 ms | 6.2856 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 50.080 ms | 51.660 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.102][serde_json] | 109.07 ms | 92.150 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.9.2][simd-json] | 71.109 ms | 109.15 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 455.38 µs | 454.68 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.0069 ns\**</span> | <span title="unvalidated">*250.03 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.8814 ns\**</span> | <span title="validated on-demand with panic">*100.42 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*191.36 ns\**</span> | <span title="validated on-demand with error">*5.2454 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6816 ns\**</span> <span title="validated upfront with error">*48.360 ns\**</span> | <span title="unvalidated">*100.42 µs\**</span> <span title="validated upfront with error">*100.48 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2061 ns\**</span> <span title="validated upfront with error">*12.864 ns\**</span> | <span title="unvalidated">*37.149 µs\**</span> <span title="validated upfront with error">*37.766 µs\**</span> | 244.51 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*100.00%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 99.88% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 10.70% | 6.41% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.4.0][bitcode] | 9.10% | 4.57% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 8.37% | 8.83% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.86% | 0.41% | 20.37% | 50.89% | 62.53% |
| [capnp 0.16.1][capnp] | 5.06% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.50% | 0.36% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 7.93% | 4.38% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 35.84% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 1.88% | 4.11% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.21% | 1.24% | 57.70% | 72.20% | 73.40% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 11.65% | 7.55% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.4][postcard] | 76.37% | 25.98% | 78.13% | 87.17% | 87.70% |
| [prost 0.11.9][prost] | <span title="encode">*4.17%\**</span> <span title="populate + encode">*3.42%\**</span> | 2.75% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 81.95% | <span title="unvalidated">*99.24%\**</span> <span title="validated upfront with error">*97.44%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.40% | 1.78% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.0][ron] | 0.19% | 0.12% | 21.12% | 52.27% | 57.60% |
| [savefile 0.15.0][savefile] | 97.65% | 98.12% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 7.04% | 7.10% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.89% | 0.86% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.102][serde_json] | 0.41% | 0.48% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.63% | 0.41% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 97.80% | 98.22% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*60.10%\**</span> | <span title="unvalidated">*14.86%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*41.86%\**</span> | <span title="validated on-demand with panic">*36.99%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.63%\**</span> | <span title="validated on-demand with error">*0.71%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.98%\**</span> <span title="validated upfront with error">*2.49%\**</span> | <span title="unvalidated">*36.99%\**</span> <span title="validated upfront with error">*36.97%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.38%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*98.37%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 232.60 µs | <span title="unvalidated">*1.8922 ms\**</span> | 1290592 | 395795 | 333235 |
| [alkahest 0.1.5][alkahest] | 263.04 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 668.32 µs | 2.5146 ms | 569975 | 240525 | 232423 |
| [bitcode 0.4.0][bitcode] | 360.94 µs | 2.6083 ms | 322798 | 214279 | 201247 |
| [borsh 0.10.3][borsh] | 579.11 µs | 2.6288 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 3.8429 ms | 11.441 ms | 1619653 | 502185 | 328399 |
| [capnp 0.16.1][capnp] | 606.86 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.8706 ms | 11.741 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 892.61 µs | 3.6399 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.4182 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.4101 ms | 3.9078 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.0919 ms | 5.1021 ms | 449745 | 252432 | 231110 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 645.43 µs | 2.7310 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.4][postcard] | 491.96 µs | 2.6481 ms | 367489 | 221913 | 207344 |
| [prost 0.11.9][prost] | <span title="encode">*1.3919 ms\**</span> <span title="populate + encode">*3.9485 ms\**</span> | 4.7001 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 402.61 µs | <span title="unvalidated">*1.8106 ms\**</span> <span title="validated upfront with error">*2.6081 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.8930 ms | 3.9251 ms | 424533 | 245214 | 226188 |
| [ron 0.8.0][ron] | 11.162 ms | 21.324 ms | 1465223 | 434935 | 343338 |
| [savefile 0.15.0][savefile] | 798.51 µs | 2.9356 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 864.73 µs | 3.1545 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.1133 ms | 6.4262 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.102][serde_json] | 4.7255 ms | 9.9294 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.7141 ms | 5.6062 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 366.84 µs | 2.2907 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*42.002 µs\**</span> | <span title="unvalidated">*43.017 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.8883 ns\**</span> | <span title="validated on-demand with panic">*6.7465 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*117.01 ns\**</span> | <span title="validated on-demand with error">*657.21 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6804 ns\**</span> <span title="validated upfront with error">*2.5763 ms\**</span> | <span title="unvalidated">*1.9925 µs\**</span> <span title="validated upfront with error">*2.5853 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2061 ns\**</span> <span title="validated upfront with error">*747.39 µs\**</span> | <span title="unvalidated">*164.10 ns\**</span> <span title="validated upfront with error">*748.72 µs\**</span> | 1.6823 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.69%\**</span> | 25.01% | 53.81% | 59.56% |
| [alkahest 0.1.5][alkahest] | 88.43% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 34.80% | 72.00% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.4.0][bitcode] | 64.44% | 69.42% | 100.00% | 99.39% | 98.63% |
| [borsh 0.10.3][borsh] | 40.17% | 68.88% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.05% | 15.83% | 19.93% | 42.41% | 60.44% |
| [capnp 0.16.1][capnp] | 38.33% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.01% | 15.42% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 26.06% | 49.74% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 6.80% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 16.50% | 46.33% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.87% | 35.49% | 71.77% | 84.37% | 85.88% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 36.04% | 66.30% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.4][postcard] | 47.28% | 68.37% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="encode">*16.71%\**</span> <span title="populate + encode">*5.89%\**</span> | 38.52% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 57.77% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*69.42%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 12.29% | 46.13% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.0][ron] | 2.08% | 8.49% | 22.03% | 48.97% | 57.81% |
| [savefile 0.15.0][savefile] | 29.13% | 61.68% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 26.90% | 57.40% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.01% | 28.18% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.102][serde_json] | 4.92% | 18.23% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.9.2][simd-json] | 8.57% | 32.30% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 63.41% | 79.04% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.38%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*41.76%\**</span> | <span title="validated on-demand with panic">*2.43%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.03%\**</span> | <span title="validated on-demand with error">*24.97%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*45.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*8.24%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 655.36 µs | <span title="unvalidated">*3.1649 ms\**</span> | 2984682 | 1447606 | 1313826 |
| [alkahest 0.1.5][alkahest] | 833.32 µs | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 3.6229 ms | 5.9483 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.7416 ms | 4.5171 ms | 870693 | 866738 | 870720 |
| [borsh 0.10.3][borsh] | 2.7060 ms | 5.3363 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 31.845 ms | 59.205 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.16.1][capnp] | 2.6181 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 20.362 ms | 54.293 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.3177 ms | 10.955 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.6318 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.7360 ms | 7.4827 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 46.987 ms | 20.914 ms | 1728519 | 1247642 | 1233323 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 2.8801 ms | 4.4249 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.4][postcard] | 2.2137 ms | 5.4198 ms | 1279599 | 1058243 | 1016738 |
| [prost 0.11.9][prost] | <span title="encode">*5.2234 ms\**</span> <span title="populate + encode">*11.195 ms\**</span> | 11.477 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 2.1435 ms | <span title="unvalidated">*2.9453 ms\**</span> <span title="validated upfront with error">*3.9066 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.1][rmp-serde] | 12.789 ms | 14.240 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.0][ron] | 44.878 ms | 122.72 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.15.0][savefile] | 3.3608 ms | 5.7348 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.7508 ms | 6.6466 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 11.366 ms | 26.493 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.102][serde_json] | 25.613 ms | 42.143 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.9.2][simd-json] | 13.351 ms | 36.773 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 1.1020 ms | 3.2685 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*122.13 µs\**</span> | <span title="unvalidated">*122.33 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.9152 ns\**</span> | <span title="validated on-demand with panic">*816.51 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*118.59 ns\**</span> | <span title="validated on-demand with error">*1.1497 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6821 ns\**</span> <span title="validated upfront with error">*5.1529 ms\**</span> | <span title="unvalidated">*3.7797 µs\**</span> <span title="validated upfront with error">*5.2314 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2058 ns\**</span> <span title="validated upfront with error">*873.20 µs\**</span> | <span title="unvalidated">*421.76 ns\**</span> <span title="validated upfront with error">*873.30 µs\**</span> | 813.78 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.06%\**</span> | 29.17% | 59.87% | 66.27% |
| [alkahest 0.1.5][alkahest] | 78.64% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 18.09% | 49.51% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.4.0][bitcode] | 37.63% | 65.20% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 24.22% | 55.19% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 2.06% | 4.97% | 8.68% | 30.59% | 54.39% |
| [capnp 0.16.1][capnp] | 25.03% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 3.22% | 5.42% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 10.37% | 26.89% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 11.64% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 13.84% | 39.36% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.39% | 14.08% | 50.37% | 69.47% | 70.60% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 22.75% | 66.56% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.4][postcard] | 29.60% | 54.34% | 68.04% | 81.90% | 85.64% |
| [prost 0.11.9][prost] | <span title="encode">*12.55%\**</span> <span title="populate + encode">*5.85%\**</span> | 25.66% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 30.57% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*75.39%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.1][rmp-serde] | 5.12% | 20.68% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.0][ron] | 1.46% | 2.40% | 10.27% | 39.74% | 48.81% |
| [savefile 0.15.0][savefile] | 19.50% | 51.36% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 13.79% | 44.31% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.77% | 11.12% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.102][serde_json] | 2.56% | 6.99% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.91% | 8.01% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 59.47% | 90.11% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.34%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*41.36%\**</span> | <span title="validated on-demand with panic">*51.65%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.02%\**</span> | <span title="validated on-demand with error">*36.68%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.16%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.3
[postcard]: https://crates.io/crates/postcard/1.0.4
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.1
[ron]: https://crates.io/crates/ron/0.8.0
[savefile]: https://crates.io/crates/savefile/0.15.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.102
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
