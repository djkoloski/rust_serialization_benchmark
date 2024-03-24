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

## Last updated: 2024-3-24 17:54:50

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 402.20 µs | <span title="unvalidated">*1.4585 ms\**</span> | 1705800 | 520075 | 413560 | 6.9289 ms |
| [alkahest 0.1.5][alkahest] | 188.40 µs | † | 1045784 | 454157 | 389424 | 6.1893 ms |
| [bincode 2.0.0-rc][bincode] | 299.03 µs | 2.4415 ms | 741295 | 303944 | 257153 | 4.0161 ms |
| [bincode 1.3.3][bincode1] | 524.76 µs | 2.0052 ms | 1045784 | 373127 | 311761 | 4.9241 ms |
| [bitcode 0.6.0][bitcode] | 141.36 µs | 1.4858 ms | 703710 | 288826 | 229755 | 2.4492 ms |
| [borsh 1.3.0][borsh] | 541.86 µs | 2.1778 ms | 885780 | 362204 | 286514 | 4.6678 ms |
| [bson 2.9.0][bson] | 2.2504 ms | 7.1773 ms | 1924682 | 532821 | 376270 | 6.1304 ms |
| [capnp 0.18.13][capnp] | 586.47 µs | † | 1443216 | 513986 | 428649 | 6.9242 ms |
| [cbor4ii 0.3.2][cbor4ii] | 894.65 µs | 4.9096 ms | 1407835 | 403440 | 324081 | 5.1635 ms |
| [ciborium 0.2.2][ciborium] | 3.9219 ms | 10.793 ms | 1407835 | 403440 | 324081 | 5.1719 ms |
| [databuf 0.5.0][databuf] | 263.08 µs | 1.9838 ms | 765778 | 311715 | 264630 | 4.2046 ms |
| [dlhn 0.1.6][dlhn] | 658.02 µs | 2.6871 ms | 724953 | 301446 | 253629 | 3.8698 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.3904 ms | † | 1276368 | 468539 | 388832 | 5.6651 ms |
| [msgpacker 0.4.3][msgpacker] | 1.0800 ms | 2.5680 ms | 764996 | 315291 | 264898 | 4.2673 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5657 ms | 3.9560 ms | 818669 | 332556 | 285514 | 4.8274 ms |
| [nanoserde 0.1.37][nanoserde] | 376.95 µs | 2.0646 ms | 1045784 | 373127 | 311761 | 4.6179 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 693.15 µs | 2.3072 ms | 765778 | 311743 | 264518 | 4.2423 ms |
| [postcard 1.0.8][postcard] | 417.31 µs | 2.2381 ms | 724953 | 302399 | 253747 | 3.9041 ms |
| [pot 3.0.0][pot] | 2.3543 ms | 6.5988 ms | 971922 | 372513 | 304122 | 5.0745 ms |
| [prost 0.12.3][prost] | <span title="encode">*798.43 µs\**</span> <span title="populate + encode">*2.3366 ms\**</span> | 3.3978 ms | 884628 | 363130 | 315494 | 5.1077 ms |
| [rkyv 0.7.44][rkyv] | 216.56 µs | <span title="unvalidated">*1.4471 ms\**</span> <span title="validated upfront with error">*1.9729 ms\**</span> | 1011488 | 383862 | 333545 | 5.3336 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3698 ms | 3.5296 ms | 784997 | 325384 | 278219 | 4.2000 ms |
| [ron 0.8.1][ron] | 14.163 ms | 16.999 ms | 1607459 | 449158 | 349713 | 5.7556 ms |
| [savefile 0.16.5][savefile] | 203.43 µs | 2.1111 ms | 1045800 | 373139 | 311755 | 4.6139 ms |
| [serde_bare 0.5.0][serde_bare] | 656.64 µs | 2.1302 ms | 765778 | 311715 | 264630 | 3.9022 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7340 ms | 5.0729 ms | 1407835 | 403440 | 324081 | 4.9047 ms |
| [serde_json 1.0.114][serde_json] | 4.0556 ms | 5.7319 ms | 1827461 | 470560 | 361090 | 5.6237 ms |
| [simd-json 0.13.9][simd-json] | 2.0659 ms | 4.6266 ms | 1827461 | 470560 | 361090 | 5.7940 ms |
| [speedy 0.8.7][speedy] | 217.99 µs | 1.7533 ms | 885780 | 362204 | 286514 | 4.3146 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*21.926 µs\**</span> | <span title="unvalidated">*38.587 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8549 ns\**</span> | <span title="validated on-demand with panic">*25.056 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.897 ns\**</span> | <span title="validated on-demand with error">*171.44 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4739 ns\**</span> <span title="validated upfront with error">*1.8695 ms\**</span> | <span title="unvalidated">*51.896 µs\**</span> <span title="validated upfront with error">*1.9809 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2362 ns\**</span> <span title="validated upfront with error">*516.81 µs\**</span> | <span title="unvalidated">*10.689 µs\**</span> <span title="validated upfront with error">*532.82 µs\**</span> | 9.7510 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 35.15% | <span title="unvalidated">*99.22%\**</span> | 41.25% | 55.54% | 55.56% | 35.35% |
| [alkahest 0.1.5][alkahest] | 75.03% | † | 67.29% | 63.60% | 59.00% | 39.57% |
| [bincode 2.0.0-rc][bincode] | 47.27% | 59.27% | 94.93% | 95.03% | 89.35% | 60.98% |
| [bincode 1.3.3][bincode1] | 26.94% | 72.17% | 67.29% | 77.41% | 73.70% | 49.74% |
| [bitcode 0.6.0][bitcode] | 100.00% | 97.40% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 26.09% | 66.45% | 79.45% | 79.74% | 80.19% | 52.47% |
| [bson 2.9.0][bson] | 6.28% | 20.16% | 36.56% | 54.21% | 61.06% | 39.95% |
| [capnp 0.18.13][capnp] | 24.10% | † | 48.76% | 56.19% | 53.60% | 35.37% |
| [cbor4ii 0.3.2][cbor4ii] | 15.80% | 29.47% | 49.99% | 71.59% | 70.89% | 47.43% |
| [ciborium 0.2.2][ciborium] | 3.60% | 13.41% | 49.99% | 71.59% | 70.89% | 47.36% |
| [databuf 0.5.0][databuf] | 53.73% | 72.95% | 91.89% | 92.66% | 86.82% | 58.25% |
| [dlhn 0.1.6][dlhn] | 21.48% | 53.85% | 97.07% | 95.81% | 90.59% | 63.29% |
| [flatbuffers 23.5.26][flatbuffers] | 10.17% | † | 55.13% | 61.64% | 59.09% | 43.23% |
| [msgpacker 0.4.3][msgpacker] | 13.09% | 56.35% | 91.99% | 91.61% | 86.73% | 57.39% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.54% | 36.58% | 85.96% | 86.85% | 80.47% | 50.74% |
| [nanoserde 0.1.37][nanoserde] | 37.50% | 70.09% | 67.29% | 77.41% | 73.70% | 53.04% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.39% | 62.72% | 91.89% | 92.65% | 86.86% | 57.73% |
| [postcard 1.0.8][postcard] | 33.87% | 64.66% | 97.07% | 95.51% | 90.54% | 62.73% |
| [pot 3.0.0][pot] | 6.00% | 21.93% | 72.40% | 77.53% | 75.55% | 48.26% |
| [prost 0.12.3][prost] | <span title="encode">*17.70%\**</span> <span title="populate + encode">*6.05%\**</span> | 42.59% | 79.55% | 79.54% | 72.82% | 47.95% |
| [rkyv 0.7.44][rkyv] | 65.28% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.35%\**</span> | 69.57% | 75.24% | 68.88% | 45.92% |
| [rmp-serde 1.1.2][rmp-serde] | 10.32% | 41.00% | 89.64% | 88.76% | 82.58% | 58.31% |
| [ron 0.8.1][ron] | 1.00% | 8.51% | 43.78% | 64.30% | 65.70% | 42.55% |
| [savefile 0.16.5][savefile] | 69.49% | 68.55% | 67.29% | 77.40% | 73.70% | 53.08% |
| [serde_bare 0.5.0][serde_bare] | 21.53% | 67.93% | 91.89% | 92.66% | 86.82% | 62.76% |
| [serde_cbor 0.11.2][serde_cbor] | 8.15% | 28.53% | 49.99% | 71.59% | 70.89% | 49.94% |
| [serde_json 1.0.114][serde_json] | 3.49% | 25.25% | 38.51% | 61.38% | 63.63% | 43.55% |
| [simd-json 0.13.9][simd-json] | 6.84% | 31.28% | 38.51% | 61.38% | 63.63% | 42.27% |
| [speedy 0.8.7][speedy] | 64.85% | 82.54% | 79.45% | 79.74% | 80.19% | 56.77% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*27.70%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.65%\**</span> | <span title="validated on-demand with panic">*42.66%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.65%\**</span> | <span title="validated on-demand with error">*6.23%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.60%\**</span> <span title="validated upfront with error">*0.54%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.01%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 259.28 µs | <span title="unvalidated">*258.60 µs\**</span> | 6000024 | 5378513 | 5345891 | 7.7072 ms |
| [alkahest 0.1.5][alkahest] | 148.07 µs | † | 6000008 | 5378500 | 5345890 | 8.0329 ms |
| [bincode 2.0.0-rc][bincode] | 2.3953 ms | 1.4090 ms | 6000005 | 5378497 | 5345897 | 7.5455 ms |
| [bincode 1.3.3][bincode1] | 4.9982 ms | 4.3476 ms | 6000008 | 5378500 | 5345890 | 7.9697 ms |
| [bitcode 0.6.0][bitcode] | 1.3952 ms | 598.09 µs | 6000006 | 5182295 | 4923880 | 12.857 ms |
| [borsh 1.3.0][borsh] | 6.1143 ms | 4.2233 ms | 6000004 | 5378496 | 5345889 | 7.4898 ms |
| [bson 2.9.0][bson] | 44.389 ms | 78.748 ms | 23013911 | 9212089 | 7497811 | 122.20 ms |
| [capnp 0.18.13][capnp] | 5.8670 ms | † | 14000088 | 7130367 | 6051062 | 94.323 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.505 ms | 47.279 ms | 13125016 | 7524114 | 6757967 | 106.07 ms |
| [ciborium 0.2.2][ciborium] | 66.274 ms | 106.76 ms | 13122324 | 7524660 | 6759658 | 100.29 ms |
| [databuf 0.5.0][databuf] | 2.4013 ms | 5.2884 ms | 6000003 | 5378495 | 5345900 | 7.8802 ms |
| [dlhn 0.1.6][dlhn] | 6.1365 ms | 5.7218 ms | 6000003 | 5378495 | 5345900 | 7.6836 ms |
| [flatbuffers 23.5.26][flatbuffers] | 642.25 µs | † | 6000024 | 5378434 | 5345910 | 7.7720 ms |
| [msgpacker 0.4.3][msgpacker] | 20.115 ms | 8.5789 ms | 7500005 | 6058442 | 6014337 | 9.7036 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 123.22 ms | 26.672 ms | 8125037 | 6493484 | 6386940 | 82.218 ms |
| [nanoserde 0.1.37][nanoserde] | 1.1673 ms | 1.4248 ms | 6000008 | 5378500 | 5345890 | 7.9709 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 5.1486 ms | 4.0049 ms | 6000004 | 5378496 | 5345889 | 7.5053 ms |
| [postcard 1.0.8][postcard] | 510.12 µs | 1.1867 ms | 6000003 | 5378495 | 5345900 | 7.8237 ms |
| [pot 3.0.0][pot] | 38.974 ms | 71.922 ms | 10122342 | 6814618 | 6852251 | 95.060 ms |
| [prost 0.12.3][prost] | <span title="encode">*7.6328 ms\**</span> <span title="populate + encode">*8.6118 ms\**</span> | 14.242 ms | 8750000 | 6665735 | 6421871 | 85.880 ms |
| [rkyv 0.7.44][rkyv] | 187.41 µs | <span title="unvalidated">*149.14 µs\**</span> <span title="validated upfront with error">*149.06 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.8768 ms |
| [rmp-serde 1.1.2][rmp-serde] | 13.421 ms | 18.687 ms | 8125006 | 6494876 | 6391037 | 82.640 ms |
| [ron 0.8.1][ron] | 174.07 ms | 256.04 ms | 22192885 | 8970395 | 8138755 | 160.35 ms |
| [savefile 0.16.5][savefile] | 263.34 µs | 262.15 µs | 6000024 | 5378518 | 5345893 | 7.5323 ms |
| [serde_bare 0.5.0][serde_bare] | 6.3807 ms | 4.0690 ms | 6000003 | 5378495 | 5345900 | 7.6869 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.481 ms | 45.799 ms | 13122324 | 7524660 | 6759658 | 99.860 ms |
| [serde_json 1.0.114][serde_json] | 92.341 ms | 88.072 ms | 26192883 | 9566084 | 8586741 | 164.27 ms |
| [simd-json 0.13.9][simd-json] | 54.612 ms | 74.604 ms | 26192883 | 9566084 | 8586741 | 164.75 ms |
| [speedy 0.8.7][speedy] | 259.13 µs | 259.71 µs | 6000004 | 5378496 | 5345889 | 7.4997 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1648 ns\**</span> | <span title="unvalidated">*140.75 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8557 ns\**</span> | <span title="validated on-demand with panic">*77.295 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*106.73 ns\**</span> | <span title="validated on-demand with error">*2.1383 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4727 ns\**</span> <span title="validated upfront with error">*37.357 ns\**</span> | <span title="unvalidated">*54.464 µs\**</span> <span title="validated upfront with error">*77.366 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2408 ns\**</span> <span title="validated upfront with error">*9.9137 ns\**</span> | <span title="unvalidated">*48.403 µs\**</span> <span title="validated upfront with error">*77.354 µs\**</span> | 100.41 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 57.11% | <span title="unvalidated">*57.64%\**</span> | 100.00% | 96.35% | 92.11% | 97.18% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 93.24% |
| [bincode 2.0.0-rc][bincode] | 6.18% | 10.58% | 100.00% | 96.35% | 92.11% | 99.26% |
| [bincode 1.3.3][bincode1] | 2.96% | 3.43% | 100.00% | 96.35% | 92.11% | 93.98% |
| [bitcode 0.6.0][bitcode] | 10.61% | 24.92% | 100.00% | 100.00% | 100.00% | 58.25% |
| [borsh 1.3.0][borsh] | 2.42% | 3.53% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bson 2.9.0][bson] | 0.33% | 0.19% | 26.07% | 56.26% | 65.67% | 6.13% |
| [capnp 0.18.13][capnp] | 2.52% | † | 42.86% | 72.68% | 81.37% | 7.94% |
| [cbor4ii 0.3.2][cbor4ii] | 1.41% | 0.32% | 45.71% | 68.88% | 72.86% | 7.06% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.14% | 45.72% | 68.87% | 72.84% | 7.47% |
| [databuf 0.5.0][databuf] | 6.17% | 2.82% | 100.00% | 96.35% | 92.11% | 95.05% |
| [dlhn 0.1.6][dlhn] | 2.41% | 2.61% | 100.00% | 96.35% | 92.11% | 97.48% |
| [flatbuffers 23.5.26][flatbuffers] | 23.05% | † | 100.00% | 96.35% | 92.11% | 96.37% |
| [msgpacker 0.4.3][msgpacker] | 0.74% | 1.74% | 80.00% | 85.54% | 81.87% | 77.19% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.56% | 73.85% | 79.81% | 77.09% | 9.11% |
| [nanoserde 0.1.37][nanoserde] | 12.68% | 10.46% | 100.00% | 96.35% | 92.11% | 93.96% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.88% | 3.72% | 100.00% | 96.35% | 92.11% | 99.79% |
| [postcard 1.0.8][postcard] | 29.03% | 12.56% | 100.00% | 96.35% | 92.11% | 95.73% |
| [pot 3.0.0][pot] | 0.38% | 0.21% | 59.27% | 76.05% | 71.86% | 7.88% |
| [prost 0.12.3][prost] | <span title="encode">*1.94%\**</span> <span title="populate + encode">*1.72%\**</span> | 1.05% | 68.57% | 77.75% | 76.67% | 8.72% |
| [rkyv 0.7.44][rkyv] | 79.01% | <span title="unvalidated">*99.95%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 95.09% |
| [rmp-serde 1.1.2][rmp-serde] | 1.10% | 0.80% | 73.85% | 79.79% | 77.04% | 9.06% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 4.67% |
| [savefile 0.16.5][savefile] | 56.23% | 56.86% | 100.00% | 96.35% | 92.11% | 99.44% |
| [serde_bare 0.5.0][serde_bare] | 2.32% | 3.66% | 100.00% | 96.35% | 92.11% | 97.44% |
| [serde_cbor 0.11.2][serde_cbor] | 0.42% | 0.33% | 45.72% | 68.87% | 72.84% | 7.50% |
| [serde_json 1.0.114][serde_json] | 0.16% | 0.17% | 22.91% | 54.17% | 57.34% | 4.56% |
| [simd-json 0.13.9][simd-json] | 0.27% | 0.20% | 22.91% | 54.17% | 57.34% | 4.55% |
| [speedy 0.8.7][speedy] | 57.14% | 57.39% | 100.00% | 96.35% | 92.11% | 99.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.32%\**</span> | <span title="unvalidated">*34.39%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.86%\**</span> | <span title="validated on-demand with panic">*62.62%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.16%\**</span> | <span title="validated on-demand with error">*2.26%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.18%\**</span> <span title="validated upfront with error">*3.32%\**</span> | <span title="unvalidated">*88.87%\**</span> <span title="validated upfront with error">*62.56%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*12.52%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*62.57%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 189.71 µs | <span title="unvalidated">*1.3119 ms\**</span> | 1290592 | 396664 | 340325 | 5.1026 ms |
| [alkahest 0.1.5][alkahest] | 217.86 µs | † | 667570 | 325484 | 320452 | 3.9917 ms |
| [bincode 2.0.0-rc][bincode] | 298.89 µs | 2.0852 ms | 367413 | 221291 | 206273 | 2.5635 ms |
| [bincode 1.3.3][bincode1] | 568.46 µs | 1.8150 ms | 569975 | 240525 | 232423 | 2.9721 ms |
| [bitcode 0.6.0][bitcode] | 125.83 µs | 1.2638 ms | 327688 | 200947 | 182736 | 769.81 µs |
| [borsh 1.3.0][borsh] | 524.44 µs | 1.8416 ms | 446595 | 234236 | 210008 | 2.5474 ms |
| [bson 2.9.0][bson] | 2.8343 ms | 8.3027 ms | 1619653 | 502185 | 328399 | 4.9487 ms |
| [capnp 0.18.13][capnp] | 527.34 µs | † | 803896 | 335606 | 280851 | 3.9918 ms |
| [cbor4ii 0.3.2][cbor4ii] | 786.11 µs | 4.5650 ms | 1109831 | 344745 | 274514 | 3.9063 ms |
| [ciborium 0.2.2][ciborium] | 3.6557 ms | 9.6592 ms | 1109821 | 344751 | 274526 | 3.9040 ms |
| [databuf 0.5.0][databuf] | 321.17 µs | 1.7465 ms | 356311 | 213062 | 198488 | 2.4647 ms |
| [dlhn 0.1.6][dlhn] | 727.04 µs | 2.5286 ms | 366496 | 220600 | 205683 | 2.5525 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.2930 ms | † | 844168 | 345696 | 294015 | 3.9109 ms |
| [msgpacker 0.4.3][msgpacker] | 871.63 µs | 2.8348 ms | 391251 | 236877 | 220476 | 2.7209 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3965 ms | 3.8980 ms | 449745 | 252432 | 231110 | 2.8269 ms |
| [nanoserde 0.1.37][nanoserde] | 304.40 µs | 1.8954 ms | 567975 | 239930 | 232419 | 2.9449 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 674.01 µs | 1.9993 ms | 356311 | 212976 | 198524 | 2.4504 ms |
| [postcard 1.0.8][postcard] | 426.89 µs | 1.9521 ms | 367489 | 221913 | 207344 | 2.5510 ms |
| [pot 3.0.0][pot] | 2.2687 ms | 6.0758 ms | 599125 | 299158 | 247693 | 3.2789 ms |
| [prost 0.12.3][prost] | <span title="encode">*1.0762 ms\**</span> <span title="populate + encode">*2.7458 ms\**</span> | 3.4272 ms | 596811 | 305319 | 269310 | 3.5389 ms |
| [rkyv 0.7.44][rkyv] | 300.63 µs | <span title="unvalidated">*1.2598 ms\**</span> <span title="validated upfront with error">*1.7713 ms\**</span> | 596952 | 253967 | 220706 | 2.7461 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.4413 ms | 2.9470 ms | 424533 | 245214 | 226188 | 2.7437 ms |
| [ron 0.8.1][ron] | 8.1757 ms | 16.953 ms | 1465223 | 434935 | 343338 | 5.9215 ms |
| [savefile 0.16.5][savefile] | 219.00 µs | 1.8357 ms | 566991 | 239361 | 232010 | 2.9490 ms |
| [serde_bare 0.5.0][serde_bare] | 717.67 µs | 2.2308 ms | 356311 | 213062 | 198488 | 2.4603 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.6954 ms | 4.9152 ms | 1109821 | 344751 | 274526 | 3.9182 ms |
| [serde_json 1.0.114][serde_json] | 3.8048 ms | 6.8793 ms | 1623191 | 466527 | 359623 | 6.1022 ms |
| [simd-json 0.13.9][simd-json] | 2.2103 ms | 4.7088 ms | 1623191 | 466527 | 359623 | 6.1095 ms |
| [speedy 0.8.7][speedy] | 274.43 µs | 1.6364 ms | 449595 | 234970 | 210361 | 2.5494 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*36.808 µs\**</span> | <span title="unvalidated">*37.321 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8545 ns\**</span> | <span title="validated on-demand with panic">*4.6408 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.947 ns\**</span> | <span title="validated on-demand with error">*459.97 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4743 ns\**</span> <span title="validated upfront with error">*2.2279 ms\**</span> | <span title="unvalidated">*1.3676 µs\**</span> <span title="validated upfront with error">*2.2282 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2414 ns\**</span> <span title="validated upfront with error">*511.87 µs\**</span> | <span title="unvalidated">*163.22 ns\**</span> <span title="validated upfront with error">*515.84 µs\**</span> | 881.85 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 66.33% | <span title="unvalidated">*96.03%\**</span> | 25.39% | 50.66% | 53.69% | 15.09% |
| [alkahest 0.1.5][alkahest] | 57.76% | † | 49.09% | 61.74% | 57.02% | 19.29% |
| [bincode 2.0.0-rc][bincode] | 42.10% | 60.42% | 89.19% | 90.81% | 88.59% | 30.03% |
| [bincode 1.3.3][bincode1] | 22.14% | 69.41% | 57.49% | 83.55% | 78.62% | 25.90% |
| [bitcode 0.6.0][bitcode] | 100.00% | 99.68% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 23.99% | 68.41% | 73.37% | 85.79% | 87.01% | 30.22% |
| [bson 2.9.0][bson] | 4.44% | 15.17% | 20.23% | 40.01% | 55.64% | 15.56% |
| [capnp 0.18.13][capnp] | 23.86% | † | 40.76% | 59.88% | 65.07% | 19.28% |
| [cbor4ii 0.3.2][cbor4ii] | 16.01% | 27.60% | 29.53% | 58.29% | 66.57% | 19.71% |
| [ciborium 0.2.2][ciborium] | 3.44% | 13.04% | 29.53% | 58.29% | 66.56% | 19.72% |
| [databuf 0.5.0][databuf] | 39.18% | 72.13% | 91.97% | 94.31% | 92.06% | 31.23% |
| [dlhn 0.1.6][dlhn] | 17.31% | 49.82% | 89.41% | 91.09% | 88.84% | 30.16% |
| [flatbuffers 23.5.26][flatbuffers] | 3.82% | † | 38.82% | 58.13% | 62.15% | 19.68% |
| [msgpacker 0.4.3][msgpacker] | 14.44% | 44.44% | 83.75% | 84.83% | 82.88% | 28.29% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.33% | 32.32% | 72.86% | 79.60% | 79.07% | 27.23% |
| [nanoserde 0.1.37][nanoserde] | 41.34% | 66.47% | 57.69% | 83.75% | 78.62% | 26.14% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 18.67% | 63.01% | 91.97% | 94.35% | 92.05% | 31.42% |
| [postcard 1.0.8][postcard] | 29.48% | 64.54% | 89.17% | 90.55% | 88.13% | 30.18% |
| [pot 3.0.0][pot] | 5.55% | 20.73% | 54.69% | 67.17% | 73.78% | 23.48% |
| [prost 0.12.3][prost] | <span title="encode">*11.69%\**</span> <span title="populate + encode">*4.58%\**</span> | 36.76% | 54.91% | 65.82% | 67.85% | 21.75% |
| [rkyv 0.7.44][rkyv] | 41.86% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.12%\**</span> | 54.89% | 79.12% | 82.80% | 28.03% |
| [rmp-serde 1.1.2][rmp-serde] | 8.73% | 42.75% | 77.19% | 81.95% | 80.79% | 28.06% |
| [ron 0.8.1][ron] | 1.54% | 7.43% | 22.36% | 46.20% | 53.22% | 13.00% |
| [savefile 0.16.5][savefile] | 57.46% | 68.63% | 57.79% | 83.95% | 78.76% | 26.10% |
| [serde_bare 0.5.0][serde_bare] | 17.53% | 56.47% | 91.97% | 94.31% | 92.06% | 31.29% |
| [serde_cbor 0.11.2][serde_cbor] | 7.42% | 25.63% | 29.53% | 58.29% | 66.56% | 19.65% |
| [serde_json 1.0.114][serde_json] | 3.31% | 18.31% | 20.19% | 43.07% | 50.81% | 12.62% |
| [simd-json 0.13.9][simd-json] | 5.69% | 26.75% | 20.19% | 43.07% | 50.81% | 12.60% |
| [speedy 0.8.7][speedy] | 45.85% | 76.99% | 72.89% | 85.52% | 86.87% | 30.20% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.44%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.94%\**</span> | <span title="validated on-demand with panic">*3.52%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*35.48%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.17%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.93%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 529.72 µs | <span title="unvalidated">*2.3029 ms\**</span> | 2984682 | 1408244 | 1273754 | 15.357 ms |
| [alkahest 0.1.5][alkahest] | 736.08 µs | † | 1863391 | 1234113 | 1202345 | 12.161 ms |
| [bincode 2.0.0-rc][bincode] | 1.2017 ms | 3.6794 ms | 1372381 | 1091486 | 1037296 | 9.5888 ms |
| [bincode 1.3.3][bincode1] | 3.7735 ms | 4.1080 ms | 1811011 | 1115281 | 1025627 | 10.361 ms |
| [bitcode 0.6.0][bitcode] | 697.46 µs | 2.2919 ms | 948499 | 857321 | 837658 | 3.1388 ms |
| [borsh 1.3.0][borsh] | 2.8868 ms | 2.7924 ms | 1486162 | 1082357 | 1013550 | 10.064 ms |
| [bson 2.9.0][bson] | 21.340 ms | 43.283 ms | 10030880 | 2833079 | 1600859 | 28.564 ms |
| [capnp 0.18.13][capnp] | 2.1608 ms | † | 2664040 | 1511895 | 1212087 | 14.866 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.2347 ms | 17.816 ms | 5878791 | 1655835 | 1431390 | 22.548 ms |
| [ciborium 0.2.2][ciborium] | 22.716 ms | 48.061 ms | 5878653 | 1655791 | 1431560 | 22.503 ms |
| [databuf 0.5.0][databuf] | 1.5155 ms | 3.6059 ms | 1288257 | 1037579 | 984337 | 8.9545 ms |
| [dlhn 0.1.6][dlhn] | 4.8354 ms | 6.2684 ms | 1279599 | 1052061 | 1021161 | 8.7886 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.1281 ms | † | 2273740 | 1408408 | 1235566 | 13.575 ms |
| [msgpacker 0.4.3][msgpacker] | 1.8606 ms | 4.5346 ms | 1424043 | 1128758 | 1110156 | 9.8326 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 32.372 ms | 15.498 ms | 1728519 | 1247642 | 1233323 | 12.447 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2927 ms | 2.8837 ms | 1770477 | 1108304 | 1029947 | 10.406 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.1149 ms | 2.9795 ms | 1288257 | 1039269 | 986510 | 9.1187 ms |
| [postcard 1.0.8][postcard] | 1.8208 ms | 3.9014 ms | 1279599 | 1058243 | 1016738 | 8.8611 ms |
| [pot 3.0.0][pot] | 13.224 ms | 31.256 ms | 2544810 | 1447453 | 1268390 | 16.325 ms |
| [prost 0.12.3][prost] | <span title="encode">*4.3188 ms\**</span> <span title="populate + encode">*8.3383 ms\**</span> | 9.1252 ms | 1818378 | 1307777 | 1266311 | 12.197 ms |
| [rkyv 0.7.44][rkyv] | 1.3010 ms | <span title="unvalidated">*2.1425 ms\**</span> <span title="validated upfront with error">*2.7731 ms\**</span> | 2029080 | 1335117 | 1158855 | 12.361 ms |
| [rmp-serde 1.1.2][rmp-serde] | 10.181 ms | 11.381 ms | 1703813 | 1231892 | 1200208 | 11.491 ms |
| [ron 0.8.1][ron] | 37.378 ms | 96.013 ms | 8476284 | 2181196 | 1783971 | 35.601 ms |
| [savefile 0.16.5][savefile] | 1.0065 ms | 2.6604 ms | 1750226 | 1101682 | 1027827 | 10.283 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9161 ms | 4.4944 ms | 1288257 | 1037597 | 984356 | 9.0254 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.2811 ms | 21.419 ms | 5878653 | 1655791 | 1431560 | 22.556 ms |
| [serde_json 1.0.114][serde_json] | 21.300 ms | 30.949 ms | 9175594 | 2334253 | 1800713 | 35.092 ms |
| [simd-json 0.13.9][simd-json] | 11.292 ms | 26.132 ms | 9175594 | 2334253 | 1800713 | 35.334 ms |
| [speedy 0.8.7][speedy] | 744.25 µs | 2.5006 ms | 1546963 | 1093532 | 1013443 | 10.154 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*65.223 µs\**</span> | <span title="unvalidated">*66.423 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8548 ns\**</span> | <span title="validated on-demand with panic">*626.67 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.887 ns\**</span> | <span title="validated on-demand with error">*711.11 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4773 ns\**</span> <span title="validated upfront with error">*5.0169 ms\**</span> | <span title="unvalidated">*2.6500 µs\**</span> <span title="validated upfront with error">*5.0251 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2364 ns\**</span> <span title="validated upfront with error">*618.50 µs\**</span> | <span title="unvalidated">*491.67 ns\**</span> <span title="validated upfront with error">*621.43 µs\**</span> | 502.74 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.03%\**</span> | 31.78% | 60.88% | 65.76% | 20.44% |
| [alkahest 0.1.5][alkahest] | 71.97% | † | 50.90% | 69.47% | 69.67% | 25.81% |
| [bincode 2.0.0-rc][bincode] | 44.08% | 58.23% | 69.11% | 78.55% | 80.75% | 32.73% |
| [bincode 1.3.3][bincode1] | 14.04% | 52.15% | 52.37% | 76.87% | 81.67% | 30.30% |
| [bitcode 0.6.0][bitcode] | 75.95% | 93.48% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 18.35% | 76.73% | 63.82% | 79.21% | 82.65% | 31.19% |
| [bson 2.9.0][bson] | 2.48% | 4.95% | 9.46% | 30.26% | 52.33% | 10.99% |
| [capnp 0.18.13][capnp] | 24.51% | † | 35.60% | 56.71% | 69.11% | 21.11% |
| [cbor4ii 0.3.2][cbor4ii] | 12.51% | 12.03% | 16.13% | 51.78% | 58.52% | 13.92% |
| [ciborium 0.2.2][ciborium] | 2.33% | 4.46% | 16.13% | 51.78% | 58.51% | 13.95% |
| [databuf 0.5.0][databuf] | 34.95% | 59.42% | 73.63% | 82.63% | 85.10% | 35.05% |
| [dlhn 0.1.6][dlhn] | 10.96% | 34.18% | 74.12% | 81.49% | 82.03% | 35.71% |
| [flatbuffers 23.5.26][flatbuffers] | 10.33% | † | 41.72% | 60.87% | 67.80% | 23.12% |
| [msgpacker 0.4.3][msgpacker] | 28.47% | 47.25% | 66.61% | 75.95% | 75.45% | 31.92% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.64% | 13.82% | 54.87% | 68.72% | 67.92% | 25.22% |
| [nanoserde 0.1.37][nanoserde] | 40.98% | 74.30% | 53.57% | 77.35% | 81.33% | 30.16% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 17.01% | 71.91% | 73.63% | 82.49% | 84.91% | 34.42% |
| [postcard 1.0.8][postcard] | 29.09% | 54.92% | 74.12% | 81.01% | 82.39% | 35.42% |
| [pot 3.0.0][pot] | 4.01% | 6.85% | 37.27% | 59.23% | 66.04% | 19.23% |
| [prost 0.12.3][prost] | <span title="encode">*12.27%\**</span> <span title="populate + encode">*6.35%\**</span> | 23.48% | 52.16% | 65.56% | 66.15% | 25.73% |
| [rkyv 0.7.44][rkyv] | 40.72% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.26%\**</span> | 46.75% | 64.21% | 72.28% | 25.39% |
| [rmp-serde 1.1.2][rmp-serde] | 5.20% | 18.83% | 55.67% | 69.59% | 69.79% | 27.31% |
| [ron 0.8.1][ron] | 1.42% | 2.23% | 11.19% | 39.31% | 46.95% | 8.82% |
| [savefile 0.16.5][savefile] | 52.63% | 80.53% | 54.19% | 77.82% | 81.50% | 30.52% |
| [serde_bare 0.5.0][serde_bare] | 10.78% | 47.67% | 73.63% | 82.63% | 85.10% | 34.78% |
| [serde_cbor 0.11.2][serde_cbor] | 5.71% | 10.00% | 16.13% | 51.78% | 58.51% | 13.92% |
| [serde_json 1.0.114][serde_json] | 2.49% | 6.92% | 10.34% | 36.73% | 46.52% | 8.94% |
| [simd-json 0.13.9][simd-json] | 4.69% | 8.20% | 10.34% | 36.73% | 46.52% | 8.88% |
| [speedy 0.8.7][speedy] | 71.18% | 85.68% | 61.31% | 78.40% | 82.65% | 30.91% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.74%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*78.46%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.65%\**</span> | <span title="validated on-demand with error">*69.14%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.91%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.55%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.08%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.0
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
[simd-json]: https://crates.io/crates/simd-json/0.13.9
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
