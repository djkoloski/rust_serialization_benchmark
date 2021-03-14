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
| abomonation | 296.84 us | † | 1705800 | 507159 |
| bincode | 571.32 us | 4.3227 ms | 1045784 | 374305 |
| borsh | 662.99 us | 4.0850 ms | 885780 | 363280 |
| capnp | 1.8191 ms | † | 1843240 | 537966 |
| cbor | 2.5207 ms | 8.9012 ms | 1407835 | 407372 |
| flatbuffers | 2.6539 ms | † | 1276368 | 469962 |
| nachricht | 4.7305 ms | 8.2779 ms | 926221 | 365209 |
| postcard | 681.17 us | 4.5506 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*5.4518 ms\**</span> <span title="encode">*1.5846 ms\**</span> | 4.9954 ms | 764951 | 269811 |
| rkyv | 430.28 us | 3.1559 ms | 1065784 | 333895 |
| rmp | 1.9255 ms | 5.3639 ms | 784997 | 326654 |
| serde_json | 4.3754 ms | 9.7525 ms | 1827461 | 474358 |
| speedy | 228.55 us | 3.3615 ms | 885780 | 363280 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | 37.243 us | 59.472 us | ‡ |
| capnp | <span title="progressive">*249.22 ns\**</span> | <span title="progressive">*717.69 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9856 ns\**</span> | <span title="unvalidated">*138.93 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3709 ns\**</span> <span title="validated">*1.4739 ms\**</span> | <span title="unvalidated">*18.593 us\**</span> <span title="validated">*1.5105 ms\**</span> | 70.076 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 76.99% | † | 44.84% | 53.20% |
| bincode | 40.00% | 73.01% | 73.15% | 72.08% |
| borsh | 34.47% | 77.26% | 86.36% | 74.27% |
| capnp | 12.56% | † | 41.50% | 50.15% |
| cbor | 9.07% | 35.45% | 54.34% | 66.23% |
| flatbuffers | 8.61% | † | 59.93% | 57.41% |
| nachricht | 4.83% | 38.12% | 82.59% | 73.88% |
| postcard | 33.55% | 69.35% | 99.89% | 86.27% |
| prost | <span title="populate + encode">*4.19%\**</span> <span title="encode">*14.42%\**</span> | 63.18% | 100.00% | 100.00% |
| rkyv | 53.12% | 100.00% | 71.77% | 80.81% |
| rmp | 11.87% | 58.84% | 97.45% | 82.60% |
| serde_json | 5.22% | 32.36% | 41.86% | 56.88% |
| speedy | 100.00% | 93.88% | 86.36% | 74.27% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | 0.00% | 31.26% | ‡ |
| capnp | <span title="progressive">*0.55%\**</span> | <span title="progressive">*2.59%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*45.92%\**</span> | <span title="unvalidated">*13.38%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.23%\**</span> | 100.00% |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 427.04 us | † | 6000024 | 5380836 |
| bincode | 6.7946 ms | 11.661 ms | 6000008 | 5380823 |
| borsh | 6.0806 ms | 8.9562 ms | 6000004 | 5380818 |
| capnp | 15.223 ms | † | 16000056 | 6780527 |
| cbor | 47.042 ms | 67.888 ms | 13122324 | 7527423 |
| flatbuffers | 1.9273 ms | † | 6000024 | 5380800 |
| nachricht | 74.686 ms | 59.577 ms | 10125030 | 7160144 |
| postcard | 6.3430 ms | 10.040 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*32.546 ms\**</span> <span title="encode">*27.934 ms\**</span> | 19.869 ms | 8750000 | 6683814 |
| rkyv | 1.1684 ms | 1.8822 ms | 6000008 | 4263104 |
| rmp | 21.623 ms | 24.912 ms | 8125006 | 6496879 |
| serde_json | 106.10 ms | 82.661 ms | 26192883 | 9612105 |
| speedy | 689.70 us | 2.2248 ms | 6000004 | 5380818 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | 2.3443 ns | 191.67 us | ‡ |
| capnp | <span title="progressive">*243.37 ns\**</span> | <span title="progressive">*9.0149 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9152 ns\**</span> | <span title="unvalidated">*186.15 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3414 ns\**</span> <span title="validated">*106.17 ns\**</span> | <span title="unvalidated">*182.54 us\**</span> <span title="validated">*190.35 us\**</span> | 646.96 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 100.00% | 79.23% |
| bincode | 6.28% | 16.14% | 100.00% | 79.23% |
| borsh | 7.02% | 21.02% | 100.00% | 79.23% |
| capnp | 2.81% | † | 37.50% | 62.87% |
| cbor | 0.91% | 2.77% | 45.72% | 56.63% |
| flatbuffers | 22.16% | † | 100.00% | 79.23% |
| nachricht | 0.57% | 3.16% | 59.26% | 59.54% |
| postcard | 6.73% | 18.75% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*1.31%\**</span> <span title="encode">*1.53%\**</span> | 9.47% | 68.57% | 63.78% |
| rkyv | 36.55% | 100.00% | 100.00% | 100.00% |
| rmp | 1.97% | 7.56% | 73.85% | 65.62% |
| serde_json | 0.40% | 2.28% | 22.91% | 44.35% |
| speedy | 61.92% | 84.60% | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | 57.22% | 95.24% | ‡ |
| capnp | <span title="progressive">*0.55%\**</span> | <span title="progressive">*2.02%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.01%\**</span> | <span title="unvalidated">*98.06%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.26%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*95.90%\**</span> | 100.00% |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 370.92 us | † | 1290592 | 392868 |
| bincode | 799.32 us | 3.2997 ms | 569975 | 240897 |
| borsh | 798.15 us | 3.0107 ms | 446595 | 234395 |
| capnp | 862.70 us | † | 835784 | 342099 |
| cbor | 2.6076 ms | 8.4790 ms | 1109821 | 347562 |
| flatbuffers | 36.830 ms | † | 849472 | 349208 |
| nachricht | 5.0234 ms | 7.1360 ms | 535881 | 281994 |
| postcard | 784.32 us | 3.7493 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*5.8932 ms\**</span> <span title="encode">*2.8476 ms\**</span> | 5.0972 ms | 596811 | 306728 |
| rkyv | 844.88 us | 2.4668 ms | 725176 | 333931 |
| rmp | 2.0772 ms | 4.8862 ms | 418025 | 244771 |
| serde_json | 4.4648 ms | 10.616 ms | 1623197 | 472162 |
| speedy | 471.69 us | 2.7482 ms | 449595 | 235136 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*41.511 us\**</span> | <span title="unvalidated">*42.284 us\**</span> | ‡ |
| capnp | <span title="progressive">*243.01 ns\**</span> | <span title="progressive">*5.5690 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9451 ns\**</span> | <span title="unvalidated">*3.5652 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3712 ns\**</span> <span title="validated">*1.7723 ms\**</span> | <span title="unvalidated">*275.27 ns\**</span> <span title="validated">*1.7747 ms\**</span> | 6.4037 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 27.61% | 54.29% |
| bincode | 46.40% | 74.76% | 62.51% | 88.53% |
| borsh | 46.47% | 81.93% | 79.78% | 90.99% |
| capnp | 43.00% | † | 42.63% | 62.34% |
| cbor | 14.22% | 29.09% | 32.11% | 61.36% |
| flatbuffers | 1.01% | † | 41.94% | 61.07% |
| nachricht | 7.38% | 34.57% | 66.49% | 75.63% |
| postcard | 47.29% | 65.79% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*6.29%\**</span> <span title="encode">*13.03%\**</span> | 48.40% | 59.70% | 69.53% |
| rkyv | 43.90% | 100.00% | 49.13% | 63.87% |
| rmp | 17.86% | 50.49% | 85.24% | 87.13% |
| serde_json | 8.31% | 23.24% | 21.95% | 45.17% |
| speedy | 78.64% | 89.76% | 79.25% | 90.70% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.65%\**</span> | ‡ |
| capnp | <span title="progressive">*0.56%\**</span> | <span title="progressive">*4.94%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.56%\**</span> | <span title="unvalidated">*7.72%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.02%\**</span> | 100.00% |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
