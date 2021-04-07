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
| abomonation | 283.71 us | † | 1705800 | 507962 |
| bincode | 565.58 us | 4.2250 ms | 1045784 | 374305 |
| borsh | 588.72 us | 3.9762 ms | 885780 | 363280 |
| capnp | 1.0546 ms | † | 1443216 | 509618 |
| cbor | 1.8369 ms | 8.7979 ms | 1407835 | 407372 |
| flatbuffers | 2.6755 ms | † | 1276368 | 469962 |
| nachricht | 5.0971 ms | 8.5754 ms | 926221 | 365209 |
| postcard | 722.11 us | 4.4421 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*5.3357 ms\**</span> <span title="encode">*1.5320 ms\**</span> | 4.8328 ms | 764951 | 269811 |
| rkyv | 304.26 us | 3.0186 ms | 1065784 | 333895 |
| rmp | 1.5120 ms | 5.1962 ms | 784997 | 326654 |
| serde_json | 4.5899 ms | 9.6851 ms | 1827461 | 474358 |
| speedy | 229.12 us | 3.2951 ms | 885780 | 363280 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*37.107 us\**</span> | <span title="unvalidated">*57.939 us\**</span> | ‡ |
| capnp | <span title="progressive">*167.91 ns\**</span> | <span title="progressive">*516.47 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9672 ns\**</span> | <span title="unvalidated">*149.57 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3755 ns\**</span> <span title="validated">*1.0828 ms\**</span> | <span title="unvalidated">*18.737 us\**</span> <span title="validated">*1.1017 ms\**</span> | 67.741 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 80.76% | † | 44.84% | 53.12% |
| bincode | 40.51% | 71.45% | 73.15% | 72.08% |
| borsh | 38.92% | 75.92% | 86.36% | 74.27% |
| capnp | 21.73% | † | 53.00% | 52.94% |
| cbor | 12.47% | 34.31% | 54.34% | 66.23% |
| flatbuffers | 8.56% | † | 59.93% | 57.41% |
| nachricht | 4.50% | 35.20% | 82.59% | 73.88% |
| postcard | 31.73% | 67.95% | 99.89% | 86.27% |
| prost | <span title="populate + encode">*4.29%\**</span> <span title="encode">*14.96%\**</span> | 62.46% | 100.00% | 100.00% |
| rkyv | 75.30% | 100.00% | 71.77% | 80.81% |
| rmp | 15.15% | 58.09% | 97.45% | 82.60% |
| serde_json | 4.99% | 31.17% | 41.86% | 56.88% |
| speedy | 100.00% | 91.61% | 86.36% | 74.27% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*32.34%\**</span> | ‡ |
| capnp | <span title="progressive">*0.82%\**</span> | <span title="progressive">*3.63%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.36%\**</span> | <span title="unvalidated">*12.53%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.70%\**</span> | 100.00% |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 427.09 us | † | 6000024 | 5380836 |
| bincode | 5.3025 ms | 8.0516 ms | 6000008 | 5380823 |
| borsh | 5.8440 ms | 8.9360 ms | 6000004 | 5380818 |
| capnp | 13.314 ms | † | 14000088 | 6729881 |
| cbor | 41.295 ms | 62.957 ms | 13122324 | 7527423 |
| flatbuffers | 1.9065 ms | † | 6000024 | 5380800 |
| nachricht | 73.230 ms | 60.749 ms | 10125030 | 7160144 |
| postcard | 6.2413 ms | 9.6155 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*31.826 ms\**</span> <span title="encode">*27.568 ms\**</span> | 17.812 ms | 8750000 | 6683814 |
| rkyv | 893.53 us | 1.8114 ms | 6000008 | 4263104 |
| rmp | 23.014 ms | 21.186 ms | 8125006 | 6496879 |
| serde_json | 102.74 ms | 83.308 ms | 26192883 | 9612105 |
| speedy | 781.92 us | 2.1897 ms | 6000004 | 5380818 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.3198 ns\**</span> | <span title="unvalidated">*169.11 us\**</span> | ‡ |
| capnp | <span title="progressive">*252.06 ns\**</span> | <span title="progressive">*7.4730 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9162 ns\**</span> | <span title="unvalidated">*150.14 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3800 ns\**</span> <span title="validated">*100.89 ns\**</span> | <span title="unvalidated">*180.28 us\**</span> <span title="validated">*184.32 us\**</span> | 625.53 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 100.00% | 79.23% |
| bincode | 8.05% | 22.50% | 100.00% | 79.23% |
| borsh | 7.31% | 20.27% | 100.00% | 79.23% |
| capnp | 3.21% | † | 42.86% | 63.35% |
| cbor | 1.03% | 2.88% | 45.72% | 56.63% |
| flatbuffers | 22.40% | † | 100.00% | 79.23% |
| nachricht | 0.58% | 2.98% | 59.26% | 59.54% |
| postcard | 6.84% | 18.84% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*1.34%\**</span> <span title="encode">*1.55%\**</span> | 10.17% | 68.57% | 63.78% |
| rkyv | 47.80% | 100.00% | 100.00% | 100.00% |
| rmp | 1.86% | 8.55% | 73.85% | 65.62% |
| serde_json | 0.42% | 2.17% | 22.91% | 44.35% |
| speedy | 54.62% | 82.72% | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*59.49%\**</span> | <span title="unvalidated">*88.78%\**</span> | ‡ |
| capnp | <span title="progressive">*0.55%\**</span> | <span title="progressive">*2.01%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*47.32%\**</span> | <span title="unvalidated">*100.00%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.37%\**</span> | <span title="unvalidated">*83.28%\**</span> <span title="validated">*81.46%\**</span> | 100.00% |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 366.47 us | † | 1290592 | 389682 |
| bincode | 703.27 us | 3.1610 ms | 569975 | 240897 |
| borsh | 716.31 us | 2.9692 ms | 446595 | 234395 |
| capnp | 807.89 us | † | 803896 | 336655 |
| cbor | 1.9711 ms | 8.0693 ms | 1109821 | 347562 |
| flatbuffers | 37.221 ms | † | 849472 | 349208 |
| nachricht | 5.2105 ms | 6.9135 ms | 535881 | 281994 |
| postcard | 786.81 us | 3.6515 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*5.8374 ms\**</span> <span title="encode">*2.7914 ms\**</span> | 4.9213 ms | 596811 | 306728 |
| rkyv | 620.38 us | 2.3272 ms | 725176 | 332267 |
| rmp | 1.8107 ms | 4.4639 ms | 418025 | 244771 |
| serde_json | 4.5117 ms | 10.203 ms | 1623197 | 472162 |
| speedy | 472.90 us | 2.6209 ms | 449595 | 235136 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*41.065 us\**</span> | <span title="unvalidated">*41.754 us\**</span> | ‡ |
| capnp | <span title="progressive">*172.15 ns\**</span> | <span title="progressive">*5.2505 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9131 ns\**</span> | <span title="unvalidated">*3.4700 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3730 ns\**</span> <span title="validated">*1.2810 ms\**</span> | <span title="unvalidated">*288.79 ns\**</span> <span title="validated">*1.2907 ms\**</span> | 6.4546 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 27.61% | 54.73% |
| bincode | 52.11% | 73.62% | 62.51% | 88.53% |
| borsh | 51.16% | 78.38% | 79.78% | 90.99% |
| capnp | 45.36% | † | 44.32% | 63.35% |
| cbor | 18.59% | 28.84% | 32.11% | 61.36% |
| flatbuffers | 0.98% | † | 41.94% | 61.07% |
| nachricht | 7.03% | 33.66% | 66.49% | 75.63% |
| postcard | 46.58% | 63.73% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*6.28%\**</span> <span title="encode">*13.13%\**</span> | 47.29% | 59.70% | 69.53% |
| rkyv | 59.07% | 100.00% | 49.13% | 64.19% |
| rmp | 20.24% | 52.13% | 85.24% | 87.13% |
| serde_json | 8.12% | 22.81% | 21.95% | 45.17% |
| speedy | 77.49% | 88.79% | 79.25% | 90.70% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.69%\**</span> | ‡ |
| capnp | <span title="progressive">*0.80%\**</span> | <span title="progressive">*5.50%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*47.13%\**</span> | <span title="unvalidated">*8.32%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.02%\**</span> | 100.00% |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
