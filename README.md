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
| abomonation | 255.64 us | <span title="unvalidated">*3.5491 ms\**</span> | 1705800 | 515093 |
| bincode | 588.33 us | 4.3874 ms | 1045784 | 374305 |
| borsh | 565.78 us | 4.2550 ms | 885780 | 363280 |
| capnp | 1.3166 ms | † | 1443216 | 509618 |
| cbor | 1.9742 ms | 8.8848 ms | 1407835 | 407372 |
| flatbuffers | 2.7453 ms | † | 1276368 | 469962 |
| nachricht | 4.6984 ms | 7.2897 ms | 926221 | 365209 |
| postcard | 757.87 us | 4.8374 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*5.7375 ms\**</span> <span title="encode">*1.6139 ms\**</span> | 5.4935 ms | 764951 | 269811 |
| rkyv | 324.62 us | <span title="unvalidated">*3.3852 ms\**</span> <span title="validated">*4.1474 ms\**</span> | 1011488 | 269353 |
| rmp | 1.5913 ms | 5.3595 ms | 784997 | 326654 |
| ron | 20.936 ms | 17.365 ms | 1607459 | 452648 |
| serde_json | 4.7580 ms | 10.166 ms | 1827461 | 474358 |
| speedy | 279.26 us | 3.6712 ms | 885780 | 363280 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*38.308 us\**</span> | <span title="unvalidated">*60.847 us\**</span> | ‡ |
| capnp | <span title="progressive">*154.39 ns\**</span> | <span title="progressive">*520.38 us\**</span> | ‡ |
| flatbuffers | <span title="unverified">*3.0196 ns\**</span> <span title="verified">*2.0877 ms\**</span> | <span title="unverified">*146.79 us\**</span> <span title="verified">*2.2544 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4170 ns\**</span> <span title="validated">*805.88 us\**</span> | <span title="unvalidated">*17.013 us\**</span> <span title="validated">*816.24 us\**</span> | 70.533 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*95.38%\**</span> | 44.84% | 52.29% |
| bincode | 43.45% | 77.16% | 73.15% | 71.96% |
| borsh | 45.18% | 79.56% | 86.36% | 74.14% |
| capnp | 19.42% | † | 53.00% | 52.85% |
| cbor | 12.95% | 38.10% | 54.34% | 66.12% |
| flatbuffers | 9.31% | † | 59.93% | 57.31% |
| nachricht | 5.44% | 46.44% | 82.59% | 73.75% |
| postcard | 33.73% | 69.98% | 99.89% | 86.13% |
| prost | <span title="populate + encode">*4.46%\**</span> <span title="encode">*15.84%\**</span> | 61.62% | 100.00% | 99.83% |
| rkyv | 78.75% | <span title="unvalidated">*100.00%\**</span> <span title="validated">*81.62%\**</span> | 75.63% | 100.00% |
| rmp | 16.06% | 63.16% | 97.45% | 82.46% |
| ron | 1.22% | 19.49% | 47.59% | 59.51% |
| serde_json | 5.37% | 33.30% | 41.86% | 56.78% |
| speedy | 91.54% | 92.21% | 86.36% | 74.14% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*27.96%\**</span> | ‡ |
| capnp | <span title="progressive">*0.92%\**</span> | <span title="progressive">*3.27%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*46.93%\**</span> <span title="verified">*0.00%\**</span> | <span title="unverified">*11.59%\**</span> <span title="verified">*0.75%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*2.08%\**</span> | 100.00% |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 256.32 us | <span title="unvalidated">*1.5699 ms\**</span> | 6000024 | 5380836 |
| bincode | 5.4074 ms | 7.3694 ms | 6000008 | 5380823 |
| borsh | 6.8322 ms | 9.2224 ms | 6000004 | 5380818 |
| capnp | 22.385 ms | † | 14000088 | 6729881 |
| cbor | 46.866 ms | 64.337 ms | 13122324 | 7527423 |
| flatbuffers | 2.7800 ms | † | 6000024 | 5380800 |
| nachricht | 67.207 ms | 41.845 ms | 10125030 | 7160144 |
| postcard | 7.5691 ms | 10.274 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*34.827 ms\**</span> <span title="encode">*28.772 ms\**</span> | 21.797 ms | 8750000 | 6683814 |
| rkyv | 1.1004 ms | <span title="unvalidated">*1.9300 ms\**</span> <span title="validated">*1.9443 ms\**</span> | 6000008 | 4263104 |
| rmp | 19.699 ms | 17.989 ms | 8125006 | 6496879 |
| ron | 209.94 ms | 376.99 ms | 22192885 | 9009575 |
| serde_json | 112.16 ms | 97.129 ms | 26192883 | 9612105 |
| speedy | 1.0318 ms | 2.4210 ms | 6000004 | 5380818 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.4643 ns\**</span> | <span title="unvalidated">*181.12 us\**</span> | ‡ |
| capnp | <span title="progressive">*248.62 ns\**</span> | <span title="progressive">*7.1449 ms\**</span> | ‡ |
| flatbuffers | <span title="unverified">*3.0352 ns\**</span> <span title="verified">*42.223 ns\**</span> | <span title="unverified">*157.39 us\**</span> <span title="verified">*155.14 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4420 ns\**</span> <span title="validated">*11.326 ns\**</span> | <span title="unvalidated">*42.916 us\**</span> <span title="validated">*42.512 us\**</span> | 651.13 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 79.23% |
| bincode | 4.74% | 21.30% | 100.00% | 79.23% |
| borsh | 3.75% | 17.02% | 100.00% | 79.23% |
| capnp | 1.15% | † | 42.86% | 63.35% |
| cbor | 0.55% | 2.44% | 45.72% | 56.63% |
| flatbuffers | 9.22% | † | 100.00% | 79.23% |
| nachricht | 0.38% | 3.75% | 59.26% | 59.54% |
| postcard | 3.39% | 15.28% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*0.74%\**</span> <span title="encode">*0.89%\**</span> | 7.20% | 68.57% | 63.78% |
| rkyv | 23.29% | <span title="unvalidated">*81.34%\**</span> <span title="validated">*80.74%\**</span> | 100.00% | 100.00% |
| rmp | 1.30% | 8.73% | 73.85% | 65.62% |
| ron | 0.12% | 0.42% | 27.04% | 47.32% |
| serde_json | 0.23% | 1.62% | 22.91% | 44.35% |
| speedy | 24.84% | 64.85% | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*58.52%\**</span> | <span title="unvalidated">*23.47%\**</span> | ‡ |
| capnp | <span title="progressive">*0.58%\**</span> | <span title="progressive">*0.59%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*47.51%\**</span> <span title="verified">*3.42%\**</span> | <span title="unverified">*27.01%\**</span> <span title="verified">*27.40%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*12.73%\**</span> | <span title="unvalidated">*99.06%\**</span> <span title="validated">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 383.10 us | <span title="unvalidated">*2.6055 ms\**</span> | 1290592 | 388633 |
| bincode | 803.41 us | 3.1804 ms | 569975 | 240897 |
| borsh | 758.62 us | 3.1243 ms | 446595 | 234395 |
| capnp | 845.98 us | † | 803896 | 336655 |
| cbor | 2.0812 ms | 8.3023 ms | 1109821 | 347562 |
| flatbuffers | 34.484 ms | † | 844168 | 346957 |
| nachricht | 5.1167 ms | 6.2947 ms | 535881 | 281994 |
| postcard | 860.41 us | 3.7330 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*6.3113 ms\**</span> <span title="encode">*3.0468 ms\**</span> | 5.3785 ms | 596811 | 306728 |
| rkyv | 558.81 us | <span title="unvalidated">*2.4896 ms\**</span> <span title="validated">*3.1572 ms\**</span> | 596952 | 241952 |
| rmp | 1.8635 ms | 4.4569 ms | 418025 | 244771 |
| ron | 10.357 ms | 18.576 ms | 1465229 | 439673 |
| serde_json | 4.6766 ms | 10.779 ms | 1623197 | 472162 |
| speedy | 551.24 us | 2.7749 ms | 449595 | 235136 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*46.794 us\**</span> | <span title="unvalidated">*47.994 us\**</span> | ‡ |
| capnp | <span title="progressive">*153.46 ns\**</span> | <span title="progressive">*5.3445 us\**</span> | ‡ |
| flatbuffers | <span title="unverified">*3.0560 ns\**</span> <span title="verified">*2.6606 ms\**</span> | <span title="unverified">*3.6759 us\**</span> <span title="verified">*2.6664 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4298 ns\**</span> <span title="validated">*648.17 us\**</span> | <span title="unvalidated">*178.41 ns\**</span> <span title="validated">*651.38 us\**</span> | 6.7311 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*95.55%\**</span> | 27.61% | 54.88% |
| bincode | 47.68% | 78.28% | 62.51% | 88.53% |
| borsh | 50.50% | 79.69% | 79.78% | 90.99% |
| capnp | 45.28% | † | 44.32% | 63.35% |
| cbor | 18.41% | 29.99% | 32.11% | 61.36% |
| flatbuffers | 1.11% | † | 42.21% | 61.47% |
| nachricht | 7.49% | 39.55% | 66.49% | 75.63% |
| postcard | 44.53% | 66.69% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*6.07%\**</span> <span title="encode">*12.57%\**</span> | 46.29% | 59.70% | 69.53% |
| rkyv | 68.56% | <span title="unvalidated">*100.00%\**</span> <span title="validated">*78.85%\**</span> | 59.69% | 88.15% |
| rmp | 20.56% | 55.86% | 85.24% | 87.13% |
| ron | 3.70% | 13.40% | 24.32% | 48.51% |
| serde_json | 8.19% | 23.10% | 21.95% | 45.17% |
| speedy | 69.50% | 89.72% | 79.25% | 90.70% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.37%\**</span> | ‡ |
| capnp | <span title="progressive">*0.93%\**</span> | <span title="progressive">*3.34%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*46.79%\**</span> <span title="verified">*0.00%\**</span> | <span title="unverified">*4.85%\**</span> <span title="verified">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.03%\**</span> | 100.00% |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
