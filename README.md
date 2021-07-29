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

## Last updated: 7/29/2021

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 237.86 us | <span title="unvalidated">*3.2487 ms\**</span> | 1705800 | 507148 |
| bincode | 527.80 us | 4.0328 ms | 1045784 | 374305 |
| borsh | 526.96 us | 3.8704 ms | 885780 | 363280 |
| capnp | 1.2341 ms | † | 1443216 | 509618 |
| cbor | 1.8670 ms | 8.2403 ms | 1407835 | 407372 |
| flatbuffers | 2.7526 ms | † | 1276368 | 469962 |
| nachricht | 4.5599 ms | 6.9329 ms | 926221 | 365209 |
| postcard | 719.88 us | 4.5856 ms | 765778 | 312739 |
| prost | <span title="populate + encode">*5.4255 ms\**</span> <span title="encode">*1.5852 ms\**</span> | 5.3388 ms | 764951 | 269811 |
| rkyv | 306.63 us | <span title="unvalidated">*3.2056 ms\**</span> <span title="validated upfront with error">*3.9919 ms\**</span> | 1011488 | 269353 |
| rmp | 1.5243 ms | 5.1389 ms | 784997 | 326654 |
| ron | 20.290 ms | 16.247 ms | 1607459 | 452648 |
| serde_json | 4.8639 ms | 9.7947 ms | 1827461 | 474358 |
| speedy | 268.85 us | 3.5334 ms | 885780 | 363280 |
| alkahest | 264.70 us | † | 1045784 | 454748 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*36.589 us\**</span> | <span title="unvalidated">*57.773 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*146.66 ns\**</span> | <span title="validated on-demand with error">*496.42 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9546 ns\**</span> <span title="validated upfront with error">*2.0092 ms\**</span> | <span title="unvalidated">*137.99 us\**</span> <span title="validated upfront with error">*2.1892 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3871 ns\**</span> <span title="validated upfront with error">*756.30 us\**</span> | <span title="unvalidated">*16.632 us\**</span> <span title="validated upfront with error">*776.72 us\**</span> | 66.600 us |
| alkahest | <span title="validated on-demand with panic">*2.0442 ns\**</span> | <span title="validated on-demand with panic">*81.230 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*98.67%\**</span> | 44.84% | 53.11% |
| bincode | 45.07% | 79.49% | 73.15% | 71.96% |
| borsh | 45.14% | 82.82% | 86.36% | 74.14% |
| capnp | 19.27% | † | 53.00% | 52.85% |
| cbor | 12.74% | 38.90% | 54.34% | 66.12% |
| flatbuffers | 8.64% | † | 59.93% | 57.31% |
| nachricht | 5.22% | 46.24% | 82.59% | 73.75% |
| postcard | 33.04% | 69.91% | 99.89% | 86.13% |
| prost | <span title="populate + encode">*4.38%\**</span> <span title="encode">*15.01%\**</span> | 60.04% | 100.00% | 99.83% |
| rkyv | 77.57% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*80.30%\**</span> | 75.63% | 100.00% |
| rmp | 15.60% | 62.38% | 97.45% | 82.46% |
| ron | 1.17% | 19.73% | 47.59% | 59.51% |
| serde_json | 4.89% | 32.73% | 41.86% | 56.78% |
| speedy | 88.47% | 90.72% | 86.36% | 74.14% |
| alkahest | 89.86% | † | 73.15% | 59.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*28.79%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.95%\**</span> | <span title="validated on-demand with error">*3.35%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.05%\**</span> <span title="validated upfront with error">*0.76%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.14%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*67.86%\**</span> | <span title="validated on-demand with panic">*20.48%\**</span> | ‡ |

## `mesh`

