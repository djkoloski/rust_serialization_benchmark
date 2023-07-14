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

## Last updated: 2023-7-14 22:29:39

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 256.26 µs | <span title="unvalidated">*2.4174 ms\**</span> | 1705800 | 530424 | 403282 |
| [alkahest 0.1.5][alkahest] | 253.77 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 425.40 µs | 3.2142 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.4.0][bitcode] | 547.88 µs | 3.4253 ms | 703664 | 317711 | 273622 |
| [borsh 0.10.3][borsh] | 478.42 µs | 3.3794 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 3.0797 ms | 10.825 ms | 1924682 | 532821 | 376270 |
| [capnp 0.16.1][capnp] | 792.59 µs | † | 1443216 | 513986 | 428649 |
| [ciborium 0.2.1][ciborium] | 2.8958 ms | 11.452 ms | 1407835 | 403440 | 324081 |
| [dlhn 0.1.6][dlhn] | 800.01 µs | 3.9209 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 2.0101 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.6055 ms | 4.0936 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.9451 ms | 6.0762 ms | 818669 | 332556 | 285514 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 617.51 µs | 4.4197 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.4][postcard] | 420.45 µs | 3.4659 ms | 724953 | 302399 | 253747 |
| [prost 0.11.9][prost] | <span title="encode">*596.01 µs\**</span> <span title="populate + encode">*3.2541 ms\**</span> | 4.0992 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 314.41 µs | <span title="unvalidated">*2.4838 ms\**</span> <span title="validated upfront with error">*3.4100 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.5182 ms | 4.6845 ms | 784997 | 325384 | 278219 |
| [ron 0.8.0][ron] | 21.208 ms | 24.814 ms | 1607459 | 449158 | 349713 |
| [savefile 0.15.0][savefile] | 964.61 µs | 3.9196 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 672.26 µs | 3.8501 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.2008 ms | 7.5521 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.102][serde_json] | 4.2421 ms | 9.2967 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.1713 ms | 5.8581 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 282.01 µs | 3.0028 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*39.314 µs\**</span> | <span title="unvalidated">*64.327 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7677 ns\**</span> | <span title="validated on-demand with panic">*84.084 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*124.72 ns\**</span> | <span title="validated on-demand with error">*400.45 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2708 ns\**</span> <span title="validated upfront with error">*2.2094 ms\**</span> | <span title="unvalidated">*200.90 µs\**</span> <span title="validated upfront with error">*2.4152 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5231 ns\**</span> <span title="validated upfront with error">*909.13 µs\**</span> | <span title="unvalidated">*16.232 µs\**</span> <span title="validated upfront with error">*924.98 µs\**</span> | 18.792 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 99.03% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 50.55% | 56.52% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 59.65% | 75.21% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.4.0][bitcode] | 46.32% | 70.57% | 100.00% | 84.40% | 83.31% |
| [borsh 0.10.3][borsh] | 53.04% | 71.53% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 8.24% | 22.33% | 36.56% | 50.32% | 60.58% |
| [capnp 0.16.1][capnp] | 32.02% | † | 48.76% | 52.17% | 53.18% |
| [ciborium 0.2.1][ciborium] | 8.76% | 21.11% | 49.98% | 66.46% | 70.34% |
| [dlhn 0.1.6][dlhn] | 31.72% | 61.65% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 12.62% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 15.81% | 59.05% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.84% | 39.78% | 85.95% | 80.63% | 79.84% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 41.10% | 54.70% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.4][postcard] | 60.36% | 69.75% | 97.06% | 88.67% | 89.83% |
| [prost 0.11.9][prost] | <span title="encode">*42.58%\**</span> <span title="populate + encode">*7.80%\**</span> | 58.97% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 80.71% | <span title="unvalidated">*97.33%\**</span> <span title="validated upfront with error">*70.89%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 16.72% | 51.60% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.0][ron] | 1.20% | 9.74% | 43.77% | 59.70% | 65.18% |
| [savefile 0.15.0][savefile] | 26.31% | 61.67% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 37.75% | 62.79% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.53% | 32.01% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.102][serde_json] | 5.98% | 26.00% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.9.2][simd-json] | 11.69% | 41.27% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 89.99% | 80.50% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*25.23%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.03%\**</span> | <span title="validated on-demand with panic">*19.30%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.22%\**</span> | <span title="validated on-demand with error">*4.05%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.57%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*8.08%\**</span> <span title="validated upfront with error">*0.67%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.75%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 513.41 µs | <span title="unvalidated">*468.58 µs\**</span> | 6000024 | 5378513 | 5345891 |
| [alkahest 0.1.5][alkahest] | 468.02 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 4.3282 ms | 2.0272 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.4.0][bitcode] | 5.1193 ms | 8.4225 ms | 4688054 | 4688491 | 4688168 |
| [borsh 0.10.3][borsh] | 5.6903 ms | 5.4462 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 58.569 ms | 121.89 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.16.1][capnp] | 16.203 ms | † | 14000088 | 7130367 | 6051062 |
| [ciborium 0.2.1][ciborium] | 76.676 ms | 110.61 ms | 13122324 | 7524660 | 6759658 |
| [dlhn 0.1.6][dlhn] | 7.2544 ms | 9.9436 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.4160 ms | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 19.428 ms | 13.012 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 213.02 ms | 51.583 ms | 8125037 | 6493484 | 6386940 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 5.3827 ms | 11.272 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.4][postcard] | 1.0554 ms | 1.5991 ms | 6000003 | 5378495 | 5345900 |
| [prost 0.11.9][prost] | <span title="encode">*10.436 ms\**</span> <span title="populate + encode">*12.505 ms\**</span> | 19.486 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 540.22 µs | <span title="unvalidated">*480.52 µs\**</span> <span title="validated upfront with error">*488.68 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 15.782 ms | 23.546 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.0][ron] | 228.52 ms | 419.35 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.15.0][savefile] | 537.92 µs | 479.77 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 5.4556 ms | 9.8260 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 49.048 ms | 69.831 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.102][serde_json] | 107.62 ms | 104.19 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.9.2][simd-json] | 66.036 ms | 123.64 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 478.99 µs | 469.46 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.4780 ns\**</span> | <span title="unvalidated">*267.88 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7677 ns\**</span> | <span title="validated on-demand with panic">*97.614 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*196.78 ns\**</span> | <span title="validated on-demand with error">*5.8264 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2736 ns\**</span> <span title="validated upfront with error">*47.176 ns\**</span> | <span title="unvalidated">*83.714 µs\**</span> <span title="validated upfront with error">*83.814 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5227 ns\**</span> <span title="validated upfront with error">*15.144 ns\**</span> | <span title="unvalidated">*47.075 µs\**</span> <span title="validated upfront with error">*47.170 µs\**</span> | 252.92 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 91.16% | <span title="unvalidated">*100.00%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 10.81% | 23.11% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.4.0][bitcode] | 9.14% | 5.56% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 8.22% | 8.60% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.80% | 0.38% | 20.37% | 50.89% | 62.53% |
| [capnp 0.16.1][capnp] | 2.89% | † | 33.49% | 65.75% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.61% | 0.42% | 35.73% | 62.31% | 69.36% |
| [dlhn 0.1.6][dlhn] | 6.45% | 4.71% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 33.05% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 2.41% | 3.60% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.22% | 0.91% | 57.70% | 72.20% | 73.40% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 8.69% | 4.16% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.4][postcard] | 44.35% | 29.30% | 78.13% | 87.17% | 87.70% |
| [prost 0.11.9][prost] | <span title="encode">*4.48%\**</span> <span title="populate + encode">*3.74%\**</span> | 2.40% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 86.64% | <span title="unvalidated">*97.52%\**</span> <span title="validated upfront with error">*95.89%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.97% | 1.99% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.0][ron] | 0.20% | 0.11% | 21.12% | 52.27% | 57.60% |
| [savefile 0.15.0][savefile] | 87.01% | 97.67% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 8.58% | 4.77% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.95% | 0.67% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.102][serde_json] | 0.43% | 0.45% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.71% | 0.38% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 97.71% | 99.81% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.45%\**</span> | <span title="unvalidated">*17.57%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.02%\**</span> | <span title="validated on-demand with panic">*48.23%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.77%\**</span> | <span title="validated on-demand with error">*0.81%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.51%\**</span> <span title="validated upfront with error">*3.23%\**</span> | <span title="unvalidated">*56.23%\**</span> <span title="validated upfront with error">*56.17%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.05%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.80%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 244.49 µs | <span title="unvalidated">*1.9998 ms\**</span> | 1290592 | 395706 | 333601 |
| [alkahest 0.1.5][alkahest] | 299.53 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 605.15 µs | 2.5993 ms | 569975 | 240525 | 232423 |
| [bitcode 0.4.0][bitcode] | 396.82 µs | 2.7057 ms | 322798 | 214279 | 201247 |
| [borsh 0.10.3][borsh] | 574.72 µs | 2.7467 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 4.0145 ms | 12.419 ms | 1619653 | 502185 | 328399 |
| [capnp 0.16.1][capnp] | 679.28 µs | † | 803896 | 335606 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.7062 ms | 10.369 ms | 1109821 | 344751 | 274526 |
| [dlhn 0.1.6][dlhn] | 900.33 µs | 3.6978 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.5202 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.3004 ms | 4.1165 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.3199 ms | 5.4063 ms | 449745 | 252432 | 231110 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 646.00 µs | 3.5715 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.4][postcard] | 504.51 µs | 2.8560 ms | 367489 | 221913 | 207344 |
| [prost 0.11.9][prost] | <span title="encode">*1.4934 ms\**</span> <span title="populate + encode">*3.9625 ms\**</span> | 4.7906 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 414.78 µs | <span title="unvalidated">*1.9041 ms\**</span> <span title="validated upfront with error">*2.6584 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.7718 ms | 4.3494 ms | 424533 | 245214 | 226188 |
| [ron 0.8.0][ron] | 11.507 ms | 25.100 ms | 1465223 | 434935 | 343338 |
| [savefile 0.15.0][savefile] | 1.0171 ms | 3.2605 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 784.17 µs | 3.7275 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.2789 ms | 7.2137 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.102][serde_json] | 4.2371 ms | 10.407 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.5043 ms | 5.7221 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 425.73 µs | 2.4264 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*64.982 µs\**</span> | <span title="unvalidated">*66.621 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7688 ns\**</span> | <span title="validated on-demand with panic">*11.208 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*124.70 ns\**</span> | <span title="validated on-demand with error">*673.17 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2716 ns\**</span> <span title="validated upfront with error">*2.2871 ms\**</span> | <span title="unvalidated">*5.7168 µs\**</span> <span title="validated upfront with error">*2.2904 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5238 ns\**</span> <span title="validated upfront with error">*740.29 µs\**</span> | <span title="unvalidated">*189.51 ns\**</span> <span title="validated upfront with error">*740.47 µs\**</span> | 1.5810 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.21%\**</span> | 25.01% | 53.82% | 59.50% |
| [alkahest 0.1.5][alkahest] | 81.62% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 40.40% | 73.25% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.4.0][bitcode] | 61.61% | 70.37% | 100.00% | 99.39% | 98.63% |
| [borsh 0.10.3][borsh] | 42.54% | 69.32% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.09% | 15.33% | 19.93% | 42.41% | 60.44% |
| [capnp 0.16.1][capnp] | 35.99% | † | 40.15% | 63.46% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.60% | 18.36% | 29.09% | 61.78% | 72.30% |
| [dlhn 0.1.6][dlhn] | 27.16% | 51.49% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 6.95% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 18.80% | 46.26% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.94% | 35.22% | 71.77% | 84.37% | 85.88% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 37.85% | 53.31% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.4][postcard] | 48.46% | 66.67% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="encode">*16.37%\**</span> <span title="populate + encode">*6.17%\**</span> | 39.75% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 58.94% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.63%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 13.80% | 43.78% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.0][ron] | 2.12% | 7.59% | 22.03% | 48.97% | 57.81% |
| [savefile 0.15.0][savefile] | 24.04% | 58.40% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 31.18% | 51.08% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 10.73% | 26.40% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.102][serde_json] | 5.77% | 18.30% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.9.2][simd-json] | 9.76% | 33.28% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 57.43% | 78.47% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.28%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.03%\**</span> | <span title="validated on-demand with panic">*1.69%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.22%\**</span> | <span title="validated on-demand with error">*28.15%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.58%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*3.31%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 585.03 µs | <span title="unvalidated">*3.3608 ms\**</span> | 2984682 | 1451958 | 1314239 |
| [alkahest 0.1.5][alkahest] | 1.0180 ms | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 3.4767 ms | 5.8101 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.6339 ms | 4.8173 ms | 870693 | 866738 | 870720 |
| [borsh 0.10.3][borsh] | 2.8132 ms | 5.4871 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 33.349 ms | 68.589 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.16.1][capnp] | 3.3041 ms | † | 2664040 | 1511895 | 1212087 |
| [ciborium 0.2.1][ciborium] | 24.662 ms | 47.377 ms | 5878653 | 1655791 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.2400 ms | 11.659 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.8509 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.0171 ms | 9.0578 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 49.856 ms | 23.168 ms | 1728519 | 1247642 | 1233323 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 2.8399 ms | 8.3124 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.4][postcard] | 2.0895 ms | 5.4503 ms | 1279599 | 1058243 | 1016738 |
| [prost 0.11.9][prost] | <span title="encode">*5.4987 ms\**</span> <span title="populate + encode">*11.683 ms\**</span> | 11.972 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.7494 ms | <span title="unvalidated">*3.1171 ms\**</span> <span title="validated upfront with error">*4.0835 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.1][rmp-serde] | 12.188 ms | 15.013 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.0][ron] | 48.763 ms | 141.87 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.15.0][savefile] | 4.2083 ms | 7.0580 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.9325 ms | 10.673 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 12.586 ms | 29.180 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.102][serde_json] | 24.070 ms | 48.688 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.9.2][simd-json] | 13.309 ms | 40.471 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 1.2753 ms | 3.6164 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*122.96 µs\**</span> | <span title="unvalidated">*124.86 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7688 ns\**</span> | <span title="validated on-demand with panic">*790.35 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*124.59 ns\**</span> | <span title="validated on-demand with error">*1.8313 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2722 ns\**</span> <span title="validated upfront with error">*4.8607 ms\**</span> | <span title="unvalidated">*8.4675 µs\**</span> <span title="validated upfront with error">*4.8601 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5229 ns\**</span> <span title="validated upfront with error">*904.00 µs\**</span> | <span title="unvalidated">*390.36 ns\**</span> <span title="validated upfront with error">*906.05 µs\**</span> | 766.25 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.75%\**</span> | 29.17% | 59.69% | 66.25% |
| [alkahest 0.1.5][alkahest] | 57.47% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 16.83% | 53.65% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.4.0][bitcode] | 35.81% | 64.71% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 20.80% | 56.81% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 1.75% | 4.54% | 8.68% | 30.59% | 54.39% |
| [capnp 0.16.1][capnp] | 17.71% | † | 32.68% | 57.33% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.37% | 6.58% | 14.81% | 52.35% | 60.82% |
| [dlhn 0.1.6][dlhn] | 9.38% | 26.74% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 10.00% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 14.56% | 34.41% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.17% | 13.45% | 50.37% | 69.47% | 70.60% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 20.60% | 37.50% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.4][postcard] | 28.00% | 57.19% | 68.04% | 81.90% | 85.64% |
| [prost 0.11.9][prost] | <span title="encode">*10.64%\**</span> <span title="populate + encode">*5.01%\**</span> | 26.04% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 33.44% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*76.33%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.1][rmp-serde] | 4.80% | 20.76% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.0][ron] | 1.20% | 2.20% | 10.27% | 39.74% | 48.81% |
| [savefile 0.15.0][savefile] | 13.90% | 44.16% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 11.86% | 29.21% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 4.65% | 10.68% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.102][serde_json] | 2.43% | 6.40% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.40% | 7.70% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 45.87% | 86.19% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.31%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.00%\**</span> | <span title="validated on-demand with panic">*49.39%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.22%\**</span> | <span title="validated on-demand with error">*21.32%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.54%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*4.61%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
