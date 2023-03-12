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

## Last updated: 2023-3-12

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 272.75 µs | <span title="unvalidated">*2.5038 ms\**</span> | 1705800 | 502508 | 414446 |
| bare | 792.88 µs | 3.2283 ms | 765778 | 312739 | 264630 |
| bincode | 454.15 µs | 3.2685 ms | 1045784 | 374305 | 311761 |
| borsh | 490.44 µs | 3.3022 ms | 885780 | 363280 | 286514 |
| bson | 2.9165 ms | 11.594 ms | 1924682 | 537661 | 376270 |
| capnp | 756.87 µs | † | 1443216 | 509618 | 428649 |
| cbor | 1.8518 ms | 6.7333 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 2.0908 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 6.6919 ms | 6.0471 ms | 818669 | 334639 | 285514 |
| postcard | 408.56 µs | 3.3318 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*3.2077 ms\**</span> <span title="encode">*606.96 µs\**</span> | 3.8758 ms | 764951 | 269811 | 227947 |
| rkyv | 349.26 µs | <span title="unvalidated">*2.5895 ms\**</span> <span title="validated upfront with error">*3.5071 ms\**</span> | 1011488 | 384478 | 333545 |
| rmp | 1.6215 ms | 4.4073 ms | 784997 | 326654 | 278219 |
| ron | 18.374 ms | 22.232 ms | 1607459 | 452648 | 349713 |
| scale | 582.97 µs | 3.2767 ms | 765778 | 312771 | 264518 |
| serde_json | 3.9122 ms | 9.2056 ms | 1827461 | 474358 | 361090 |
| simd-json | 4.3502 ms | 6.5077 ms | 1827461 | 474358 | 361090 |
| speedy | 314.90 µs | 2.7904 ms | 885780 | 363280 | 286514 |
| alkahest | 261.98 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 765.26 µs | 3.7853 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*38.567 µs\**</span> | <span title="unvalidated">*60.703 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*105.99 ns\**</span> | <span title="validated on-demand with error">*378.57 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2078 ns\**</span> <span title="validated upfront with error">*2.2613 ms\**</span> | <span title="unvalidated">*100.85 µs\**</span> <span title="validated upfront with error">*2.3809 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4874 ns\**</span> <span title="validated upfront with error">*1.0070 ms\**</span> | <span title="unvalidated">*17.012 µs\**</span> <span title="validated upfront with error">*986.85 µs\**</span> | 19.521 µs |
| alkahest | <span title="validated on-demand with panic">*2.7356 ns\**</span> | <span title="validated on-demand with panic">*52.580 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 96.05% | <span title="unvalidated">*100.00%\**</span> | 42.50% | 53.69% | 55.00% |
| bare | 33.04% | 77.56% | 94.67% | 86.27% | 86.14% |
| bincode | 57.69% | 76.60% | 69.32% | 72.08% | 73.12% |
| borsh | 53.42% | 75.82% | 81.84% | 74.27% | 79.56% |
| bson | 8.98% | 21.60% | 37.67% | 50.18% | 60.58% |
| capnp | 34.61% | † | 50.23% | 52.94% | 53.18% |
| cbor | 14.15% | 37.19% | 51.49% | 66.23% | 70.34% |
| flatbuffers | 12.53% | † | 56.80% | 57.41% | 58.62% |
| nachricht | 3.91% | 41.40% | 88.55% | 80.63% | 79.84% |
| postcard | 64.12% | 75.15% | 100.00% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*8.17%\**</span> <span title="encode">*43.16%\**</span> | 64.60% | 94.77% | 100.00% | 100.00% |
| rkyv | 75.01% | <span title="unvalidated">*96.69%\**</span> <span title="validated upfront with error">*71.39%\**</span> | 71.67% | 70.18% | 68.34% |
| rmp | 16.16% | 56.81% | 92.35% | 82.60% | 81.93% |
| ron | 1.43% | 11.26% | 45.10% | 59.61% | 65.18% |
| scale | 44.94% | 76.41% | 94.67% | 86.26% | 86.17% |
| serde_json | 6.70% | 27.20% | 39.67% | 56.88% | 63.13% |
| simd-json | 6.02% | 38.47% | 39.67% | 56.88% | 63.13% |
| speedy | 83.19% | 89.73% | 81.84% | 74.27% | 79.56% |
| alkahest | 100.00% | † | 69.32% | 59.33% | 58.53% |
| dlhn | 34.23% | 66.15% | 100.00% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*28.02%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.40%\**</span> | <span title="validated on-demand with error">*4.49%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.37%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.87%\**</span> <span title="validated upfront with error">*0.71%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.72%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*54.37%\**</span> | <span title="validated on-demand with panic">*32.35%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 511.18 µs | <span title="unvalidated">*509.64 µs\**</span> | 6000024 | 5380837 | 5345891 |
| bare | 5.9274 ms | 5.7132 ms | 6000003 | 5380817 | 5345900 |
| bincode | 4.4554 ms | 5.6998 ms | 6000008 | 5380823 | 5345890 |
| borsh | 5.3946 ms | 5.7872 ms | 6000004 | 5380818 | 5345889 |
| bson | 64.121 ms | 127.34 ms | 23013911 | 9211138 | 7497811 |
| capnp | 9.8149 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 46.675 ms | 55.805 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 879.31 µs | † | 6000024 | 5380800 | 5345910 |
| nachricht | 153.89 ms | 39.431 ms | 8125037 | 6495174 | 6386940 |
| postcard | 746.25 µs | 1.4454 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*9.5792 ms\**</span> <span title="encode">*7.7317 ms\**</span> | 17.622 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 600.76 µs | <span title="unvalidated">*510.41 µs\**</span> <span title="validated upfront with error">*508.78 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 16.973 ms | 20.036 ms | 8125006 | 6496879 | 6391037 |
| ron | 224.89 ms | 381.01 ms | 22192885 | 9009575 | 8138755 |
| scale | 4.8492 ms | 5.6044 ms | 6000004 | 5380818 | 5345889 |
| serde_json | 101.10 ms | 99.404 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 109.06 ms | 160.04 ms | 39152823 | 16587283 | 14549214 |
| speedy | 508.25 µs | 508.84 µs | 6000004 | 5380818 | 5345889 |
| alkahest | 508.35 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 5.8955 ms | 8.0786 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.5900 ns\**</span> | <span title="unvalidated">*270.86 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*175.03 ns\**</span> | <span title="validated on-demand with error">*5.3535 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.3138 ns\**</span> <span title="validated upfront with error">*46.180 ns\**</span> | <span title="unvalidated">*85.427 µs\**</span> <span title="validated upfront with error">*87.514 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.5406 ns\**</span> <span title="validated upfront with error">*11.572 ns\**</span> | <span title="unvalidated">*46.584 µs\**</span> <span title="validated upfront with error">*48.056 µs\**</span> | 263.58 µs |
| alkahest | <span title="validated on-demand with panic">*2.7369 ns\**</span> | <span title="validated on-demand with panic">*162.11 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 99.43% | <span title="unvalidated">*99.83%\**</span> | 100.00% | 100.00% | 100.00% |
| bare | 8.57% | 8.91% | 100.00% | 100.00% | 100.00% |
| bincode | 11.41% | 8.93% | 100.00% | 100.00% | 100.00% |
| borsh | 9.42% | 8.79% | 100.00% | 100.00% | 100.00% |
| bson | 0.79% | 0.40% | 26.07% | 58.42% | 71.30% |
| capnp | 5.18% | † | 42.86% | 79.95% | 88.35% |
| cbor | 1.09% | 0.91% | 45.72% | 71.48% | 79.09% |
| flatbuffers | 57.80% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.33% | 1.29% | 73.85% | 82.84% | 83.70% |
| postcard | 68.11% | 35.20% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*5.31%\**</span> <span title="encode">*6.57%\**</span> | 2.89% | 68.57% | 80.50% | 83.25% |
| rkyv | 84.60% | <span title="unvalidated">*99.68%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 2.99% | 2.54% | 73.85% | 82.82% | 83.65% |
| ron | 0.23% | 0.13% | 27.04% | 59.72% | 65.68% |
| scale | 10.48% | 9.08% | 100.00% | 100.00% | 100.00% |
| serde_json | 0.50% | 0.51% | 22.91% | 55.98% | 62.26% |
| simd-json | 0.47% | 0.32% | 15.32% | 32.44% | 36.74% |
| speedy | 100.00% | 99.99% | 100.00% | 100.00% | 100.00% |
| alkahest | 99.98% | † | 100.00% | 100.00% | 100.00% |
| dlhn | 8.62% | 6.30% | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*59.48%\**</span> | <span title="unvalidated">*17.20%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.88%\**</span> | <span title="validated on-demand with error">*0.87%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.49%\**</span> <span title="validated upfront with error">*3.34%\**</span> | <span title="unvalidated">*54.53%\**</span> <span title="validated upfront with error">*53.23%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*13.31%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*96.94%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*56.29%\**</span> | <span title="validated on-demand with panic">*28.74%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 255.70 µs | <span title="unvalidated">*1.9249 ms\**</span> | 1290592 | 397928 | 339063 |
| bare | 871.13 µs | 3.1827 ms | 356311 | 213270 | 198488 |
| bincode | 642.57 µs | 2.5893 ms | 569975 | 240897 | 232423 |
| borsh | 570.14 µs | 2.7127 ms | 446595 | 234395 | 210008 |
| bson | 4.3511 ms | 12.290 ms | 1619653 | 506953 | 328399 |
| capnp | 622.41 µs | † | 803896 | 336655 | 280851 |
| cbor | 1.9276 ms | 6.4994 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 3.2942 ms | † | 844168 | 346957 | 294015 |
| nachricht | 6.6536 ms | 5.1471 ms | 449745 | 252743 | 231110 |
| postcard | 459.41 µs | 2.6699 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*3.8316 ms\**</span> <span title="encode">*1.3238 ms\**</span> | 4.5505 ms | 596811 | 306728 | 269310 |
| rkyv | 413.02 µs | <span title="unvalidated">*1.8094 ms\**</span> <span title="validated upfront with error">*2.6971 ms\**</span> | 596952 | 254523 | 220366 |
| rmp | 1.7313 ms | 3.9884 ms | 424533 | 245594 | 226188 |
| ron | 9.8254 ms | 22.654 ms | 1465223 | 439761 | 343338 |
| scale | 688.02 µs | 2.7914 ms | 356311 | 213188 | 198524 |
| serde_json | 4.1702 ms | 10.033 ms | 1623191 | 472275 | 359623 |
| simd-json | 4.1968 ms | 6.3116 ms | 1663769 | 496401 | 383682 |
| speedy | 422.44 µs | 2.2893 ms | 449595 | 235136 | 210361 |
| alkahest | 298.30 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*60.515 µs\**</span> | <span title="unvalidated">*61.636 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*106.18 ns\**</span> | <span title="validated on-demand with error">*628.65 ns\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.3440 ns\**</span> <span title="validated upfront with error">*2.3031 ms\**</span> | <span title="unvalidated">*2.7482 µs\**</span> <span title="validated upfront with error">*2.2716 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.6073 ns\**</span> <span title="validated upfront with error">*835.87 µs\**</span> | <span title="unvalidated">*194.36 ns\**</span> <span title="validated upfront with error">*799.16 µs\**</span> | 1.5155 µs |
| alkahest | <span title="validated on-demand with panic">*2.9192 ns\**</span> | <span title="validated on-demand with panic">*20.155 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*94.00%\**</span> | 27.61% | 53.57% | 58.54% |
| bare | 29.35% | 56.85% | 100.00% | 99.96% | 100.00% |
| bincode | 39.79% | 69.88% | 62.51% | 88.50% | 85.40% |
| borsh | 44.85% | 66.70% | 79.78% | 90.95% | 94.51% |
| bson | 5.88% | 14.72% | 22.00% | 42.05% | 60.44% |
| capnp | 41.08% | † | 44.32% | 63.33% | 70.67% |
| cbor | 13.27% | 27.84% | 32.11% | 61.29% | 72.30% |
| flatbuffers | 7.76% | † | 42.21% | 61.45% | 67.51% |
| nachricht | 3.84% | 35.15% | 79.23% | 84.35% | 85.88% |
| postcard | 55.66% | 67.77% | 96.96% | 95.97% | 95.73% |
| prost | <span title="populate + encode">*6.67%\**</span> <span title="encode">*19.32%\**</span> | 39.76% | 59.70% | 69.50% | 73.70% |
| rkyv | 61.91% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*67.09%\**</span> | 59.69% | 83.76% | 90.07% |
| rmp | 14.77% | 45.37% | 83.93% | 86.81% | 87.75% |
| ron | 2.60% | 7.99% | 24.32% | 48.48% | 57.81% |
| scale | 37.16% | 64.82% | 100.00% | 100.00% | 99.98% |
| serde_json | 6.13% | 18.03% | 21.95% | 45.14% | 55.19% |
| simd-json | 6.09% | 28.67% | 21.42% | 42.95% | 51.73% |
| speedy | 60.53% | 79.04% | 79.25% | 90.67% | 94.36% |
| alkahest | 85.72% | † | 53.37% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.32%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.51%\**</span> | <span title="validated on-demand with error">*30.92%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*48.07%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.07%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.02%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*55.06%\**</span> | <span title="validated on-demand with panic">*0.96%\**</span> | ‡ |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
