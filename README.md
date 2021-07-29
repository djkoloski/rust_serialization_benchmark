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

## Last updated: 7/21/2021

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 151.15 us | <span title="unvalidated">*2.1461 ms\**</span> | 1705800 | 507150 |
| bincode | 340.71 us | 2.8175 ms | 1045784 | 374305 |
| borsh | 339.60 us | 2.7603 ms | 885780 | 363280 |
| capnp | 876.17 us | † | 1443216 | 509618 |
| cbor | 1.2811 ms | 5.7277 ms | 1407835 | 407372 |
| flatbuffers | 1.8810 ms | † | 1276368 | 469962 |
| nachricht | 3.0369 ms | 4.8450 ms | 926221 | 365209 |
| postcard | 480.20 us | 3.0996 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*3.7203 ms\**</span> <span title="encode">*1.0249 ms\**</span> | 3.5563 ms | 764951 | 269811 |
| rkyv | 187.63 us | <span title="unvalidated">*2.1425 ms\**</span> <span title="validated">*2.6788 ms\**</span> | 1011488 | 269353 |
| rmp | 1.0057 ms | 3.5469 ms | 784997 | 326654 |
| ron | 14.307 ms | 11.560 ms | 1607459 | 452648 |
| serde_json | 3.1042 ms | 6.5873 ms | 1827461 | 474358 |
| speedy | 156.73 us | 2.2675 ms | 885780 | 363280 |
| alkahest | 153.21 us | † | 1045784 | 454748 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*26.252 us\**</span> | <span title="unvalidated">*44.825 us\**</span> | ‡ |
| capnp | <span title="progressive">*106.18 ns\**</span> | <span title="progressive">*357.63 us\**</span> | ‡ |
| flatbuffers | <span title="unverified">*2.0034 ns\**</span> <span title="verified">*1.4044 ms\**</span> | <span title="unverified">*98.447 us\**</span> <span title="verified">*1.5116 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.0289 ns\**</span> <span title="validated">*504.84 us\**</span> | <span title="unvalidated">*12.414 us\**</span> <span title="validated">*516.76 us\**</span> | 30.239 us |
| alkahest | <span title="validated">*1.4493 ns\**</span> | <span title="validated">*52.704 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*99.83%\**</span> | 44.84% | 53.11% |
| bincode | 44.36% | 76.04% | 73.15% | 71.96% |
| borsh | 44.51% | 77.62% | 86.36% | 74.14% |
| capnp | 17.25% | † | 53.00% | 52.85% |
| cbor | 11.80% | 37.41% | 54.34% | 66.12% |
| flatbuffers | 8.04% | † | 59.93% | 57.31% |
| nachricht | 4.98% | 44.22% | 82.59% | 73.75% |
| postcard | 31.48% | 69.12% | 99.89% | 86.13% |
| prost | <span title="populate + encode">*4.06%\**</span> <span title="encode">*14.75%\**</span> | 60.25% | 100.00% | 99.83% |
| rkyv | 80.56% | <span title="unvalidated">*100.00%\**</span> <span title="validated">*79.98%\**</span> | 75.63% | 100.00% |
| rmp | 15.03% | 60.40% | 97.45% | 82.46% |
| ron | 1.06% | 18.53% | 47.59% | 59.51% |
| serde_json | 4.87% | 32.52% | 41.86% | 56.78% |
| speedy | 96.44% | 94.49% | 86.36% | 74.14% |
| alkahest | 98.66% | † | 73.15% | 59.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*27.69%\**</span> | ‡ |
| capnp | <span title="progressive">*0.97%\**</span> | <span title="progressive">*3.47%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*51.36%\**</span> <span title="verified">*0.00%\**</span> | <span title="unverified">*12.61%\**</span> <span title="verified">*0.82%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*2.40%\**</span> | 100.00% |
| alkahest | <span title="validated">*70.99%\**</span> | <span title="validated">*23.55%\**</span> | ‡ |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 186.51 us | <span title="unvalidated">*982.47 us\**</span> | 6000024 | 5380836 |
| bincode | 3.0766 ms | 4.8567 ms | 6000008 | 5380823 |
| borsh | 3.7069 ms | 6.2125 ms | 6000004 | 5380818 |
| capnp | 15.773 ms | † | 14000088 | 6729881 |
| cbor | 29.037 ms | 41.508 ms | 13122324 | 7527423 |
| flatbuffers | 1.8078 ms | † | 6000024 | 5380800 |
| nachricht | 42.883 ms | 26.069 ms | 10125030 | 7160144 |
| postcard | 4.3587 ms | 6.6918 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*22.245 ms\**</span> <span title="encode">*18.290 ms\**</span> | 12.784 ms | 8750000 | 6683814 |
| rkyv | 602.79 us | <span title="unvalidated">*1.3591 ms\**</span> <span title="validated">*1.3844 ms\**</span> | 6000008 | 4263104 |
| rmp | 12.338 ms | 11.812 ms | 8125006 | 6496879 |
| ron | 143.69 ms | 186.41 ms | 22192885 | 9009575 |
| serde_json | 72.967 ms | 61.678 ms | 26192883 | 9612105 |
| speedy | 538.89 us | 1.5064 ms | 6000004 | 5380818 |
| alkahest | 482.67 us | † | 6000008 | 5380823 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*1.5547 ns\**</span> | <span title="unvalidated">*123.20 us\**</span> | ‡ |
| capnp | <span title="progressive">*585.88 ns\**</span> | <span title="progressive">*4.9779 ms\**</span> | ‡ |
| flatbuffers | <span title="unverified">*2.0062 ns\**</span> <span title="verified">*29.167 ns\**</span> | <span title="unverified">*107.14 us\**</span> <span title="verified">*107.40 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.0301 ns\**</span> <span title="validated">*6.1047 ns\**</span> | <span title="unvalidated">*28.236 us\**</span> <span title="validated">*26.848 us\**</span> | 222.17 us |
| alkahest | <span title="validated">*1.4755 ns\**</span> | <span title="validated">*53.546 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 79.23% |
| bincode | 6.06% | 20.23% | 100.00% | 79.23% |
| borsh | 5.03% | 15.81% | 100.00% | 79.23% |
| capnp | 1.18% | † | 42.86% | 63.35% |
| cbor | 0.64% | 2.37% | 45.72% | 56.63% |
| flatbuffers | 10.32% | † | 100.00% | 79.23% |
| nachricht | 0.43% | 3.77% | 59.26% | 59.54% |
| postcard | 4.28% | 14.68% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*0.84%\**</span> <span title="encode">*1.02%\**</span> | 7.69% | 68.57% | 63.78% |
| rkyv | 30.94% | <span title="unvalidated">*72.29%\**</span> <span title="validated">*70.97%\**</span> | 100.00% | 100.00% |
| rmp | 1.51% | 8.32% | 73.85% | 65.62% |
| ron | 0.13% | 0.53% | 27.04% | 47.32% |
| serde_json | 0.26% | 1.59% | 22.91% | 44.35% |
| speedy | 34.61% | 65.22% | 100.00% | 79.23% |
| alkahest | 38.64% | † | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*66.26%\**</span> | <span title="unvalidated">*21.79%\**</span> | ‡ |
| capnp | <span title="progressive">*0.18%\**</span> | <span title="progressive">*0.54%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*51.35%\**</span> <span title="verified">*3.53%\**</span> | <span title="unverified">*25.06%\**</span> <span title="verified">*25.00%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*16.87%\**</span> | <span title="unvalidated">*95.08%\**</span> <span title="validated">*100.00%\**</span> | 100.00% |
| alkahest | <span title="validated">*69.81%\**</span> | <span title="validated">*50.14%\**</span> | ‡ |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 242.68 us | <span title="unvalidated">*1.6726 ms\**</span> | 1290592 | 389149 |
| bincode | 501.22 us | 2.2141 ms | 569975 | 240897 |
| borsh | 495.80 us | 2.0383 ms | 446595 | 234395 |
| capnp | 585.55 us | † | 803896 | 336655 |
| cbor | 1.3653 ms | 5.4501 ms | 1109821 | 347562 |
| flatbuffers | 24.314 ms | † | 844168 | 346957 |
| nachricht | 3.3741 ms | 4.1833 ms | 535881 | 281994 |
| postcard | 552.10 us | 2.4839 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*4.0777 ms\**</span> <span title="encode">*1.9348 ms\**</span> | 3.6414 ms | 596811 | 306728 |
| rkyv | 359.50 us | <span title="unvalidated">*1.6110 ms\**</span> <span title="validated">*2.0056 ms\**</span> | 596952 | 241952 |
| rmp | 1.2112 ms | 2.8606 ms | 418025 | 244771 |
| ron | 7.0528 ms | 11.836 ms | 1465229 | 439673 |
| serde_json | 3.1457 ms | 6.9588 ms | 1623197 | 472162 |
| speedy | 327.52 us | 1.7337 ms | 449595 | 235136 |
| alkahest | 261.19 us | † | 667570 | 325536 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*30.977 us\**</span> | <span title="unvalidated">*31.684 us\**</span> | ‡ |
| capnp | <span title="progressive">*105.16 ns\**</span> | <span title="progressive">*3.7481 us\**</span> | ‡ |
| flatbuffers | <span title="unverified">*2.0052 ns\**</span> <span title="verified">*1.7797 ms\**</span> | <span title="unverified">*2.5750 us\**</span> <span title="verified">*1.7853 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.0247 ns\**</span> <span title="validated">*403.28 us\**</span> | <span title="unvalidated">*118.70 ns\**</span> <span title="validated">*393.12 us\**</span> | 2.0469 us |
| alkahest | <span title="validated">*1.4474 ns\**</span> | <span title="validated">*21.985 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*96.32%\**</span> | 27.61% | 54.80% |
| bincode | 48.42% | 72.76% | 62.51% | 88.53% |
| borsh | 48.95% | 79.04% | 79.78% | 90.99% |
| capnp | 41.44% | † | 44.32% | 63.35% |
| cbor | 17.77% | 29.56% | 32.11% | 61.36% |
| flatbuffers | 1.00% | † | 42.21% | 61.47% |
| nachricht | 7.19% | 38.51% | 66.49% | 75.63% |
| postcard | 43.96% | 64.86% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*5.95%\**</span> <span title="encode">*12.54%\**</span> | 44.24% | 59.70% | 69.53% |
| rkyv | 67.50% | <span title="unvalidated">*100.00%\**</span> <span title="validated">*80.33%\**</span> | 59.69% | 88.15% |
| rmp | 20.04% | 56.32% | 85.24% | 87.13% |
| ron | 3.44% | 13.61% | 24.32% | 48.51% |
| serde_json | 7.71% | 23.15% | 21.95% | 45.17% |
| speedy | 74.10% | 92.92% | 79.25% | 90.70% |
| alkahest | 92.91% | † | 53.37% | 65.51% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.37%\**</span> | ‡ |
| capnp | <span title="progressive">*0.97%\**</span> | <span title="progressive">*3.17%\**</span> | ‡ |
| flatbuffers | <span title="unverified">*51.10%\**</span> <span title="verified">*0.00%\**</span> | <span title="unverified">*4.61%\**</span> <span title="verified">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated">*70.80%\**</span> | <span title="validated">*0.54%\**</span> | ‡ |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
