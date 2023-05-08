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

## Last updated: 2023-5-8

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 259.37 µs | <span title="unvalidated">*2.4011 ms\**</span> | 1705800 | 507668 | 403429 |
| [bincode 1.3.3][bincode] | 466.12 µs | 3.3135 ms | 1045784 | 374305 | 311761 |
| [bitcode 0.3.7][bitcode] | 794.01 µs | 3.5907 ms | 703664 | 320922 | 273622 |
| [borsh 0.10.3][borsh] | 490.92 µs | 3.6749 ms | 885780 | 363280 | 286514 |
| [bson 2.6.0][bson] | 2.4891 ms | 9.8576 ms | 1924682 | 537661 | 376270 |
| [capnp 0.16.1][capnp] | 775.22 µs | † | 1443216 | 509618 | 428649 |
| [ciborium 0.2.0][ciborium] | 3.9859 ms | 12.458 ms | 1407835 | 407372 | 324081 |
| [flatbuffers 23.1.21][flatbuffers] | 1.8869 ms | † | 1276368 | 469962 | 388832 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.3969 ms | 5.5645 ms | 818669 | 334639 | 285514 |
| [postcard 1.0.4][postcard] | 397.50 µs | 3.3510 ms | 724953 | 303462 | 253747 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.1761 ms\**</span> <span title="encode">*524.77 µs\**</span> | 3.8467 ms | 764951 | 269811 | 227947 |
| [rkyv 0.7.41][rkyv] | 304.43 µs | <span title="unvalidated">*2.5148 ms\**</span> <span title="validated upfront with error">*3.5159 ms\**</span> | 1011488 | 384478 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.7111 ms | 4.5539 ms | 784997 | 326654 | 278219 |
| [ron 0.8.0][ron] | 20.826 ms | 20.940 ms | 1607459 | 452648 | 349713 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 642.39 µs | 3.3609 ms | 765778 | 312771 | 264518 |
| [serde_bare 0.5.0][serde_bare] | 739.95 µs | 3.4448 ms | 765778 | 312739 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.1402 ms | 6.4169 ms | 1407835 | 407372 | 324081 |
| [serde_json 1.0.96][serde_json] | 4.1757 ms | 7.8545 ms | 1827461 | 474358 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.3134 ms | 5.5640 ms | 1827461 | 474358 | 361090 |
| [speedy 0.8.6][speedy] | 241.66 µs | 2.7127 ms | 885780 | 363280 | 286514 |
| [alkahest 0.1.5][alkahest] | 247.30 µs | † | 1045784 | 454748 | 389424 |
| [dlhn 0.1.4][dlhn] | 783.25 µs | 3.7835 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*28.490 µs\**</span> | <span title="unvalidated">*50.671 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*118.33 ns\**</span> | <span title="validated on-demand with error">*389.82 µs\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6876 ns\**</span> <span title="validated upfront with error">*2.2271 ms\**</span> | <span title="unvalidated">*80.381 µs\**</span> <span title="validated upfront with error">*2.9521 ms\**</span> | ‡ |
| [rkyv 0.7.41][rkyv] | <span title="unvalidated">*1.2081 ns\**</span> <span title="validated upfront with error">*899.79 µs\**</span> | <span title="unvalidated">*17.651 µs\**</span> <span title="validated upfront with error">*912.96 µs\**</span> | 19.724 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.1217 ns\**</span> | <span title="validated on-demand with panic">*34.170 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 93.17% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 53.15% | 56.50% |
| [bincode 1.3.3][bincode] | 51.85% | 72.46% | 67.29% | 72.08% | 73.12% |
| [bitcode 0.3.7][bitcode] | 30.44% | 66.87% | 100.00% | 84.07% | 83.31% |
| [borsh 0.10.3][borsh] | 49.23% | 65.34% | 79.44% | 74.27% | 79.56% |
| [bson 2.6.0][bson] | 9.71% | 24.36% | 36.56% | 50.18% | 60.58% |
| [capnp 0.16.1][capnp] | 31.17% | † | 48.76% | 52.94% | 53.18% |
| [ciborium 0.2.0][ciborium] | 6.06% | 19.27% | 49.98% | 66.23% | 70.34% |
| [flatbuffers 23.1.21][flatbuffers] | 12.81% | † | 55.13% | 57.41% | 58.62% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.27% | 43.15% | 85.95% | 80.63% | 79.84% |
| [postcard 1.0.4][postcard] | 60.79% | 71.65% | 97.06% | 88.91% | 89.83% |
| [prost 0.11.9][prost] | <span title="populate + encode">*7.61%\**</span> <span title="encode">*46.05%\**</span> | 62.42% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.41][rkyv] | 79.38% | <span title="unvalidated">*95.48%\**</span> <span title="validated upfront with error">*68.29%\**</span> | 69.57% | 70.18% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 14.12% | 52.73% | 89.64% | 82.60% | 81.93% |
| [ron 0.8.0][ron] | 1.16% | 11.47% | 43.77% | 59.61% | 65.18% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 37.62% | 71.44% | 91.89% | 86.26% | 86.17% |
| [serde_bare 0.5.0][serde_bare] | 32.66% | 69.70% | 91.89% | 86.27% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.29% | 37.42% | 49.98% | 66.23% | 70.34% |
| [serde_json 1.0.96][serde_json] | 5.79% | 30.57% | 38.51% | 56.88% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.45% | 43.15% | 38.51% | 56.88% | 63.13% |
| [speedy 0.8.6][speedy] | 100.00% | 88.51% | 79.44% | 74.27% | 79.56% |
| [alkahest 0.1.5][alkahest] | 97.72% | † | 67.29% | 59.33% | 58.53% |
| [dlhn 0.1.4][dlhn] | 30.85% | 63.46% | 97.06% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*34.83%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.02%\**</span> | <span title="validated on-demand with error">*4.53%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*44.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.96%\**</span> <span title="validated upfront with error">*0.60%\**</span> | ‡ |
| [rkyv 0.7.41][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.93%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*56.94%\**</span> | <span title="validated on-demand with panic">*51.66%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 415.65 µs | <span title="unvalidated">*416.52 µs\**</span> | 6000024 | 5380837 | 5345891 |
| [bincode 1.3.3][bincode] | 3.7583 ms | 11.009 ms | 6000008 | 5380823 | 5345890 |
| [bitcode 0.3.7][bitcode] | 7.8129 ms | 14.519 ms | 4737624 | 4740613 | 4737741 |
| [borsh 0.10.3][borsh] | 5.2144 ms | 6.4628 ms | 6000004 | 5380818 | 5345889 |
| [bson 2.6.0][bson] | 47.613 ms | 108.87 ms | 23013911 | 9211138 | 7497811 |
| [capnp 0.16.1][capnp] | 13.656 ms | † | 14000088 | 6729881 | 6051062 |
| [ciborium 0.2.0][ciborium] | 89.369 ms | 112.78 ms | 13122324 | 7527423 | 6759658 |
| [flatbuffers 23.1.21][flatbuffers] | 911.40 µs | † | 6000024 | 5380800 | 5345910 |
| [nachricht-serde 0.4.0][nachricht-serde] | 179.04 ms | 33.401 ms | 8125037 | 6495174 | 6386940 |
| [postcard 1.0.4][postcard] | 579.97 µs | 1.2656 ms | 6000003 | 5380817 | 5345900 |
| [prost 0.11.9][prost] | <span title="populate + encode">*7.6767 ms\**</span> <span title="encode">*5.6231 ms\**</span> | 15.903 ms | 8750000 | 6683814 | 6421871 |
| [rkyv 0.7.41][rkyv] | 508.51 µs | <span title="unvalidated">*452.50 µs\**</span> <span title="validated upfront with error">*453.34 µs\**</span> | 6000008 | 5380822 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 20.165 ms | 24.473 ms | 8125006 | 6496879 | 6391037 |
| [ron 0.8.0][ron] | 228.77 ms | 355.13 ms | 22192885 | 9009575 | 8138755 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 3.8389 ms | 6.3212 ms | 6000004 | 5380818 | 5345889 |
| [serde_bare 0.5.0][serde_bare] | 7.0120 ms | 6.5051 ms | 6000003 | 5380817 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 50.592 ms | 52.575 ms | 13122324 | 7527423 | 6759658 |
| [serde_json 1.0.96][serde_json] | 108.66 ms | 91.780 ms | 26192883 | 9612105 | 8586741 |
| [simd-json 0.9.2][simd-json] | 69.927 ms | 108.38 ms | 26192883 | 9612105 | 8586741 |
| [speedy 0.8.6][speedy] | 415.73 µs | 416.82 µs | 6000004 | 5380818 | 5345889 |
| [alkahest 0.1.5][alkahest] | 416.38 µs | † | 6000008 | 5380823 | 5345890 |
| [dlhn 0.1.4][dlhn] | 5.5287 ms | 9.4663 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*1.9469 ns\**</span> | <span title="unvalidated">*247.49 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*188.07 ns\**</span> | <span title="validated on-demand with error">*5.1428 ms\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6930 ns\**</span> <span title="validated upfront with error">*48.443 ns\**</span> | <span title="unvalidated">*100.42 µs\**</span> <span title="validated upfront with error">*100.46 µs\**</span> | ‡ |
| [rkyv 0.7.41][rkyv] | <span title="unvalidated">*1.2079 ns\**</span> <span title="validated upfront with error">*7.5301 ns\**</span> | <span title="unvalidated">*35.152 µs\**</span> <span title="validated upfront with error">*35.732 µs\**</span> | 229.45 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.1256 ns\**</span> | <span title="validated on-demand with panic">*100.51 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*100.00%\**</span> | 78.96% | 88.10% | 88.62% |
| [bincode 1.3.3][bincode] | 11.06% | 3.78% | 78.96% | 88.10% | 88.62% |
| [bitcode 0.3.7][bitcode] | 5.32% | 2.87% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 7.97% | 6.44% | 78.96% | 88.10% | 88.62% |
| [bson 2.6.0][bson] | 0.87% | 0.38% | 20.59% | 51.47% | 63.19% |
| [capnp 0.16.1][capnp] | 3.04% | † | 33.84% | 70.44% | 78.30% |
| [ciborium 0.2.0][ciborium] | 0.47% | 0.37% | 36.10% | 62.98% | 70.09% |
| [flatbuffers 23.1.21][flatbuffers] | 45.61% | † | 78.96% | 88.10% | 88.62% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.23% | 1.25% | 58.31% | 72.99% | 74.18% |
| [postcard 1.0.4][postcard] | 71.67% | 32.91% | 78.96% | 88.10% | 88.62% |
| [prost 0.11.9][prost] | <span title="populate + encode">*5.41%\**</span> <span title="encode">*7.39%\**</span> | 2.62% | 54.14% | 70.93% | 73.78% |
| [rkyv 0.7.41][rkyv] | 81.74% | <span title="unvalidated">*92.05%\**</span> <span title="validated upfront with error">*91.88%\**</span> | 78.96% | 88.10% | 88.62% |
| [rmp-serde 1.1.1][rmp-serde] | 2.06% | 1.70% | 58.31% | 72.97% | 74.13% |
| [ron 0.8.0][ron] | 0.18% | 0.12% | 21.35% | 52.62% | 58.21% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 10.83% | 6.59% | 78.96% | 88.10% | 88.62% |
| [serde_bare 0.5.0][serde_bare] | 5.93% | 6.40% | 78.96% | 88.10% | 88.62% |
| [serde_cbor 0.11.2][serde_cbor] | 0.82% | 0.79% | 36.10% | 62.98% | 70.09% |
| [serde_json 1.0.96][serde_json] | 0.38% | 0.45% | 18.09% | 49.32% | 55.18% |
| [simd-json 0.9.2][simd-json] | 0.59% | 0.38% | 18.09% | 49.32% | 55.18% |
| [speedy 0.8.6][speedy] | 99.98% | 99.93% | 78.96% | 88.10% | 88.62% |
| [alkahest 0.1.5][alkahest] | 99.82% | † | 78.96% | 88.10% | 88.62% |
| [dlhn 0.1.4][dlhn] | 7.52% | 4.40% | 78.96% | 88.10% | 88.62% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*62.04%\**</span> | <span title="unvalidated">*14.20%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.64%\**</span> | <span title="validated on-demand with error">*0.68%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*44.85%\**</span> <span title="validated upfront with error">*2.49%\**</span> | <span title="unvalidated">*35.00%\**</span> <span title="validated upfront with error">*34.99%\**</span> | ‡ |
| [rkyv 0.7.41][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*16.04%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*98.38%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*56.83%\**</span> | <span title="validated on-demand with panic">*34.97%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 230.94 µs | <span title="unvalidated">*1.8861 ms\**</span> | 1290592 | 391059 | 330489 |
| [bincode 1.3.3][bincode] | 640.09 µs | 2.5781 ms | 569975 | 240897 | 232423 |
| [bitcode 0.3.7][bitcode] | 703.73 µs | 2.8392 ms | 323111 | 215477 | 201612 |
| [borsh 0.10.3][borsh] | 576.24 µs | 2.9762 ms | 446595 | 234395 | 210008 |
| [bson 2.6.0][bson] | 3.5856 ms | 11.541 ms | 1619653 | 506953 | 328399 |
| [capnp 0.16.1][capnp] | 632.91 µs | † | 803896 | 336655 | 280851 |
| [ciborium 0.2.0][ciborium] | 3.7070 ms | 10.961 ms | 1109821 | 347812 | 274526 |
| [flatbuffers 23.1.21][flatbuffers] | 3.3760 ms | † | 844168 | 346957 | 294015 |
| [nachricht-serde 0.4.0][nachricht-serde] | 7.2273 ms | 5.1621 ms | 449745 | 252743 | 231110 |
| [postcard 1.0.4][postcard] | 487.32 µs | 2.7850 ms | 367489 | 222144 | 207344 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.9228 ms\**</span> <span title="encode">*1.3818 ms\**</span> | 4.7332 ms | 596811 | 306728 | 269310 |
| [rkyv 0.7.41][rkyv] | 395.91 µs | <span title="unvalidated">*1.8058 ms\**</span> <span title="validated upfront with error">*2.6189 ms\**</span> | 596952 | 254139 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.8532 ms | 4.1055 ms | 424533 | 245594 | 226188 |
| [ron 0.8.0][ron] | 10.804 ms | 21.537 ms | 1465223 | 439761 | 343338 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 681.31 µs | 2.6034 ms | 356311 | 213188 | 198524 |
| [serde_bare 0.5.0][serde_bare] | 807.99 µs | 3.1882 ms | 356311 | 213270 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.1257 ms | 6.5562 ms | 1109821 | 347812 | 274526 |
| [serde_json 1.0.96][serde_json] | 4.3724 ms | 9.6919 ms | 1623191 | 472275 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.7945 ms | 5.6951 ms | 1623191 | 472275 | 359623 |
| [speedy 0.8.6][speedy] | 358.48 µs | 2.3387 ms | 449595 | 235136 | 210361 |
| [alkahest 0.1.5][alkahest] | 276.78 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*35.829 µs\**</span> | <span title="unvalidated">*36.478 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*119.56 ns\**</span> | <span title="validated on-demand with error">*656.77 ns\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6900 ns\**</span> <span title="validated upfront with error">*2.6325 ms\**</span> | <span title="unvalidated">*1.9943 µs\**</span> <span title="validated upfront with error">*2.6355 ms\**</span> | ‡ |
| [rkyv 0.7.41][rkyv] | <span title="unvalidated">*1.6066 ns\**</span> <span title="validated upfront with error">*764.53 µs\**</span> | <span title="unvalidated">*145.48 ns\**</span> <span title="validated upfront with error">*767.28 µs\**</span> | 1.6834 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.1178 ns\**</span> | <span title="validated on-demand with panic">*8.0356 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*95.74%\**</span> | 25.04% | 54.52% | 60.06% |
| [bincode 1.3.3][bincode] | 36.08% | 70.04% | 56.69% | 88.50% | 85.40% |
| [bitcode 0.3.7][bitcode] | 32.82% | 63.60% | 100.00% | 98.94% | 98.45% |
| [borsh 0.10.3][borsh] | 40.08% | 60.67% | 72.35% | 90.95% | 94.51% |
| [bson 2.6.0][bson] | 6.44% | 15.65% | 19.95% | 42.05% | 60.44% |
| [capnp 0.16.1][capnp] | 36.49% | † | 40.19% | 63.33% | 70.67% |
| [ciborium 0.2.0][ciborium] | 6.23% | 16.47% | 29.11% | 61.29% | 72.30% |
| [flatbuffers 23.1.21][flatbuffers] | 6.84% | † | 38.28% | 61.45% | 67.51% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.20% | 34.98% | 71.84% | 84.35% | 85.88% |
| [postcard 1.0.4][postcard] | 47.39% | 64.84% | 87.92% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="populate + encode">*5.89%\**</span> <span title="encode">*16.71%\**</span> | 38.15% | 54.14% | 69.50% | 73.70% |
| [rkyv 0.7.41][rkyv] | 58.33% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*68.95%\**</span> | 54.13% | 83.89% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 12.46% | 43.98% | 76.11% | 86.81% | 87.75% |
| [ron 0.8.0][ron] | 2.14% | 8.38% | 22.05% | 48.48% | 57.81% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 33.90% | 69.36% | 90.68% | 100.00% | 99.98% |
| [serde_bare 0.5.0][serde_bare] | 28.58% | 56.64% | 90.68% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 10.86% | 27.54% | 29.11% | 61.29% | 72.30% |
| [serde_json 1.0.96][serde_json] | 5.28% | 18.63% | 19.91% | 45.14% | 55.19% |
| [simd-json 0.9.2][simd-json] | 8.26% | 31.71% | 19.91% | 45.14% | 55.19% |
| [speedy 0.8.6][speedy] | 64.42% | 77.21% | 71.87% | 90.67% | 94.36% |
| [alkahest 0.1.5][alkahest] | 83.44% | † | 48.40% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.40%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.34%\**</span> | <span title="validated on-demand with error">*22.15%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*59.72%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.29%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.41][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*75.86%\**</span> | <span title="validated on-demand with panic">*1.81%\**</span> | ‡ |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.3.7
[borsh]: https://crates.io/crates/borsh/0.10.3
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.16.1
[ciborium]: https://crates.io/crates/ciborium/0.2.0
[flatbuffers]: https://crates.io/crates/flatbuffers/23.1.21
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[postcard]: https://crates.io/crates/postcard/1.0.4
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.41
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.1
[ron]: https://crates.io/crates/ron/0.8.0
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.5.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.96
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[dlhn]: https://crates.io/crates/dlhn/0.1.4




## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
