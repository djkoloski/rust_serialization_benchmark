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

## Last updated: 2023-7-14 13:55:59

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 258.05 µs | <span title="unvalidated">*2.3687 ms\**</span> | 1705800 | 530424 | 403325 |
| [alkahest 0.1.5][alkahest] | 247.06 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 608.56 µs | 3.1072 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.4.0][bitcode] | 517.64 µs | 3.2504 ms | 703664 | 317711 | 273622 |
| [borsh 0.10.3][borsh] | 502.51 µs | 3.3742 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 2.8406 ms | 9.9612 ms | 1924682 | 532821 | 376270 |
| [capnp 0.16.1][capnp] | 712.22 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 2.9245 ms | 12.872 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 806.41 µs | 3.8384 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.9312 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.6562 ms | 3.7079 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.5797 ms | 5.5613 ms | 818669 | 332556 | 285514 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 619.69 µs | 3.6012 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.4][postcard] | 410.07 µs | 3.3463 ms | 724953 | 302399 | 253747 |
| [prost 0.11.9][prost] | <span title="encode">*521.78 µs\**</span> <span title="populate + encode">*3.2574 ms\**</span> | 3.9966 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 308.55 µs | <span title="unvalidated">*2.4517 ms\**</span> <span title="validated upfront with error">*3.4505 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.7225 ms | 4.5442 ms | 784997 | 325384 | 278219 |
| [ron 0.8.0][ron] | 21.516 ms | 20.381 ms | 1607459 | 449158 | 349713 |
| [savefile 0.14.3][savefile] | 795.93 µs | 3.5071 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 760.37 µs | 3.5757 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.1473 ms | 6.6895 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.102][serde_json] | 4.3062 ms | 8.5478 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.2335 ms | 5.4053 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 244.52 µs | 2.6632 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*27.220 µs\**</span> | <span title="unvalidated">*48.706 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7058 ns\**</span> | <span title="validated on-demand with panic">*35.189 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*118.59 ns\**</span> | <span title="validated on-demand with error">*397.30 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6924 ns\**</span> <span title="validated upfront with error">*2.3218 ms\**</span> | <span title="unvalidated">*79.225 µs\**</span> <span title="validated upfront with error">*2.4032 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6072 ns\**</span> <span title="validated upfront with error">*872.52 µs\**</span> | <span title="unvalidated">*17.630 µs\**</span> <span title="validated upfront with error">*889.74 µs\**</span> | 19.760 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 94.76% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 50.55% | 56.52% |
| [alkahest 0.1.5][alkahest] | 98.97% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 40.18% | 76.23% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.4.0][bitcode] | 47.24% | 72.87% | 100.00% | 84.40% | 83.31% |
| [borsh 0.10.3][borsh] | 48.66% | 70.20% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.61% | 23.78% | 36.56% | 50.32% | 60.58% |
| [capnp 0.16.1][capnp] | 34.33% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 8.36% | 18.40% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 30.32% | 61.71% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 12.66% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 14.76% | 63.88% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.85% | 42.59% | 85.95% | 80.63% | 79.84% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 39.46% | 65.78% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.4][postcard] | 59.63% | 70.79% | 97.06% | 88.67% | 89.83% |
| [prost 0.11.9][prost] | <span title="encode">*46.86%\**</span> <span title="populate + encode">*7.51%\**</span> | 59.27% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 79.25% | <span title="unvalidated">*96.61%\**</span> <span title="validated upfront with error">*68.65%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 14.20% | 52.13% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.0][ron] | 1.14% | 11.62% | 43.77% | 59.70% | 65.18% |
| [savefile 0.14.3][savefile] | 30.72% | 67.54% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 32.16% | 66.24% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.39% | 35.41% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.102][serde_json] | 5.68% | 27.71% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.95% | 43.82% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 100.00% | 88.94% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*36.20%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*59.40%\**</span> | <span title="validated on-demand with panic">*50.10%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.36%\**</span> | <span title="validated on-demand with error">*4.44%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*59.69%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*22.25%\**</span> <span title="validated upfront with error">*0.73%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.98%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 427.40 µs | <span title="unvalidated">*427.44 µs\**</span> | 6000024 | 5378513 | 5345891 |
| [alkahest 0.1.5][alkahest] | 429.51 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 4.1742 ms | 6.1800 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.4.0][bitcode] | 4.9309 ms | 9.7456 ms | 4688054 | 4688491 | 4688168 |
| [borsh 0.10.3][borsh] | 5.2708 ms | 4.7092 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 50.375 ms | 108.09 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.16.1][capnp] | 8.5655 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 90.068 ms | 121.00 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 5.6993 ms | 10.187 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.2432 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 23.521 ms | 10.767 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 210.36 ms | 33.482 ms | 8125037 | 6493484 | 6386940 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 3.9210 ms | 6.0418 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.4][postcard] | 596.15 µs | 1.5152 ms | 6000003 | 5378495 | 5345900 |
| [prost 0.11.9][prost] | <span title="encode">*6.1577 ms\**</span> <span title="populate + encode">*7.9366 ms\**</span> | 15.982 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 532.06 µs | <span title="unvalidated">*429.15 µs\**</span> <span title="validated upfront with error">*429.05 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 18.453 ms | 24.412 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.0][ron] | 235.63 ms | 364.37 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.14.3][savefile] | 7.1101 ms | 7.9917 ms | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.4155 ms | 6.3726 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 50.763 ms | 51.999 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.102][serde_json] | 109.59 ms | 97.658 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.9.2][simd-json] | 71.482 ms | 97.578 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 427.26 µs | 421.58 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*1.9532 ns\**</span> | <span title="unvalidated">*246.40 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7056 ns\**</span> | <span title="validated on-demand with panic">*100.42 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*179.84 ns\**</span> | <span title="validated on-demand with error">*5.1454 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6912 ns\**</span> <span title="validated upfront with error">*48.832 ns\**</span> | <span title="unvalidated">*100.42 µs\**</span> <span title="validated upfront with error">*100.47 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2067 ns\**</span> <span title="validated upfront with error">*12.355 ns\**</span> | <span title="unvalidated">*35.175 µs\**</span> <span title="validated upfront with error">*35.843 µs\**</span> | 234.98 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 99.97% | <span title="unvalidated">*98.63%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 99.48% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 10.24% | 6.82% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.4.0][bitcode] | 8.66% | 4.33% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 8.11% | 8.95% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.85% | 0.39% | 20.37% | 50.89% | 62.53% |
| [capnp 0.16.1][capnp] | 4.99% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.47% | 0.35% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 7.50% | 4.14% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 34.37% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 1.82% | 3.92% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.20% | 1.26% | 57.70% | 72.20% | 73.40% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 10.90% | 6.98% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.4][postcard] | 71.67% | 27.82% | 78.13% | 87.17% | 87.70% |
| [prost 0.11.9][prost] | <span title="encode">*6.94%\**</span> <span title="populate + encode">*5.38%\**</span> | 2.64% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 80.30% | <span title="unvalidated">*98.24%\**</span> <span title="validated upfront with error">*98.26%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.32% | 1.73% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.0][ron] | 0.18% | 0.12% | 21.12% | 52.27% | 57.60% |
| [savefile 0.14.3][savefile] | 6.01% | 5.28% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 6.66% | 6.62% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.84% | 0.81% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.102][serde_json] | 0.39% | 0.43% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.60% | 0.43% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 100.00% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.78%\**</span> | <span title="unvalidated">*14.28%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*44.60%\**</span> | <span title="validated on-demand with panic">*35.03%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.67%\**</span> | <span title="validated on-demand with error">*0.68%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.84%\**</span> <span title="validated upfront with error">*2.47%\**</span> | <span title="unvalidated">*35.03%\**</span> <span title="validated upfront with error">*35.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.77%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*98.14%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 237.04 µs | <span title="unvalidated">*1.9074 ms\**</span> | 1290592 | 395729 | 333402 |
| [alkahest 0.1.5][alkahest] | 263.30 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 665.42 µs | 2.5097 ms | 569975 | 240525 | 232423 |
| [bitcode 0.4.0][bitcode] | 377.37 µs | 2.6271 ms | 322798 | 214279 | 201247 |
| [borsh 0.10.3][borsh] | 570.00 µs | 2.6076 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 3.8535 ms | 11.435 ms | 1619653 | 502185 | 328399 |
| [capnp 0.16.1][capnp] | 605.06 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.9406 ms | 11.652 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 888.06 µs | 3.6585 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.4106 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.3221 ms | 3.8896 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.0046 ms | 5.0554 ms | 449745 | 252432 | 231110 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 639.00 µs | 2.8042 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.4][postcard] | 483.88 µs | 2.7232 ms | 367489 | 221913 | 207344 |
| [prost 0.11.9][prost] | <span title="encode">*1.3616 ms\**</span> <span title="populate + encode">*3.9276 ms\**</span> | 4.8180 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 397.07 µs | <span title="unvalidated">*1.8194 ms\**</span> <span title="validated upfront with error">*2.6318 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.8587 ms | 3.9626 ms | 424533 | 245214 | 226188 |
| [ron 0.8.0][ron] | 11.050 ms | 21.029 ms | 1465223 | 434935 | 343338 |
| [savefile 0.14.3][savefile] | 787.88 µs | 2.8584 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 859.51 µs | 3.1525 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0997 ms | 6.4226 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.102][serde_json] | 4.7066 ms | 9.9420 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.6948 ms | 5.6102 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 365.63 µs | 2.3088 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*36.822 µs\**</span> | <span title="unvalidated">*37.912 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7038 ns\**</span> | <span title="validated on-demand with panic">*6.7585 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*118.59 ns\**</span> | <span title="validated on-demand with error">*741.06 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6903 ns\**</span> <span title="validated upfront with error">*2.6106 ms\**</span> | <span title="unvalidated">*1.9974 µs\**</span> <span title="validated upfront with error">*2.6063 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6090 ns\**</span> <span title="validated upfront with error">*764.87 µs\**</span> | <span title="unvalidated">*146.09 ns\**</span> <span title="validated upfront with error">*765.14 µs\**</span> | 1.6819 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.39%\**</span> | 25.01% | 53.82% | 59.53% |
| [alkahest 0.1.5][alkahest] | 90.03% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 35.62% | 72.49% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.4.0][bitcode] | 62.81% | 69.26% | 100.00% | 99.39% | 98.63% |
| [borsh 0.10.3][borsh] | 41.59% | 69.77% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.15% | 15.91% | 19.93% | 42.41% | 60.44% |
| [capnp 0.16.1][capnp] | 39.18% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.02% | 15.61% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 26.69% | 49.73% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 6.95% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 17.93% | 46.78% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.96% | 35.99% | 71.77% | 84.37% | 85.88% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 37.10% | 64.88% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.4][postcard] | 48.99% | 66.81% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="encode">*17.41%\**</span> <span title="populate + encode">*6.04%\**</span> | 37.76% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 59.70% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*69.13%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 12.75% | 45.91% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.0][ron] | 2.15% | 8.65% | 22.03% | 48.97% | 57.81% |
| [savefile 0.14.3][savefile] | 30.09% | 63.65% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 27.58% | 57.71% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.29% | 28.33% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.102][serde_json] | 5.04% | 18.30% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.9.2][simd-json] | 8.80% | 32.43% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 64.83% | 78.80% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.39%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*59.51%\**</span> | <span title="validated on-demand with panic">*2.16%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.36%\**</span> | <span title="validated on-demand with error">*19.71%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*59.81%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.31%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 650.26 µs | <span title="unvalidated">*3.1504 ms\**</span> | 2984682 | 1444877 | 1312105 |
| [alkahest 0.1.5][alkahest] | 842.69 µs | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 3.5687 ms | 6.2521 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.7344 ms | 4.4992 ms | 870693 | 866738 | 870720 |
| [borsh 0.10.3][borsh] | 2.6984 ms | 5.3229 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 31.947 ms | 59.308 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.16.1][capnp] | 2.5651 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 25.506 ms | 53.883 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.1558 ms | 11.346 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.5229 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.5425 ms | 7.4233 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 45.457 ms | 21.179 ms | 1728519 | 1247642 | 1233323 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 2.8261 ms | 4.4141 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.4][postcard] | 2.2119 ms | 5.3842 ms | 1279599 | 1058243 | 1016738 |
| [prost 0.11.9][prost] | <span title="encode">*5.3573 ms\**</span> <span title="populate + encode">*11.140 ms\**</span> | 11.562 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 2.1038 ms | <span title="unvalidated">*2.9059 ms\**</span> <span title="validated upfront with error">*3.8816 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.1][rmp-serde] | 12.688 ms | 14.597 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.0][ron] | 44.007 ms | 118.94 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.14.3][savefile] | 3.3463 ms | 5.6472 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.7399 ms | 6.7081 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 11.267 ms | 28.218 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.102][serde_json] | 25.563 ms | 42.544 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.9.2][simd-json] | 13.554 ms | 34.881 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 1.0631 ms | 3.2278 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*115.35 µs\**</span> | <span title="unvalidated">*115.04 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7246 ns\**</span> | <span title="validated on-demand with panic">*816.06 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*118.64 ns\**</span> | <span title="validated on-demand with error">*1.1517 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6928 ns\**</span> <span title="validated upfront with error">*5.1891 ms\**</span> | <span title="unvalidated">*3.7779 µs\**</span> <span title="validated upfront with error">*5.0994 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6067 ns\**</span> <span title="validated upfront with error">*876.28 µs\**</span> | <span title="unvalidated">*423.73 ns\**</span> <span title="validated upfront with error">*877.21 µs\**</span> | 813.16 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.24%\**</span> | 29.17% | 59.99% | 66.36% |
| [alkahest 0.1.5][alkahest] | 77.16% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 18.22% | 46.48% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.4.0][bitcode] | 37.49% | 64.59% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 24.10% | 54.59% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 2.04% | 4.90% | 8.68% | 30.59% | 54.39% |
| [capnp 0.16.1][capnp] | 25.35% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.55% | 5.39% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 10.56% | 25.61% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 11.77% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 14.32% | 39.15% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.43% | 13.72% | 50.37% | 69.47% | 70.60% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 23.01% | 65.83% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.4][postcard] | 29.40% | 53.97% | 68.04% | 81.90% | 85.64% |
| [prost 0.11.9][prost] | <span title="encode">*12.14%\**</span> <span title="populate + encode">*5.84%\**</span> | 25.13% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 30.91% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*74.86%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.1][rmp-serde] | 5.12% | 19.91% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.0][ron] | 1.48% | 2.44% | 10.27% | 39.74% | 48.81% |
| [savefile 0.14.3][savefile] | 19.43% | 51.46% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 13.72% | 43.32% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.77% | 10.30% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.102][serde_json] | 2.54% | 6.83% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.80% | 8.33% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 61.17% | 90.03% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.37%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.97%\**</span> | <span title="validated on-demand with panic">*51.92%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.35%\**</span> | <span title="validated on-demand with error">*36.79%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*59.67%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.22%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
[savefile]: https://crates.io/crates/savefile/0.14.3
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.102
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
