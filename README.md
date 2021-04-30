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

## Last updated: 4/30/2021

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 245.86 us | † | 1705800 | 507151 |
| bincode | 568.04 us | 4.2913 ms | 1045784 | 374305 |
| borsh | 529.84 us | 4.1123 ms | 885780 | 363280 |
| capnp | 1.0981 ms | † | 1443216 | 509618 |
| cbor | 1.8748 ms | 8.8985 ms | 1407835 | 407372 |
| flatbuffers | 2.7749 ms | † | 1276368 | 469962 |
| nachricht | 4.7300 ms | 8.0276 ms | 926221 | 365209 |
| postcard | 749.76 us | 4.6122 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*5.4242 ms\**</span> <span title="encode">*1.4954 ms\**</span> | 5.1980 ms | 764951 | 269811 |
| rkyv | 322.67 us | 3.2328 ms | 1065784 | 333895 |
| rmp | 1.6202 ms | 5.2989 ms | 784997 | 326654 |
| ron | 20.493 ms | 17.342 ms | 1607459 | 452648 |
| serde_json | 4.5972 ms | 9.7915 ms | 1827461 | 474358 |
| speedy | 276.19 us | 3.5432 ms | 885780 | 363280 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*36.914 us\**</span> | <span title="unvalidated">*59.278 us\**</span> | ‡ |
| capnp | <span title="progressive">*164.50 ns\**</span> | <span title="progressive">*537.14 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9521 ns\**</span> | <span title="unvalidated">*163.23 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3836 ns\**</span> <span title="validated">*1.2067 ms\**</span> | <span title="unvalidated">*18.803 us\**</span> <span title="validated">*1.2169 ms\**</span> | 67.701 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 44.84% | 53.20% |
| bincode | 43.28% | 75.33% | 73.15% | 72.08% |
| borsh | 46.40% | 78.61% | 86.36% | 74.27% |
| capnp | 22.39% | † | 53.00% | 52.94% |
| cbor | 13.11% | 36.33% | 54.34% | 66.23% |
| flatbuffers | 8.86% | † | 59.93% | 57.41% |
| nachricht | 5.20% | 40.27% | 82.59% | 73.88% |
| postcard | 32.79% | 70.09% | 99.89% | 86.27% |
| prost | <span title="populate + encode">*4.53%\**</span> <span title="encode">*16.44%\**</span> | 62.19% | 100.00% | 100.00% |
| rkyv | 76.20% | 100.00% | 71.77% | 80.81% |
| rmp | 15.17% | 61.01% | 97.45% | 82.60% |
| ron | 1.20% | 18.64% | 47.59% | 59.61% |
| serde_json | 5.35% | 33.02% | 41.86% | 56.88% |
| speedy | 89.02% | 91.24% | 86.36% | 74.27% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*31.72%\**</span> | ‡ |
| capnp | <span title="progressive">*0.84%\**</span> | <span title="progressive">*3.50%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.87%\**</span> | <span title="unvalidated">*11.52%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.55%\**</span> | 100.00% |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 475.70 us | † | 6000024 | 5380836 |
| bincode | 5.2860 ms | 8.0551 ms | 6000008 | 5380823 |
| borsh | 6.2602 ms | 9.1229 ms | 6000004 | 5380818 |
| capnp | 14.090 ms | † | 14000088 | 6729881 |
| cbor | 44.369 ms | 65.164 ms | 13122324 | 7527423 |
| flatbuffers | 1.9908 ms | † | 6000024 | 5380800 |
| nachricht | 74.643 ms | 60.644 ms | 10125030 | 7160144 |
| postcard | 6.9949 ms | 10.410 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*33.057 ms\**</span> <span title="encode">*28.284 ms\**</span> | 24.748 ms | 8750000 | 6683814 |
| rkyv | 935.15 us | 1.8861 ms | 6000008 | 4263104 |
| rmp | 21.851 ms | 21.253 ms | 8125006 | 6496879 |
| ron | 210.52 ms | 402.49 ms | 22192885 | 9009575 |
| serde_json | 111.47 ms | 86.980 ms | 26192883 | 9612105 |
| speedy | 841.08 us | 2.3470 ms | 6000004 | 5380818 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.3558 ns\**</span> | <span title="unvalidated">*186.29 us\**</span> | ‡ |
| capnp | <span title="progressive">*248.37 ns\**</span> | <span title="progressive">*7.6474 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9233 ns\**</span> | <span title="unvalidated">*150.71 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3528 ns\**</span> <span title="validated">*98.728 ns\**</span> | <span title="unvalidated">*195.20 us\**</span> <span title="validated">*198.16 us\**</span> | 617.94 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 100.00% | 79.23% |
| bincode | 9.00% | 23.41% | 100.00% | 79.23% |
| borsh | 7.60% | 20.67% | 100.00% | 79.23% |
| capnp | 3.38% | † | 42.86% | 63.35% |
| cbor | 1.07% | 2.89% | 45.72% | 56.63% |
| flatbuffers | 23.89% | † | 100.00% | 79.23% |
| nachricht | 0.64% | 3.11% | 59.26% | 59.54% |
| postcard | 6.80% | 18.12% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*1.44%\**</span> <span title="encode">*1.68%\**</span> | 7.62% | 68.57% | 63.78% |
| rkyv | 50.87% | 100.00% | 100.00% | 100.00% |
| rmp | 2.18% | 8.87% | 73.85% | 65.62% |
| ron | 0.23% | 0.47% | 27.04% | 47.32% |
| serde_json | 0.43% | 2.17% | 22.91% | 44.35% |
| speedy | 56.56% | 80.36% | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*57.42%\**</span> | <span title="unvalidated">*80.90%\**</span> | ‡ |
| capnp | <span title="progressive">*0.54%\**</span> | <span title="progressive">*1.97%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.28%\**</span> | <span title="unvalidated">*100.00%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*1.37%\**</span> | <span title="unvalidated">*77.21%\**</span> <span title="validated">*76.05%\**</span> | 100.00% |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 348.87 us | † | 1290592 | 389684 |
| bincode | 745.96 us | 3.2118 ms | 569975 | 240897 |
| borsh | 732.26 us | 2.9677 ms | 446595 | 234395 |
| capnp | 824.75 us | † | 803896 | 336655 |
| cbor | 1.9978 ms | 8.2028 ms | 1109821 | 347562 |
| flatbuffers | 36.026 ms | † | 849472 | 349208 |
| nachricht | 5.1774 ms | 7.1682 ms | 535881 | 281994 |
| postcard | 858.99 us | 3.7483 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*5.8802 ms\**</span> <span title="encode">*2.7872 ms\**</span> | 5.2291 ms | 596811 | 306728 |
| rkyv | 673.84 us | 2.3965 ms | 725176 | 327770 |
| rmp | 1.8697 ms | 4.4598 ms | 418025 | 244771 |
| ron | 10.450 ms | 18.823 ms | 1465229 | 439673 |
| serde_json | 4.6952 ms | 10.238 ms | 1623197 | 472162 |
| speedy | 530.11 us | 2.7056 ms | 449595 | 235136 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*44.003 us\**</span> | <span title="unvalidated">*45.019 us\**</span> | ‡ |
| capnp | <span title="progressive">*159.34 ns\**</span> | <span title="progressive">*5.3044 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.8555 ns\**</span> | <span title="unvalidated">*3.4601 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3729 ns\**</span> <span title="validated">*1.2781 ms\**</span> | <span title="unvalidated">*265.76 ns\**</span> <span title="validated">*1.2777 ms\**</span> | 6.2807 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | † | 27.61% | 54.73% |
| bincode | 46.77% | 74.62% | 62.51% | 88.53% |
| borsh | 47.64% | 80.75% | 79.78% | 90.99% |
| capnp | 42.30% | † | 44.32% | 63.35% |
| cbor | 17.46% | 29.22% | 32.11% | 61.36% |
| flatbuffers | 0.97% | † | 41.94% | 61.07% |
| nachricht | 6.74% | 33.43% | 66.49% | 75.63% |
| postcard | 40.61% | 63.94% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*5.93%\**</span> <span title="encode">*12.52%\**</span> | 45.83% | 59.70% | 69.53% |
| rkyv | 51.77% | 100.00% | 49.13% | 65.07% |
| rmp | 18.66% | 53.74% | 85.24% | 87.13% |
| ron | 3.34% | 12.73% | 24.32% | 48.51% |
| serde_json | 7.43% | 23.41% | 21.95% | 45.17% |
| speedy | 65.81% | 88.58% | 79.25% | 90.70% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.59%\**</span> | ‡ |
| capnp | <span title="progressive">*0.86%\**</span> | <span title="progressive">*5.01%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*48.08%\**</span> | <span title="unvalidated">*7.68%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated">*0.02%\**</span> | 100.00% |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
