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

## Last updated: 2022-10-7

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 255.44 µs | <span title="unvalidated">*2.3979 ms\**</span> | 1705800 | 506923 | 403614 |
| bare | 806.95 µs | 3.3489 ms | 765778 | 312739 | 264630 |
| bincode | 456.76 µs | 3.4680 ms | 1045784 | 374305 | 311761 |
| borsh | 501.87 µs | 3.7201 ms | 885780 | 363280 | 286514 |
| bson | 2.9998 ms | 11.048 ms | 1924682 | 537661 | 376270 |
| capnp | 921.33 µs | † | 1443216 | 509618 | 428649 |
| cbor | 1.8449 ms | 6.6787 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 2.1536 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 7.0453 ms | 6.4483 ms | 818669 | 334639 | 285514 |
| postcard | 420.68 µs | 3.6124 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*3.4967 ms\**</span> <span title="encode">*567.89 µs\**</span> | 4.7094 ms | 764951 | 269811 | 227947 |
| rkyv | 377.93 µs | <span title="unvalidated">*2.6183 ms\**</span> <span title="validated upfront with error">*3.3088 ms\**</span> | 1011488 | 392809 | 331932 |
| rmp | 1.6248 ms | 4.7483 ms | 784997 | 326654 | 278219 |
| ron | 17.627 ms | 18.624 ms | 1607459 | 452648 | 349713 |
| scale | 561.22 µs | 3.5721 ms | 765778 | 312771 | 264518 |
| serde_json | 4.4827 ms | 8.9961 ms | 1827461 | 474358 | 361090 |
| simd-json | 4.2416 ms | 6.1625 ms | 1827461 | 474358 | 361090 |
| speedy | 257.22 µs | 2.8491 ms | 885780 | 363280 | 286514 |
| alkahest | 254.78 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 723.64 µs | 4.0979 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*65.974 µs\**</span> | <span title="unvalidated">*100.07 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*90.677 ns\**</span> | <span title="validated on-demand with error">*405.00 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.1104 ns\**</span> <span title="validated upfront with error">*2.0545 ms\**</span> | <span title="unvalidated">*103.17 µs\**</span> <span title="validated upfront with error">*2.4299 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3600 ns\**</span> <span title="validated upfront with error">*773.69 µs\**</span> | <span title="unvalidated">*21.258 µs\**</span> <span title="validated upfront with error">*790.87 µs\**</span> | 26.930 µs |
| alkahest | <span title="validated on-demand with panic">*2.5168 ns\**</span> | <span title="validated on-demand with panic">*36.950 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 99.74% | <span title="unvalidated">*100.00%\**</span> | 42.50% | 53.23% | 56.48% |
| bare | 31.57% | 71.60% | 94.67% | 86.27% | 86.14% |
| bincode | 55.78% | 69.14% | 69.32% | 72.08% | 73.12% |
| borsh | 50.77% | 64.46% | 81.84% | 74.27% | 79.56% |
| bson | 8.49% | 21.70% | 37.67% | 50.18% | 60.58% |
| capnp | 27.65% | † | 50.23% | 52.94% | 53.18% |
| cbor | 13.81% | 35.90% | 51.49% | 66.23% | 70.34% |
| flatbuffers | 11.83% | † | 56.80% | 57.41% | 58.62% |
| nachricht | 3.62% | 37.19% | 88.55% | 80.63% | 79.84% |
| postcard | 60.56% | 66.38% | 100.00% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*7.29%\**</span> <span title="encode">*44.86%\**</span> | 50.92% | 94.77% | 100.00% | 100.00% |
| rkyv | 67.41% | <span title="unvalidated">*91.58%\**</span> <span title="validated upfront with error">*72.47%\**</span> | 71.67% | 68.69% | 68.67% |
| rmp | 15.68% | 50.50% | 92.35% | 82.60% | 81.93% |
| ron | 1.45% | 12.88% | 45.10% | 59.61% | 65.18% |
| scale | 45.40% | 67.13% | 94.67% | 86.26% | 86.17% |
| serde_json | 5.68% | 26.65% | 39.67% | 56.88% | 63.13% |
| simd-json | 6.01% | 38.91% | 39.67% | 56.88% | 63.13% |
| speedy | 99.05% | 84.16% | 81.84% | 74.27% | 79.56% |
| alkahest | 100.00% | † | 69.32% | 59.33% | 58.53% |
| dlhn | 35.21% | 58.52% | 100.00% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*21.24%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.50%\**</span> | <span title="validated on-demand with error">*5.25%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*43.72%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.60%\**</span> <span title="validated upfront with error">*0.87%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.69%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*54.04%\**</span> | <span title="validated on-demand with panic">*57.53%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 482.07 µs | <span title="unvalidated">*464.64 µs\**</span> | 6000024 | 5380836 | 5345890 |
| bare | 7.0498 ms | 5.8930 ms | 6000003 | 5380817 | 5345900 |
| bincode | 4.9530 ms | 6.0019 ms | 6000008 | 5380823 | 5345890 |
| borsh | 6.0257 ms | 5.5200 ms | 6000004 | 5380818 | 5345889 |
| bson | 62.871 ms | 125.31 ms | 23013911 | 9211138 | 7497811 |
| capnp | 10.370 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 42.684 ms | 52.854 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 1.2233 ms | † | 6000024 | 5380800 | 5345910 |
| nachricht | 157.46 ms | 37.047 ms | 8125037 | 6495174 | 6386940 |
| postcard | 739.95 µs | 1.6409 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*13.059 ms\**</span> <span title="encode">*10.850 ms\**</span> | 18.966 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 879.35 µs | <span title="unvalidated">*501.01 µs\**</span> <span title="validated upfront with error">*499.28 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 18.045 ms | 20.696 ms | 8125006 | 6496879 | 6391037 |
| ron | 241.67 ms | 288.57 ms | 22192885 | 9009575 | 8138755 |
| scale | 6.1253 ms | 4.8124 ms | 6000004 | 5380818 | 5345889 |
| serde_json | 110.83 ms | 106.06 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 112.22 ms | 147.70 ms | 39152823 | 16587283 | 14549214 |
| speedy | 681.27 µs | 668.61 µs | 6000004 | 5380818 | 5345889 |
| alkahest | 514.31 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 6.3384 ms | 9.5106 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.8555 ns\**</span> | <span title="unvalidated">*231.41 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*160.01 ns\**</span> | <span title="validated on-demand with error">*5.5349 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.1121 ns\**</span> <span title="validated upfront with error">*51.386 ns\**</span> | <span title="unvalidated">*41.132 µs\**</span> <span title="validated upfront with error">*41.227 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3687 ns\**</span> <span title="validated upfront with error">*15.638 ns\**</span> | <span title="unvalidated">*42.897 µs\**</span> <span title="validated upfront with error">*40.850 µs\**</span> | 330.09 µs |
| alkahest | <span title="validated on-demand with panic">*2.5306 ns\**</span> | <span title="validated on-demand with panic">*82.409 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 100.00% | 100.00% |
| bare | 6.84% | 7.88% | 100.00% | 100.00% | 100.00% |
| bincode | 9.73% | 7.74% | 100.00% | 100.00% | 100.00% |
| borsh | 8.00% | 8.42% | 100.00% | 100.00% | 100.00% |
| bson | 0.77% | 0.37% | 26.07% | 58.42% | 71.30% |
| capnp | 4.65% | † | 42.86% | 79.95% | 88.35% |
| cbor | 1.13% | 0.88% | 45.72% | 71.48% | 79.09% |
| flatbuffers | 39.41% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.31% | 1.25% | 73.85% | 82.84% | 83.70% |
| postcard | 65.15% | 28.32% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*3.69%\**</span> <span title="encode">*4.44%\**</span> | 2.45% | 68.57% | 80.50% | 83.25% |
| rkyv | 54.82% | <span title="unvalidated">*92.74%\**</span> <span title="validated upfront with error">*93.06%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 2.67% | 2.25% | 73.85% | 82.82% | 83.65% |
| ron | 0.20% | 0.16% | 27.04% | 59.72% | 65.68% |
| scale | 7.87% | 9.66% | 100.00% | 100.00% | 100.00% |
| serde_json | 0.43% | 0.44% | 22.91% | 55.98% | 62.26% |
| simd-json | 0.43% | 0.31% | 15.32% | 32.44% | 36.74% |
| speedy | 70.76% | 69.49% | 100.00% | 100.00% | 100.00% |
| alkahest | 93.73% | † | 100.00% | 100.00% | 100.00% |
| dlhn | 7.61% | 4.89% | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*47.93%\**</span> | <span title="unvalidated">*17.65%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.86%\**</span> | <span title="validated on-demand with error">*0.74%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*43.98%\**</span> <span title="validated upfront with error">*2.66%\**</span> | <span title="unvalidated">*99.31%\**</span> <span title="validated upfront with error">*99.09%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*8.75%\**</span> | <span title="unvalidated">*95.23%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*54.09%\**</span> | <span title="validated on-demand with panic">*49.57%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 247.62 µs | <span title="unvalidated">*1.9754 ms\**</span> | 1290592 | 389814 | 328479 |
| bare | 826.99 µs | 3.4925 ms | 356311 | 213270 | 198488 |
| bincode | 653.95 µs | 2.8477 ms | 569975 | 240897 | 232423 |
| borsh | 595.66 µs | 2.5333 ms | 446595 | 234395 | 210008 |
| bson | 4.4305 ms | 12.284 ms | 1619653 | 506953 | 328399 |
| capnp | 724.86 µs | † | 803896 | 336655 | 280851 |
| cbor | 1.8928 ms | 6.7161 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 3.8221 ms | † | 844168 | 346957 | 294015 |
| nachricht | 6.5890 ms | 5.3495 ms | 449745 | 252743 | 231110 |
| postcard | 451.40 µs | 2.9210 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*3.9833 ms\**</span> <span title="encode">*1.3809 ms\**</span> | 5.0129 ms | 596811 | 306728 | 269310 |
| rkyv | 430.30 µs | <span title="unvalidated">*1.9963 ms\**</span> <span title="validated upfront with error">*2.5057 ms\**</span> | 596952 | 254571 | 219976 |
| rmp | 1.5688 ms | 4.0420 ms | 424533 | 245594 | 226188 |
| ron | 10.802 ms | 18.188 ms | 1465223 | 439761 | 343338 |
| scale | 649.70 µs | 2.8945 ms | 356311 | 213188 | 198524 |
| serde_json | 4.3853 ms | 9.8321 ms | 1623191 | 472275 | 359623 |
| simd-json | 4.1622 ms | 6.1122 ms | 1663769 | 496401 | 383682 |
| speedy | 374.06 µs | 2.5619 ms | 449595 | 235136 | 210361 |
| alkahest | 287.93 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*70.807 µs\**</span> | <span title="unvalidated">*72.107 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*93.556 ns\**</span> | <span title="validated on-demand with error">*635.83 ns\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.0747 ns\**</span> <span title="validated upfront with error">*2.3990 ms\**</span> | <span title="unvalidated">*3.3657 µs\**</span> <span title="validated upfront with error">*2.3220 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3783 ns\**</span> <span title="validated upfront with error">*614.23 µs\**</span> | <span title="unvalidated">*179.26 ns\**</span> <span title="validated upfront with error">*624.12 µs\**</span> | 2.1829 µs |
| alkahest | <span title="validated on-demand with panic">*2.4971 ns\**</span> | <span title="validated on-demand with panic">*36.709 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 27.61% | 54.69% | 60.43% |
| bare | 29.94% | 56.56% | 100.00% | 99.96% | 100.00% |
| bincode | 37.87% | 69.37% | 62.51% | 88.50% | 85.40% |
| borsh | 41.57% | 77.98% | 79.78% | 90.95% | 94.51% |
| bson | 5.59% | 16.08% | 22.00% | 42.05% | 60.44% |
| capnp | 34.16% | † | 44.32% | 63.33% | 70.67% |
| cbor | 13.08% | 29.41% | 32.11% | 61.29% | 72.30% |
| flatbuffers | 6.48% | † | 42.21% | 61.45% | 67.51% |
| nachricht | 3.76% | 36.93% | 79.23% | 84.35% | 85.88% |
| postcard | 54.86% | 67.63% | 96.96% | 95.97% | 95.73% |
| prost | <span title="populate + encode">*6.22%\**</span> <span title="encode">*17.93%\**</span> | 39.41% | 59.70% | 69.50% | 73.70% |
| rkyv | 57.55% | <span title="unvalidated">*98.95%\**</span> <span title="validated upfront with error">*78.84%\**</span> | 59.69% | 83.74% | 90.23% |
| rmp | 15.78% | 48.87% | 83.93% | 86.81% | 87.75% |
| ron | 2.29% | 10.86% | 24.32% | 48.48% | 57.81% |
| scale | 38.11% | 68.25% | 100.00% | 100.00% | 99.98% |
| serde_json | 5.65% | 20.09% | 21.95% | 45.14% | 55.19% |
| simd-json | 5.95% | 32.32% | 21.42% | 42.95% | 51.73% |
| speedy | 66.20% | 77.11% | 79.25% | 90.67% | 94.36% |
| alkahest | 86.00% | † | 53.37% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.25%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.47%\**</span> | <span title="validated on-demand with error">*28.19%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*44.83%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*5.33%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*55.20%\**</span> | <span title="validated on-demand with panic">*0.49%\**</span> | ‡ |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
