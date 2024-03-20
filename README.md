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

## Last updated: 2024-3-20 6:40:7

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 394.74 µs | <span title="unvalidated">*1.4640 ms\**</span> | 1705800 | 520086 | 413340 | 7.0053 ms |
| [alkahest 0.1.5][alkahest] | 189.47 µs | † | 1045784 | 454157 | 389424 | 6.2996 ms |
| [bincode 2.0.0-rc][bincode] | 205.47 µs | 2.4451 ms | 741295 | 303944 | 257153 | 4.1511 ms |
| [bincode 1.3.3][bincode1] | 521.41 µs | 2.0372 ms | 1045784 | 373127 | 311761 | 4.8995 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 147.58 µs | 1.4956 ms | 703710 | 288826 | 229755 | 2.4491 ms |
| [borsh 1.3.0][borsh] | 543.51 µs | 2.1869 ms | 885780 | 362204 | 286514 | 4.5368 ms |
| [bson 2.9.0][bson] | 2.2216 ms | 7.3049 ms | 1924682 | 532821 | 376270 | 5.7578 ms |
| [capnp 0.18.13][capnp] | 463.06 µs | † | 1443216 | 513986 | 428649 | 7.0597 ms |
| [cbor4ii 0.3.2][cbor4ii] | 905.54 µs | 4.7995 ms | 1407835 | 403440 | 324081 | 5.0059 ms |
| [ciborium 0.2.2][ciborium] | 3.9787 ms | 10.170 ms | 1407835 | 403440 | 324081 | 4.9440 ms |
| [databuf 0.5.0][databuf] | 270.00 µs | 2.0358 ms | 765778 | 311715 | 264630 | 4.1548 ms |
| [dlhn 0.1.6][dlhn] | 767.74 µs | 2.4345 ms | 724953 | 301446 | 253629 | 3.8018 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.3562 ms | † | 1276368 | 468539 | 388832 | 5.6441 ms |
| [msgpacker 0.4.3][msgpacker] | 1.2089 ms | 2.5155 ms | 764996 | 315291 | 264898 | 4.2577 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.6806 ms | 4.0726 ms | 818669 | 332556 | 285514 | 4.7062 ms |
| [nanoserde 0.1.37][nanoserde] | 252.92 µs | 2.0619 ms | 1045784 | 373127 | 311761 | 4.7154 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 674.22 µs | 2.2030 ms | 765778 | 311743 | 264518 | 4.1575 ms |
| [postcard 1.0.8][postcard] | 403.57 µs | 2.0804 ms | 724953 | 302399 | 253747 | 3.9098 ms |
| [pot 3.0.0][pot] | 2.2278 ms | 6.4209 ms | 971922 | 372513 | 304122 | 5.2308 ms |
| [prost 0.12.3][prost] | <span title="encode">*805.58 µs\**</span> <span title="populate + encode">*2.3036 ms\**</span> | 3.5425 ms | 884628 | 363130 | 315494 | 5.2832 ms |
| [rkyv 0.7.44][rkyv] | 217.96 µs | <span title="unvalidated">*1.4430 ms\**</span> <span title="validated upfront with error">*1.9757 ms\**</span> | 1011488 | 383862 | 333545 | 5.3971 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3562 ms | 3.4944 ms | 784997 | 325384 | 278219 | 4.4527 ms |
| [ron 0.8.1][ron] | 14.064 ms | 16.574 ms | 1607459 | 449158 | 349713 | 5.9024 ms |
| [savefile 0.16.5][savefile] | 202.94 µs | 2.0774 ms | 1045800 | 373139 | 311755 | 4.6311 ms |
| [serde_bare 0.5.0][serde_bare] | 676.15 µs | 2.0914 ms | 765778 | 311715 | 264630 | 3.9485 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9542 ms | 5.2629 ms | 1407835 | 403440 | 324081 | 4.9943 ms |
| [serde_json 1.0.114][serde_json] | 4.0102 ms | 5.7308 ms | 1827461 | 470560 | 361090 | 5.7869 ms |
| [simd-json 0.13.8][simd-json] | 2.1348 ms | 4.6406 ms | 1827461 | 470560 | 361090 | 6.0433 ms |
| [speedy 0.8.7][speedy] | 197.77 µs | 1.7588 ms | 885780 | 362204 | 286514 | 4.3794 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.158 µs\**</span> | <span title="unvalidated">*40.764 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8555 ns\**</span> | <span title="validated on-demand with panic">*24.989 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*73.953 ns\**</span> | <span title="validated on-demand with error">*169.85 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4879 ns\**</span> <span title="validated upfront with error">*1.8361 ms\**</span> | <span title="unvalidated">*51.077 µs\**</span> <span title="validated upfront with error">*1.9432 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*516.95 µs\**</span> | <span title="unvalidated">*10.634 µs\**</span> <span title="validated upfront with error">*527.73 µs\**</span> | 10.040 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 37.39% | <span title="unvalidated">*98.57%\**</span> | 41.25% | 55.53% | 55.58% | 34.96% |
| [alkahest 0.1.5][alkahest] | 77.89% | † | 67.29% | 63.60% | 59.00% | 38.88% |
| [bincode 2.0.0-rc][bincode] | 71.83% | 59.02% | 94.93% | 95.03% | 89.35% | 59.00% |
| [bincode 1.3.3][bincode1] | 28.30% | 70.83% | 67.29% | 77.41% | 73.70% | 49.99% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 96.48% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 27.15% | 65.98% | 79.45% | 79.74% | 80.19% | 53.98% |
| [bson 2.9.0][bson] | 6.64% | 19.75% | 36.56% | 54.21% | 61.06% | 42.54% |
| [capnp 0.18.13][capnp] | 31.87% | † | 48.76% | 56.19% | 53.60% | 34.69% |
| [cbor4ii 0.3.2][cbor4ii] | 16.30% | 30.07% | 49.99% | 71.59% | 70.89% | 48.92% |
| [ciborium 0.2.2][ciborium] | 3.71% | 14.19% | 49.99% | 71.59% | 70.89% | 49.54% |
| [databuf 0.5.0][databuf] | 54.66% | 70.88% | 91.89% | 92.66% | 86.82% | 58.95% |
| [dlhn 0.1.6][dlhn] | 19.22% | 59.27% | 97.07% | 95.81% | 90.59% | 64.42% |
| [flatbuffers 23.5.26][flatbuffers] | 10.88% | † | 55.13% | 61.64% | 59.09% | 43.39% |
| [msgpacker 0.4.3][msgpacker] | 12.21% | 57.36% | 91.99% | 91.61% | 86.73% | 57.52% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.60% | 35.43% | 85.96% | 86.85% | 80.47% | 52.04% |
| [nanoserde 0.1.37][nanoserde] | 58.35% | 69.98% | 67.29% | 77.41% | 73.70% | 51.94% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 21.89% | 65.50% | 91.89% | 92.65% | 86.86% | 58.91% |
| [postcard 1.0.8][postcard] | 36.57% | 69.36% | 97.07% | 95.51% | 90.54% | 62.64% |
| [pot 3.0.0][pot] | 6.62% | 22.47% | 72.40% | 77.53% | 75.55% | 46.82% |
| [prost 0.12.3][prost] | <span title="encode">*18.32%\**</span> <span title="populate + encode">*6.41%\**</span> | 40.73% | 79.55% | 79.54% | 72.82% | 46.36% |
| [rkyv 0.7.44][rkyv] | 67.71% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.04%\**</span> | 69.57% | 75.24% | 68.88% | 45.38% |
| [rmp-serde 1.1.2][rmp-serde] | 10.88% | 41.29% | 89.64% | 88.76% | 82.58% | 55.00% |
| [ron 0.8.1][ron] | 1.05% | 8.71% | 43.78% | 64.30% | 65.70% | 41.49% |
| [savefile 0.16.5][savefile] | 72.72% | 69.46% | 67.29% | 77.40% | 73.70% | 52.88% |
| [serde_bare 0.5.0][serde_bare] | 21.83% | 69.00% | 91.89% | 92.66% | 86.82% | 62.03% |
| [serde_cbor 0.11.2][serde_cbor] | 7.55% | 27.42% | 49.99% | 71.59% | 70.89% | 49.04% |
| [serde_json 1.0.114][serde_json] | 3.68% | 25.18% | 38.51% | 61.38% | 63.63% | 42.32% |
| [simd-json 0.13.8][simd-json] | 6.91% | 31.10% | 38.51% | 61.38% | 63.63% | 40.53% |
| [speedy 0.8.7][speedy] | 74.62% | 82.04% | 79.45% | 79.74% | 80.19% | 55.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*26.09%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*42.55%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.67%\**</span> | <span title="validated on-demand with error">*6.26%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.72%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.82%\**</span> <span title="validated upfront with error">*0.55%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.02%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 237.32 µs | <span title="unvalidated">*236.89 µs\**</span> | 6000024 | 5378513 | 5345890 | 9.0173 ms |
| [alkahest 0.1.5][alkahest] | 197.68 µs | † | 6000008 | 5378500 | 5345890 | 7.5496 ms |
| [bincode 2.0.0-rc][bincode] | 423.00 µs | 831.18 µs | 6000005 | 5378497 | 5345897 | 7.7071 ms |
| [bincode 1.3.3][bincode1] | 4.7755 ms | 4.0445 ms | 6000008 | 5378500 | 5345890 | 7.6805 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 1.4161 ms | 617.36 µs | 6000006 | 5182295 | 4923880 | 13.165 ms |
| [borsh 1.3.0][borsh] | 5.8756 ms | 4.9029 ms | 6000004 | 5378496 | 5345889 | 7.7031 ms |
| [bson 2.9.0][bson] | 42.837 ms | 82.152 ms | 23013911 | 9212089 | 7497811 | 123.42 ms |
| [capnp 0.18.13][capnp] | 5.3228 ms | † | 14000088 | 7130367 | 6051062 | 99.285 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.474 ms | 47.127 ms | 13125016 | 7524114 | 6757967 | 100.98 ms |
| [ciborium 0.2.2][ciborium] | 63.736 ms | 99.895 ms | 13122324 | 7524660 | 6759658 | 102.63 ms |
| [databuf 0.5.0][databuf] | 2.3996 ms | 5.2570 ms | 6000003 | 5378495 | 5345900 | 7.7038 ms |
| [dlhn 0.1.6][dlhn] | 7.1480 ms | 5.8176 ms | 6000003 | 5378495 | 5345900 | 7.7108 ms |
| [flatbuffers 23.5.26][flatbuffers] | 673.76 µs | † | 6000024 | 5378434 | 5345910 | 7.7443 ms |
| [msgpacker 0.4.3][msgpacker] | 21.238 ms | 8.7929 ms | 7500005 | 6058442 | 6014337 | 9.8394 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.91 ms | 26.478 ms | 8125037 | 6493484 | 6386940 | 84.626 ms |
| [nanoserde 0.1.37][nanoserde] | 1.4243 ms | 902.57 µs | 6000008 | 5378500 | 5345890 | 7.7199 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.8469 ms | 4.0305 ms | 6000004 | 5378496 | 5345889 | 7.4808 ms |
| [postcard 1.0.8][postcard] | 477.47 µs | 1.8825 ms | 6000003 | 5378495 | 5345900 | 7.7353 ms |
| [pot 3.0.0][pot] | 37.442 ms | 71.178 ms | 10122342 | 6814618 | 6852251 | 94.421 ms |
| [prost 0.12.3][prost] | <span title="encode">*6.8027 ms\**</span> <span title="populate + encode">*8.8214 ms\**</span> | 14.094 ms | 8750000 | 6665735 | 6421871 | 85.976 ms |
| [rkyv 0.7.44][rkyv] | 203.67 µs | <span title="unvalidated">*198.76 µs\**</span> <span title="validated upfront with error">*200.51 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.7581 ms |
| [rmp-serde 1.1.2][rmp-serde] | 12.096 ms | 19.344 ms | 8125006 | 6494876 | 6391037 | 82.715 ms |
| [ron 0.8.1][ron] | 173.29 ms | 259.80 ms | 22192885 | 8970395 | 8138755 | 160.65 ms |
| [savefile 0.16.5][savefile] | 239.11 µs | 239.62 µs | 6000024 | 5378518 | 5345893 | 7.7320 ms |
| [serde_bare 0.5.0][serde_bare] | 5.9993 ms | 4.2905 ms | 6000003 | 5378495 | 5345900 | 7.7778 ms |
| [serde_cbor 0.11.2][serde_cbor] | 32.729 ms | 44.869 ms | 13122324 | 7524660 | 6759658 | 101.41 ms |
| [serde_json 1.0.114][serde_json] | 91.754 ms | 91.085 ms | 26192883 | 9566084 | 8586741 | 163.79 ms |
| [simd-json 0.13.8][simd-json] | 54.180 ms | 74.187 ms | 26192883 | 9566084 | 8586741 | 164.08 ms |
| [speedy 0.8.7][speedy] | 238.19 µs | 238.37 µs | 6000004 | 5378496 | 5345889 | 7.5465 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1646 ns\**</span> | <span title="unvalidated">*141.12 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8550 ns\**</span> | <span title="validated on-demand with panic">*77.339 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*107.64 ns\**</span> | <span title="validated on-demand with error">*2.1331 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4739 ns\**</span> <span title="validated upfront with error">*37.643 ns\**</span> | <span title="unvalidated">*54.003 µs\**</span> <span title="validated upfront with error">*77.461 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2374 ns\**</span> <span title="validated upfront with error">*10.571 ns\**</span> | <span title="unvalidated">*48.514 µs\**</span> <span title="validated upfront with error">*38.700 µs\**</span> | 106.30 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 83.30% | <span title="unvalidated">*83.90%\**</span> | 100.00% | 96.35% | 92.11% | 82.96% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 99.09% |
| [bincode 2.0.0-rc][bincode] | 46.73% | 23.91% | 100.00% | 96.35% | 92.11% | 97.06% |
| [bincode 1.3.3][bincode1] | 4.14% | 4.91% | 100.00% | 96.35% | 92.11% | 97.40% |
| [bitcode 0.6.0-alpha.2][bitcode] | 13.96% | 32.20% | 100.00% | 100.00% | 100.00% | 56.82% |
| [borsh 1.3.0][borsh] | 3.36% | 4.05% | 100.00% | 96.35% | 92.11% | 97.11% |
| [bson 2.9.0][bson] | 0.46% | 0.24% | 26.07% | 56.26% | 65.67% | 6.06% |
| [capnp 0.18.13][capnp] | 3.71% | † | 42.86% | 72.68% | 81.37% | 7.53% |
| [cbor4ii 0.3.2][cbor4ii] | 1.89% | 0.42% | 45.71% | 68.88% | 72.86% | 7.41% |
| [ciborium 0.2.2][ciborium] | 0.31% | 0.20% | 45.72% | 68.87% | 72.84% | 7.29% |
| [databuf 0.5.0][databuf] | 8.24% | 3.78% | 100.00% | 96.35% | 92.11% | 97.11% |
| [dlhn 0.1.6][dlhn] | 2.77% | 3.42% | 100.00% | 96.35% | 92.11% | 97.02% |
| [flatbuffers 23.5.26][flatbuffers] | 29.34% | † | 100.00% | 96.35% | 92.11% | 96.60% |
| [msgpacker 0.4.3][msgpacker] | 0.93% | 2.26% | 80.00% | 85.54% | 81.87% | 76.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.75% | 73.85% | 79.81% | 77.09% | 8.84% |
| [nanoserde 0.1.37][nanoserde] | 13.88% | 22.02% | 100.00% | 96.35% | 92.11% | 96.90% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.08% | 4.93% | 100.00% | 96.35% | 92.11% | 100.00% |
| [postcard 1.0.8][postcard] | 41.40% | 10.56% | 100.00% | 96.35% | 92.11% | 96.71% |
| [pot 3.0.0][pot] | 0.53% | 0.28% | 59.27% | 76.05% | 71.86% | 7.92% |
| [prost 0.12.3][prost] | <span title="encode">*2.91%\**</span> <span title="populate + encode">*2.24%\**</span> | 1.41% | 68.57% | 77.75% | 76.67% | 8.70% |
| [rkyv 0.7.44][rkyv] | 97.06% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.13%\**</span> | 100.00% | 96.35% | 92.11% | 96.43% |
| [rmp-serde 1.1.2][rmp-serde] | 1.63% | 1.03% | 73.85% | 79.79% | 77.04% | 9.04% |
| [ron 0.8.1][ron] | 0.11% | 0.08% | 27.04% | 57.77% | 60.50% | 4.66% |
| [savefile 0.16.5][savefile] | 82.67% | 82.95% | 100.00% | 96.35% | 92.11% | 96.75% |
| [serde_bare 0.5.0][serde_bare] | 3.30% | 4.63% | 100.00% | 96.35% | 92.11% | 96.18% |
| [serde_cbor 0.11.2][serde_cbor] | 0.60% | 0.44% | 45.72% | 68.87% | 72.84% | 7.38% |
| [serde_json 1.0.114][serde_json] | 0.22% | 0.22% | 22.91% | 54.17% | 57.34% | 4.57% |
| [simd-json 0.13.8][simd-json] | 0.36% | 0.27% | 22.91% | 54.17% | 57.34% | 4.56% |
| [speedy 0.8.7][speedy] | 82.99% | 83.38% | 100.00% | 96.35% | 92.11% | 99.13% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.17%\**</span> | <span title="unvalidated">*27.42%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.71%\**</span> | <span title="validated on-demand with panic">*50.04%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.15%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*3.29%\**</span> | <span title="unvalidated">*71.66%\**</span> <span title="validated upfront with error">*49.96%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*11.71%\**</span> | <span title="unvalidated">*79.77%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 191.75 µs | <span title="unvalidated">*1.3196 ms\**</span> | 1290592 | 396547 | 340498 | 5.1370 ms |
| [alkahest 0.1.5][alkahest] | 211.11 µs | † | 667570 | 325484 | 320452 | 4.1109 ms |
| [bincode 2.0.0-rc][bincode] | 269.82 µs | 2.0742 ms | 367413 | 221291 | 206273 | 2.5760 ms |
| [bincode 1.3.3][bincode1] | 547.47 µs | 1.8141 ms | 569975 | 240525 | 232423 | 2.9861 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 134.04 µs | 1.2828 ms | 327688 | 200947 | 182736 | 763.35 µs |
| [borsh 1.3.0][borsh] | 539.62 µs | 1.8562 ms | 446595 | 234236 | 210008 | 2.6181 ms |
| [bson 2.9.0][bson] | 2.8920 ms | 8.3843 ms | 1619653 | 502185 | 328399 | 5.0039 ms |
| [capnp 0.18.13][capnp] | 448.83 µs | † | 803896 | 335606 | 280851 | 4.0582 ms |
| [cbor4ii 0.3.2][cbor4ii] | 795.02 µs | 4.8400 ms | 1109831 | 344745 | 274514 | 3.9724 ms |
| [ciborium 0.2.2][ciborium] | 3.6723 ms | 9.8424 ms | 1109821 | 344751 | 274526 | 3.9771 ms |
| [databuf 0.5.0][databuf] | 324.35 µs | 1.7410 ms | 356311 | 213062 | 198488 | 2.5078 ms |
| [dlhn 0.1.6][dlhn] | 761.13 µs | 2.5532 ms | 366496 | 220600 | 205683 | 2.5853 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.3376 ms | † | 844168 | 345696 | 294015 | 4.0921 ms |
| [msgpacker 0.4.3][msgpacker] | 911.30 µs | 2.7935 ms | 391251 | 236877 | 220476 | 2.7542 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2621 ms | 3.9358 ms | 449745 | 252432 | 231110 | 2.8836 ms |
| [nanoserde 0.1.37][nanoserde] | 289.02 µs | 1.8968 ms | 567975 | 239930 | 232419 | 3.0183 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 667.37 µs | 2.0213 ms | 356311 | 212976 | 198524 | 2.5938 ms |
| [postcard 1.0.8][postcard] | 428.56 µs | 2.0037 ms | 367489 | 221913 | 207344 | 2.6430 ms |
| [pot 3.0.0][pot] | 2.2859 ms | 5.8682 ms | 599125 | 299158 | 247693 | 3.3433 ms |
| [prost 0.12.3][prost] | <span title="encode">*1.0274 ms\**</span> <span title="populate + encode">*2.7074 ms\**</span> | 3.5766 ms | 596811 | 305319 | 269310 | 3.5721 ms |
| [rkyv 0.7.44][rkyv] | 297.33 µs | <span title="unvalidated">*1.2580 ms\**</span> <span title="validated upfront with error">*1.7772 ms\**</span> | 596952 | 253967 | 220706 | 2.8208 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.4109 ms | 2.9467 ms | 424533 | 245214 | 226188 | 2.8384 ms |
| [ron 0.8.1][ron] | 8.2322 ms | 17.180 ms | 1465223 | 434935 | 343338 | 6.0344 ms |
| [savefile 0.16.5][savefile] | 218.69 µs | 1.8321 ms | 566991 | 239361 | 232010 | 3.0049 ms |
| [serde_bare 0.5.0][serde_bare] | 698.86 µs | 2.2137 ms | 356311 | 213062 | 198488 | 2.5258 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8019 ms | 4.7464 ms | 1109821 | 344751 | 274526 | 3.9509 ms |
| [serde_json 1.0.114][serde_json] | 3.9496 ms | 6.9101 ms | 1623191 | 466527 | 359623 | 6.1992 ms |
| [simd-json 0.13.8][simd-json] | 2.2230 ms | 4.5655 ms | 1623191 | 466527 | 359623 | 6.1439 ms |
| [speedy 0.8.7][speedy] | 275.88 µs | 1.6493 ms | 449595 | 234970 | 210361 | 2.5689 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.479 µs\**</span> | <span title="unvalidated">*37.977 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8556 ns\**</span> | <span title="validated on-demand with panic">*4.6071 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.378 ns\**</span> | <span title="validated on-demand with error">*431.33 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4902 ns\**</span> <span title="validated upfront with error">*2.1620 ms\**</span> | <span title="unvalidated">*1.3714 µs\**</span> <span title="validated upfront with error">*2.1707 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2361 ns\**</span> <span title="validated upfront with error">*497.23 µs\**</span> | <span title="unvalidated">*163.38 ns\**</span> <span title="validated upfront with error">*499.06 µs\**</span> | 853.58 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 69.90% | <span title="unvalidated">*95.33%\**</span> | 25.39% | 50.67% | 53.67% | 14.86% |
| [alkahest 0.1.5][alkahest] | 63.49% | † | 49.09% | 61.74% | 57.02% | 18.57% |
| [bincode 2.0.0-rc][bincode] | 49.68% | 60.65% | 89.19% | 90.81% | 88.59% | 29.63% |
| [bincode 1.3.3][bincode1] | 24.48% | 69.35% | 57.49% | 83.55% | 78.62% | 25.56% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 98.07% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 24.84% | 67.77% | 73.37% | 85.79% | 87.01% | 29.16% |
| [bson 2.9.0][bson] | 4.63% | 15.00% | 20.23% | 40.01% | 55.64% | 15.26% |
| [capnp 0.18.13][capnp] | 29.86% | † | 40.76% | 59.88% | 65.07% | 18.81% |
| [cbor4ii 0.3.2][cbor4ii] | 16.86% | 25.99% | 29.53% | 58.29% | 66.57% | 19.22% |
| [ciborium 0.2.2][ciborium] | 3.65% | 12.78% | 29.53% | 58.29% | 66.56% | 19.19% |
| [databuf 0.5.0][databuf] | 41.33% | 72.26% | 91.97% | 94.31% | 92.06% | 30.44% |
| [dlhn 0.1.6][dlhn] | 17.61% | 49.27% | 89.41% | 91.09% | 88.84% | 29.53% |
| [flatbuffers 23.5.26][flatbuffers] | 4.02% | † | 38.82% | 58.13% | 62.15% | 18.65% |
| [msgpacker 0.4.3][msgpacker] | 14.71% | 45.03% | 83.75% | 84.83% | 82.88% | 27.72% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.55% | 31.96% | 72.86% | 79.60% | 79.07% | 26.47% |
| [nanoserde 0.1.37][nanoserde] | 46.38% | 66.32% | 57.69% | 83.75% | 78.62% | 25.29% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.08% | 62.24% | 91.97% | 94.35% | 92.05% | 29.43% |
| [postcard 1.0.8][postcard] | 31.28% | 62.78% | 89.17% | 90.55% | 88.13% | 28.88% |
| [pot 3.0.0][pot] | 5.86% | 21.44% | 54.69% | 67.17% | 73.78% | 22.83% |
| [prost 0.12.3][prost] | <span title="encode">*13.05%\**</span> <span title="populate + encode">*4.95%\**</span> | 35.17% | 54.91% | 65.82% | 67.85% | 21.37% |
| [rkyv 0.7.44][rkyv] | 45.08% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*70.79%\**</span> | 54.89% | 79.12% | 82.80% | 27.06% |
| [rmp-serde 1.1.2][rmp-serde] | 9.50% | 42.69% | 77.19% | 81.95% | 80.79% | 26.89% |
| [ron 0.8.1][ron] | 1.63% | 7.32% | 22.36% | 46.20% | 53.22% | 12.65% |
| [savefile 0.16.5][savefile] | 61.29% | 68.66% | 57.79% | 83.95% | 78.76% | 25.40% |
| [serde_bare 0.5.0][serde_bare] | 19.18% | 56.83% | 91.97% | 94.31% | 92.06% | 30.22% |
| [serde_cbor 0.11.2][serde_cbor] | 7.44% | 26.50% | 29.53% | 58.29% | 66.56% | 19.32% |
| [serde_json 1.0.114][serde_json] | 3.39% | 18.21% | 20.19% | 43.07% | 50.81% | 12.31% |
| [simd-json 0.13.8][simd-json] | 6.03% | 27.55% | 20.19% | 43.07% | 50.81% | 12.42% |
| [speedy 0.8.7][speedy] | 48.59% | 76.27% | 72.89% | 85.52% | 86.87% | 29.72% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.43%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.61%\**</span> | <span title="validated on-demand with panic">*3.55%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*37.88%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.64%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.91%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 483.46 µs | <span title="unvalidated">*2.3094 ms\**</span> | 2984682 | 1408201 | 1273808 | 15.523 ms |
| [alkahest 0.1.5][alkahest] | 734.23 µs | † | 1863391 | 1234113 | 1202345 | 12.244 ms |
| [bincode 2.0.0-rc][bincode] | 690.13 µs | 3.5470 ms | 1372381 | 1091486 | 1037296 | 9.6851 ms |
| [bincode 1.3.3][bincode1] | 3.5905 ms | 4.0160 ms | 1811011 | 1115281 | 1025627 | 10.547 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 722.63 µs | 2.3573 ms | 948499 | 857321 | 837658 | 3.1442 ms |
| [borsh 1.3.0][borsh] | 2.7150 ms | 2.7298 ms | 1486162 | 1082357 | 1013550 | 10.420 ms |
| [bson 2.9.0][bson] | 21.420 ms | 44.361 ms | 10030880 | 2833079 | 1600859 | 29.743 ms |
| [capnp 0.18.13][capnp] | 2.3431 ms | † | 2664040 | 1511895 | 1212087 | 15.127 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.2773 ms | 17.324 ms | 5878791 | 1655835 | 1431390 | 23.227 ms |
| [ciborium 0.2.2][ciborium] | 21.903 ms | 48.616 ms | 5878653 | 1655791 | 1431560 | 23.099 ms |
| [databuf 0.5.0][databuf] | 1.7787 ms | 3.6067 ms | 1288257 | 1037579 | 984337 | 9.2971 ms |
| [dlhn 0.1.6][dlhn] | 5.0315 ms | 7.1935 ms | 1279599 | 1052061 | 1021161 | 8.9799 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.0868 ms | † | 2273740 | 1408408 | 1235566 | 13.441 ms |
| [msgpacker 0.4.3][msgpacker] | 1.8835 ms | 4.5052 ms | 1424043 | 1128758 | 1110156 | 9.9427 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 31.229 ms | 15.696 ms | 1728519 | 1247642 | 1233323 | 12.633 ms |
| [nanoserde 0.1.37][nanoserde] | 1.5098 ms | 2.8932 ms | 1770477 | 1108304 | 1029947 | 10.398 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.9160 ms | 3.0130 ms | 1288257 | 1039269 | 986510 | 9.2078 ms |
| [postcard 1.0.8][postcard] | 1.7052 ms | 3.9328 ms | 1279599 | 1058243 | 1016738 | 9.0968 ms |
| [pot 3.0.0][pot] | 13.295 ms | 30.875 ms | 2544810 | 1447453 | 1268390 | 16.220 ms |
| [prost 0.12.3][prost] | <span title="encode">*4.0396 ms\**</span> <span title="populate + encode">*8.0749 ms\**</span> | 9.4783 ms | 1818378 | 1307777 | 1266311 | 12.532 ms |
| [rkyv 0.7.44][rkyv] | 1.1105 ms | <span title="unvalidated">*2.1748 ms\**</span> <span title="validated upfront with error">*2.7841 ms\**</span> | 2029080 | 1335117 | 1158855 | 13.118 ms |
| [rmp-serde 1.1.2][rmp-serde] | 10.263 ms | 11.597 ms | 1703813 | 1231892 | 1200208 | 11.902 ms |
| [ron 0.8.1][ron] | 36.916 ms | 96.221 ms | 8476284 | 2181196 | 1783971 | 35.686 ms |
| [savefile 0.16.5][savefile] | 1.0204 ms | 2.6746 ms | 1750226 | 1101682 | 1027827 | 10.598 ms |
| [serde_bare 0.5.0][serde_bare] | 4.6128 ms | 4.4455 ms | 1288257 | 1037597 | 984356 | 9.1322 ms |
| [serde_cbor 0.11.2][serde_cbor] | 8.9670 ms | 21.338 ms | 5878653 | 1655791 | 1431560 | 22.535 ms |
| [serde_json 1.0.114][serde_json] | 21.674 ms | 30.619 ms | 9175594 | 2334253 | 1800713 | 35.790 ms |
| [simd-json 0.13.8][simd-json] | 11.403 ms | 26.516 ms | 9175594 | 2334253 | 1800713 | 35.547 ms |
| [speedy 0.8.7][speedy] | 720.72 µs | 2.5170 ms | 1546963 | 1093532 | 1013443 | 10.242 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.424 µs\**</span> | <span title="unvalidated">*67.426 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8555 ns\**</span> | <span title="validated on-demand with panic">*627.05 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.094 ns\**</span> | <span title="validated on-demand with error">*708.18 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4733 ns\**</span> <span title="validated upfront with error">*4.8767 ms\**</span> | <span title="unvalidated">*2.6657 µs\**</span> <span title="validated upfront with error">*4.8753 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2372 ns\**</span> <span title="validated upfront with error">*619.55 µs\**</span> | <span title="unvalidated">*486.69 ns\**</span> <span title="validated upfront with error">*622.50 µs\**</span> | 504.02 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*94.17%\**</span> | 31.78% | 60.88% | 65.76% | 20.26% |
| [alkahest 0.1.5][alkahest] | 65.85% | † | 50.90% | 69.47% | 69.67% | 25.68% |
| [bincode 2.0.0-rc][bincode] | 70.05% | 61.31% | 69.11% | 78.55% | 80.75% | 32.46% |
| [bincode 1.3.3][bincode1] | 13.46% | 54.15% | 52.37% | 76.87% | 81.67% | 29.81% |
| [bitcode 0.6.0-alpha.2][bitcode] | 66.90% | 92.26% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 17.81% | 79.67% | 63.82% | 79.21% | 82.65% | 30.17% |
| [bson 2.9.0][bson] | 2.26% | 4.90% | 9.46% | 30.26% | 52.33% | 10.57% |
| [capnp 0.18.13][capnp] | 20.63% | † | 35.60% | 56.71% | 69.11% | 20.78% |
| [cbor4ii 0.3.2][cbor4ii] | 11.30% | 12.55% | 16.13% | 51.78% | 58.52% | 13.54% |
| [ciborium 0.2.2][ciborium] | 2.21% | 4.47% | 16.13% | 51.78% | 58.51% | 13.61% |
| [databuf 0.5.0][databuf] | 27.18% | 60.30% | 73.63% | 82.63% | 85.10% | 33.82% |
| [dlhn 0.1.6][dlhn] | 9.61% | 30.23% | 74.12% | 81.49% | 82.03% | 35.01% |
| [flatbuffers 23.5.26][flatbuffers] | 9.50% | † | 41.72% | 60.87% | 67.80% | 23.39% |
| [msgpacker 0.4.3][msgpacker] | 25.67% | 48.27% | 66.61% | 75.95% | 75.45% | 31.62% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.55% | 13.86% | 54.87% | 68.72% | 67.92% | 24.89% |
| [nanoserde 0.1.37][nanoserde] | 32.02% | 75.17% | 53.57% | 77.35% | 81.33% | 30.24% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 16.58% | 72.18% | 73.63% | 82.49% | 84.91% | 34.15% |
| [postcard 1.0.8][postcard] | 28.35% | 55.30% | 74.12% | 81.01% | 82.39% | 34.56% |
| [pot 3.0.0][pot] | 3.64% | 7.04% | 37.27% | 59.23% | 66.04% | 19.38% |
| [prost 0.12.3][prost] | <span title="encode">*11.97%\**</span> <span title="populate + encode">*5.99%\**</span> | 22.95% | 52.16% | 65.56% | 66.15% | 25.09% |
| [rkyv 0.7.44][rkyv] | 43.54% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.12%\**</span> | 46.75% | 64.21% | 72.28% | 23.97% |
| [rmp-serde 1.1.2][rmp-serde] | 4.71% | 18.75% | 55.67% | 69.59% | 69.79% | 26.42% |
| [ron 0.8.1][ron] | 1.31% | 2.26% | 11.19% | 39.31% | 46.95% | 8.81% |
| [savefile 0.16.5][savefile] | 47.38% | 81.31% | 54.19% | 77.82% | 81.50% | 29.67% |
| [serde_bare 0.5.0][serde_bare] | 10.48% | 48.92% | 73.63% | 82.63% | 85.10% | 34.43% |
| [serde_cbor 0.11.2][serde_cbor] | 5.39% | 10.19% | 16.13% | 51.78% | 58.51% | 13.95% |
| [serde_json 1.0.114][serde_json] | 2.23% | 7.10% | 10.34% | 36.73% | 46.52% | 8.79% |
| [simd-json 0.13.8][simd-json] | 4.24% | 8.20% | 10.34% | 36.73% | 46.52% | 8.85% |
| [speedy 0.8.7][speedy] | 67.08% | 86.40% | 61.31% | 78.40% | 82.65% | 30.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.72%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.68%\**</span> | <span title="validated on-demand with panic">*77.62%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.67%\**</span> | <span title="validated on-demand with error">*68.72%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.26%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
[simd-json]: https://crates.io/crates/simd-json/0.13.8
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
