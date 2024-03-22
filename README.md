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
* **Zstd**: the size of the buffer after zstd compression
* **Zstd Time**: the time taken to compress the serialized buffer with zstd

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2024-3-17 17:2:47

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 396.11 µs | <span title="unvalidated">*1.3644 ms\**</span> | 1705800 | 520091 | 413322 | 6.6194 ms |
| [alkahest 0.1.5][alkahest] | 180.47 µs | † | 1045784 | 454157 | 389424 | 6.0594 ms |
| [bincode 2.0.0-rc][bincode] | 206.02 µs | 2.2984 ms | 741295 | 303944 | 257153 | 4.0485 ms |
| [bincode 1.3.3][bincode1] | 494.72 µs | 1.9447 ms | 1045784 | 373127 | 311761 | 4.6721 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 136.83 µs | 1.4074 ms | 703710 | 288826 | 229755 | 2.4524 ms |
| [borsh 1.3.0][borsh] | 512.14 µs | 2.1747 ms | 885780 | 362204 | 286514 | 4.4500 ms |
| [bson 2.9.0][bson] | 2.2131 ms | 6.7858 ms | 1924682 | 532821 | 376270 | 5.7688 ms |
| [capnp 0.18.13][capnp] | 437.59 µs | † | 1443216 | 513986 | 428649 | 6.9451 ms |
| [cbor4ii 0.3.2][cbor4ii] | 870.63 µs | 4.5723 ms | 1407835 | 403440 | 324081 | 4.8908 ms |
| [ciborium 0.2.2][ciborium] | 3.8633 ms | 9.4348 ms | 1407835 | 403440 | 324081 | 4.5520 ms |
| [databuf 0.5.0][databuf] | 241.69 µs | 1.9876 ms | 765778 | 311715 | 264630 | 4.0599 ms |
| [dlhn 0.1.6][dlhn] | 788.97 µs | 2.3576 ms | 724953 | 301446 | 253629 | 3.5807 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.4056 ms | † | 1276368 | 468539 | 388832 | 5.4928 ms |
| [msgpacker 0.4.3][msgpacker] | 1.1826 ms | 2.7557 ms | 764996 | 315291 | 264898 | 4.1878 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3093 ms | 3.8669 ms | 818669 | 332556 | 285514 | 4.5036 ms |
| [nanoserde 0.1.37][nanoserde] | 251.36 µs | 1.9972 ms | 1045784 | 373127 | 311761 | 4.3444 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 646.71 µs | 2.3826 ms | 765778 | 311743 | 264518 | 4.3992 ms |
| [postcard 1.0.8][postcard] | 407.98 µs | 2.0345 ms | 724953 | 302399 | 253747 | 3.7052 ms |
| [pot 3.0.0][pot] | 2.2027 ms | 6.4596 ms | 971922 | 372513 | 304122 | 5.0204 ms |
| [prost 0.12.3][prost] | <span title="encode">*775.70 µs\**</span> <span title="populate + encode">*2.2613 ms\**</span> | 3.3516 ms | 884628 | 363130 | 315494 | 4.8273 ms |
| [rkyv 0.7.44][rkyv] | 203.48 µs | <span title="unvalidated">*1.3638 ms\**</span> <span title="validated upfront with error">*1.8497 ms\**</span> | 1011488 | 383862 | 333545 | 4.9594 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.2805 ms | 3.3091 ms | 784997 | 325384 | 278219 | 4.7362 ms |
| [ron 0.8.1][ron] | 13.374 ms | 16.220 ms | 1607459 | 449158 | 349713 | 6.0093 ms |
| [savefile 0.16.5][savefile] | 190.80 µs | 1.9531 ms | 1045800 | 373139 | 311755 | 4.7810 ms |
| [serde_bare 0.5.0][serde_bare] | 621.36 µs | 2.0137 ms | 765778 | 311715 | 264630 | 3.9786 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8252 ms | 4.6009 ms | 1407835 | 403440 | 324081 | 4.8413 ms |
| [serde_json 1.0.114][serde_json] | 3.5718 ms | 5.4603 ms | 1827461 | 470560 | 361090 | 5.7711 ms |
| [simd-json 0.13.8][simd-json] | 1.9908 ms | 4.4120 ms | 1827461 | 470560 | 361090 | 5.6446 ms |
| [speedy 0.8.7][speedy] | 187.65 µs | 1.7361 ms | 885780 | 362204 | 286514 | 4.4882 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*21.105 µs\**</span> | <span title="unvalidated">*34.769 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8269 ns\**</span> | <span title="validated on-demand with panic">*24.365 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*73.571 ns\**</span> | <span title="validated on-demand with error">*159.53 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.3746 ns\**</span> <span title="validated upfront with error">*1.7890 ms\**</span> | <span title="unvalidated">*47.805 µs\**</span> <span title="validated upfront with error">*1.8544 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.1691 ns\**</span> <span title="validated upfront with error">*478.93 µs\**</span> | <span title="unvalidated">*10.714 µs\**</span> <span title="validated upfront with error">*511.18 µs\**</span> | 9.7280 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 34.54% | <span title="unvalidated">*99.96%\**</span> | 41.25% | 55.53% | 55.59% | 37.05% |
| [alkahest 0.1.5][alkahest] | 75.82% | † | 67.29% | 63.60% | 59.00% | 40.47% |
| [bincode 2.0.0-rc][bincode] | 66.42% | 59.34% | 94.93% | 95.03% | 89.35% | 60.58% |
| [bincode 1.3.3][bincode1] | 27.66% | 70.13% | 67.29% | 77.41% | 73.70% | 52.49% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 96.90% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 26.72% | 62.71% | 79.45% | 79.74% | 80.19% | 55.11% |
| [bson 2.9.0][bson] | 6.18% | 20.10% | 36.56% | 54.21% | 61.06% | 42.51% |
| [capnp 0.18.13][capnp] | 31.27% | † | 48.76% | 56.19% | 53.60% | 35.31% |
| [cbor4ii 0.3.2][cbor4ii] | 15.72% | 29.83% | 49.99% | 71.59% | 70.89% | 50.14% |
| [ciborium 0.2.2][ciborium] | 3.54% | 14.45% | 49.99% | 71.59% | 70.89% | 53.88% |
| [databuf 0.5.0][databuf] | 56.61% | 68.62% | 91.89% | 92.66% | 86.82% | 60.41% |
| [dlhn 0.1.6][dlhn] | 17.34% | 57.85% | 97.07% | 95.81% | 90.59% | 68.49% |
| [flatbuffers 23.5.26][flatbuffers] | 9.73% | † | 55.13% | 61.64% | 59.09% | 44.65% |
| [msgpacker 0.4.3][msgpacker] | 11.57% | 49.49% | 91.99% | 91.61% | 86.73% | 58.56% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.58% | 35.27% | 85.96% | 86.85% | 80.47% | 54.45% |
| [nanoserde 0.1.37][nanoserde] | 54.44% | 68.29% | 67.29% | 77.41% | 73.70% | 56.45% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 21.16% | 57.24% | 91.89% | 92.65% | 86.86% | 55.75% |
| [postcard 1.0.8][postcard] | 33.54% | 67.03% | 97.07% | 95.51% | 90.54% | 66.19% |
| [pot 3.0.0][pot] | 6.21% | 21.11% | 72.40% | 77.53% | 75.55% | 48.85% |
| [prost 0.12.3][prost] | <span title="encode">*17.64%\**</span> <span title="populate + encode">*6.05%\**</span> | 40.69% | 79.55% | 79.54% | 72.82% | 50.80% |
| [rkyv 0.7.44][rkyv] | 67.24% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.73%\**</span> | 69.57% | 75.24% | 68.88% | 49.45% |
| [rmp-serde 1.1.2][rmp-serde] | 10.69% | 41.21% | 89.64% | 88.76% | 82.58% | 51.78% |
| [ron 0.8.1][ron] | 1.02% | 8.41% | 43.78% | 64.30% | 65.70% | 40.81% |
| [savefile 0.16.5][savefile] | 71.71% | 69.83% | 67.29% | 77.40% | 73.70% | 51.29% |
| [serde_bare 0.5.0][serde_bare] | 22.02% | 67.73% | 91.89% | 92.66% | 86.82% | 61.64% |
| [serde_cbor 0.11.2][serde_cbor] | 7.50% | 29.64% | 49.99% | 71.59% | 70.89% | 50.66% |
| [serde_json 1.0.114][serde_json] | 3.83% | 24.98% | 38.51% | 61.38% | 63.63% | 42.49% |
| [simd-json 0.13.8][simd-json] | 6.87% | 30.91% | 38.51% | 61.38% | 63.63% | 43.45% |
| [speedy 0.8.7][speedy] | 72.92% | 78.56% | 79.45% | 79.74% | 80.19% | 54.64% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*30.81%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*63.99%\**</span> | <span title="validated on-demand with panic">*43.97%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.59%\**</span> | <span title="validated on-demand with error">*6.72%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.23%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*22.41%\**</span> <span title="validated upfront with error">*0.58%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.10%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 259.26 µs | <span title="unvalidated">*258.46 µs\**</span> | 6000024 | 5378514 | 5345891 | 7.0252 ms |
| [alkahest 0.1.5][alkahest] | 185.69 µs | † | 6000008 | 5378500 | 5345890 | 7.2939 ms |
| [bincode 2.0.0-rc][bincode] | 410.35 µs | 780.19 µs | 6000005 | 5378497 | 5345897 | 7.2675 ms |
| [bincode 1.3.3][bincode1] | 4.8533 ms | 4.1500 ms | 6000008 | 5378500 | 5345890 | 7.3535 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 1.3328 ms | 562.47 µs | 6000006 | 5182295 | 4923880 | 13.000 ms |
| [borsh 1.3.0][borsh] | 5.8626 ms | 4.6520 ms | 6000004 | 5378496 | 5345889 | 7.6449 ms |
| [bson 2.9.0][bson] | 44.747 ms | 77.130 ms | 23013911 | 9212089 | 7497811 | 119.22 ms |
| [capnp 0.18.13][capnp] | 6.9201 ms | † | 14000088 | 7130367 | 6051062 | 87.583 ms |
| [cbor4ii 0.3.2][cbor4ii] | 9.9004 ms | 44.636 ms | 13125016 | 7524114 | 6757967 | 97.096 ms |
| [ciborium 0.2.2][ciborium] | 62.802 ms | 102.72 ms | 13122324 | 7524660 | 6759658 | 99.682 ms |
| [databuf 0.5.0][databuf] | 2.2815 ms | 5.0312 ms | 6000003 | 5378495 | 5345900 | 7.7310 ms |
| [dlhn 0.1.6][dlhn] | 6.7637 ms | 5.4389 ms | 6000003 | 5378495 | 5345900 | 7.0804 ms |
| [flatbuffers 23.5.26][flatbuffers] | 623.38 µs | † | 6000024 | 5378434 | 5345910 | 7.1477 ms |
| [msgpacker 0.4.3][msgpacker] | 18.976 ms | 8.1034 ms | 7500005 | 6058442 | 6014337 | 9.1624 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 114.41 ms | 25.912 ms | 8125037 | 6493484 | 6386940 | 82.883 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2711 ms | 843.81 µs | 6000008 | 5378500 | 5345890 | 7.1836 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.8819 ms | 3.8327 ms | 6000004 | 5378496 | 5345889 | 7.2944 ms |
| [postcard 1.0.8][postcard] | 458.90 µs | 1.7157 ms | 6000003 | 5378495 | 5345900 | 7.6678 ms |
| [pot 3.0.0][pot] | 36.089 ms | 68.865 ms | 10122342 | 6814618 | 6852251 | 90.530 ms |
| [prost 0.12.3][prost] | <span title="encode">*6.4241 ms\**</span> <span title="populate + encode">*8.4591 ms\**</span> | 13.445 ms | 8750000 | 6665735 | 6421871 | 80.115 ms |
| [rkyv 0.7.44][rkyv] | 190.69 µs | <span title="unvalidated">*158.12 µs\**</span> <span title="validated upfront with error">*140.01 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.0441 ms |
| [rmp-serde 1.1.2][rmp-serde] | 12.335 ms | 18.142 ms | 8125006 | 6494876 | 6391037 | 76.450 ms |
| [ron 0.8.1][ron] | 164.43 ms | 246.45 ms | 22192885 | 8970395 | 8138755 | 157.95 ms |
| [savefile 0.16.5][savefile] | 258.53 µs | 599.27 µs | 6000024 | 5378518 | 5345893 | 7.7083 ms |
| [serde_bare 0.5.0][serde_bare] | 6.1794 ms | 3.9776 ms | 6000003 | 5378495 | 5345900 | 7.5638 ms |
| [serde_cbor 0.11.2][serde_cbor] | 34.092 ms | 40.384 ms | 13122324 | 7524660 | 6759658 | 94.514 ms |
| [serde_json 1.0.114][serde_json] | 85.851 ms | 83.039 ms | 26192883 | 9566084 | 8586741 | 152.30 ms |
| [simd-json 0.13.8][simd-json] | 51.566 ms | 69.867 ms | 26192883 | 9566084 | 8586741 | 159.50 ms |
| [speedy 0.8.7][speedy] | 258.68 µs | 258.94 µs | 6000004 | 5378496 | 5345889 | 7.1246 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.0215 ns\**</span> | <span title="unvalidated">*132.63 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.7459 ns\**</span> | <span title="validated on-demand with panic">*73.157 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*100.40 ns\**</span> | <span title="validated on-demand with error">*2.3972 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.3411 ns\**</span> <span title="validated upfront with error">*37.123 ns\**</span> | <span title="unvalidated">*52.738 µs\**</span> <span title="validated upfront with error">*73.777 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.1806 ns\**</span> <span title="validated upfront with error">*9.5711 ns\**</span> | <span title="unvalidated">*46.267 µs\**</span> <span title="validated upfront with error">*38.585 µs\**</span> | 103.70 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 71.62% | <span title="unvalidated">*54.17%\**</span> | 100.00% | 96.35% | 92.11% | 100.00% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 96.32% |
| [bincode 2.0.0-rc][bincode] | 45.25% | 17.95% | 100.00% | 96.35% | 92.11% | 96.67% |
| [bincode 1.3.3][bincode1] | 3.83% | 3.37% | 100.00% | 96.35% | 92.11% | 95.54% |
| [bitcode 0.6.0-alpha.2][bitcode] | 13.93% | 24.89% | 100.00% | 100.00% | 100.00% | 54.04% |
| [borsh 1.3.0][borsh] | 3.17% | 3.01% | 100.00% | 96.35% | 92.11% | 91.89% |
| [bson 2.9.0][bson] | 0.41% | 0.18% | 26.07% | 56.26% | 65.67% | 5.89% |
| [capnp 0.18.13][capnp] | 2.68% | † | 42.86% | 72.68% | 81.37% | 8.02% |
| [cbor4ii 0.3.2][cbor4ii] | 1.88% | 0.31% | 45.71% | 68.88% | 72.86% | 7.24% |
| [ciborium 0.2.2][ciborium] | 0.30% | 0.14% | 45.72% | 68.87% | 72.84% | 7.05% |
| [databuf 0.5.0][databuf] | 8.14% | 2.78% | 100.00% | 96.35% | 92.11% | 90.87% |
| [dlhn 0.1.6][dlhn] | 2.75% | 2.57% | 100.00% | 96.35% | 92.11% | 99.22% |
| [flatbuffers 23.5.26][flatbuffers] | 29.79% | † | 100.00% | 96.35% | 92.11% | 98.29% |
| [msgpacker 0.4.3][msgpacker] | 0.98% | 1.73% | 80.00% | 85.54% | 81.87% | 76.67% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.54% | 73.85% | 79.81% | 77.09% | 8.48% |
| [nanoserde 0.1.37][nanoserde] | 14.61% | 16.59% | 100.00% | 96.35% | 92.11% | 97.79% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.80% | 3.65% | 100.00% | 96.35% | 92.11% | 96.31% |
| [postcard 1.0.8][postcard] | 40.46% | 8.16% | 100.00% | 96.35% | 92.11% | 91.62% |
| [pot 3.0.0][pot] | 0.51% | 0.20% | 59.27% | 76.05% | 71.86% | 7.76% |
| [prost 0.12.3][prost] | <span title="encode">*2.89%\**</span> <span title="populate + encode">*2.20%\**</span> | 1.04% | 68.57% | 77.75% | 76.67% | 8.77% |
| [rkyv 0.7.44][rkyv] | 97.38% | <span title="unvalidated">*88.55%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 99.73% |
| [rmp-serde 1.1.2][rmp-serde] | 1.51% | 0.77% | 73.85% | 79.79% | 77.04% | 9.19% |
| [ron 0.8.1][ron] | 0.11% | 0.06% | 27.04% | 57.77% | 60.50% | 4.45% |
| [savefile 0.16.5][savefile] | 71.83% | 23.36% | 100.00% | 96.35% | 92.11% | 91.14% |
| [serde_bare 0.5.0][serde_bare] | 3.00% | 3.52% | 100.00% | 96.35% | 92.11% | 92.88% |
| [serde_cbor 0.11.2][serde_cbor] | 0.54% | 0.35% | 45.72% | 68.87% | 72.84% | 7.43% |
| [serde_json 1.0.114][serde_json] | 0.22% | 0.17% | 22.91% | 54.17% | 57.34% | 4.61% |
| [simd-json 0.13.8][simd-json] | 0.36% | 0.20% | 22.91% | 54.17% | 57.34% | 4.40% |
| [speedy 0.8.7][speedy] | 71.78% | 54.07% | 100.00% | 96.35% | 92.11% | 98.60% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*58.40%\**</span> | <span title="unvalidated">*29.09%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*67.62%\**</span> | <span title="validated on-demand with panic">*52.74%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.61%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.43%\**</span> <span title="validated upfront with error">*3.18%\**</span> | <span title="unvalidated">*73.16%\**</span> <span title="validated upfront with error">*52.30%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*12.34%\**</span> | <span title="unvalidated">*83.40%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 179.96 µs | <span title="unvalidated">*1.2354 ms\**</span> | 1290592 | 396595 | 340267 | 5.0983 ms |
| [alkahest 0.1.5][alkahest] | 204.35 µs | † | 667570 | 325484 | 320452 | 3.8393 ms |
| [bincode 2.0.0-rc][bincode] | 252.27 µs | 1.9228 ms | 367413 | 221291 | 206273 | 2.4449 ms |
| [bincode 1.3.3][bincode1] | 554.94 µs | 1.6896 ms | 569975 | 240525 | 232423 | 2.8356 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 119.91 µs | 1.1844 ms | 327688 | 200947 | 182736 | 722.35 µs |
| [borsh 1.3.0][borsh] | 516.81 µs | 1.6897 ms | 446595 | 234236 | 210008 | 2.4886 ms |
| [bson 2.9.0][bson] | 2.8039 ms | 7.7980 ms | 1619653 | 502185 | 328399 | 4.5617 ms |
| [capnp 0.18.13][capnp] | 445.33 µs | † | 803896 | 335606 | 280851 | 3.6929 ms |
| [cbor4ii 0.3.2][cbor4ii] | 778.56 µs | 4.6209 ms | 1109831 | 344745 | 274514 | 3.7203 ms |
| [ciborium 0.2.2][ciborium] | 3.6318 ms | 9.1126 ms | 1109821 | 344751 | 274526 | 3.6143 ms |
| [databuf 0.5.0][databuf] | 292.23 µs | 1.6229 ms | 356311 | 213062 | 198488 | 2.4721 ms |
| [dlhn 0.1.6][dlhn] | 753.51 µs | 2.3485 ms | 366496 | 220600 | 205683 | 2.3579 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.1440 ms | † | 844168 | 345696 | 294015 | 3.8447 ms |
| [msgpacker 0.4.3][msgpacker] | 902.16 µs | 2.7421 ms | 391251 | 236877 | 220476 | 2.6269 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1214 ms | 3.7289 ms | 449745 | 252432 | 231110 | 2.7392 ms |
| [nanoserde 0.1.37][nanoserde] | 271.33 µs | 1.7524 ms | 567975 | 239930 | 232419 | 2.8648 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 619.55 µs | 1.8987 ms | 356311 | 212976 | 198524 | 2.4045 ms |
| [postcard 1.0.8][postcard] | 401.82 µs | 1.8851 ms | 367489 | 221913 | 207344 | 2.3834 ms |
| [pot 3.0.0][pot] | 2.2092 ms | 5.7990 ms | 599125 | 299158 | 247693 | 3.0667 ms |
| [prost 0.12.3][prost] | <span title="encode">*990.56 µs\**</span> <span title="populate + encode">*2.5319 ms\**</span> | 3.4355 ms | 596811 | 305319 | 269310 | 3.5186 ms |
| [rkyv 0.7.44][rkyv] | 293.28 µs | <span title="unvalidated">*1.2134 ms\**</span> <span title="validated upfront with error">*1.7070 ms\**</span> | 596952 | 253967 | 220706 | 2.8081 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3719 ms | 2.7591 ms | 424533 | 245214 | 226188 | 2.7579 ms |
| [ron 0.8.1][ron] | 7.9253 ms | 16.682 ms | 1465223 | 434935 | 343338 | 5.5824 ms |
| [savefile 0.16.5][savefile] | 207.10 µs | 1.7210 ms | 566991 | 239361 | 232010 | 2.9241 ms |
| [serde_bare 0.5.0][serde_bare] | 670.01 µs | 2.0660 ms | 356311 | 213062 | 198488 | 2.4676 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7971 ms | 4.6096 ms | 1109821 | 344751 | 274526 | 3.6329 ms |
| [serde_json 1.0.114][serde_json] | 3.6089 ms | 6.5199 ms | 1623191 | 466527 | 359623 | 5.7653 ms |
| [simd-json 0.13.8][simd-json] | 2.1387 ms | 4.3781 ms | 1623191 | 466527 | 359623 | 5.6145 ms |
| [speedy 0.8.7][speedy] | 256.27 µs | 1.5368 ms | 449595 | 234970 | 210361 | 2.5406 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*35.221 µs\**</span> | <span title="unvalidated">*36.076 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.7903 ns\**</span> | <span title="validated on-demand with panic">*4.4293 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.754 ns\**</span> | <span title="validated on-demand with error">*541.06 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4130 ns\**</span> <span title="validated upfront with error">*2.1482 ms\**</span> | <span title="unvalidated">*1.2926 µs\**</span> <span title="validated upfront with error">*2.0831 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.1690 ns\**</span> <span title="validated upfront with error">*465.38 µs\**</span> | <span title="unvalidated">*224.45 ns\**</span> <span title="validated upfront with error">*458.51 µs\**</span> | 852.40 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 66.63% | <span title="unvalidated">*95.87%\**</span> | 25.39% | 50.67% | 53.70% | 14.17% |
| [alkahest 0.1.5][alkahest] | 58.68% | † | 49.09% | 61.74% | 57.02% | 18.81% |
| [bincode 2.0.0-rc][bincode] | 47.53% | 61.60% | 89.19% | 90.81% | 88.59% | 29.54% |
| [bincode 1.3.3][bincode1] | 21.61% | 70.10% | 57.49% | 83.55% | 78.62% | 25.47% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 23.20% | 70.10% | 73.37% | 85.79% | 87.01% | 29.03% |
| [bson 2.9.0][bson] | 4.28% | 15.19% | 20.23% | 40.01% | 55.64% | 15.83% |
| [capnp 0.18.13][capnp] | 26.93% | † | 40.76% | 59.88% | 65.07% | 19.56% |
| [cbor4ii 0.3.2][cbor4ii] | 15.40% | 25.63% | 29.53% | 58.29% | 66.57% | 19.42% |
| [ciborium 0.2.2][ciborium] | 3.30% | 13.00% | 29.53% | 58.29% | 66.56% | 19.99% |
| [databuf 0.5.0][databuf] | 41.03% | 72.98% | 91.97% | 94.31% | 92.06% | 29.22% |
| [dlhn 0.1.6][dlhn] | 15.91% | 50.43% | 89.41% | 91.09% | 88.84% | 30.64% |
| [flatbuffers 23.5.26][flatbuffers] | 3.81% | † | 38.82% | 58.13% | 62.15% | 18.79% |
| [msgpacker 0.4.3][msgpacker] | 13.29% | 43.19% | 83.75% | 84.83% | 82.88% | 27.50% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.34% | 31.76% | 72.86% | 79.60% | 79.07% | 26.37% |
| [nanoserde 0.1.37][nanoserde] | 44.19% | 67.59% | 57.69% | 83.75% | 78.62% | 25.21% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 19.35% | 62.38% | 91.97% | 94.35% | 92.05% | 30.04% |
| [postcard 1.0.8][postcard] | 29.84% | 62.83% | 89.17% | 90.55% | 88.13% | 30.31% |
| [pot 3.0.0][pot] | 5.43% | 20.42% | 54.69% | 67.17% | 73.78% | 23.55% |
| [prost 0.12.3][prost] | <span title="encode">*12.11%\**</span> <span title="populate + encode">*4.74%\**</span> | 34.48% | 54.91% | 65.82% | 67.85% | 20.53% |
| [rkyv 0.7.44][rkyv] | 40.89% | <span title="unvalidated">*97.61%\**</span> <span title="validated upfront with error">*69.38%\**</span> | 54.89% | 79.12% | 82.80% | 25.72% |
| [rmp-serde 1.1.2][rmp-serde] | 8.74% | 42.93% | 77.19% | 81.95% | 80.79% | 26.19% |
| [ron 0.8.1][ron] | 1.51% | 7.10% | 22.36% | 46.20% | 53.22% | 12.94% |
| [savefile 0.16.5][savefile] | 57.90% | 68.82% | 57.79% | 83.95% | 78.76% | 24.70% |
| [serde_bare 0.5.0][serde_bare] | 17.90% | 57.33% | 91.97% | 94.31% | 92.06% | 29.27% |
| [serde_cbor 0.11.2][serde_cbor] | 6.67% | 25.69% | 29.53% | 58.29% | 66.56% | 19.88% |
| [serde_json 1.0.114][serde_json] | 3.32% | 18.17% | 20.19% | 43.07% | 50.81% | 12.53% |
| [simd-json 0.13.8][simd-json] | 5.61% | 27.05% | 20.19% | 43.07% | 50.81% | 12.87% |
| [speedy 0.8.7][speedy] | 46.79% | 77.07% | 72.89% | 85.52% | 86.87% | 28.43% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.62%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*65.30%\**</span> | <span title="validated on-demand with panic">*5.07%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.56%\**</span> | <span title="validated on-demand with error">*41.48%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*48.45%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.36%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 447.36 µs | <span title="unvalidated">*2.1667 ms\**</span> | 2984682 | 1408209 | 1273800 | 15.540 ms |
| [alkahest 0.1.5][alkahest] | 673.66 µs | † | 1863391 | 1234113 | 1202345 | 11.837 ms |
| [bincode 2.0.0-rc][bincode] | 698.85 µs | 3.3882 ms | 1372381 | 1091486 | 1037296 | 9.0337 ms |
| [bincode 1.3.3][bincode1] | 3.4282 ms | 3.7820 ms | 1811011 | 1115281 | 1025627 | 9.7470 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 662.75 µs | 2.2214 ms | 948499 | 857321 | 837658 | 3.0146 ms |
| [borsh 1.3.0][borsh] | 2.7213 ms | 2.7432 ms | 1486162 | 1082357 | 1013550 | 10.032 ms |
| [bson 2.9.0][bson] | 20.181 ms | 41.069 ms | 10030880 | 2833079 | 1600859 | 26.483 ms |
| [capnp 0.18.13][capnp] | 2.2015 ms | † | 2664040 | 1511895 | 1212087 | 14.340 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.0006 ms | 16.594 ms | 5878791 | 1655835 | 1431390 | 21.096 ms |
| [ciborium 0.2.2][ciborium] | 21.717 ms | 45.395 ms | 5878653 | 1655791 | 1431560 | 22.459 ms |
| [databuf 0.5.0][databuf] | 1.4110 ms | 3.3732 ms | 1288257 | 1037579 | 984337 | 8.7712 ms |
| [dlhn 0.1.6][dlhn] | 4.6732 ms | 6.8693 ms | 1279599 | 1052061 | 1021161 | 8.7880 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.1359 ms | † | 2273740 | 1408408 | 1235566 | 13.339 ms |
| [msgpacker 0.4.3][msgpacker] | 1.8083 ms | 4.3621 ms | 1424043 | 1128758 | 1110156 | 9.1921 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.233 ms | 14.694 ms | 1728519 | 1247642 | 1233323 | 12.516 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2031 ms | 2.7735 ms | 1770477 | 1108304 | 1029947 | 9.8488 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.9702 ms | 2.9079 ms | 1288257 | 1039269 | 986510 | 8.6346 ms |
| [postcard 1.0.8][postcard] | 1.7003 ms | 3.7470 ms | 1279599 | 1058243 | 1016738 | 8.4765 ms |
| [pot 3.0.0][pot] | 13.307 ms | 30.808 ms | 2544810 | 1447453 | 1268390 | 15.311 ms |
| [prost 0.12.3][prost] | <span title="encode">*3.9077 ms\**</span> <span title="populate + encode">*7.8293 ms\**</span> | 9.1852 ms | 1818378 | 1307777 | 1266311 | 11.494 ms |
| [rkyv 0.7.44][rkyv] | 1.0533 ms | <span title="unvalidated">*2.0785 ms\**</span> <span title="validated upfront with error">*2.6844 ms\**</span> | 2029080 | 1335117 | 1158855 | 11.561 ms |
| [rmp-serde 1.1.2][rmp-serde] | 9.7939 ms | 10.873 ms | 1703813 | 1231892 | 1200208 | 10.721 ms |
| [ron 0.8.1][ron] | 35.727 ms | 93.064 ms | 8476284 | 2181196 | 1783971 | 34.246 ms |
| [savefile 0.16.5][savefile] | 984.44 µs | 2.5246 ms | 1750226 | 1101682 | 1027827 | 9.9840 ms |
| [serde_bare 0.5.0][serde_bare] | 4.5822 ms | 4.2869 ms | 1288257 | 1037597 | 984356 | 9.0901 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.2124 ms | 20.284 ms | 5878653 | 1655791 | 1431560 | 21.840 ms |
| [serde_json 1.0.114][serde_json] | 20.082 ms | 29.056 ms | 9175594 | 2334253 | 1800713 | 35.092 ms |
| [simd-json 0.13.8][simd-json] | 10.929 ms | 25.395 ms | 9175594 | 2334253 | 1800713 | 35.579 ms |
| [speedy 0.8.7][speedy] | 711.56 µs | 2.4121 ms | 1546963 | 1093532 | 1013443 | 10.056 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.708 µs\**</span> | <span title="unvalidated">*62.472 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.7516 ns\**</span> | <span title="validated on-demand with panic">*588.16 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*72.406 ns\**</span> | <span title="validated on-demand with error">*668.19 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.3251 ns\**</span> <span title="validated upfront with error">*4.6136 ms\**</span> | <span title="unvalidated">*2.5447 µs\**</span> <span title="validated upfront with error">*4.6524 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.1861 ns\**</span> <span title="validated upfront with error">*588.77 µs\**</span> | <span title="unvalidated">*343.33 ns\**</span> <span title="validated upfront with error">*594.20 µs\**</span> | 470.69 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.93%\**</span> | 31.78% | 60.88% | 65.76% | 19.40% |
| [alkahest 0.1.5][alkahest] | 66.41% | † | 50.90% | 69.47% | 69.67% | 25.47% |
| [bincode 2.0.0-rc][bincode] | 64.01% | 61.35% | 69.11% | 78.55% | 80.75% | 33.37% |
| [bincode 1.3.3][bincode1] | 13.05% | 54.96% | 52.37% | 76.87% | 81.67% | 30.93% |
| [bitcode 0.6.0-alpha.2][bitcode] | 67.50% | 93.57% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 16.44% | 75.77% | 63.82% | 79.21% | 82.65% | 30.05% |
| [bson 2.9.0][bson] | 2.22% | 5.06% | 9.46% | 30.26% | 52.33% | 11.38% |
| [capnp 0.18.13][capnp] | 20.32% | † | 35.60% | 56.71% | 69.11% | 21.02% |
| [cbor4ii 0.3.2][cbor4ii] | 11.18% | 12.53% | 16.13% | 51.78% | 58.52% | 14.29% |
| [ciborium 0.2.2][ciborium] | 2.06% | 4.58% | 16.13% | 51.78% | 58.51% | 13.42% |
| [databuf 0.5.0][databuf] | 31.71% | 61.62% | 73.63% | 82.63% | 85.10% | 34.37% |
| [dlhn 0.1.6][dlhn] | 9.57% | 30.26% | 74.12% | 81.49% | 82.03% | 34.30% |
| [flatbuffers 23.5.26][flatbuffers] | 8.71% | † | 41.72% | 60.87% | 67.80% | 22.60% |
| [msgpacker 0.4.3][msgpacker] | 24.74% | 47.65% | 66.61% | 75.95% | 75.45% | 32.80% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.48% | 14.15% | 54.87% | 68.72% | 67.92% | 24.09% |
| [nanoserde 0.1.37][nanoserde] | 37.18% | 74.94% | 53.57% | 77.35% | 81.33% | 30.61% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 15.06% | 71.48% | 73.63% | 82.49% | 84.91% | 34.91% |
| [postcard 1.0.8][postcard] | 26.31% | 55.47% | 74.12% | 81.01% | 82.39% | 35.56% |
| [pot 3.0.0][pot] | 3.36% | 6.75% | 37.27% | 59.23% | 66.04% | 19.69% |
| [prost 0.12.3][prost] | <span title="encode">*11.45%\**</span> <span title="populate + encode">*5.71%\**</span> | 22.63% | 52.16% | 65.56% | 66.15% | 26.23% |
| [rkyv 0.7.44][rkyv] | 42.47% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.43%\**</span> | 46.75% | 64.21% | 72.28% | 26.08% |
| [rmp-serde 1.1.2][rmp-serde] | 4.57% | 19.12% | 55.67% | 69.59% | 69.79% | 28.12% |
| [ron 0.8.1][ron] | 1.25% | 2.23% | 11.19% | 39.31% | 46.95% | 8.80% |
| [savefile 0.16.5][savefile] | 45.44% | 82.33% | 54.19% | 77.82% | 81.50% | 30.19% |
| [serde_bare 0.5.0][serde_bare] | 9.76% | 48.48% | 73.63% | 82.63% | 85.10% | 33.16% |
| [serde_cbor 0.11.2][serde_cbor] | 4.86% | 10.25% | 16.13% | 51.78% | 58.51% | 13.80% |
| [serde_json 1.0.114][serde_json] | 2.23% | 7.15% | 10.34% | 36.73% | 46.52% | 8.59% |
| [simd-json 0.13.8][simd-json] | 4.09% | 8.18% | 10.34% | 36.73% | 46.52% | 8.47% |
| [speedy 0.8.7][speedy] | 62.87% | 86.17% | 61.31% | 78.40% | 82.65% | 29.98% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.55%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*67.72%\**</span> | <span title="validated on-demand with panic">*58.37%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.64%\**</span> | <span title="validated on-demand with error">*51.38%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*51.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.49%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
