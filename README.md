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

## Last updated: 2023-5-8

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 269.28 µs | <span title="unvalidated">*2.7737 ms\**</span> | 1705800 | 507672 | 403308 |
| bare | 833.30 µs | 3.7564 ms | 765778 | 312739 | 264630 |
| bincode | 567.60 µs | 3.7794 ms | 1045784 | 374305 | 311761 |
| bitcode | 841.14 µs | 4.2138 ms | 703664 | 320922 | 273622 |
| borsh | 601.99 µs | 4.1229 ms | 885780 | 363280 | 286514 |
| bson | 3.3557 ms | 11.901 ms | 1924682 | 537661 | 376270 |
| capnp | 833.25 µs | † | 1443216 | 509618 | 428649 |
| cbor | 2.2871 ms | 7.6305 ms | 1407835 | 407372 | 324081 |
| ciborium | 4.3776 ms | 12.279 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 2.2183 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 7.9719 ms | 5.9808 ms | 818669 | 334639 | 285514 |
| postcard | 442.39 µs | 3.7218 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*3.4261 ms\**</span> <span title="encode">*593.98 µs\**</span> | 4.2298 ms | 764951 | 269811 | 227947 |
| rkyv | 345.43 µs | <span title="unvalidated">*2.7607 ms\**</span> <span title="validated upfront with error">*3.6704 ms\**</span> | 1011488 | 384478 | 333545 |
| rmp | 1.7587 ms | 5.0754 ms | 784997 | 326654 | 278219 |
| ron | 23.134 ms | 22.832 ms | 1607459 | 452648 | 349713 |
| scale | 678.94 µs | 3.5882 ms | 765778 | 312771 | 264518 |
| serde_json | 5.1384 ms | 9.4371 ms | 1827461 | 474358 | 361090 |
| simd-json | 2.4322 ms | 6.1684 ms | 1827461 | 474358 | 361090 |
| speedy | 308.39 µs | 2.9755 ms | 885780 | 363280 | 286514 |
| alkahest | 270.46 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 884.67 µs | 4.5234 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*67.125 µs\**</span> | <span title="unvalidated">*99.690 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*118.40 ns\**</span> | <span title="validated on-demand with error">*431.43 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.3432 ns\**</span> <span title="validated upfront with error">*2.2144 ms\**</span> | <span title="unvalidated">*114.43 µs\**</span> <span title="validated upfront with error">*2.3366 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4641 ns\**</span> <span title="validated upfront with error">*883.45 µs\**</span> | <span title="unvalidated">*21.057 µs\**</span> <span title="validated upfront with error">*906.05 µs\**</span> | 37.212 µs |
| alkahest | <span title="validated on-demand with panic">*2.7870 ns\**</span> | <span title="validated on-demand with panic">*39.816 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*99.53%\**</span> | 41.25% | 53.15% | 56.52% |
| bare | 32.31% | 73.49% | 91.89% | 86.27% | 86.14% |
| bincode | 47.44% | 73.05% | 67.29% | 72.08% | 73.12% |
| bitcode | 32.01% | 65.52% | 100.00% | 84.07% | 83.31% |
| borsh | 44.73% | 66.96% | 79.44% | 74.27% | 79.56% |
| bson | 8.02% | 23.20% | 36.56% | 50.18% | 60.58% |
| capnp | 32.32% | † | 48.76% | 52.94% | 53.18% |
| cbor | 11.77% | 36.18% | 49.98% | 66.23% | 70.34% |
| ciborium | 6.15% | 22.48% | 49.98% | 66.23% | 70.34% |
| flatbuffers | 12.14% | † | 55.13% | 57.41% | 58.62% |
| nachricht | 3.38% | 46.16% | 85.95% | 80.63% | 79.84% |
| postcard | 60.87% | 74.18% | 97.06% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*7.86%\**</span> <span title="encode">*45.33%\**</span> | 65.27% | 91.99% | 100.00% | 100.00% |
| rkyv | 77.96% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*75.22%\**</span> | 69.57% | 70.18% | 68.34% |
| rmp | 15.31% | 54.39% | 89.64% | 82.60% | 81.93% |
| ron | 1.16% | 12.09% | 43.77% | 59.61% | 65.18% |
| scale | 39.66% | 76.94% | 91.89% | 86.26% | 86.17% |
| serde_json | 5.24% | 29.25% | 38.51% | 56.88% | 63.13% |
| simd-json | 11.07% | 44.76% | 38.51% | 56.88% | 63.13% |
| speedy | 87.32% | 92.78% | 79.44% | 74.27% | 79.56% |
| alkahest | 99.56% | † | 67.29% | 59.33% | 58.53% |
| dlhn | 30.44% | 61.03% | 97.06% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*21.12%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.24%\**</span> | <span title="validated on-demand with error">*4.88%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*43.79%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.40%\**</span> <span title="validated upfront with error">*0.90%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.32%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*52.53%\**</span> | <span title="validated on-demand with panic">*52.89%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 448.53 µs | <span title="unvalidated">*430.12 µs\**</span> | 6000024 | 5380836 | 5345890 |
| bare | 7.2579 ms | 6.1245 ms | 6000003 | 5380817 | 5345900 |
| bincode | 5.0009 ms | 11.021 ms | 6000008 | 5380823 | 5345890 |
| bitcode | 9.4186 ms | 15.518 ms | 4737624 | 4740613 | 4737741 |
| borsh | 5.5459 ms | 2.0819 ms | 6000004 | 5380818 | 5345889 |
| bson | 63.241 ms | 151.69 ms | 23013911 | 9211138 | 7497811 |
| capnp | 10.910 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 49.247 ms | 62.759 ms | 13122324 | 7527423 | 6759658 |
| ciborium | 94.142 ms | 111.12 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 1.2345 ms | † | 6000024 | 5380800 | 5345910 |
| nachricht | 178.70 ms | 41.478 ms | 8125037 | 6495174 | 6386940 |
| postcard | 771.86 µs | 1.9492 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*10.700 ms\**</span> <span title="encode">*7.9959 ms\**</span> | 18.981 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 819.71 µs | <span title="unvalidated">*493.37 µs\**</span> <span title="validated upfront with error">*489.37 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 23.195 ms | 25.579 ms | 8125006 | 6496879 | 6391037 |
| ron | 253.79 ms | 406.29 ms | 22192885 | 9009575 | 8138755 |
| scale | 5.1108 ms | 5.7728 ms | 6000004 | 5380818 | 5345889 |
| serde_json | 125.47 ms | 117.99 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 74.617 ms | 131.55 ms | 26192883 | 9612105 | 8586741 |
| speedy | 442.77 µs | 440.44 µs | 6000004 | 5380818 | 5345889 |
| alkahest | 510.81 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 5.4768 ms | 10.522 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.9779 ns\**</span> | <span title="unvalidated">*247.03 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*199.19 ns\**</span> | <span title="validated on-demand with error">*5.9992 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.3399 ns\**</span> <span title="validated upfront with error">*50.300 ns\**</span> | <span title="unvalidated">*90.244 µs\**</span> <span title="validated upfront with error">*90.110 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4427 ns\**</span> <span title="validated upfront with error">*13.740 ns\**</span> | <span title="unvalidated">*45.591 µs\**</span> <span title="validated upfront with error">*44.965 µs\**</span> | 307.30 µs |
| alkahest | <span title="validated on-demand with panic">*2.9242 ns\**</span> | <span title="validated on-demand with panic">*95.106 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 98.72% | <span title="unvalidated">*100.00%\**</span> | 78.96% | 88.10% | 88.62% |
| bare | 6.10% | 7.02% | 78.96% | 88.10% | 88.62% |
| bincode | 8.85% | 3.90% | 78.96% | 88.10% | 88.62% |
| bitcode | 4.70% | 2.77% | 100.00% | 100.00% | 100.00% |
| borsh | 7.98% | 20.66% | 78.96% | 88.10% | 88.62% |
| bson | 0.70% | 0.28% | 20.59% | 51.47% | 63.19% |
| capnp | 4.06% | † | 33.84% | 70.44% | 78.30% |
| cbor | 0.90% | 0.69% | 36.10% | 62.98% | 70.09% |
| ciborium | 0.47% | 0.39% | 36.10% | 62.98% | 70.09% |
| flatbuffers | 35.87% | † | 78.96% | 88.10% | 88.62% |
| nachricht | 0.25% | 1.04% | 58.31% | 72.99% | 74.18% |
| postcard | 57.36% | 22.07% | 78.96% | 88.10% | 88.62% |
| prost | <span title="populate + encode">*4.14%\**</span> <span title="encode">*5.54%\**</span> | 2.27% | 54.14% | 70.93% | 73.78% |
| rkyv | 54.02% | <span title="unvalidated">*87.18%\**</span> <span title="validated upfront with error">*87.89%\**</span> | 78.96% | 88.10% | 88.62% |
| rmp | 1.91% | 1.68% | 58.31% | 72.97% | 74.13% |
| ron | 0.17% | 0.11% | 21.35% | 52.62% | 58.21% |
| scale | 8.66% | 7.45% | 78.96% | 88.10% | 88.62% |
| serde_json | 0.35% | 0.36% | 18.09% | 49.32% | 55.18% |
| simd-json | 0.59% | 0.33% | 18.09% | 49.32% | 55.18% |
| speedy | 100.00% | 97.66% | 78.96% | 88.10% | 88.62% |
| alkahest | 86.68% | † | 78.96% | 88.10% | 88.62% |
| dlhn | 8.08% | 4.09% | 78.96% | 88.10% | 88.62% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*48.45%\**</span> | <span title="unvalidated">*18.20%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.72%\**</span> | <span title="validated on-demand with error">*0.75%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*43.20%\**</span> <span title="validated upfront with error">*2.87%\**</span> | <span title="unvalidated">*49.83%\**</span> <span title="validated upfront with error">*49.90%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.50%\**</span> | <span title="unvalidated">*98.63%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*49.34%\**</span> | <span title="validated on-demand with panic">*47.28%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 269.71 µs | <span title="unvalidated">*2.2475 ms\**</span> | 1290592 | 390871 | 330750 |
| bare | 934.67 µs | 3.7192 ms | 356311 | 213270 | 198488 |
| bincode | 733.10 µs | 2.9952 ms | 569975 | 240897 | 232423 |
| bitcode | 732.60 µs | 3.3287 ms | 323111 | 215477 | 201612 |
| borsh | 670.34 µs | 3.3998 ms | 446595 | 234395 | 210008 |
| bson | 5.2185 ms | 13.577 ms | 1619653 | 506953 | 328399 |
| capnp | 696.02 µs | † | 803896 | 336655 | 280851 |
| cbor | 2.3705 ms | 7.6389 ms | 1109821 | 347812 | 274526 |
| ciborium | 4.0960 ms | 11.469 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 4.3606 ms | † | 844168 | 346957 | 294015 |
| nachricht | 7.7891 ms | 6.1840 ms | 449745 | 252743 | 231110 |
| postcard | 549.98 µs | 3.0992 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*4.1673 ms\**</span> <span title="encode">*1.4782 ms\**</span> | 5.5262 ms | 596811 | 306728 | 269310 |
| rkyv | 461.44 µs | <span title="unvalidated">*2.2969 ms\**</span> <span title="validated upfront with error">*2.9231 ms\**</span> | 596952 | 254139 | 220706 |
| rmp | 1.9834 ms | 4.6801 ms | 424533 | 245594 | 226188 |
| ron | 12.520 ms | 25.054 ms | 1465223 | 439761 | 343338 |
| scale | 741.99 µs | 3.0309 ms | 356311 | 213188 | 198524 |
| serde_json | 5.2479 ms | 11.570 ms | 1623191 | 472275 | 359623 |
| simd-json | 2.9547 ms | 6.4533 ms | 1623191 | 472275 | 359623 |
| speedy | 396.08 µs | 2.6184 ms | 449595 | 235136 | 210361 |
| alkahest | 312.89 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*77.092 µs\**</span> | <span title="unvalidated">*78.934 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*127.24 ns\**</span> | <span title="validated on-demand with error">*747.31 ns\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.3404 ns\**</span> <span title="validated upfront with error">*2.5856 ms\**</span> | <span title="unvalidated">*3.1928 µs\**</span> <span title="validated upfront with error">*2.5805 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.6387 ns\**</span> <span title="validated upfront with error">*825.75 µs\**</span> | <span title="unvalidated">*215.06 ns\**</span> <span title="validated upfront with error">*792.90 µs\**</span> | 2.7241 µs |
| alkahest | <span title="validated on-demand with panic">*2.8564 ns\**</span> | <span title="validated on-demand with panic">*27.186 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 25.04% | 54.54% | 60.01% |
| bare | 28.86% | 60.43% | 90.68% | 99.96% | 100.00% |
| bincode | 36.79% | 75.04% | 56.69% | 88.50% | 85.40% |
| bitcode | 36.82% | 67.52% | 100.00% | 98.94% | 98.45% |
| borsh | 40.23% | 66.11% | 72.35% | 90.95% | 94.51% |
| bson | 5.17% | 16.55% | 19.95% | 42.05% | 60.44% |
| capnp | 38.75% | † | 40.19% | 63.33% | 70.67% |
| cbor | 11.38% | 29.42% | 29.11% | 61.29% | 72.30% |
| ciborium | 6.58% | 19.60% | 29.11% | 61.29% | 72.30% |
| flatbuffers | 6.19% | † | 38.28% | 61.45% | 67.51% |
| nachricht | 3.46% | 36.34% | 71.84% | 84.35% | 85.88% |
| postcard | 49.04% | 72.52% | 87.92% | 95.97% | 95.73% |
| prost | <span title="populate + encode">*6.47%\**</span> <span title="encode">*18.25%\**</span> | 40.67% | 54.14% | 69.50% | 73.70% |
| rkyv | 58.45% | <span title="unvalidated">*97.85%\**</span> <span title="validated upfront with error">*76.89%\**</span> | 54.13% | 83.89% | 89.93% |
| rmp | 13.60% | 48.02% | 76.11% | 86.81% | 87.75% |
| ron | 2.15% | 8.97% | 22.05% | 48.48% | 57.81% |
| scale | 36.35% | 74.15% | 90.68% | 100.00% | 99.98% |
| serde_json | 5.14% | 19.43% | 19.91% | 45.14% | 55.19% |
| simd-json | 9.13% | 34.83% | 19.91% | 45.14% | 55.19% |
| speedy | 68.09% | 85.83% | 71.87% | 90.67% | 94.36% |
| alkahest | 86.20% | † | 48.40% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.27%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.29%\**</span> | <span title="validated on-demand with error">*28.78%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*49.06%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.74%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*57.37%\**</span> | <span title="validated on-demand with panic">*0.79%\**</span> | ‡ |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