The data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 238.00 us | <span title="unvalidated">*1.5709 ms\**</span> | 6000024 | 5380836 |
| bincode | 7.1569 ms | 7.1089 ms | 6000008 | 5380823 |
| borsh | 6.3575 ms | 9.1960 ms | 6000004 | 5380818 |
| capnp | 21.372 ms | † | 14000088 | 6729881 |
| cbor | 43.461 ms | 63.095 ms | 13122324 | 7527423 |
| flatbuffers | 2.6383 ms | † | 6000024 | 5380800 |
| nachricht | 65.692 ms | 40.529 ms | 10125030 | 7160144 |
| postcard | 7.4290 ms | 9.9395 ms | 6000003 | 5380817 |
| prost | <span title="populate + encode">*34.207 ms\**</span> <span title="encode">*27.866 ms\**</span> | 21.499 ms | 8750000 | 6683814 |
| rkyv | 1.0296 ms | <span title="unvalidated">*1.8563 ms\**</span> <span title="validated upfront with error">*1.8722 ms\**</span> | 6000008 | 4263104 |
| rmp | 19.365 ms | 17.591 ms | 8125006 | 6496879 |
| ron | 199.29 ms | 279.11 ms | 22192885 | 9009575 |
| serde_json | 109.08 ms | 89.157 ms | 26192883 | 9612105 |
| speedy | 928.63 us | 2.3347 ms | 6000004 | 5380818 |
| alkahest | 700.77 us | † | 6000008 | 5380823 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.3509 ns\**</span> | <span title="unvalidated">*204.32 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*233.54 ns\**</span> | <span title="validated on-demand with error">*6.7789 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9247 ns\**</span> <span title="validated upfront with error">*40.962 ns\**</span> | <span title="unvalidated">*184.47 us\**</span> <span title="validated upfront with error">*186.86 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3646 ns\**</span> <span title="validated upfront with error">*10.994 ns\**</span> | <span title="unvalidated">*40.600 us\**</span> <span title="validated upfront with error">*40.870 us\**</span> | 624.86 us |
| alkahest | <span title="validated on-demand with panic">*2.0153 ns\**</span> | <span title="validated on-demand with panic">*74.309 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 79.23% |
| bincode | 3.33% | 22.10% | 100.00% | 79.23% |
| borsh | 3.74% | 17.08% | 100.00% | 79.23% |
| capnp | 1.11% | † | 42.86% | 63.35% |
| cbor | 0.55% | 2.49% | 45.72% | 56.63% |
| flatbuffers | 9.02% | † | 100.00% | 79.23% |
| nachricht | 0.36% | 3.88% | 59.26% | 59.54% |
| postcard | 3.20% | 15.80% | 100.00% | 79.23% |
| prost | <span title="populate + encode">*0.70%\**</span> <span title="encode">*0.85%\**</span> | 7.31% | 68.57% | 63.78% |
| rkyv | 23.12% | <span title="unvalidated">*84.63%\**</span> <span title="validated upfront with error">*83.91%\**</span> | 100.00% | 100.00% |
| rmp | 1.23% | 8.93% | 73.85% | 65.62% |
| ron | 0.12% | 0.56% | 27.04% | 47.32% |
| serde_json | 0.22% | 1.76% | 22.91% | 44.35% |
| speedy | 25.63% | 67.28% | 100.00% | 79.23% |
| alkahest | 33.96% | † | 100.00% | 79.23% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*58.05%\**</span> | <span title="unvalidated">*19.87%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.58%\**</span> | <span title="validated on-demand with error">*0.60%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.66%\**</span> <span title="validated upfront with error">*3.33%\**</span> | <span title="unvalidated">*22.01%\**</span> <span title="validated upfront with error">*21.73%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*12.41%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.34%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*67.71%\**</span> | <span title="validated on-demand with panic">*54.64%\**</span> | ‡ |

## `minecraft_savedata`

The data set is composed of Minecraft player saves that contain highly-structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 365.54 us | <span title="unvalidated">*2.4293 ms\**</span> | 1290592 | 389259 |
| bincode | 755.51 us | 3.0414 ms | 569975 | 240897 |
| borsh | 752.56 us | 2.9753 ms | 446595 | 234395 |
| capnp | 828.77 us | † | 803896 | 336655 |
| cbor | 2.0224 ms | 7.8268 ms | 1109821 | 347562 |
| flatbuffers | 33.323 ms | † | 844168 | 346957 |
| nachricht | 4.9511 ms | 6.0666 ms | 535881 | 281994 |
| postcard | 885.43 us | 3.6000 ms | 356311 | 213270 |
| prost | <span title="populate + encode">*6.0083 ms\**</span> <span title="encode">*2.8965 ms\**</span> | 5.2160 ms | 596811 | 306728 |
| rkyv | 538.85 us | <span title="unvalidated">*2.4413 ms\**</span> <span title="validated upfront with error">*3.0630 ms\**</span> | 596952 | 241952 |
| rmp | 1.8155 ms | 4.2848 ms | 418025 | 244771 |
| ron | 10.106 ms | 16.938 ms | 1465229 | 439673 |
| serde_json | 4.7927 ms | 10.320 ms | 1623197 | 472162 |
| speedy | 536.45 us | 2.6385 ms | 449595 | 235136 |
| alkahest | 416.27 us | † | 667570 | 325536 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*42.169 us\**</span> | <span title="unvalidated">*42.559 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*148.53 ns\**</span> | <span title="validated on-demand with error">*5.1845 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9109 ns\**</span> <span title="validated upfront with error">*2.5496 ms\**</span> | <span title="unvalidated">*3.5856 us\**</span> <span title="validated upfront with error">*2.5603 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3600 ns\**</span> <span title="validated upfront with error">*612.41 us\**</span> | <span title="unvalidated">*167.71 ns\**</span> <span title="validated upfront with error">*600.39 us\**</span> | 6.2397 us |
| alkahest | <span title="validated on-demand with panic">*2.0319 ns\**</span> | <span title="validated on-demand with panic">*31.299 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib |
|---|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 27.61% | 54.79% |
| bincode | 48.38% | 79.87% | 62.51% | 88.53% |
| borsh | 48.57% | 81.65% | 79.78% | 90.99% |
| capnp | 44.11% | † | 44.32% | 63.35% |
| cbor | 18.07% | 31.04% | 32.11% | 61.36% |
| flatbuffers | 1.10% | † | 42.21% | 61.47% |
| nachricht | 7.38% | 40.04% | 66.49% | 75.63% |
| postcard | 41.28% | 67.48% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*6.08%\**</span> <span title="encode">*12.62%\**</span> | 46.57% | 59.70% | 69.53% |
| rkyv | 67.84% | <span title="unvalidated">*99.51%\**</span> <span title="validated upfront with error">*79.31%\**</span> | 59.69% | 88.15% |
| rmp | 20.13% | 56.70% | 85.24% | 87.13% |
| ron | 3.62% | 14.34% | 24.32% | 48.51% |
| serde_json | 7.63% | 23.54% | 21.95% | 45.17% |
| speedy | 68.14% | 92.07% | 79.25% | 90.70% |
| alkahest | 87.81% | † | 53.37% | 65.51% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.39%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.92%\**</span> | <span title="validated on-demand with error">*3.23%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.72%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*4.68%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*66.93%\**</span> | <span title="validated on-demand with panic">*0.54%\**</span> | ‡ |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
