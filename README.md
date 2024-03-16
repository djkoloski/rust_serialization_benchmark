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

## Last updated: 2024-3-16 3:59:29

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 396.17 µs | <span title="unvalidated">*1.4660 ms\**</span> | 1705800 | 520073 | 413496 | 7.0259 ms |
| [alkahest 0.1.5][alkahest] | 195.44 µs | † | 1045784 | 454157 | 389424 | 6.2266 ms |
| [bincode 2.0.0-rc][bincode] | 213.62 µs | 2.4307 ms | 741295 | 303944 | 257153 | 3.9236 ms |
| [bincode 1.3.3][bincode1] | 520.12 µs | 2.1032 ms | 1045784 | 373127 | 311761 | 4.9756 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 144.43 µs | 1.4402 ms | 703710 | 288826 | 229755 | 2.7414 ms |
| [borsh 1.3.0][borsh] | 542.77 µs | 2.2028 ms | 885780 | 362204 | 286514 | 4.6592 ms |
| [bson 2.9.0][bson] | 2.2279 ms | 7.2769 ms | 1924682 | 532821 | 376270 | 6.2369 ms |
| [capnp 0.18.13][capnp] | 465.71 µs | † | 1443216 | 513986 | 428649 | 6.7681 ms |
| [cbor4ii 0.3.2][cbor4ii] | 903.80 µs | 4.7457 ms | 1407835 | 403440 | 324081 | 5.2300 ms |
| [ciborium 0.2.2][ciborium] | 3.9469 ms | 9.9136 ms | 1407835 | 403440 | 324081 | 5.2726 ms |
| [databuf 0.5.0][databuf] | 265.60 µs | 2.0309 ms | 765778 | 311715 | 264630 | 4.2292 ms |
| [dlhn 0.1.6][dlhn] | 812.93 µs | 2.3975 ms | 724953 | 301446 | 253629 | 3.9558 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.3727 ms | † | 1276368 | 468539 | 388832 | 5.6941 ms |
| [msgpacker 0.4.3][msgpacker] | 1.2690 ms | 2.5042 ms | 764996 | 315291 | 264898 | 4.2990 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.1436 ms | 4.0433 ms | 818669 | 332556 | 285514 | 4.8573 ms |
| [nanoserde 0.1.37][nanoserde] | 281.06 µs | 2.0495 ms | 1045784 | 373127 | 311761 | 4.6902 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 652.02 µs | 2.2162 ms | 765778 | 311743 | 264518 | 4.2729 ms |
| [postcard 1.0.8][postcard] | 415.07 µs | 2.1145 ms | 724953 | 302399 | 253747 | 3.9076 ms |
| [pot 3.0.0][pot] | 2.3155 ms | 6.5251 ms | 971922 | 372513 | 304122 | 5.0927 ms |
| [prost 0.12.3][prost] | <span title="encode">*815.29 µs\**</span> <span title="populate + encode">*2.2952 ms\**</span> | 3.3575 ms | 884628 | 363130 | 315494 | 5.2402 ms |
| [rkyv 0.7.44][rkyv] | 217.44 µs | <span title="unvalidated">*1.4539 ms\**</span> <span title="validated upfront with error">*1.9829 ms\**</span> | 1011488 | 383862 | 333545 | 5.0762 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3756 ms | 3.5122 ms | 784997 | 325384 | 278219 | 4.2662 ms |
| [ron 0.8.1][ron] | 14.023 ms | 16.800 ms | 1607459 | 449158 | 349713 | 5.8330 ms |
| [savefile 0.16.5][savefile] | 203.48 µs | 2.0837 ms | 1045800 | 373139 | 311755 | 4.6327 ms |
| [serde_bare 0.5.0][serde_bare] | 671.41 µs | 2.1101 ms | 765778 | 311715 | 264630 | 3.9887 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7043 ms | 5.2988 ms | 1407835 | 403440 | 324081 | 4.9402 ms |
| [serde_json 1.0.114][serde_json] | 3.7399 ms | 6.1631 ms | 1827461 | 470560 | 361090 | 5.7964 ms |
| [simd-json 0.13.8][simd-json] | 2.0960 ms | 4.6712 ms | 1827461 | 470560 | 361090 | 5.9642 ms |
| [speedy 0.8.7][speedy] | 201.57 µs | 1.7577 ms | 885780 | 362204 | 286514 | 4.3688 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.694 µs\**</span> | <span title="unvalidated">*40.345 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8776 ns\**</span> | <span title="validated on-demand with panic">*24.788 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*73.663 ns\**</span> | <span title="validated on-demand with error">*170.93 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4738 ns\**</span> <span title="validated upfront with error">*1.9092 ms\**</span> | <span title="unvalidated">*50.687 µs\**</span> <span title="validated upfront with error">*1.9121 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2373 ns\**</span> <span title="validated upfront with error">*521.66 µs\**</span> | <span title="unvalidated">*10.520 µs\**</span> <span title="validated upfront with error">*531.69 µs\**</span> | 9.6511 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 36.46% | <span title="unvalidated">*98.24%\**</span> | 41.25% | 55.54% | 55.56% | 39.02% |
| [alkahest 0.1.5][alkahest] | 73.90% | † | 67.29% | 63.60% | 59.00% | 44.03% |
| [bincode 2.0.0-rc][bincode] | 67.61% | 59.25% | 94.93% | 95.03% | 89.35% | 69.87% |
| [bincode 1.3.3][bincode1] | 27.77% | 68.48% | 67.29% | 77.41% | 73.70% | 55.10% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 26.61% | 65.38% | 79.45% | 79.74% | 80.19% | 58.84% |
| [bson 2.9.0][bson] | 6.48% | 19.79% | 36.56% | 54.21% | 61.06% | 43.95% |
| [capnp 0.18.13][capnp] | 31.01% | † | 48.76% | 56.19% | 53.60% | 40.50% |
| [cbor4ii 0.3.2][cbor4ii] | 15.98% | 30.35% | 49.99% | 71.59% | 70.89% | 52.42% |
| [ciborium 0.2.2][ciborium] | 3.66% | 14.53% | 49.99% | 71.59% | 70.89% | 51.99% |
| [databuf 0.5.0][databuf] | 54.38% | 70.91% | 91.89% | 92.66% | 86.82% | 64.82% |
| [dlhn 0.1.6][dlhn] | 17.77% | 60.07% | 97.07% | 95.81% | 90.59% | 69.30% |
| [flatbuffers 23.5.26][flatbuffers] | 10.52% | † | 55.13% | 61.64% | 59.09% | 48.14% |
| [msgpacker 0.4.3][msgpacker] | 11.38% | 57.51% | 91.99% | 91.61% | 86.73% | 63.77% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.02% | 35.62% | 85.96% | 86.85% | 80.47% | 56.44% |
| [nanoserde 0.1.37][nanoserde] | 51.39% | 70.27% | 67.29% | 77.41% | 73.70% | 58.45% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 22.15% | 64.99% | 91.89% | 92.65% | 86.86% | 64.16% |
| [postcard 1.0.8][postcard] | 34.80% | 68.11% | 97.07% | 95.51% | 90.54% | 70.16% |
| [pot 3.0.0][pot] | 6.24% | 22.07% | 72.40% | 77.53% | 75.55% | 53.83% |
| [prost 0.12.3][prost] | <span title="encode">*17.72%\**</span> <span title="populate + encode">*6.29%\**</span> | 42.90% | 79.55% | 79.54% | 72.82% | 52.31% |
| [rkyv 0.7.44][rkyv] | 66.42% | <span title="unvalidated">*99.06%\**</span> <span title="validated upfront with error">*72.63%\**</span> | 69.57% | 75.24% | 68.88% | 54.00% |
| [rmp-serde 1.1.2][rmp-serde] | 10.50% | 41.01% | 89.64% | 88.76% | 82.58% | 64.26% |
| [ron 0.8.1][ron] | 1.03% | 8.57% | 43.78% | 64.30% | 65.70% | 47.00% |
| [savefile 0.16.5][savefile] | 70.98% | 69.12% | 67.29% | 77.40% | 73.70% | 59.17% |
| [serde_bare 0.5.0][serde_bare] | 21.51% | 68.25% | 91.89% | 92.66% | 86.82% | 68.73% |
| [serde_cbor 0.11.2][serde_cbor] | 8.47% | 27.18% | 49.99% | 71.59% | 70.89% | 55.49% |
| [serde_json 1.0.114][serde_json] | 3.86% | 23.37% | 38.51% | 61.38% | 63.63% | 47.29% |
| [simd-json 0.13.8][simd-json] | 6.89% | 30.83% | 38.51% | 61.38% | 63.63% | 45.96% |
| [speedy 0.8.7][speedy] | 71.65% | 81.94% | 79.45% | 79.74% | 80.19% | 62.75% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*26.08%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*65.90%\**</span> | <span title="validated on-demand with panic">*42.44%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*6.15%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.75%\**</span> <span title="validated upfront with error">*0.55%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.98%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 265.33 µs | <span title="unvalidated">*265.50 µs\**</span> | 6000024 | 5378513 | 5345890 | 7.7891 ms |
| [alkahest 0.1.5][alkahest] | 210.34 µs | † | 6000008 | 5378500 | 5345890 | 8.2526 ms |
| [bincode 2.0.0-rc][bincode] | 423.78 µs | 828.16 µs | 6000005 | 5378497 | 5345897 | 7.6973 ms |
| [bincode 1.3.3][bincode1] | 5.1206 ms | 4.6031 ms | 6000008 | 5378500 | 5345890 | 7.6765 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 1.4611 ms | 602.52 µs | 6000006 | 5182295 | 4923880 | 14.034 ms |
| [borsh 1.3.0][borsh] | 6.0101 ms | 4.5362 ms | 6000004 | 5378496 | 5345889 | 7.8087 ms |
| [bson 2.9.0][bson] | 41.184 ms | 78.195 ms | 23013911 | 9212089 | 7497811 | 122.35 ms |
| [capnp 0.18.13][capnp] | 5.5029 ms | † | 14000088 | 7130367 | 6051062 | 95.748 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.379 ms | 47.283 ms | 13125016 | 7524114 | 6757967 | 101.74 ms |
| [ciborium 0.2.2][ciborium] | 66.304 ms | 101.76 ms | 13122324 | 7524660 | 6759658 | 101.74 ms |
| [databuf 0.5.0][databuf] | 2.4005 ms | 5.3111 ms | 6000003 | 5378495 | 5345900 | 7.7525 ms |
| [dlhn 0.1.6][dlhn] | 6.6898 ms | 5.6523 ms | 6000003 | 5378495 | 5345900 | 7.7369 ms |
| [flatbuffers 23.5.26][flatbuffers] | 670.81 µs | † | 6000024 | 5378434 | 5345910 | 7.7712 ms |
| [msgpacker 0.4.3][msgpacker] | 20.274 ms | 8.6852 ms | 7500005 | 6058442 | 6014337 | 10.256 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 129.45 ms | 26.735 ms | 8125037 | 6493484 | 6386940 | 86.748 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2402 ms | 901.61 µs | 6000008 | 5378500 | 5345890 | 7.8919 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 5.0781 ms | 4.6742 ms | 6000004 | 5378496 | 5345889 | 7.6362 ms |
| [postcard 1.0.8][postcard] | 507.43 µs | 1.4116 ms | 6000003 | 5378495 | 5345900 | 7.8791 ms |
| [pot 3.0.0][pot] | 37.217 ms | 72.315 ms | 10122342 | 6814618 | 6852251 | 97.048 ms |
| [prost 0.12.3][prost] | <span title="encode">*7.0248 ms\**</span> <span title="populate + encode">*8.3500 ms\**</span> | 13.821 ms | 8750000 | 6665735 | 6421871 | 86.170 ms |
| [rkyv 0.7.44][rkyv] | 207.33 µs | <span title="unvalidated">*199.32 µs\**</span> <span title="validated upfront with error">*198.94 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.7647 ms |
| [rmp-serde 1.1.2][rmp-serde] | 13.458 ms | 19.078 ms | 8125006 | 6494876 | 6391037 | 82.530 ms |
| [ron 0.8.1][ron] | 179.38 ms | 255.02 ms | 22192885 | 8970395 | 8138755 | 161.71 ms |
| [savefile 0.16.5][savefile] | 264.29 µs | 264.68 µs | 6000024 | 5378518 | 5345893 | 7.8821 ms |
| [serde_bare 0.5.0][serde_bare] | 6.2134 ms | 4.1290 ms | 6000003 | 5378495 | 5345900 | 8.0970 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.512 ms | 45.212 ms | 13122324 | 7524660 | 6759658 | 102.58 ms |
| [serde_json 1.0.114][serde_json] | 91.349 ms | 87.075 ms | 26192883 | 9566084 | 8586741 | 168.85 ms |
| [simd-json 0.13.8][simd-json] | 55.830 ms | 74.625 ms | 26192883 | 9566084 | 8586741 | 169.90 ms |
| [speedy 0.8.7][speedy] | 264.29 µs | 263.80 µs | 6000004 | 5378496 | 5345889 | 7.6940 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1645 ns\**</span> | <span title="unvalidated">*141.30 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8550 ns\**</span> | <span title="validated on-demand with panic">*77.406 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*104.39 ns\**</span> | <span title="validated on-demand with error">*2.1881 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4734 ns\**</span> <span title="validated upfront with error">*37.836 ns\**</span> | <span title="unvalidated">*54.183 µs\**</span> <span title="validated upfront with error">*77.416 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*9.9055 ns\**</span> | <span title="unvalidated">*45.943 µs\**</span> <span title="validated upfront with error">*77.447 µs\**</span> | 102.83 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 78.14% | <span title="unvalidated">*74.93%\**</span> | 100.00% | 96.35% | 92.11% | 98.04% |
| [alkahest 0.1.5][alkahest] | 98.57% | † | 100.00% | 96.35% | 92.11% | 92.53% |
| [bincode 2.0.0-rc][bincode] | 48.92% | 24.02% | 100.00% | 96.35% | 92.11% | 99.21% |
| [bincode 1.3.3][bincode1] | 4.05% | 4.32% | 100.00% | 96.35% | 92.11% | 99.48% |
| [bitcode 0.6.0-alpha.2][bitcode] | 14.19% | 33.02% | 100.00% | 100.00% | 100.00% | 54.41% |
| [borsh 1.3.0][borsh] | 3.45% | 4.39% | 100.00% | 96.35% | 92.11% | 97.79% |
| [bson 2.9.0][bson] | 0.50% | 0.25% | 26.07% | 56.26% | 65.67% | 6.24% |
| [capnp 0.18.13][capnp] | 3.77% | † | 42.86% | 72.68% | 81.37% | 7.98% |
| [cbor4ii 0.3.2][cbor4ii] | 2.00% | 0.42% | 45.71% | 68.88% | 72.86% | 7.51% |
| [ciborium 0.2.2][ciborium] | 0.31% | 0.20% | 45.72% | 68.87% | 72.84% | 7.51% |
| [databuf 0.5.0][databuf] | 8.64% | 3.75% | 100.00% | 96.35% | 92.11% | 98.50% |
| [dlhn 0.1.6][dlhn] | 3.10% | 3.52% | 100.00% | 96.35% | 92.11% | 98.70% |
| [flatbuffers 23.5.26][flatbuffers] | 30.91% | † | 100.00% | 96.35% | 92.11% | 98.26% |
| [msgpacker 0.4.3][msgpacker] | 1.02% | 2.29% | 80.00% | 85.54% | 81.87% | 74.46% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.74% | 73.85% | 79.81% | 77.09% | 8.80% |
| [nanoserde 0.1.37][nanoserde] | 16.72% | 22.06% | 100.00% | 96.35% | 92.11% | 96.76% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.08% | 4.26% | 100.00% | 96.35% | 92.11% | 100.00% |
| [postcard 1.0.8][postcard] | 40.86% | 14.09% | 100.00% | 96.35% | 92.11% | 96.92% |
| [pot 3.0.0][pot] | 0.56% | 0.28% | 59.27% | 76.05% | 71.86% | 7.87% |
| [prost 0.12.3][prost] | <span title="encode">*2.95%\**</span> <span title="populate + encode">*2.48%\**</span> | 1.44% | 68.57% | 77.75% | 76.67% | 8.86% |
| [rkyv 0.7.44][rkyv] | 100.00% | <span title="unvalidated">*99.81%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 98.35% |
| [rmp-serde 1.1.2][rmp-serde] | 1.54% | 1.04% | 73.85% | 79.79% | 77.04% | 9.25% |
| [ron 0.8.1][ron] | 0.12% | 0.08% | 27.04% | 57.77% | 60.50% | 4.72% |
| [savefile 0.16.5][savefile] | 78.45% | 75.16% | 100.00% | 96.35% | 92.11% | 96.88% |
| [serde_bare 0.5.0][serde_bare] | 3.34% | 4.82% | 100.00% | 96.35% | 92.11% | 94.31% |
| [serde_cbor 0.11.2][serde_cbor] | 0.58% | 0.44% | 45.72% | 68.87% | 72.84% | 7.44% |
| [serde_json 1.0.114][serde_json] | 0.23% | 0.23% | 22.91% | 54.17% | 57.34% | 4.52% |
| [simd-json 0.13.8][simd-json] | 0.37% | 0.27% | 22.91% | 54.17% | 57.34% | 4.49% |
| [speedy 0.8.7][speedy] | 78.45% | 75.41% | 100.00% | 96.35% | 92.11% | 99.25% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.14%\**</span> | <span title="unvalidated">*32.51%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.68%\**</span> | <span title="validated on-demand with panic">*59.35%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*2.10%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*3.27%\**</span> | <span title="unvalidated">*84.79%\**</span> <span title="validated upfront with error">*59.35%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*12.49%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*59.32%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 189.94 µs | <span title="unvalidated">*1.3279 ms\**</span> | 1290592 | 396572 | 340338 | 5.1194 ms |
| [alkahest 0.1.5][alkahest] | 225.17 µs | † | 667570 | 325484 | 320452 | 4.0867 ms |
| [bincode 2.0.0-rc][bincode] | 261.77 µs | 2.0792 ms | 367413 | 221291 | 206273 | 2.5700 ms |
| [bincode 1.3.3][bincode1] | 563.91 µs | 1.8025 ms | 569975 | 240525 | 232423 | 2.9592 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 128.09 µs | 1.2676 ms | 327688 | 200947 | 182736 | 753.02 µs |
| [borsh 1.3.0][borsh] | 552.34 µs | 1.8340 ms | 446595 | 234236 | 210008 | 2.5966 ms |
| [bson 2.9.0][bson] | 2.8229 ms | 8.3024 ms | 1619653 | 502185 | 328399 | 4.9871 ms |
| [capnp 0.18.13][capnp] | 462.89 µs | † | 803896 | 335606 | 280851 | 4.0514 ms |
| [cbor4ii 0.3.2][cbor4ii] | 792.60 µs | 4.7151 ms | 1109831 | 344745 | 274514 | 3.9973 ms |
| [ciborium 0.2.2][ciborium] | 3.6737 ms | 9.4027 ms | 1109821 | 344751 | 274526 | 3.9761 ms |
| [databuf 0.5.0][databuf] | 313.74 µs | 1.7559 ms | 356311 | 213062 | 198488 | 2.4943 ms |
| [dlhn 0.1.6][dlhn] | 831.42 µs | 2.5217 ms | 366496 | 220600 | 205683 | 2.5418 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.3243 ms | † | 844168 | 345696 | 294015 | 4.0114 ms |
| [msgpacker 0.4.3][msgpacker] | 957.17 µs | 2.8162 ms | 391251 | 236877 | 220476 | 2.6982 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2414 ms | 3.9596 ms | 449745 | 252432 | 231110 | 2.9260 ms |
| [nanoserde 0.1.37][nanoserde] | 291.93 µs | 1.8906 ms | 567975 | 239930 | 232419 | 3.0009 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 649.78 µs | 2.0269 ms | 356311 | 212976 | 198524 | 2.4573 ms |
| [postcard 1.0.8][postcard] | 440.07 µs | 1.9900 ms | 367489 | 221913 | 207344 | 2.5954 ms |
| [pot 3.0.0][pot] | 2.3411 ms | 5.9970 ms | 599125 | 299158 | 247693 | 3.2308 ms |
| [prost 0.12.3][prost] | <span title="encode">*1.0287 ms\**</span> <span title="populate + encode">*2.7117 ms\**</span> | 3.4971 ms | 596811 | 305319 | 269310 | 3.5669 ms |
| [rkyv 0.7.44][rkyv] | 303.01 µs | <span title="unvalidated">*1.2586 ms\**</span> <span title="validated upfront with error">*1.7749 ms\**</span> | 596952 | 253967 | 220706 | 2.8291 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.4400 ms | 2.9572 ms | 424533 | 245214 | 226188 | 2.7705 ms |
| [ron 0.8.1][ron] | 8.2744 ms | 17.211 ms | 1465223 | 434935 | 343338 | 6.0322 ms |
| [savefile 0.16.5][savefile] | 222.89 µs | 1.8245 ms | 566991 | 239361 | 232010 | 2.9912 ms |
| [serde_bare 0.5.0][serde_bare] | 710.61 µs | 2.1897 ms | 356311 | 213062 | 198488 | 2.4624 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.6930 ms | 4.7460 ms | 1109821 | 344751 | 274526 | 3.9826 ms |
| [serde_json 1.0.114][serde_json] | 3.7656 ms | 6.7584 ms | 1623191 | 466527 | 359623 | 6.1441 ms |
| [simd-json 0.13.8][simd-json] | 2.2418 ms | 4.5198 ms | 1623191 | 466527 | 359623 | 6.1201 ms |
| [speedy 0.8.7][speedy] | 273.29 µs | 1.6458 ms | 449595 | 234970 | 210361 | 2.5665 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.023 µs\**</span> | <span title="unvalidated">*37.545 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8553 ns\**</span> | <span title="validated on-demand with panic">*4.5936 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*72.947 ns\**</span> | <span title="validated on-demand with error">*435.97 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4736 ns\**</span> <span title="validated upfront with error">*2.1707 ms\**</span> | <span title="unvalidated">*1.3725 µs\**</span> <span title="validated upfront with error">*2.1749 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*504.90 µs\**</span> | <span title="unvalidated">*238.90 ns\**</span> <span title="validated upfront with error">*505.00 µs\**</span> | 854.69 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 67.44% | <span title="unvalidated">*94.78%\**</span> | 25.39% | 50.67% | 53.69% | 14.71% |
| [alkahest 0.1.5][alkahest] | 56.89% | † | 49.09% | 61.74% | 57.02% | 18.43% |
| [bincode 2.0.0-rc][bincode] | 48.93% | 60.53% | 89.19% | 90.81% | 88.59% | 29.30% |
| [bincode 1.3.3][bincode1] | 22.71% | 69.83% | 57.49% | 83.55% | 78.62% | 25.45% |
| [bitcode 0.6.0-alpha.2][bitcode] | 100.00% | 99.29% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 23.19% | 68.63% | 73.37% | 85.79% | 87.01% | 29.00% |
| [bson 2.9.0][bson] | 4.54% | 15.16% | 20.23% | 40.01% | 55.64% | 15.10% |
| [capnp 0.18.13][capnp] | 27.67% | † | 40.76% | 59.88% | 65.07% | 18.59% |
| [cbor4ii 0.3.2][cbor4ii] | 16.16% | 26.69% | 29.53% | 58.29% | 66.57% | 18.84% |
| [ciborium 0.2.2][ciborium] | 3.49% | 13.39% | 29.53% | 58.29% | 66.56% | 18.94% |
| [databuf 0.5.0][databuf] | 40.83% | 71.68% | 91.97% | 94.31% | 92.06% | 30.19% |
| [dlhn 0.1.6][dlhn] | 15.41% | 49.91% | 89.41% | 91.09% | 88.84% | 29.63% |
| [flatbuffers 23.5.26][flatbuffers] | 3.85% | † | 38.82% | 58.13% | 62.15% | 18.77% |
| [msgpacker 0.4.3][msgpacker] | 13.38% | 44.69% | 83.75% | 84.83% | 82.88% | 27.91% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.44% | 31.79% | 72.86% | 79.60% | 79.07% | 25.74% |
| [nanoserde 0.1.37][nanoserde] | 43.88% | 66.57% | 57.69% | 83.75% | 78.62% | 25.09% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 19.71% | 62.09% | 91.97% | 94.35% | 92.05% | 30.64% |
| [postcard 1.0.8][postcard] | 29.11% | 63.25% | 89.17% | 90.55% | 88.13% | 29.01% |
| [pot 3.0.0][pot] | 5.47% | 20.99% | 54.69% | 67.17% | 73.78% | 23.31% |
| [prost 0.12.3][prost] | <span title="encode">*12.45%\**</span> <span title="populate + encode">*4.72%\**</span> | 35.99% | 54.91% | 65.82% | 67.85% | 21.11% |
| [rkyv 0.7.44][rkyv] | 42.27% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*70.91%\**</span> | 54.89% | 79.12% | 82.80% | 26.62% |
| [rmp-serde 1.1.2][rmp-serde] | 8.90% | 42.56% | 77.19% | 81.95% | 80.79% | 27.18% |
| [ron 0.8.1][ron] | 1.55% | 7.31% | 22.36% | 46.20% | 53.22% | 12.48% |
| [savefile 0.16.5][savefile] | 57.47% | 68.98% | 57.79% | 83.95% | 78.76% | 25.17% |
| [serde_bare 0.5.0][serde_bare] | 18.03% | 57.48% | 91.97% | 94.31% | 92.06% | 30.58% |
| [serde_cbor 0.11.2][serde_cbor] | 7.57% | 26.52% | 29.53% | 58.29% | 66.56% | 18.91% |
| [serde_json 1.0.114][serde_json] | 3.40% | 18.62% | 20.19% | 43.07% | 50.81% | 12.26% |
| [simd-json 0.13.8][simd-json] | 5.71% | 27.85% | 20.19% | 43.07% | 50.81% | 12.30% |
| [speedy 0.8.7][speedy] | 46.87% | 76.47% | 72.89% | 85.52% | 86.87% | 29.34% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.64%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*5.20%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*54.80%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.41%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 487.86 µs | <span title="unvalidated">*2.3227 ms\**</span> | 2984682 | 1408245 | 1274069 | 15.609 ms |
| [alkahest 0.1.5][alkahest] | 730.40 µs | † | 1863391 | 1234113 | 1202345 | 12.139 ms |
| [bincode 2.0.0-rc][bincode] | 701.28 µs | 3.7695 ms | 1372381 | 1091486 | 1037296 | 9.9361 ms |
| [bincode 1.3.3][bincode1] | 3.7854 ms | 3.9050 ms | 1811011 | 1115281 | 1025627 | 10.606 ms |
| [bitcode 0.6.0-alpha.2][bitcode] | 725.33 µs | 2.3389 ms | 948499 | 857321 | 837658 | 3.1928 ms |
| [borsh 1.3.0][borsh] | 2.8575 ms | 2.6824 ms | 1486162 | 1082357 | 1013550 | 10.164 ms |
| [bson 2.9.0][bson] | 22.030 ms | 43.698 ms | 10030880 | 2833079 | 1600859 | 29.428 ms |
| [capnp 0.18.13][capnp] | 2.3388 ms | † | 2664040 | 1511895 | 1212087 | 15.099 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.2861 ms | 18.707 ms | 5878791 | 1655835 | 1431390 | 23.021 ms |
| [ciborium 0.2.2][ciborium] | 23.291 ms | 47.156 ms | 5878653 | 1655791 | 1431560 | 23.057 ms |
| [databuf 0.5.0][databuf] | 1.7756 ms | 3.6209 ms | 1288257 | 1037579 | 984337 | 9.2623 ms |
| [dlhn 0.1.6][dlhn] | 5.1764 ms | 7.0553 ms | 1279599 | 1052061 | 1021161 | 8.9404 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.1341 ms | † | 2273740 | 1408408 | 1235566 | 13.561 ms |
| [msgpacker 0.4.3][msgpacker] | 1.9218 ms | 4.5228 ms | 1424043 | 1128758 | 1110156 | 10.037 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.936 ms | 15.783 ms | 1728519 | 1247642 | 1233323 | 13.073 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3281 ms | 2.8959 ms | 1770477 | 1108304 | 1029947 | 10.416 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.0491 ms | 3.0793 ms | 1288257 | 1039269 | 986510 | 9.2458 ms |
| [postcard 1.0.8][postcard] | 1.7349 ms | 3.9130 ms | 1279599 | 1058243 | 1016738 | 9.1757 ms |
| [pot 3.0.0][pot] | 13.607 ms | 31.207 ms | 2544810 | 1447453 | 1268390 | 16.657 ms |
| [prost 0.12.3][prost] | <span title="encode">*4.0282 ms\**</span> <span title="populate + encode">*8.2134 ms\**</span> | 8.9563 ms | 1818378 | 1307777 | 1266311 | 12.207 ms |
| [rkyv 0.7.44][rkyv] | 1.1124 ms | <span title="unvalidated">*2.1525 ms\**</span> <span title="validated upfront with error">*2.7667 ms\**</span> | 2029080 | 1335117 | 1158855 | 12.378 ms |
| [rmp-serde 1.1.2][rmp-serde] | 10.401 ms | 11.431 ms | 1703813 | 1231892 | 1200208 | 11.705 ms |
| [ron 0.8.1][ron] | 37.733 ms | 95.379 ms | 8476284 | 2181196 | 1783971 | 35.930 ms |
| [savefile 0.16.5][savefile] | 999.66 µs | 2.6827 ms | 1750226 | 1101682 | 1027827 | 10.479 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8070 ms | 4.4208 ms | 1288257 | 1037597 | 984356 | 9.2573 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.3743 ms | 21.724 ms | 5878653 | 1655791 | 1431560 | 22.813 ms |
| [serde_json 1.0.114][serde_json] | 20.024 ms | 30.483 ms | 9175594 | 2334253 | 1800713 | 35.581 ms |
| [simd-json 0.13.8][simd-json] | 11.289 ms | 28.439 ms | 9175594 | 2334253 | 1800713 | 35.474 ms |
| [speedy 0.8.7][speedy] | 739.90 µs | 2.4972 ms | 1546963 | 1093532 | 1013443 | 10.308 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.013 µs\**</span> | <span title="unvalidated">*67.557 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8560 ns\**</span> | <span title="validated on-demand with panic">*628.71 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*73.128 ns\**</span> | <span title="validated on-demand with error">*713.22 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4737 ns\**</span> <span title="validated upfront with error">*4.6027 ms\**</span> | <span title="unvalidated">*2.6439 µs\**</span> <span title="validated upfront with error">*4.6058 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2373 ns\**</span> <span title="validated upfront with error">*622.43 µs\**</span> | <span title="unvalidated">*490.33 ns\**</span> <span title="validated upfront with error">*623.96 µs\**</span> | 501.56 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.67%\**</span> | 31.78% | 60.88% | 65.75% | 20.45% |
| [alkahest 0.1.5][alkahest] | 66.79% | † | 50.90% | 69.47% | 69.67% | 26.30% |
| [bincode 2.0.0-rc][bincode] | 69.57% | 57.10% | 69.11% | 78.55% | 80.75% | 32.13% |
| [bincode 1.3.3][bincode1] | 12.89% | 55.12% | 52.37% | 76.87% | 81.67% | 30.10% |
| [bitcode 0.6.0-alpha.2][bitcode] | 67.26% | 92.03% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.0][borsh] | 17.07% | 80.25% | 63.82% | 79.21% | 82.65% | 31.41% |
| [bson 2.9.0][bson] | 2.21% | 4.93% | 9.46% | 30.26% | 52.33% | 10.85% |
| [capnp 0.18.13][capnp] | 20.86% | † | 35.60% | 56.71% | 69.11% | 21.15% |
| [cbor4ii 0.3.2][cbor4ii] | 11.38% | 11.51% | 16.13% | 51.78% | 58.52% | 13.87% |
| [ciborium 0.2.2][ciborium] | 2.09% | 4.56% | 16.13% | 51.78% | 58.51% | 13.85% |
| [databuf 0.5.0][databuf] | 27.48% | 59.45% | 73.63% | 82.63% | 85.10% | 34.47% |
| [dlhn 0.1.6][dlhn] | 9.42% | 30.51% | 74.12% | 81.49% | 82.03% | 35.71% |
| [flatbuffers 23.5.26][flatbuffers] | 9.50% | † | 41.72% | 60.87% | 67.80% | 23.54% |
| [msgpacker 0.4.3][msgpacker] | 25.39% | 47.59% | 66.61% | 75.95% | 75.45% | 31.81% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.58% | 13.64% | 54.87% | 68.72% | 67.92% | 24.42% |
| [nanoserde 0.1.37][nanoserde] | 36.73% | 74.33% | 53.57% | 77.35% | 81.33% | 30.65% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 16.00% | 69.90% | 73.63% | 82.49% | 84.91% | 34.53% |
| [postcard 1.0.8][postcard] | 28.12% | 55.01% | 74.12% | 81.01% | 82.39% | 34.80% |
| [pot 3.0.0][pot] | 3.59% | 6.90% | 37.27% | 59.23% | 66.04% | 19.17% |
| [prost 0.12.3][prost] | <span title="encode">*12.11%\**</span> <span title="populate + encode">*5.94%\**</span> | 24.03% | 52.16% | 65.56% | 66.15% | 26.16% |
| [rkyv 0.7.44][rkyv] | 43.86% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.80%\**</span> | 46.75% | 64.21% | 72.28% | 25.79% |
| [rmp-serde 1.1.2][rmp-serde] | 4.69% | 18.83% | 55.67% | 69.59% | 69.79% | 27.28% |
| [ron 0.8.1][ron] | 1.29% | 2.26% | 11.19% | 39.31% | 46.95% | 8.89% |
| [savefile 0.16.5][savefile] | 48.80% | 80.24% | 54.19% | 77.82% | 81.50% | 30.47% |
| [serde_bare 0.5.0][serde_bare] | 10.15% | 48.69% | 73.63% | 82.63% | 85.10% | 34.49% |
| [serde_cbor 0.11.2][serde_cbor] | 5.20% | 9.91% | 16.13% | 51.78% | 58.51% | 14.00% |
| [serde_json 1.0.114][serde_json] | 2.44% | 7.06% | 10.34% | 36.73% | 46.52% | 8.97% |
| [simd-json 0.13.8][simd-json] | 4.32% | 7.57% | 10.34% | 36.73% | 46.52% | 9.00% |
| [speedy 0.8.7][speedy] | 65.94% | 86.20% | 61.31% | 78.40% | 82.65% | 30.97% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.73%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*77.99%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*68.75%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.55%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
