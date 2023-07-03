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

## Last updated: 2023-7-3

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 250.88 µs | <span title="unvalidated">*2.5569 ms\**</span> | 1705800 | 507682 | 403013 |
| [alkahest 0.1.5][alkahest] | 256.03 µs | † | 1045784 | 454748 | 389424 |
| [bincode 1.3.3][bincode] | 488.96 µs | 3.1972 ms | 1045784 | 374305 | 311761 |
| [bitcode 0.4.0][bitcode] | 506.32 µs | 3.4365 ms | 703664 | 320922 | 273622 |
| [borsh 0.10.3][borsh] | 488.28 µs | 3.3446 ms | 885780 | 363280 | 286514 |
| [bson 2.6.0][bson] | 2.7348 ms | 10.694 ms | 1924682 | 537661 | 376270 |
| [capnp 0.16.1][capnp] | 812.20 µs | † | 1443216 | 509618 | 428649 |
| [ciborium 0.2.1][ciborium] | 3.4508 ms | 11.464 ms | 1407835 | 407372 | 324081 |
| [dlhn 0.1.6][dlhn] | 781.39 µs | 3.8482 ms | 724953 | 302512 | 253629 |
| [flatbuffers 23.5.26][flatbuffers] | 1.8836 ms | † | 1276368 | 469962 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.7000 ms | 3.9366 ms | 764996 | 316445 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.3324 ms | 5.6373 ms | 818669 | 334639 | 285514 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 628.22 µs | 3.4906 ms | 765778 | 312771 | 264518 |
| [postcard 1.0.4][postcard] | 420.83 µs | 3.4077 ms | 724953 | 303462 | 253747 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.3245 ms\**</span> <span title="encode">*640.59 µs\**</span> | 4.3698 ms | 764951 | 269811 | 227947 |
| [rkyv 0.7.42][rkyv] | 280.59 µs | <span title="unvalidated">*2.5551 ms\**</span> <span title="validated upfront with error">*3.4649 ms\**</span> | 1011488 | 384478 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.3840 ms | 5.0847 ms | 784997 | 326654 | 278219 |
| [ron 0.8.0][ron] | 22.455 ms | 26.252 ms | 1607459 | 452648 | 349713 |
| [serde_bare 0.5.0][serde_bare] | 849.48 µs | 3.9551 ms | 765778 | 312739 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0275 ms | 8.2373 ms | 1407835 | 407372 | 324081 |
| [serde_json 1.0.99][serde_json] | 4.8723 ms | 11.227 ms | 1827461 | 474358 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.7015 ms | 7.3753 ms | 1827461 | 474358 | 361090 |
| [speedy 0.8.6][speedy] | 333.00 µs | 3.3306 ms | 885780 | 363280 | 286514 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*40.413 µs\**</span> | <span title="unvalidated">*63.788 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.7139 ns\**</span> | <span title="validated on-demand with panic">*56.616 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*106.13 ns\**</span> | <span title="validated on-demand with error">*375.88 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.1978 ns\**</span> <span title="validated upfront with error">*2.2085 ms\**</span> | <span title="unvalidated">*97.161 µs\**</span> <span title="validated upfront with error">*2.2857 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5547 ns\**</span> <span title="validated upfront with error">*1.0705 ms\**</span> | <span title="unvalidated">*16.407 µs\**</span> <span title="validated upfront with error">*981.80 µs\**</span> | 19.551 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*99.93%\**</span> | 41.25% | 53.15% | 56.56% |
| [alkahest 0.1.5][alkahest] | 97.99% | † | 67.29% | 59.33% | 58.53% |
| [bincode 1.3.3][bincode] | 51.31% | 79.92% | 67.29% | 72.08% | 73.12% |
| [bitcode 0.4.0][bitcode] | 49.55% | 74.35% | 100.00% | 84.07% | 83.31% |
| [borsh 0.10.3][borsh] | 51.38% | 76.39% | 79.44% | 74.27% | 79.56% |
| [bson 2.6.0][bson] | 9.17% | 23.89% | 36.56% | 50.18% | 60.58% |
| [capnp 0.16.1][capnp] | 30.89% | † | 48.76% | 52.94% | 53.18% |
| [ciborium 0.2.1][ciborium] | 7.27% | 22.29% | 49.98% | 66.23% | 70.34% |
| [dlhn 0.1.6][dlhn] | 32.11% | 66.40% | 97.06% | 89.19% | 89.87% |
| [flatbuffers 23.5.26][flatbuffers] | 13.32% | † | 55.13% | 57.41% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 14.76% | 64.91% | 91.98% | 85.26% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.42% | 45.32% | 85.95% | 80.63% | 79.84% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 39.94% | 73.20% | 91.89% | 86.26% | 86.17% |
| [postcard 1.0.4][postcard] | 59.62% | 74.98% | 97.06% | 88.91% | 89.83% |
| [prost 0.11.9][prost] | <span title="populate + encode">*7.55%\**</span> <span title="encode">*39.16%\**</span> | 58.47% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 89.41% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.74%\**</span> | 69.57% | 70.18% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 18.13% | 50.25% | 89.64% | 82.60% | 81.93% |
| [ron 0.8.0][ron] | 1.12% | 9.73% | 43.77% | 59.61% | 65.18% |
| [serde_bare 0.5.0][serde_bare] | 29.53% | 64.60% | 91.89% | 86.27% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 12.37% | 31.02% | 49.98% | 66.23% | 70.34% |
| [serde_json 1.0.99][serde_json] | 5.15% | 22.76% | 38.51% | 56.88% | 63.13% |
| [simd-json 0.9.2][simd-json] | 9.29% | 34.64% | 38.51% | 56.88% | 63.13% |
| [speedy 0.8.6][speedy] | 75.34% | 76.72% | 79.44% | 74.27% | 79.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*25.72%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*57.29%\**</span> | <span title="validated on-demand with panic">*28.98%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.46%\**</span> | <span title="validated on-demand with error">*4.36%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*48.62%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.89%\**</span> <span title="validated upfront with error">*0.72%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.67%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 530.01 µs | <span title="unvalidated">*527.31 µs\**</span> | 6000024 | 5380836 | 5345890 |
| [alkahest 0.1.5][alkahest] | 530.72 µs | † | 6000008 | 5380823 | 5345890 |
| [bincode 1.3.3][bincode] | 5.5090 ms | 5.9034 ms | 6000008 | 5380823 | 5345890 |
| [bitcode 0.4.0][bitcode] | 5.9609 ms | 10.132 ms | 4688054 | 4688484 | 4688168 |
| [borsh 0.10.3][borsh] | 6.5617 ms | 4.0778 ms | 6000004 | 5380818 | 5345889 |
| [bson 2.6.0][bson] | 61.508 ms | 124.61 ms | 23013911 | 9211138 | 7497811 |
| [capnp 0.16.1][capnp] | 16.526 ms | † | 14000088 | 6729881 | 6051062 |
| [ciborium 0.2.1][ciborium] | 100.25 ms | 124.68 ms | 13122324 | 7527423 | 6759658 |
| [dlhn 0.1.6][dlhn] | 8.6132 ms | 10.315 ms | 6000003 | 5380817 | 5345900 |
| [flatbuffers 23.5.26][flatbuffers] | 1.4415 ms | † | 6000024 | 5380800 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 22.172 ms | 11.227 ms | 7500005 | 6059282 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 195.03 ms | 43.084 ms | 8125037 | 6495174 | 6386940 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 5.8818 ms | 5.3520 ms | 6000004 | 5380818 | 5345889 |
| [postcard 1.0.4][postcard] | 810.21 µs | 1.6247 ms | 6000003 | 5380817 | 5345900 |
| [prost 0.11.9][prost] | <span title="populate + encode">*14.673 ms\**</span> <span title="encode">*12.183 ms\**</span> | 22.622 ms | 8750000 | 6683814 | 6421871 |
| [rkyv 0.7.42][rkyv] | 577.20 µs | <span title="unvalidated">*529.12 µs\**</span> <span title="validated upfront with error">*527.25 µs\**</span> | 6000008 | 5380822 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 20.529 ms | 26.103 ms | 8125006 | 6496879 | 6391037 |
| [ron 0.8.0][ron] | 263.09 ms | 457.50 ms | 22192885 | 9009575 | 8138755 |
| [serde_bare 0.5.0][serde_bare] | 7.2592 ms | 5.7394 ms | 6000003 | 5380817 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 52.766 ms | 65.011 ms | 13122324 | 7527423 | 6759658 |
| [serde_json 1.0.99][serde_json] | 110.34 ms | 116.45 ms | 26192883 | 9612105 | 8586741 |
| [simd-json 0.9.2][simd-json] | 72.921 ms | 134.29 ms | 26192883 | 9612105 | 8586741 |
| [speedy 0.8.6][speedy] | 523.10 µs | 526.50 µs | 6000004 | 5380818 | 5345889 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.9032 ns\**</span> | <span title="unvalidated">*295.46 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.2248 ns\**</span> | <span title="validated on-demand with panic">*95.186 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*216.40 ns\**</span> | <span title="validated on-demand with error">*6.3874 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7790 ns\**</span> <span title="validated upfront with error">*49.207 ns\**</span> | <span title="unvalidated">*96.003 µs\**</span> <span title="validated upfront with error">*96.598 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7517 ns\**</span> <span title="validated upfront with error">*17.320 ns\**</span> | <span title="unvalidated">*53.776 µs\**</span> <span title="validated upfront with error">*53.952 µs\**</span> | 272.87 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 98.70% | <span title="unvalidated">*99.85%\**</span> | 78.13% | 87.13% | 87.70% |
| [alkahest 0.1.5][alkahest] | 98.56% | † | 78.13% | 87.13% | 87.70% |
| [bincode 1.3.3][bincode] | 9.50% | 8.92% | 78.13% | 87.13% | 87.70% |
| [bitcode 0.4.0][bitcode] | 8.78% | 5.20% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 7.97% | 12.91% | 78.13% | 87.13% | 87.70% |
| [bson 2.6.0][bson] | 0.85% | 0.42% | 20.37% | 50.90% | 62.53% |
| [capnp 0.16.1][capnp] | 3.17% | † | 33.49% | 69.67% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.52% | 0.42% | 35.73% | 62.29% | 69.36% |
| [dlhn 0.1.6][dlhn] | 6.07% | 5.10% | 78.13% | 87.13% | 87.70% |
| [flatbuffers 23.5.26][flatbuffers] | 36.29% | † | 78.13% | 87.13% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 2.36% | 4.69% | 62.51% | 77.38% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.27% | 1.22% | 57.70% | 72.18% | 73.40% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 8.89% | 9.84% | 78.13% | 87.13% | 87.70% |
| [postcard 1.0.4][postcard] | 64.56% | 32.41% | 78.13% | 87.13% | 87.70% |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.57%\**</span> <span title="encode">*4.29%\**</span> | 2.33% | 53.58% | 70.15% | 73.00% |
| [rkyv 0.7.42][rkyv] | 90.63% | <span title="unvalidated">*99.50%\**</span> <span title="validated upfront with error">*99.86%\**</span> | 78.13% | 87.13% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.55% | 2.02% | 57.70% | 72.17% | 73.36% |
| [ron 0.8.0][ron] | 0.20% | 0.12% | 21.12% | 52.04% | 57.60% |
| [serde_bare 0.5.0][serde_bare] | 7.21% | 9.17% | 78.13% | 87.13% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.99% | 0.81% | 35.73% | 62.29% | 69.36% |
| [serde_json 1.0.99][serde_json] | 0.47% | 0.45% | 17.90% | 48.78% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.72% | 0.39% | 17.90% | 48.78% | 54.60% |
| [speedy 0.8.6][speedy] | 100.00% | 100.00% | 78.13% | 87.13% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*60.34%\**</span> | <span title="unvalidated">*18.20%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*54.32%\**</span> | <span title="validated on-demand with panic">*56.50%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.81%\**</span> | <span title="validated on-demand with error">*0.84%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.35%\**</span> <span title="validated upfront with error">*3.56%\**</span> | <span title="unvalidated">*56.01%\**</span> <span title="validated upfront with error">*55.67%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.11%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.67%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 278.38 µs | <span title="unvalidated">*2.3423 ms\**</span> | 1290592 | 395248 | 333366 |
| [alkahest 0.1.5][alkahest] | 345.68 µs | † | 667570 | 325536 | 320452 |
| [bincode 1.3.3][bincode] | 730.37 µs | 2.9685 ms | 569975 | 240897 | 232423 |
| [bitcode 0.4.0][bitcode] | 451.15 µs | 3.0942 ms | 322798 | 215013 | 201247 |
| [borsh 0.10.3][borsh] | 642.41 µs | 3.1343 ms | 446595 | 234395 | 210008 |
| [bson 2.6.0][bson] | 4.1416 ms | 13.435 ms | 1619653 | 506953 | 328399 |
| [capnp 0.16.1][capnp] | 704.70 µs | † | 803896 | 336655 | 280851 |
| [ciborium 0.2.1][ciborium] | 4.1419 ms | 11.769 ms | 1109821 | 347812 | 274526 |
| [dlhn 0.1.6][dlhn] | 995.97 µs | 4.1786 ms | 366496 | 220835 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 3.8375 ms | † | 844168 | 346957 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.5411 ms | 4.6387 ms | 391251 | 237270 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.9422 ms | 5.8956 ms | 449745 | 252743 | 231110 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 748.70 µs | 3.2023 ms | 356311 | 213188 | 198524 |
| [postcard 1.0.4][postcard] | 556.99 µs | 3.2629 ms | 367489 | 222144 | 207344 |
| [prost 0.11.9][prost] | <span title="populate + encode">*4.4362 ms\**</span> <span title="encode">*1.5085 ms\**</span> | 5.4120 ms | 596811 | 306728 | 269310 |
| [rkyv 0.7.42][rkyv] | 472.17 µs | <span title="unvalidated">*2.2061 ms\**</span> <span title="validated upfront with error">*3.0656 ms\**</span> | 596952 | 254139 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.9317 ms | 4.7126 ms | 424533 | 245594 | 226188 |
| [ron 0.8.0][ron] | 12.128 ms | 25.799 ms | 1465223 | 439761 | 343338 |
| [serde_bare 0.5.0][serde_bare] | 994.95 µs | 3.8198 ms | 356311 | 213270 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.3897 ms | 7.6864 ms | 1109821 | 347812 | 274526 |
| [serde_json 1.0.99][serde_json] | 4.9295 ms | 11.905 ms | 1623191 | 472275 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.9223 ms | 6.4725 ms | 1623191 | 472275 | 359623 |
| [speedy 0.8.6][speedy] | 487.37 µs | 2.7556 ms | 449595 | 235136 | 210361 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.807 µs\**</span> | <span title="unvalidated">*69.594 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.1345 ns\**</span> | <span title="validated on-demand with panic">*8.4088 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*129.45 ns\**</span> | <span title="validated on-demand with error">*733.89 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.6792 ns\**</span> <span title="validated upfront with error">*2.6437 ms\**</span> | <span title="unvalidated">*2.9266 µs\**</span> <span title="validated upfront with error">*2.5732 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6889 ns\**</span> <span title="validated upfront with error">*846.46 µs\**</span> | <span title="unvalidated">*217.61 ns\**</span> <span title="validated upfront with error">*851.14 µs\**</span> | 1.8368 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*94.19%\**</span> | 25.01% | 53.94% | 59.54% |
| [alkahest 0.1.5][alkahest] | 80.53% | † | 48.35% | 65.49% | 61.94% |
| [bincode 1.3.3][bincode] | 38.11% | 74.32% | 56.63% | 88.50% | 85.40% |
| [bitcode 0.4.0][bitcode] | 61.70% | 71.30% | 100.00% | 99.15% | 98.63% |
| [borsh 0.10.3][borsh] | 43.33% | 70.39% | 72.28% | 90.95% | 94.51% |
| [bson 2.6.0][bson] | 6.72% | 16.42% | 19.93% | 42.05% | 60.44% |
| [capnp 0.16.1][capnp] | 39.50% | † | 40.15% | 63.33% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.72% | 18.75% | 29.09% | 61.29% | 72.30% |
| [dlhn 0.1.6][dlhn] | 27.95% | 52.80% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 7.25% | † | 38.24% | 61.45% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 18.06% | 47.56% | 82.50% | 89.85% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.51% | 37.42% | 71.77% | 84.35% | 85.88% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 37.18% | 68.89% | 90.59% | 100.00% | 99.98% |
| [postcard 1.0.4][postcard] | 49.98% | 67.61% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="populate + encode">*6.28%\**</span> <span title="encode">*18.45%\**</span> | 40.76% | 54.09% | 69.50% | 73.70% |
| [rkyv 0.7.42][rkyv] | 58.96% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.96%\**</span> | 54.07% | 83.89% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 14.41% | 46.81% | 76.04% | 86.81% | 87.75% |
| [ron 0.8.0][ron] | 2.30% | 8.55% | 22.03% | 48.48% | 57.81% |
| [serde_bare 0.5.0][serde_bare] | 27.98% | 57.75% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.65% | 28.70% | 29.09% | 61.29% | 72.30% |
| [serde_json 1.0.99][serde_json] | 5.65% | 18.53% | 19.89% | 45.14% | 55.19% |
| [simd-json 0.9.2][simd-json] | 9.53% | 34.08% | 19.89% | 45.14% | 55.19% |
| [speedy 0.8.6][speedy] | 57.12% | 80.06% | 71.80% | 90.67% | 94.36% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.31%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*53.88%\**</span> | <span title="validated on-demand with panic">*2.59%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.30%\**</span> | <span title="validated on-demand with error">*29.65%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*45.90%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.44%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 659.44 µs | <span title="unvalidated">*3.8157 ms\**</span> | 2984682 | 1457390 | 1321062 |
| [alkahest 0.1.5][alkahest] | 1.1294 ms | † | 1863391 | 1234238 | 1202345 |
| [bincode 1.3.3][bincode] | 4.0845 ms | 6.9856 ms | 1811011 | 1115517 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.9076 ms | 5.5088 ms | 870693 | 866743 | 870720 |
| [borsh 0.10.3][borsh] | 2.9406 ms | 6.1633 ms | 1486162 | 1083024 | 1013550 |
| [bson 2.6.0][bson] | 35.759 ms | 68.827 ms | 10030880 | 2847180 | 1600859 |
| [capnp 0.16.1][capnp] | 3.5586 ms | † | 2664040 | 1514537 | 1212087 |
| [ciborium 0.2.1][ciborium] | 28.093 ms | 53.631 ms | 5878653 | 1662596 | 1431560 |
| [dlhn 0.1.6][dlhn] | 6.7521 ms | 12.987 ms | 1279599 | 1052966 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 6.3927 ms | † | 2273740 | 1408433 | 1235566 |
| [msgpacker 0.4.3][msgpacker] | 4.6810 ms | 8.6599 ms | 1424043 | 1131284 | 1110156 |
| [nachricht-serde 0.4.0][nachricht-serde] | 46.586 ms | 23.342 ms | 1728519 | 1249189 | 1233323 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 3.5136 ms | 5.2540 ms | 1288257 | 1039829 | 986510 |
| [postcard 1.0.4][postcard] | 2.3978 ms | 6.2183 ms | 1279599 | 1058773 | 1016738 |
| [prost 0.11.9][prost] | <span title="populate + encode">*13.192 ms\**</span> <span title="encode">*6.3968 ms\**</span> | 13.946 ms | 1818378 | 1311199 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.8872 ms | <span title="unvalidated">*3.5551 ms\**</span> <span title="validated upfront with error">*4.6232 ms\**</span> | 2029080 | 1335913 | 1158855 |
| [rmp-serde 1.1.1][rmp-serde] | 12.799 ms | 16.325 ms | 1703813 | 1233850 | 1200208 |
| [ron 0.8.0][ron] | 46.972 ms | 143.48 ms | 8476284 | 2199231 | 1783971 |
| [serde_bare 0.5.0][serde_bare] | 5.6153 ms | 7.8658 ms | 1288257 | 1038144 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 12.295 ms | 30.814 ms | 5878653 | 1662596 | 1431560 |
| [serde_json 1.0.99][serde_json] | 27.951 ms | 52.214 ms | 9175594 | 2352701 | 1800713 |
| [simd-json 0.9.2][simd-json] | 15.546 ms | 44.745 ms | 9175594 | 2352701 | 1800713 |
| [speedy 0.8.6][speedy] | 1.5316 ms | 4.1083 ms | 1546963 | 1093931 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*138.78 µs\**</span> | <span title="unvalidated">*139.93 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.1436 ns\**</span> | <span title="validated on-demand with panic">*898.11 ns\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*125.43 ns\**</span> | <span title="validated on-demand with error">*2.1104 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.7154 ns\**</span> <span title="validated upfront with error">*5.4944 ms\**</span> | <span title="unvalidated">*5.9373 µs\**</span> <span title="validated upfront with error">*5.6025 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7403 ns\**</span> <span title="validated upfront with error">*1.0358 ms\**</span> | <span title="unvalidated">*444.02 ns\**</span> <span title="validated upfront with error">*1.0453 ms\**</span> | 871.17 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.17%\**</span> | 29.17% | 59.47% | 65.91% |
| [alkahest 0.1.5][alkahest] | 58.39% | † | 46.73% | 70.22% | 72.42% |
| [bincode 1.3.3][bincode] | 16.14% | 50.89% | 48.08% | 77.70% | 84.90% |
| [bitcode 0.4.0][bitcode] | 34.57% | 64.53% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 22.43% | 57.68% | 58.59% | 80.03% | 85.91% |
| [bson 2.6.0][bson] | 1.84% | 5.17% | 8.68% | 30.44% | 54.39% |
| [capnp 0.16.1][capnp] | 18.53% | † | 32.68% | 57.23% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.35% | 6.63% | 14.81% | 52.13% | 60.82% |
| [dlhn 0.1.6][dlhn] | 9.77% | 27.37% | 68.04% | 82.31% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 10.32% | † | 38.29% | 61.54% | 70.47% |
| [msgpacker 0.4.3][msgpacker] | 14.09% | 41.05% | 61.14% | 76.62% | 78.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.42% | 15.23% | 50.37% | 69.38% | 70.60% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 18.77% | 67.66% | 67.59% | 83.35% | 88.26% |
| [postcard 1.0.4][postcard] | 27.50% | 57.17% | 68.04% | 81.86% | 85.64% |
| [prost 0.11.9][prost] | <span title="populate + encode">*5.00%\**</span> <span title="encode">*10.31%\**</span> | 25.49% | 47.88% | 66.10% | 68.76% |
| [rkyv 0.7.42][rkyv] | 34.94% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*76.90%\**</span> | 42.91% | 64.88% | 75.14% |
| [rmp-serde 1.1.1][rmp-serde] | 5.15% | 21.78% | 51.10% | 70.25% | 72.55% |
| [ron 0.8.0][ron] | 1.40% | 2.48% | 10.27% | 39.41% | 48.81% |
| [serde_bare 0.5.0][serde_bare] | 11.74% | 45.20% | 67.59% | 83.49% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.36% | 11.54% | 14.81% | 52.13% | 60.82% |
| [serde_json 1.0.99][serde_json] | 2.36% | 6.81% | 9.49% | 36.84% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.24% | 7.95% | 9.49% | 36.84% | 48.35% |
| [speedy 0.8.6][speedy] | 43.06% | 86.53% | 56.28% | 79.23% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.32%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.36%\**</span> | <span title="validated on-demand with panic">*49.44%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.39%\**</span> | <span title="validated on-demand with error">*21.04%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*46.84%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.48%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
[serde_json]: https://crates.io/crates/serde_json/1.0.99
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6




## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
