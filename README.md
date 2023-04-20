<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

You can use [this horrible tiny webpage](https://davidkoloski.me/rust_serialization_benchmark) to
turn a benchmark log into nicely-formatted markdown tables.

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

## Last updated: 2023-4-19

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 267.64 µs | <span title="unvalidated">*2.3459 ms\**</span> | 1705800 | 502524 | 414456 |
| bare | 740.31 µs | 3.3721 ms | 765778 | 312739 | 264630 |
| bincode | 466.34 µs | 3.2819 ms | 1045784 | 374305 | 311761 |
| bitcode | 860.65 µs | 3.7466 ms | 758664 | 349057 | 299171 |
| borsh | 495.97 µs | 3.3202 ms | 885780 | 363280 | 286514 |
| bson | 3.0810 ms | 11.344 ms | 1924682 | 537661 | 376270 |
| capnp | 719.51 µs | † | 1443216 | 509618 | 428649 |
| cbor | 2.1520 ms | 6.4792 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 2.2038 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 8.6535 ms | 5.5039 ms | 818669 | 334639 | 285514 |
| postcard | 404.38 µs | 3.2226 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*3.1238 ms\**</span> <span title="encode">*516.28 µs\**</span> | 3.8130 ms | 764951 | 269811 | 227947 |
| rkyv | 302.97 µs | <span title="unvalidated">*2.4796 ms\**</span> <span title="validated upfront with error">*3.7302 ms\**</span> | 1011488 | 384478 | 333545 |
| rmp | 1.7320 ms | 4.5666 ms | 784997 | 326654 | 278219 |
| ron | 15.963 ms | 20.758 ms | 1607459 | 452648 | 349713 |
| scale | 632.49 µs | 3.2328 ms | 765778 | 312771 | 264518 |
| serde_json | 4.0192 ms | 7.9113 ms | 1827461 | 474358 | 361090 |
| simd-json | 4.2075 ms | 6.4865 ms | 1827461 | 474358 | 361090 |
| speedy | 257.58 µs | 2.7690 ms | 885780 | 363280 | 286514 |
| alkahest | 245.31 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 819.35 µs | 3.6157 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*28.654 µs\**</span> | <span title="unvalidated">*50.147 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*139.29 ns\**</span> | <span title="validated on-demand with error">*400.15 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.6890 ns\**</span> <span title="validated upfront with error">*2.1219 ms\**</span> | <span title="unvalidated">*78.812 µs\**</span> <span title="validated upfront with error">*2.2153 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.6068 ns\**</span> <span title="validated upfront with error">*882.19 µs\**</span> | <span title="unvalidated">*17.640 µs\**</span> <span title="validated upfront with error">*901.16 µs\**</span> | 19.659 µs |
| alkahest | <span title="validated on-demand with panic">*2.7213 ns\**</span> | <span title="validated on-demand with panic">*34.174 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 91.66% | <span title="unvalidated">*100.00%\**</span> | 42.50% | 53.69% | 55.00% |
| bare | 33.14% | 69.57% | 94.67% | 86.27% | 86.14% |
| bincode | 52.60% | 71.48% | 69.32% | 72.08% | 73.12% |
| bitcode | 28.50% | 62.61% | 95.56% | 77.30% | 76.19% |
| borsh | 49.46% | 70.66% | 81.84% | 74.27% | 79.56% |
| bson | 7.96% | 20.68% | 37.67% | 50.18% | 60.58% |
| capnp | 34.09% | † | 50.23% | 52.94% | 53.18% |
| cbor | 11.40% | 36.21% | 51.49% | 66.23% | 70.34% |
| flatbuffers | 11.13% | † | 56.80% | 57.41% | 58.62% |
| nachricht | 2.83% | 42.62% | 88.55% | 80.63% | 79.84% |
| postcard | 60.66% | 72.80% | 100.00% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*7.85%\**</span> <span title="encode">*47.51%\**</span> | 61.52% | 94.77% | 100.00% | 100.00% |
| rkyv | 80.97% | <span title="unvalidated">*94.61%\**</span> <span title="validated upfront with error">*62.89%\**</span> | 71.67% | 70.18% | 68.34% |
| rmp | 14.16% | 51.37% | 92.35% | 82.60% | 81.93% |
| ron | 1.54% | 11.30% | 45.10% | 59.61% | 65.18% |
| scale | 38.78% | 72.57% | 94.67% | 86.26% | 86.17% |
| serde_json | 6.10% | 29.65% | 39.67% | 56.88% | 63.13% |
| simd-json | 5.83% | 36.17% | 39.67% | 56.88% | 63.13% |
| speedy | 95.24% | 84.72% | 81.84% | 74.27% | 79.56% |
| alkahest | 100.00% | † | 69.32% | 59.33% | 58.53% |
| dlhn | 29.94% | 64.88% | 100.00% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*35.18%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.15%\**</span> | <span title="validated on-demand with error">*4.41%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*59.75%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*22.38%\**</span> <span title="validated upfront with error">*0.80%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.96%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*59.05%\**</span> | <span title="validated on-demand with panic">*51.62%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 434.19 µs | <span title="unvalidated">*434.46 µs\**</span> | 6000024 | 5380836 | 5345891 |
| bare | 6.8034 ms | 6.2354 ms | 6000003 | 5380817 | 5345900 |
| bincode | 3.7437 ms | 6.2194 ms | 6000008 | 5380823 | 5345890 |
| bitcode | 4.6187 ms | 9.3041 ms | 6000005 | 5401224 | 5360629 |
| borsh | 5.2443 ms | 6.5550 ms | 6000004 | 5380818 | 5345889 |
| bson | 46.057 ms | 129.23 ms | 23013911 | 9211138 | 7497811 |
| capnp | 7.9387 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 50.521 ms | 49.233 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 920.87 µs | † | 6000024 | 5380800 | 5345910 |
| nachricht | 211.42 ms | 36.933 ms | 8125037 | 6495174 | 6386940 |
| postcard | 587.12 µs | 1.2986 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*8.0827 ms\**</span> <span title="encode">*6.1265 ms\**</span> | 15.552 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 525.03 µs | <span title="unvalidated">*429.83 µs\**</span> <span title="validated upfront with error">*430.58 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 21.370 ms | 22.794 ms | 8125006 | 6496879 | 6391037 |
| ron | 232.58 ms | 363.33 ms | 22192885 | 9009575 | 8138755 |
| scale | 3.9097 ms | 7.6810 ms | 6000004 | 5380818 | 5345889 |
| serde_json | 108.53 ms | 91.455 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 107.31 ms | 138.91 ms | 39152823 | 16587283 | 14549214 |
| speedy | 433.18 µs | 434.76 µs | 6000004 | 5380818 | 5345889 |
| alkahest | 438.79 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 7.1287 ms | 8.3448 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.0115 ns\**</span> | <span title="unvalidated">*248.19 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*202.80 ns\**</span> | <span title="validated on-demand with error">*5.3911 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.6923 ns\**</span> <span title="validated upfront with error">*46.504 ns\**</span> | <span title="unvalidated">*100.40 µs\**</span> <span title="validated upfront with error">*100.45 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.2080 ns\**</span> <span title="validated upfront with error">*12.575 ns\**</span> | <span title="unvalidated">*37.167 µs\**</span> <span title="validated upfront with error">*37.795 µs\**</span> | 235.14 µs |
| alkahest | <span title="validated on-demand with panic">*2.7232 ns\**</span> | <span title="validated on-demand with panic">*100.40 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 99.77% | <span title="unvalidated">*98.93%\**</span> | 100.00% | 100.00% | 100.00% |
| bare | 6.37% | 6.89% | 100.00% | 100.00% | 100.00% |
| bincode | 11.57% | 6.91% | 100.00% | 100.00% | 100.00% |
| bitcode | 9.38% | 4.62% | 100.00% | 99.62% | 99.73% |
| borsh | 8.26% | 6.56% | 100.00% | 100.00% | 100.00% |
| bson | 0.94% | 0.33% | 26.07% | 58.42% | 71.30% |
| capnp | 5.46% | † | 42.86% | 79.95% | 88.35% |
| cbor | 0.86% | 0.87% | 45.72% | 71.48% | 79.09% |
| flatbuffers | 47.04% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.20% | 1.16% | 73.85% | 82.84% | 83.70% |
| postcard | 73.78% | 33.10% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*5.36%\**</span> <span title="encode">*7.07%\**</span> | 2.76% | 68.57% | 80.50% | 83.25% |
| rkyv | 82.51% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.83%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 2.03% | 1.89% | 73.85% | 82.82% | 83.65% |
| ron | 0.19% | 0.12% | 27.04% | 59.72% | 65.68% |
| scale | 11.08% | 5.60% | 100.00% | 100.00% | 100.00% |
| serde_json | 0.40% | 0.47% | 22.91% | 55.98% | 62.26% |
| simd-json | 0.40% | 0.31% | 15.32% | 32.44% | 36.74% |
| speedy | 100.00% | 98.87% | 100.00% | 100.00% | 100.00% |
| alkahest | 98.72% | † | 100.00% | 100.00% | 100.00% |
| dlhn | 6.08% | 5.15% | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*60.05%\**</span> | <span title="unvalidated">*14.98%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.60%\**</span> | <span title="validated on-demand with error">*0.69%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*44.87%\**</span> <span title="validated upfront with error">*2.60%\**</span> | <span title="unvalidated">*37.02%\**</span> <span title="validated upfront with error">*37.00%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.61%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*98.34%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*44.36%\**</span> | <span title="validated on-demand with panic">*37.02%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 239.19 µs | <span title="unvalidated">*1.8858 ms\**</span> | 1290592 | 392962 | 340430 |
| bare | 840.31 µs | 3.1770 ms | 356311 | 213270 | 198488 |
| bincode | 640.09 µs | 2.6172 ms | 569975 | 240897 | 232423 |
| bitcode | 705.70 µs | 2.8645 ms | 333471 | 225804 | 212046 |
| borsh | 579.71 µs | 2.5851 ms | 446595 | 234395 | 210008 |
| bson | 4.6273 ms | 12.352 ms | 1619653 | 506953 | 328399 |
| capnp | 613.42 µs | † | 803896 | 336655 | 280851 |
| cbor | 2.0787 ms | 6.3388 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 3.2990 ms | † | 844168 | 346957 | 294015 |
| nachricht | 8.0364 ms | 5.2080 ms | 449745 | 252743 | 231110 |
| postcard | 492.27 µs | 2.6832 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*3.9543 ms\**</span> <span title="encode">*1.3748 ms\**</span> | 4.6588 ms | 596811 | 306728 | 269310 |
| rkyv | 408.55 µs | <span title="unvalidated">*1.8156 ms\**</span> <span title="validated upfront with error">*2.6254 ms\**</span> | 596952 | 254523 | 220366 |
| rmp | 1.8573 ms | 4.1216 ms | 424533 | 245594 | 226188 |
| ron | 9.5111 ms | 21.245 ms | 1465223 | 439761 | 343338 |
| scale | 677.11 µs | 2.7152 ms | 356311 | 213188 | 198524 |
| serde_json | 4.3659 ms | 9.7565 ms | 1623191 | 472275 | 359623 |
| simd-json | 4.2932 ms | 6.2512 ms | 1663769 | 496401 | 383682 |
| speedy | 358.34 µs | 2.3096 ms | 449595 | 235136 | 210361 |
| alkahest | 271.33 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*39.402 µs\**</span> | <span title="unvalidated">*39.981 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*139.28 ns\**</span> | <span title="validated on-demand with error">*680.27 ns\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.6924 ns\**</span> <span title="validated upfront with error">*2.6282 ms\**</span> | <span title="unvalidated">*1.9844 µs\**</span> <span title="validated upfront with error">*2.6211 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.6066 ns\**</span> <span title="validated upfront with error">*749.22 µs\**</span> | <span title="unvalidated">*143.92 ns\**</span> <span title="validated upfront with error">*749.49 µs\**</span> | 1.6899 µs |
| alkahest | <span title="validated on-demand with panic">*2.7935 ns\**</span> | <span title="validated on-demand with panic">*12.198 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*96.28%\**</span> | 25.84% | 54.25% | 58.31% |
| bare | 28.46% | 57.15% | 93.59% | 99.96% | 100.00% |
| bincode | 37.37% | 69.37% | 58.51% | 88.50% | 85.40% |
| bitcode | 33.89% | 63.38% | 100.00% | 94.41% | 93.61% |
| borsh | 41.26% | 70.23% | 74.67% | 90.95% | 94.51% |
| bson | 5.17% | 14.70% | 20.59% | 42.05% | 60.44% |
| capnp | 38.99% | † | 41.48% | 63.33% | 70.67% |
| cbor | 11.51% | 28.64% | 30.05% | 61.29% | 72.30% |
| flatbuffers | 7.25% | † | 39.50% | 61.45% | 67.51% |
| nachricht | 2.98% | 34.86% | 74.15% | 84.35% | 85.88% |
| postcard | 48.59% | 67.67% | 90.74% | 95.97% | 95.73% |
| prost | <span title="populate + encode">*6.05%\**</span> <span title="encode">*17.40%\**</span> | 38.97% | 55.88% | 69.50% | 73.70% |
| rkyv | 58.55% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*69.16%\**</span> | 55.86% | 83.76% | 90.07% |
| rmp | 12.88% | 44.05% | 78.55% | 86.81% | 87.75% |
| ron | 2.51% | 8.55% | 22.76% | 48.48% | 57.81% |
| scale | 35.33% | 66.87% | 93.59% | 100.00% | 99.98% |
| serde_json | 5.48% | 18.61% | 20.54% | 45.14% | 55.19% |
| simd-json | 5.57% | 29.04% | 20.04% | 42.95% | 51.73% |
| speedy | 66.75% | 78.61% | 74.17% | 90.67% | 94.36% |
| alkahest | 88.15% | † | 49.95% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.36%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.15%\**</span> | <span title="validated on-demand with error">*21.16%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*59.67%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.25%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*57.51%\**</span> | <span title="validated on-demand with panic">*1.18%\**</span> | ‡ |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
