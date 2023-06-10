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

## Last updated: 2023-6-10

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 248.27 µs | <span title="unvalidated">*2.4201 ms\**</span> | 1705800 | 507682 | 403444 |
| [bincode 1.3.3][bincode] | 443.27 µs | 3.2357 ms | 1045784 | 374305 | 311761 |
| [bitcode 0.4.0][bitcode] | 531.77 µs | 3.5397 ms | 703664 | 320922 | 273622 |
| [borsh 0.10.3][borsh] | 486.47 µs | 3.4306 ms | 885780 | 363280 | 286514 |
| [bson 2.6.0][bson] | 2.4812 ms | 10.232 ms | 1924682 | 537661 | 376270 |
| [capnp 0.16.1][capnp] | 722.69 µs | † | 1443216 | 509618 | 428649 |
| [ciborium 0.2.1][ciborium] | 4.0453 ms | 11.959 ms | 1407835 | 407372 | 324081 |
| [flatbuffers 23.5.26][flatbuffers] | 1.9235 ms | † | 1276368 | 469962 | 388832 |
| [msgpacker 0.4.2][msgpacker] | 1.7519 ms | 3.8805 ms | 764996 | 316445 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.2096 ms | 5.6577 ms | 818669 | 334639 | 285514 |
| [postcard 1.0.4][postcard] | 426.95 µs | 3.5055 ms | 724953 | 303462 | 253747 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.4934 ms\**</span> <span title="encode">*804.13 µs\**</span> | 3.9747 ms | 764951 | 269811 | 227947 |
| [rkyv 0.7.42][rkyv] | 304.33 µs | <span title="unvalidated">*2.4903 ms\**</span> <span title="validated upfront with error">*3.4285 ms\**</span> | 1011488 | 384478 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.5333 ms | 4.6692 ms | 784997 | 326654 | 278219 |
| [ron 0.8.0][ron] | 20.561 ms | 22.241 ms | 1607459 | 452648 | 349713 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 614.79 µs | 3.4331 ms | 765778 | 312771 | 264518 |
| [serde_bare 0.5.0][serde_bare] | 782.90 µs | 3.3888 ms | 765778 | 312739 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0356 ms | 6.9833 ms | 1407835 | 407372 | 324081 |
| [serde_json 1.0.96][serde_json] | 3.9908 ms | 9.0319 ms | 1827461 | 474358 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.2409 ms | 5.7819 ms | 1827461 | 474358 | 361090 |
| [speedy 0.8.6][speedy] | 291.70 µs | 2.8963 ms | 885780 | 363280 | 286514 |
| [alkahest 0.1.5][alkahest] | 240.34 µs | † | 1045784 | 454748 | 389424 |
| [dlhn 0.1.6][dlhn] | 635.44 µs | 3.7925 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*38.180 µs\**</span> | <span title="unvalidated">*62.321 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*107.73 ns\**</span> | <span title="validated on-demand with error">*398.20 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2695 ns\**</span> <span title="validated upfront with error">*2.0129 ms\**</span> | <span title="unvalidated">*97.666 µs\**</span> <span title="validated upfront with error">*2.1532 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5218 ns\**</span> <span title="validated upfront with error">*922.91 µs\**</span> | <span title="unvalidated">*16.377 µs\**</span> <span title="validated upfront with error">*939.87 µs\**</span> | 18.803 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7675 ns\**</span> | <span title="validated on-demand with panic">*57.728 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 96.81% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 53.15% | 56.50% |
| [bincode 1.3.3][bincode] | 54.22% | 74.79% | 67.29% | 72.08% | 73.12% |
| [bitcode 0.4.0][bitcode] | 45.20% | 68.37% | 100.00% | 84.07% | 83.31% |
| [borsh 0.10.3][borsh] | 49.40% | 70.54% | 79.44% | 74.27% | 79.56% |
| [bson 2.6.0][bson] | 9.69% | 23.65% | 36.56% | 50.18% | 60.58% |
| [capnp 0.16.1][capnp] | 33.26% | † | 48.76% | 52.94% | 53.18% |
| [ciborium 0.2.1][ciborium] | 5.94% | 20.24% | 49.98% | 66.23% | 70.34% |
| [flatbuffers 23.5.26][flatbuffers] | 12.49% | † | 55.13% | 57.41% | 58.62% |
| [msgpacker 0.4.2][msgpacker] | 13.72% | 62.37% | 91.98% | 85.26% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.93% | 42.78% | 85.95% | 80.63% | 79.84% |
| [postcard 1.0.4][postcard] | 56.29% | 69.04% | 97.06% | 88.91% | 89.83% |
| [prost 0.11.9][prost] | <span title="populate + encode">*6.88%\**</span> <span title="encode">*29.89%\**</span> | 60.89% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 78.97% | <span title="unvalidated">*97.18%\**</span> <span title="validated upfront with error">*70.59%\**</span> | 69.57% | 70.18% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 15.67% | 51.83% | 89.64% | 82.60% | 81.93% |
| [ron 0.8.0][ron] | 1.17% | 10.88% | 43.77% | 59.61% | 65.18% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 39.09% | 70.49% | 91.89% | 86.26% | 86.17% |
| [serde_bare 0.5.0][serde_bare] | 30.70% | 71.41% | 91.89% | 86.27% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.81% | 34.66% | 49.98% | 66.23% | 70.34% |
| [serde_json 1.0.96][serde_json] | 6.02% | 26.80% | 38.51% | 56.88% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.73% | 41.86% | 38.51% | 56.88% | 63.13% |
| [speedy 0.8.6][speedy] | 82.39% | 83.56% | 79.44% | 74.27% | 79.56% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.33% | 58.53% |
| [dlhn 0.1.6][dlhn] | 37.82% | 63.81% | 97.06% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*26.28%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*4.11%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.55%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.77%\**</span> <span title="validated upfront with error">*0.76%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.74%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*54.99%\**</span> | <span title="validated on-demand with panic">*28.37%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 472.08 µs | <span title="unvalidated">*471.75 µs\**</span> | 6000024 | 5380836 | 5345890 |
| [bincode 1.3.3][bincode] | 4.7172 ms | 8.5246 ms | 6000008 | 5380823 | 5345890 |
| [bitcode 0.4.0][bitcode] | 4.8896 ms | 8.4733 ms | 4688054 | 4688484 | 4688168 |
| [borsh 0.10.3][borsh] | 5.6627 ms | 5.5066 ms | 6000004 | 5380818 | 5345889 |
| [bson 2.6.0][bson] | 43.504 ms | 109.66 ms | 23013911 | 9211138 | 7497811 |
| [capnp 0.16.1][capnp] | 11.023 ms | † | 14000088 | 6729881 | 6051062 |
| [ciborium 0.2.1][ciborium] | 88.861 ms | 110.30 ms | 13122324 | 7527423 | 6759658 |
| [flatbuffers 23.5.26][flatbuffers] | 1.2710 ms | † | 6000024 | 5380800 | 5345910 |
| [msgpacker 0.4.2][msgpacker] | 19.516 ms | 9.8133 ms | 7500005 | 6059282 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 200.38 ms | 36.114 ms | 8125037 | 6495174 | 6386940 |
| [postcard 1.0.4][postcard] | 796.30 µs | 1.5943 ms | 6000003 | 5380817 | 5345900 |
| [prost 0.11.9][prost] | <span title="populate + encode">*12.023 ms\**</span> <span title="encode">*10.549 ms\**</span> | 19.192 ms | 8750000 | 6683814 | 6421871 |
| [rkyv 0.7.42][rkyv] | 521.77 µs | <span title="unvalidated">*473.33 µs\**</span> <span title="validated upfront with error">*473.65 µs\**</span> | 6000008 | 5380822 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 19.707 ms | 22.818 ms | 8125006 | 6496879 | 6391037 |
| [ron 0.8.0][ron] | 223.74 ms | 398.51 ms | 22192885 | 9009575 | 8138755 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 3.7223 ms | 4.7848 ms | 6000004 | 5380818 | 5345889 |
| [serde_bare 0.5.0][serde_bare] | 6.5334 ms | 4.4952 ms | 6000003 | 5380817 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 45.652 ms | 58.447 ms | 13122324 | 7527423 | 6759658 |
| [serde_json 1.0.96][serde_json] | 104.86 ms | 101.93 ms | 26192883 | 9612105 | 8586741 |
| [simd-json 0.9.2][simd-json] | 65.003 ms | 120.69 ms | 26192883 | 9612105 | 8586741 |
| [speedy 0.8.6][speedy] | 471.54 µs | 491.48 µs | 6000004 | 5380818 | 5345889 |
| [alkahest 0.1.5][alkahest] | 475.43 µs | † | 6000008 | 5380823 | 5345890 |
| [dlhn 0.1.6][dlhn] | 5.8801 ms | 8.4749 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.4768 ns\**</span> | <span title="unvalidated">*266.65 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*184.24 ns\**</span> | <span title="validated on-demand with error">*5.5295 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2704 ns\**</span> <span title="validated upfront with error">*44.079 ns\**</span> | <span title="unvalidated">*83.681 µs\**</span> <span title="validated upfront with error">*83.766 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5218 ns\**</span> <span title="validated upfront with error">*15.631 ns\**</span> | <span title="unvalidated">*47.021 µs\**</span> <span title="validated upfront with error">*47.236 µs\**</span> | 252.79 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7680 ns\**</span> | <span title="validated on-demand with panic">*83.677 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 99.89% | <span title="unvalidated">*100.00%\**</span> | 78.13% | 87.13% | 87.70% |
| [bincode 1.3.3][bincode] | 10.00% | 5.53% | 78.13% | 87.13% | 87.70% |
| [bitcode 0.4.0][bitcode] | 9.64% | 5.57% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 8.33% | 8.57% | 78.13% | 87.13% | 87.70% |
| [bson 2.6.0][bson] | 1.08% | 0.43% | 20.37% | 50.90% | 62.53% |
| [capnp 0.16.1][capnp] | 4.28% | † | 33.49% | 69.67% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.53% | 0.43% | 35.73% | 62.29% | 69.36% |
| [flatbuffers 23.5.26][flatbuffers] | 37.10% | † | 78.13% | 87.13% | 87.70% |
| [msgpacker 0.4.2][msgpacker] | 2.42% | 4.81% | 62.51% | 77.38% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.24% | 1.31% | 57.70% | 72.18% | 73.40% |
| [postcard 1.0.4][postcard] | 59.22% | 29.59% | 78.13% | 87.13% | 87.70% |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.92%\**</span> <span title="encode">*4.47%\**</span> | 2.46% | 53.58% | 70.15% | 73.00% |
| [rkyv 0.7.42][rkyv] | 90.37% | <span title="unvalidated">*99.67%\**</span> <span title="validated upfront with error">*99.60%\**</span> | 78.13% | 87.13% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.39% | 2.07% | 57.70% | 72.17% | 73.36% |
| [ron 0.8.0][ron] | 0.21% | 0.12% | 21.12% | 52.04% | 57.60% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 12.67% | 9.86% | 78.13% | 87.13% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 7.22% | 10.49% | 78.13% | 87.13% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 1.03% | 0.81% | 35.73% | 62.29% | 69.36% |
| [serde_json 1.0.96][serde_json] | 0.45% | 0.46% | 17.90% | 48.78% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.73% | 0.39% | 17.90% | 48.78% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 95.99% | 78.13% | 87.13% | 87.70% |
| [alkahest 0.1.5][alkahest] | 99.18% | † | 78.13% | 87.13% | 87.70% |
| [dlhn 0.1.6][dlhn] | 8.02% | 5.57% | 78.13% | 87.13% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.44%\**</span> | <span title="unvalidated">*17.63%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.83%\**</span> | <span title="validated on-demand with error">*0.85%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.53%\**</span> <span title="validated upfront with error">*3.45%\**</span> | <span title="unvalidated">*56.19%\**</span> <span title="validated upfront with error">*56.13%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.74%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.54%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*54.98%\**</span> | <span title="validated on-demand with panic">*56.19%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 252.67 µs | <span title="unvalidated">*1.9810 ms\**</span> | 1290592 | 392314 | 331622 |
| [bincode 1.3.3][bincode] | 667.34 µs | 2.6252 ms | 569975 | 240897 | 232423 |
| [bitcode 0.4.0][bitcode] | 429.74 µs | 2.7834 ms | 322798 | 215013 | 201247 |
| [borsh 0.10.3][borsh] | 582.89 µs | 2.7333 ms | 446595 | 234395 | 210008 |
| [bson 2.6.0][bson] | 3.5652 ms | 11.702 ms | 1619653 | 506953 | 328399 |
| [capnp 0.16.1][capnp] | 605.26 µs | † | 803896 | 336655 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.7177 ms | 10.086 ms | 1109821 | 347812 | 274526 |
| [dlhn 0.1.6][dlhn] | 768.29 µs | 3.7714 ms | 366496 | 220835 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.3858 ms | † | 844168 | 346957 | 294015 |
| [msgpacker 0.4.2][msgpacker] | 1.1920 ms | 3.9606 ms | 391251 | 237270 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.6412 ms | 5.1506 ms | 449745 | 252743 | 231110 |
| [postcard 1.0.4][postcard] | 492.31 µs | 2.9046 ms | 367489 | 222144 | 207344 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.9834 ms\**</span> <span title="encode">*1.4044 ms\**</span> | 4.7040 ms | 596811 | 306728 | 269310 |
| [rkyv 0.7.42][rkyv] | 417.12 µs | <span title="unvalidated">*1.9070 ms\**</span> <span title="validated upfront with error">*2.6349 ms\**</span> | 596952 | 254139 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.7426 ms | 4.0547 ms | 424533 | 245594 | 226188 |
| [ron 0.8.0][ron] | 10.586 ms | 22.662 ms | 1465223 | 439761 | 343338 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 655.93 µs | 2.6719 ms | 356311 | 213188 | 198524 |
| [serde_bare 0.5.0][serde_bare] | 908.46 µs | 3.3008 ms | 356311 | 213270 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0683 ms | 6.6562 ms | 1109821 | 347812 | 274526 |
| [serde_json 1.0.96][serde_json] | 4.2883 ms | 10.210 ms | 1623191 | 472275 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.5443 ms | 5.6451 ms | 1623191 | 472275 | 359623 |
| [speedy 0.8.6][speedy] | 419.31 µs | 2.4720 ms | 449595 | 235136 | 210361 |
| [alkahest 0.1.5][alkahest] | 325.89 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*64.066 µs\**</span> | <span title="unvalidated">*66.077 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*107.06 ns\**</span> | <span title="validated on-demand with error">*651.53 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2724 ns\**</span> <span title="validated upfront with error">*2.2691 ms\**</span> | <span title="unvalidated">*2.9017 µs\**</span> <span title="validated upfront with error">*2.2735 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5278 ns\**</span> <span title="validated upfront with error">*715.87 µs\**</span> | <span title="unvalidated">*189.58 ns\**</span> <span title="validated upfront with error">*716.40 µs\**</span> | 1.5781 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7679 ns\**</span> | <span title="validated on-demand with panic">*6.9177 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*96.26%\**</span> | 25.01% | 54.34% | 59.85% |
| [bincode 1.3.3][bincode] | 37.86% | 72.64% | 56.63% | 88.50% | 85.40% |
| [bitcode 0.4.0][bitcode] | 58.80% | 68.51% | 100.00% | 99.15% | 98.63% |
| [borsh 0.10.3][borsh] | 43.35% | 69.77% | 72.28% | 90.95% | 94.51% |
| [bson 2.6.0][bson] | 7.09% | 16.30% | 19.93% | 42.05% | 60.44% |
| [capnp 0.16.1][capnp] | 41.75% | † | 40.15% | 63.33% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.80% | 18.91% | 29.09% | 61.29% | 72.30% |
| [dlhn 0.1.6][dlhn] | 32.89% | 50.56% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 7.46% | † | 38.24% | 61.45% | 67.51% |
| [msgpacker 0.4.2][msgpacker] | 21.20% | 48.15% | 82.50% | 89.85% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.31% | 37.02% | 71.77% | 84.35% | 85.88% |
| [postcard 1.0.4][postcard] | 51.32% | 65.65% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="populate + encode">*6.34%\**</span> <span title="encode">*17.99%\**</span> | 40.54% | 54.09% | 69.50% | 73.70% |
| [rkyv 0.7.42][rkyv] | 60.57% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.37%\**</span> | 54.07% | 83.89% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 14.50% | 47.03% | 76.04% | 86.81% | 87.75% |
| [ron 0.8.0][ron] | 2.39% | 8.41% | 22.03% | 48.48% | 57.81% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 38.52% | 71.37% | 90.59% | 100.00% | 99.98% |
| [serde_bare 0.5.0][serde_bare] | 27.81% | 57.77% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 12.22% | 28.65% | 29.09% | 61.29% | 72.30% |
| [serde_json 1.0.96][serde_json] | 5.89% | 18.68% | 19.89% | 45.14% | 55.19% |
| [simd-json 0.9.2][simd-json] | 9.93% | 33.78% | 19.89% | 45.14% | 55.19% |
| [speedy 0.8.6][speedy] | 60.26% | 77.14% | 71.80% | 90.67% | 94.36% |
| [alkahest 0.1.5][alkahest] | 77.53% | † | 48.35% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.29%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.43%\**</span> | <span title="validated on-demand with error">*29.10%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.69%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.53%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.20%\**</span> | <span title="validated on-demand with panic">*2.74%\**</span> | ‡ |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.4.0
[borsh]: https://crates.io/crates/borsh/0.10.3
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.16.1
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.2
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
[dlhn]: https://crates.io/crates/dlhn/0.1.6




## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
