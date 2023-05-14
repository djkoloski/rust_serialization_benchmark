<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

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

## Last updated: 2023-5-14

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 258.24 µs | <span title="unvalidated">*2.4014 ms\**</span> | 1705800 | 507685 | 403564 |
| [bincode 1.3.3][bincode] | 471.51 µs | 3.3309 ms | 1045784 | 374305 | 311761 |
| [bitcode 0.4.0][bitcode] | 508.91 µs | 3.2650 ms | 703664 | 320922 | 273622 |
| [borsh 0.10.3][borsh] | 495.85 µs | 3.5756 ms | 885780 | 363280 | 286514 |
| [bson 2.6.0][bson] | 2.3932 ms | 9.8929 ms | 1924682 | 537661 | 376270 |
| [capnp 0.16.1][capnp] | 719.76 µs | † | 1443216 | 509618 | 428649 |
| [ciborium 0.2.1][ciborium] | 3.8468 ms | 12.654 ms | 1407835 | 407372 | 324081 |
| [flatbuffers 23.1.21][flatbuffers] | 2.0120 ms | † | 1276368 | 469962 | 388832 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.5280 ms | 5.5383 ms | 818669 | 334639 | 285514 |
| [postcard 1.0.4][postcard] | 413.60 µs | 3.3395 ms | 724953 | 303462 | 253747 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.2179 ms\**</span> <span title="encode">*511.99 µs\**</span> | 3.8634 ms | 764951 | 269811 | 227947 |
| [rkyv 0.7.42][rkyv] | 309.10 µs | <span title="unvalidated">*2.4972 ms\**</span> <span title="validated upfront with error">*3.4520 ms\**</span> | 1011488 | 384478 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.7599 ms | 4.5535 ms | 784997 | 326654 | 278219 |
| [ron 0.8.0][ron] | 20.923 ms | 20.387 ms | 1607459 | 452648 | 349713 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 629.81 µs | 3.3989 ms | 765778 | 312771 | 264518 |
| [serde_bare 0.5.0][serde_bare] | 748.91 µs | 3.4011 ms | 765778 | 312739 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.1744 ms | 6.5003 ms | 1407835 | 407372 | 324081 |
| [serde_json 1.0.96][serde_json] | 4.1553 ms | 8.3258 ms | 1827461 | 474358 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.3924 ms | 5.5556 ms | 1827461 | 474358 | 361090 |
| [speedy 0.8.6][speedy] | 299.47 µs | 2.7806 ms | 885780 | 363280 | 286514 |
| [alkahest 0.1.5][alkahest] | 246.02 µs | † | 1045784 | 454748 | 389424 |
| [dlhn 0.1.5][dlhn] | 612.27 µs | 3.7450 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*28.827 µs\**</span> | <span title="unvalidated">*50.854 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*119.45 ns\**</span> | <span title="validated on-demand with error">*397.97 µs\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6810 ns\**</span> <span title="validated upfront with error">*2.2485 ms\**</span> | <span title="unvalidated">*80.326 µs\**</span> <span title="validated upfront with error">*2.3165 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2063 ns\**</span> <span title="validated upfront with error">*886.45 µs\**</span> | <span title="unvalidated">*17.629 µs\**</span> <span title="validated upfront with error">*921.14 µs\**</span> | 19.891 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.5647 ns\**</span> | <span title="validated on-demand with panic">*34.172 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 95.27% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 53.15% | 56.48% |
| [bincode 1.3.3][bincode] | 52.18% | 72.09% | 67.29% | 72.08% | 73.12% |
| [bitcode 0.4.0][bitcode] | 48.34% | 73.55% | 100.00% | 84.07% | 83.31% |
| [borsh 0.10.3][borsh] | 49.62% | 67.16% | 79.44% | 74.27% | 79.56% |
| [bson 2.6.0][bson] | 10.28% | 24.27% | 36.56% | 50.18% | 60.58% |
| [capnp 0.16.1][capnp] | 34.18% | † | 48.76% | 52.94% | 53.18% |
| [ciborium 0.2.1][ciborium] | 6.40% | 18.98% | 49.98% | 66.23% | 70.34% |
| [flatbuffers 23.1.21][flatbuffers] | 12.23% | † | 55.13% | 57.41% | 58.62% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.88% | 43.36% | 85.95% | 80.63% | 79.84% |
| [postcard 1.0.4][postcard] | 59.48% | 71.91% | 97.06% | 88.91% | 89.83% |
| [prost 0.11.9][prost] | <span title="populate + encode">*7.65%\**</span> <span title="encode">*48.05%\**</span> | 62.16% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 79.59% | <span title="unvalidated">*96.16%\**</span> <span title="validated upfront with error">*69.57%\**</span> | 69.57% | 70.18% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 13.98% | 52.74% | 89.64% | 82.60% | 81.93% |
| [ron 0.8.0][ron] | 1.18% | 11.78% | 43.77% | 59.61% | 65.18% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 39.06% | 70.65% | 91.89% | 86.26% | 86.17% |
| [serde_bare 0.5.0][serde_bare] | 32.85% | 70.61% | 91.89% | 86.27% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.31% | 36.94% | 49.98% | 66.23% | 70.34% |
| [serde_json 1.0.96][serde_json] | 5.92% | 28.84% | 38.51% | 56.88% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.28% | 43.22% | 38.51% | 56.88% | 63.13% |
| [speedy 0.8.6][speedy] | 82.15% | 86.36% | 79.44% | 74.27% | 79.56% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.33% | 58.53% |
| [dlhn 0.1.5][dlhn] | 40.18% | 64.12% | 97.06% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*34.67%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.01%\**</span> | <span title="validated on-demand with error">*4.43%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*44.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.95%\**</span> <span title="validated upfront with error">*0.76%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.91%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*47.03%\**</span> | <span title="validated on-demand with panic">*51.59%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 423.82 µs | <span title="unvalidated">*420.82 µs\**</span> | 6000024 | 5380836 | 5345891 |
| [bincode 1.3.3][bincode] | 3.7792 ms | 11.441 ms | 6000008 | 5380823 | 5345890 |
| [bitcode 0.4.0][bitcode] | 4.9081 ms | 9.8707 ms | 4688054 | 4688484 | 4688168 |
| [borsh 0.10.3][borsh] | 5.3395 ms | 6.0930 ms | 6000004 | 5380818 | 5345889 |
| [bson 2.6.0][bson] | 43.388 ms | 109.48 ms | 23013911 | 9211138 | 7497811 |
| [capnp 0.16.1][capnp] | 8.2142 ms | † | 14000088 | 6729881 | 6051062 |
| [ciborium 0.2.1][ciborium] | 89.707 ms | 121.75 ms | 13122324 | 7527423 | 6759658 |
| [flatbuffers 23.1.21][flatbuffers] | 971.28 µs | † | 6000024 | 5380800 | 5345910 |
| [nachricht-serde 0.4.0][nachricht-serde] | 211.21 ms | 33.863 ms | 8125037 | 6495174 | 6386940 |
| [postcard 1.0.4][postcard] | 599.53 µs | 1.6799 ms | 6000003 | 5380817 | 5345900 |
| [prost 0.11.9][prost] | <span title="populate + encode">*8.6723 ms\**</span> <span title="encode">*6.6592 ms\**</span> | 15.352 ms | 8750000 | 6683814 | 6421871 |
| [rkyv 0.7.42][rkyv] | 526.33 µs | <span title="unvalidated">*426.53 µs\**</span> <span title="validated upfront with error">*426.82 µs\**</span> | 6000008 | 5380822 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 19.374 ms | 24.479 ms | 8125006 | 6496879 | 6391037 |
| [ron 0.8.0][ron] | 229.58 ms | 344.12 ms | 22192885 | 9009575 | 8138755 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 5.0361 ms | 7.5103 ms | 6000004 | 5380818 | 5345889 |
| [serde_bare 0.5.0][serde_bare] | 6.8971 ms | 6.4543 ms | 6000003 | 5380817 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 50.510 ms | 51.333 ms | 13122324 | 7527423 | 6759658 |
| [serde_json 1.0.96][serde_json] | 108.22 ms | 96.151 ms | 26192883 | 9612105 | 8586741 |
| [simd-json 0.9.2][simd-json] | 71.121 ms | 105.60 ms | 26192883 | 9612105 | 8586741 |
| [speedy 0.8.6][speedy] | 424.13 µs | 424.04 µs | 6000004 | 5380818 | 5345889 |
| [alkahest 0.1.5][alkahest] | 431.12 µs | † | 6000008 | 5380823 | 5345890 |
| [dlhn 0.1.5][dlhn] | 6.2971 ms | 9.4263 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.0070 ns\**</span> | <span title="unvalidated">*246.10 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*180.72 ns\**</span> | <span title="validated on-demand with error">*5.1578 ms\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6996 ns\**</span> <span title="validated upfront with error">*50.431 ns\**</span> | <span title="unvalidated">*100.41 µs\**</span> <span title="validated upfront with error">*100.47 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2063 ns\**</span> <span title="validated upfront with error">*12.689 ns\**</span> | <span title="unvalidated">*35.152 µs\**</span> <span title="validated upfront with error">*35.776 µs\**</span> | 233.92 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6523 ns\**</span> | <span title="validated on-demand with panic">*100.42 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*100.00%\**</span> | 78.13% | 87.13% | 87.70% |
| [bincode 1.3.3][bincode] | 11.21% | 3.68% | 78.13% | 87.13% | 87.70% |
| [bitcode 0.4.0][bitcode] | 8.64% | 4.26% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 7.94% | 6.91% | 78.13% | 87.13% | 87.70% |
| [bson 2.6.0][bson] | 0.98% | 0.38% | 20.37% | 50.90% | 62.53% |
| [capnp 0.16.1][capnp] | 5.16% | † | 33.49% | 69.67% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.47% | 0.35% | 35.73% | 62.29% | 69.36% |
| [flatbuffers 23.1.21][flatbuffers] | 43.64% | † | 78.13% | 87.13% | 87.70% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.20% | 1.24% | 57.70% | 72.18% | 73.40% |
| [postcard 1.0.4][postcard] | 70.69% | 25.05% | 78.13% | 87.13% | 87.70% |
| [prost 0.11.9][prost] | <span title="populate + encode">*4.89%\**</span> <span title="encode">*6.36%\**</span> | 2.74% | 53.58% | 70.15% | 73.00% |
| [rkyv 0.7.42][rkyv] | 80.52% | <span title="unvalidated">*98.66%\**</span> <span title="validated upfront with error">*98.59%\**</span> | 78.13% | 87.13% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.19% | 1.72% | 57.70% | 72.17% | 73.36% |
| [ron 0.8.0][ron] | 0.18% | 0.12% | 21.12% | 52.04% | 57.60% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 8.42% | 5.60% | 78.13% | 87.13% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 6.14% | 6.52% | 78.13% | 87.13% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.84% | 0.82% | 35.73% | 62.29% | 69.36% |
| [serde_json 1.0.96][serde_json] | 0.39% | 0.44% | 17.90% | 48.78% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.60% | 0.40% | 17.90% | 48.78% | 54.60% |
| [speedy 0.8.6][speedy] | 99.93% | 99.24% | 78.13% | 87.13% | 87.70% |
| [alkahest 0.1.5][alkahest] | 98.31% | † | 78.13% | 87.13% | 87.70% |
| [dlhn 0.1.5][dlhn] | 6.73% | 4.46% | 78.13% | 87.13% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*60.10%\**</span> | <span title="unvalidated">*14.28%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.67%\**</span> | <span title="validated on-demand with error">*0.68%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*44.68%\**</span> <span title="validated upfront with error">*2.39%\**</span> | <span title="unvalidated">*35.01%\**</span> <span title="validated upfront with error">*34.99%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.51%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*98.26%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*45.48%\**</span> | <span title="validated on-demand with panic">*35.00%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 234.08 µs | <span title="unvalidated">*1.9008 ms\**</span> | 1290592 | 391245 | 329256 |
| [bincode 1.3.3][bincode] | 643.66 µs | 2.5988 ms | 569975 | 240897 | 232423 |
| [bitcode 0.4.0][bitcode] | 416.55 µs | 2.7157 ms | 322798 | 215013 | 201247 |
| [borsh 0.10.3][borsh] | 573.27 µs | 2.9773 ms | 446595 | 234395 | 210008 |
| [bson 2.6.0][bson] | 3.3156 ms | 11.440 ms | 1619653 | 506953 | 328399 |
| [capnp 0.16.1][capnp] | 609.45 µs | † | 803896 | 336655 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.7473 ms | 11.497 ms | 1109821 | 347812 | 274526 |
| [dlhn 0.1.5][dlhn] | 781.94 µs | 3.6581 ms | 366496 | 220835 | 205683 |
| [flatbuffers 23.1.21][flatbuffers] | 3.3066 ms | † | 844168 | 346957 | 294015 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.0847 ms | 5.1735 ms | 449745 | 252743 | 231110 |
| [postcard 1.0.4][postcard] | 477.68 µs | 2.8028 ms | 367489 | 222144 | 207344 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.8309 ms\**</span> <span title="encode">*1.2937 ms\**</span> | 4.6976 ms | 596811 | 306728 | 269310 |
| [rkyv 0.7.42][rkyv] | 399.70 µs | <span title="unvalidated">*1.8333 ms\**</span> <span title="validated upfront with error">*2.6346 ms\**</span> | 596952 | 254139 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.8669 ms | 4.0578 ms | 424533 | 245594 | 226188 |
| [ron 0.8.0][ron] | 10.833 ms | 20.714 ms | 1465223 | 439761 | 343338 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 641.50 µs | 2.5865 ms | 356311 | 213188 | 198524 |
| [serde_bare 0.5.0][serde_bare] | 853.55 µs | 3.1669 ms | 356311 | 213270 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0881 ms | 6.5019 ms | 1109821 | 347812 | 274526 |
| [serde_json 1.0.96][serde_json] | 4.3262 ms | 10.119 ms | 1623191 | 472275 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.6942 ms | 5.6572 ms | 1623191 | 472275 | 359623 |
| [speedy 0.8.6][speedy] | 360.32 µs | 2.2246 ms | 449595 | 235136 | 210361 |
| [alkahest 0.1.5][alkahest] | 264.46 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*38.915 µs\**</span> | <span title="unvalidated">*39.716 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*118.03 ns\**</span> | <span title="validated on-demand with error">*658.50 ns\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6937 ns\**</span> <span title="validated upfront with error">*2.1493 ms\**</span> | <span title="unvalidated">*1.9860 µs\**</span> <span title="validated upfront with error">*2.1871 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2062 ns\**</span> <span title="validated upfront with error">*753.38 µs\**</span> | <span title="unvalidated">*145.34 ns\**</span> <span title="validated upfront with error">*752.52 µs\**</span> | 1.6882 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6467 ns\**</span> | <span title="validated on-demand with panic">*6.3086 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*96.45%\**</span> | 25.01% | 54.49% | 60.28% |
| [bincode 1.3.3][bincode] | 36.37% | 70.54% | 56.63% | 88.50% | 85.40% |
| [bitcode 0.4.0][bitcode] | 56.19% | 67.51% | 100.00% | 99.15% | 98.63% |
| [borsh 0.10.3][borsh] | 40.83% | 61.58% | 72.28% | 90.95% | 94.51% |
| [bson 2.6.0][bson] | 7.06% | 16.03% | 19.93% | 42.05% | 60.44% |
| [capnp 0.16.1][capnp] | 38.41% | † | 40.15% | 63.33% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.25% | 15.95% | 29.09% | 61.29% | 72.30% |
| [dlhn 0.1.5][dlhn] | 29.94% | 50.12% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.1.21][flatbuffers] | 7.08% | † | 38.24% | 61.45% | 67.51% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.90% | 35.44% | 71.77% | 84.35% | 85.88% |
| [postcard 1.0.4][postcard] | 49.00% | 65.41% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="populate + encode">*6.11%\**</span> <span title="encode">*18.09%\**</span> | 39.03% | 54.09% | 69.50% | 73.70% |
| [rkyv 0.7.42][rkyv] | 58.56% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*69.59%\**</span> | 54.07% | 83.89% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 12.54% | 45.18% | 76.04% | 86.81% | 87.75% |
| [ron 0.8.0][ron] | 2.16% | 8.85% | 22.03% | 48.48% | 57.81% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 36.49% | 70.88% | 90.59% | 100.00% | 99.98% |
| [serde_bare 0.5.0][serde_bare] | 27.42% | 57.89% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.21% | 28.20% | 29.09% | 61.29% | 72.30% |
| [serde_json 1.0.96][serde_json] | 5.41% | 18.12% | 19.89% | 45.14% | 55.19% |
| [simd-json 0.9.2][simd-json] | 8.69% | 32.41% | 19.89% | 45.14% | 55.19% |
| [speedy 0.8.6][speedy] | 64.96% | 82.41% | 71.80% | 90.67% | 94.36% |
| [alkahest 0.1.5][alkahest] | 88.51% | † | 48.35% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.37%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.02%\**</span> | <span title="validated on-demand with error">*22.07%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*44.78%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.32%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*45.57%\**</span> | <span title="validated on-demand with panic">*2.30%\**</span> | ‡ |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.4.0
[borsh]: https://crates.io/crates/borsh/0.10.3
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.16.1
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[flatbuffers]: https://crates.io/crates/flatbuffers/23.1.21
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[postcard]: https://crates.io/crates/postcard/1.0.4
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.1
[ron]: https://crates.io/crates/ron/0.8.0
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.5.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.96
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[dlhn]: https://crates.io/crates/dlhn/0.1.5




## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
