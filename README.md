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

## Last updated: 2023-7-13 15:22:53

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 285.69 µs | <span title="unvalidated">*3.0135 ms\**</span> | 1705800 | 522800 | 406100 |
| [alkahest 0.1.5][alkahest] | 304.11 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 714.18 µs | 4.2025 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.4.0][bitcode] | 610.70 µs | 4.2086 ms | 703664 | 317711 | 273622 |
| [borsh 0.10.3][borsh] | 584.73 µs | 4.1763 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 3.5849 ms | 13.014 ms | 1924682 | 532821 | 376270 |
| [capnp 0.16.1][capnp] | 917.42 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 5.2135 ms | 14.208 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 988.67 µs | 4.9307 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 2.5917 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.6310 ms | 4.7387 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 9.0712 ms | 7.2579 ms | 818669 | 332556 | 285514 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 747.70 µs | 4.5505 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.4][postcard] | 516.50 µs | 4.2778 ms | 724953 | 302399 | 253747 |
| [prost 0.11.9][prost] | <span title="encode">*651.81 µs\**</span> <span title="populate + encode">*4.0399 ms\**</span> | 5.1908 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 403.35 µs | <span title="unvalidated">*3.2183 ms\**</span> <span title="validated upfront with error">*4.1787 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.9832 ms | 5.8687 ms | 784997 | 325384 | 278219 |
| [ron 0.8.0][ron] | 25.571 ms | 25.878 ms | 1607459 | 449158 | 349713 |
| [savefile 0.14.1][savefile] | 898.89 µs | 4.5741 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 941.25 µs | 4.3578 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.6192 ms | 8.3759 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.102][serde_json] | 6.0509 ms | 12.306 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.8505 ms | 7.2600 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 288.72 µs | 3.4337 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*72.567 µs\**</span> | <span title="unvalidated">*110.31 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.1262 ns\**</span> | <span title="validated on-demand with panic">*45.673 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*132.02 ns\**</span> | <span title="validated on-demand with error">*494.31 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7403 ns\**</span> <span title="validated upfront with error">*2.6482 ms\**</span> | <span title="unvalidated">*126.94 µs\**</span> <span title="validated upfront with error">*3.6766 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6628 ns\**</span> <span title="validated upfront with error">*988.21 µs\**</span> | <span title="unvalidated">*23.190 µs\**</span> <span title="validated upfront with error">*1.0123 ms\**</span> | 37.991 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 51.29% | 56.13% |
| [alkahest 0.1.5][alkahest] | 93.94% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 40.00% | 71.71% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.4.0][bitcode] | 46.78% | 71.60% | 100.00% | 84.40% | 83.31% |
| [borsh 0.10.3][borsh] | 48.86% | 72.16% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 7.97% | 23.16% | 36.56% | 50.32% | 60.58% |
| [capnp 0.16.1][capnp] | 31.14% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 5.48% | 21.21% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 28.90% | 61.12% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 11.02% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 17.52% | 63.59% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.15% | 41.52% | 85.95% | 80.63% | 79.84% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 38.21% | 66.22% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.4][postcard] | 55.31% | 70.45% | 97.06% | 88.67% | 89.83% |
| [prost 0.11.9][prost] | <span title="encode">*43.83%\**</span> <span title="populate + encode">*7.07%\**</span> | 58.05% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 70.83% | <span title="unvalidated">*93.64%\**</span> <span title="validated upfront with error">*72.12%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 14.41% | 51.35% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.0][ron] | 1.12% | 11.65% | 43.77% | 59.70% | 65.18% |
| [savefile 0.14.1][savefile] | 31.78% | 65.88% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 30.35% | 69.15% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 10.91% | 35.98% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.102][serde_json] | 4.72% | 24.49% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.02% | 41.51% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 98.95% | 87.76% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*21.02%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*53.19%\**</span> | <span title="validated on-demand with panic">*50.77%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.26%\**</span> | <span title="validated on-demand with error">*4.69%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.46%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.27%\**</span> <span title="validated upfront with error">*0.63%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.29%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 504.46 µs | <span title="unvalidated">*482.80 µs\**</span> | 6000024 | 5378513 | 5345891 |
| [alkahest 0.1.5][alkahest] | 570.12 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 5.3267 ms | 6.7559 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.4.0][bitcode] | 6.2892 ms | 10.816 ms | 4688054 | 4688491 | 4688168 |
| [borsh 0.10.3][borsh] | 7.0544 ms | 5.4579 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 63.118 ms | 141.44 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.16.1][capnp] | 14.150 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 108.81 ms | 139.72 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 7.3266 ms | 13.309 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.9003 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 26.719 ms | 13.324 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 194.53 ms | 44.674 ms | 8125037 | 6493484 | 6386940 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 5.0678 ms | 6.3481 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.4][postcard] | 867.34 µs | 2.1834 ms | 6000003 | 5378495 | 5345900 |
| [prost 0.11.9][prost] | <span title="encode">*8.9218 ms\**</span> <span title="populate + encode">*13.281 ms\**</span> | 21.779 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 937.22 µs | <span title="unvalidated">*565.83 µs\**</span> <span title="validated upfront with error">*571.73 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 21.657 ms | 29.666 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.0][ron] | 288.89 ms | 464.10 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.14.1][savefile] | 8.2947 ms | 10.044 ms | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 8.7789 ms | 7.0056 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 53.372 ms | 70.590 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.102][serde_json] | 136.94 ms | 126.85 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.9.2][simd-json] | 80.419 ms | 148.29 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 482.93 µs | 497.31 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*3.2092 ns\**</span> | <span title="unvalidated">*261.93 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.0994 ns\**</span> | <span title="validated on-demand with panic">*100.80 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*223.24 ns\**</span> | <span title="validated on-demand with error">*7.0753 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7789 ns\**</span> <span title="validated upfront with error">*55.714 ns\**</span> | <span title="unvalidated">*103.42 µs\**</span> <span title="validated upfront with error">*99.887 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7017 ns\**</span> <span title="validated upfront with error">*17.115 ns\**</span> | <span title="unvalidated">*51.222 µs\**</span> <span title="validated upfront with error">*51.391 µs\**</span> | 368.08 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 95.73% | <span title="unvalidated">*100.00%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 84.71% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 9.07% | 7.15% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.4.0][bitcode] | 7.68% | 4.46% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 6.85% | 8.85% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.77% | 0.34% | 20.37% | 50.89% | 62.53% |
| [capnp 0.16.1][capnp] | 3.41% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.44% | 0.35% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 6.59% | 3.63% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 25.41% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 1.81% | 3.62% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.25% | 1.08% | 57.70% | 72.20% | 73.40% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 9.53% | 7.61% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.4][postcard] | 55.68% | 22.11% | 78.13% | 87.17% | 87.70% |
| [prost 0.11.9][prost] | <span title="encode">*5.41%\**</span> <span title="populate + encode">*3.64%\**</span> | 2.22% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 51.53% | <span title="unvalidated">*85.33%\**</span> <span title="validated upfront with error">*84.45%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.23% | 1.63% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.0][ron] | 0.17% | 0.10% | 21.12% | 52.27% | 57.60% |
| [savefile 0.14.1][savefile] | 5.82% | 4.81% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 5.50% | 6.89% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.90% | 0.68% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.102][serde_json] | 0.35% | 0.38% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.60% | 0.33% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 97.08% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*53.03%\**</span> | <span title="unvalidated">*19.56%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*54.90%\**</span> | <span title="validated on-demand with panic">*50.82%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.76%\**</span> | <span title="validated on-demand with error">*0.72%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*45.03%\**</span> <span title="validated upfront with error">*3.05%\**</span> | <span title="unvalidated">*49.53%\**</span> <span title="validated upfront with error">*51.28%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.94%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.67%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 284.32 µs | <span title="unvalidated">*2.4529 ms\**</span> | 1290592 | 395825 | 332855 |
| [alkahest 0.1.5][alkahest] | 349.34 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 804.99 µs | 3.1806 ms | 569975 | 240525 | 232423 |
| [bitcode 0.4.0][bitcode] | 494.42 µs | 3.2534 ms | 322798 | 214279 | 201247 |
| [borsh 0.10.3][borsh] | 682.49 µs | 3.2150 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 4.7127 ms | 15.162 ms | 1619653 | 502185 | 328399 |
| [capnp 0.16.1][capnp] | 755.88 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 4.5017 ms | 13.412 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 1.0755 ms | 4.7985 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 4.9135 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.4482 ms | 4.7144 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.2681 ms | 6.6372 ms | 449745 | 252432 | 231110 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 771.96 µs | 3.5712 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.4][postcard] | 589.46 µs | 3.3172 ms | 367489 | 221913 | 207344 |
| [prost 0.11.9][prost] | <span title="encode">*1.6061 ms\**</span> <span title="populate + encode">*4.8335 ms\**</span> | 5.8026 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 476.37 µs | <span title="unvalidated">*2.3971 ms\**</span> <span title="validated upfront with error">*3.2715 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 2.1389 ms | 4.9464 ms | 424533 | 245214 | 226188 |
| [ron 0.8.0][ron] | 13.810 ms | 26.959 ms | 1465223 | 434935 | 343338 |
| [savefile 0.14.1][savefile] | 922.16 µs | 3.8408 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 1.0693 ms | 4.0748 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.5816 ms | 8.1403 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.102][serde_json] | 5.9980 ms | 12.791 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.9.2][simd-json] | 3.1745 ms | 7.4654 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 427.91 µs | 3.0066 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*79.195 µs\**</span> | <span title="unvalidated">*80.011 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.1170 ns\**</span> | <span title="validated on-demand with panic">*9.1205 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*132.48 ns\**</span> | <span title="validated on-demand with error">*813.97 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7874 ns\**</span> <span title="validated upfront with error">*2.7984 ms\**</span> | <span title="unvalidated">*3.4392 µs\**</span> <span title="validated upfront with error">*2.9757 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7105 ns\**</span> <span title="validated upfront with error">*814.75 µs\**</span> | <span title="unvalidated">*219.89 ns\**</span> <span title="validated upfront with error">*820.58 µs\**</span> | 2.8431 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*97.73%\**</span> | 25.01% | 53.81% | 59.63% |
| [alkahest 0.1.5][alkahest] | 81.39% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 35.32% | 75.37% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.4.0][bitcode] | 57.51% | 73.68% | 100.00% | 99.39% | 98.63% |
| [borsh 0.10.3][borsh] | 41.66% | 74.56% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.03% | 15.81% | 19.93% | 42.41% | 60.44% |
| [capnp 0.16.1][capnp] | 37.61% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.32% | 17.87% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 26.44% | 49.96% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 5.79% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 19.63% | 50.85% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.44% | 36.12% | 71.77% | 84.37% | 85.88% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 36.83% | 67.12% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.4][postcard] | 48.23% | 72.26% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="encode">*17.70%\**</span> <span title="populate + encode">*5.88%\**</span> | 41.31% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 59.68% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.27%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 13.29% | 48.46% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.0][ron] | 2.06% | 8.89% | 22.03% | 48.97% | 57.81% |
| [savefile 0.14.1][savefile] | 30.83% | 62.41% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 26.59% | 58.83% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.01% | 29.45% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.102][serde_json] | 4.74% | 18.74% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.9.2][simd-json] | 8.96% | 32.11% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 66.44% | 79.73% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.27%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*54.88%\**</span> | <span title="validated on-demand with panic">*2.41%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.29%\**</span> | <span title="validated on-demand with error">*27.01%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*45.16%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.39%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 687.01 µs | <span title="unvalidated">*4.1798 ms\**</span> | 2984682 | 1447310 | 1317587 |
| [alkahest 0.1.5][alkahest] | 1.0271 ms | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 4.4798 ms | 7.5316 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.9083 ms | 5.8875 ms | 870693 | 866738 | 870720 |
| [borsh 0.10.3][borsh] | 3.2972 ms | 6.6691 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 40.263 ms | 77.406 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.16.1][capnp] | 3.2464 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 25.401 ms | 61.025 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 7.8538 ms | 14.510 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 7.8446 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 5.5893 ms | 10.214 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 46.364 ms | 26.151 ms | 1728519 | 1247642 | 1233323 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 3.3238 ms | 5.8206 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.4][postcard] | 2.3816 ms | 6.5364 ms | 1279599 | 1058243 | 1016738 |
| [prost 0.11.9][prost] | <span title="encode">*6.5194 ms\**</span> <span title="populate + encode">*14.674 ms\**</span> | 15.750 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 2.1705 ms | <span title="unvalidated">*3.7737 ms\**</span> <span title="validated upfront with error">*4.8634 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.1][rmp-serde] | 14.645 ms | 18.548 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.0][ron] | 53.869 ms | 147.32 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.14.1][savefile] | 3.7458 ms | 7.2110 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 5.8830 ms | 9.3043 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 14.334 ms | 33.726 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.102][serde_json] | 33.368 ms | 55.683 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.9.2][simd-json] | 17.547 ms | 48.453 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 1.2297 ms | 4.3028 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*181.59 µs\**</span> | <span title="unvalidated">*182.44 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.0539 ns\**</span> | <span title="validated on-demand with panic">*977.32 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*140.04 ns\**</span> | <span title="validated on-demand with error">*1.3800 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7522 ns\**</span> <span title="validated upfront with error">*6.4894 ms\**</span> | <span title="unvalidated">*5.8621 µs\**</span> <span title="validated upfront with error">*7.5369 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6720 ns\**</span> <span title="validated upfront with error">*938.56 µs\**</span> | <span title="unvalidated">*558.09 ns\**</span> <span title="validated upfront with error">*938.23 µs\**</span> | 953.75 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*90.28%\**</span> | 29.17% | 59.89% | 66.08% |
| [alkahest 0.1.5][alkahest] | 66.89% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 15.34% | 50.10% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.4.0][bitcode] | 36.00% | 64.10% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 20.84% | 56.58% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 1.71% | 4.88% | 8.68% | 30.59% | 54.39% |
| [capnp 0.16.1][capnp] | 21.16% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.70% | 6.18% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 8.75% | 26.01% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 8.76% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 12.29% | 36.95% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.48% | 14.43% | 50.37% | 69.47% | 70.60% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 20.67% | 64.83% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.4][postcard] | 28.85% | 57.73% | 68.04% | 81.90% | 85.64% |
| [prost 0.11.9][prost] | <span title="encode">*10.54%\**</span> <span title="populate + encode">*4.68%\**</span> | 23.96% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 31.65% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.59%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.1][rmp-serde] | 4.69% | 20.35% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.0][ron] | 1.28% | 2.56% | 10.27% | 39.74% | 48.81% |
| [savefile 0.14.1][savefile] | 18.34% | 52.33% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 11.68% | 40.56% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 4.79% | 11.19% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.102][serde_json] | 2.06% | 6.78% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.9.2][simd-json] | 3.92% | 7.79% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 55.87% | 87.70% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.31%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*54.75%\**</span> | <span title="validated on-demand with panic">*57.10%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.19%\**</span> | <span title="validated on-demand with error">*40.44%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.56%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*9.52%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

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
[savefile]: https://crates.io/crates/savefile/0.14.1
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.102
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
