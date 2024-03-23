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

## Last updated: 2024-3-23 15:26:56

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 405.81 µs | <span title="unvalidated">*1.4620 ms\**</span> | 1705800 | 520074 | 413370 | 6.9773 ms |
| [alkahest 0.1.5][alkahest] | 191.81 µs | † | 1045784 | 454157 | 389424 | 6.2019 ms |
| [bincode 2.0.0-rc][bincode] | 214.15 µs | 2.4753 ms | 741295 | 303944 | 257153 | 4.0533 ms |
| [bincode 1.3.3][bincode1] | 524.67 µs | 1.9896 ms | 1045784 | 373127 | 311761 | 4.9713 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 139.00 µs | 1.4842 ms | 703710 | 288826 | 229755 | 2.4393 ms |
| [borsh 1.3.0][borsh] | 539.76 µs | 2.1785 ms | 885780 | 362204 | 286514 | 4.5190 ms |
| [bson 2.9.0][bson] | 2.2160 ms | 7.0814 ms | 1924682 | 532821 | 376270 | 5.7662 ms |
| [capnp 0.18.13][capnp] | 480.25 µs | † | 1443216 | 513986 | 428649 | 7.0325 ms |
| [cbor4ii 0.3.2][cbor4ii] | 903.92 µs | 4.7831 ms | 1407835 | 403440 | 324081 | 4.8859 ms |
| [ciborium 0.2.2][ciborium] | 3.8978 ms | 9.9727 ms | 1407835 | 403440 | 324081 | 4.8945 ms |
| [databuf 0.5.0][databuf] | 258.10 µs | 2.0273 ms | 765778 | 311715 | 264630 | 4.1559 ms |
| [dlhn 0.1.6][dlhn] | 794.19 µs | 2.4576 ms | 724953 | 301446 | 253629 | 3.7948 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.5106 ms | † | 1276368 | 468539 | 388832 | 5.6714 ms |
| [msgpacker 0.4.3][msgpacker] | 1.3667 ms | 2.5416 ms | 764996 | 315291 | 264898 | 4.2062 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.6576 ms | 4.0246 ms | 818669 | 332556 | 285514 | 4.6535 ms |
| [nanoserde 0.1.37][nanoserde] | 304.82 µs | 2.0534 ms | 1045784 | 373127 | 311761 | 4.5916 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 668.21 µs | 2.2054 ms | 765778 | 311743 | 264518 | 4.1349 ms |
| [postcard 1.0.8][postcard] | 414.79 µs | 2.1109 ms | 724953 | 302399 | 253747 | 3.8825 ms |
| [pot 3.0.0][pot] | 2.3433 ms | 6.4284 ms | 971922 | 372513 | 304122 | 4.9300 ms |
| [prost 0.12.3][prost] | <span title="encode">*831.84 µs\**</span> <span title="populate + encode">*2.3479 ms\**</span> | 3.5893 ms | 884628 | 363130 | 315494 | 5.2270 ms |
| [rkyv 0.7.44][rkyv] | 216.82 µs | <span title="unvalidated">*1.4515 ms\**</span> <span title="validated upfront with error">*1.9797 ms\**</span> | 1011488 | 383862 | 333545 | 5.2943 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3256 ms | 3.5101 ms | 784997 | 325384 | 278219 | 4.4741 ms |
| [ron 0.8.1][ron] | 14.100 ms | 16.878 ms | 1607459 | 449158 | 349713 | 6.1435 ms |
| [savefile 0.16.5][savefile] | 200.79 µs | 2.1352 ms | 1045800 | 373139 | 311755 | 4.9280 ms |
| [serde_bare 0.5.0][serde_bare] | 666.29 µs | 2.1114 ms | 765778 | 311715 | 264630 | 3.9278 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9325 ms | 5.0649 ms | 1407835 | 403440 | 324081 | 4.8700 ms |
| [serde_json 1.0.114][serde_json] | 3.7349 ms | 5.8798 ms | 1827461 | 470560 | 361090 | 5.6021 ms |
| [simd-json 0.13.9][simd-json] | 2.1438 ms | 4.6254 ms | 1827461 | 470560 | 361090 | 5.8610 ms |
| [speedy 0.8.7][speedy] | 198.76 µs | 1.7501 ms | 885780 | 362204 | 286514 | 4.3198 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.202 µs\**</span> | <span title="unvalidated">*37.766 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8548 ns\**</span> | <span title="validated on-demand with panic">*24.897 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.095 ns\**</span> | <span title="validated on-demand with error">*162.52 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4731 ns\**</span> <span title="validated upfront with error">*1.8789 ms\**</span> | <span title="unvalidated">*50.896 µs\**</span> <span title="validated upfront with error">*1.9631 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2372 ns\**</span> <span title="validated upfront with error">*524.94 µs\**</span> | <span title="unvalidated">*10.854 µs\**</span> <span title="validated upfront with error">*538.55 µs\**</span> | 10.137 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 34.25% | <span title="unvalidated">*99.28%\**</span> | 41.25% | 55.54% | 55.58% | 34.96% |
| [alkahest 0.1.5][alkahest] | 72.47% | † | 67.29% | 63.60% | 59.00% | 39.33% |
| [bincode 2.0.0-rc][bincode] | 64.91% | 58.64% | 94.93% | 95.03% | 89.35% | 60.18% |
| [bincode 1.3.3][bincode1] | 26.49% | 72.95% | 67.29% | 77.41% | 73.70% | 49.07% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 97.80% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 25.75% | 66.63% | 79.45% | 79.74% | 80.19% | 53.98% |
| [bson 2.9.0][bson] | 6.27% | 20.50% | 36.56% | 54.21% | 61.06% | 42.30% |
| [capnp 0.18.13][capnp] | 28.94% | † | 48.76% | 56.19% | 53.60% | 34.69% |
| [cbor4ii 0.3.2][cbor4ii] | 15.38% | 30.35% | 49.99% | 71.59% | 70.89% | 49.93% |
| [ciborium 0.2.2][ciborium] | 3.57% | 14.55% | 49.99% | 71.59% | 70.89% | 49.84% |
| [databuf 0.5.0][databuf] | 53.86% | 71.60% | 91.89% | 92.66% | 86.82% | 58.69% |
| [dlhn 0.1.6][dlhn] | 17.50% | 59.06% | 97.07% | 95.81% | 90.59% | 64.28% |
| [flatbuffers 23.5.26][flatbuffers] | 9.20% | † | 55.13% | 61.64% | 59.09% | 43.01% |
| [msgpacker 0.4.3][msgpacker] | 10.17% | 57.11% | 91.99% | 91.61% | 86.73% | 57.99% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.46% | 36.07% | 85.96% | 86.85% | 80.47% | 52.42% |
| [nanoserde 0.1.37][nanoserde] | 45.60% | 70.69% | 67.29% | 77.41% | 73.70% | 53.13% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.80% | 65.82% | 91.89% | 92.65% | 86.86% | 58.99% |
| [postcard 1.0.8][postcard] | 33.51% | 68.76% | 97.07% | 95.51% | 90.54% | 62.83% |
| [pot 3.0.0][pot] | 5.93% | 22.58% | 72.40% | 77.53% | 75.55% | 49.48% |
| [prost 0.12.3][prost] | <span title="encode">*16.71%\**</span> <span title="populate + encode">*5.92%\**</span> | 40.44% | 79.55% | 79.54% | 72.82% | 46.67% |
| [rkyv 0.7.44][rkyv] | 64.11% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.32%\**</span> | 69.57% | 75.24% | 68.88% | 46.07% |
| [rmp-serde 1.1.2][rmp-serde] | 10.49% | 41.35% | 89.64% | 88.76% | 82.58% | 54.52% |
| [ron 0.8.1][ron] | 0.99% | 8.60% | 43.78% | 64.30% | 65.70% | 39.71% |
| [savefile 0.16.5][savefile] | 69.23% | 67.98% | 67.29% | 77.40% | 73.70% | 49.50% |
| [serde_bare 0.5.0][serde_bare] | 20.86% | 68.75% | 91.89% | 92.66% | 86.82% | 62.10% |
| [serde_cbor 0.11.2][serde_cbor] | 7.19% | 28.66% | 49.99% | 71.59% | 70.89% | 50.09% |
| [serde_json 1.0.114][serde_json] | 3.72% | 24.69% | 38.51% | 61.38% | 63.63% | 43.54% |
| [simd-json 0.13.9][simd-json] | 6.48% | 31.38% | 38.51% | 61.38% | 63.63% | 41.62% |
| [speedy 0.8.7][speedy] | 69.93% | 82.94% | 79.45% | 79.74% | 80.19% | 56.47% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*28.74%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.70%\**</span> | <span title="validated on-demand with panic">*43.60%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.67%\**</span> | <span title="validated on-demand with error">*6.68%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.33%\**</span> <span title="validated upfront with error">*0.55%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.02%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 259.06 µs | <span title="unvalidated">*259.11 µs\**</span> | 6000024 | 5378513 | 5345890 | 7.4562 ms |
| [alkahest 0.1.5][alkahest] | 148.31 µs | † | 6000008 | 5378500 | 5345890 | 7.6031 ms |
| [bincode 2.0.0-rc][bincode] | 422.11 µs | 828.01 µs | 6000005 | 5378497 | 5345897 | 7.4450 ms |
| [bincode 1.3.3][bincode1] | 5.0373 ms | 4.0435 ms | 6000008 | 5378500 | 5345890 | 7.6141 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 1.3977 ms | 616.43 µs | 6000006 | 5182295 | 4923880 | 13.648 ms |
| [borsh 1.3.0][borsh] | 6.2040 ms | 4.2336 ms | 6000004 | 5378496 | 5345889 | 7.4847 ms |
| [bson 2.9.0][bson] | 41.453 ms | 80.913 ms | 23013911 | 9212089 | 7497811 | 121.04 ms |
| [capnp 0.18.13][capnp] | 5.5026 ms | † | 14000088 | 7130367 | 6051062 | 93.048 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.432 ms | 46.900 ms | 13125016 | 7524114 | 6757967 | 99.848 ms |
| [ciborium 0.2.2][ciborium] | 63.863 ms | 106.14 ms | 13122324 | 7524660 | 6759658 | 99.934 ms |
| [databuf 0.5.0][databuf] | 2.3991 ms | 5.2664 ms | 6000003 | 5378495 | 5345900 | 7.5478 ms |
| [dlhn 0.1.6][dlhn] | 7.1343 ms | 5.7843 ms | 6000003 | 5378495 | 5345900 | 7.4602 ms |
| [flatbuffers 23.5.26][flatbuffers] | 649.64 µs | † | 6000024 | 5378434 | 5345910 | 7.4845 ms |
| [msgpacker 0.4.3][msgpacker] | 19.307 ms | 8.6478 ms | 7500005 | 6058442 | 6014337 | 9.7705 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 125.12 ms | 26.808 ms | 8125037 | 6493484 | 6386940 | 85.982 ms |
| [nanoserde 0.1.37][nanoserde] | 1.0745 ms | 899.15 µs | 6000008 | 5378500 | 5345890 | 7.6300 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 5.1530 ms | 4.0082 ms | 6000004 | 5378496 | 5345889 | 7.4720 ms |
| [postcard 1.0.8][postcard] | 507.99 µs | 1.4346 ms | 6000003 | 5378495 | 5345900 | 7.5370 ms |
| [pot 3.0.0][pot] | 39.277 ms | 71.310 ms | 10122342 | 6814618 | 6852251 | 94.760 ms |
| [prost 0.12.3][prost] | <span title="encode">*7.0948 ms\**</span> <span title="populate + encode">*8.2902 ms\**</span> | 13.890 ms | 8750000 | 6665735 | 6421871 | 84.147 ms |
| [rkyv 0.7.44][rkyv] | 204.58 µs | <span title="unvalidated">*150.13 µs\**</span> <span title="validated upfront with error">*148.98 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.5521 ms |
| [rmp-serde 1.1.2][rmp-serde] | 13.451 ms | 18.631 ms | 8125006 | 6494876 | 6391037 | 81.596 ms |
| [ron 0.8.1][ron] | 171.42 ms | 246.11 ms | 22192885 | 8970395 | 8138755 | 157.60 ms |
| [savefile 0.16.5][savefile] | 259.73 µs | 1.5794 ms | 6000024 | 5378518 | 5345893 | 7.4197 ms |
| [serde_bare 0.5.0][serde_bare] | 6.5525 ms | 4.1384 ms | 6000003 | 5378495 | 5345900 | 7.7195 ms |
| [serde_cbor 0.11.2][serde_cbor] | 34.520 ms | 44.088 ms | 13122324 | 7524660 | 6759658 | 101.15 ms |
| [serde_json 1.0.114][serde_json] | 89.719 ms | 88.200 ms | 26192883 | 9566084 | 8586741 | 165.71 ms |
| [simd-json 0.13.9][simd-json] | 54.284 ms | 74.279 ms | 26192883 | 9566084 | 8586741 | 165.62 ms |
| [speedy 0.8.7][speedy] | 259.15 µs | 259.29 µs | 6000004 | 5378496 | 5345889 | 7.4892 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1637 ns\**</span> | <span title="unvalidated">*141.52 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8565 ns\**</span> | <span title="validated on-demand with panic">*77.327 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*107.32 ns\**</span> | <span title="validated on-demand with error">*2.1362 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4799 ns\**</span> <span title="validated upfront with error">*37.368 ns\**</span> | <span title="unvalidated">*54.101 µs\**</span> <span title="validated upfront with error">*77.420 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2366 ns\**</span> <span title="validated upfront with error">*10.233 ns\**</span> | <span title="unvalidated">*48.441 µs\**</span> <span title="validated upfront with error">*38.767 µs\**</span> | 104.23 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 57.25% | <span title="unvalidated">*57.50%\**</span> | 100.00% | 96.35% | 92.11% | 99.51% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 97.59% |
| [bincode 2.0.0-rc][bincode] | 35.14% | 17.99% | 100.00% | 96.35% | 92.11% | 99.66% |
| [bincode 1.3.3][bincode1] | 2.94% | 3.68% | 100.00% | 96.35% | 92.11% | 97.45% |
| [bitcode 0.6.0-alpha.2][bitcode] | 10.61% | 24.17% | 100.00% | 100.00% | 100.00% | 54.36% |
| [borsh 1.3.0][borsh] | 2.39% | 3.52% | 100.00% | 96.35% | 92.11% | 99.13% |
| [bson 2.9.0][bson] | 0.36% | 0.18% | 26.07% | 56.26% | 65.67% | 6.13% |
| [capnp 0.18.13][capnp] | 2.70% | † | 42.86% | 72.68% | 81.37% | 7.97% |
| [cbor4ii 0.3.2][cbor4ii] | 1.42% | 0.32% | 45.71% | 68.88% | 72.86% | 7.43% |
| [ciborium 0.2.2][ciborium] | 0.23% | 0.14% | 45.72% | 68.87% | 72.84% | 7.42% |
| [databuf 0.5.0][databuf] | 6.18% | 2.83% | 100.00% | 96.35% | 92.11% | 98.30% |
| [dlhn 0.1.6][dlhn] | 2.08% | 2.58% | 100.00% | 96.35% | 92.11% | 99.46% |
| [flatbuffers 23.5.26][flatbuffers] | 22.83% | † | 100.00% | 96.35% | 92.11% | 99.13% |
| [msgpacker 0.4.3][msgpacker] | 0.77% | 1.72% | 80.00% | 85.54% | 81.87% | 75.94% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.56% | 73.85% | 79.81% | 77.09% | 8.63% |
| [nanoserde 0.1.37][nanoserde] | 13.80% | 16.57% | 100.00% | 96.35% | 92.11% | 97.24% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.88% | 3.72% | 100.00% | 96.35% | 92.11% | 99.30% |
| [postcard 1.0.8][postcard] | 29.20% | 10.38% | 100.00% | 96.35% | 92.11% | 98.44% |
| [pot 3.0.0][pot] | 0.38% | 0.21% | 59.27% | 76.05% | 71.86% | 7.83% |
| [prost 0.12.3][prost] | <span title="encode">*2.09%\**</span> <span title="populate + encode">*1.79%\**</span> | 1.07% | 68.57% | 77.75% | 76.67% | 8.82% |
| [rkyv 0.7.44][rkyv] | 72.49% | <span title="unvalidated">*99.23%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 98.25% |
| [rmp-serde 1.1.2][rmp-serde] | 1.10% | 0.80% | 73.85% | 79.79% | 77.04% | 9.09% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 4.71% |
| [savefile 0.16.5][savefile] | 57.10% | 9.43% | 100.00% | 96.35% | 92.11% | 100.00% |
| [serde_bare 0.5.0][serde_bare] | 2.26% | 3.60% | 100.00% | 96.35% | 92.11% | 96.12% |
| [serde_cbor 0.11.2][serde_cbor] | 0.43% | 0.34% | 45.72% | 68.87% | 72.84% | 7.34% |
| [serde_json 1.0.114][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.48% |
| [simd-json 0.13.9][simd-json] | 0.27% | 0.20% | 22.91% | 54.17% | 57.34% | 4.48% |
| [speedy 0.8.7][speedy] | 57.23% | 57.46% | 100.00% | 96.35% | 92.11% | 99.07% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.15%\**</span> | <span title="unvalidated">*27.39%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.61%\**</span> | <span title="validated on-demand with panic">*50.13%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.15%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.86%\**</span> <span title="validated upfront with error">*3.31%\**</span> | <span title="unvalidated">*71.66%\**</span> <span title="validated upfront with error">*50.07%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*12.08%\**</span> | <span title="unvalidated">*80.03%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 190.08 µs | <span title="unvalidated">*1.3148 ms\**</span> | 1290592 | 396627 | 340243 | 5.0517 ms |
| [alkahest 0.1.5][alkahest] | 217.82 µs | † | 667570 | 325484 | 320452 | 4.0045 ms |
| [bincode 2.0.0-rc][bincode] | 268.18 µs | 2.0695 ms | 367413 | 221291 | 206273 | 2.5391 ms |
| [bincode 1.3.3][bincode1] | 573.06 µs | 1.7841 ms | 569975 | 240525 | 232423 | 2.9765 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 126.94 µs | 1.2675 ms | 327688 | 200947 | 182736 | 772.37 µs |
| [borsh 1.3.0][borsh] | 558.23 µs | 1.8261 ms | 446595 | 234236 | 210008 | 2.5454 ms |
| [bson 2.9.0][bson] | 2.8481 ms | 8.2384 ms | 1619653 | 502185 | 328399 | 5.0147 ms |
| [capnp 0.18.13][capnp] | 455.19 µs | † | 803896 | 335606 | 280851 | 4.0429 ms |
| [cbor4ii 0.3.2][cbor4ii] | 807.41 µs | 4.8451 ms | 1109831 | 344745 | 274514 | 3.9059 ms |
| [ciborium 0.2.2][ciborium] | 3.7526 ms | 9.4784 ms | 1109821 | 344751 | 274526 | 3.9346 ms |
| [databuf 0.5.0][databuf] | 310.84 µs | 1.7433 ms | 356311 | 213062 | 198488 | 2.4619 ms |
| [dlhn 0.1.6][dlhn] | 801.88 µs | 2.5783 ms | 366496 | 220600 | 205683 | 2.5313 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.3341 ms | † | 844168 | 345696 | 294015 | 3.9782 ms |
| [msgpacker 0.4.3][msgpacker] | 961.93 µs | 2.8277 ms | 391251 | 236877 | 220476 | 2.7273 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2990 ms | 3.8997 ms | 449745 | 252432 | 231110 | 2.8115 ms |
| [nanoserde 0.1.37][nanoserde] | 287.77 µs | 1.8872 ms | 567975 | 239930 | 232419 | 2.9684 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 642.70 µs | 1.9900 ms | 356311 | 212976 | 198524 | 2.4648 ms |
| [postcard 1.0.8][postcard] | 428.85 µs | 1.9941 ms | 367489 | 221913 | 207344 | 2.5409 ms |
| [pot 3.0.0][pot] | 2.2467 ms | 6.0245 ms | 599125 | 299158 | 247693 | 3.2259 ms |
| [prost 0.12.3][prost] | <span title="encode">*1.0880 ms\**</span> <span title="populate + encode">*2.7743 ms\**</span> | 3.5699 ms | 596811 | 305319 | 269310 | 3.5311 ms |
| [rkyv 0.7.44][rkyv] | 302.07 µs | <span title="unvalidated">*1.2621 ms\**</span> <span title="validated upfront with error">*1.7720 ms\**</span> | 596952 | 253967 | 220706 | 2.7732 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.4083 ms | 2.9338 ms | 424533 | 245214 | 226188 | 2.7722 ms |
| [ron 0.8.1][ron] | 8.1951 ms | 17.785 ms | 1465223 | 434935 | 343338 | 5.9580 ms |
| [savefile 0.16.5][savefile] | 216.95 µs | 1.8550 ms | 566991 | 239361 | 232010 | 2.9391 ms |
| [serde_bare 0.5.0][serde_bare] | 717.65 µs | 2.2073 ms | 356311 | 213062 | 198488 | 2.4794 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7822 ms | 4.7103 ms | 1109821 | 344751 | 274526 | 3.9335 ms |
| [serde_json 1.0.114][serde_json] | 3.7550 ms | 6.8199 ms | 1623191 | 466527 | 359623 | 6.1370 ms |
| [simd-json 0.13.9][simd-json] | 2.2281 ms | 5.0982 ms | 1623191 | 466527 | 359623 | 6.1139 ms |
| [speedy 0.8.7][speedy] | 271.16 µs | 1.6585 ms | 449595 | 234970 | 210361 | 2.5481 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.766 µs\**</span> | <span title="unvalidated">*37.520 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8545 ns\**</span> | <span title="validated on-demand with panic">*4.6005 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.053 ns\**</span> | <span title="validated on-demand with error">*563.07 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4733 ns\**</span> <span title="validated upfront with error">*2.1558 ms\**</span> | <span title="unvalidated">*1.3745 µs\**</span> <span title="validated upfront with error">*2.1580 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2367 ns\**</span> <span title="validated upfront with error">*500.32 µs\**</span> | <span title="unvalidated">*239.29 ns\**</span> <span title="validated upfront with error">*499.77 µs\**</span> | 963.65 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 66.78% | <span title="unvalidated">*95.99%\**</span> | 25.39% | 50.66% | 53.71% | 15.29% |
| [alkahest 0.1.5][alkahest] | 58.28% | † | 49.09% | 61.74% | 57.02% | 19.29% |
| [bincode 2.0.0-rc][bincode] | 47.33% | 60.99% | 89.19% | 90.81% | 88.59% | 30.42% |
| [bincode 1.3.3][bincode1] | 22.15% | 70.74% | 57.49% | 83.55% | 78.62% | 25.95% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 99.57% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 22.74% | 69.11% | 73.37% | 85.79% | 87.01% | 30.34% |
| [bson 2.9.0][bson] | 4.46% | 15.32% | 20.23% | 40.01% | 55.64% | 15.40% |
| [capnp 0.18.13][capnp] | 27.89% | † | 40.76% | 59.88% | 65.07% | 19.10% |
| [cbor4ii 0.3.2][cbor4ii] | 15.72% | 26.05% | 29.53% | 58.29% | 66.57% | 19.77% |
| [ciborium 0.2.2][ciborium] | 3.38% | 13.32% | 29.53% | 58.29% | 66.56% | 19.63% |
| [databuf 0.5.0][databuf] | 40.84% | 72.40% | 91.97% | 94.31% | 92.06% | 31.37% |
| [dlhn 0.1.6][dlhn] | 15.83% | 48.95% | 89.41% | 91.09% | 88.84% | 30.51% |
| [flatbuffers 23.5.26][flatbuffers] | 3.81% | † | 38.82% | 58.13% | 62.15% | 19.41% |
| [msgpacker 0.4.3][msgpacker] | 13.20% | 44.63% | 83.75% | 84.83% | 82.88% | 28.32% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.40% | 32.36% | 72.86% | 79.60% | 79.07% | 27.47% |
| [nanoserde 0.1.37][nanoserde] | 44.11% | 66.88% | 57.69% | 83.75% | 78.62% | 26.02% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 19.75% | 63.42% | 91.97% | 94.35% | 92.05% | 31.34% |
| [postcard 1.0.8][postcard] | 29.60% | 63.29% | 89.17% | 90.55% | 88.13% | 30.40% |
| [pot 3.0.0][pot] | 5.65% | 20.95% | 54.69% | 67.17% | 73.78% | 23.94% |
| [prost 0.12.3][prost] | <span title="encode">*11.67%\**</span> <span title="populate + encode">*4.58%\**</span> | 35.35% | 54.91% | 65.82% | 67.85% | 21.87% |
| [rkyv 0.7.44][rkyv] | 42.02% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.22%\**</span> | 54.89% | 79.12% | 82.80% | 27.85% |
| [rmp-serde 1.1.2][rmp-serde] | 9.01% | 43.02% | 77.19% | 81.95% | 80.79% | 27.86% |
| [ron 0.8.1][ron] | 1.55% | 7.10% | 22.36% | 46.20% | 53.22% | 12.96% |
| [savefile 0.16.5][savefile] | 58.51% | 68.04% | 57.79% | 83.95% | 78.76% | 26.28% |
| [serde_bare 0.5.0][serde_bare] | 17.69% | 57.18% | 91.97% | 94.31% | 92.06% | 31.15% |
| [serde_cbor 0.11.2][serde_cbor] | 7.12% | 26.79% | 29.53% | 58.29% | 66.56% | 19.64% |
| [serde_json 1.0.114][serde_json] | 3.38% | 18.51% | 20.19% | 43.07% | 50.81% | 12.59% |
| [simd-json 0.13.9][simd-json] | 5.70% | 24.76% | 20.19% | 43.07% | 50.81% | 12.63% |
| [speedy 0.8.7][speedy] | 46.81% | 76.10% | 72.89% | 85.52% | 86.87% | 30.31% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.64%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.69%\**</span> | <span title="validated on-demand with panic">*5.20%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.67%\**</span> | <span title="validated on-demand with error">*42.50%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.41%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 484.47 µs | <span title="unvalidated">*2.3140 ms\**</span> | 2984682 | 1406973 | 1270089 | 15.519 ms |
| [alkahest 0.1.5][alkahest] | 744.91 µs | † | 1863391 | 1234113 | 1202345 | 12.007 ms |
| [bincode 2.0.0-rc][bincode] | 697.73 µs | 3.5795 ms | 1372381 | 1091486 | 1037296 | 9.6514 ms |
| [bincode 1.3.3][bincode1] | 3.7642 ms | 3.8985 ms | 1811011 | 1115281 | 1025627 | 10.317 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 709.59 µs | 2.3564 ms | 948499 | 857321 | 837658 | 3.1570 ms |
| [borsh 1.3.0][borsh] | 2.8615 ms | 2.7701 ms | 1486162 | 1082357 | 1013550 | 9.9978 ms |
| [bson 2.9.0][bson] | 21.479 ms | 44.081 ms | 10030880 | 2833079 | 1600859 | 28.732 ms |
| [capnp 0.18.13][capnp] | 2.3339 ms | † | 2664040 | 1511895 | 1212087 | 14.920 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.2957 ms | 17.758 ms | 5878791 | 1655835 | 1431390 | 22.515 ms |
| [ciborium 0.2.2][ciborium] | 23.536 ms | 46.907 ms | 5878653 | 1655791 | 1431560 | 22.652 ms |
| [databuf 0.5.0][databuf] | 1.5005 ms | 3.6086 ms | 1288257 | 1037579 | 984337 | 9.0219 ms |
| [dlhn 0.1.6][dlhn] | 5.1491 ms | 7.2939 ms | 1279599 | 1052061 | 1021161 | 8.8817 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.1820 ms | † | 2273740 | 1408408 | 1235566 | 13.428 ms |
| [msgpacker 0.4.3][msgpacker] | 1.9165 ms | 4.5361 ms | 1424043 | 1128758 | 1110156 | 9.8039 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 31.268 ms | 16.087 ms | 1728519 | 1247642 | 1233323 | 12.559 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3015 ms | 2.8895 ms | 1770477 | 1108304 | 1029947 | 10.406 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.7642 ms | 2.9967 ms | 1288257 | 1039269 | 986510 | 9.1426 ms |
| [postcard 1.0.8][postcard] | 1.6999 ms | 3.9906 ms | 1279599 | 1058243 | 1016738 | 8.9229 ms |
| [pot 3.0.0][pot] | 13.494 ms | 30.298 ms | 2544810 | 1447453 | 1268390 | 16.368 ms |
| [prost 0.12.3][prost] | <span title="encode">*4.9631 ms\**</span> <span title="populate + encode">*8.9618 ms\**</span> | 9.3303 ms | 1818378 | 1307777 | 1266311 | 12.119 ms |
| [rkyv 0.7.44][rkyv] | 1.1167 ms | <span title="unvalidated">*2.1623 ms\**</span> <span title="validated upfront with error">*2.7955 ms\**</span> | 2029080 | 1335117 | 1158855 | 12.334 ms |
| [rmp-serde 1.1.2][rmp-serde] | 10.063 ms | 11.383 ms | 1703813 | 1231892 | 1200208 | 11.713 ms |
| [ron 0.8.1][ron] | 38.751 ms | 100.73 ms | 8476284 | 2181196 | 1783971 | 35.113 ms |
| [savefile 0.16.5][savefile] | 1.0154 ms | 2.6830 ms | 1750226 | 1101682 | 1027827 | 10.413 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8937 ms | 4.4394 ms | 1288257 | 1037597 | 984356 | 9.1199 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.2810 ms | 21.483 ms | 5878653 | 1655791 | 1431560 | 22.570 ms |
| [serde_json 1.0.114][serde_json] | 20.072 ms | 31.617 ms | 9175594 | 2334253 | 1800713 | 35.129 ms |
| [simd-json 0.13.9][simd-json] | 11.493 ms | 26.099 ms | 9175594 | 2334253 | 1800713 | 36.083 ms |
| [speedy 0.8.7][speedy] | 709.22 µs | 2.4862 ms | 1546963 | 1093532 | 1013443 | 10.097 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*65.480 µs\**</span> | <span title="unvalidated">*67.803 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8542 ns\**</span> | <span title="validated on-demand with panic">*625.95 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.002 ns\**</span> | <span title="validated on-demand with error">*718.46 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4740 ns\**</span> <span title="validated upfront with error">*4.6874 ms\**</span> | <span title="unvalidated">*2.6640 µs\**</span> <span title="validated upfront with error">*4.7085 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2366 ns\**</span> <span title="validated upfront with error">*633.44 µs\**</span> | <span title="unvalidated">*484.54 ns\**</span> <span title="validated upfront with error">*635.23 µs\**</span> | 625.28 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.44%\**</span> | 31.78% | 60.93% | 65.95% | 20.34% |
| [alkahest 0.1.5][alkahest] | 65.04% | † | 50.90% | 69.47% | 69.67% | 26.29% |
| [bincode 2.0.0-rc][bincode] | 69.44% | 60.41% | 69.11% | 78.55% | 80.75% | 32.71% |
| [bincode 1.3.3][bincode1] | 12.87% | 55.46% | 52.37% | 76.87% | 81.67% | 30.60% |
| [bitcode 0.6.0-alpha.2][bitcode] | 68.27% | 91.76% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 16.93% | 78.06% | 63.82% | 79.21% | 82.65% | 31.58% |
| [bson 2.9.0][bson] | 2.26% | 4.91% | 9.46% | 30.26% | 52.33% | 10.99% |
| [capnp 0.18.13][capnp] | 20.76% | † | 35.60% | 56.71% | 69.11% | 21.16% |
| [cbor4ii 0.3.2][cbor4ii] | 11.28% | 12.18% | 16.13% | 51.78% | 58.52% | 14.02% |
| [ciborium 0.2.2][ciborium] | 2.06% | 4.61% | 16.13% | 51.78% | 58.51% | 13.94% |
| [databuf 0.5.0][databuf] | 32.29% | 59.92% | 73.63% | 82.63% | 85.10% | 34.99% |
| [dlhn 0.1.6][dlhn] | 9.41% | 29.65% | 74.12% | 81.49% | 82.03% | 35.54% |
| [flatbuffers 23.5.26][flatbuffers] | 9.35% | † | 41.72% | 60.87% | 67.80% | 23.51% |
| [msgpacker 0.4.3][msgpacker] | 25.28% | 47.67% | 66.61% | 75.95% | 75.45% | 32.20% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.55% | 13.44% | 54.87% | 68.72% | 67.92% | 25.14% |
| [nanoserde 0.1.37][nanoserde] | 37.22% | 74.83% | 53.57% | 77.35% | 81.33% | 30.34% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 17.53% | 72.16% | 73.63% | 82.49% | 84.91% | 34.53% |
| [postcard 1.0.8][postcard] | 28.50% | 54.18% | 74.12% | 81.01% | 82.39% | 35.38% |
| [pot 3.0.0][pot] | 3.59% | 7.14% | 37.27% | 59.23% | 66.04% | 19.29% |
| [prost 0.12.3][prost] | <span title="encode">*9.76%\**</span> <span title="populate + encode">*5.41%\**</span> | 23.18% | 52.16% | 65.56% | 66.15% | 26.05% |
| [rkyv 0.7.44][rkyv] | 43.38% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.35%\**</span> | 46.75% | 64.21% | 72.28% | 25.60% |
| [rmp-serde 1.1.2][rmp-serde] | 4.81% | 19.00% | 55.67% | 69.59% | 69.79% | 26.95% |
| [ron 0.8.1][ron] | 1.25% | 2.15% | 11.19% | 39.31% | 46.95% | 8.99% |
| [savefile 0.16.5][savefile] | 47.71% | 80.59% | 54.19% | 77.82% | 81.50% | 30.32% |
| [serde_bare 0.5.0][serde_bare] | 9.90% | 48.71% | 73.63% | 82.63% | 85.10% | 34.62% |
| [serde_cbor 0.11.2][serde_cbor] | 5.22% | 10.07% | 16.13% | 51.78% | 58.51% | 13.99% |
| [serde_json 1.0.114][serde_json] | 2.41% | 6.84% | 10.34% | 36.73% | 46.52% | 8.99% |
| [simd-json 0.13.9][simd-json] | 4.22% | 8.28% | 10.34% | 36.73% | 46.52% | 8.75% |
| [speedy 0.8.7][speedy] | 68.31% | 86.97% | 61.31% | 78.40% | 82.65% | 31.27% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.71%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.69%\**</span> | <span title="validated on-demand with panic">*77.41%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.67%\**</span> | <span title="validated on-demand with error">*67.44%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.19%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.08%\**</span> | 100.00% |

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
[simd-json]: https://crates.io/crates/simd-json/0.13.9
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
