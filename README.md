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

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 309.12 us | † | 1705800 | 513555 |
| bincode | 608.29 us | 4.3153 ms | 1045784 | 374305 |
| borsh | 642.70 us | 4.0454 ms | 885780 | 363280 |
| capnp | 1.1145 ms | † | 1443216 | 509618 |
| cbor | 2.6636 ms | 9.0151 ms | 1407835 | 407372 |
| flatbuffers | 2.6439 ms | † | 1276368 | 469962 |
| nachricht | 4.7173 ms | 8.3851 ms | 926221 | 365209 |
| postcard | 688.72 us | 4.6072 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*5.4434 ms\**</span> <span title="encode">*1.5206 ms\**</span> | 4.9772 ms | 764951 | 269811 |
| rkyv | 412.46 us | 3.2086 ms | 1065784 | 333895 |
| rmp | 1.7976 ms | 5.4838 ms | 784997 | 326654 |
| serde_json | 4.3679 ms | 9.6170 ms | 1827461 | 474358 |
| speedy | 234.46 us | 3.3268 ms | 885780 | 363280 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*38.648 us\**</span> | <span title="unvalidated">*61.376 us\**</span> | ‡ |
| capnp | <span title="progressive">*176.31 ns\**</span> | <span title="progressive">*555.33 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9739 ns\**</span> | <span title="unvalidated">*155.62 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3861 ns\**</span> <span title="validated">*1.4008 ms\**</span> | <span title="unvalidated">*18.567 us\**</span> <span title="validated">*1.4173 ms\**</span> | 69.008 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 75.85% | † | 44.84% | 52.54% |
| bincode | 38.54% | 74.35% | 73.15% | 72.08% |
| borsh | 36.48% | 79.31% | 86.36% | 74.27% |
| capnp | 21.04% | † | 53.00% | 52.94% |
| cbor | 8.80% | 35.59% | 54.34% | 66.23% |
| flatbuffers | 8.87% | † | 59.93% | 57.41% |
| nachricht | 4.97% | 38.27% | 82.59% | 73.88% |
| postcard | 34.04% | 69.64% | 99.89% | 86.27% |
| prost | <span title="populate + encode">*4.31%\**</span> <span title="encode">*15.42%\**</span> | 64.47% | 100.00% | 100.00% |
| rkyv | 56.84% | 100.00% | 71.77% | 80.81% |
| rmp | 13.04% | 58.51% | 97.45% | 82.60% |
| serde_json | 5.37% | 33.36% | 41.86% | 56.88% |
| speedy | 100.00% | 96.45% | 86.36% | 74.27% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*30.25%\**</span> | ‡ |
| capnp | <span title="progressive">*0.79%\**</span> | <span title="progressive">*3.34%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.61%\**</span> | <span title="unvalidated">*11.93%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.31%\**</span> | 100.00% |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 442.26 us | † | 6000024 | 5380837 |
| bincode | 6.5212 ms | 8.2292 ms | 6000008 | 5380823 |
| borsh | 6.2993 ms | 9.1370 ms | 6000004 | 5380818 |
| capnp | 13.955 ms | † | 14000088 | 6729881 |
| cbor | 53.421 ms | 63.574 ms | 13122324 | 7527423 |
| flatbuffers | 1.8650 ms | † | 6000024 | 5380800 |
| nachricht | 73.740 ms | 62.899 ms | 10125030 | 7160144 |
| postcard | 6.4621 ms | 10.129 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*32.797 ms\**</span> <span title="encode">*28.955 ms\**</span> | 18.018 ms | 8750000 | 6683814 |
| rkyv | 1.1173 ms | 1.8938 ms | 6000008 | 4263104 |
| rmp | 27.338 ms | 25.788 ms | 8125006 | 6496879 |
| serde_json | 110.94 ms | 85.765 ms | 26192883 | 9612105 |
| speedy | 767.41 us | 2.1705 ms | 6000004 | 5380818 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*4.5978 ns\**</span> | <span title="unvalidated">*182.02 us\**</span> | ‡ |
| capnp | <span title="progressive">*259.46 ns\**</span> | <span title="progressive">*7.8335 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.8904 ns\**</span> | <span title="unvalidated">*147.00 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3824 ns\**</span> <span title="validated">*105.87 ns\**</span> | <span title="unvalidated">*166.37 us\**</span> <span title="validated">*170.64 us\**</span> | 650.43 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 100.00% | 79.23% |
| bincode | 6.78% | 23.01% | 100.00% | 79.23% |
| borsh | 7.02% | 20.73% | 100.00% | 79.23% |
| capnp | 3.17% | † | 42.86% | 63.35% |
| cbor | 0.83% | 2.98% | 45.72% | 56.63% |
| flatbuffers | 23.71% | † | 100.00% | 79.23% |
| nachricht | 0.60% | 3.01% | 59.26% | 59.54% |
| postcard | 6.84% | 18.70% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*1.35%\**</span> <span title="encode">*1.53%\**</span> | 10.51% | 68.57% | 63.78% |
| rkyv | 39.58% | 100.00% | 100.00% | 100.00% |
| rmp | 1.62% | 7.34% | 73.85% | 65.62% |
| serde_json | 0.40% | 2.21% | 22.91% | 44.35% |
| speedy | 57.63% | 87.25% | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*30.07%\**</span> | <span title="unvalidated">*80.76%\**</span> | ‡ |
| capnp | <span title="progressive">*0.53%\**</span> | <span title="progressive">*1.88%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*47.83%\**</span> | <span title="unvalidated">*100.00%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.31%\**</span> | <span title="unvalidated">*88.36%\**</span> <span title="validated">*86.15%\**</span> | 100.00% |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 376.83 us | † | 1290592 | 392428 |
| bincode | 812.51 us | 3.3252 ms | 569975 | 240897 |
| borsh | 819.31 us | 3.0513 ms | 446595 | 234395 |
| capnp | 824.14 us | † | 803896 | 336655 |
| cbor | 2.5694 ms | 8.4288 ms | 1109821 | 347562 |
| flatbuffers | 35.973 ms | † | 849472 | 349208 |
| nachricht | 5.0746 ms | 7.4325 ms | 535881 | 281994 |
| postcard | 809.02 us | 3.7454 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*6.0789 ms\**</span> <span title="encode">*2.8664 ms\**</span> | 5.0887 ms | 596811 | 306728 |
| rkyv | 850.96 us | 2.4278 ms | 725176 | 333872 |
| rmp | 2.0564 ms | 4.7795 ms | 418025 | 244771 |
| serde_json | 4.4232 ms | 10.577 ms | 1623197 | 472162 |
| speedy | 508.66 us | 2.6102 ms | 449595 | 235136 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*45.091 us\**</span> | <span title="unvalidated">*45.941 us\**</span> | ‡ |
| capnp | <span title="progressive">*173.76 ns\**</span> | <span title="progressive">*5.3281 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9387 ns\**</span> | <span title="unvalidated">*3.7072 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3708 ns\**</span> <span title="validated">*1.3923 ms\**</span> | <span title="unvalidated">*295.40 ns\**</span> <span title="validated">*1.3855 ms\**</span> | 6.4764 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 27.61% | 54.35% |
| bincode | 46.38% | 73.01% | 62.51% | 88.53% |
| borsh | 45.99% | 79.57% | 79.78% | 90.99% |
| capnp | 45.72% | † | 44.32% | 63.35% |
| cbor | 14.67% | 28.80% | 32.11% | 61.36% |
| flatbuffers | 1.05% | † | 41.94% | 61.07% |
| nachricht | 7.43% | 32.66% | 66.49% | 75.63% |
| postcard | 46.58% | 64.82% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*6.20%\**</span> <span title="encode">*13.15%\**</span> | 47.71% | 59.70% | 69.53% |
| rkyv | 44.28% | 100.00% | 49.13% | 63.88% |
| rmp | 18.32% | 50.80% | 85.24% | 87.13% |
| serde_json | 8.52% | 22.95% | 21.95% | 45.17% |
| speedy | 74.08% | 93.01% | 79.25% | 90.70% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.64%\**</span> | ‡ |
| capnp | <span title="progressive">*0.79%\**</span> | <span title="progressive">*5.54%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.65%\**</span> | <span title="unvalidated">*7.97%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.02%\**</span> | 100.00% |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
