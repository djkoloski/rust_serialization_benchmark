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

## Last updated: 2023-7-7 2:13:59

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 254.95 µs | <span title="unvalidated">*2.4158 ms\**</span> | 1705800 | 499267 | 405816 |
| [alkahest 0.1.5][alkahest] | 253.23 µs | † | 1045784 | 454748 | 389424 |
| [bincode 1.3.3][bincode] | 493.88 µs | 3.1070 ms | 1045784 | 374305 | 311761 |
| [bitcode 0.4.0][bitcode] | 529.16 µs | 3.4604 ms | 703664 | 320922 | 273622 |
| [borsh 0.10.3][borsh] | 482.82 µs | 3.4651 ms | 885780 | 363280 | 286514 |
| [bson 2.6.0][bson] | 2.5878 ms | 10.481 ms | 1924682 | 537661 | 376270 |
| [capnp 0.16.1][capnp] | 837.49 µs | † | 1443216 | 509618 | 428649 |
| [ciborium 0.2.1][ciborium] | 4.1334 ms | 11.471 ms | 1407835 | 407372 | 324081 |
| [dlhn 0.1.6][dlhn] | 787.80 µs | 3.9315 ms | 724953 | 302512 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.8116 ms | † | 1276368 | 469962 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.6382 ms | 3.9592 ms | 764996 | 316445 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.2218 ms | 5.4555 ms | 818669 | 334639 | 285514 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 625.38 µs | 3.6001 ms | 765778 | 312771 | 264518 |
| [postcard 1.0.4][postcard] | 391.51 µs | 3.4913 ms | 724953 | 303462 | 253747 |
| [prost 0.11.9][prost] | <span title="encode">*582.11 µs\**</span> <span title="populate + encode">*3.2624 ms\**</span> | 4.1187 ms | 764951 | 269811 | 227947 |
| [rkyv 0.7.42][rkyv] | 309.57 µs | <span title="unvalidated">*2.4850 ms\**</span> <span title="validated upfront with error">*3.4170 ms\**</span> | 1011488 | 384478 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.5572 ms | 4.5826 ms | 784997 | 326654 | 278219 |
| [ron 0.8.0][ron] | 20.613 ms | 22.074 ms | 1607459 | 452648 | 349713 |
| [serde_bare 0.5.0][serde_bare] | 803.89 µs | 3.4909 ms | 765778 | 312739 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0540 ms | 6.8305 ms | 1407835 | 407372 | 324081 |
| [serde_json 1.0.100][serde_json] | 4.2821 ms | 9.1314 ms | 1827461 | 474358 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.1718 ms | 5.7885 ms | 1827461 | 474358 | 361090 |
| [speedy 0.8.6][speedy] | 289.40 µs | 2.8415 ms | 885780 | 363280 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*36.484 µs\**</span> | <span title="unvalidated">*56.457 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7674 ns\**</span> | <span title="validated on-demand with panic">*36.918 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*109.82 ns\**</span> | <span title="validated on-demand with error">*380.46 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2692 ns\**</span> <span title="validated upfront with error">*2.2384 ms\**</span> | <span title="unvalidated">*93.685 µs\**</span> <span title="validated upfront with error">*2.3372 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5217 ns\**</span> <span title="validated upfront with error">*923.55 µs\**</span> | <span title="unvalidated">*16.334 µs\**</span> <span title="validated upfront with error">*940.31 µs\**</span> | 23.430 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 99.33% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 54.04% | 56.17% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.33% | 58.53% |
| [bincode 1.3.3][bincode] | 51.27% | 77.75% | 67.29% | 72.08% | 73.12% |
| [bitcode 0.4.0][bitcode] | 47.86% | 69.81% | 100.00% | 84.07% | 83.31% |
| [borsh 0.10.3][borsh] | 52.45% | 69.72% | 79.44% | 74.27% | 79.56% |
| [bson 2.6.0][bson] | 9.79% | 23.05% | 36.56% | 50.18% | 60.58% |
| [capnp 0.16.1][capnp] | 30.24% | † | 48.76% | 52.94% | 53.18% |
| [ciborium 0.2.1][ciborium] | 6.13% | 21.06% | 49.98% | 66.23% | 70.34% |
| [dlhn 0.1.6][dlhn] | 32.14% | 61.45% | 97.06% | 89.19% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 13.98% | † | 55.13% | 57.41% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 15.46% | 61.02% | 91.98% | 85.26% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.08% | 44.28% | 85.95% | 80.63% | 79.84% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 40.49% | 67.10% | 91.89% | 86.26% | 86.17% |
| [postcard 1.0.4][postcard] | 64.68% | 69.19% | 97.06% | 88.91% | 89.83% |
| [prost 0.11.9][prost] | <span title="encode">*43.50%\**</span> <span title="populate + encode">*7.76%\**</span> | 58.65% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 81.80% | <span title="unvalidated">*97.22%\**</span> <span title="validated upfront with error">*70.70%\**</span> | 69.57% | 70.18% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 16.26% | 52.72% | 89.64% | 82.60% | 81.93% |
| [ron 0.8.0][ron] | 1.23% | 10.94% | 43.77% | 59.61% | 65.18% |
| [serde_bare 0.5.0][serde_bare] | 31.50% | 69.20% | 91.89% | 86.27% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 12.33% | 35.37% | 49.98% | 66.23% | 70.34% |
| [serde_json 1.0.100][serde_json] | 5.91% | 26.46% | 38.51% | 56.88% | 63.13% |
| [simd-json 0.9.2][simd-json] | 11.66% | 41.73% | 38.51% | 56.88% | 63.13% |
| [speedy 0.8.6][speedy] | 87.50% | 85.02% | 79.44% | 74.27% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*28.93%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*54.99%\**</span> | <span title="validated on-demand with panic">*44.24%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.39%\**</span> | <span title="validated on-demand with error">*4.29%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.55%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.44%\**</span> <span title="validated upfront with error">*0.70%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.74%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 470.18 µs | <span title="unvalidated">*470.69 µs\**</span> | 6000024 | 5380836 | 5345890 |
| [alkahest 0.1.5][alkahest] | 472.40 µs | † | 6000008 | 5380823 | 5345890 |
| [bincode 1.3.3][bincode] | 4.6226 ms | 5.3245 ms | 6000008 | 5380823 | 5345890 |
| [bitcode 0.4.0][bitcode] | 4.8811 ms | 8.4272 ms | 4688054 | 4688484 | 4688168 |
| [borsh 0.10.3][borsh] | 5.5259 ms | 3.6723 ms | 6000004 | 5380818 | 5345889 |
| [bson 2.6.0][bson] | 45.460 ms | 111.28 ms | 23013911 | 9211138 | 7497811 |
| [capnp 0.16.1][capnp] | 10.619 ms | † | 14000088 | 6729881 | 6051062 |
| [ciborium 0.2.1][ciborium] | 89.971 ms | 110.31 ms | 13122324 | 7527423 | 6759658 |
| [dlhn 0.1.6][dlhn] | 7.3196 ms | 9.5176 ms | 6000003 | 5380817 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.2615 ms | † | 6000024 | 5380800 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 19.972 ms | 10.200 ms | 7500005 | 6059282 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 200.08 ms | 36.730 ms | 8125037 | 6495174 | 6386940 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 3.6874 ms | 4.7670 ms | 6000004 | 5380818 | 5345889 |
| [postcard 1.0.4][postcard] | 1.0472 ms | 1.7292 ms | 6000003 | 5380817 | 5345900 |
| [prost 0.11.9][prost] | <span title="encode">*10.640 ms\**</span> <span title="populate + encode">*13.165 ms\**</span> | 19.906 ms | 8750000 | 6683814 | 6421871 |
| [rkyv 0.7.42][rkyv] | 518.57 µs | <span title="unvalidated">*471.90 µs\**</span> <span title="validated upfront with error">*471.58 µs\**</span> | 6000008 | 5380822 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 17.545 ms | 23.329 ms | 8125006 | 6496879 | 6391037 |
| [ron 0.8.0][ron] | 225.70 ms | 384.37 ms | 22192885 | 9009575 | 8138755 |
| [serde_bare 0.5.0][serde_bare] | 6.5218 ms | 5.4199 ms | 6000003 | 5380817 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 45.046 ms | 60.382 ms | 13122324 | 7527423 | 6759658 |
| [serde_json 1.0.100][serde_json] | 102.64 ms | 102.74 ms | 26192883 | 9612105 | 8586741 |
| [simd-json 0.9.2][simd-json] | 66.054 ms | 126.12 ms | 26192883 | 9612105 | 8586741 |
| [speedy 0.8.6][speedy] | 471.24 µs | 470.90 µs | 6000004 | 5380818 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.4760 ns\**</span> | <span title="unvalidated">*250.78 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7673 ns\**</span> | <span title="validated on-demand with panic">*167.31 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*180.02 ns\**</span> | <span title="validated on-demand with error">*5.4433 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2694 ns\**</span> <span title="validated upfront with error">*44.083 ns\**</span> | <span title="unvalidated">*83.721 µs\**</span> <span title="validated upfront with error">*83.774 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5216 ns\**</span> <span title="validated upfront with error">*15.842 ns\**</span> | <span title="unvalidated">*47.030 µs\**</span> <span title="validated upfront with error">*47.106 µs\**</span> | 238.29 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*100.00%\**</span> | 78.13% | 87.13% | 87.70% |
| [alkahest 0.1.5][alkahest] | 99.53% | † | 78.13% | 87.13% | 87.70% |
| [bincode 1.3.3][bincode] | 10.17% | 8.84% | 78.13% | 87.13% | 87.70% |
| [bitcode 0.4.0][bitcode] | 9.63% | 5.59% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 8.51% | 12.82% | 78.13% | 87.13% | 87.70% |
| [bson 2.6.0][bson] | 1.03% | 0.42% | 20.37% | 50.90% | 62.53% |
| [capnp 0.16.1][capnp] | 4.43% | † | 33.49% | 69.67% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.52% | 0.43% | 35.73% | 62.29% | 69.36% |
| [dlhn 0.1.6][dlhn] | 6.42% | 4.95% | 78.13% | 87.13% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 37.27% | † | 78.13% | 87.13% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 2.35% | 4.61% | 62.51% | 77.38% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.23% | 1.28% | 57.70% | 72.18% | 73.40% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 12.75% | 9.87% | 78.13% | 87.13% | 87.70% |
| [postcard 1.0.4][postcard] | 44.90% | 27.22% | 78.13% | 87.13% | 87.70% |
| [prost 0.11.9][prost] | <span title="encode">*4.42%\**</span> <span title="populate + encode">*3.57%\**</span> | 2.36% | 53.58% | 70.15% | 73.00% |
| [rkyv 0.7.42][rkyv] | 90.67% | <span title="unvalidated">*99.74%\**</span> <span title="validated upfront with error">*99.81%\**</span> | 78.13% | 87.13% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.68% | 2.02% | 57.70% | 72.17% | 73.36% |
| [ron 0.8.0][ron] | 0.21% | 0.12% | 21.12% | 52.04% | 57.60% |
| [serde_bare 0.5.0][serde_bare] | 7.21% | 8.68% | 78.13% | 87.13% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 1.04% | 0.78% | 35.73% | 62.29% | 69.36% |
| [serde_json 1.0.100][serde_json] | 0.46% | 0.46% | 17.90% | 48.78% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.71% | 0.37% | 17.90% | 48.78% | 54.60% |
| [speedy 0.8.6][speedy] | 99.78% | 99.96% | 78.13% | 87.13% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*61.45%\**</span> | <span title="unvalidated">*18.75%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*54.99%\**</span> | <span title="validated on-demand with panic">*28.11%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.85%\**</span> | <span title="validated on-demand with error">*0.86%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.54%\**</span> <span title="validated upfront with error">*3.45%\**</span> | <span title="unvalidated">*56.17%\**</span> <span title="validated upfront with error">*56.14%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.60%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.84%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 253.48 µs | <span title="unvalidated">*1.9749 ms\**</span> | 1290592 | 395532 | 333340 |
| [alkahest 0.1.5][alkahest] | 305.06 µs | † | 667570 | 325536 | 320452 |
| [bincode 1.3.3][bincode] | 604.39 µs | 2.5740 ms | 569975 | 240897 | 232423 |
| [bitcode 0.4.0][bitcode] | 399.33 µs | 2.7155 ms | 322798 | 215013 | 201247 |
| [borsh 0.10.3][borsh] | 575.18 µs | 2.7242 ms | 446595 | 234395 | 210008 |
| [bson 2.6.0][bson] | 3.6335 ms | 11.816 ms | 1619653 | 506953 | 328399 |
| [capnp 0.16.1][capnp] | 647.67 µs | † | 803896 | 336655 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.6964 ms | 10.263 ms | 1109821 | 347812 | 274526 |
| [dlhn 0.1.6][dlhn] | 869.59 µs | 3.6886 ms | 366496 | 220835 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.5277 ms | † | 844168 | 346957 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.2929 ms | 3.9675 ms | 391251 | 237270 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.6496 ms | 5.0730 ms | 449745 | 252743 | 231110 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 652.11 µs | 2.8033 ms | 356311 | 213188 | 198524 |
| [postcard 1.0.4][postcard] | 500.13 µs | 2.8437 ms | 367489 | 222144 | 207344 |
| [prost 0.11.9][prost] | <span title="encode">*1.4111 ms\**</span> <span title="populate + encode">*3.9334 ms\**</span> | 4.6596 ms | 596811 | 306728 | 269310 |
| [rkyv 0.7.42][rkyv] | 415.87 µs | <span title="unvalidated">*1.8976 ms\**</span> <span title="validated upfront with error">*2.6212 ms\**</span> | 596952 | 254139 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.6658 ms | 3.9819 ms | 424533 | 245594 | 226188 |
| [ron 0.8.0][ron] | 10.606 ms | 22.498 ms | 1465223 | 439761 | 343338 |
| [serde_bare 0.5.0][serde_bare] | 926.09 µs | 3.3334 ms | 356311 | 213270 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0844 ms | 6.7352 ms | 1109821 | 347812 | 274526 |
| [serde_json 1.0.100][serde_json] | 4.4043 ms | 10.216 ms | 1623191 | 472275 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.5344 ms | 5.7436 ms | 1623191 | 472275 | 359623 |
| [speedy 0.8.6][speedy] | 413.85 µs | 2.3933 ms | 449595 | 235136 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*62.720 µs\**</span> | <span title="unvalidated">*64.182 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7671 ns\**</span> | <span title="validated on-demand with panic">*7.0677 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*119.01 ns\**</span> | <span title="validated on-demand with error">*649.40 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2693 ns\**</span> <span title="validated upfront with error">*2.2935 ms\**</span> | <span title="unvalidated">*2.9710 µs\**</span> <span title="validated upfront with error">*2.2980 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5229 ns\**</span> <span title="validated upfront with error">*712.60 µs\**</span> | <span title="unvalidated">*189.54 ns\**</span> <span title="validated upfront with error">*713.09 µs\**</span> | 1.5725 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*96.09%\**</span> | 25.01% | 53.90% | 59.55% |
| [alkahest 0.1.5][alkahest] | 83.09% | † | 48.35% | 65.49% | 61.94% |
| [bincode 1.3.3][bincode] | 41.94% | 73.72% | 56.63% | 88.50% | 85.40% |
| [bitcode 0.4.0][bitcode] | 63.48% | 69.88% | 100.00% | 99.15% | 98.63% |
| [borsh 0.10.3][borsh] | 44.07% | 69.66% | 72.28% | 90.95% | 94.51% |
| [bson 2.6.0][bson] | 6.98% | 16.06% | 19.93% | 42.05% | 60.44% |
| [capnp 0.16.1][capnp] | 39.14% | † | 40.15% | 63.33% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.86% | 18.49% | 29.09% | 61.29% | 72.30% |
| [dlhn 0.1.6][dlhn] | 29.15% | 51.44% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 7.19% | † | 38.24% | 61.45% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 19.61% | 47.83% | 82.50% | 89.85% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.31% | 37.41% | 71.77% | 84.35% | 85.88% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 38.87% | 67.69% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.4][postcard] | 50.68% | 66.73% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="encode">*17.96%\**</span> <span title="populate + encode">*6.44%\**</span> | 40.72% | 54.09% | 69.50% | 73.70% |
| [rkyv 0.7.42][rkyv] | 60.95% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.39%\**</span> | 54.07% | 83.89% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 15.22% | 47.66% | 76.04% | 86.81% | 87.75% |
| [ron 0.8.0][ron] | 2.39% | 8.43% | 22.03% | 48.48% | 57.81% |
| [serde_bare 0.5.0][serde_bare] | 27.37% | 56.93% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 12.16% | 28.17% | 29.09% | 61.29% | 72.30% |
| [serde_json 1.0.100][serde_json] | 5.76% | 18.57% | 19.89% | 45.14% | 55.19% |
| [simd-json 0.9.2][simd-json] | 10.00% | 33.04% | 19.89% | 45.14% | 55.19% |
| [speedy 0.8.6][speedy] | 61.25% | 79.29% | 71.80% | 90.67% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.30%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.04%\**</span> | <span title="validated on-demand with panic">*2.68%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.28%\**</span> | <span title="validated on-demand with error">*29.19%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.58%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.38%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 621.31 µs | <span title="unvalidated">*3.2968 ms\**</span> | 2984682 | 1457694 | 1321864 |
| [alkahest 0.1.5][alkahest] | 957.98 µs | † | 1863391 | 1234238 | 1202345 |
| [bincode 1.3.3][bincode] | 3.7410 ms | 5.8111 ms | 1811011 | 1115517 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.5738 ms | 4.7829 ms | 870693 | 866743 | 870720 |
| [borsh 0.10.3][borsh] | 2.6937 ms | 5.4294 ms | 1486162 | 1083024 | 1013550 |
| [bson 2.6.0][bson] | 32.268 ms | 62.659 ms | 10030880 | 2847180 | 1600859 |
| [capnp 0.16.1][capnp] | 2.6421 ms | † | 2664040 | 1514537 | 1212087 |
| [ciborium 0.2.1][ciborium] | 24.719 ms | 46.911 ms | 5878653 | 1662596 | 1431560 |
| [dlhn 0.1.6][dlhn] | 5.9411 ms | 11.322 ms | 1279599 | 1052966 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 5.5491 ms | † | 2273740 | 1408433 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.0046 ms | 7.5894 ms | 1424043 | 1131284 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 44.053 ms | 20.248 ms | 1728519 | 1249189 | 1233323 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 2.7722 ms | 4.5713 ms | 1288257 | 1039829 | 986510 |
| [postcard 1.0.4][postcard] | 2.0342 ms | 5.3612 ms | 1279599 | 1058773 | 1016738 |
| [prost 0.11.9][prost] | <span title="encode">*5.5793 ms\**</span> <span title="populate + encode">*11.528 ms\**</span> | 11.331 ms | 1818378 | 1311199 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.7402 ms | <span title="unvalidated">*3.0742 ms\**</span> <span title="validated upfront with error">*3.9978 ms\**</span> | 2029080 | 1335913 | 1158855 |
| [rmp-serde 1.1.1][rmp-serde] | 10.443 ms | 14.218 ms | 1703813 | 1233850 | 1200208 |
| [ron 0.8.0][ron] | 42.438 ms | 126.03 ms | 8476284 | 2199231 | 1783971 |
| [serde_bare 0.5.0][serde_bare] | 5.0361 ms | 6.9688 ms | 1288257 | 1038144 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 11.071 ms | 27.526 ms | 5878653 | 1662596 | 1431560 |
| [serde_json 1.0.100][serde_json] | 24.585 ms | 44.319 ms | 9175594 | 2352701 | 1800713 |
| [simd-json 0.9.2][simd-json] | 13.251 ms | 40.057 ms | 9175594 | 2352701 | 1800713 |
| [speedy 0.8.6][speedy] | 1.2209 ms | 3.7004 ms | 1546963 | 1093931 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*120.02 µs\**</span> | <span title="unvalidated">*120.40 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7671 ns\**</span> | <span title="validated on-demand with panic">*791.69 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*107.66 ns\**</span> | <span title="validated on-demand with error">*1.8205 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.2698 ns\**</span> <span title="validated upfront with error">*4.8534 ms\**</span> | <span title="unvalidated">*4.8186 µs\**</span> <span title="validated upfront with error">*4.8415 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5232 ns\**</span> <span title="validated upfront with error">*905.78 µs\**</span> | <span title="unvalidated">*403.51 ns\**</span> <span title="validated upfront with error">*906.42 µs\**</span> | 766.33 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.25%\**</span> | 29.17% | 59.46% | 65.87% |
| [alkahest 0.1.5][alkahest] | 64.86% | † | 46.73% | 70.22% | 72.42% |
| [bincode 1.3.3][bincode] | 16.61% | 52.90% | 48.08% | 77.70% | 84.90% |
| [bitcode 0.4.0][bitcode] | 39.48% | 64.27% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 23.07% | 56.62% | 58.59% | 80.03% | 85.91% |
| [bson 2.6.0][bson] | 1.93% | 4.91% | 8.68% | 30.44% | 54.39% |
| [capnp 0.16.1][capnp] | 23.52% | † | 32.68% | 57.23% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.51% | 6.55% | 14.81% | 52.13% | 60.82% |
| [dlhn 0.1.6][dlhn] | 10.46% | 27.15% | 68.04% | 82.31% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 11.20% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 15.51% | 40.51% | 61.14% | 76.62% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.41% | 15.18% | 50.37% | 69.38% | 70.60% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 22.41% | 67.25% | 67.59% | 83.35% | 88.26% |
| [postcard 1.0.4][postcard] | 30.54% | 57.34% | 68.04% | 81.86% | 85.64% |
| [prost 0.11.9][prost] | <span title="encode">*11.14%\**</span> <span title="populate + encode">*5.39%\**</span> | 27.13% | 47.88% | 66.10% | 68.76% |
| [rkyv 0.7.42][rkyv] | 35.70% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*76.90%\**</span> | 42.91% | 64.88% | 75.14% |
| [rmp-serde 1.1.1][rmp-serde] | 5.95% | 21.62% | 51.10% | 70.25% | 72.55% |
| [ron 0.8.0][ron] | 1.46% | 2.44% | 10.27% | 39.41% | 48.81% |
| [serde_bare 0.5.0][serde_bare] | 12.34% | 44.11% | 67.59% | 83.49% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.61% | 11.17% | 14.81% | 52.13% | 60.82% |
| [serde_json 1.0.100][serde_json] | 2.53% | 6.94% | 9.49% | 36.84% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.69% | 7.67% | 9.49% | 36.84% | 48.35% |
| [speedy 0.8.6][speedy] | 50.89% | 83.08% | 56.28% | 79.23% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.34%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.05%\**</span> | <span title="validated on-demand with panic">*50.97%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*22.16%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.58%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*8.37%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.04%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.4.0
[borsh]: https://crates.io/crates/borsh/0.10.3
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.16.1
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.3
[postcard]: https://crates.io/crates/postcard/1.0.4
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.1
[ron]: https://crates.io/crates/ron/0.8.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.100
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
