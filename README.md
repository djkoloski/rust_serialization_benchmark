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

## Last updated: 9/7/2021

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 243.31 us | <span title="unvalidated">*3.0194 ms\**</span> | 1705800 | 513562 | 405177 |
| bincode | 503.86 us | 3.6906 ms | 1045784 | 374305 | 312658 |
| borsh | 494.03 us | 3.6864 ms | 885780 | 363280 | 286948 |
| capnp | 1.4638 ms | † | 1443216 | 509618 | 429784 |
| cbor | 1.8176 ms | 8.0207 ms | 1407835 | 407372 | 325016 |
| flatbuffers | 2.5609 ms | † | 1276368 | 469962 | 389885 |
| nachricht | 4.4270 ms | 6.6325 ms | 926221 | 365209 | 304177 |
| postcard | 655.47 us | 4.1709 ms | 765778 | 312739 | 265462 |
| prost | <span title="populate + encode">*4.2579 ms\**</span> <span title="encode">*803.85 us\**</span> | 9.3070 ms | 764951 | 269811 | 228807 |
| rkyv | 273.98 us | <span title="unvalidated">*2.6621 ms\**</span> <span title="validated upfront with error">*3.3296 ms\**</span> | 1011488 | 392809 | 332689 |
| rmp | 1.8891 ms | 4.8509 ms | 784997 | 326654 | 278933 |
| ron | 19.973 ms | 15.970 ms | 1607459 | 452648 | 351604 |
| serde_json | 4.1749 ms | 9.0393 ms | 1827461 | 474358 | 364177 |
| simd-json | 3.9858 ms | 8.7330 ms | 1827461 | 474358 | 364177 |
| speedy | 244.39 us | 3.1232 ms | 885780 | 363280 | 286948 |
| alkahest | 230.29 us | † | 1045784 | 454748 | 389910 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*50.232 us\**</span> | <span title="unvalidated">*76.207 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*161.94 ns\**</span> | <span title="validated on-demand with error">*578.83 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.5782 ns\**</span> <span title="validated upfront with error">*2.1154 ms\**</span> | <span title="unvalidated">*140.18 us\**</span> <span title="validated upfront with error">*2.2021 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.2615 ns\**</span> <span title="validated upfront with error">*661.92 us\**</span> | <span title="unvalidated">*15.939 us\**</span> <span title="validated upfront with error">*676.06 us\**</span> | 55.117 us |
| alkahest | <span title="validated on-demand with panic">*2.7880 ns\**</span> | <span title="validated on-demand with panic">*73.598 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 94.65% | <span title="unvalidated">*88.17%\**</span> | 44.84% | 52.54% | 56.47% |
| bincode | 45.71% | 72.13% | 73.15% | 72.08% | 73.18% |
| borsh | 46.61% | 72.21% | 86.36% | 74.27% | 79.74% |
| capnp | 15.73% | † | 53.00% | 52.94% | 53.24% |
| cbor | 12.67% | 33.19% | 54.34% | 66.23% | 70.40% |
| flatbuffers | 8.99% | † | 59.93% | 57.41% | 58.69% |
| nachricht | 5.20% | 40.14% | 82.59% | 73.88% | 75.22% |
| postcard | 35.13% | 63.83% | 99.89% | 86.27% | 86.19% |
| prost | <span title="populate + encode">*5.41%\**</span> <span title="encode">*28.65%\**</span> | 28.60% | 100.00% | 100.00% | 100.00% |
| rkyv | 84.05% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.95%\**</span> | 75.63% | 68.69% | 68.78% |
| rmp | 12.19% | 54.88% | 97.45% | 82.60% | 82.03% |
| ron | 1.15% | 16.67% | 47.59% | 59.61% | 65.08% |
| serde_json | 5.52% | 29.45% | 41.86% | 56.88% | 62.83% |
| simd-json | 5.78% | 30.48% | 41.86% | 56.88% | 62.83% |
| speedy | 94.23% | 85.24% | 86.36% | 74.27% | 79.74% |
| alkahest | 100.00% | † | 73.15% | 59.33% | 58.68% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*20.92%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.78%\**</span> | <span title="validated on-demand with error">*2.75%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*48.93%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.37%\**</span> <span title="validated upfront with error">*0.72%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.36%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*45.25%\**</span> | <span title="validated on-demand with panic">*21.66%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 354.66 us | <span title="unvalidated">*1.6373 ms\**</span> | 6000024 | 5380837 | 5345893 |
| bincode | 4.4211 ms | 7.2785 ms | 6000008 | 5380823 | 5345885 |
| borsh | 5.5438 ms | 8.6707 ms | 6000004 | 5380818 | 5345888 |
| capnp | 21.868 ms | † | 14000088 | 6729881 | 6051413 |
| cbor | 40.161 ms | 65.278 ms | 13122324 | 7527423 | 6761276 |
| flatbuffers | 2.3313 ms | † | 6000024 | 5380800 | 5345925 |
| nachricht | 68.632 ms | 35.435 ms | 10125030 | 7160144 | 6777986 |
| postcard | 5.8628 ms | 10.125 ms | 6000003 | 5380817 | 5345893 |
| prost | <span title="populate + encode">*21.149 ms\**</span> <span title="encode">*15.479 ms\**</span> | 17.809 ms | 8750000 | 6683814 | 6421721 |
| rkyv | 727.16 us | <span title="unvalidated">*1.7844 ms\**</span> <span title="validated upfront with error">*1.7985 ms\**</span> | 6000008 | 5380822 | 5345887 |
| rmp | 18.726 ms | 20.421 ms | 8125006 | 6496879 | 6391025 |
| ron | 206.91 ms | 252.57 ms | 22192885 | 9009575 | 8147524 |
| serde_json | 96.574 ms | 87.405 ms | 26192883 | 9612105 | 8595089 |
| simd-json | 98.614 ms | 120.95 ms | 39152823 | 16587283 | 14555990 |
| speedy | 654.29 us | 2.1691 ms | 6000004 | 5380818 | 5345888 |
| alkahest | 625.40 us | † | 6000008 | 5380823 | 5345885 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.4215 ns\**</span> | <span title="unvalidated">*176.41 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*260.34 ns\**</span> | <span title="validated on-demand with error">*7.9093 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.6026 ns\**</span> <span title="validated upfront with error">*42.460 ns\**</span> | <span title="unvalidated">*174.82 us\**</span> <span title="validated upfront with error">*176.86 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.2620 ns\**</span> <span title="validated upfront with error">*14.071 ns\**</span> | <span title="unvalidated">*34.963 us\**</span> <span title="validated upfront with error">*34.889 us\**</span> | 528.13 us |
| alkahest | <span title="validated on-demand with panic">*2.7900 ns\**</span> | <span title="validated on-demand with panic">*69.741 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 100.00% | 100.00% |
| bincode | 8.02% | 22.50% | 100.00% | 100.00% | 100.00% |
| borsh | 6.40% | 18.88% | 100.00% | 100.00% | 100.00% |
| capnp | 1.62% | † | 42.86% | 79.95% | 88.34% |
| cbor | 0.88% | 2.51% | 45.72% | 71.48% | 79.07% |
| flatbuffers | 15.21% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.52% | 4.62% | 59.26% | 75.15% | 78.87% |
| postcard | 6.05% | 16.17% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*1.68%\**</span> <span title="encode">*2.29%\**</span> | 9.19% | 68.57% | 80.50% | 83.25% |
| rkyv | 48.77% | <span title="unvalidated">*91.76%\**</span> <span title="validated upfront with error">*91.04%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 1.89% | 8.02% | 73.85% | 82.82% | 83.65% |
| ron | 0.17% | 0.65% | 27.04% | 59.72% | 65.61% |
| serde_json | 0.37% | 1.87% | 22.91% | 55.98% | 62.20% |
| simd-json | 0.36% | 1.35% | 15.32% | 32.44% | 36.73% |
| speedy | 54.21% | 75.48% | 100.00% | 100.00% | 100.00% |
| alkahest | 56.71% | † | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*52.12%\**</span> | <span title="unvalidated">*19.78%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.48%\**</span> | <span title="validated on-demand with error">*0.44%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*48.49%\**</span> <span title="validated upfront with error">*2.97%\**</span> | <span title="unvalidated">*19.96%\**</span> <span title="validated upfront with error">*19.73%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*8.97%\**</span> | <span title="unvalidated">*99.79%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*45.23%\**</span> | <span title="validated on-demand with panic">*50.03%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 318.48 us | <span title="unvalidated">*2.3256 ms\**</span> | 1290592 | 391039 | 329656 |
| bincode | 648.73 us | 3.0267 ms | 569975 | 240897 | 232904 |
| borsh | 654.99 us | 2.9264 ms | 446595 | 234395 | 210623 |
| capnp | 839.61 us | † | 803896 | 336655 | 281557 |
| cbor | 1.8588 ms | 7.4703 ms | 1109821 | 347562 | 275184 |
| flatbuffers | 32.824 ms | † | 844168 | 346957 | 295321 |
| nachricht | 5.0638 ms | 6.0554 ms | 535881 | 281994 | 250096 |
| postcard | 693.21 us | 3.5504 ms | 356311 | 213270 | 199012 |
| prost | <span title="populate + encode">*4.5773 ms\**</span> <span title="encode">*1.7028 ms\**</span> | 8.2999 ms | 596811 | 306728 | 270153 |
| rkyv | 464.84 us | <span title="unvalidated">*2.1732 ms\**</span> <span title="validated upfront with error">*2.7405 ms\**</span> | 596952 | 254571 | 220523 |
| rmp | 1.6456 ms | 4.0537 ms | 418025 | 244771 | 225969 |
| ron | 9.8878 ms | 16.211 ms | 1465229 | 439673 | 345438 |
| serde_json | 4.2674 ms | 10.021 ms | 1623197 | 472162 | 361701 |
| simd-json | 4.2821 ms | 7.9348 ms | 1663775 | 496275 | 385917 |
| speedy | 395.82 us | 2.4622 ms | 449595 | 235136 | 210904 |
| alkahest | 356.36 us | † | 667570 | 325536 | 320888 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*53.965 us\**</span> | <span title="unvalidated">*56.694 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*161.56 ns\**</span> | <span title="validated on-demand with error">*5.6220 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.5715 ns\**</span> <span title="validated upfront with error">*2.6568 ms\**</span> | <span title="unvalidated">*4.4308 us\**</span> <span title="validated upfront with error">*2.5172 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.2619 ns\**</span> <span title="validated upfront with error">*532.21 us\**</span> | <span title="unvalidated">*147.27 ns\**</span> <span title="validated upfront with error">*533.86 us\**</span> | 4.7133 us |
| alkahest | <span title="validated on-demand with panic">*2.7896 ns\**</span> | <span title="validated on-demand with panic">*29.070 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*93.45%\**</span> | 27.61% | 54.54% | 60.37% |
| bincode | 49.09% | 71.80% | 62.51% | 88.53% | 85.45% |
| borsh | 48.62% | 74.26% | 79.78% | 90.99% | 94.49% |
| capnp | 37.93% | † | 44.32% | 63.35% | 70.68% |
| cbor | 17.13% | 29.09% | 32.11% | 61.36% | 72.32% |
| flatbuffers | 0.97% | † | 42.21% | 61.47% | 67.39% |
| nachricht | 6.29% | 35.89% | 66.49% | 75.63% | 79.57% |
| postcard | 45.94% | 61.21% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*6.96%\**</span> <span title="encode">*18.70%\**</span> | 26.18% | 59.70% | 69.53% | 73.67% |
| rkyv | 68.51% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.30%\**</span> | 59.69% | 83.78% | 90.25% |
| rmp | 19.35% | 53.61% | 85.24% | 87.13% | 88.07% |
| ron | 3.22% | 13.41% | 24.32% | 48.51% | 57.61% |
| serde_json | 7.46% | 21.69% | 21.95% | 45.17% | 55.02% |
| simd-json | 7.44% | 27.39% | 21.42% | 42.97% | 51.57% |
| speedy | 80.46% | 88.26% | 79.25% | 90.70% | 94.36% |
| alkahest | 89.37% | † | 53.37% | 65.51% | 62.02% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.26%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.78%\**</span> | <span title="validated on-demand with error">*2.62%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*49.07%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*3.32%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*45.24%\**</span> | <span title="validated on-demand with panic">*0.51%\**</span> | ‡ |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
