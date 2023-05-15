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
| [abomonation 0.7.3][abomonation] | 251.93 µs | <span title="unvalidated">*2.3970 ms\**</span> | 1705800 | 507683 | 403411 |
| [bincode 1.3.3][bincode] | 465.35 µs | 3.3851 ms | 1045784 | 374305 | 311761 |
| [bitcode 0.4.0][bitcode] | 510.36 µs | 3.2763 ms | 703664 | 320922 | 273622 |
| [borsh 0.10.3][borsh] | 498.59 µs | 3.5804 ms | 885780 | 363280 | 286514 |
| [bson 2.6.0][bson] | 2.4240 ms | 9.9087 ms | 1924682 | 537661 | 376270 |
| [capnp 0.16.1][capnp] | 741.76 µs | † | 1443216 | 509618 | 428649 |
| [ciborium 0.2.1][ciborium] | 3.8676 ms | 12.543 ms | 1407835 | 407372 | 324081 |
| [flatbuffers 23.1.21][flatbuffers] | 2.0682 ms | † | 1276368 | 469962 | 388832 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.5704 ms | 5.5684 ms | 818669 | 334639 | 285514 |
| [postcard 1.0.4][postcard] | 401.59 µs | 3.3476 ms | 724953 | 303462 | 253747 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.2318 ms\**</span> <span title="encode">*514.68 µs\**</span> | 3.8544 ms | 764951 | 269811 | 227947 |
| [rkyv 0.7.42][rkyv] | 307.89 µs | <span title="unvalidated">*2.4933 ms\**</span> <span title="validated upfront with error">*3.4964 ms\**</span> | 1011488 | 384478 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.7526 ms | 4.6024 ms | 784997 | 326654 | 278219 |
| [ron 0.8.0][ron] | 20.780 ms | 20.308 ms | 1607459 | 452648 | 349713 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 613.81 µs | 3.4091 ms | 765778 | 312771 | 264518 |
| [serde_bare 0.5.0][serde_bare] | 748.28 µs | 3.4090 ms | 765778 | 312739 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.1718 ms | 6.4786 ms | 1407835 | 407372 | 324081 |
| [serde_json 1.0.96][serde_json] | 4.1003 ms | 8.2949 ms | 1827461 | 474358 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.3034 ms | 5.5608 ms | 1827461 | 474358 | 361090 |
| [speedy 0.8.6][speedy] | 298.56 µs | 2.7951 ms | 885780 | 363280 | 286514 |
| [alkahest 0.1.5][alkahest] | 247.76 µs | † | 1045784 | 454748 | 389424 |
| [dlhn 0.1.5][dlhn] | 610.83 µs | 3.7473 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*27.755 µs\**</span> | <span title="unvalidated">*49.848 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*119.94 ns\**</span> | <span title="validated on-demand with error">*419.55 µs\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6844 ns\**</span> <span title="validated upfront with error">*2.2515 ms\**</span> | <span title="unvalidated">*79.818 µs\**</span> <span title="validated upfront with error">*2.3197 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2059 ns\**</span> <span title="validated upfront with error">*887.71 µs\**</span> | <span title="unvalidated">*17.638 µs\**</span> <span title="validated upfront with error">*901.64 µs\**</span> | 19.761 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.5452 ns\**</span> | <span title="validated on-demand with panic">*34.168 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 98.34% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 53.15% | 56.50% |
| [bincode 1.3.3][bincode] | 53.24% | 70.81% | 67.29% | 72.08% | 73.12% |
| [bitcode 0.4.0][bitcode] | 48.55% | 73.16% | 100.00% | 84.07% | 83.31% |
| [borsh 0.10.3][borsh] | 49.69% | 66.95% | 79.44% | 74.27% | 79.56% |
| [bson 2.6.0][bson] | 10.22% | 24.19% | 36.56% | 50.18% | 60.58% |
| [capnp 0.16.1][capnp] | 33.40% | † | 48.76% | 52.94% | 53.18% |
| [ciborium 0.2.1][ciborium] | 6.41% | 19.11% | 49.98% | 66.23% | 70.34% |
| [flatbuffers 23.1.21][flatbuffers] | 11.98% | † | 55.13% | 57.41% | 58.62% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.89% | 43.05% | 85.95% | 80.63% | 79.84% |
| [postcard 1.0.4][postcard] | 61.69% | 71.60% | 97.06% | 88.91% | 89.83% |
| [prost 0.11.9][prost] | <span title="populate + encode">*7.67%\**</span> <span title="encode">*48.14%\**</span> | 62.19% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 80.47% | <span title="unvalidated">*96.14%\**</span> <span title="validated upfront with error">*68.56%\**</span> | 69.57% | 70.18% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 14.14% | 52.08% | 89.64% | 82.60% | 81.93% |
| [ron 0.8.0][ron] | 1.19% | 11.80% | 43.77% | 59.61% | 65.18% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 40.36% | 70.31% | 91.89% | 86.26% | 86.17% |
| [serde_bare 0.5.0][serde_bare] | 33.11% | 70.31% | 91.89% | 86.27% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.41% | 37.00% | 49.98% | 66.23% | 70.34% |
| [serde_json 1.0.96][serde_json] | 6.04% | 28.90% | 38.51% | 56.88% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.76% | 43.11% | 38.51% | 56.88% | 63.13% |
| [speedy 0.8.6][speedy] | 82.98% | 85.76% | 79.44% | 74.27% | 79.56% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 67.29% | 59.33% | 58.53% |
| [dlhn 0.1.5][dlhn] | 40.56% | 63.97% | 97.06% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*35.38%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.01%\**</span> | <span title="validated on-demand with error">*4.20%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*44.92%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*22.10%\**</span> <span title="validated upfront with error">*0.76%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.96%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*47.38%\**</span> | <span title="validated on-demand with panic">*51.62%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 424.66 µs | <span title="unvalidated">*424.35 µs\**</span> | 6000024 | 5380836 | 5345891 |
| [bincode 1.3.3][bincode] | 3.7779 ms | 11.534 ms | 6000008 | 5380823 | 5345890 |
| [bitcode 0.4.0][bitcode] | 4.8975 ms | 9.5550 ms | 4688054 | 4688484 | 4688168 |
| [borsh 0.10.3][borsh] | 5.3109 ms | 6.1706 ms | 6000004 | 5380818 | 5345889 |
| [bson 2.6.0][bson] | 43.584 ms | 112.75 ms | 23013911 | 9211138 | 7497811 |
| [capnp 0.16.1][capnp] | 7.9881 ms | † | 14000088 | 6729881 | 6051062 |
| [ciborium 0.2.1][ciborium] | 89.367 ms | 121.62 ms | 13122324 | 7527423 | 6759658 |
| [flatbuffers 23.1.21][flatbuffers] | 965.07 µs | † | 6000024 | 5380800 | 5345910 |
| [nachricht-serde 0.4.0][nachricht-serde] | 210.32 ms | 33.861 ms | 8125037 | 6495174 | 6386940 |
| [postcard 1.0.4][postcard] | 592.65 µs | 1.2782 ms | 6000003 | 5380817 | 5345900 |
| [prost 0.11.9][prost] | <span title="populate + encode">*7.5748 ms\**</span> <span title="encode">*6.0550 ms\**</span> | 15.205 ms | 8750000 | 6683814 | 6421871 |
| [rkyv 0.7.42][rkyv] | 517.19 µs | <span title="unvalidated">*406.81 µs\**</span> <span title="validated upfront with error">*406.53 µs\**</span> | 6000008 | 5380822 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 18.835 ms | 24.317 ms | 8125006 | 6496879 | 6391037 |
| [ron 0.8.0][ron] | 227.17 ms | 346.55 ms | 22192885 | 9009575 | 8138755 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 5.0160 ms | 7.4722 ms | 6000004 | 5380818 | 5345889 |
| [serde_bare 0.5.0][serde_bare] | 6.6480 ms | 6.3563 ms | 6000003 | 5380817 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 50.483 ms | 50.822 ms | 13122324 | 7527423 | 6759658 |
| [serde_json 1.0.96][serde_json] | 108.37 ms | 96.575 ms | 26192883 | 9612105 | 8586741 |
| [simd-json 0.9.2][simd-json] | 70.962 ms | 104.50 ms | 26192883 | 9612105 | 8586741 |
| [speedy 0.8.6][speedy] | 405.46 µs | 405.47 µs | 6000004 | 5380818 | 5345889 |
| [alkahest 0.1.5][alkahest] | 404.22 µs | † | 6000008 | 5380823 | 5345890 |
| [dlhn 0.1.5][dlhn] | 6.2775 ms | 9.3837 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.0003 ns\**</span> | <span title="unvalidated">*246.79 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*180.69 ns\**</span> | <span title="validated on-demand with error">*5.1485 ms\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6930 ns\**</span> <span title="validated upfront with error">*50.612 ns\**</span> | <span title="unvalidated">*100.39 µs\**</span> <span title="validated upfront with error">*100.45 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2059 ns\**</span> <span title="validated upfront with error">*13.010 ns\**</span> | <span title="unvalidated">*35.155 µs\**</span> <span title="validated upfront with error">*36.500 µs\**</span> | 224.85 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6607 ns\**</span> | <span title="validated on-demand with panic">*100.39 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 95.19% | <span title="unvalidated">*95.55%\**</span> | 78.13% | 87.13% | 87.70% |
| [bincode 1.3.3][bincode] | 10.70% | 3.52% | 78.13% | 87.13% | 87.70% |
| [bitcode 0.4.0][bitcode] | 8.25% | 4.24% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 7.61% | 6.57% | 78.13% | 87.13% | 87.70% |
| [bson 2.6.0][bson] | 0.93% | 0.36% | 20.37% | 50.90% | 62.53% |
| [capnp 0.16.1][capnp] | 5.06% | † | 33.49% | 69.67% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.45% | 0.33% | 35.73% | 62.29% | 69.36% |
| [flatbuffers 23.1.21][flatbuffers] | 41.89% | † | 78.13% | 87.13% | 87.70% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.19% | 1.20% | 57.70% | 72.18% | 73.40% |
| [postcard 1.0.4][postcard] | 68.21% | 31.72% | 78.13% | 87.13% | 87.70% |
| [prost 0.11.9][prost] | <span title="populate + encode">*5.34%\**</span> <span title="encode">*6.68%\**</span> | 2.67% | 53.58% | 70.15% | 73.00% |
| [rkyv 0.7.42][rkyv] | 78.16% | <span title="unvalidated">*99.67%\**</span> <span title="validated upfront with error">*99.74%\**</span> | 78.13% | 87.13% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.15% | 1.67% | 57.70% | 72.17% | 73.36% |
| [ron 0.8.0][ron] | 0.18% | 0.12% | 21.12% | 52.04% | 57.60% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 8.06% | 5.43% | 78.13% | 87.13% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 6.08% | 6.38% | 78.13% | 87.13% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.80% | 0.80% | 35.73% | 62.29% | 69.36% |
| [serde_json 1.0.96][serde_json] | 0.37% | 0.42% | 17.90% | 48.78% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.57% | 0.39% | 17.90% | 48.78% | 54.60% |
| [speedy 0.8.6][speedy] | 99.69% | 100.00% | 78.13% | 87.13% | 87.70% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 78.13% | 87.13% | 87.70% |
| [dlhn 0.1.5][dlhn] | 6.44% | 4.32% | 78.13% | 87.13% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*60.29%\**</span> | <span title="unvalidated">*14.24%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.67%\**</span> | <span title="validated on-demand with error">*0.68%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*44.78%\**</span> <span title="validated upfront with error">*2.38%\**</span> | <span title="unvalidated">*35.02%\**</span> <span title="validated upfront with error">*35.00%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.27%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*96.32%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*45.32%\**</span> | <span title="validated on-demand with panic">*35.02%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 239.13 µs | <span title="unvalidated">*1.9785 ms\**</span> | 1290592 | 391267 | 329024 |
| [bincode 1.3.3][bincode] | 641.09 µs | 2.5944 ms | 569975 | 240897 | 232423 |
| [bitcode 0.4.0][bitcode] | 374.20 µs | 2.6474 ms | 322798 | 215013 | 201247 |
| [borsh 0.10.3][borsh] | 573.48 µs | 2.9798 ms | 446595 | 234395 | 210008 |
| [bson 2.6.0][bson] | 3.3492 ms | 11.599 ms | 1619653 | 506953 | 328399 |
| [capnp 0.16.1][capnp] | 613.83 µs | † | 803896 | 336655 | 280851 |
| [ciborium 0.2.1][ciborium] | 3.7393 ms | 11.488 ms | 1109821 | 347812 | 274526 |
| [dlhn 0.1.5][dlhn] | 773.57 µs | 3.6286 ms | 366496 | 220835 | 205683 |
| [flatbuffers 23.1.21][flatbuffers] | 3.3288 ms | † | 844168 | 346957 | 294015 |
| [nachricht-serde 0.4.0][nachricht-serde] | 8.0994 ms | 5.2619 ms | 449745 | 252743 | 231110 |
| [postcard 1.0.4][postcard] | 476.07 µs | 2.7807 ms | 367489 | 222144 | 207344 |
| [prost 0.11.9][prost] | <span title="populate + encode">*3.8566 ms\**</span> <span title="encode">*1.2925 ms\**</span> | 4.7417 ms | 596811 | 306728 | 269310 |
| [rkyv 0.7.42][rkyv] | 404.71 µs | <span title="unvalidated">*1.8263 ms\**</span> <span title="validated upfront with error">*2.6276 ms\**</span> | 596952 | 254139 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 1.8676 ms | 4.0865 ms | 424533 | 245594 | 226188 |
| [ron 0.8.0][ron] | 10.909 ms | 20.891 ms | 1465223 | 439761 | 343338 |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 639.09 µs | 2.6090 ms | 356311 | 213188 | 198524 |
| [serde_bare 0.5.0][serde_bare] | 849.61 µs | 3.1893 ms | 356311 | 213270 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.0998 ms | 6.4081 ms | 1109821 | 347812 | 274526 |
| [serde_json 1.0.96][serde_json] | 4.3291 ms | 9.9933 ms | 1623191 | 472275 | 359623 |
| [simd-json 0.9.2][simd-json] | 2.6978 ms | 5.5874 ms | 1623191 | 472275 | 359623 |
| [speedy 0.8.6][speedy] | 356.31 µs | 2.2225 ms | 449595 | 235136 | 210361 |
| [alkahest 0.1.5][alkahest] | 274.19 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*38.374 µs\**</span> | <span title="unvalidated">*39.222 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*118.31 ns\**</span> | <span title="validated on-demand with error">*655.84 ns\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*2.6960 ns\**</span> <span title="validated upfront with error">*2.1381 ms\**</span> | <span title="unvalidated">*1.9903 µs\**</span> <span title="validated upfront with error">*2.1425 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.2058 ns\**</span> <span title="validated upfront with error">*751.14 µs\**</span> | <span title="unvalidated">*145.46 ns\**</span> <span title="validated upfront with error">*752.84 µs\**</span> | 1.6784 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*2.6675 ns\**</span> | <span title="validated on-demand with panic">*6.3262 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.31%\**</span> | 25.01% | 54.49% | 60.33% |
| [bincode 1.3.3][bincode] | 37.30% | 70.39% | 56.63% | 88.50% | 85.40% |
| [bitcode 0.4.0][bitcode] | 63.90% | 68.98% | 100.00% | 99.15% | 98.63% |
| [borsh 0.10.3][borsh] | 41.70% | 61.29% | 72.28% | 90.95% | 94.51% |
| [bson 2.6.0][bson] | 7.14% | 15.75% | 19.93% | 42.05% | 60.44% |
| [capnp 0.16.1][capnp] | 38.96% | † | 40.15% | 63.33% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.40% | 15.90% | 29.09% | 61.29% | 72.30% |
| [dlhn 0.1.5][dlhn] | 30.91% | 50.33% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.1.21][flatbuffers] | 7.18% | † | 38.24% | 61.45% | 67.51% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.95% | 34.71% | 71.77% | 84.35% | 85.88% |
| [postcard 1.0.4][postcard] | 50.23% | 65.68% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="populate + encode">*6.20%\**</span> <span title="encode">*18.50%\**</span> | 38.52% | 54.09% | 69.50% | 73.70% |
| [rkyv 0.7.42][rkyv] | 59.09% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*69.50%\**</span> | 54.07% | 83.89% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 12.80% | 44.69% | 76.04% | 86.81% | 87.75% |
| [ron 0.8.0][ron] | 2.19% | 8.74% | 22.03% | 48.48% | 57.81% |
| [parity-scale-codec 3.5.0][parity-scale-codec] | 37.42% | 70.00% | 90.59% | 100.00% | 99.98% |
| [serde_bare 0.5.0][serde_bare] | 28.15% | 57.26% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.39% | 28.50% | 29.09% | 61.29% | 72.30% |
| [serde_json 1.0.96][serde_json] | 5.52% | 18.28% | 19.89% | 45.14% | 55.19% |
| [simd-json 0.9.2][simd-json] | 8.86% | 32.69% | 19.89% | 45.14% | 55.19% |
| [speedy 0.8.6][speedy] | 67.11% | 82.17% | 71.80% | 90.67% | 94.36% |
| [alkahest 0.1.5][alkahest] | 87.21% | † | 48.35% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.37%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.02%\**</span> | <span title="validated on-demand with error">*22.18%\**</span> | ‡ |
| [flatbuffers 23.1.21][flatbuffers] | <span title="unvalidated">*44.73%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.31%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*45.20%\**</span> | <span title="validated on-demand with panic">*2.30%\**</span> | ‡ |

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
