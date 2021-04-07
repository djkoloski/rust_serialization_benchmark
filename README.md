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

## Last updated: 4/7/2021

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 284.97 us | † | 1705800 | 515071 |
| bincode | 577.70 us | 4.4412 ms | 1045784 | 374305 |
| borsh | 595.21 us | 4.3560 ms | 885780 | 363280 |
| capnp | 1.1160 ms | † | 1443216 | 509618 |
| cbor | 2.0576 ms | 9.1698 ms | 1407835 | 407372 |
| flatbuffers | 2.7043 ms | † | 1276368 | 469962 |
| nachricht | 4.8077 ms | 8.5652 ms | 926221 | 365209 |
| postcard | 708.18 us | 4.9370 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*5.7292 ms\**</span> <span title="encode">*1.6435 ms\**</span> | 5.4172 ms | 764951 | 269811 |
| rkyv | 328.56 us | 3.2858 ms | 1065784 | 333895 |
| rmp | 1.4879 ms | 5.5829 ms | 784997 | 326654 |
| ron | 19.496 ms | 16.740 ms | 1607459 | 452648 |
| serde_json | 4.3452 ms | 10.109 ms | 1827461 | 474358 |
| speedy | 278.71 us | 3.6255 ms | 885780 | 363280 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*40.042 us\**</span> | <span title="unvalidated">*62.263 us\**</span> | ‡ |
| capnp | <span title="progressive">*176.97 ns\**</span> | <span title="progressive">*551.11 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9487 ns\**</span> | <span title="unvalidated">*148.24 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3858 ns\**</span> <span title="validated">*1.1662 ms\**</span> | <span title="unvalidated">*19.188 us\**</span> <span title="validated">*1.1877 ms\**</span> | 69.544 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 97.80% | † | 44.84% | 52.38% |
| bincode | 48.24% | 73.98% | 73.15% | 72.08% |
| borsh | 46.83% | 75.43% | 86.36% | 74.27% |
| capnp | 24.97% | † | 53.00% | 52.94% |
| cbor | 13.55% | 35.83% | 54.34% | 66.23% |
| flatbuffers | 10.31% | † | 59.93% | 57.41% |
| nachricht | 5.80% | 38.36% | 82.59% | 73.88% |
| postcard | 39.36% | 66.55% | 99.89% | 86.27% |
| prost | <span title="populate + encode">*4.86%\**</span> <span title="encode">*16.96%\**</span> | 60.65% | 100.00% | 100.00% |
| rkyv | 84.83% | 100.00% | 71.77% | 80.81% |
| rmp | 18.73% | 58.85% | 97.45% | 82.60% |
| ron | 1.43% | 19.63% | 47.59% | 59.61% |
| serde_json | 6.41% | 32.50% | 41.86% | 56.88% |
| speedy | 100.00% | 90.63% | 86.36% | 74.27% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*30.82%\**</span> | ‡ |
| capnp | <span title="progressive">*0.78%\**</span> | <span title="progressive">*3.48%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*47.00%\**</span> | <span title="unvalidated">*12.94%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.62%\**</span> | 100.00% |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 478.92 us | † | 6000024 | 5380837 |
| bincode | 5.6116 ms | 8.4845 ms | 6000008 | 5380823 |
| borsh | 5.9487 ms | 9.6293 ms | 6000004 | 5380818 |
| capnp | 14.574 ms | † | 14000088 | 6729881 |
| cbor | 44.869 ms | 66.815 ms | 13122324 | 7527423 |
| flatbuffers | 1.9508 ms | † | 6000024 | 5380800 |
| nachricht | 72.542 ms | 64.138 ms | 10125030 | 7160144 |
| postcard | 6.7459 ms | 10.736 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*36.184 ms\**</span> <span title="encode">*31.349 ms\**</span> | 27.983 ms | 8750000 | 6683814 |
| rkyv | 928.28 us | 1.9977 ms | 6000008 | 4263104 |
| rmp | 26.107 ms | 22.102 ms | 8125006 | 6496879 |
| ron | 216.28 ms | 389.68 ms | 22192885 | 9009575 |
| serde_json | 108.05 ms | 87.423 ms | 26192883 | 9612105 |
| speedy | 1.0281 ms | 2.3742 ms | 6000004 | 5380818 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.3093 ns\**</span> | <span title="unvalidated">*172.75 us\**</span> | ‡ |
| capnp | <span title="progressive">*261.25 ns\**</span> | <span title="progressive">*8.0320 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9439 ns\**</span> | <span title="unvalidated">*187.85 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3819 ns\**</span> <span title="validated">*104.01 ns\**</span> | <span title="unvalidated">*167.06 us\**</span> <span title="validated">*172.42 us\**</span> | 622.92 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 100.00% | 79.23% |
| bincode | 8.53% | 23.55% | 100.00% | 79.23% |
| borsh | 8.05% | 20.75% | 100.00% | 79.23% |
| capnp | 3.29% | † | 42.86% | 63.35% |
| cbor | 1.07% | 2.99% | 45.72% | 56.63% |
| flatbuffers | 24.55% | † | 100.00% | 79.23% |
| nachricht | 0.66% | 3.11% | 59.26% | 59.54% |
| postcard | 7.10% | 18.61% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*1.32%\**</span> <span title="encode">*1.53%\**</span> | 7.14% | 68.57% | 63.78% |
| rkyv | 51.59% | 100.00% | 100.00% | 100.00% |
| rmp | 1.83% | 9.04% | 73.85% | 65.62% |
| ron | 0.22% | 0.51% | 27.04% | 47.32% |
| serde_json | 0.44% | 2.29% | 22.91% | 44.35% |
| speedy | 46.58% | 84.14% | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*59.84%\**</span> | <span title="unvalidated">*96.71%\**</span> | ‡ |
| capnp | <span title="progressive">*0.53%\**</span> | <span title="progressive">*2.08%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.94%\**</span> | <span title="unvalidated">*88.93%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.33%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*96.89%\**</span> | 100.00% |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 372.00 us | † | 1290592 | 388594 |
| bincode | 767.25 us | 3.3674 ms | 569975 | 240897 |
| borsh | 757.89 us | 3.1162 ms | 446595 | 234395 |
| capnp | 829.51 us | † | 803896 | 336655 |
| cbor | 2.0202 ms | 8.7192 ms | 1109821 | 347562 |
| flatbuffers | 42.050 ms | † | 849472 | 349208 |
| nachricht | 5.1081 ms | 7.5351 ms | 535881 | 281994 |
| postcard | 818.22 us | 3.8986 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*6.2615 ms\**</span> <span title="encode">*2.9483 ms\**</span> | 5.3465 ms | 596811 | 306728 |
| rkyv | 676.95 us | 2.5305 ms | 725176 | 332248 |
| rmp | 1.8352 ms | 4.7061 ms | 418025 | 244771 |
| ron | 10.348 ms | 18.935 ms | 1465229 | 439673 |
| serde_json | 4.3218 ms | 10.603 ms | 1623197 | 472162 |
| speedy | 535.23 us | 2.8670 ms | 449595 | 235136 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*47.266 us\**</span> | <span title="unvalidated">*48.642 us\**</span> | ‡ |
| capnp | <span title="progressive">*172.67 ns\**</span> | <span title="progressive">*5.2440 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.8813 ns\**</span> | <span title="unvalidated">*4.6359 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3759 ns\**</span> <span title="validated">*1.3305 ms\**</span> | <span title="unvalidated">*288.19 ns\**</span> <span title="validated">*1.3264 ms\**</span> | 6.4466 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 27.61% | 54.88% |
| bincode | 48.48% | 75.15% | 62.51% | 88.53% |
| borsh | 49.08% | 81.20% | 79.78% | 90.99% |
| capnp | 44.85% | † | 44.32% | 63.35% |
| cbor | 18.41% | 29.02% | 32.11% | 61.36% |
| flatbuffers | 0.88% | † | 41.94% | 61.07% |
| nachricht | 7.28% | 33.58% | 66.49% | 75.63% |
| postcard | 45.46% | 64.91% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*5.94%\**</span> <span title="encode">*12.62%\**</span> | 47.33% | 59.70% | 69.53% |
| rkyv | 54.95% | 100.00% | 49.13% | 64.19% |
| rmp | 20.27% | 53.77% | 85.24% | 87.13% |
| ron | 3.59% | 13.36% | 24.32% | 48.51% |
| serde_json | 8.61% | 23.87% | 21.95% | 45.17% |
| speedy | 69.50% | 88.26% | 79.25% | 90.70% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.59%\**</span> | ‡ |
| capnp | <span title="progressive">*0.80%\**</span> | <span title="progressive">*5.50%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*47.75%\**</span> | <span title="unvalidated">*6.22%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.02%\**</span> | 100.00% |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
