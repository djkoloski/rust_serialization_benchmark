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

## Last updated: 2023-5-4

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 247.86 µs | <span title="unvalidated">*2.4141 ms\**</span> | 1705800 | 507671 | 403250 |
| bare | 689.23 µs | 3.3950 ms | 765778 | 312739 | 264630 |
| bincode | 457.50 µs | 3.2692 ms | 1045784 | 374305 | 311761 |
| bitcode | 776.61 µs | 3.6602 ms | 703664 | 320922 | 273622 |
| borsh | 472.09 µs | 3.7021 ms | 885780 | 363280 | 286514 |
| bson | 2.8525 ms | 10.624 ms | 1924682 | 537661 | 376270 |
| capnp | 739.15 µs | † | 1443216 | 509618 | 428649 |
| cbor | 2.0562 ms | 6.5459 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 1.8822 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 8.2217 ms | 5.5604 ms | 818669 | 334639 | 285514 |
| postcard | 391.43 µs | 3.3801 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*3.2141 ms\**</span> <span title="encode">*590.71 µs\**</span> | 3.8492 ms | 764951 | 269811 | 227947 |
| rkyv | 303.13 µs | <span title="unvalidated">*2.4788 ms\**</span> <span title="validated upfront with error">*3.3405 ms\**</span> | 1011488 | 384478 | 333545 |
| rmp | 1.6336 ms | 4.5078 ms | 784997 | 326654 | 278219 |
| ron | 20.387 ms | 20.954 ms | 1607459 | 452648 | 349713 |
| scale | 604.87 µs | 3.3837 ms | 765778 | 312771 | 264518 |
| serde_json | 4.1212 ms | 8.5959 ms | 1827461 | 474358 | 361090 |
| simd-json | 2.2449 ms | 5.8901 ms | 1827461 | 474358 | 361090 |
| speedy | 291.63 µs | 2.8578 ms | 885780 | 363280 | 286514 |
| alkahest | 249.12 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 775.68 µs | 3.8049 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*36.897 µs\**</span> | <span title="unvalidated">*55.689 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*108.10 ns\**</span> | <span title="validated on-demand with error">*384.19 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2711 ns\**</span> <span title="validated upfront with error">*1.9882 ms\**</span> | <span title="unvalidated">*96.438 µs\**</span> <span title="validated upfront with error">*2.0852 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.5231 ns\**</span> <span title="validated upfront with error">*848.04 µs\**</span> | <span title="unvalidated">*16.355 µs\**</span> <span title="validated upfront with error">*865.52 µs\**</span> | 23.445 µs |
| alkahest | <span title="validated on-demand with panic">*2.7687 ns\**</span> | <span title="validated on-demand with panic">*56.918 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 53.15% | 56.53% |
| bare | 35.96% | 71.11% | 91.89% | 86.27% | 86.14% |
| bincode | 54.18% | 73.84% | 67.29% | 72.08% | 73.12% |
| bitcode | 31.92% | 65.96% | 100.00% | 84.07% | 83.31% |
| borsh | 52.50% | 65.21% | 79.44% | 74.27% | 79.56% |
| bson | 8.69% | 22.72% | 36.56% | 50.18% | 60.58% |
| capnp | 33.53% | † | 48.76% | 52.94% | 53.18% |
| cbor | 12.05% | 36.88% | 49.98% | 66.23% | 70.34% |
| flatbuffers | 13.17% | † | 55.13% | 57.41% | 58.62% |
| nachricht | 3.01% | 43.42% | 85.95% | 80.63% | 79.84% |
| postcard | 63.32% | 71.42% | 97.06% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*7.71%\**</span> <span title="encode">*41.96%\**</span> | 62.72% | 91.99% | 100.00% | 100.00% |
| rkyv | 81.77% | <span title="unvalidated">*97.39%\**</span> <span title="validated upfront with error">*72.27%\**</span> | 69.57% | 70.18% | 68.34% |
| rmp | 15.17% | 53.55% | 89.64% | 82.60% | 81.93% |
| ron | 1.22% | 11.52% | 43.77% | 59.61% | 65.18% |
| scale | 40.98% | 71.34% | 91.89% | 86.26% | 86.17% |
| serde_json | 6.01% | 28.08% | 38.51% | 56.88% | 63.13% |
| simd-json | 11.04% | 40.99% | 38.51% | 56.88% | 63.13% |
| speedy | 84.99% | 84.47% | 79.44% | 74.27% | 79.56% |
| alkahest | 99.49% | † | 67.29% | 59.33% | 58.53% |
| dlhn | 31.95% | 63.45% | 97.06% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*29.37%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*4.26%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.56%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.96%\**</span> <span title="validated upfront with error">*0.78%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.89%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*55.01%\**</span> | <span title="validated on-demand with panic">*28.73%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 482.06 µs | <span title="unvalidated">*474.52 µs\**</span> | 6000024 | 5380836 | 5345890 |
| bare | 6.9841 ms | 5.5883 ms | 6000003 | 5380817 | 5345900 |
| bincode | 4.4180 ms | 5.2250 ms | 6000008 | 5380823 | 5345890 |
| bitcode | 7.8367 ms | 12.277 ms | 4737624 | 4740613 | 4737741 |
| borsh | 4.7277 ms | 1.9509 ms | 6000004 | 5380818 | 5345889 |
| bson | 53.544 ms | 121.48 ms | 23013911 | 9211138 | 7497811 |
| capnp | 9.9016 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 45.970 ms | 53.968 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 977.72 µs | † | 6000024 | 5380800 | 5345910 |
| nachricht | 198.85 ms | 35.415 ms | 8125037 | 6495174 | 6386940 |
| postcard | 883.92 µs | 1.4591 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*8.6115 ms\**</span> <span title="encode">*6.1947 ms\**</span> | 18.447 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 501.76 µs | <span title="unvalidated">*471.80 µs\**</span> <span title="validated upfront with error">*472.50 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 19.929 ms | 23.260 ms | 8125006 | 6496879 | 6391037 |
| ron | 223.10 ms | 363.19 ms | 22192885 | 9009575 | 8138755 |
| scale | 5.1880 ms | 6.0104 ms | 6000004 | 5380818 | 5345889 |
| serde_json | 106.32 ms | 95.447 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 67.324 ms | 119.80 ms | 26192883 | 9612105 | 8586741 |
| speedy | 473.02 µs | 473.76 µs | 6000004 | 5380818 | 5345889 |
| alkahest | 477.05 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 4.5995 ms | 8.3438 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.4776 ns\**</span> | <span title="unvalidated">*253.91 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*182.38 ns\**</span> | <span title="validated on-demand with error">*5.5665 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2712 ns\**</span> <span title="validated upfront with error">*44.151 ns\**</span> | <span title="unvalidated">*83.712 µs\**</span> <span title="validated upfront with error">*83.791 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.5234 ns\**</span> <span title="validated upfront with error">*13.281 ns\**</span> | <span title="unvalidated">*47.132 µs\**</span> <span title="validated upfront with error">*47.161 µs\**</span> | 238.89 µs |
| alkahest | <span title="validated on-demand with panic">*2.7690 ns\**</span> | <span title="validated on-demand with panic">*167.45 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 98.12% | <span title="unvalidated">*99.43%\**</span> | 78.96% | 88.10% | 88.62% |
| bare | 6.77% | 8.44% | 78.96% | 88.10% | 88.62% |
| bincode | 10.71% | 9.03% | 78.96% | 88.10% | 88.62% |
| bitcode | 6.04% | 3.84% | 100.00% | 100.00% | 100.00% |
| borsh | 10.01% | 24.18% | 78.96% | 88.10% | 88.62% |
| bson | 0.88% | 0.39% | 20.59% | 51.47% | 63.19% |
| capnp | 4.78% | † | 33.84% | 70.44% | 78.30% |
| cbor | 1.03% | 0.87% | 36.10% | 62.98% | 70.09% |
| flatbuffers | 48.38% | † | 78.96% | 88.10% | 88.62% |
| nachricht | 0.24% | 1.33% | 58.31% | 72.99% | 74.18% |
| postcard | 53.51% | 32.34% | 78.96% | 88.10% | 88.62% |
| prost | <span title="populate + encode">*5.49%\**</span> <span title="encode">*7.64%\**</span> | 2.56% | 54.14% | 70.93% | 73.78% |
| rkyv | 94.27% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.85%\**</span> | 78.96% | 88.10% | 88.62% |
| rmp | 2.37% | 2.03% | 58.31% | 72.97% | 74.13% |
| ron | 0.21% | 0.13% | 21.35% | 52.62% | 58.21% |
| scale | 9.12% | 7.85% | 78.96% | 88.10% | 88.62% |
| serde_json | 0.44% | 0.49% | 18.09% | 49.32% | 55.18% |
| simd-json | 0.70% | 0.39% | 18.09% | 49.32% | 55.18% |
| speedy | 100.00% | 99.59% | 78.96% | 88.10% | 88.62% |
| alkahest | 99.16% | † | 78.96% | 88.10% | 88.62% |
| dlhn | 10.28% | 5.65% | 78.96% | 88.10% | 88.62% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*61.49%\**</span> | <span title="unvalidated">*18.56%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.84%\**</span> | <span title="validated on-demand with error">*0.85%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.57%\**</span> <span title="validated upfront with error">*3.45%\**</span> | <span title="unvalidated">*56.30%\**</span> <span title="validated upfront with error">*56.25%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*11.47%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.94%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*55.02%\**</span> | <span title="validated on-demand with panic">*28.15%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 240.88 µs | <span title="unvalidated">*1.9119 ms\**</span> | 1290592 | 390931 | 330440 |
| bare | 859.24 µs | 3.1768 ms | 356311 | 213270 | 198488 |
| bincode | 609.63 µs | 2.5691 ms | 569975 | 240897 | 232423 |
| bitcode | 680.65 µs | 2.8822 ms | 323111 | 215477 | 201612 |
| borsh | 564.65 µs | 2.9307 ms | 446595 | 234395 | 210008 |
| bson | 4.2623 ms | 11.639 ms | 1619653 | 506953 | 328399 |
| capnp | 625.16 µs | † | 803896 | 336655 | 280851 |
| cbor | 2.0753 ms | 6.5377 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 3.2829 ms | † | 844168 | 346957 | 294015 |
| nachricht | 8.1181 ms | 5.0878 ms | 449745 | 252743 | 231110 |
| postcard | 496.25 µs | 2.7074 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*3.6878 ms\**</span> <span title="encode">*1.2420 ms\**</span> | 4.5540 ms | 596811 | 306728 | 269310 |
| rkyv | 408.32 µs | <span title="unvalidated">*1.8302 ms\**</span> <span title="validated upfront with error">*2.5808 ms\**</span> | 596952 | 254139 | 220706 |
| rmp | 1.7387 ms | 3.9854 ms | 424533 | 245594 | 226188 |
| ron | 10.490 ms | 21.368 ms | 1465223 | 439761 | 343338 |
| scale | 646.02 µs | 2.6529 ms | 356311 | 213188 | 198524 |
| serde_json | 4.2214 ms | 9.9847 ms | 1623191 | 472275 | 359623 |
| simd-json | 2.5047 ms | 5.7094 ms | 1623191 | 472275 | 359623 |
| speedy | 384.90 µs | 2.3259 ms | 449595 | 235136 | 210361 |
| alkahest | 293.33 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*58.984 µs\**</span> | <span title="unvalidated">*60.712 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*108.14 ns\**</span> | <span title="validated on-demand with error">*648.81 ns\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2712 ns\**</span> <span title="validated upfront with error">*2.2832 ms\**</span> | <span title="unvalidated">*2.5551 µs\**</span> <span title="validated upfront with error">*2.2967 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.5230 ns\**</span> <span title="validated upfront with error">*717.28 µs\**</span> | <span title="unvalidated">*189.41 ns\**</span> <span title="validated upfront with error">*717.73 µs\**</span> | 1.5494 µs |
| alkahest | <span title="validated on-demand with panic">*2.7687 ns\**</span> | <span title="validated on-demand with panic">*14.167 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*95.73%\**</span> | 25.04% | 54.53% | 60.07% |
| bare | 28.03% | 57.61% | 90.68% | 99.96% | 100.00% |
| bincode | 39.51% | 71.24% | 56.69% | 88.50% | 85.40% |
| bitcode | 35.39% | 63.50% | 100.00% | 98.94% | 98.45% |
| borsh | 42.66% | 62.45% | 72.35% | 90.95% | 94.51% |
| bson | 5.65% | 15.72% | 19.95% | 42.05% | 60.44% |
| capnp | 38.53% | † | 40.19% | 63.33% | 70.67% |
| cbor | 11.61% | 27.99% | 29.11% | 61.29% | 72.30% |
| flatbuffers | 7.34% | † | 38.28% | 61.45% | 67.51% |
| nachricht | 2.97% | 35.97% | 71.84% | 84.35% | 85.88% |
| postcard | 48.54% | 67.60% | 87.92% | 95.97% | 95.73% |
| prost | <span title="populate + encode">*6.53%\**</span> <span title="encode">*19.39%\**</span> | 40.19% | 54.14% | 69.50% | 73.70% |
| rkyv | 58.99% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*70.92%\**</span> | 54.13% | 83.89% | 89.93% |
| rmp | 13.85% | 45.92% | 76.11% | 86.81% | 87.75% |
| ron | 2.30% | 8.57% | 22.05% | 48.48% | 57.81% |
| scale | 37.29% | 68.99% | 90.68% | 100.00% | 99.98% |
| serde_json | 5.71% | 18.33% | 19.91% | 45.14% | 55.19% |
| simd-json | 9.62% | 32.06% | 19.91% | 45.14% | 55.19% |
| speedy | 62.58% | 78.69% | 71.87% | 90.67% | 94.36% |
| alkahest | 82.12% | † | 48.40% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.31%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*29.19%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.56%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.41%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*55.01%\**</span> | <span title="validated on-demand with panic">*1.34%\**</span> | ‡ |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
