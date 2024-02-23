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

## Last updated: 2024-2-22 22:34:37

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 411.01 µs | <span title="unvalidated">*1.4565 ms\**</span> | 1705800 | 519588 | 412910 |
| [alkahest 0.1.5][alkahest] | 197.27 µs | † | 1045784 | 454157 | 389424 |
| [bincode 2.0.0-rc][bincode] | 209.67 µs | 2.4296 ms | 741295 | 303944 | 257153 |
| [bincode 1.3.3][bincode1] | 522.13 µs | 1.9889 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.6.0][bitcode] | 145.10 µs | 1.4437 ms | 703715 | 288827 | 229804 |
| [borsh 1.3.0][borsh] | 541.77 µs | 2.1543 ms | 885780 | 362204 | 286514 |
| [bson 2.9.0][bson] | 2.2017 ms | 7.4528 ms | 1924682 | 532821 | 376270 |
| [capnp 0.18.13][capnp] | 745.82 µs | † | 1443216 | 513986 | 428649 |
| [cbor4ii 0.3.2][cbor4ii] | 904.98 µs | 5.0319 ms | 1407835 | 403440 | 324081 |
| [ciborium 0.2.2][ciborium] | 3.9051 ms | 10.784 ms | 1407835 | 403440 | 324081 |
| [databuf 0.5.0][databuf] | 272.74 µs | 2.0220 ms | 765778 | 311715 | 264630 |
| [dlhn 0.1.6][dlhn] | 789.74 µs | 2.4125 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.3883 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.1675 ms | 2.5357 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4301 ms | 4.1529 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.37][nanoserde] | 254.00 µs | 2.0513 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 653.57 µs | 2.2298 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.8][postcard] | 416.84 µs | 2.1807 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 2.3574 ms | 6.5066 ms | 971922 | 372513 | 304122 |
| [prost 0.12.3][prost] | <span title="encode">*802.91 µs\**</span> <span title="populate + encode">*2.3397 ms\**</span> | 3.4954 ms | 884628 | 363130 | 315494 |
| [rkyv 0.7.44][rkyv] | 217.39 µs | <span title="unvalidated">*1.4457 ms\**</span> <span title="validated upfront with error">*1.9934 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.3948 ms | 3.5021 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 14.772 ms | 15.912 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.5][savefile] | 204.34 µs | 2.0689 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 655.76 µs | 2.1066 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 1.9490 ms | 5.0221 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.114][serde_json] | 3.8468 ms | 5.6240 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.13.8][simd-json] | 2.1389 ms | 5.0039 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.7][speedy] | 196.93 µs | 1.7488 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.123 µs\**</span> | <span title="unvalidated">*37.656 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8558 ns\**</span> | <span title="validated on-demand with panic">*24.792 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.441 ns\**</span> | <span title="validated on-demand with error">*168.36 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4732 ns\**</span> <span title="validated upfront with error">*1.8498 ms\**</span> | <span title="unvalidated">*48.731 µs\**</span> <span title="validated upfront with error">*1.9159 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2367 ns\**</span> <span title="validated upfront with error">*536.21 µs\**</span> | <span title="unvalidated">*10.716 µs\**</span> <span title="validated upfront with error">*551.07 µs\**</span> | 9.8891 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 35.30% | <span title="unvalidated">*99.12%\**</span> | 41.25% | 55.59% | 55.65% |
| [alkahest 0.1.5][alkahest] | 73.55% | † | 67.29% | 63.60% | 59.01% |
| [bincode 2.0.0-rc][bincode] | 69.20% | 59.42% | 94.93% | 95.03% | 89.36% |
| [bincode 1.3.3][bincode1] | 27.79% | 72.59% | 67.29% | 77.41% | 73.71% |
| [bitcode 0.6.0][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 26.78% | 67.01% | 79.45% | 79.74% | 80.21% |
| [bson 2.9.0][bson] | 6.59% | 19.37% | 36.56% | 54.21% | 61.07% |
| [capnp 0.18.13][capnp] | 19.46% | † | 48.76% | 56.19% | 53.61% |
| [cbor4ii 0.3.2][cbor4ii] | 16.03% | 28.69% | 49.99% | 71.59% | 70.91% |
| [ciborium 0.2.2][ciborium] | 3.72% | 13.39% | 49.99% | 71.59% | 70.91% |
| [databuf 0.5.0][databuf] | 53.20% | 71.40% | 91.90% | 92.66% | 86.84% |
| [dlhn 0.1.6][dlhn] | 18.37% | 59.84% | 97.07% | 95.81% | 90.61% |
| [flatbuffers 23.5.26][flatbuffers] | 10.45% | † | 55.13% | 61.64% | 59.10% |
| [msgpacker 0.4.3][msgpacker] | 12.43% | 56.93% | 91.99% | 91.61% | 86.75% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.67% | 34.76% | 85.96% | 86.85% | 80.49% |
| [nanoserde 0.1.37][nanoserde] | 57.13% | 70.38% | 67.29% | 77.41% | 73.71% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 22.20% | 64.75% | 91.90% | 92.65% | 86.88% |
| [postcard 1.0.8][postcard] | 34.81% | 66.20% | 97.07% | 95.51% | 90.56% |
| [pot 3.0.0][pot] | 6.16% | 22.19% | 72.40% | 77.53% | 75.56% |
| [prost 0.12.3][prost] | <span title="encode">*18.07%\**</span> <span title="populate + encode">*6.20%\**</span> | 41.30% | 79.55% | 79.54% | 72.84% |
| [rkyv 0.7.44][rkyv] | 66.75% | <span title="unvalidated">*99.86%\**</span> <span title="validated upfront with error">*72.42%\**</span> | 69.57% | 75.24% | 68.90% |
| [rmp-serde 1.1.2][rmp-serde] | 10.40% | 41.22% | 89.65% | 88.76% | 82.60% |
| [ron 0.8.1][ron] | 0.98% | 9.07% | 43.78% | 64.30% | 65.71% |
| [savefile 0.16.5][savefile] | 71.01% | 69.78% | 67.29% | 77.40% | 73.71% |
| [serde_bare 0.5.0][serde_bare] | 22.13% | 68.53% | 91.90% | 92.66% | 86.84% |
| [serde_cbor 0.11.2][serde_cbor] | 7.44% | 28.75% | 49.99% | 71.59% | 70.91% |
| [serde_json 1.0.114][serde_json] | 3.77% | 25.67% | 38.51% | 61.38% | 63.64% |
| [simd-json 0.13.8][simd-json] | 6.78% | 28.85% | 38.51% | 61.38% | 63.64% |
| [speedy 0.8.7][speedy] | 73.68% | 82.55% | 79.45% | 79.74% | 80.21% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*28.46%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.64%\**</span> | <span title="validated on-demand with panic">*43.22%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*6.36%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.99%\**</span> <span title="validated upfront with error">*0.56%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.94%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 259.54 µs | <span title="unvalidated">*258.72 µs\**</span> | 6000024 | 5378513 | 5345890 |
| [alkahest 0.1.5][alkahest] | 157.99 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 2.0.0-rc][bincode] | 425.11 µs | 819.91 µs | 6000005 | 5378497 | 5345897 |
| [bincode 1.3.3][bincode1] | 5.1294 ms | 4.5163 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.6.0][bitcode] | 1.3996 ms | 596.35 µs | 6000009 | 5182295 | 4923776 |
| [borsh 1.3.0][borsh] | 6.1434 ms | 4.6115 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.9.0][bson] | 45.283 ms | 78.966 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.18.13][capnp] | 5.4437 ms | † | 14000088 | 7130367 | 6051062 |
| [cbor4ii 0.3.2][cbor4ii] | 10.342 ms | 47.567 ms | 13125016 | 7524114 | 6757967 |
| [ciborium 0.2.2][ciborium] | 66.116 ms | 106.87 ms | 13122324 | 7524660 | 6759658 |
| [databuf 0.5.0][databuf] | 2.4001 ms | 5.3123 ms | 6000003 | 5378495 | 5345900 |
| [dlhn 0.1.6][dlhn] | 7.1862 ms | 5.6943 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 648.73 µs | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 20.031 ms | 9.2680 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.35 ms | 27.144 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.37][nanoserde] | 1.6009 ms | 898.83 µs | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 5.0771 ms | 4.0400 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.8][postcard] | 488.30 µs | 1.2696 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 39.683 ms | 74.635 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.12.3][prost] | <span title="encode">*6.9470 ms\**</span> <span title="populate + encode">*8.1173 ms\**</span> | 13.559 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.44][rkyv] | 186.60 µs | <span title="unvalidated">*156.28 µs\**</span> <span title="validated upfront with error">*197.93 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 13.411 ms | 18.723 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 174.16 ms | 249.55 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.5][savefile] | 259.61 µs | 260.06 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.4701 ms | 4.5644 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 35.074 ms | 46.545 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.114][serde_json] | 86.744 ms | 73.084 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.13.8][simd-json] | 53.751 ms | 72.608 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.7][speedy] | 259.50 µs | 258.78 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1652 ns\**</span> | <span title="unvalidated">*140.53 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8555 ns\**</span> | <span title="validated on-demand with panic">*77.471 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*112.32 ns\**</span> | <span title="validated on-demand with error">*2.1406 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4743 ns\**</span> <span title="validated upfront with error">*37.120 ns\**</span> | <span title="unvalidated">*54.096 µs\**</span> <span title="validated upfront with error">*77.545 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*6.1886 ns\**</span> | <span title="unvalidated">*48.566 µs\**</span> <span title="validated upfront with error">*38.701 µs\**</span> | 99.869 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 60.87% | <span title="unvalidated">*60.41%\**</span> | 100.00% | 96.35% | 92.10% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.10% |
| [bincode 2.0.0-rc][bincode] | 37.16% | 19.06% | 100.00% | 96.35% | 92.10% |
| [bincode 1.3.3][bincode1] | 3.08% | 3.46% | 100.00% | 96.35% | 92.10% |
| [bitcode 0.6.0][bitcode] | 11.29% | 26.21% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 2.57% | 3.39% | 100.00% | 96.35% | 92.10% |
| [bson 2.9.0][bson] | 0.35% | 0.20% | 26.07% | 56.26% | 65.67% |
| [capnp 0.18.13][capnp] | 2.90% | † | 42.86% | 72.68% | 81.37% |
| [cbor4ii 0.3.2][cbor4ii] | 1.53% | 0.33% | 45.71% | 68.88% | 72.86% |
| [ciborium 0.2.2][ciborium] | 0.24% | 0.15% | 45.72% | 68.87% | 72.84% |
| [databuf 0.5.0][databuf] | 6.58% | 2.94% | 100.00% | 96.35% | 92.10% |
| [dlhn 0.1.6][dlhn] | 2.20% | 2.74% | 100.00% | 96.35% | 92.10% |
| [flatbuffers 23.5.26][flatbuffers] | 24.35% | † | 100.00% | 96.35% | 92.10% |
| [msgpacker 0.4.3][msgpacker] | 0.79% | 1.69% | 80.00% | 85.54% | 81.87% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.13% | 0.58% | 73.85% | 79.81% | 77.09% |
| [nanoserde 0.1.37][nanoserde] | 9.87% | 17.39% | 100.00% | 96.35% | 92.10% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.11% | 3.87% | 100.00% | 96.35% | 92.10% |
| [postcard 1.0.8][postcard] | 32.36% | 12.31% | 100.00% | 96.35% | 92.10% |
| [pot 3.0.0][pot] | 0.40% | 0.21% | 59.27% | 76.05% | 71.86% |
| [prost 0.12.3][prost] | <span title="encode">*2.27%\**</span> <span title="populate + encode">*1.95%\**</span> | 1.15% | 68.57% | 77.75% | 76.67% |
| [rkyv 0.7.44][rkyv] | 84.67% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.96%\**</span> | 100.00% | 96.35% | 92.10% |
| [rmp-serde 1.1.2][rmp-serde] | 1.18% | 0.83% | 73.85% | 79.79% | 77.04% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% |
| [savefile 0.16.5][savefile] | 60.86% | 60.09% | 100.00% | 96.35% | 92.10% |
| [serde_bare 0.5.0][serde_bare] | 2.44% | 3.42% | 100.00% | 96.35% | 92.10% |
| [serde_cbor 0.11.2][serde_cbor] | 0.45% | 0.34% | 45.72% | 68.87% | 72.84% |
| [serde_json 1.0.114][serde_json] | 0.18% | 0.21% | 22.91% | 54.17% | 57.34% |
| [simd-json 0.13.8][simd-json] | 0.29% | 0.22% | 22.91% | 54.17% | 57.34% |
| [speedy 0.8.7][speedy] | 60.88% | 60.39% | 100.00% | 96.35% | 92.10% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.13%\**</span> | <span title="unvalidated">*27.54%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*49.96%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.10%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*3.33%\**</span> | <span title="unvalidated">*71.54%\**</span> <span title="validated upfront with error">*49.91%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*19.99%\**</span> | <span title="unvalidated">*79.69%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 191.76 µs | <span title="unvalidated">*1.3167 ms\**</span> | 1290592 | 397059 | 340586 |
| [alkahest 0.1.5][alkahest] | 219.62 µs | † | 667570 | 325484 | 320452 |
| [bincode 2.0.0-rc][bincode] | 274.28 µs | 2.0635 ms | 367413 | 221291 | 206273 |
| [bincode 1.3.3][bincode1] | 576.52 µs | 1.8175 ms | 569975 | 240525 | 232423 |
| [bitcode 0.6.0][bitcode] | 124.45 µs | 1.2709 ms | 327693 | 200948 | 182747 |
| [borsh 1.3.0][borsh] | 551.80 µs | 1.8184 ms | 446595 | 234236 | 210008 |
| [bson 2.9.0][bson] | 2.8821 ms | 8.2377 ms | 1619653 | 502185 | 328399 |
| [capnp 0.18.13][capnp] | 481.85 µs | † | 803896 | 335606 | 280851 |
| [cbor4ii 0.3.2][cbor4ii] | 801.55 µs | 4.6010 ms | 1109831 | 344745 | 274514 |
| [ciborium 0.2.2][ciborium] | 3.7434 ms | 9.5435 ms | 1109821 | 344751 | 274526 |
| [databuf 0.5.0][databuf] | 325.32 µs | 1.7351 ms | 356311 | 213062 | 198488 |
| [dlhn 0.1.6][dlhn] | 766.24 µs | 2.5914 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.3149 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 913.34 µs | 2.8062 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3914 ms | 3.9113 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.37][nanoserde] | 282.39 µs | 1.8812 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 651.72 µs | 1.9772 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.8][postcard] | 430.40 µs | 1.9454 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 2.2890 ms | 5.9534 ms | 599125 | 299158 | 247693 |
| [prost 0.12.3][prost] | <span title="encode">*1.0254 ms\**</span> <span title="populate + encode">*2.7100 ms\**</span> | 3.6082 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.44][rkyv] | 298.28 µs | <span title="unvalidated">*1.2566 ms\**</span> <span title="validated upfront with error">*1.7800 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.4283 ms | 2.9538 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 8.6725 ms | 17.127 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.5][savefile] | 219.40 µs | 1.8256 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 710.22 µs | 2.2259 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 1.7731 ms | 4.6976 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.114][serde_json] | 3.6073 ms | 6.5143 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.13.8][simd-json] | 2.2195 ms | 4.5061 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.7][speedy] | 271.01 µs | 1.6554 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.192 µs\**</span> | <span title="unvalidated">*37.633 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8569 ns\**</span> | <span title="validated on-demand with panic">*4.5778 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.361 ns\**</span> | <span title="validated on-demand with error">*433.52 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4746 ns\**</span> <span title="validated upfront with error">*2.1430 ms\**</span> | <span title="unvalidated">*1.3744 µs\**</span> <span title="validated upfront with error">*2.1500 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2366 ns\**</span> <span title="validated upfront with error">*527.18 µs\**</span> | <span title="unvalidated">*164.02 ns\**</span> <span title="validated upfront with error">*513.10 µs\**</span> | 848.94 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 64.90% | <span title="unvalidated">*95.44%\**</span> | 25.39% | 50.61% | 53.66% |
| [alkahest 0.1.5][alkahest] | 56.67% | † | 49.09% | 61.74% | 57.03% |
| [bincode 2.0.0-rc][bincode] | 45.37% | 60.90% | 89.19% | 90.81% | 88.59% |
| [bincode 1.3.3][bincode1] | 21.59% | 69.14% | 57.49% | 83.55% | 78.63% |
| [bitcode 0.6.0][bitcode] | 100.00% | 98.87% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 22.55% | 69.10% | 73.38% | 85.79% | 87.02% |
| [bson 2.9.0][bson] | 4.32% | 15.25% | 20.23% | 40.01% | 55.65% |
| [capnp 0.18.13][capnp] | 25.83% | † | 40.76% | 59.88% | 65.07% |
| [cbor4ii 0.3.2][cbor4ii] | 15.53% | 27.31% | 29.53% | 58.29% | 66.57% |
| [ciborium 0.2.2][ciborium] | 3.32% | 13.17% | 29.53% | 58.29% | 66.57% |
| [databuf 0.5.0][databuf] | 38.25% | 72.42% | 91.97% | 94.31% | 92.07% |
| [dlhn 0.1.6][dlhn] | 16.24% | 48.49% | 89.41% | 91.09% | 88.85% |
| [flatbuffers 23.5.26][flatbuffers] | 3.75% | † | 38.82% | 58.13% | 62.16% |
| [msgpacker 0.4.3][msgpacker] | 13.63% | 44.78% | 83.76% | 84.83% | 82.89% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.31% | 32.13% | 72.86% | 79.60% | 79.07% |
| [nanoserde 0.1.37][nanoserde] | 44.07% | 66.80% | 57.69% | 83.75% | 78.63% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 19.10% | 63.55% | 91.97% | 94.35% | 92.05% |
| [postcard 1.0.8][postcard] | 28.91% | 64.59% | 89.17% | 90.55% | 88.14% |
| [pot 3.0.0][pot] | 5.44% | 21.11% | 54.70% | 67.17% | 73.78% |
| [prost 0.12.3][prost] | <span title="encode">*12.14%\**</span> <span title="populate + encode">*4.59%\**</span> | 34.83% | 54.91% | 65.82% | 67.86% |
| [rkyv 0.7.44][rkyv] | 41.72% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*70.60%\**</span> | 54.89% | 79.12% | 82.80% |
| [rmp-serde 1.1.2][rmp-serde] | 8.71% | 42.54% | 77.19% | 81.95% | 80.79% |
| [ron 0.8.1][ron] | 1.43% | 7.34% | 22.36% | 46.20% | 53.23% |
| [savefile 0.16.5][savefile] | 56.72% | 68.83% | 57.80% | 83.95% | 78.77% |
| [serde_bare 0.5.0][serde_bare] | 17.52% | 56.45% | 91.97% | 94.31% | 92.07% |
| [serde_cbor 0.11.2][serde_cbor] | 7.02% | 26.75% | 29.53% | 58.29% | 66.57% |
| [serde_json 1.0.114][serde_json] | 3.45% | 19.29% | 20.19% | 43.07% | 50.82% |
| [simd-json 0.13.8][simd-json] | 5.61% | 27.89% | 20.19% | 43.07% | 50.82% |
| [speedy 0.8.7][speedy] | 45.92% | 75.91% | 72.89% | 85.52% | 86.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.44%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.59%\**</span> | <span title="validated on-demand with panic">*3.58%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*37.83%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.93%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 483.05 µs | <span title="unvalidated">*2.3250 ms\**</span> | 2984682 | 1406922 | 1270221 |
| [alkahest 0.1.5][alkahest] | 739.03 µs | † | 1863391 | 1234113 | 1202345 |
| [bincode 2.0.0-rc][bincode] | 700.56 µs | 3.6627 ms | 1372381 | 1091486 | 1037296 |
| [bincode 1.3.3][bincode1] | 3.7217 ms | 3.8387 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.6.0][bitcode] | 736.75 µs | 2.3698 ms | 958426 | 860768 | 838712 |
| [borsh 1.3.0][borsh] | 2.9005 ms | 2.5420 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.9.0][bson] | 21.245 ms | 44.262 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.18.13][capnp] | 2.1679 ms | † | 2664040 | 1511895 | 1212087 |
| [cbor4ii 0.3.2][cbor4ii] | 4.2451 ms | 18.155 ms | 5878791 | 1655835 | 1431390 |
| [ciborium 0.2.2][ciborium] | 22.708 ms | 47.938 ms | 5878653 | 1655791 | 1431560 |
| [databuf 0.5.0][databuf] | 1.7742 ms | 3.6090 ms | 1288257 | 1037579 | 984337 |
| [dlhn 0.1.6][dlhn] | 5.1945 ms | 7.0770 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.0878 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 1.8698 ms | 4.5337 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 32.602 ms | 15.395 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.37][nanoserde] | 1.3043 ms | 2.9917 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.0583 ms | 3.0194 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.8][postcard] | 1.7380 ms | 3.9254 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 13.838 ms | 30.624 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.12.3][prost] | <span title="encode">*4.0722 ms\**</span> <span title="populate + encode">*8.1733 ms\**</span> | 9.3744 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.44][rkyv] | 1.2772 ms | <span title="unvalidated">*2.1531 ms\**</span> <span title="validated upfront with error">*2.7659 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 10.128 ms | 11.480 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 38.555 ms | 96.704 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.5][savefile] | 1.0094 ms | 2.6258 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.9776 ms | 4.3764 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 9.3565 ms | 21.594 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.114][serde_json] | 20.084 ms | 29.014 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.13.8][simd-json] | 11.448 ms | 25.699 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.7][speedy] | 707.15 µs | 2.5079 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*65.727 µs\**</span> | <span title="unvalidated">*66.417 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8548 ns\**</span> | <span title="validated on-demand with panic">*629.24 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.341 ns\**</span> | <span title="validated on-demand with error">*710.71 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4726 ns\**</span> <span title="validated upfront with error">*4.6907 ms\**</span> | <span title="unvalidated">*2.6554 µs\**</span> <span title="validated upfront with error">*4.7204 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2365 ns\**</span> <span title="validated upfront with error">*599.67 µs\**</span> | <span title="unvalidated">*475.46 ns\**</span> <span title="validated upfront with error">*598.83 µs\**</span> | 504.94 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.61%\**</span> | 32.11% | 61.18% | 66.03% |
| [alkahest 0.1.5][alkahest] | 65.36% | † | 51.43% | 69.75% | 69.76% |
| [bincode 2.0.0-rc][bincode] | 68.95% | 58.78% | 69.84% | 78.86% | 80.86% |
| [bincode 1.3.3][bincode1] | 12.98% | 56.09% | 52.92% | 77.18% | 81.78% |
| [bitcode 0.6.0][bitcode] | 65.56% | 90.86% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 16.65% | 84.70% | 64.49% | 79.53% | 82.75% |
| [bson 2.9.0][bson] | 2.27% | 4.86% | 9.55% | 30.38% | 52.39% |
| [capnp 0.18.13][capnp] | 22.28% | † | 35.98% | 56.93% | 69.20% |
| [cbor4ii 0.3.2][cbor4ii] | 11.38% | 11.86% | 16.30% | 51.98% | 58.59% |
| [ciborium 0.2.2][ciborium] | 2.13% | 4.49% | 16.30% | 51.99% | 58.59% |
| [databuf 0.5.0][databuf] | 27.23% | 59.66% | 74.40% | 82.96% | 85.21% |
| [dlhn 0.1.6][dlhn] | 9.30% | 30.42% | 74.90% | 81.82% | 82.13% |
| [flatbuffers 23.5.26][flatbuffers] | 9.49% | † | 42.15% | 61.12% | 67.88% |
| [msgpacker 0.4.3][msgpacker] | 25.83% | 47.49% | 67.30% | 76.26% | 75.55% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.48% | 13.99% | 55.45% | 68.99% | 68.00% |
| [nanoserde 0.1.37][nanoserde] | 37.04% | 71.97% | 54.13% | 77.67% | 81.43% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 15.79% | 71.31% | 74.40% | 82.82% | 85.02% |
| [postcard 1.0.8][postcard] | 27.79% | 54.85% | 74.90% | 81.34% | 82.49% |
| [pot 3.0.0][pot] | 3.49% | 7.03% | 37.66% | 59.47% | 66.12% |
| [prost 0.12.3][prost] | <span title="encode">*11.86%\**</span> <span title="populate + encode">*5.91%\**</span> | 22.97% | 52.71% | 65.82% | 66.23% |
| [rkyv 0.7.44][rkyv] | 37.82% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.84%\**</span> | 47.23% | 64.47% | 72.37% |
| [rmp-serde 1.1.2][rmp-serde] | 4.77% | 18.76% | 56.25% | 69.87% | 69.88% |
| [ron 0.8.1][ron] | 1.25% | 2.23% | 11.31% | 39.46% | 47.01% |
| [savefile 0.16.5][savefile] | 47.86% | 82.00% | 54.76% | 78.13% | 81.60% |
| [serde_bare 0.5.0][serde_bare] | 9.70% | 49.20% | 74.40% | 82.96% | 85.20% |
| [serde_cbor 0.11.2][serde_cbor] | 5.16% | 9.97% | 16.30% | 51.99% | 58.59% |
| [serde_json 1.0.114][serde_json] | 2.41% | 7.42% | 10.45% | 36.88% | 46.58% |
| [simd-json 0.13.8][simd-json] | 4.22% | 8.38% | 10.45% | 36.88% | 46.58% |
| [speedy 0.8.7][speedy] | 68.31% | 85.85% | 61.96% | 78.71% | 82.76% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.72%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*75.56%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*66.90%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.91%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
[simd-json]: https://crates.io/crates/simd-json/0.13.8
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
