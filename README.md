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
| [abomonation 0.7.3][abomonation] | 295.45 µs | <span title="unvalidated">*3.1177 ms\**</span> | 1705800 | 507675 | 403426 |
| [bincode 1.3.3][bincode] | 735.12 µs | 3.9995 ms | 1045784 | 374305 | 311761 |
| [bitcode 0.4.0][bitcode] | 625.58 µs | 4.2192 ms | 703664 | 320922 | 273622 |
| [borsh 0.10.3][borsh] | 609.83 µs | 4.3589 ms | 885780 | 363280 | 286514 |
| [bson 2.6.0][bson] | 3.3841 ms | 13.304 ms | 1924682 | 537661 | 376270 |
| [capnp 0.16.1][capnp] | 923.61 µs | † | 1443216 | 509618 | 428649 |
| [ciborium 0.2.1][ciborium] | 5.3362 ms | 14.608 ms | 1407835 | 407372 | 324081 |
| [flatbuffers 23.5.26][flatbuffers] | 2.4398 ms | † | 1276368 | 469962 | 388832 |
| [msgpacker 0.4.3][msgpacker] | 1.4151 ms | 4.8305 ms | 764996 | 316445 | 264898 |
| [nachricht-serde 0.4.0][nachricht-serde] | 9.4739 ms | 7.5119 ms | 818669 | 334639 | 285514 |
| [postcard 1.0.4][postcard] | 529.81 µs | 4.3635 ms | 724953 | 303462 | 253747 |
| [prost 0.11.9][prost] | <span title="populate + encode">*4.3393 ms\**</span> <span title="encode">*700.25 µs\**</span> | 4.9500 ms | 764951 | 269811 | 227947 |
| [rkyv 0.7.42][rkyv] | 408.38 µs | <span title="unvalidated">*3.2918 ms\**</span> <span title="validated upfront with error">*4.3122 ms\**</span> | 1011488 | 384478 | 333545 |
| [rmp-serde 1.1.1][rmp-serde] | 1.9429 ms | 5.8697 ms | 784997 | 326654 | 278219 |
| [ron 0.8.0][ron] | 26.332 ms | 26.531 ms | 1607459 | 452648 | 349713 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 742.11 µs | 4.6229 ms | 765778 | 312771 | 264518 |
| [serde_bare 0.5.0][serde_bare] | 976.06 µs | 4.0813 ms | 765778 | 312739 | 264630 |
| [serde_cbor 0.11.2][serde_cbor] | 2.6407 ms | 8.7158 ms | 1407835 | 407372 | 324081 |
| [serde_json 1.0.99][serde_json] | 6.3301 ms | 11.779 ms | 1827461 | 474358 | 361090 |
| [simd-json 0.9.2][simd-json] | 2.8736 ms | 7.7344 ms | 1827461 | 474358 | 361090 |
| [speedy 0.8.6][speedy] | 298.41 µs | 3.5292 ms | 885780 | 363280 | 286514 |
| [alkahest 0.1.5][alkahest] | 314.87 µs | † | 1045784 | 454748 | 389424 |
| [dlhn 0.1.6][dlhn] | 1.0417 ms | 4.8495 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*72.810 µs\**</span> | <span title="unvalidated">*111.75 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*134.42 ns\**</span> | <span title="validated on-demand with error">*486.21 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.8765 ns\**</span> <span title="validated upfront with error">*2.8985 ms\**</span> | <span title="unvalidated">*126.04 µs\**</span> <span title="validated upfront with error">*2.7552 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7175 ns\**</span> <span title="validated upfront with error">*987.11 µs\**</span> | <span title="unvalidated">*23.815 µs\**</span> <span title="validated upfront with error">*1.0197 ms\**</span> | 39.112 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.2088 ns\**</span> | <span title="validated on-demand with panic">*46.921 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 53.15% | 56.50% |
| [bincode 1.3.3][bincode] | 40.19% | 77.95% | 67.29% | 72.08% | 73.12% |
| [bitcode 0.4.0][bitcode] | 47.23% | 73.89% | 100.00% | 84.07% | 83.31% |
| [borsh 0.10.3][borsh] | 48.45% | 71.52% | 79.44% | 74.27% | 79.56% |
| [bson 2.6.0][bson] | 8.73% | 23.43% | 36.56% | 50.18% | 60.58% |
| [capnp 0.16.1][capnp] | 31.99% | † | 48.76% | 52.94% | 53.18% |
| [ciborium 0.2.1][ciborium] | 5.54% | 21.34% | 49.98% | 66.23% | 70.34% |
| [flatbuffers 23.5.26][flatbuffers] | 12.11% | † | 55.13% | 57.41% | 58.62% |
| [msgpacker 0.4.3][msgpacker] | 20.88% | 64.54% | 91.98% | 85.26% | 86.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.12% | 41.50% | 85.95% | 80.63% | 79.84% |
| [postcard 1.0.4][postcard] | 55.77% | 71.45% | 97.06% | 88.91% | 89.83% |
| [prost 0.11.9][prost] | <span title="populate + encode">*6.81%\**</span> <span title="encode">*42.19%\**</span> | 62.98% | 91.99% | 100.00% | 100.00% |
| [rkyv 0.7.42][rkyv] | 72.35% | <span title="unvalidated">*94.71%\**</span> <span title="validated upfront with error">*72.30%\**</span> | 69.57% | 70.18% | 68.34% |
| [rmp-serde 1.1.1][rmp-serde] | 15.21% | 53.12% | 89.64% | 82.60% | 81.93% |
| [ron 0.8.0][ron] | 1.12% | 11.75% | 43.77% | 59.61% | 65.18% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 39.81% | 67.44% | 91.89% | 86.26% | 86.17% |
| [serde_bare 0.5.0][serde_bare] | 30.27% | 76.39% | 91.89% | 86.27% | 86.14% |
| [serde_cbor 0.11.2][serde_cbor] | 11.19% | 35.77% | 49.98% | 66.23% | 70.34% |
| [serde_json 1.0.99][serde_json] | 4.67% | 26.47% | 38.51% | 56.88% | 63.13% |
| [simd-json 0.9.2][simd-json] | 10.28% | 40.31% | 38.51% | 56.88% | 63.13% |
| [speedy 0.8.6][speedy] | 99.01% | 88.34% | 79.44% | 74.27% | 79.56% |
| [alkahest 0.1.5][alkahest] | 93.83% | † | 67.29% | 59.33% | 58.53% |
| [dlhn 0.1.6][dlhn] | 28.36% | 64.29% | 97.06% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*21.31%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.28%\**</span> | <span title="validated on-demand with error">*4.90%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*44.31%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.89%\**</span> <span title="validated upfront with error">*0.86%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.34%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*53.52%\**</span> | <span title="validated on-demand with panic">*50.76%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 490.32 µs | <span title="unvalidated">*514.78 µs\**</span> | 6000024 | 5380836 | 5345891 |
| [bincode 1.3.3][bincode] | 5.8003 ms | 7.3427 ms | 6000008 | 5380823 | 5345890 |
| [bitcode 0.4.0][bitcode] | 6.7082 ms | 11.418 ms | 4688054 | 4688484 | 4688168 |
| [borsh 0.10.3][borsh] | 7.5123 ms | 4.7987 ms | 6000004 | 5380818 | 5345889 |
| [bson 2.6.0][bson] | 72.516 ms | 144.81 ms | 23013911 | 9211138 | 7497811 |
| [capnp 0.16.1][capnp] | 14.326 ms | † | 14000088 | 6729881 | 6051062 |
| [ciborium 0.2.1][ciborium] | 114.81 ms | 147.39 ms | 13122324 | 7527423 | 6759658 |
| [flatbuffers 23.5.26][flatbuffers] | 1.9356 ms | † | 6000024 | 5380800 | 5345910 |
| [msgpacker 0.4.3][msgpacker] | 29.635 ms | 13.899 ms | 7500005 | 6059282 | 6014337 |
| [nachricht-serde 0.4.0][nachricht-serde] | 197.75 ms | 49.058 ms | 8125037 | 6495174 | 6386940 |
| [postcard 1.0.4][postcard] | 873.57 µs | 1.9764 ms | 6000003 | 5380817 | 5345900 |
| [prost 0.11.9][prost] | <span title="populate + encode">*11.520 ms\**</span> <span title="encode">*9.1906 ms\**</span> | 19.898 ms | 8750000 | 6683814 | 6421871 |
| [rkyv 0.7.42][rkyv] | 930.56 µs | <span title="unvalidated">*582.16 µs\**</span> <span title="validated upfront with error">*588.84 µs\**</span> | 6000008 | 5380822 | 5345892 |
| [rmp-serde 1.1.1][rmp-serde] | 21.980 ms | 29.730 ms | 8125006 | 6496879 | 6391037 |
| [ron 0.8.0][ron] | 296.75 ms | 460.47 ms | 22192885 | 9009575 | 8138755 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 5.6373 ms | 7.8811 ms | 6000004 | 5380818 | 5345889 |
| [serde_bare 0.5.0][serde_bare] | 8.0449 ms | 7.4758 ms | 6000003 | 5380817 | 5345900 |
| [serde_cbor 0.11.2][serde_cbor] | 57.267 ms | 73.639 ms | 13122324 | 7527423 | 6759658 |
| [serde_json 1.0.99][serde_json] | 151.79 ms | 134.97 ms | 26192883 | 9612105 | 8586741 |
| [simd-json 0.9.2][simd-json] | 85.307 ms | 151.62 ms | 26192883 | 9612105 | 8586741 |
| [speedy 0.8.6][speedy] | 514.76 µs | 502.45 µs | 6000004 | 5380818 | 5345889 |
| [alkahest 0.1.5][alkahest] | 604.28 µs | † | 6000008 | 5380823 | 5345890 |
| [dlhn 0.1.6][dlhn] | 10.509 ms | 13.909 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*3.2586 ns\**</span> | <span title="unvalidated">*268.85 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*230.13 ns\**</span> | <span title="validated on-demand with error">*7.4570 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.8799 ns\**</span> <span title="validated upfront with error">*58.186 ns\**</span> | <span title="unvalidated">*106.65 µs\**</span> <span title="validated upfront with error">*105.06 µs\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.6752 ns\**</span> <span title="validated upfront with error">*17.290 ns\**</span> | <span title="unvalidated">*51.452 µs\**</span> <span title="validated upfront with error">*51.759 µs\**</span> | 372.24 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.2653 ns\**</span> | <span title="validated on-demand with panic">*104.85 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*97.60%\**</span> | 78.13% | 87.13% | 87.70% |
| [bincode 1.3.3][bincode] | 8.45% | 6.84% | 78.13% | 87.13% | 87.70% |
| [bitcode 0.4.0][bitcode] | 7.31% | 4.40% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 6.53% | 10.47% | 78.13% | 87.13% | 87.70% |
| [bson 2.6.0][bson] | 0.68% | 0.35% | 20.37% | 50.90% | 62.53% |
| [capnp 0.16.1][capnp] | 3.42% | † | 33.49% | 69.67% | 77.48% |
| [ciborium 0.2.1][ciborium] | 0.43% | 0.34% | 35.73% | 62.29% | 69.36% |
| [flatbuffers 23.5.26][flatbuffers] | 25.33% | † | 78.13% | 87.13% | 87.70% |
| [msgpacker 0.4.3][msgpacker] | 1.65% | 3.62% | 62.51% | 77.38% | 77.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.25% | 1.02% | 57.70% | 72.18% | 73.40% |
| [postcard 1.0.4][postcard] | 56.13% | 25.42% | 78.13% | 87.13% | 87.70% |
| [prost 0.11.9][prost] | <span title="populate + encode">*4.26%\**</span> <span title="encode">*5.34%\**</span> | 2.53% | 53.58% | 70.15% | 73.00% |
| [rkyv 0.7.42][rkyv] | 52.69% | <span title="unvalidated">*86.31%\**</span> <span title="validated upfront with error">*85.33%\**</span> | 78.13% | 87.13% | 87.70% |
| [rmp-serde 1.1.1][rmp-serde] | 2.23% | 1.69% | 57.70% | 72.17% | 73.36% |
| [ron 0.8.0][ron] | 0.17% | 0.11% | 21.12% | 52.04% | 57.60% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 8.70% | 6.38% | 78.13% | 87.13% | 87.70% |
| [serde_bare 0.5.0][serde_bare] | 6.09% | 6.72% | 78.13% | 87.13% | 87.70% |
| [serde_cbor 0.11.2][serde_cbor] | 0.86% | 0.68% | 35.73% | 62.29% | 69.36% |
| [serde_json 1.0.99][serde_json] | 0.32% | 0.37% | 17.90% | 48.78% | 54.60% |
| [simd-json 0.9.2][simd-json] | 0.57% | 0.33% | 17.90% | 48.78% | 54.60% |
| [speedy 0.8.6][speedy] | 95.25% | 100.00% | 78.13% | 87.13% | 87.70% |
| [alkahest 0.1.5][alkahest] | 81.14% | † | 78.13% | 87.13% | 87.70% |
| [dlhn 0.1.6][dlhn] | 4.67% | 3.61% | 78.13% | 87.13% | 87.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*51.41%\**</span> | <span title="unvalidated">*19.14%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*0.73%\**</span> | <span title="validated on-demand with error">*0.69%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*43.18%\**</span> <span title="validated upfront with error">*2.88%\**</span> | <span title="unvalidated">*48.24%\**</span> <span title="validated upfront with error">*48.97%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.69%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.41%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*51.30%\**</span> | <span title="validated on-demand with panic">*49.07%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 294.76 µs | <span title="unvalidated">*2.5395 ms\**</span> | 1290592 | 395298 | 332924 |
| [bincode 1.3.3][bincode] | 815.10 µs | 3.3422 ms | 569975 | 240897 | 232423 |
| [bitcode 0.4.0][bitcode] | 513.02 µs | 3.3926 ms | 322798 | 215013 | 201247 |
| [borsh 0.10.3][borsh] | 709.57 µs | 3.4208 ms | 446595 | 234395 | 210008 |
| [bson 2.6.0][bson] | 4.7760 ms | 16.528 ms | 1619653 | 506953 | 328399 |
| [capnp 0.16.1][capnp] | 827.21 µs | † | 803896 | 336655 | 280851 |
| [ciborium 0.2.1][ciborium] | 4.8743 ms | 13.818 ms | 1109821 | 347812 | 274526 |
| [dlhn 0.1.6][dlhn] | 1.1129 ms | 4.9088 ms | 366496 | 220835 | 205683 |
| [flatbuffers 23.5.26][flatbuffers] | 4.8767 ms | † | 844168 | 346957 | 294015 |
| [msgpacker 0.4.3][msgpacker] | 1.6952 ms | 5.0540 ms | 391251 | 237270 | 220476 |
| [nachricht-serde 0.4.0][nachricht-serde] | 9.3370 ms | 6.7423 ms | 449745 | 252743 | 231110 |
| [postcard 1.0.4][postcard] | 630.37 µs | 3.5214 ms | 367489 | 222144 | 207344 |
| [prost 0.11.9][prost] | <span title="populate + encode">*5.0477 ms\**</span> <span title="encode">*1.6810 ms\**</span> | 5.9484 ms | 596811 | 306728 | 269310 |
| [rkyv 0.7.42][rkyv] | 493.77 µs | <span title="unvalidated">*2.4480 ms\**</span> <span title="validated upfront with error">*3.3923 ms\**</span> | 596952 | 254139 | 220706 |
| [rmp-serde 1.1.1][rmp-serde] | 2.1859 ms | 5.2444 ms | 424533 | 245594 | 226188 |
| [ron 0.8.0][ron] | 14.425 ms | 27.921 ms | 1465223 | 439761 | 343338 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 824.19 µs | 3.8483 ms | 356311 | 213188 | 198524 |
| [serde_bare 0.5.0][serde_bare] | 1.1007 ms | 4.1642 ms | 356311 | 213270 | 198488 |
| [serde_cbor 0.11.2][serde_cbor] | 2.5306 ms | 8.6123 ms | 1109821 | 347812 | 274526 |
| [serde_json 1.0.99][serde_json] | 6.2620 ms | 13.480 ms | 1623191 | 472275 | 359623 |
| [simd-json 0.9.2][simd-json] | 3.3313 ms | 7.6785 ms | 1623191 | 472275 | 359623 |
| [speedy 0.8.6][speedy] | 459.40 µs | 3.1539 ms | 449595 | 235136 | 210361 |
| [alkahest 0.1.5][alkahest] | 361.11 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*80.971 µs\**</span> | <span title="unvalidated">*82.623 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*138.71 ns\**</span> | <span title="validated on-demand with error">*850.51 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*4.0251 ns\**</span> <span title="validated upfront with error">*2.9314 ms\**</span> | <span title="unvalidated">*3.7453 µs\**</span> <span title="validated upfront with error">*2.9719 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.7140 ns\**</span> <span title="validated upfront with error">*847.19 µs\**</span> | <span title="unvalidated">*225.90 ns\**</span> <span title="validated upfront with error">*848.59 µs\**</span> | 2.9590 µs |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.0978 ns\**</span> | <span title="validated on-demand with panic">*9.6722 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*96.40%\**</span> | 25.01% | 53.93% | 59.62% |
| [bincode 1.3.3][bincode] | 36.16% | 73.25% | 56.63% | 88.50% | 85.40% |
| [bitcode 0.4.0][bitcode] | 57.46% | 72.16% | 100.00% | 99.15% | 98.63% |
| [borsh 0.10.3][borsh] | 41.54% | 71.56% | 72.28% | 90.95% | 94.51% |
| [bson 2.6.0][bson] | 6.17% | 14.81% | 19.93% | 42.05% | 60.44% |
| [capnp 0.16.1][capnp] | 35.63% | † | 40.15% | 63.33% | 70.67% |
| [ciborium 0.2.1][ciborium] | 6.05% | 17.72% | 29.09% | 61.29% | 72.30% |
| [dlhn 0.1.6][dlhn] | 26.49% | 49.87% | 88.08% | 96.54% | 96.50% |
| [flatbuffers 23.5.26][flatbuffers] | 6.04% | † | 38.24% | 61.45% | 67.51% |
| [msgpacker 0.4.3][msgpacker] | 17.39% | 48.44% | 82.50% | 89.85% | 90.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.16% | 36.31% | 71.77% | 84.35% | 85.88% |
| [postcard 1.0.4][postcard] | 46.76% | 69.52% | 87.84% | 95.97% | 95.73% |
| [prost 0.11.9][prost] | <span title="populate + encode">*5.84%\**</span> <span title="encode">*17.53%\**</span> | 41.15% | 54.09% | 69.50% | 73.70% |
| [rkyv 0.7.42][rkyv] | 59.70% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.16%\**</span> | 54.07% | 83.89% | 89.93% |
| [rmp-serde 1.1.1][rmp-serde] | 13.48% | 46.68% | 76.04% | 86.81% | 87.75% |
| [ron 0.8.0][ron] | 2.04% | 8.77% | 22.03% | 48.48% | 57.81% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 35.76% | 63.61% | 90.59% | 100.00% | 99.98% |
| [serde_bare 0.5.0][serde_bare] | 26.78% | 58.79% | 90.59% | 99.96% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 11.65% | 28.42% | 29.09% | 61.29% | 72.30% |
| [serde_json 1.0.99][serde_json] | 4.71% | 18.16% | 19.89% | 45.14% | 55.19% |
| [simd-json 0.9.2][simd-json] | 8.85% | 31.88% | 19.89% | 45.14% | 55.19% |
| [speedy 0.8.6][speedy] | 64.16% | 77.62% | 71.80% | 90.67% | 94.36% |
| [alkahest 0.1.5][alkahest] | 81.63% | † | 48.35% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.27%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.24%\**</span> | <span title="validated on-demand with error">*26.56%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*42.58%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.03%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*55.33%\**</span> | <span title="validated on-demand with panic">*2.34%\**</span> | ‡ |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 725.55 µs | <span title="unvalidated">*4.3504 ms\**</span> | 2984682 | 1444851 | 1312128 |
| [alkahest 0.1.5][alkahest] | 1.0468 ms | † | 1863391 | 1234238 | 1202345 |
| [bincode 1.3.3][bincode] | 4.3803 ms | 6.9129 ms | 1811011 | 1115517 | 1025627 |
| [bitcode 0.4.0][bitcode] | 1.8385 ms | 5.8281 ms | 870693 | 866743 | 870720 |
| [borsh 0.10.3][borsh] | 3.4940 ms | 6.8670 ms | 1486162 | 1083024 | 1013550 |
| [bson 2.6.0][bson] | 40.411 ms | 78.391 ms | 10030880 | 2847180 | 1600859 |
| [capnp 0.16.1][capnp] | 3.2525 ms | † | 2664040 | 1514537 | 1212087 |
| [ciborium 0.2.1][ciborium] | 26.681 ms | 66.391 ms | 5878653 | 1662596 | 1431560 |
| [dlhn 0.1.6][dlhn] | 7.8524 ms | 13.725 ms | 1279599 | 1052966 | 1021161 |
| [flatbuffers 23.5.26][flatbuffers] | 6.6630 ms | † | 2273740 | 1408433 | 1235566 |
| [nachricht-serde 0.4.0][nachricht-serde] | 48.138 ms | 26.335 ms | 1728519 | 1249189 | 1233323 |
| [postcard 1.0.4][postcard] | 2.4777 ms | 6.5153 ms | 1279599 | 1058773 | 1016738 |
| [prost 0.11.9][prost] | <span title="populate + encode">*13.700 ms\**</span> <span title="encode">*6.4389 ms\**</span> | 14.727 ms | 1818378 | 1311199 | 1266311 |
| [rkyv 0.7.42][rkyv] | 1.9683 ms | <span title="unvalidated">*3.6426 ms\**</span> <span title="validated upfront with error">*4.8712 ms\**</span> | 2029080 | 1335913 | 1158855 |
| [rmp-serde 1.1.1][rmp-serde] | 14.044 ms | 18.230 ms | 1703813 | 1233850 | 1200208 |
| [ron 0.8.0][ron] | 53.774 ms | 147.55 ms | 8476284 | 2199231 | 1783971 |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 3.4253 ms | 5.3496 ms | 1288257 | 1039829 | 986510 |
| [serde_bare 0.5.0][serde_bare] | 5.4187 ms | 8.7860 ms | 1288257 | 1038144 | 984356 |
| [serde_cbor 0.11.2][serde_cbor] | 13.720 ms | 35.281 ms | 5878653 | 1662596 | 1431560 |
| [serde_json 1.0.99][serde_json] | 35.957 ms | 56.169 ms | 9175594 | 2352701 | 1800713 |
| [simd-json 0.9.2][simd-json] | 17.786 ms | 49.240 ms | 9175594 | 2352701 | 1800713 |
| [speedy 0.8.6][speedy] | 1.2897 ms | 4.2001 ms | 1546963 | 1093931 | 1013443 |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*186.53 µs\**</span> | <span title="unvalidated">*186.28 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*3.1702 ns\**</span> | <span title="validated on-demand with panic">*1.0144 µs\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*131.01 ns\**</span> | <span title="validated on-demand with error">*1.4444 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*3.5845 ns\**</span> <span title="validated upfront with error">*5.7911 ms\**</span> | <span title="unvalidated">*6.0646 µs\**</span> <span title="validated upfront with error">*6.8449 ms\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*1.5259 ns\**</span> <span title="validated upfront with error">*974.33 µs\**</span> | <span title="unvalidated">*561.11 ns\**</span> <span title="validated upfront with error">*969.22 µs\**</span> | 985.64 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*83.73%\**</span> | 29.17% | 59.99% | 66.36% |
| [alkahest 0.1.5][alkahest] | 69.31% | † | 46.73% | 70.22% | 72.42% |
| [bincode 1.3.3][bincode] | 16.56% | 52.69% | 48.08% | 77.70% | 84.90% |
| [bitcode 0.4.0][bitcode] | 39.46% | 62.50% | 100.00% | 100.00% | 100.00% |
| [borsh 0.10.3][borsh] | 20.77% | 53.04% | 58.59% | 80.03% | 85.91% |
| [bson 2.6.0][bson] | 1.80% | 4.65% | 8.68% | 30.44% | 54.39% |
| [capnp 0.16.1][capnp] | 22.31% | † | 32.68% | 57.23% | 71.84% |
| [ciborium 0.2.1][ciborium] | 2.72% | 5.49% | 14.81% | 52.13% | 60.82% |
| [dlhn 0.1.6][dlhn] | 9.24% | 26.54% | 68.04% | 82.31% | 85.27% |
| [flatbuffers 23.5.26][flatbuffers] | 10.89% | † | 38.29% | 61.54% | 70.47% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.51% | 13.83% | 50.37% | 69.38% | 70.60% |
| [postcard 1.0.4][postcard] | 29.28% | 55.91% | 68.04% | 81.86% | 85.64% |
| [prost 0.11.9][prost] | <span title="populate + encode">*5.30%\**</span> <span title="encode">*11.27%\**</span> | 24.73% | 47.88% | 66.10% | 68.76% |
| [rkyv 0.7.42][rkyv] | 36.86% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*74.78%\**</span> | 42.91% | 64.88% | 75.14% |
| [rmp-serde 1.1.1][rmp-serde] | 5.17% | 19.98% | 51.10% | 70.25% | 72.55% |
| [ron 0.8.0][ron] | 1.35% | 2.47% | 10.27% | 39.41% | 48.81% |
| [parity-scale-codec 3.6.3][parity-scale-codec] | 21.18% | 68.09% | 67.59% | 83.35% | 88.26% |
| [serde_bare 0.5.0][serde_bare] | 13.39% | 41.46% | 67.59% | 83.49% | 88.46% |
| [serde_cbor 0.11.2][serde_cbor] | 5.29% | 10.32% | 14.81% | 52.13% | 60.82% |
| [serde_json 1.0.99][serde_json] | 2.02% | 6.49% | 9.49% | 36.84% | 48.35% |
| [simd-json 0.9.2][simd-json] | 4.08% | 7.40% | 9.49% | 36.84% | 48.35% |
| [speedy 0.8.6][speedy] | 56.26% | 86.73% | 56.28% | 79.23% | 85.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.30%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*48.13%\**</span> | <span title="validated on-demand with panic">*55.31%\**</span> | ‡ |
| [capnp 0.16.1][capnp] | <span title="validated on-demand with error">*1.16%\**</span> | <span title="validated on-demand with error">*38.85%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*42.57%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*9.25%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.42][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[bincode]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.4.0
[borsh]: https://crates.io/crates/borsh/0.10.3
[bson]: https://crates.io/crates/bson/2.6.0
[capnp]: https://crates.io/crates/capnp/0.16.1
[ciborium]: https://crates.io/crates/ciborium/0.2.1
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[postcard]: https://crates.io/crates/postcard/1.0.4
[prost]: https://crates.io/crates/prost/0.11.9
[rkyv]: https://crates.io/crates/rkyv/0.7.42
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.1
[ron]: https://crates.io/crates/ron/0.8.0
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.3
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.99
[simd-json]: https://crates.io/crates/simd-json/0.9.2
[speedy]: https://crates.io/crates/speedy/0.8.6
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[dlhn]: https://crates.io/crates/dlhn/0.1.6




## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
