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

## Last updated: 2024-4-10 14:43:10

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 395.03 µs | <span title="unvalidated">*1.4560 ms\**</span> | 1705800 | 520081 | 413510 | 6.7815 ms |
| [alkahest 0.1.5][alkahest] | 211.00 µs | † | 1045784 | 454157 | 389424 | 5.9706 ms |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*793.77 µs\**</span> <span title="only prepend">*811.16 µs\**</span> | 3.3041 ms | 874632 | 355446 | 311723 | 5.0534 ms |
| [bincode 2.0.0-rc][bincode] | 320.95 µs | 2.4216 ms | 741295 | 303944 | 257153 | 3.9412 ms |
| [bincode 1.3.3][bincode1] | 519.45 µs | 2.0355 ms | 1045784 | 373127 | 311761 | 4.7786 ms |
| [bitcode 0.6.0][bitcode] | 139.14 µs | 1.4989 ms | 703710 | 288826 | 229755 | 2.4295 ms |
| [borsh 1.3.1][borsh] | 539.55 µs | 2.2006 ms | 885780 | 362204 | 286514 | 4.5562 ms |
| [bson 2.9.0][bson] | 2.2719 ms | 7.0577 ms | 1924682 | 532821 | 376270 | 5.7706 ms |
| [capnp 0.18.13][capnp] | 491.73 µs | † | 1443216 | 513986 | 428649 | 6.8166 ms |
| [cbor4ii 0.3.2][cbor4ii] | 897.81 µs | 5.2726 ms | 1407835 | 403440 | 324081 | 4.8529 ms |
| [ciborium 0.2.2][ciborium] | 3.8760 ms | 10.234 ms | 1407835 | 403440 | 324081 | 4.8336 ms |
| [databuf 0.5.0][databuf] | 257.25 µs | 2.0345 ms | 765778 | 311715 | 264630 | 4.1418 ms |
| [dlhn 0.1.6][dlhn] | 661.63 µs | 2.4240 ms | 724953 | 301446 | 253629 | 3.8524 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.3923 ms | † | 1276368 | 468539 | 388832 | 5.4460 ms |
| [msgpacker 0.4.3][msgpacker] | 1.3738 ms | 2.5202 ms | 764996 | 315291 | 264898 | 3.9853 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4265 ms | 4.1449 ms | 818669 | 332556 | 285514 | 4.6100 ms |
| [nanoserde 0.1.37][nanoserde] | 245.62 µs | 2.0808 ms | 1045784 | 373127 | 311761 | 4.5280 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 653.74 µs | 2.2673 ms | 765778 | 311743 | 264518 | 3.9021 ms |
| [postcard 1.0.8][postcard] | 419.93 µs | 2.5332 ms | 724953 | 302399 | 253747 | 3.6994 ms |
| [pot 3.0.0][pot] | 2.2852 ms | 6.5472 ms | 971922 | 372513 | 304122 | 4.9624 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.0321 ms\**</span> <span title="populate + encode">*2.5953 ms\**</span> | 3.4235 ms | 884628 | 363130 | 315494 | 4.7201 ms |
| [rkyv 0.7.44][rkyv] | 219.70 µs | <span title="unvalidated">*1.4528 ms\**</span> <span title="validated upfront with error">*1.9550 ms\**</span> | 1011488 | 383862 | 333545 | 4.9037 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3536 ms | 3.4946 ms | 784997 | 325384 | 278219 | 4.1147 ms |
| [ron 0.8.1][ron] | 14.017 ms | 16.789 ms | 1607459 | 449158 | 349713 | 5.7289 ms |
| [savefile 0.16.5][savefile] | 200.62 µs | 2.1233 ms | 1045800 | 373139 | 311755 | 4.4853 ms |
| [serde_bare 0.5.0][serde_bare] | 662.85 µs | 2.1260 ms | 765778 | 311715 | 264630 | 3.8322 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8592 ms | 5.3374 ms | 1407835 | 403440 | 324081 | 4.7714 ms |
| [serde_json 1.0.115][serde_json] | 3.8204 ms | 5.8878 ms | 1827461 | 470560 | 361090 | 5.5891 ms |
| [simd-json 0.13.9][simd-json] | 2.0739 ms | 4.9646 ms | 1827461 | 470560 | 361090 | 5.9703 ms |
| [speedy 0.8.7][speedy] | 198.19 µs | 1.7550 ms | 885780 | 362204 | 286514 | 4.2867 ms |
| [wiring 0.1.6][wiring] | 822.20 µs | 6.6041 ms | 1045784 | 337930 | 276188 | 3.9521 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.104 µs\**</span> | <span title="unvalidated">*42.041 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8637 ns\**</span> | <span title="validated on-demand with panic">*24.905 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*65.100 ns\**</span> | <span title="validated on-demand with error">*180.42 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4748 ns\**</span> <span title="validated upfront with error">*1.8249 ms\**</span> | <span title="unvalidated">*51.206 µs\**</span> <span title="validated upfront with error">*1.9026 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2365 ns\**</span> <span title="validated upfront with error">*494.73 µs\**</span> | <span title="unvalidated">*10.501 µs\**</span> <span title="validated upfront with error">*507.36 µs\**</span> | 9.7902 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 35.22% | <span title="unvalidated">*99.78%\**</span> | 41.25% | 55.53% | 55.56% | 35.83% |
| [alkahest 0.1.5][alkahest] | 65.94% | † | 67.29% | 63.60% | 59.00% | 40.69% |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*17.53%\**</span> <span title="only prepend">*17.15%\**</span> | 43.97% | 80.46% | 81.26% | 73.70% | 48.08% |
| [bincode 2.0.0-rc][bincode] | 43.35% | 59.99% | 94.93% | 95.03% | 89.35% | 61.64% |
| [bincode 1.3.3][bincode1] | 26.79% | 71.37% | 67.29% | 77.41% | 73.70% | 50.84% |
| [bitcode 0.6.0][bitcode] | 100.00% | 96.92% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 25.79% | 66.02% | 79.45% | 79.74% | 80.19% | 53.32% |
| [bson 2.9.0][bson] | 6.12% | 20.58% | 36.56% | 54.21% | 61.06% | 42.10% |
| [capnp 0.18.13][capnp] | 28.30% | † | 48.76% | 56.19% | 53.60% | 35.64% |
| [cbor4ii 0.3.2][cbor4ii] | 15.50% | 27.55% | 49.99% | 71.59% | 70.89% | 50.06% |
| [ciborium 0.2.2][ciborium] | 3.59% | 14.20% | 49.99% | 71.59% | 70.89% | 50.26% |
| [databuf 0.5.0][databuf] | 54.09% | 71.41% | 91.89% | 92.66% | 86.82% | 58.66% |
| [dlhn 0.1.6][dlhn] | 21.03% | 59.93% | 97.07% | 95.81% | 90.59% | 63.06% |
| [flatbuffers 23.5.26][flatbuffers] | 9.99% | † | 55.13% | 61.64% | 59.09% | 44.61% |
| [msgpacker 0.4.3][msgpacker] | 10.13% | 57.65% | 91.99% | 91.61% | 86.73% | 60.96% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.56% | 35.05% | 85.96% | 86.85% | 80.47% | 52.70% |
| [nanoserde 0.1.37][nanoserde] | 56.65% | 69.82% | 67.29% | 77.41% | 73.70% | 53.66% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 21.28% | 64.08% | 91.89% | 92.65% | 86.86% | 62.26% |
| [postcard 1.0.8][postcard] | 33.13% | 57.35% | 97.07% | 95.51% | 90.54% | 65.67% |
| [pot 3.0.0][pot] | 6.09% | 22.19% | 72.40% | 77.53% | 75.55% | 48.96% |
| [prost 0.12.4][prost] | <span title="encode">*13.48%\**</span> <span title="populate + encode">*5.36%\**</span> | 42.44% | 79.55% | 79.54% | 72.82% | 51.47% |
| [rkyv 0.7.44][rkyv] | 63.33% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*74.31%\**</span> | 69.57% | 75.24% | 68.88% | 49.54% |
| [rmp-serde 1.1.2][rmp-serde] | 10.28% | 41.57% | 89.64% | 88.76% | 82.58% | 59.04% |
| [ron 0.8.1][ron] | 0.99% | 8.65% | 43.78% | 64.30% | 65.70% | 42.41% |
| [savefile 0.16.5][savefile] | 69.35% | 68.42% | 67.29% | 77.40% | 73.70% | 54.17% |
| [serde_bare 0.5.0][serde_bare] | 20.99% | 68.33% | 91.89% | 92.66% | 86.82% | 63.40% |
| [serde_cbor 0.11.2][serde_cbor] | 7.48% | 27.22% | 49.99% | 71.59% | 70.89% | 50.92% |
| [serde_json 1.0.115][serde_json] | 3.64% | 24.67% | 38.51% | 61.38% | 63.63% | 43.47% |
| [simd-json 0.13.9][simd-json] | 6.71% | 29.26% | 38.51% | 61.38% | 63.63% | 40.69% |
| [speedy 0.8.7][speedy] | 70.21% | 82.78% | 79.45% | 79.74% | 80.19% | 56.68% |
| [wiring 0.1.6][wiring] | 16.92% | 22.00% | 67.29% | 85.47% | 83.19% | 61.47% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*24.98%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.35%\**</span> | <span title="validated on-demand with panic">*42.16%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.90%\**</span> | <span title="validated on-demand with error">*5.82%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.51%\**</span> <span title="validated upfront with error">*0.55%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.07%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 260.04 µs | <span title="unvalidated">*259.91 µs\**</span> | 6000024 | 5378513 | 5345890 | 7.4788 ms |
| [alkahest 0.1.5][alkahest] | 201.39 µs | † | 6000008 | 5378500 | 5345890 | 7.4848 ms |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*6.5787 ms\**</span> <span title="only prepend">*8.5243 ms\**</span> | 8.3761 ms | 8625005 | 6443961 | 6231572 | 72.157 ms |
| [bincode 2.0.0-rc][bincode] | 2.3950 ms | 1.4073 ms | 6000005 | 5378497 | 5345897 | 7.4470 ms |
| [bincode 1.3.3][bincode1] | 5.0388 ms | 4.3507 ms | 6000008 | 5378500 | 5345890 | 7.5185 ms |
| [bitcode 0.6.0][bitcode] | 1.4255 ms | 604.75 µs | 6000006 | 5182295 | 4923880 | 12.531 ms |
| [borsh 1.3.1][borsh] | 6.0297 ms | 4.3226 ms | 6000004 | 5378496 | 5345889 | 7.7417 ms |
| [bson 2.9.0][bson] | 43.461 ms | 80.375 ms | 23013911 | 9212089 | 7497811 | 108.78 ms |
| [capnp 0.18.13][capnp] | 6.0388 ms | † | 14000088 | 7130367 | 6051062 | 80.317 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.521 ms | 47.428 ms | 13125016 | 7524114 | 6757967 | 92.708 ms |
| [ciborium 0.2.2][ciborium] | 63.701 ms | 107.45 ms | 13122324 | 7524660 | 6759658 | 92.575 ms |
| [databuf 0.5.0][databuf] | 2.4014 ms | 5.2777 ms | 6000003 | 5378495 | 5345900 | 7.7248 ms |
| [dlhn 0.1.6][dlhn] | 5.8808 ms | 5.5913 ms | 6000003 | 5378495 | 5345900 | 7.7815 ms |
| [flatbuffers 23.5.26][flatbuffers] | 672.66 µs | † | 6000024 | 5378434 | 5345910 | 7.7638 ms |
| [msgpacker 0.4.3][msgpacker] | 19.548 ms | 8.5433 ms | 7500005 | 6058442 | 6014337 | 9.7812 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 129.72 ms | 26.821 ms | 8125037 | 6493484 | 6386940 | 69.576 ms |
| [nanoserde 0.1.37][nanoserde] | 1.4557 ms | 1.4354 ms | 6000008 | 5378500 | 5345890 | 7.6640 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.7637 ms | 4.0746 ms | 6000004 | 5378496 | 5345889 | 7.7628 ms |
| [postcard 1.0.8][postcard] | 475.38 µs | 1.2550 ms | 6000003 | 5378495 | 5345900 | 8.1285 ms |
| [pot 3.0.0][pot] | 39.573 ms | 71.327 ms | 10122342 | 6814618 | 6852251 | 82.683 ms |
| [prost 0.12.4][prost] | <span title="encode">*13.609 ms\**</span> <span title="populate + encode">*14.887 ms\**</span> | 13.408 ms | 8750000 | 6665735 | 6421871 | 74.723 ms |
| [rkyv 0.7.44][rkyv] | 185.82 µs | <span title="unvalidated">*150.78 µs\**</span> <span title="validated upfront with error">*150.41 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.6518 ms |
| [rmp-serde 1.1.2][rmp-serde] | 12.726 ms | 18.729 ms | 8125006 | 6494876 | 6391037 | 73.536 ms |
| [ron 0.8.1][ron] | 172.07 ms | 254.82 ms | 22192885 | 8970395 | 8138755 | 152.58 ms |
| [savefile 0.16.5][savefile] | 258.91 µs | 259.37 µs | 6000024 | 5378518 | 5345893 | 7.6447 ms |
| [serde_bare 0.5.0][serde_bare] | 6.5515 ms | 4.0216 ms | 6000003 | 5378495 | 5345900 | 7.5387 ms |
| [serde_cbor 0.11.2][serde_cbor] | 34.804 ms | 42.617 ms | 13122324 | 7524660 | 6759658 | 90.661 ms |
| [serde_json 1.0.115][serde_json] | 89.983 ms | 87.761 ms | 26192883 | 9566084 | 8586741 | 157.13 ms |
| [simd-json 0.13.9][simd-json] | 53.773 ms | 73.508 ms | 26192883 | 9566084 | 8586741 | 156.82 ms |
| [speedy 0.8.7][speedy] | 259.02 µs | 260.61 µs | 6000004 | 5378496 | 5345889 | 7.6087 ms |
| [wiring 0.1.6][wiring] | 9.1597 ms | 24.963 ms | 6000008 | 5378952 | 5345894 | 7.4748 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1647 ns\**</span> | <span title="unvalidated">*142.04 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8553 ns\**</span> | <span title="validated on-demand with panic">*77.328 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*95.586 ns\**</span> | <span title="validated on-demand with error">*2.1549 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4740 ns\**</span> <span title="validated upfront with error">*38.229 ns\**</span> | <span title="unvalidated">*54.096 µs\**</span> <span title="validated upfront with error">*77.391 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2371 ns\**</span> <span title="validated upfront with error">*9.9043 ns\**</span> | <span title="unvalidated">*46.080 µs\**</span> <span title="validated upfront with error">*38.720 µs\**</span> | 101.43 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 71.46% | <span title="unvalidated">*57.87%\**</span> | 100.00% | 96.35% | 92.11% | 99.57% |
| [alkahest 0.1.5][alkahest] | 92.27% | † | 100.00% | 96.35% | 92.11% | 99.49% |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*2.82%\**</span> <span title="only prepend">*2.18%\**</span> | 1.80% | 69.57% | 80.42% | 79.02% | 10.32% |
| [bincode 2.0.0-rc][bincode] | 7.76% | 10.69% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bincode 1.3.3][bincode1] | 3.69% | 3.46% | 100.00% | 96.35% | 92.11% | 99.05% |
| [bitcode 0.6.0][bitcode] | 13.04% | 24.87% | 100.00% | 100.00% | 100.00% | 59.43% |
| [borsh 1.3.1][borsh] | 3.08% | 3.48% | 100.00% | 96.35% | 92.11% | 96.19% |
| [bson 2.9.0][bson] | 0.43% | 0.19% | 26.07% | 56.26% | 65.67% | 6.85% |
| [capnp 0.18.13][capnp] | 3.08% | † | 42.86% | 72.68% | 81.37% | 9.27% |
| [cbor4ii 0.3.2][cbor4ii] | 1.77% | 0.32% | 45.71% | 68.88% | 72.86% | 8.03% |
| [ciborium 0.2.2][ciborium] | 0.29% | 0.14% | 45.72% | 68.87% | 72.84% | 8.04% |
| [databuf 0.5.0][databuf] | 7.74% | 2.85% | 100.00% | 96.35% | 92.11% | 96.40% |
| [dlhn 0.1.6][dlhn] | 3.16% | 2.69% | 100.00% | 96.35% | 92.11% | 95.70% |
| [flatbuffers 23.5.26][flatbuffers] | 27.62% | † | 100.00% | 96.35% | 92.11% | 95.92% |
| [msgpacker 0.4.3][msgpacker] | 0.95% | 1.76% | 80.00% | 85.54% | 81.87% | 76.14% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.14% | 0.56% | 73.85% | 79.81% | 77.09% | 10.70% |
| [nanoserde 0.1.37][nanoserde] | 12.76% | 10.48% | 100.00% | 96.35% | 92.11% | 97.17% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.90% | 3.69% | 100.00% | 96.35% | 92.11% | 95.93% |
| [postcard 1.0.8][postcard] | 39.09% | 11.98% | 100.00% | 96.35% | 92.11% | 91.62% |
| [pot 3.0.0][pot] | 0.47% | 0.21% | 59.27% | 76.05% | 71.86% | 9.01% |
| [prost 0.12.4][prost] | <span title="encode">*1.37%\**</span> <span title="populate + encode">*1.25%\**</span> | 1.12% | 68.57% | 77.75% | 76.67% | 9.97% |
| [rkyv 0.7.44][rkyv] | 100.00% | <span title="unvalidated">*99.75%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 97.32% |
| [rmp-serde 1.1.2][rmp-serde] | 1.46% | 0.80% | 73.85% | 79.79% | 77.04% | 10.13% |
| [ron 0.8.1][ron] | 0.11% | 0.06% | 27.04% | 57.77% | 60.50% | 4.88% |
| [savefile 0.16.5][savefile] | 71.77% | 57.99% | 100.00% | 96.35% | 92.11% | 97.41% |
| [serde_bare 0.5.0][serde_bare] | 2.84% | 3.74% | 100.00% | 96.35% | 92.11% | 98.78% |
| [serde_cbor 0.11.2][serde_cbor] | 0.53% | 0.35% | 45.72% | 68.87% | 72.84% | 8.21% |
| [serde_json 1.0.115][serde_json] | 0.21% | 0.17% | 22.91% | 54.17% | 57.34% | 4.74% |
| [simd-json 0.13.9][simd-json] | 0.35% | 0.20% | 22.91% | 54.17% | 57.34% | 4.75% |
| [speedy 0.8.7][speedy] | 71.74% | 57.71% | 100.00% | 96.35% | 92.11% | 97.87% |
| [wiring 0.1.6][wiring] | 2.03% | 0.60% | 100.00% | 96.34% | 92.11% | 99.63% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.15%\**</span> | <span title="unvalidated">*27.26%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.68%\**</span> | <span title="validated on-demand with panic">*50.07%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.29%\**</span> | <span title="validated on-demand with error">*1.80%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*3.24%\**</span> | <span title="unvalidated">*71.58%\**</span> <span title="validated upfront with error">*50.03%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*12.49%\**</span> | <span title="unvalidated">*84.03%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 189.82 µs | <span title="unvalidated">*1.3151 ms\**</span> | 1290592 | 396000 | 339881 | 4.9791 ms |
| [alkahest 0.1.5][alkahest] | 224.55 µs | † | 667570 | 325484 | 320452 | 3.8812 ms |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*1.0056 ms\**</span> <span title="only prepend">*851.02 µs\**</span> <span title="populate + encode">*2.3443 ms\**</span> <span title="populate + prepend">*2.1854 ms\**</span> | 3.3215 ms | 476547 | 281071 | 249907 | 3.0381 ms |
| [bincode 2.0.0-rc][bincode] | 306.19 µs | 2.0645 ms | 367413 | 221291 | 206273 | 2.4596 ms |
| [bincode 1.3.3][bincode1] | 561.49 µs | 1.7287 ms | 569975 | 240525 | 232423 | 2.9416 ms |
| [bitcode 0.6.0][bitcode] | 131.83 µs | 1.2629 ms | 327688 | 200947 | 182736 | 745.66 µs |
| [borsh 1.3.1][borsh] | 556.76 µs | 1.8092 ms | 446595 | 234236 | 210008 | 2.4971 ms |
| [bson 2.9.0][bson] | 2.9527 ms | 8.2564 ms | 1619653 | 502185 | 328399 | 4.8727 ms |
| [capnp 0.18.13][capnp] | 471.70 µs | † | 803896 | 335606 | 280851 | 3.9213 ms |
| [cbor4ii 0.3.2][cbor4ii] | 792.34 µs | 4.6349 ms | 1109831 | 344745 | 274514 | 3.7973 ms |
| [ciborium 0.2.2][ciborium] | 3.6236 ms | 9.6298 ms | 1109821 | 344751 | 274526 | 3.7965 ms |
| [databuf 0.5.0][databuf] | 316.70 µs | 1.7160 ms | 356311 | 213062 | 198488 | 2.3767 ms |
| [dlhn 0.1.6][dlhn] | 715.28 µs | 2.5051 ms | 366496 | 220600 | 205683 | 2.4814 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.3967 ms | † | 844168 | 345696 | 294015 | 3.8193 ms |
| [msgpacker 0.4.3][msgpacker] | 933.65 µs | 2.8044 ms | 391251 | 236877 | 220476 | 2.6125 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1728 ms | 3.9198 ms | 449745 | 252432 | 231110 | 2.7659 ms |
| [nanoserde 0.1.37][nanoserde] | 288.88 µs | 1.8833 ms | 567975 | 239930 | 232419 | 2.8744 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 640.90 µs | 1.9917 ms | 356311 | 212976 | 198524 | 2.3696 ms |
| [postcard 1.0.8][postcard] | 441.58 µs | 1.9305 ms | 367489 | 221913 | 207344 | 2.5296 ms |
| [pot 3.0.0][pot] | 2.5181 ms | 5.9913 ms | 599125 | 299158 | 247693 | 3.1517 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.3255 ms\**</span> <span title="populate + encode">*2.9778 ms\**</span> | 3.5200 ms | 596811 | 305319 | 269310 | 3.4402 ms |
| [rkyv 0.7.44][rkyv] | 303.38 µs | <span title="unvalidated">*1.2506 ms\**</span> <span title="validated upfront with error">*1.7451 ms\**</span> | 596952 | 253967 | 220706 | 2.7179 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.4454 ms | 2.9523 ms | 424533 | 245214 | 226188 | 2.6769 ms |
| [ron 0.8.1][ron] | 8.1785 ms | 16.735 ms | 1465223 | 434935 | 343338 | 5.8600 ms |
| [savefile 0.16.5][savefile] | 219.30 µs | 1.8118 ms | 566991 | 239361 | 232010 | 2.8906 ms |
| [serde_bare 0.5.0][serde_bare] | 718.63 µs | 2.1893 ms | 356311 | 213062 | 198488 | 2.4198 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7462 ms | 4.6282 ms | 1109821 | 344751 | 274526 | 3.8696 ms |
| [serde_json 1.0.115][serde_json] | 3.6942 ms | 6.7177 ms | 1623191 | 466527 | 359623 | 6.1120 ms |
| [simd-json 0.13.9][simd-json] | 2.2209 ms | 4.5569 ms | 1623191 | 466527 | 359623 | 6.0950 ms |
| [speedy 0.8.7][speedy] | 274.85 µs | 1.6344 ms | 449595 | 234970 | 210361 | 2.4787 ms |
| [wiring 0.1.6][wiring] | 887.69 µs | 5.1961 ms | 566975 | 247810 | 225259 | 2.9739 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*36.900 µs\**</span> | <span title="unvalidated">*37.461 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8551 ns\**</span> | <span title="validated on-demand with panic">*4.6365 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*65.392 ns\**</span> | <span title="validated on-demand with error">*416.38 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4752 ns\**</span> <span title="validated upfront with error">*1.8260 ms\**</span> | <span title="unvalidated">*1.3867 µs\**</span> <span title="validated upfront with error">*1.8256 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2368 ns\**</span> <span title="validated upfront with error">*488.89 µs\**</span> | <span title="unvalidated">*238.87 ns\**</span> <span title="validated upfront with error">*491.55 µs\**</span> | 888.02 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 69.45% | <span title="unvalidated">*95.10%\**</span> | 25.39% | 50.74% | 53.76% | 14.98% |
| [alkahest 0.1.5][alkahest] | 58.71% | † | 49.09% | 61.74% | 57.02% | 19.21% |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*13.11%\**</span> <span title="only prepend">*15.49%\**</span> <span title="populate + encode">*5.62%\**</span> <span title="populate + prepend">*6.03%\**</span> | 37.65% | 68.76% | 71.49% | 73.12% | 24.54% |
| [bincode 2.0.0-rc][bincode] | 43.05% | 60.58% | 89.19% | 90.81% | 88.59% | 30.32% |
| [bincode 1.3.3][bincode1] | 23.48% | 72.34% | 57.49% | 83.55% | 78.62% | 25.35% |
| [bitcode 0.6.0][bitcode] | 100.00% | 99.03% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 23.68% | 69.12% | 73.37% | 85.79% | 87.01% | 29.86% |
| [bson 2.9.0][bson] | 4.46% | 15.15% | 20.23% | 40.01% | 55.64% | 15.30% |
| [capnp 0.18.13][capnp] | 27.95% | † | 40.76% | 59.88% | 65.07% | 19.02% |
| [cbor4ii 0.3.2][cbor4ii] | 16.64% | 26.98% | 29.53% | 58.29% | 66.57% | 19.64% |
| [ciborium 0.2.2][ciborium] | 3.64% | 12.99% | 29.53% | 58.29% | 66.56% | 19.64% |
| [databuf 0.5.0][databuf] | 41.63% | 72.88% | 91.97% | 94.31% | 92.06% | 31.37% |
| [dlhn 0.1.6][dlhn] | 18.43% | 49.92% | 89.41% | 91.09% | 88.84% | 30.05% |
| [flatbuffers 23.5.26][flatbuffers] | 3.88% | † | 38.82% | 58.13% | 62.15% | 19.52% |
| [msgpacker 0.4.3][msgpacker] | 14.12% | 44.59% | 83.75% | 84.83% | 82.88% | 28.54% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.55% | 31.90% | 72.86% | 79.60% | 79.07% | 26.96% |
| [nanoserde 0.1.37][nanoserde] | 45.63% | 66.40% | 57.69% | 83.75% | 78.62% | 25.94% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.57% | 62.79% | 91.97% | 94.35% | 92.05% | 31.47% |
| [postcard 1.0.8][postcard] | 29.85% | 64.78% | 89.17% | 90.55% | 88.13% | 29.48% |
| [pot 3.0.0][pot] | 5.24% | 20.87% | 54.69% | 67.17% | 73.78% | 23.66% |
| [prost 0.12.4][prost] | <span title="encode">*9.95%\**</span> <span title="populate + encode">*4.43%\**</span> | 35.53% | 54.91% | 65.82% | 67.85% | 21.67% |
| [rkyv 0.7.44][rkyv] | 43.45% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.66%\**</span> | 54.89% | 79.12% | 82.80% | 27.44% |
| [rmp-serde 1.1.2][rmp-serde] | 9.12% | 42.36% | 77.19% | 81.95% | 80.79% | 27.86% |
| [ron 0.8.1][ron] | 1.61% | 7.47% | 22.36% | 46.20% | 53.22% | 12.72% |
| [savefile 0.16.5][savefile] | 60.11% | 69.03% | 57.79% | 83.95% | 78.76% | 25.80% |
| [serde_bare 0.5.0][serde_bare] | 18.34% | 57.12% | 91.97% | 94.31% | 92.06% | 30.82% |
| [serde_cbor 0.11.2][serde_cbor] | 7.55% | 27.02% | 29.53% | 58.29% | 66.56% | 19.27% |
| [serde_json 1.0.115][serde_json] | 3.57% | 18.62% | 20.19% | 43.07% | 50.81% | 12.20% |
| [simd-json 0.13.9][simd-json] | 5.94% | 27.44% | 20.19% | 43.07% | 50.81% | 12.23% |
| [speedy 0.8.7][speedy] | 47.96% | 76.52% | 72.89% | 85.52% | 86.87% | 30.08% |
| [wiring 0.1.6][wiring] | 14.85% | 24.07% | 57.80% | 81.09% | 81.12% | 25.07% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.64%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*5.15%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.89%\**</span> | <span title="validated on-demand with error">*57.37%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.23%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 473.63 µs | <span title="unvalidated">*2.3350 ms\**</span> | 2984682 | 1406955 | 1270206 | 14.546 ms |
| [alkahest 0.1.5][alkahest] | 718.50 µs | † | 1863391 | 1234113 | 1202345 | 11.617 ms |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*4.7339 ms\**</span> <span title="only prepend">*2.8371 ms\**</span> <span title="populate + encode">*7.0072 ms\**</span> <span title="populate + prepend">*5.1283 ms\**</span> | 8.7213 ms | 1664428 | 1263911 | 1215841 | 11.707 ms |
| [bincode 2.0.0-rc][bincode] | 1.2373 ms | 3.7022 ms | 1372381 | 1091486 | 1037296 | 9.0835 ms |
| [bincode 1.3.3][bincode1] | 3.6526 ms | 3.3189 ms | 1811011 | 1115281 | 1025627 | 10.031 ms |
| [bitcode 0.6.0][bitcode] | 712.22 µs | 2.3148 ms | 948499 | 857321 | 837658 | 3.1093 ms |
| [borsh 1.3.1][borsh] | 2.9221 ms | 2.7592 ms | 1486162 | 1082357 | 1013550 | 9.5756 ms |
| [bson 2.9.0][bson] | 22.618 ms | 43.491 ms | 10030880 | 2833079 | 1600859 | 27.902 ms |
| [capnp 0.18.13][capnp] | 2.3979 ms | † | 2664040 | 1511895 | 1212087 | 14.594 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.3145 ms | 17.957 ms | 5878791 | 1655835 | 1431390 | 21.024 ms |
| [ciborium 0.2.2][ciborium] | 22.355 ms | 47.109 ms | 5878653 | 1655791 | 1431560 | 20.864 ms |
| [databuf 0.5.0][databuf] | 1.7764 ms | 3.6276 ms | 1288257 | 1037579 | 984337 | 8.6082 ms |
| [dlhn 0.1.6][dlhn] | 4.7685 ms | 7.3033 ms | 1279599 | 1052061 | 1021161 | 8.2847 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.2019 ms | † | 2273740 | 1408408 | 1235566 | 12.791 ms |
| [msgpacker 0.4.3][msgpacker] | 1.9233 ms | 4.5602 ms | 1424043 | 1128758 | 1110156 | 9.2648 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 31.045 ms | 15.636 ms | 1728519 | 1247642 | 1233323 | 11.761 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3033 ms | 2.8948 ms | 1770477 | 1108304 | 1029947 | 9.8462 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.1283 ms | 3.0581 ms | 1288257 | 1039269 | 986510 | 8.5834 ms |
| [postcard 1.0.8][postcard] | 1.6926 ms | 3.9215 ms | 1279599 | 1058243 | 1016738 | 8.2700 ms |
| [pot 3.0.0][pot] | 13.453 ms | 31.703 ms | 2544810 | 1447453 | 1268390 | 15.176 ms |
| [prost 0.12.4][prost] | <span title="encode">*5.9145 ms\**</span> <span title="populate + encode">*10.062 ms\**</span> | 8.9230 ms | 1818378 | 1307777 | 1266311 | 11.464 ms |
| [rkyv 0.7.44][rkyv] | 1.2571 ms | <span title="unvalidated">*2.1575 ms\**</span> <span title="validated upfront with error">*2.7841 ms\**</span> | 2029080 | 1335117 | 1158855 | 11.635 ms |
| [rmp-serde 1.1.2][rmp-serde] | 10.001 ms | 11.420 ms | 1703813 | 1231892 | 1200208 | 10.915 ms |
| [ron 0.8.1][ron] | 37.040 ms | 94.286 ms | 8476284 | 2181196 | 1783971 | 33.425 ms |
| [savefile 0.16.5][savefile] | 1.0145 ms | 2.6241 ms | 1750226 | 1101682 | 1027827 | 9.8148 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8830 ms | 4.4887 ms | 1288257 | 1037597 | 984356 | 8.4608 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.4373 ms | 21.439 ms | 5878653 | 1655791 | 1431560 | 20.850 ms |
| [serde_json 1.0.115][serde_json] | 20.011 ms | 30.469 ms | 9175594 | 2334253 | 1800713 | 33.842 ms |
| [simd-json 0.13.9][simd-json] | 11.373 ms | 26.791 ms | 9175594 | 2334253 | 1800713 | 33.609 ms |
| [speedy 0.8.7][speedy] | 738.21 µs | 2.5080 ms | 1546963 | 1093532 | 1013443 | 10.043 ms |
| [wiring 0.1.6][wiring] | 4.5346 ms | 13.046 ms | 1750210 | 1129857 | 1058906 | 10.086 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.136 µs\**</span> | <span title="unvalidated">*67.568 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8549 ns\**</span> | <span title="validated on-demand with panic">*626.84 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*65.197 ns\**</span> | <span title="validated on-demand with error">*697.26 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4740 ns\**</span> <span title="validated upfront with error">*4.0018 ms\**</span> | <span title="unvalidated">*2.7114 µs\**</span> <span title="validated upfront with error">*4.0001 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*608.82 µs\**</span> | <span title="unvalidated">*367.09 ns\**</span> <span title="validated upfront with error">*620.27 µs\**</span> | 503.10 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.40%\**</span> | 31.78% | 60.93% | 65.95% | 21.38% |
| [alkahest 0.1.5][alkahest] | 65.92% | † | 50.90% | 69.47% | 69.67% | 26.76% |
| [bilrost 0.1005.1][bilrost] | <span title="only encode">*10.01%\**</span> <span title="only prepend">*16.69%\**</span> <span title="populate + encode">*6.76%\**</span> <span title="populate + prepend">*9.24%\**</span> | 24.74% | 56.99% | 67.83% | 68.90% | 26.56% |
| [bincode 2.0.0-rc][bincode] | 38.28% | 58.28% | 69.11% | 78.55% | 80.75% | 34.23% |
| [bincode 1.3.3][bincode1] | 12.97% | 65.01% | 52.37% | 76.87% | 81.67% | 31.00% |
| [bitcode 0.6.0][bitcode] | 66.50% | 93.20% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 16.21% | 78.19% | 63.82% | 79.21% | 82.65% | 32.47% |
| [bson 2.9.0][bson] | 2.09% | 4.96% | 9.46% | 30.26% | 52.33% | 11.14% |
| [capnp 0.18.13][capnp] | 19.75% | † | 35.60% | 56.71% | 69.11% | 21.30% |
| [cbor4ii 0.3.2][cbor4ii] | 10.98% | 12.01% | 16.13% | 51.78% | 58.52% | 14.79% |
| [ciborium 0.2.2][ciborium] | 2.12% | 4.58% | 16.13% | 51.78% | 58.51% | 14.90% |
| [databuf 0.5.0][databuf] | 26.66% | 59.47% | 73.63% | 82.63% | 85.10% | 36.12% |
| [dlhn 0.1.6][dlhn] | 9.93% | 29.54% | 74.12% | 81.49% | 82.03% | 37.53% |
| [flatbuffers 23.5.26][flatbuffers] | 9.10% | † | 41.72% | 60.87% | 67.80% | 24.31% |
| [msgpacker 0.4.3][msgpacker] | 24.63% | 47.31% | 66.61% | 75.95% | 75.45% | 33.56% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.53% | 13.80% | 54.87% | 68.72% | 67.92% | 26.44% |
| [nanoserde 0.1.37][nanoserde] | 36.34% | 74.53% | 53.57% | 77.35% | 81.33% | 31.58% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 15.14% | 70.55% | 73.63% | 82.49% | 84.91% | 36.22% |
| [postcard 1.0.8][postcard] | 27.98% | 55.02% | 74.12% | 81.01% | 82.39% | 37.60% |
| [pot 3.0.0][pot] | 3.52% | 6.81% | 37.27% | 59.23% | 66.04% | 20.49% |
| [prost 0.12.4][prost] | <span title="encode">*8.01%\**</span> <span title="populate + encode">*4.71%\**</span> | 24.18% | 52.16% | 65.56% | 66.15% | 27.12% |
| [rkyv 0.7.44][rkyv] | 37.68% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.49%\**</span> | 46.75% | 64.21% | 72.28% | 26.72% |
| [rmp-serde 1.1.2][rmp-serde] | 4.74% | 18.89% | 55.67% | 69.59% | 69.79% | 28.49% |
| [ron 0.8.1][ron] | 1.28% | 2.29% | 11.19% | 39.31% | 46.95% | 9.30% |
| [savefile 0.16.5][savefile] | 46.69% | 82.22% | 54.19% | 77.82% | 81.50% | 31.68% |
| [serde_bare 0.5.0][serde_bare] | 9.70% | 48.07% | 73.63% | 82.63% | 85.10% | 36.75% |
| [serde_cbor 0.11.2][serde_cbor] | 5.02% | 10.06% | 16.13% | 51.78% | 58.51% | 14.91% |
| [serde_json 1.0.115][serde_json] | 2.37% | 7.08% | 10.34% | 36.73% | 46.52% | 9.19% |
| [simd-json 0.13.9][simd-json] | 4.16% | 8.05% | 10.34% | 36.73% | 46.52% | 9.25% |
| [speedy 0.8.7][speedy] | 64.16% | 86.02% | 61.31% | 78.40% | 82.65% | 30.96% |
| [wiring 0.1.6][wiring] | 10.44% | 16.54% | 54.19% | 75.88% | 79.11% | 30.83% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.54%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.68%\**</span> | <span title="validated on-demand with panic">*58.56%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.90%\**</span> | <span title="validated on-demand with error">*52.65%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.54%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

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
[wiring]: https://crates.io/crates/wiring/0.1.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
