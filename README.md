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

## Last updated: 2023-11-5 23:45:31

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 260.30 µs | <span title="unvalidated">*2.3287 ms\**</span> | 1705800 | 530428 | 403744 |
| [alkahest 0.1.5][alkahest] | 239.33 µs | † | 1045784 | 454157 | 389424 |
| [bincode 1.3.3][bincode] | 611.66 µs | 3.0228 ms | 1045784 | 373127 | 311761 |
| [bitcode 0.5.0][bitcode] | 539.27 µs | 3.3111 ms | 703664 | 317711 | 273622 |
| [borsh 1.1.1][borsh] | 498.18 µs | 3.3315 ms | 885780 | 362204 | 286514 |
| [bson 2.6.0][bson] | 2.5798 ms | 9.6547 ms | 1924682 | 532821 | 376270 |
| [capnp 0.18.3][capnp] | 589.74 µs | † | 1443216 | 513986 | 428649 |
| [cbor4ii 0.3.1][cbor4ii] | 1.2581 ms | 6.4234 ms | 1407835 | 403440 | 324081 |
| [ciborium 0.2.1][ciborium] | 2.7487 ms | 12.081 ms | 1407835 | 403440 | 324081 |
| [databuf 0.5.0][databuf] | 410.22 µs | 3.0645 ms | 765778 | 311715 | 264630 |
| [dlhn 0.1.6][dlhn] | 825.61 µs | 3.5158 ms | 724953 | 301446 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.7462 ms | † | 1276368 | 468539 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.5520 ms | 3.5557 ms | 764996 | 315291 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.5302 ms | 5.8462 ms | 818669 | 332556 | 285514 |
| [nanoserde 0.1.35][nanoserde] | 334.50 µs | 2.9938 ms | 1045784 | 373127 | 311761 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 616.26 µs | 3.2907 ms | 765778 | 311743 | 264518 |
| [postcard 1.0.8][postcard] | 407.03 µs | 3.1688 ms | 724953 | 302399 | 253747 |
| [pot 3.0.0][pot] | 2.7818 ms | 7.9580 ms | 971922 | 372513 | 304122 |
| [prost 0.12.1][prost] | <span title="encode">*449.84 µs\**</span> <span title="populate + encode">*3.1265 ms\**</span> | 3.9878 ms | 764951 | 268137 | 227947 |
| [rkyv 0.7.42][rkyv] | 312.22 µs | <span title="unvalidated">*2.3555 ms\**</span> <span title="validated upfront with error">*3.2806 ms\**</span> | 1011488 | 383862 | 333545 |
| [rmp-serde 1.1.2][rmp-serde] | 1.4169 ms | 4.5216 ms | 784997 | 325384 | 278219 |
| [ron 0.8.1][ron] | 16.433 ms | 21.407 ms | 1607459 | 449158 | 349713 |
| [savefile 0.16.2][savefile] | 245.55 µs | 3.1197 ms | 1045800 | 373139 | 311755 |
| [serde_bare 0.5.0][serde_bare] | 673.48 µs | 3.0918 ms | 765778 | 311715 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 1.9627 ms | 6.2045 ms | 1407835 | 403440 | 324081 |
| [serde_json 1.0.108][serde_json] | 4.3460 ms | 7.7952 ms | 1827461 | 470560 | 361090 |
| [simd-json 0.13.4][simd-json] | 2.4093 ms | 6.8655 ms | 1827461 | 470560 | 361090 |
| [speedy 0.8.6][speedy] | 250.31 µs | 2.7216 ms | 885780 | 362204 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*32.734 µs\**</span> | <span title="unvalidated">*57.320 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.4097 ns\**</span> | <span title="validated on-demand with panic">*35.300 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*85.777 ns\**</span> | <span title="validated on-demand with error">*191.30 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6924 ns\**</span> <span title="validated upfront with error">*2.3031 ms\**</span> | <span title="unvalidated">*69.605 µs\**</span> <span title="validated upfront with error">*2.4055 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2059 ns\**</span> <span title="validated upfront with error">*822.26 µs\**</span> | <span title="unvalidated">*17.653 µs\**</span> <span title="validated upfront with error">*840.25 µs\**</span> | 19.713 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 91.94% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 50.55% | 56.46% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.04% | 58.53% |
| [bincode 1.3.3][bincode] | 39.13% | 77.04% | 67.29% | 71.86% | 73.12% |
| [bitcode 0.5.0][bitcode] | 44.38% | 70.33% | 100.00% | 84.40% | 83.31% |
| [borsh 1.1.1][borsh] | 48.04% | 69.90% | 79.44% | 74.03% | 79.56% |
| [bson 2.6.0][bson] | 9.28% | 24.12% | 36.56% | 50.32% | 60.58% |
| [capnp 0.18.3][capnp] | 40.58% | † | 48.76% | 52.17% | 53.18% |
| [cbor4ii 0.3.1][cbor4ii] | 19.02% | 36.25% | 49.98% | 66.46% | 70.34% |
| [ciborium 0.2.1][ciborium] | 8.71% | 19.28% | 49.98% | 66.46% | 70.34% |
| [databuf 0.5.0][databuf] | 58.34% | 75.99% | 91.89% | 86.02% | 86.14% |
| [dlhn 0.1.6][dlhn] | 28.99% | 66.24% | 97.06% | 88.95% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 13.71% | † | 55.13% | 57.23% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 15.42% | 65.49% | 91.98% | 85.04% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.81% | 39.83% | 85.95% | 80.63% | 79.84% |
| [nanoserde 0.1.35][nanoserde] | 71.55% | 77.78% | 67.29% | 71.86% | 73.12% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 38.84% | 70.77% | 91.89% | 86.01% | 86.17% |
| [postcard 1.0.8][postcard] | 58.80% | 73.49% | 97.06% | 88.67% | 89.83% |
| [pot 3.0.0][pot] | 8.60% | 29.26% | 72.40% | 71.98% | 74.95% |
| [prost 0.12.1][prost] | <span title="encode">*53.20%\**</span> <span title="populate + encode">*7.65%\**</span> | 58.40% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 76.65% | <span title="unvalidated">*98.86%\**</span> <span title="validated upfront with error">*70.98%\**</span> | 69.57% | 69.85% | 68.34% |
| [rmp-serde 1.1.2][rmp-serde] | 16.89% | 51.50% | 89.64% | 82.41% | 81.93% |
| [ron 0.8.1][ron] | 1.46% | 10.88% | 43.77% | 59.70% | 65.18% |
| [savefile 0.16.2][savefile] | 97.47% | 74.64% | 67.28% | 71.86% | 73.12% |
| [serde_bare 0.5.0][serde_bare] | 35.54% | 75.32% | 91.89% | 86.02% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 12.19% | 37.53% | 49.98% | 66.46% | 70.34% |
| [serde_json 1.0.108][serde_json] | 5.51% | 29.87% | 38.51% | 56.98% | 63.13% |
| [simd-json 0.13.4][simd-json] | 9.93% | 33.92% | 38.51% | 56.98% | 63.13% |
| [speedy 0.8.6][speedy] | 95.61% | 85.56% | 79.44% | 74.03% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*30.80%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*50.04%\**</span> | <span title="validated on-demand with panic">*50.01%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*9.23%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.79%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*25.36%\**</span> <span title="validated upfront with error">*0.73%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.10%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 424.89 µs | <span title="unvalidated">*424.28 µs\**</span> | 6000024 | 5378513 | 5345890 |
| [alkahest 0.1.5][alkahest] | 436.36 µs | † | 6000008 | 5378500 | 5345890 |
| [bincode 1.3.3][bincode] | 3.7774 ms | 6.2197 ms | 6000008 | 5378500 | 5345890 |
| [bitcode 0.5.0][bitcode] | 4.8602 ms | 9.7191 ms | 4688054 | 4688491 | 4688168 |
| [borsh 1.1.1][borsh] | 5.2517 ms | 6.3593 ms | 6000004 | 5378496 | 5345889 |
| [bson 2.6.0][bson] | 52.530 ms | 103.07 ms | 23013911 | 9212089 | 7497811 |
| [capnp 0.18.3][capnp] | 8.0638 ms | † | 14000088 | 7130367 | 6051062 |
| [cbor4ii 0.3.1][cbor4ii] | 16.443 ms | 59.566 ms | 13125016 | 7524114 | 6757967 |
| [ciborium 0.2.1][ciborium] | 69.334 ms | 112.48 ms | 13122324 | 7524660 | 6759658 |
| [databuf 0.5.0][databuf] | 1.9503 ms | 7.0366 ms | 6000003 | 5378495 | 5345900 |
| [dlhn 0.1.6][dlhn] | 7.2621 ms | 9.8146 ms | 6000003 | 5378495 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 992.64 µs | † | 6000024 | 5378434 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 24.536 ms | 10.627 ms | 7500005 | 6058442 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 205.51 ms | 34.514 ms | 8125037 | 6493484 | 6386940 |
| [nanoserde 0.1.35][nanoserde] | 2.2163 ms | 1.1843 ms | 6000008 | 5378500 | 5345890 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 4.2512 ms | 6.0289 ms | 6000004 | 5378496 | 5345889 |
| [postcard 1.0.8][postcard] | 589.03 µs | 1.7932 ms | 6000003 | 5378495 | 5345900 |
| [pot 3.0.0][pot] | 45.845 ms | 78.288 ms | 10122342 | 6814618 | 6852251 |
| [prost 0.12.1][prost] | <span title="encode">*10.626 ms\**</span> <span title="populate + encode">*12.736 ms\**</span> | 14.749 ms | 8750000 | 6665735 | 6421871 |
| [rkyv 0.7.42][rkyv] | 579.70 µs | <span title="unvalidated">*434.42 µs\**</span> <span title="validated upfront with error">*434.47 µs\**</span> | 6000008 | 5378500 | 5345892 |
| [rmp-serde 1.1.2][rmp-serde] | 11.899 ms | 20.091 ms | 8125006 | 6494876 | 6391037 |
| [ron 0.8.1][ron] | 216.84 ms | 355.63 ms | 22192885 | 8970395 | 8138755 |
| [savefile 0.16.2][savefile] | 424.54 µs | 424.74 µs | 6000024 | 5378518 | 5345893 |
| [serde_bare 0.5.0][serde_bare] | 6.1374 ms | 6.0287 ms | 6000003 | 5378495 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 39.069 ms | 50.710 ms | 13122324 | 7524660 | 6759658 |
| [serde_json 1.0.108][serde_json] | 108.40 ms | 85.095 ms | 26192883 | 9566084 | 8586741 |
| [simd-json 0.13.4][simd-json] | 70.986 ms | 106.61 ms | 26192883 | 9566084 | 8586741 |
| [speedy 0.8.6][speedy] | 424.52 µs | 424.92 µs | 6000004 | 5378496 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.0190 ns\**</span> | <span title="unvalidated">*245.97 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.4101 ns\**</span> | <span title="validated on-demand with panic">*100.63 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*141.54 ns\**</span> | <span title="validated on-demand with error">*2.7976 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6901 ns\**</span> <span title="validated upfront with error">*49.324 ns\**</span> | <span title="unvalidated">*65.221 µs\**</span> <span title="validated upfront with error">*100.50 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6064 ns\**</span> <span title="validated upfront with error">*7.8165 ns\**</span> | <span title="unvalidated">*37.201 µs\**</span> <span title="validated upfront with error">*37.506 µs\**</span> | 237.27 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 99.91% | <span title="unvalidated">*100.00%\**</span> | 78.13% | 87.17% | 87.70% |
| [alkahest 0.1.5][alkahest] | 97.29% | † | 78.13% | 87.17% | 87.70% |
| [bincode 1.3.3][bincode] | 11.24% | 6.82% | 78.13% | 87.17% | 87.70% |
| [bitcode 0.5.0][bitcode] | 8.73% | 4.37% | 100.00% | 100.00% | 100.00% |
| [borsh 1.1.1][borsh] | 8.08% | 6.67% | 78.13% | 87.17% | 87.70% |
| [bson 2.6.0][bson] | 0.81% | 0.41% | 20.37% | 50.89% | 62.53% |
| [capnp 0.18.3][capnp] | 5.26% | † | 33.49% | 65.75% | 77.48% |
| [cbor4ii 0.3.1][cbor4ii] | 2.58% | 0.71% | 35.72% | 62.31% | 69.37% |
| [ciborium 0.2.1][ciborium] | 0.61% | 0.38% | 35.73% | 62.31% | 69.36% |
| [databuf 0.5.0][databuf] | 21.77% | 6.03% | 78.13% | 87.17% | 87.70% |
| [dlhn 0.1.6][dlhn] | 5.85% | 4.32% | 78.13% | 87.17% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 42.77% | † | 78.13% | 87.17% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 1.73% | 3.99% | 62.51% | 77.39% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.21% | 1.23% | 57.70% | 72.20% | 73.40% |
| [nanoserde 0.1.35][nanoserde] | 19.15% | 35.83% | 78.13% | 87.17% | 87.70% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 9.99% | 7.04% | 78.13% | 87.17% | 87.70% |
| [postcard 1.0.8][postcard] | 72.07% | 23.66% | 78.13% | 87.17% | 87.70% |
| [pot 3.0.0][pot] | 0.93% | 0.54% | 46.31% | 68.80% | 68.42% |
| [prost 0.12.1][prost] | <span title="encode">*4.00%\**</span> <span title="populate + encode">*3.33%\**</span> | 2.88% | 53.58% | 70.34% | 73.00% |
| [rkyv 0.7.42][rkyv] | 73.23% | <span title="unvalidated">*97.67%\**</span> <span title="validated upfront with error">*97.65%\**</span> | 78.13% | 87.17% | 87.70% |
| [rmp-serde 1.1.2][rmp-serde] | 3.57% | 2.11% | 57.70% | 72.19% | 73.36% |
| [ron 0.8.1][ron] | 0.20% | 0.12% | 21.12% | 52.27% | 57.60% |
| [savefile 0.16.2][savefile] | 100.00% | 99.89% | 78.13% | 87.17% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 6.92% | 7.04% | 78.13% | 87.17% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 1.09% | 0.84% | 35.73% | 62.31% | 69.36% |
| [serde_json 1.0.108][serde_json] | 0.39% | 0.50% | 17.90% | 49.01% | 54.60% |
| [simd-json 0.13.4][simd-json] | 0.60% | 0.40% | 17.90% | 49.01% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 99.85% | 78.13% | 87.17% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*79.56%\**</span> | <span title="unvalidated">*15.12%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.65%\**</span> | <span title="validated on-demand with panic">*36.97%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.13%\**</span> | <span title="validated on-demand with error">*1.33%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*59.72%\**</span> <span title="validated upfront with error">*3.26%\**</span> | <span title="unvalidated">*57.04%\**</span> <span title="validated upfront with error">*37.02%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*20.55%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.19%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 228.35 µs | <span title="unvalidated">*1.8930 ms\**</span> | 1290592 | 394642 | 331565 |
| [alkahest 0.1.5][alkahest] | 260.04 µs | † | 667570 | 325484 | 320452 |
| [bincode 1.3.3][bincode] | 646.63 µs | 2.5008 ms | 569975 | 240525 | 232423 |
| [bitcode 0.5.0][bitcode] | 371.13 µs | 2.6455 ms | 322798 | 214279 | 201247 |
| [borsh 1.1.1][borsh] | 566.77 µs | 2.5177 ms | 446595 | 234236 | 210008 |
| [bson 2.6.0][bson] | 3.6259 ms | 11.295 ms | 1619653 | 502185 | 328399 |
| [capnp 0.18.3][capnp] | 597.12 µs | † | 803896 | 335606 | 280851 |
| [cbor4ii 0.3.1][cbor4ii] | 1.3185 ms | 6.3818 ms | 1109831 | 344745 | 274514 |
| [ciborium 0.2.1][ciborium] | 3.6629 ms | 11.150 ms | 1109821 | 344751 | 274526 |
| [databuf 0.5.0][databuf] | 468.37 µs | 2.4031 ms | 356311 | 213062 | 198488 |
| [dlhn 0.1.6][dlhn] | 899.43 µs | 3.4345 ms | 366496 | 220600 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.3032 ms | † | 844168 | 345696 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.2435 ms | 3.8844 ms | 391251 | 236877 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.1852 ms | 5.1700 ms | 449745 | 252432 | 231110 |
| [nanoserde 0.1.35][nanoserde] | 332.29 µs | 2.5975 ms | 567975 | 239930 | 232419 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 686.48 µs | 2.7559 ms | 356311 | 212976 | 198524 |
| [postcard 1.0.8][postcard] | 478.92 µs | 2.6910 ms | 367489 | 221913 | 207344 |
| [pot 3.0.0][pot] | 2.8120 ms | 7.3315 ms | 599125 | 299158 | 247693 |
| [prost 0.12.1][prost] | <span title="encode">*1.3776 ms\**</span> <span title="populate + encode">*3.9419 ms\**</span> | 4.6882 ms | 596811 | 305319 | 269310 |
| [rkyv 0.7.42][rkyv] | 426.83 µs | <span title="unvalidated">*1.8029 ms\**</span> <span title="validated upfront with error">*2.6746 ms\**</span> | 596952 | 253967 | 220706 |
| [rmp-serde 1.1.2][rmp-serde] | 1.6405 ms | 3.9731 ms | 424533 | 245214 | 226188 |
| [ron 0.8.1][ron] | 10.421 ms | 21.989 ms | 1465223 | 434935 | 343338 |
| [savefile 0.16.2][savefile] | 278.63 µs | 2.4646 ms | 566991 | 239361 | 232010 |
| [serde_bare 0.5.0][serde_bare] | 817.33 µs | 3.1184 ms | 356311 | 213062 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.1287 ms | 6.2878 ms | 1109821 | 344751 | 274526 |
| [serde_json 1.0.108][serde_json] | 4.6011 ms | 9.6567 ms | 1623191 | 466527 | 359623 |
| [simd-json 0.13.4][simd-json] | 2.7741 ms | 6.0332 ms | 1623191 | 466527 | 359623 |
| [speedy 0.8.6][speedy] | 334.92 µs | 2.2227 ms | 449595 | 234970 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*36.018 µs\**</span> | <span title="unvalidated">*36.629 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.4096 ns\**</span> | <span title="validated on-demand with panic">*7.0811 µs\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*85.754 ns\**</span> | <span title="validated on-demand with error">*663.94 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6824 ns\**</span> <span title="validated upfront with error">*2.5876 ms\**</span> | <span title="unvalidated">*1.7827 µs\**</span> <span title="validated upfront with error">*2.5912 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2058 ns\**</span> <span title="validated upfront with error">*752.90 µs\**</span> | <span title="unvalidated">*145.52 ns\**</span> <span title="validated upfront with error">*755.35 µs\**</span> | 1.7123 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.24%\**</span> | 25.01% | 53.97% | 59.86% |
| [alkahest 0.1.5][alkahest] | 87.81% | † | 48.35% | 65.43% | 61.94% |
| [bincode 1.3.3][bincode] | 35.31% | 72.09% | 56.63% | 88.55% | 85.40% |
| [bitcode 0.5.0][bitcode] | 61.53% | 68.15% | 100.00% | 99.39% | 98.63% |
| [borsh 1.1.1][borsh] | 40.29% | 71.61% | 72.28% | 90.92% | 94.51% |
| [bson 2.6.0][bson] | 6.30% | 15.96% | 19.93% | 42.41% | 60.44% |
| [capnp 0.18.3][capnp] | 38.24% | † | 40.15% | 63.46% | 70.67% |
| [cbor4ii 0.3.1][cbor4ii] | 17.32% | 28.25% | 29.09% | 61.78% | 72.31% |
| [ciborium 0.2.1][ciborium] | 6.23% | 16.17% | 29.09% | 61.78% | 72.30% |
| [databuf 0.5.0][databuf] | 48.75% | 75.02% | 90.59% | 99.96% | 100.00% |
| [dlhn 0.1.6][dlhn] | 25.39% | 52.49% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 6.91% | † | 38.24% | 61.61% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 18.36% | 46.41% | 82.50% | 89.91% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.79% | 34.87% | 71.77% | 84.37% | 85.88% |
| [nanoserde 0.1.35][nanoserde] | 68.72% | 69.41% | 56.83% | 88.77% | 85.40% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 33.26% | 65.42% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.8][postcard] | 47.68% | 67.00% | 87.84% | 95.97% | 95.73% |
| [pot 3.0.0][pot] | 8.12% | 24.59% | 53.88% | 71.19% | 80.13% |
| [prost 0.12.1][prost] | <span title="encode">*16.58%\**</span> <span title="populate + encode">*5.79%\**</span> | 38.46% | 54.09% | 69.76% | 73.70% |
| [rkyv 0.7.42][rkyv] | 53.50% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*67.41%\**</span> | 54.07% | 83.86% | 89.93% |
| [rmp-serde 1.1.2][rmp-serde] | 13.92% | 45.38% | 76.04% | 86.85% | 87.75% |
| [ron 0.8.1][ron] | 2.19% | 8.20% | 22.03% | 48.97% | 57.81% |
| [savefile 0.16.2][savefile] | 81.95% | 73.15% | 56.93% | 88.98% | 85.55% |
| [serde_bare 0.5.0][serde_bare] | 27.94% | 57.81% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 10.73% | 28.67% | 29.09% | 61.78% | 72.30% |
| [serde_json 1.0.108][serde_json] | 4.96% | 18.67% | 19.89% | 45.65% | 55.19% |
| [simd-json 0.13.4][simd-json] | 8.23% | 29.88% | 19.89% | 45.65% | 55.19% |
| [speedy 0.8.6][speedy] | 68.18% | 81.11% | 71.80% | 90.64% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.40%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*50.04%\**</span> | <span title="validated on-demand with panic">*2.06%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*21.92%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*8.16%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 640.76 µs | <span title="unvalidated">*3.1375 ms\**</span> | 2984682 | 1410641 | 1270491 |
| [alkahest 0.1.5][alkahest] | 811.55 µs | † | 1863391 | 1234113 | 1202345 |
| [bincode 1.3.3][bincode] | 3.4572 ms | 5.5209 ms | 1811011 | 1115281 | 1025627 |
| [bitcode 0.5.0][bitcode] | 1.5527 ms | 4.5092 ms | 870693 | 866738 | 870720 |
| [borsh 1.1.1][borsh] | 2.6708 ms | 3.5583 ms | 1486162 | 1082357 | 1013550 |
| [bson 2.6.0][bson] | 29.166 ms | 62.123 ms | 10030880 | 2833079 | 1600859 |
| [capnp 0.18.3][capnp] | 2.5692 ms | † | 2664040 | 1511895 | 1212087 |
| [cbor4ii 0.3.1][cbor4ii] | 5.2383 ms | 25.885 ms | 5878791 | 1655835 | 1431390 |
| [ciborium 0.2.1][ciborium] | 22.100 ms | 51.318 ms | 5878653 | 1655791 | 1431560 |
| [databuf 0.5.0][databuf] | 1.9839 ms | 4.9907 ms | 1288257 | 1037579 | 984337 |
| [dlhn 0.1.6][dlhn] | 5.6472 ms | 10.914 ms | 1279599 | 1052061 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.4721 ms | † | 2273740 | 1408408 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 2.6695 ms | 7.5391 ms | 1424043 | 1128758 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 45.566 ms | 22.478 ms | 1728519 | 1247642 | 1233323 |
| [nanoserde 0.1.35][nanoserde] | 1.2015 ms | 4.3363 ms | 1770477 | 1108304 | 1029947 |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 2.9293 ms | 4.0699 ms | 1288257 | 1039269 | 986510 |
| [postcard 1.0.8][postcard] | 2.2025 ms | 5.4008 ms | 1279599 | 1058243 | 1016738 |
| [pot 3.0.0][pot] | 16.738 ms | 35.907 ms | 2544810 | 1447453 | 1268390 |
| [prost 0.12.1][prost] | <span title="encode">*5.0375 ms\**</span> <span title="populate + encode">*10.937 ms\**</span> | 11.239 ms | 1818378 | 1307777 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.9010 ms | <span title="unvalidated">*2.9242 ms\**</span> <span title="validated upfront with error">*3.8866 ms\**</span> | 2029080 | 1335117 | 1158855 |
| [rmp-serde 1.1.2][rmp-serde] | 10.987 ms | 14.045 ms | 1703813 | 1231892 | 1200208 |
| [ron 0.8.1][ron] | 48.524 ms | 123.29 ms | 8476284 | 2181196 | 1783971 |
| [savefile 0.16.2][savefile] | 1.5209 ms | 3.6079 ms | 1750226 | 1101682 | 1027827 |
| [serde_bare 0.5.0][serde_bare] | 4.5105 ms | 6.4840 ms | 1288257 | 1037597 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 10.147 ms | 28.100 ms | 5878653 | 1655791 | 1431560 |
| [serde_json 1.0.108][serde_json] | 24.693 ms | 40.615 ms | 9175594 | 2334253 | 1800713 |
| [simd-json 0.13.4][simd-json] | 13.465 ms | 37.567 ms | 9175594 | 2334253 | 1800713 |
| [speedy 0.8.6][speedy] | 964.03 µs | 3.2671 ms | 1546963 | 1093532 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*115.68 µs\**</span> | <span title="unvalidated">*116.33 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.4093 ns\**</span> | <span title="validated on-demand with panic">*828.05 ns\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*85.736 ns\**</span> | <span title="validated on-demand with error">*1.1108 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.6805 ns\**</span> <span title="validated upfront with error">*5.0589 ms\**</span> | <span title="unvalidated">*3.4385 µs\**</span> <span title="validated upfront with error">*5.0683 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2058 ns\**</span> <span title="validated upfront with error">*884.78 µs\**</span> | <span title="unvalidated">*424.16 ns\**</span> <span title="validated upfront with error">*887.13 µs\**</span> | 816.33 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.20%\**</span> | 29.17% | 61.44% | 68.53% |
| [alkahest 0.1.5][alkahest] | 78.96% | † | 46.73% | 70.23% | 72.42% |
| [bincode 1.3.3][bincode] | 18.53% | 52.97% | 48.08% | 77.71% | 84.90% |
| [bitcode 0.5.0][bitcode] | 41.27% | 64.85% | 100.00% | 100.00% | 100.00% |
| [borsh 1.1.1][borsh] | 23.99% | 82.18% | 58.59% | 80.08% | 85.91% |
| [bson 2.6.0][bson] | 2.20% | 4.71% | 8.68% | 30.59% | 54.39% |
| [capnp 0.18.3][capnp] | 24.94% | † | 32.68% | 57.33% | 71.84% |
| [cbor4ii 0.3.1][cbor4ii] | 12.23% | 11.30% | 14.81% | 52.34% | 60.83% |
| [ciborium 0.2.1][ciborium] | 2.90% | 5.70% | 14.81% | 52.35% | 60.82% |
| [databuf 0.5.0][databuf] | 32.30% | 58.59% | 67.59% | 83.53% | 88.46% |
| [dlhn 0.1.6][dlhn] | 11.35% | 26.79% | 68.04% | 82.38% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 11.71% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 24.00% | 38.79% | 61.14% | 76.79% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.41% | 13.01% | 50.37% | 69.47% | 70.60% |
| [nanoserde 0.1.35][nanoserde] | 53.33% | 67.44% | 49.18% | 78.20% | 84.54% |
| [parity-scale-codec 3.6.5][parity-scale-codec] | 21.87% | 71.85% | 67.59% | 83.40% | 88.26% |
| [postcard 1.0.8][postcard] | 29.09% | 54.14% | 68.04% | 81.90% | 85.64% |
| [pot 3.0.0][pot] | 3.83% | 8.14% | 34.21% | 59.88% | 68.65% |
| [prost 0.12.1][prost] | <span title="encode">*12.72%\**</span> <span title="populate + encode">*5.86%\**</span> | 26.02% | 47.88% | 66.28% | 68.76% |
| [rkyv 0.7.42][rkyv] | 33.71% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*75.24%\**</span> | 42.91% | 64.92% | 75.14% |
| [rmp-serde 1.1.2][rmp-serde] | 5.83% | 20.82% | 51.10% | 70.36% | 72.55% |
| [ron 0.8.1][ron] | 1.32% | 2.37% | 10.27% | 39.74% | 48.81% |
| [savefile 0.16.2][savefile] | 42.13% | 81.05% | 49.75% | 78.67% | 84.71% |
| [serde_bare 0.5.0][serde_bare] | 14.21% | 45.10% | 67.59% | 83.53% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 6.31% | 10.41% | 14.81% | 52.35% | 60.82% |
| [serde_json 1.0.108][serde_json] | 2.59% | 7.20% | 9.49% | 37.13% | 48.35% |
| [simd-json 0.13.4][simd-json] | 4.76% | 7.78% | 9.49% | 37.13% | 48.35% |
| [speedy 0.8.6][speedy] | 66.47% | 89.50% | 56.28% | 79.26% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.36%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*50.05%\**</span> | <span title="validated on-demand with panic">*51.22%\**</span> | ‡ |
| [capnp 0.18.3][capnp] | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*38.19%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.34%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.5.0
[borsh]: https://crates.io/crates/borsh/1.1.1
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.18.3
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.1
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.35
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.5
[postcard]: https://crates.io/crates/postcard/1.0.8
[pot]: https://crates.io/crates/pot/3.0.0
[prost]: https://crates.io/crates/prost/0.12.1
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.16.2
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.108
[simd-json]: https://crates.io/crates/simd-json/0.13.4
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
