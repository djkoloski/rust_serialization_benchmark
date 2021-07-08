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

## Last updated: 7/8/2021

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 263.87 us | 3.5273 ms | 1705800 | 507959 |
| bincode | 568.87 us | 4.3810 ms | 1045784 | 374305 |
| borsh | 573.28 us | 4.2894 ms | 885780 | 363280 |
| capnp | 1.2977 ms | † | 1443216 | 509618 |
| cbor | 1.9669 ms | 8.9866 ms | 1407835 | 407372 |
| flatbuffers | 2.7988 ms | † | 1276368 | 469962 |
| nachricht | 4.8314 ms | 7.2423 ms | 926221 | 365209 |
| postcard | 772.29 us | 4.7900 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*5.7039 ms\**</span> <span title="encode">*1.5565 ms\**</span> | 5.5017 ms | 764951 | 269811 |
| rkyv | 301.63 us | 3.3178 ms | 1011488 | 269353 |
| rmp | 1.7060 ms | 5.4399 ms | 784997 | 326654 |
| ron | 20.516 ms | 17.014 ms | 1607459 | 452648 |
| serde_json | 4.7070 ms | 10.261 ms | 1827461 | 474358 |
| speedy | 285.71 us | 3.5636 ms | 885780 | 363280 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*38.392 us\**</span> | <span title="unvalidated">*61.208 us\**</span> | ‡ |
| capnp | <span title="progressive">*160.17 ns\**</span> | <span title="progressive">*522.74 us\**</span> | ‡ |
| flatbuffers | <span title="unverified">*3.0748 ns\**</span> <span title="verified">*2.0884 ms\**</span> | <span title="unverified">*148.17 us\**</span> <span title="verified">*2.2442 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4334 ns\**</span> <span title="validated">*781.86 us\**</span> | <span title="unvalidated">*17.171 us\**</span> <span title="validated">*805.60 us\**</span> | 67.746 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | 94.06% | 44.84% | 53.03% |
| bincode | 46.38% | 75.73% | 73.15% | 71.96% |
| borsh | 46.03% | 77.35% | 86.36% | 74.14% |
| capnp | 20.33% | † | 53.00% | 52.85% |
| cbor | 13.42% | 36.92% | 54.34% | 66.12% |
| flatbuffers | 9.43% | † | 59.93% | 57.31% |
| nachricht | 5.46% | 45.81% | 82.59% | 73.75% |
| postcard | 34.17% | 69.27% | 99.89% | 86.13% |
| prost | <span title="populate + encode">*4.63%\**</span> <span title="encode">*16.95%\**</span> | 60.30% | 100.00% | 99.83% |
| rkyv | 87.48% | 100.00% | 75.63% | 100.00% |
| rmp | 15.47% | 60.99% | 97.45% | 82.46% |
| ron | 1.29% | 19.50% | 47.59% | 59.51% |
| serde_json | 5.61% | 32.33% | 41.86% | 56.78% |
| speedy | 92.36% | 93.10% | 86.36% | 74.14% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*28.05%\**</span> | ‡ |
| capnp | <span title="progressive">*0.89%\**</span> | <span title="progressive">*3.28%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*46.62%\**</span> <span title="verified">*0.00%\**</span> | <span title="unverified">*11.59%\**</span> <span title="verified">*0.77%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*2.13%\**</span> | 100.00% |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 320.05 us | 1.5084 ms | 6000024 | 5380836 |
| bincode | 5.6956 ms | 6.9924 ms | 6000008 | 5380823 |
| borsh | 6.8222 ms | 9.3052 ms | 6000004 | 5380818 |
| capnp | 22.499 ms | † | 14000088 | 6729881 |
| cbor | 46.372 ms | 65.293 ms | 13122324 | 7527423 |
| flatbuffers | 2.7042 ms | † | 6000024 | 5380800 |
| nachricht | 68.892 ms | 41.301 ms | 10125030 | 7160144 |
| postcard | 7.6779 ms | 10.123 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*34.193 ms\**</span> <span title="encode">*28.369 ms\**</span> | 21.705 ms | 8750000 | 6683814 |
| rkyv | 1.0675 ms | 1.9211 ms | 6000008 | 4263104 |
| rmp | 19.816 ms | 17.538 ms | 8125006 | 6496879 |
| ron | 210.33 ms | 397.00 ms | 22192885 | 9009575 |
| serde_json | 110.58 ms | 94.521 ms | 26192883 | 9612105 |
| speedy | 1.0370 ms | 2.3548 ms | 6000004 | 5380818 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.4447 ns\**</span> | <span title="unvalidated">*203.69 us\**</span> | ‡ |
| capnp | <span title="progressive">*243.60 ns\**</span> | <span title="progressive">*7.1067 ms\**</span> | ‡ |
| flatbuffers | <span title="unverified">*3.0112 ns\**</span> <span title="verified">*42.438 ns\**</span> | <span title="unverified">*193.77 us\**</span> <span title="verified">*153.66 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4226 ns\**</span> <span title="validated">*10.269 ns\**</span> | <span title="unvalidated">*42.244 us\**</span> <span title="validated">*41.982 us\**</span> | 638.44 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | 100.00% | 100.00% | 79.23% |
| bincode | 5.62% | 21.57% | 100.00% | 79.23% |
| borsh | 4.69% | 16.21% | 100.00% | 79.23% |
| capnp | 1.42% | † | 42.86% | 63.35% |
| cbor | 0.69% | 2.31% | 45.72% | 56.63% |
| flatbuffers | 11.84% | † | 100.00% | 79.23% |
| nachricht | 0.46% | 3.65% | 59.26% | 59.54% |
| postcard | 4.17% | 14.90% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*0.94%\**</span> <span title="encode">*1.13%\**</span> | 6.95% | 68.57% | 63.78% |
| rkyv | 29.98% | 78.52% | 100.00% | 100.00% |
| rmp | 1.62% | 8.60% | 73.85% | 65.62% |
| ron | 0.15% | 0.38% | 27.04% | 47.32% |
| serde_json | 0.29% | 1.60% | 22.91% | 44.35% |
| speedy | 30.86% | 64.06% | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*58.19%\**</span> | <span title="unvalidated">*20.61%\**</span> | ‡ |
| capnp | <span title="progressive">*0.58%\**</span> | <span title="progressive">*0.59%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*47.24%\**</span> <span title="verified">*3.35%\**</span> | <span title="unverified">*21.67%\**</span> <span title="verified">*27.32%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*13.85%\**</span> | <span title="unvalidated">*99.38%\**</span> <span title="validated">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 386.09 us | 2.5110 ms | 1290592 | 388760 |
| bincode | 816.29 us | 3.1478 ms | 569975 | 240897 |
| borsh | 768.26 us | 3.0586 ms | 446595 | 234395 |
| capnp | 853.91 us | † | 803896 | 336655 |
| cbor | 2.1301 ms | 8.2502 ms | 1109821 | 347562 |
| flatbuffers | 35.469 ms | † | 844168 | 346957 |
| nachricht | 5.3575 ms | 6.3173 ms | 535881 | 281994 |
| postcard | 872.95 us | 3.7714 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*6.1258 ms\**</span> <span title="encode">*2.9073 ms\**</span> | 5.2903 ms | 596811 | 306728 |
| rkyv | 574.77 us | 2.4885 ms | 596952 | 241952 |
| rmp | 1.9530 ms | 4.4401 ms | 418025 | 244771 |
| ron | 10.501 ms | 18.667 ms | 1465229 | 439673 |
| serde_json | 4.7200 ms | 10.702 ms | 1623197 | 472162 |
| speedy | 553.44 us | 2.6691 ms | 449595 | 235136 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*46.675 us\**</span> | <span title="unvalidated">*47.556 us\**</span> | ‡ |
| capnp | <span title="progressive">*154.40 ns\**</span> | <span title="progressive">*5.8997 us\**</span> | ‡ |
| flatbuffers | <span title="unverified">*2.9999 ns\**</span> <span title="verified">*2.6472 ms\**</span> | <span title="unverified">*3.6585 us\**</span> <span title="verified">*2.6349 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4421 ns\**</span> <span title="validated">*658.99 us\**</span> | <span title="unvalidated">*174.60 ns\**</span> <span title="validated">*659.27 us\**</span> | 6.6326 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | 99.10% | 27.61% | 54.86% |
| bincode | 47.30% | 79.06% | 62.51% | 88.53% |
| borsh | 50.26% | 81.36% | 79.78% | 90.99% |
| capnp | 45.21% | † | 44.32% | 63.35% |
| cbor | 18.13% | 30.16% | 32.11% | 61.36% |
| flatbuffers | 1.09% | † | 42.21% | 61.47% |
| nachricht | 7.21% | 39.39% | 66.49% | 75.63% |
| postcard | 44.23% | 65.98% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*6.30%\**</span> <span title="encode">*13.28%\**</span> | 47.04% | 59.70% | 69.53% |
| rkyv | 67.17% | 100.00% | 59.69% | 88.15% |
| rmp | 19.77% | 56.05% | 85.24% | 87.13% |
| ron | 3.68% | 13.33% | 24.32% | 48.51% |
| serde_json | 8.18% | 23.25% | 21.95% | 45.17% |
| speedy | 69.76% | 93.23% | 79.25% | 90.70% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.37%\**</span> | ‡ |
| capnp | <span title="progressive">*0.93%\**</span> | <span title="progressive">*2.96%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*48.07%\**</span> <span title="verified">*0.00%\**</span> | <span title="unverified">*4.77%\**</span> <span title="verified">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.03%\**</span> | 100.00% |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
