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

## Last updated: 2023-7-22 15:19:11

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 296.78 µs | <span title="unvalidated">*2.9852 ms\**</span> | 1705800 | 530419 | 403304 |
| [alkahest 0.1.5][alkahest] | 300.49 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 697.27 µs | 4.2059 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.4.0][bitcode] | 610.09 µs | 4.1721 ms | 703664 | 317711 | 273622 |
| [borsh 0.10.3][borsh] | 560.76 µs | 4.1540 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 3.4703 ms | 12.831 ms | 1924682 | 532821 | 376270 |
| [capnp 0.16.1][capnp] | 904.26 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 4.3867 ms | 14.290 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 1.0118 ms | 4.7915 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 2.7901 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.5924 ms | 4.5680 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 9.0815 ms | 7.0720 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.33][nanoserde] | 409.54 µs | 4.0285 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 720.58 µs | 4.5589 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.6][postcard] | 517.78 µs | 4.2953 ms | 724953 | 302399 | 253747 |
| [prost 0.11.9][prost] | <span title="encode">*673.57 µs\**</span> <span title="populate + encode">*4.1343 ms\**</span> | 5.0876 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 390.74 µs | <span title="unvalidated">*3.2543 ms\**</span> <span title="validated upfront with error">*4.2306 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.6095 ms | 5.8322 ms | 784997 | 325384 | 278219 |
| [ron 0.8.0][ron] | 25.623 ms | 24.925 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.0][savefile] | 313.54 µs | 3.9789 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 969.75 µs | 4.0761 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.4884 ms | 8.2101 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.103][serde_json] | 5.9847 ms | 11.408 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.8277 ms | 7.2679 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 287.97 µs | 3.4328 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*71.849 µs\**</span> | <span title="unvalidated">*111.01 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.9198 ns\**</span> | <span title="validated on-demand with panic">*45.843 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*132.04 ns\**</span> | <span title="validated on-demand with error">*486.33 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7042 ns\**</span> <span title="validated upfront with error">*2.8184 ms\**</span> | <span title="unvalidated">*123.06 µs\**</span> <span title="validated upfront with error">*2.6973 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6601 ns\**</span> <span title="validated upfront with error">*1.0129 ms\**</span> | <span title="unvalidated">*23.092 µs\**</span> <span title="validated upfront with error">*1.0279 ms\**</span> | 37.621 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 97.03% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 50.55% | 56.52% |
| [alkahest 0.1.5][alkahest] | 95.83% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 41.30% | 70.98% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.4.0][bitcode] | 47.20% | 71.55% | 100.00% | 84.40% | 83.31% |
| [borsh 0.10.3][borsh] | 51.35% | 71.86% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.30% | 23.27% | 36.56% | 50.32% | 60.58% |
| [capnp 0.16.1][capnp] | 31.85% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 6.56% | 20.89% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 28.46% | 62.30% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 10.32% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 18.08% | 65.35% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.17% | 42.21% | 85.95% | 80.63% | 79.84% |
| [nanoserde 0.1.33][nanoserde] | 70.32% | 74.10% | 67.29% | 71.86% | 73.12% |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 39.96% | 65.48% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.6][postcard] | 55.62% | 69.50% | 97.06% | 88.67% | 89.83% |
| [prost 0.11.9][prost] | <span title="encode">*42.75%\**</span> <span title="populate + encode">*6.97%\**</span> | 58.68% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 73.70% | <span title="unvalidated">*91.73%\**</span> <span title="validated upfront with error">*70.56%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.2][rmp-serde] | 17.89% | 51.18% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.0][ron] | 1.12% | 11.98% | 43.77% | 59.70% | 65.18% |
| [savefile 0.16.0][savefile] | 91.84% | 75.03% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 29.70% | 73.24% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.57% | 36.36% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.103][serde_json] | 4.81% | 26.17% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.18% | 41.07% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 100.00% | 86.96% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*20.80%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*56.86%\**</span> | <span title="validated on-demand with panic">*50.37%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.26%\**</span> | <span title="validated on-demand with error">*4.75%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.82%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.76%\**</span> <span title="validated upfront with error">*0.86%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.25%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 509.39 µs | <span title="unvalidated">*502.03 µs\**</span> | 6000024 | 5378513 | 5345890 |
| [alkahest 0.1.5][alkahest] | 564.40 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 5.3792 ms | 7.3951 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.4.0][bitcode] | 6.2505 ms | 10.913 ms | 4688054 | 4688491 | 4688168 |
| [borsh 0.10.3][borsh] | 7.0653 ms | 9.1615 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 57.309 ms | 141.43 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.16.1][capnp] | 11.975 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 90.131 ms | 142.94 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 10.014 ms | 13.076 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.8412 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 28.309 ms | 12.445 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 192.89 ms | 46.767 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.33][nanoserde] | 2.6102 ms | 2.1658 ms | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 5.1330 ms | 6.3059 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.6][postcard] | 879.88 µs | 2.0316 ms | 6000003 | 5378495 | 5345900 |
| [prost 0.11.9][prost] | <span title="encode">*9.2804 ms\**</span> <span title="populate + encode">*12.452 ms\**</span> | 19.655 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 949.27 µs | <span title="unvalidated">*557.11 µs\**</span> <span title="validated upfront with error">*552.71 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 20.830 ms | 30.261 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.0][ron] | 281.59 ms | 452.65 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.0][savefile] | 478.62 µs | 499.78 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 7.8128 ms | 6.8165 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 62.370 ms | 68.716 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.103][serde_json] | 137.05 ms | 126.59 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.9.2][simd-json] | 80.824 ms | 144.18 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 488.44 µs | 486.31 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*3.1763 ns\**</span> | <span title="unvalidated">*260.20 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.9023 ns\**</span> | <span title="validated on-demand with panic">*101.44 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*225.26 ns\**</span> | <span title="validated on-demand with error">*7.3093 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.8195 ns\**</span> <span title="validated upfront with error">*55.542 ns\**</span> | <span title="unvalidated">*98.270 µs\**</span> <span title="validated upfront with error">*99.003 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7470 ns\**</span> <span title="validated upfront with error">*16.307 ns\**</span> | <span title="unvalidated">*50.519 µs\**</span> <span title="validated upfront with error">*50.158 µs\**</span> | 360.68 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 93.96% | <span title="unvalidated">*96.87%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 84.80% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 8.90% | 6.58% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.4.0][bitcode] | 7.66% | 4.46% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 6.77% | 5.31% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.84% | 0.34% | 20.37% | 50.89% | 62.53% |
| [capnp 0.16.1][capnp] | 4.00% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.53% | 0.34% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 4.78% | 3.72% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 26.00% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 1.69% | 3.91% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.25% | 1.04% | 57.70% | 72.20% | 73.40% |
| [nanoserde 0.1.33][nanoserde] | 18.34% | 22.45% | 78.13% | 87.17% | 87.70% |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 9.32% | 7.71% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.6][postcard] | 54.40% | 23.94% | 78.13% | 87.17% | 87.70% |
| [prost 0.11.9][prost] | <span title="encode">*5.16%\**</span> <span title="populate + encode">*3.84%\**</span> | 2.47% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 50.42% | <span title="unvalidated">*87.29%\**</span> <span title="validated upfront with error">*87.99%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.2][rmp-serde] | 2.30% | 1.61% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.0][ron] | 0.17% | 0.11% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.0][savefile] | 100.00% | 97.30% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 6.13% | 7.13% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.77% | 0.71% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.103][serde_json] | 0.35% | 0.38% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.59% | 0.34% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 97.99% | 100.00% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*55.00%\**</span> | <span title="unvalidated">*19.28%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*60.19%\**</span> | <span title="validated on-demand with panic">*49.45%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.78%\**</span> | <span title="validated on-demand with error">*0.69%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*45.74%\**</span> <span title="validated upfront with error">*3.15%\**</span> | <span title="unvalidated">*51.04%\**</span> <span title="validated upfront with error">*50.66%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.71%\**</span> | <span title="unvalidated">*99.29%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 287.43 µs | <span title="unvalidated">*2.4721 ms\**</span> | 1290592 | 393058 | 332069 |
| [alkahest 0.1.5][alkahest] | 348.80 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 813.11 µs | 3.2062 ms | 569975 | 240525 | 232423 |
| [bitcode 0.4.0][bitcode] | 515.17 µs | 3.3509 ms | 322798 | 214279 | 201247 |
| [borsh 0.10.3][borsh] | 690.86 µs | 3.3449 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 4.4513 ms | 15.024 ms | 1619653 | 502185 | 328399 |
| [capnp 0.16.1][capnp] | 743.80 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 4.2776 ms | 13.420 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 1.0842 ms | 4.7311 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 4.8056 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.5437 ms | 4.8315 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.0342 ms | 6.2928 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.33][nanoserde] | 411.46 µs | 4.1710 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 779.51 µs | 3.9638 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.6][postcard] | 611.62 µs | 3.4889 ms | 367489 | 221913 | 207344 |
| [prost 0.11.9][prost] | <span title="encode">*1.4969 ms\**</span> <span title="populate + encode">*4.7059 ms\**</span> | 6.0861 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 484.18 µs | <span title="unvalidated">*2.4890 ms\**</span> <span title="validated upfront with error">*3.1942 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.8901 ms | 5.0289 ms | 424533 | 245214 | 226188 |
| [ron 0.8.0][ron] | 13.100 ms | 26.330 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.0][savefile] | 360.84 µs | 3.1940 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 1.0679 ms | 4.1493 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.4836 ms | 8.3732 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.103][serde_json] | 5.9162 ms | 12.212 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.9.2][simd-json] | 3.0959 ms | 7.1137 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 432.82 µs | 2.9926 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*79.552 µs\**</span> | <span title="unvalidated">*81.388 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.8508 ns\**</span> | <span title="validated on-demand with panic">*9.0395 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*130.63 ns\**</span> | <span title="validated on-demand with error">*832.21 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7963 ns\**</span> <span title="validated upfront with error">*3.0596 ms\**</span> | <span title="unvalidated">*3.3491 µs\**</span> <span title="validated upfront with error">*2.8672 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6350 ns\**</span> <span title="validated upfront with error">*825.57 µs\**</span> | <span title="unvalidated">*222.76 ns\**</span> <span title="validated upfront with error">*811.08 µs\**</span> | 2.8483 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*100.00%\**</span> | 25.01% | 54.18% | 59.77% |
| [alkahest 0.1.5][alkahest] | 82.41% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 35.35% | 77.10% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.4.0][bitcode] | 55.79% | 73.77% | 100.00% | 99.39% | 98.63% |
| [borsh 0.10.3][borsh] | 41.60% | 73.91% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.46% | 16.45% | 19.93% | 42.41% | 60.44% |
| [capnp 0.16.1][capnp] | 38.64% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.72% | 18.42% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 26.51% | 52.25% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 5.98% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 18.62% | 51.17% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.58% | 39.28% | 71.77% | 84.37% | 85.88% |
| [nanoserde 0.1.33][nanoserde] | 69.86% | 59.27% | 56.83% | 88.77% | 85.40% |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 36.87% | 62.37% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.6][postcard] | 46.99% | 70.86% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="encode">*19.20%\**</span> <span title="populate + encode">*6.11%\**</span> | 40.62% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 59.36% | <span title="unvalidated">*99.32%\**</span> <span title="validated upfront with error">*77.39%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.2][rmp-serde] | 15.21% | 49.16% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.0][ron] | 2.19% | 9.39% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.0][savefile] | 79.66% | 77.40% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 26.92% | 59.58% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.57% | 29.52% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.103][serde_json] | 4.86% | 20.24% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.9.2][simd-json] | 9.28% | 34.75% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 66.41% | 82.61% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.27%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*57.35%\**</span> | <span title="validated on-demand with panic">*2.46%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.25%\**</span> | <span title="validated on-demand with error">*26.77%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*43.07%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.65%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 705.93 µs | <span title="unvalidated">*4.1238 ms\**</span> | 2984682 | 1452145 | 1314318 |
| [alkahest 0.1.5][alkahest] | 1.0372 ms | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 4.3421 ms | 7.1736 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.9566 ms | 5.9081 ms | 870693 | 866738 | 870720 |
| [borsh 0.10.3][borsh] | 3.3102 ms | 6.5906 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 37.103 ms | 80.618 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.16.1][capnp] | 3.1462 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 25.451 ms | 59.061 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 7.2301 ms | 13.411 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 7.3427 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.8982 ms | 9.7964 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 48.845 ms | 25.827 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.33][nanoserde] | 1.5510 ms | 8.4899 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 3.3287 ms | 5.6841 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.6][postcard] | 2.4414 ms | 6.5609 ms | 1279599 | 1058243 | 1016738 |
| [prost 0.11.9][prost] | <span title="encode">*6.4275 ms\**</span> <span title="populate + encode">*14.005 ms\**</span> | 14.738 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.9644 ms | <span title="unvalidated">*3.7603 ms\**</span> <span title="validated upfront with error">*4.8183 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 11.903 ms | 18.560 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.0][ron] | 50.601 ms | 141.73 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.0][savefile] | 1.3631 ms | 5.1379 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 6.0083 ms | 8.8858 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 13.727 ms | 32.792 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.103][serde_json] | 33.671 ms | 51.856 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.9.2][simd-json] | 16.710 ms | 49.084 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 1.1187 ms | 4.3427 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*177.39 µs\**</span> | <span title="unvalidated">*177.51 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.8786 ns\**</span> | <span title="validated on-demand with panic">*993.67 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*126.30 ns\**</span> | <span title="validated on-demand with error">*1.4015 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7834 ns\**</span> <span title="validated upfront with error">*5.9156 ms\**</span> | <span title="unvalidated">*6.0567 µs\**</span> <span title="validated upfront with error">*6.0853 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6572 ns\**</span> <span title="validated upfront with error">*908.68 µs\**</span> | <span title="unvalidated">*532.69 ns\**</span> <span title="validated upfront with error">*925.68 µs\**</span> | 961.02 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*91.19%\**</span> | 29.17% | 59.69% | 66.25% |
| [alkahest 0.1.5][alkahest] | 68.06% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 16.26% | 52.42% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.4.0][bitcode] | 36.08% | 63.65% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 21.33% | 57.06% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 1.90% | 4.66% | 8.68% | 30.59% | 54.39% |
| [capnp 0.16.1][capnp] | 22.44% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.77% | 6.37% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 9.76% | 28.04% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 9.61% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 14.41% | 38.38% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.45% | 14.56% | 50.37% | 69.47% | 70.60% |
| [nanoserde 0.1.33][nanoserde] | 45.51% | 44.29% | 49.18% | 78.20% | 84.54% |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 21.21% | 66.15% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.6][postcard] | 28.91% | 57.31% | 68.04% | 81.90% | 85.64% |
| [prost 0.11.9][prost] | <span title="encode">*10.98%\**</span> <span title="populate + encode">*5.04%\**</span> | 25.51% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 35.94% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.04%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.2][rmp-serde] | 5.93% | 20.26% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.0][ron] | 1.40% | 2.65% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.0][savefile] | 51.79% | 73.19% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 11.75% | 42.32% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.14% | 11.47% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.103][serde_json] | 2.10% | 7.25% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.22% | 7.66% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 63.10% | 86.59% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.30%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*57.57%\**</span> | <span title="validated on-demand with panic">*53.61%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.31%\**</span> | <span title="validated on-demand with error">*38.01%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*43.80%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*8.80%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
[nanoserde]: https://crates.io/crates/nanoserde/0.1.33
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.4
[postcard]: https://crates.io/crates/postcard/1.0.6
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.0
[savefile]: https://crates.io/crates/savefile/0.16.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.103
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
