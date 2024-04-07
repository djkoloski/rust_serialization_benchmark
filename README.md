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

## Last updated: 2024-4-7 14:28:32

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 402.96 µs | <span title="unvalidated">*1.4593 ms\**</span> | 1705800 | 520074 | 413538 | 6.8274 ms |
| [alkahest 0.1.5][alkahest] | 191.18 µs | † | 1045784 | 454157 | 389424 | 6.0073 ms |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*761.66 µs\**</span> <span title="only prepend">*780.23 µs\**</span> | 3.3904 ms | 874632 | 355446 | 311723 | 5.1134 ms |
| [bincode 2.0.0-rc][bincode] | 213.30 µs | 2.4723 ms | 741295 | 303944 | 257153 | 3.9549 ms |
| [bincode 1.3.3][bincode1] | 530.87 µs | 2.0061 ms | 1045784 | 373127 | 311761 | 5.8932 ms |
| [bitcode 0.6.0][bitcode] | 137.69 µs | 1.4911 ms | 703710 | 288826 | 229755 | 2.4264 ms |
| [borsh 1.3.1][borsh] | 542.72 µs | 2.2047 ms | 885780 | 362204 | 286514 | 4.5601 ms |
| [bson 2.9.0][bson] | 2.2411 ms | 7.0367 ms | 1924682 | 532821 | 376270 | 5.6968 ms |
| [capnp 0.18.13][capnp] | 481.20 µs | † | 1443216 | 513986 | 428649 | 6.8195 ms |
| [cbor4ii 0.3.2][cbor4ii] | 896.64 µs | 4.9515 ms | 1407835 | 403440 | 324081 | 4.9140 ms |
| [ciborium 0.2.2][ciborium] | 3.0851 ms | 10.223 ms | 1407835 | 403440 | 324081 | 4.8632 ms |
| [databuf 0.5.0][databuf] | 269.91 µs | 2.0371 ms | 765778 | 311715 | 264630 | 4.1036 ms |
| [dlhn 0.1.6][dlhn] | 671.72 µs | 2.6846 ms | 724953 | 301446 | 253629 | 3.8056 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.5352 ms | † | 1276368 | 468539 | 388832 | 5.3761 ms |
| [msgpacker 0.4.3][msgpacker] | 1.1885 ms | 2.5114 ms | 764996 | 315291 | 264898 | 4.1148 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3650 ms | 4.0899 ms | 818669 | 332556 | 285514 | 4.5837 ms |
| [nanoserde 0.1.37][nanoserde] | 277.56 µs | 2.0747 ms | 1045784 | 373127 | 311761 | 4.5741 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 672.47 µs | 2.2940 ms | 765778 | 311743 | 264518 | 4.0625 ms |
| [postcard 1.0.8][postcard] | 421.75 µs | 2.5910 ms | 724953 | 302399 | 253747 | 3.8115 ms |
| [pot 3.0.0][pot] | 2.1423 ms | 6.3546 ms | 971922 | 372513 | 304122 | 4.9019 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.0224 ms\**</span> <span title="populate + encode">*2.8980 ms\**</span> | 3.5491 ms | 884628 | 363130 | 315494 | 5.1134 ms |
| [rkyv 0.7.44][rkyv] | 217.51 µs | <span title="unvalidated">*1.4468 ms\**</span> <span title="validated upfront with error">*1.9747 ms\**</span> | 1011488 | 383862 | 333545 | 5.1856 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3918 ms | 3.4702 ms | 784997 | 325384 | 278219 | 4.4196 ms |
| [ron 0.8.1][ron] | 14.239 ms | 15.641 ms | 1607459 | 449158 | 349713 | 5.9292 ms |
| [savefile 0.16.5][savefile] | 202.68 µs | 2.1457 ms | 1045800 | 373139 | 311755 | 4.7364 ms |
| [serde_bare 0.5.0][serde_bare] | 662.84 µs | 2.1519 ms | 765778 | 311715 | 264630 | 3.9883 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9304 ms | 5.3978 ms | 1407835 | 403440 | 324081 | 4.8601 ms |
| [serde_json 1.0.115][serde_json] | 3.7809 ms | 6.1116 ms | 1827461 | 470560 | 361090 | 5.6385 ms |
| [simd-json 0.13.9][simd-json] | 2.0456 ms | 4.6390 ms | 1827461 | 470560 | 361090 | 5.9236 ms |
| [speedy 0.8.7][speedy] | 195.41 µs | 1.7472 ms | 885780 | 362204 | 286514 | 4.2463 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*23.016 µs\**</span> | <span title="unvalidated">*37.636 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8643 ns\**</span> | <span title="validated on-demand with panic">*25.096 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.045 ns\**</span> | <span title="validated on-demand with error">*180.56 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4756 ns\**</span> <span title="validated upfront with error">*1.8244 ms\**</span> | <span title="unvalidated">*51.181 µs\**</span> <span title="validated upfront with error">*1.8708 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2363 ns\**</span> <span title="validated upfront with error">*517.16 µs\**</span> | <span title="unvalidated">*10.664 µs\**</span> <span title="validated upfront with error">*532.84 µs\**</span> | 9.9219 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 34.17% | <span title="unvalidated">*99.14%\**</span> | 41.25% | 55.54% | 55.56% | 35.54% |
| [alkahest 0.1.5][alkahest] | 72.02% | † | 67.29% | 63.60% | 59.00% | 40.39% |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*18.08%\**</span> <span title="only prepend">*17.65%\**</span> | 42.67% | 80.46% | 81.26% | 73.70% | 47.45% |
| [bincode 2.0.0-rc][bincode] | 64.55% | 58.52% | 94.93% | 95.03% | 89.35% | 61.35% |
| [bincode 1.3.3][bincode1] | 25.94% | 72.12% | 67.29% | 77.41% | 73.70% | 41.17% |
| [bitcode 0.6.0][bitcode] | 100.00% | 97.03% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 25.37% | 65.62% | 79.45% | 79.74% | 80.19% | 53.21% |
| [bson 2.9.0][bson] | 6.14% | 20.56% | 36.56% | 54.21% | 61.06% | 42.59% |
| [capnp 0.18.13][capnp] | 28.61% | † | 48.76% | 56.19% | 53.60% | 35.58% |
| [cbor4ii 0.3.2][cbor4ii] | 15.36% | 29.22% | 49.99% | 71.59% | 70.89% | 49.38% |
| [ciborium 0.2.2][ciborium] | 4.46% | 14.15% | 49.99% | 71.59% | 70.89% | 49.89% |
| [databuf 0.5.0][databuf] | 51.01% | 71.02% | 91.89% | 92.66% | 86.82% | 59.13% |
| [dlhn 0.1.6][dlhn] | 20.50% | 53.89% | 97.07% | 95.81% | 90.59% | 63.76% |
| [flatbuffers 23.5.26][flatbuffers] | 8.97% | † | 55.13% | 61.64% | 59.09% | 45.13% |
| [msgpacker 0.4.3][msgpacker] | 11.59% | 57.61% | 91.99% | 91.61% | 86.73% | 58.97% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.57% | 35.37% | 85.96% | 86.85% | 80.47% | 52.94% |
| [nanoserde 0.1.37][nanoserde] | 49.61% | 69.74% | 67.29% | 77.41% | 73.70% | 53.05% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.48% | 63.07% | 91.89% | 92.65% | 86.86% | 59.73% |
| [postcard 1.0.8][postcard] | 32.65% | 55.84% | 97.07% | 95.51% | 90.54% | 63.66% |
| [pot 3.0.0][pot] | 6.43% | 22.77% | 72.40% | 77.53% | 75.55% | 49.50% |
| [prost 0.12.4][prost] | <span title="encode">*13.47%\**</span> <span title="populate + encode">*4.75%\**</span> | 40.77% | 79.55% | 79.54% | 72.82% | 47.45% |
| [rkyv 0.7.44][rkyv] | 63.30% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.27%\**</span> | 69.57% | 75.24% | 68.88% | 46.79% |
| [rmp-serde 1.1.2][rmp-serde] | 9.89% | 41.69% | 89.64% | 88.76% | 82.58% | 54.90% |
| [ron 0.8.1][ron] | 0.97% | 9.25% | 43.78% | 64.30% | 65.70% | 40.92% |
| [savefile 0.16.5][savefile] | 67.93% | 67.43% | 67.29% | 77.40% | 73.70% | 51.23% |
| [serde_bare 0.5.0][serde_bare] | 20.77% | 67.23% | 91.89% | 92.66% | 86.82% | 60.84% |
| [serde_cbor 0.11.2][serde_cbor] | 7.13% | 26.80% | 49.99% | 71.59% | 70.89% | 49.92% |
| [serde_json 1.0.115][serde_json] | 3.64% | 23.67% | 38.51% | 61.38% | 63.63% | 43.03% |
| [simd-json 0.13.9][simd-json] | 6.73% | 31.19% | 38.51% | 61.38% | 63.63% | 40.96% |
| [speedy 0.8.7][speedy] | 70.46% | 82.81% | 79.45% | 79.74% | 80.19% | 57.14% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*28.33%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.31%\**</span> | <span title="validated on-demand with panic">*42.49%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.67%\**</span> | <span title="validated on-demand with error">*5.91%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.94%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.84%\**</span> <span title="validated upfront with error">*0.57%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.00%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 259.83 µs | <span title="unvalidated">*259.00 µs\**</span> | 6000024 | 5378513 | 5345891 | 7.3332 ms |
| [alkahest 0.1.5][alkahest] | 149.05 µs | † | 6000008 | 5378500 | 5345890 | 7.2940 ms |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*6.5801 ms\**</span> <span title="only prepend">*8.3179 ms\**</span> | 8.5269 ms | 8625005 | 6443961 | 6231572 | 69.369 ms |
| [bincode 2.0.0-rc][bincode] | 422.70 µs | 828.59 µs | 6000005 | 5378497 | 5345897 | 7.5391 ms |
| [bincode 1.3.3][bincode1] | 5.1139 ms | 4.5729 ms | 6000008 | 5378500 | 5345890 | 7.3091 ms |
| [bitcode 0.6.0][bitcode] | 1.3929 ms | 599.62 µs | 6000006 | 5182295 | 4923880 | 12.721 ms |
| [borsh 1.3.1][borsh] | 5.8044 ms | 4.2213 ms | 6000004 | 5378496 | 5345889 | 7.4529 ms |
| [bson 2.9.0][bson] | 44.744 ms | 78.496 ms | 23013911 | 9212089 | 7497811 | 105.21 ms |
| [capnp 0.18.13][capnp] | 5.3286 ms | † | 14000088 | 7130367 | 6051062 | 77.819 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.136 ms | 48.725 ms | 13125016 | 7524114 | 6757967 | 90.065 ms |
| [ciborium 0.2.2][ciborium] | 68.140 ms | 99.626 ms | 13122324 | 7524660 | 6759658 | 90.191 ms |
| [databuf 0.5.0][databuf] | 2.4094 ms | 5.3071 ms | 6000003 | 5378495 | 5345900 | 7.5754 ms |
| [dlhn 0.1.6][dlhn] | 5.8293 ms | 5.8127 ms | 6000003 | 5378495 | 5345900 | 7.5363 ms |
| [flatbuffers 23.5.26][flatbuffers] | 649.88 µs | † | 6000024 | 5378434 | 5345910 | 7.4034 ms |
| [msgpacker 0.4.3][msgpacker] | 19.365 ms | 8.8002 ms | 7500005 | 6058442 | 6014337 | 10.936 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 123.63 ms | 27.631 ms | 8125037 | 6493484 | 6386940 | 72.517 ms |
| [nanoserde 0.1.37][nanoserde] | 1.1524 ms | 902.77 µs | 6000008 | 5378500 | 5345890 | 7.5819 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 5.1512 ms | 4.5426 ms | 6000004 | 5378496 | 5345889 | 7.4843 ms |
| [postcard 1.0.8][postcard] | 509.79 µs | 1.1899 ms | 6000003 | 5378495 | 5345900 | 7.5652 ms |
| [pot 3.0.0][pot] | 37.484 ms | 72.840 ms | 10122342 | 6814618 | 6852251 | 80.309 ms |
| [prost 0.12.4][prost] | <span title="encode">*7.8689 ms\**</span> <span title="populate + encode">*9.1034 ms\**</span> | 13.398 ms | 8750000 | 6665735 | 6421871 | 73.804 ms |
| [rkyv 0.7.44][rkyv] | 186.65 µs | <span title="unvalidated">*149.00 µs\**</span> <span title="validated upfront with error">*148.97 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.4762 ms |
| [rmp-serde 1.1.2][rmp-serde] | 13.485 ms | 18.732 ms | 8125006 | 6494876 | 6391037 | 71.288 ms |
| [ron 0.8.1][ron] | 169.76 ms | 261.26 ms | 22192885 | 8970395 | 8138755 | 146.44 ms |
| [savefile 0.16.5][savefile] | 259.89 µs | 259.70 µs | 6000024 | 5378518 | 5345893 | 7.7590 ms |
| [serde_bare 0.5.0][serde_bare] | 6.1900 ms | 4.4773 ms | 6000003 | 5378495 | 5345900 | 7.6175 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.272 ms | 44.532 ms | 13122324 | 7524660 | 6759658 | 90.173 ms |
| [serde_json 1.0.115][serde_json] | 90.423 ms | 86.824 ms | 26192883 | 9566084 | 8586741 | 152.69 ms |
| [simd-json 0.13.9][simd-json] | 54.656 ms | 73.978 ms | 26192883 | 9566084 | 8586741 | 153.47 ms |
| [speedy 0.8.7][speedy] | 259.66 µs | 260.50 µs | 6000004 | 5378496 | 5345889 | 7.4683 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1645 ns\**</span> | <span title="unvalidated">*141.04 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8588 ns\**</span> | <span title="validated on-demand with panic">*77.314 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*105.31 ns\**</span> | <span title="validated on-demand with error">*2.1412 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4749 ns\**</span> <span title="validated upfront with error">*38.409 ns\**</span> | <span title="unvalidated">*53.934 µs\**</span> <span title="validated upfront with error">*77.396 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2364 ns\**</span> <span title="validated upfront with error">*10.838 ns\**</span> | <span title="unvalidated">*46.100 µs\**</span> <span title="validated upfront with error">*38.773 µs\**</span> | 104.73 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 57.36% | <span title="unvalidated">*57.52%\**</span> | 100.00% | 96.35% | 92.11% | 99.47% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 100.00% |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*2.27%\**</span> <span title="only prepend">*1.79%\**</span> | 1.75% | 69.57% | 80.42% | 79.02% | 10.51% |
| [bincode 2.0.0-rc][bincode] | 35.26% | 17.98% | 100.00% | 96.35% | 92.11% | 96.75% |
| [bincode 1.3.3][bincode1] | 2.91% | 3.26% | 100.00% | 96.35% | 92.11% | 99.79% |
| [bitcode 0.6.0][bitcode] | 10.70% | 24.84% | 100.00% | 100.00% | 100.00% | 57.34% |
| [borsh 1.3.1][borsh] | 2.57% | 3.53% | 100.00% | 96.35% | 92.11% | 97.87% |
| [bson 2.9.0][bson] | 0.33% | 0.19% | 26.07% | 56.26% | 65.67% | 6.93% |
| [capnp 0.18.13][capnp] | 2.80% | † | 42.86% | 72.68% | 81.37% | 9.37% |
| [cbor4ii 0.3.2][cbor4ii] | 1.47% | 0.31% | 45.71% | 68.88% | 72.86% | 8.10% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.15% | 45.72% | 68.87% | 72.84% | 8.09% |
| [databuf 0.5.0][databuf] | 6.19% | 2.81% | 100.00% | 96.35% | 92.11% | 96.29% |
| [dlhn 0.1.6][dlhn] | 2.56% | 2.56% | 100.00% | 96.35% | 92.11% | 96.78% |
| [flatbuffers 23.5.26][flatbuffers] | 22.94% | † | 100.00% | 96.35% | 92.11% | 98.52% |
| [msgpacker 0.4.3][msgpacker] | 0.77% | 1.69% | 80.00% | 85.54% | 81.87% | 66.70% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.54% | 73.85% | 79.81% | 77.09% | 10.06% |
| [nanoserde 0.1.37][nanoserde] | 12.93% | 16.50% | 100.00% | 96.35% | 92.11% | 96.20% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 2.89% | 3.28% | 100.00% | 96.35% | 92.11% | 97.46% |
| [postcard 1.0.8][postcard] | 29.24% | 12.52% | 100.00% | 96.35% | 92.11% | 96.42% |
| [pot 3.0.0][pot] | 0.40% | 0.20% | 59.27% | 76.05% | 71.86% | 9.08% |
| [prost 0.12.4][prost] | <span title="encode">*1.89%\**</span> <span title="populate + encode">*1.64%\**</span> | 1.11% | 68.57% | 77.75% | 76.67% | 9.88% |
| [rkyv 0.7.44][rkyv] | 79.86% | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 97.56% |
| [rmp-serde 1.1.2][rmp-serde] | 1.11% | 0.80% | 73.85% | 79.79% | 77.04% | 10.23% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 4.98% |
| [savefile 0.16.5][savefile] | 57.35% | 57.36% | 100.00% | 96.35% | 92.11% | 94.01% |
| [serde_bare 0.5.0][serde_bare] | 2.41% | 3.33% | 100.00% | 96.35% | 92.11% | 95.75% |
| [serde_cbor 0.11.2][serde_cbor] | 0.42% | 0.33% | 45.72% | 68.87% | 72.84% | 8.09% |
| [serde_json 1.0.115][serde_json] | 0.16% | 0.17% | 22.91% | 54.17% | 57.34% | 4.78% |
| [simd-json 0.13.9][simd-json] | 0.27% | 0.20% | 22.91% | 54.17% | 57.34% | 4.75% |
| [speedy 0.8.7][speedy] | 57.40% | 57.19% | 100.00% | 96.35% | 92.11% | 97.67% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.12%\**</span> | <span title="unvalidated">*27.49%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.52%\**</span> | <span title="validated on-demand with panic">*50.15%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.17%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*3.22%\**</span> | <span title="unvalidated">*71.89%\**</span> <span title="validated upfront with error">*50.10%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*11.41%\**</span> | <span title="unvalidated">*84.11%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 191.35 µs | <span title="unvalidated">*1.3078 ms\**</span> | 1290592 | 397110 | 340555 | 4.9572 ms |
| [alkahest 0.1.5][alkahest] | 218.62 µs | † | 667570 | 325484 | 320452 | 3.8978 ms |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*1.0035 ms\**</span> <span title="only prepend">*846.81 µs\**</span> <span title="populate + encode">*2.3340 ms\**</span> <span title="populate + prepend">*2.1692 ms\**</span> | 3.2573 ms | 476547 | 281071 | 249907 | 3.0928 ms |
| [bincode 2.0.0-rc][bincode] | 263.49 µs | 2.0889 ms | 367413 | 221291 | 206273 | 2.4768 ms |
| [bincode 1.3.3][bincode1] | 571.59 µs | 1.7816 ms | 569975 | 240525 | 232423 | 2.8815 ms |
| [bitcode 0.6.0][bitcode] | 133.28 µs | 1.2602 ms | 327688 | 200947 | 182736 | 762.35 µs |
| [borsh 1.3.1][borsh] | 517.92 µs | 1.8143 ms | 446595 | 234236 | 210008 | 2.4771 ms |
| [bson 2.9.0][bson] | 2.9284 ms | 8.2584 ms | 1619653 | 502185 | 328399 | 4.8452 ms |
| [capnp 0.18.13][capnp] | 467.66 µs | † | 803896 | 335606 | 280851 | 3.9273 ms |
| [cbor4ii 0.3.2][cbor4ii] | 788.59 µs | 4.7056 ms | 1109831 | 344745 | 274514 | 3.8564 ms |
| [ciborium 0.2.2][ciborium] | 3.6524 ms | 9.4470 ms | 1109821 | 344751 | 274526 | 3.8408 ms |
| [databuf 0.5.0][databuf] | 320.54 µs | 1.7237 ms | 356311 | 213062 | 198488 | 2.4074 ms |
| [dlhn 0.1.6][dlhn] | 704.58 µs | 2.5204 ms | 366496 | 220600 | 205683 | 2.4837 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.3019 ms | † | 844168 | 345696 | 294015 | 3.8365 ms |
| [msgpacker 0.4.3][msgpacker] | 910.66 µs | 2.8207 ms | 391251 | 236877 | 220476 | 2.6342 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2190 ms | 3.9199 ms | 449745 | 252432 | 231110 | 2.7544 ms |
| [nanoserde 0.1.37][nanoserde] | 302.54 µs | 1.8841 ms | 567975 | 239930 | 232419 | 2.8813 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 653.55 µs | 1.9830 ms | 356311 | 212976 | 198524 | 2.4035 ms |
| [postcard 1.0.8][postcard] | 436.78 µs | 1.9493 ms | 367489 | 221913 | 207344 | 2.4706 ms |
| [pot 3.0.0][pot] | 2.3118 ms | 5.9449 ms | 599125 | 299158 | 247693 | 3.1828 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.3479 ms\**</span> <span title="populate + encode">*3.0081 ms\**</span> | 3.7878 ms | 596811 | 305319 | 269310 | 3.4726 ms |
| [rkyv 0.7.44][rkyv] | 298.95 µs | <span title="unvalidated">*1.2510 ms\**</span> <span title="validated upfront with error">*1.7710 ms\**</span> | 596952 | 253967 | 220706 | 2.7182 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.4461 ms | 2.9553 ms | 424533 | 245214 | 226188 | 2.7076 ms |
| [ron 0.8.1][ron] | 8.1872 ms | 17.696 ms | 1465223 | 434935 | 343338 | 5.9209 ms |
| [savefile 0.16.5][savefile] | 218.00 µs | 1.8239 ms | 566991 | 239361 | 232010 | 2.8721 ms |
| [serde_bare 0.5.0][serde_bare] | 692.78 µs | 2.2260 ms | 356311 | 213062 | 198488 | 2.3799 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7826 ms | 4.7499 ms | 1109821 | 344751 | 274526 | 3.8136 ms |
| [serde_json 1.0.115][serde_json] | 3.6023 ms | 7.0178 ms | 1623191 | 466527 | 359623 | 6.0277 ms |
| [simd-json 0.13.9][simd-json] | 2.2016 ms | 4.5320 ms | 1623191 | 466527 | 359623 | 6.0305 ms |
| [speedy 0.8.7][speedy] | 274.03 µs | 1.6383 ms | 449595 | 234970 | 210361 | 2.4903 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.056 µs\**</span> | <span title="unvalidated">*37.686 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8554 ns\**</span> | <span title="validated on-demand with panic">*4.6045 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*73.422 ns\**</span> | <span title="validated on-demand with error">*431.99 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4733 ns\**</span> <span title="validated upfront with error">*1.8736 ms\**</span> | <span title="unvalidated">*1.3801 µs\**</span> <span title="validated upfront with error">*1.8613 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2368 ns\**</span> <span title="validated upfront with error">*509.01 µs\**</span> | <span title="unvalidated">*163.30 ns\**</span> <span title="validated upfront with error">*510.68 µs\**</span> | 917.49 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 69.65% | <span title="unvalidated">*95.66%\**</span> | 25.39% | 50.60% | 53.66% | 15.38% |
| [alkahest 0.1.5][alkahest] | 60.96% | † | 49.09% | 61.74% | 57.02% | 19.56% |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*13.28%\**</span> <span title="only prepend">*15.74%\**</span> <span title="populate + encode">*5.71%\**</span> <span title="populate + prepend">*6.14%\**</span> | 38.41% | 68.76% | 71.49% | 73.12% | 24.65% |
| [bincode 2.0.0-rc][bincode] | 50.58% | 59.89% | 89.19% | 90.81% | 88.59% | 30.78% |
| [bincode 1.3.3][bincode1] | 23.32% | 70.22% | 57.49% | 83.55% | 78.62% | 26.46% |
| [bitcode 0.6.0][bitcode] | 100.00% | 99.27% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 25.73% | 68.95% | 73.37% | 85.79% | 87.01% | 30.78% |
| [bson 2.9.0][bson] | 4.55% | 15.15% | 20.23% | 40.01% | 55.64% | 15.73% |
| [capnp 0.18.13][capnp] | 28.50% | † | 40.76% | 59.88% | 65.07% | 19.41% |
| [cbor4ii 0.3.2][cbor4ii] | 16.90% | 26.59% | 29.53% | 58.29% | 66.57% | 19.77% |
| [ciborium 0.2.2][ciborium] | 3.65% | 13.24% | 29.53% | 58.29% | 66.56% | 19.85% |
| [databuf 0.5.0][databuf] | 41.58% | 72.58% | 91.97% | 94.31% | 92.06% | 31.67% |
| [dlhn 0.1.6][dlhn] | 18.92% | 49.63% | 89.41% | 91.09% | 88.84% | 30.69% |
| [flatbuffers 23.5.26][flatbuffers] | 4.04% | † | 38.82% | 58.13% | 62.15% | 19.87% |
| [msgpacker 0.4.3][msgpacker] | 14.64% | 44.35% | 83.75% | 84.83% | 82.88% | 28.94% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.55% | 31.91% | 72.86% | 79.60% | 79.07% | 27.68% |
| [nanoserde 0.1.37][nanoserde] | 44.05% | 66.40% | 57.69% | 83.75% | 78.62% | 26.46% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.39% | 63.09% | 91.97% | 94.35% | 92.05% | 31.72% |
| [postcard 1.0.8][postcard] | 30.51% | 64.18% | 89.17% | 90.55% | 88.13% | 30.86% |
| [pot 3.0.0][pot] | 5.77% | 21.04% | 54.69% | 67.17% | 73.78% | 23.95% |
| [prost 0.12.4][prost] | <span title="encode">*9.89%\**</span> <span title="populate + encode">*4.43%\**</span> | 33.03% | 54.91% | 65.82% | 67.85% | 21.95% |
| [rkyv 0.7.44][rkyv] | 44.58% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*70.64%\**</span> | 54.89% | 79.12% | 82.80% | 28.05% |
| [rmp-serde 1.1.2][rmp-serde] | 9.22% | 42.33% | 77.19% | 81.95% | 80.79% | 28.16% |
| [ron 0.8.1][ron] | 1.63% | 7.07% | 22.36% | 46.20% | 53.22% | 12.88% |
| [savefile 0.16.5][savefile] | 61.14% | 68.59% | 57.79% | 83.95% | 78.76% | 26.54% |
| [serde_bare 0.5.0][serde_bare] | 19.24% | 56.20% | 91.97% | 94.31% | 92.06% | 32.03% |
| [serde_cbor 0.11.2][serde_cbor] | 7.48% | 26.34% | 29.53% | 58.29% | 66.56% | 19.99% |
| [serde_json 1.0.115][serde_json] | 3.70% | 17.83% | 20.19% | 43.07% | 50.81% | 12.65% |
| [simd-json 0.13.9][simd-json] | 6.05% | 27.60% | 20.19% | 43.07% | 50.81% | 12.64% |
| [speedy 0.8.7][speedy] | 48.64% | 76.36% | 72.89% | 85.52% | 86.87% | 30.61% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.43%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*3.55%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*37.80%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.83%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 476.00 µs | <span title="unvalidated">*2.3251 ms\**</span> | 2984682 | 1406950 | 1270102 | 14.323 ms |
| [alkahest 0.1.5][alkahest] | 725.88 µs | † | 1863391 | 1234113 | 1202345 | 11.405 ms |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*4.6975 ms\**</span> <span title="only prepend">*2.8062 ms\**</span> <span title="populate + encode">*6.9342 ms\**</span> <span title="populate + prepend">*5.0564 ms\**</span> | 8.5785 ms | 1664428 | 1263911 | 1215841 | 11.116 ms |
| [bincode 2.0.0-rc][bincode] | 699.62 µs | 3.5940 ms | 1372381 | 1091486 | 1037296 | 8.9488 ms |
| [bincode 1.3.3][bincode1] | 3.6935 ms | 3.8390 ms | 1811011 | 1115281 | 1025627 | 9.7508 ms |
| [bitcode 0.6.0][bitcode] | 708.36 µs | 2.3352 ms | 948499 | 857321 | 837658 | 3.0349 ms |
| [borsh 1.3.1][borsh] | 2.8860 ms | 2.7953 ms | 1486162 | 1082357 | 1013550 | 9.4599 ms |
| [bson 2.9.0][bson] | 20.550 ms | 44.280 ms | 10030880 | 2833079 | 1600859 | 27.813 ms |
| [capnp 0.18.13][capnp] | 2.2267 ms | † | 2664040 | 1511895 | 1212087 | 14.072 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.2774 ms | 17.679 ms | 5878791 | 1655835 | 1431390 | 20.687 ms |
| [ciborium 0.2.2][ciborium] | 23.115 ms | 46.868 ms | 5878653 | 1655791 | 1431560 | 20.684 ms |
| [databuf 0.5.0][databuf] | 1.7739 ms | 3.5457 ms | 1288257 | 1037579 | 984337 | 8.4537 ms |
| [dlhn 0.1.6][dlhn] | 4.8602 ms | 6.1547 ms | 1279599 | 1052061 | 1021161 | 8.1253 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.2511 ms | † | 2273740 | 1408408 | 1235566 | 12.844 ms |
| [msgpacker 0.4.3][msgpacker] | 1.8888 ms | 4.5474 ms | 1424043 | 1128758 | 1110156 | 9.2785 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.352 ms | 15.602 ms | 1728519 | 1247642 | 1233323 | 11.592 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2913 ms | 2.8777 ms | 1770477 | 1108304 | 1029947 | 9.7478 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.1238 ms | 3.0075 ms | 1288257 | 1039269 | 986510 | 8.4642 ms |
| [postcard 1.0.8][postcard] | 1.6824 ms | 3.9313 ms | 1279599 | 1058243 | 1016738 | 8.5145 ms |
| [pot 3.0.0][pot] | 13.476 ms | 31.012 ms | 2544810 | 1447453 | 1268390 | 15.222 ms |
| [prost 0.12.4][prost] | <span title="encode">*5.8886 ms\**</span> <span title="populate + encode">*9.9379 ms\**</span> | 9.3706 ms | 1818378 | 1307777 | 1266311 | 11.807 ms |
| [rkyv 0.7.44][rkyv] | 1.2958 ms | <span title="unvalidated">*2.1544 ms\**</span> <span title="validated upfront with error">*2.7555 ms\**</span> | 2029080 | 1335117 | 1158855 | 11.560 ms |
| [rmp-serde 1.1.2][rmp-serde] | 10.332 ms | 11.340 ms | 1703813 | 1231892 | 1200208 | 10.707 ms |
| [ron 0.8.1][ron] | 36.125 ms | 97.972 ms | 8476284 | 2181196 | 1783971 | 33.241 ms |
| [savefile 0.16.5][savefile] | 1.0036 ms | 2.6188 ms | 1750226 | 1101682 | 1027827 | 9.6549 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8980 ms | 4.4374 ms | 1288257 | 1037597 | 984356 | 8.4519 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.1518 ms | 21.721 ms | 5878653 | 1655791 | 1431560 | 20.668 ms |
| [serde_json 1.0.115][serde_json] | 20.345 ms | 31.051 ms | 9175594 | 2334253 | 1800713 | 33.232 ms |
| [simd-json 0.13.9][simd-json] | 11.272 ms | 25.926 ms | 9175594 | 2334253 | 1800713 | 33.326 ms |
| [speedy 0.8.7][speedy] | 708.37 µs | 2.4845 ms | 1546963 | 1093532 | 1013443 | 9.5921 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*65.648 µs\**</span> | <span title="unvalidated">*67.086 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8549 ns\**</span> | <span title="validated on-demand with panic">*626.53 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.624 ns\**</span> | <span title="validated on-demand with error">*1.0170 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4749 ns\**</span> <span title="validated upfront with error">*3.8619 ms\**</span> | <span title="unvalidated">*2.6563 µs\**</span> <span title="validated upfront with error">*3.8669 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*597.16 µs\**</span> | <span title="unvalidated">*487.32 ns\**</span> <span title="validated upfront with error">*600.81 µs\**</span> | 501.71 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.66%\**</span> | 31.78% | 60.93% | 65.95% | 21.19% |
| [alkahest 0.1.5][alkahest] | 65.58% | † | 50.90% | 69.47% | 69.67% | 26.61% |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*10.13%\**</span> <span title="only prepend">*16.96%\**</span> <span title="populate + encode">*6.86%\**</span> <span title="populate + prepend">*9.41%\**</span> | 25.11% | 56.99% | 67.83% | 68.90% | 27.30% |
| [bincode 2.0.0-rc][bincode] | 68.04% | 59.94% | 69.11% | 78.55% | 80.75% | 33.91% |
| [bincode 1.3.3][bincode1] | 12.89% | 56.12% | 52.37% | 76.87% | 81.67% | 31.12% |
| [bitcode 0.6.0][bitcode] | 67.20% | 92.26% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 16.49% | 77.07% | 63.82% | 79.21% | 82.65% | 32.08% |
| [bson 2.9.0][bson] | 2.32% | 4.87% | 9.46% | 30.26% | 52.33% | 10.91% |
| [capnp 0.18.13][capnp] | 21.38% | † | 35.60% | 56.71% | 69.11% | 21.57% |
| [cbor4ii 0.3.2][cbor4ii] | 11.13% | 12.19% | 16.13% | 51.78% | 58.52% | 14.67% |
| [ciborium 0.2.2][ciborium] | 2.06% | 4.60% | 16.13% | 51.78% | 58.51% | 14.67% |
| [databuf 0.5.0][databuf] | 26.83% | 60.76% | 73.63% | 82.63% | 85.10% | 35.90% |
| [dlhn 0.1.6][dlhn] | 9.79% | 35.00% | 74.12% | 81.49% | 82.03% | 37.35% |
| [flatbuffers 23.5.26][flatbuffers] | 9.06% | † | 41.72% | 60.87% | 67.80% | 23.63% |
| [msgpacker 0.4.3][msgpacker] | 25.20% | 47.38% | 66.61% | 75.95% | 75.45% | 32.71% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.57% | 13.81% | 54.87% | 68.72% | 67.92% | 26.18% |
| [nanoserde 0.1.37][nanoserde] | 36.86% | 74.87% | 53.57% | 77.35% | 81.33% | 31.13% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 15.24% | 71.63% | 73.63% | 82.49% | 84.91% | 35.86% |
| [postcard 1.0.8][postcard] | 28.29% | 54.80% | 74.12% | 81.01% | 82.39% | 35.64% |
| [pot 3.0.0][pot] | 3.53% | 6.95% | 37.27% | 59.23% | 66.04% | 19.94% |
| [prost 0.12.4][prost] | <span title="encode">*8.08%\**</span> <span title="populate + encode">*4.79%\**</span> | 22.99% | 52.16% | 65.56% | 66.15% | 25.71% |
| [rkyv 0.7.44][rkyv] | 36.73% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.19%\**</span> | 46.75% | 64.21% | 72.28% | 26.25% |
| [rmp-serde 1.1.2][rmp-serde] | 4.61% | 19.00% | 55.67% | 69.59% | 69.79% | 28.35% |
| [ron 0.8.1][ron] | 1.32% | 2.20% | 11.19% | 39.31% | 46.95% | 9.13% |
| [savefile 0.16.5][savefile] | 47.43% | 82.27% | 54.19% | 77.82% | 81.50% | 31.43% |
| [serde_bare 0.5.0][serde_bare] | 9.72% | 48.55% | 73.63% | 82.63% | 85.10% | 35.91% |
| [serde_cbor 0.11.2][serde_cbor] | 5.20% | 9.92% | 16.13% | 51.78% | 58.51% | 14.68% |
| [serde_json 1.0.115][serde_json] | 2.34% | 6.94% | 10.34% | 36.73% | 46.52% | 9.13% |
| [simd-json 0.13.9][simd-json] | 4.22% | 8.31% | 10.34% | 36.73% | 46.52% | 9.11% |
| [speedy 0.8.7][speedy] | 67.20% | 86.71% | 61.31% | 78.40% | 82.65% | 31.64% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.73%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.69%\**</span> | <span title="validated on-demand with panic">*77.78%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*47.92%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.35%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.08%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bilrost]: https://crates.io/crates/bilrost/0.1005.1
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.0
[borsh]: https://crates.io/crates/borsh/1.3.1
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
[prost]: https://crates.io/crates/prost/0.12.4
[rkyv]: https://crates.io/crates/rkyv/0.7.44
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.16.5
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.115
[simd-json]: https://crates.io/crates/simd-json/0.13.9
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
