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

## Last updated: 2023-7-20 12:47:51

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 254.27 µs | <span title="unvalidated">*2.4129 ms\**</span> | 1705800 | 530420 | 403492 |
| [alkahest 0.1.5][alkahest] | 229.07 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 579.94 µs | 3.2866 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.4.0][bitcode] | 533.93 µs | 3.4477 ms | 703664 | 317711 | 273622 |
| [borsh 0.10.3][borsh] | 484.49 µs | 3.4655 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 2.5227 ms | 9.5208 ms | 1924682 | 532821 | 376270 |
| [capnp 0.16.1][capnp] | 812.26 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 3.6051 ms | 11.309 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 794.02 µs | 3.8991 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.9658 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.6596 ms | 3.8575 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.9779 ms | 5.5328 ms | 818669 | 332556 | 285514 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 617.65 µs | 3.6696 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.6][postcard] | 391.14 µs | 3.4716 ms | 724953 | 302399 | 253747 |
| [prost 0.11.9][prost] | <span title="encode">*595.83 µs\**</span> <span title="populate + encode">*3.3232 ms\**</span> | 4.1739 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 303.19 µs | <span title="unvalidated">*2.4801 ms\**</span> <span title="validated upfront with error">*3.3941 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.6375 ms | 4.6331 ms | 784997 | 325384 | 278219 |
| [ron 0.8.0][ron] | 20.161 ms | 21.838 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.0][savefile] | 232.46 µs | 3.2402 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 768.66 µs | 3.4933 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 1.9623 ms | 6.9751 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.103][serde_json] | 4.1454 ms | 8.9687 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.2489 ms | 5.8866 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 290.78 µs | 2.8686 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.777 µs\**</span> | <span title="unvalidated">*62.348 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6106 ns\**</span> | <span title="validated on-demand with panic">*35.270 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*108.03 ns\**</span> | <span title="validated on-demand with error">*383.12 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2701 ns\**</span> <span title="validated upfront with error">*2.2385 ms\**</span> | <span title="unvalidated">*96.861 µs\**</span> <span title="validated upfront with error">*2.3647 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5216 ns\**</span> <span title="validated upfront with error">*894.24 µs\**</span> | <span title="unvalidated">*16.410 µs\**</span> <span title="validated upfront with error">*911.74 µs\**</span> | 23.432 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 90.09% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 50.55% | 56.49% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 39.50% | 73.42% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.4.0][bitcode] | 42.90% | 69.99% | 100.00% | 84.40% | 83.31% |
| [borsh 0.10.3][borsh] | 47.28% | 69.63% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 9.08% | 25.34% | 36.56% | 50.32% | 60.58% |
| [capnp 0.16.1][capnp] | 28.20% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 6.35% | 21.34% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 28.85% | 61.88% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 11.65% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 13.80% | 62.55% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.87% | 43.61% | 85.95% | 80.63% | 79.84% |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 37.09% | 65.75% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.6][postcard] | 58.56% | 69.50% | 97.06% | 88.67% | 89.83% |
| [prost 0.11.9][prost] | <span title="encode">*38.45%\**</span> <span title="populate + encode">*6.89%\**</span> | 57.81% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 75.55% | <span title="unvalidated">*97.29%\**</span> <span title="validated upfront with error">*71.09%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 13.99% | 52.08% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.0][ron] | 1.14% | 11.05% | 43.77% | 59.70% | 65.18% |
| [savefile 0.16.0][savefile] | 98.54% | 74.47% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 29.80% | 69.07% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.67% | 34.59% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.103][serde_json] | 5.53% | 26.90% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.19% | 40.99% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 78.78% | 84.11% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*26.32%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.29%\**</span> | <span title="validated on-demand with panic">*46.53%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*4.28%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.53%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.94%\**</span> <span title="validated upfront with error">*0.69%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.80%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 464.94 µs | <span title="unvalidated">*463.80 µs\**</span> | 6000024 | 5378513 | 5345891 |
| [alkahest 0.1.5][alkahest] | 465.79 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 4.6338 ms | 5.2400 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.4.0][bitcode] | 5.2547 ms | 8.5581 ms | 4688054 | 4688491 | 4688168 |
| [borsh 0.10.3][borsh] | 5.4917 ms | 3.5310 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 55.050 ms | 97.759 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.16.1][capnp] | 11.347 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 75.411 ms | 110.16 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 7.4315 ms | 9.4899 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.2538 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 19.984 ms | 8.6594 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 191.37 ms | 36.539 ms | 8125037 | 6493484 | 6386940 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 3.8149 ms | 4.8759 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.6][postcard] | 724.36 µs | 1.5955 ms | 6000003 | 5378495 | 5345900 |
| [prost 0.11.9][prost] | <span title="encode">*11.654 ms\**</span> <span title="populate + encode">*13.769 ms\**</span> | 21.004 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 496.58 µs | <span title="unvalidated">*465.68 µs\**</span> <span title="validated upfront with error">*465.77 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 15.941 ms | 23.096 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.0][ron] | 216.75 ms | 392.10 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.0][savefile] | 470.71 µs | 469.90 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.7030 ms | 5.4727 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 55.216 ms | 60.339 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.103][serde_json] | 103.08 ms | 101.80 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.9.2][simd-json] | 64.949 ms | 113.26 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 474.50 µs | 472.99 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.4767 ns\**</span> | <span title="unvalidated">*252.57 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6106 ns\**</span> | <span title="validated on-demand with panic">*83.667 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*181.34 ns\**</span> | <span title="validated on-demand with error">*5.3848 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2696 ns\**</span> <span title="validated upfront with error">*43.870 ns\**</span> | <span title="unvalidated">*83.672 µs\**</span> <span title="validated upfront with error">*83.770 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5220 ns\**</span> <span title="validated upfront with error">*14.566 ns\**</span> | <span title="unvalidated">*47.020 µs\**</span> <span title="validated upfront with error">*46.942 µs\**</span> | 239.28 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*100.00%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 99.82% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 10.03% | 8.85% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.4.0][bitcode] | 8.85% | 5.42% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 8.47% | 13.14% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.84% | 0.47% | 20.37% | 50.89% | 62.53% |
| [capnp 0.16.1][capnp] | 4.10% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.62% | 0.42% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 6.26% | 4.89% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 37.08% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 2.33% | 5.36% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.24% | 1.27% | 57.70% | 72.20% | 73.40% |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 12.19% | 9.51% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.6][postcard] | 64.19% | 29.07% | 78.13% | 87.17% | 87.70% |
| [prost 0.11.9][prost] | <span title="encode">*3.99%\**</span> <span title="populate + encode">*3.38%\**</span> | 2.21% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 93.63% | <span title="unvalidated">*99.60%\**</span> <span title="validated upfront with error">*99.58%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.92% | 2.01% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.0][ron] | 0.21% | 0.12% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.0][savefile] | 98.77% | 98.70% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 6.94% | 8.47% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.84% | 0.77% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.103][serde_json] | 0.45% | 0.46% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.72% | 0.41% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 97.99% | 98.06% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.45%\**</span> | <span title="unvalidated">*18.59%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.30%\**</span> | <span title="validated on-demand with panic">*56.11%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.84%\**</span> | <span title="validated on-demand with error">*0.87%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.55%\**</span> <span title="validated upfront with error">*3.47%\**</span> | <span title="unvalidated">*56.10%\**</span> <span title="validated upfront with error">*56.04%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.45%\**</span> | <span title="unvalidated">*99.83%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 251.85 µs | <span title="unvalidated">*1.9992 ms\**</span> | 1290592 | 395738 | 333279 |
| [alkahest 0.1.5][alkahest] | 306.00 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 663.06 µs | 2.5658 ms | 569975 | 240525 | 232423 |
| [bitcode 0.4.0][bitcode] | 391.75 µs | 2.7257 ms | 322798 | 214279 | 201247 |
| [borsh 0.10.3][borsh] | 575.40 µs | 2.7376 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 3.5180 ms | 10.752 ms | 1619653 | 502185 | 328399 |
| [capnp 0.16.1][capnp] | 649.87 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.5789 ms | 10.260 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 882.84 µs | 3.6802 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.4234 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.2824 ms | 3.9934 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.5017 ms | 5.1102 ms | 449745 | 252432 | 231110 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 655.67 µs | 2.8103 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.6][postcard] | 509.25 µs | 2.8451 ms | 367489 | 221913 | 207344 |
| [prost 0.11.9][prost] | <span title="encode">*1.4341 ms\**</span> <span title="populate + encode">*3.9965 ms\**</span> | 4.6647 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 419.58 µs | <span title="unvalidated">*1.9102 ms\**</span> <span title="validated upfront with error">*2.6463 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.7046 ms | 4.0417 ms | 424533 | 245214 | 226188 |
| [ron 0.8.0][ron] | 10.307 ms | 22.299 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.0][savefile] | 298.53 µs | 2.6255 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 845.86 µs | 3.3204 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0766 ms | 6.8285 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.103][serde_json] | 4.2594 ms | 10.156 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.5421 ms | 5.7203 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 429.27 µs | 2.4180 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*64.598 µs\**</span> | <span title="unvalidated">*66.045 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6113 ns\**</span> | <span title="validated on-demand with panic">*7.0511 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*108.01 ns\**</span> | <span title="validated on-demand with error">*648.35 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2707 ns\**</span> <span title="validated upfront with error">*2.3117 ms\**</span> | <span title="unvalidated">*2.6714 µs\**</span> <span title="validated upfront with error">*2.3215 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5238 ns\**</span> <span title="validated upfront with error">*720.38 µs\**</span> | <span title="unvalidated">*189.43 ns\**</span> <span title="validated upfront with error">*720.27 µs\**</span> | 1.5695 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.55%\**</span> | 25.01% | 53.82% | 59.56% |
| [alkahest 0.1.5][alkahest] | 82.30% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 37.98% | 74.45% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.4.0][bitcode] | 64.29% | 70.08% | 100.00% | 99.39% | 98.63% |
| [borsh 0.10.3][borsh] | 43.77% | 69.78% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 7.16% | 17.77% | 19.93% | 42.41% | 60.44% |
| [capnp 0.16.1][capnp] | 38.75% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 7.04% | 18.62% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 28.53% | 51.90% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 7.36% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 19.64% | 47.83% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.36% | 37.38% | 71.77% | 84.37% | 85.88% |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 38.41% | 67.97% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.6][postcard] | 49.46% | 67.14% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="encode">*17.56%\**</span> <span title="populate + encode">*6.30%\**</span> | 40.95% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 60.02% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.18%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 14.77% | 47.26% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.0][ron] | 2.44% | 8.57% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.0][savefile] | 84.36% | 72.76% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 29.77% | 57.53% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 12.13% | 27.97% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.103][serde_json] | 5.91% | 18.81% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.9.2][simd-json] | 9.91% | 33.39% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 58.67% | 79.00% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.29%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.35%\**</span> | <span title="validated on-demand with panic">*2.69%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*29.22%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.59%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.09%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 632.12 µs | <span title="unvalidated">*3.3212 ms\**</span> | 2984682 | 1457929 | 1321678 |
| [alkahest 0.1.5][alkahest] | 962.51 µs | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 3.6234 ms | 6.0945 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.6621 ms | 4.7984 ms | 870693 | 866738 | 870720 |
| [borsh 0.10.3][borsh] | 2.7877 ms | 5.3259 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 30.908 ms | 56.080 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.16.1][capnp] | 2.6522 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 22.320 ms | 47.188 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.0433 ms | 11.262 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.4639 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.0115 ms | 8.1752 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 43.362 ms | 20.249 ms | 1728519 | 1247642 | 1233323 |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 2.8576 ms | 4.4879 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.6][postcard] | 2.0907 ms | 5.4712 ms | 1279599 | 1058243 | 1016738 |
| [prost 0.11.9][prost] | <span title="encode">*5.6864 ms\**</span> <span title="populate + encode">*11.695 ms\**</span> | 11.687 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.5059 ms | <span title="unvalidated">*3.0846 ms\**</span> <span title="validated upfront with error">*4.0223 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.1][rmp-serde] | 12.531 ms | 14.399 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.0][ron] | 40.411 ms | 121.32 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.0][savefile] | 1.1035 ms | 4.0382 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.8035 ms | 6.9522 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 11.170 ms | 27.800 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.103][serde_json] | 24.289 ms | 44.215 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.9.2][simd-json] | 13.346 ms | 40.693 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 1.2044 ms | 3.6859 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*124.09 µs\**</span> | <span title="unvalidated">*124.42 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6113 ns\**</span> | <span title="validated on-demand with panic">*790.46 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*110.36 ns\**</span> | <span title="validated on-demand with error">*1.8227 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2701 ns\**</span> <span title="validated upfront with error">*4.8495 ms\**</span> | <span title="unvalidated">*4.8104 µs\**</span> <span title="validated upfront with error">*4.7992 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5233 ns\**</span> <span title="validated upfront with error">*920.01 µs\**</span> | <span title="unvalidated">*400.23 ns\**</span> <span title="validated upfront with error">*919.42 µs\**</span> | 765.90 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.88%\**</span> | 29.17% | 59.45% | 65.88% |
| [alkahest 0.1.5][alkahest] | 65.67% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 17.45% | 50.61% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.4.0][bitcode] | 38.03% | 64.28% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 22.68% | 57.92% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 2.05% | 5.50% | 8.68% | 30.59% | 54.39% |
| [capnp 0.16.1][capnp] | 23.83% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.83% | 6.54% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 10.46% | 27.39% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 11.57% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 15.76% | 37.73% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.46% | 15.23% | 50.37% | 69.47% | 70.60% |
| [parity-scale-codec 3.6.4][parity-scale-codec] | 22.12% | 68.73% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.6][postcard] | 30.23% | 56.38% | 68.04% | 81.90% | 85.64% |
| [prost 0.11.9][prost] | <span title="encode">*11.12%\**</span> <span title="populate + encode">*5.41%\**</span> | 26.39% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 41.98% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*76.69%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.1][rmp-serde] | 5.04% | 21.42% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.0][ron] | 1.56% | 2.54% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.0][savefile] | 57.28% | 76.39% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 13.16% | 44.37% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.66% | 11.10% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.103][serde_json] | 2.60% | 6.98% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.74% | 7.58% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 52.48% | 83.69% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.32%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*58.33%\**</span> | <span title="validated on-demand with panic">*50.63%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.38%\**</span> | <span title="validated on-demand with error">*21.96%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.58%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*8.32%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.4
[postcard]: https://crates.io/crates/postcard/1.0.6
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.1
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
