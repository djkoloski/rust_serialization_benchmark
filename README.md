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
| abomonation | 238.13 us | <span title="unvalidated">*3.0620 ms\**</span> | 1705800 | 515079 | 406294 |
| bincode | 503.32 us | 3.8133 ms | 1045784 | 374305 | 312658 |
| borsh | 487.53 us | 3.7019 ms | 885780 | 363280 | 286948 |
| capnp | 1.4651 ms | † | 1443216 | 509618 | 429784 |
| cbor | 1.8000 ms | 7.8975 ms | 1407835 | 407372 | 325016 |
| flatbuffers | 2.5576 ms | † | 1276368 | 469962 | 389885 |
| nachricht | 4.1988 ms | 6.6114 ms | 926221 | 365209 | 304177 |
| postcard | 669.02 us | 4.2904 ms | 765778 | 312739 | 265462 |
| prost | <span title="populate + encode">*4.3176 ms\**</span> <span title="encode">*802.45 us\**</span> | 4.9419 ms | 764951 | 269811 | 228807 |
| rkyv | 284.19 us | <span title="unvalidated">*4.3698 ms\**</span> <span title="validated upfront with error">*4.5608 ms\**</span> | 1011488 | 269353 | 209292 |
| rmp | 1.8184 ms | 5.2791 ms | 784997 | 326654 | 278933 |
| ron | 25.158 ms | 17.756 ms | 1607459 | 452648 | 351604 |
| serde_json | 5.0376 ms | 10.132 ms | 1827461 | 474358 | 364177 |
| speedy | 287.19 us | 3.3568 ms | 885780 | 363280 | 286948 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*50.440 us\**</span> | <span title="unvalidated">*77.612 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*162.82 ns\**</span> | <span title="validated on-demand with error">*582.53 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.5706 ns\**</span> <span title="validated upfront with error">*2.0693 ms\**</span> | <span title="unvalidated">*142.61 us\**</span> <span title="validated upfront with error">*2.2841 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3938 ns\**</span> <span title="validated upfront with error">*673.18 us\**</span> | <span title="unvalidated">*15.922 us\**</span> <span title="validated upfront with error">*894.41 us\**</span> | 65.606 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 44.84% | 52.29% | 51.51% |
| bincode | 47.31% | 80.30% | 73.15% | 71.96% | 66.94% |
| borsh | 48.84% | 82.71% | 86.36% | 74.14% | 72.94% |
| capnp | 16.25% | † | 53.00% | 52.85% | 48.70% |
| cbor | 13.23% | 38.77% | 54.34% | 66.12% | 64.39% |
| flatbuffers | 9.31% | † | 59.93% | 57.31% | 53.68% |
| nachricht | 5.67% | 46.31% | 82.59% | 73.75% | 68.81% |
| postcard | 35.59% | 71.37% | 99.89% | 86.13% | 78.84% |
| prost | <span title="populate + encode">*5.52%\**</span> <span title="encode">*29.68%\**</span> | 61.96% | 100.00% | 99.83% | 91.47% |
| rkyv | 83.79% | <span title="unvalidated">*70.07%\**</span> <span title="validated upfront with error">*67.14%\**</span> | 75.63% | 100.00% | 100.00% |
| rmp | 13.10% | 58.00% | 97.45% | 82.46% | 75.03% |
| ron | 0.95% | 17.24% | 47.59% | 59.51% | 59.52% |
| serde_json | 4.73% | 30.22% | 41.86% | 56.78% | 57.47% |
| speedy | 82.92% | 91.22% | 86.36% | 74.14% | 72.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*20.51%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.86%\**</span> | <span title="validated on-demand with error">*2.73%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*54.22%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.16%\**</span> <span title="validated upfront with error">*0.70%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.78%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 378.32 us | <span title="unvalidated">*1.7598 ms\**</span> | 6000024 | 5380836 | 5345893 |
| bincode | 6.2870 ms | 8.4397 ms | 6000008 | 5380823 | 5345885 |
| borsh | 7.1556 ms | 9.2858 ms | 6000004 | 5380818 | 5345888 |
| capnp | 23.198 ms | † | 14000088 | 6729881 | 6051413 |
| cbor | 44.880 ms | 71.414 ms | 13122324 | 7527423 | 6761276 |
| flatbuffers | 2.5647 ms | † | 6000024 | 5380800 | 5345925 |
| nachricht | 70.778 ms | 40.043 ms | 10125030 | 7160144 | 6777986 |
| postcard | 7.3187 ms | 11.549 ms | 6000003 | 5380817 | 5345893 |
| prost | <span title="populate + encode">*21.699 ms\**</span> <span title="encode">*15.526 ms\**</span> | 18.932 ms | 8750000 | 6683814 | 6421721 |
| rkyv | 1.0714 ms | <span title="unvalidated">*1.9899 ms\**</span> <span title="validated upfront with error">*1.9549 ms\**</span> | 6000008 | 4263104 | 4118396 |
| rmp | 21.369 ms | 19.844 ms | 8125006 | 6496879 | 6391025 |
| ron | 233.79 ms | 425.16 ms | 22192885 | 9009575 | 8147524 |
| serde_json | 118.76 ms | 93.573 ms | 26192883 | 9612105 | 8595089 |
| speedy | 759.24 us | 2.2069 ms | 6000004 | 5380818 | 5345888 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.5740 ns\**</span> | <span title="unvalidated">*182.95 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*272.78 ns\**</span> | <span title="validated on-demand with error">*8.5077 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.7734 ns\**</span> <span title="validated upfront with error">*45.761 ns\**</span> | <span title="unvalidated">*147.77 us\**</span> <span title="validated upfront with error">*184.53 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4814 ns\**</span> <span title="validated upfront with error">*15.178 ns\**</span> | <span title="unvalidated">*38.461 us\**</span> <span title="validated upfront with error">*37.184 us\**</span> | 560.01 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 79.23% | 77.04% |
| bincode | 6.02% | 20.85% | 100.00% | 79.23% | 77.04% |
| borsh | 5.29% | 18.95% | 100.00% | 79.23% | 77.04% |
| capnp | 1.63% | † | 42.86% | 63.35% | 68.06% |
| cbor | 0.84% | 2.46% | 45.72% | 56.63% | 60.91% |
| flatbuffers | 14.75% | † | 100.00% | 79.23% | 77.04% |
| nachricht | 0.53% | 4.39% | 59.26% | 59.54% | 60.76% |
| postcard | 5.17% | 15.24% | 100.00% | 79.23% | 77.04% |
| prost | <span title="populate + encode">*1.74%\**</span> <span title="encode">*2.44%\**</span> | 9.30% | 68.57% | 63.78% | 64.13% |
| rkyv | 35.31% | <span title="unvalidated">*88.44%\**</span> <span title="validated upfront with error">*90.02%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 1.77% | 8.87% | 73.85% | 65.62% | 64.44% |
| ron | 0.16% | 0.41% | 27.04% | 47.32% | 50.55% |
| serde_json | 0.32% | 1.88% | 22.91% | 44.35% | 47.92% |
| speedy | 49.83% | 79.74% | 100.00% | 79.23% | 77.04% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*57.55%\**</span> | <span title="unvalidated">*20.32%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.54%\**</span> | <span title="validated on-demand with error">*0.44%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*53.41%\**</span> <span title="validated upfront with error">*3.24%\**</span> | <span title="unvalidated">*25.16%\**</span> <span title="validated upfront with error">*20.15%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.76%\**</span> | <span title="unvalidated">*96.68%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 350.58 us | <span title="unvalidated">*2.3909 ms\**</span> | 1290592 | 391564 | 330298 |
| bincode | 700.63 us | 3.0317 ms | 569975 | 240897 | 232904 |
| borsh | 684.69 us | 2.9562 ms | 446595 | 234395 | 210623 |
| capnp | 824.57 us | † | 803896 | 336655 | 281557 |
| cbor | 2.0083 ms | 7.6353 ms | 1109821 | 347562 | 275184 |
| flatbuffers | 32.945 ms | † | 844168 | 346957 | 295321 |
| nachricht | 6.8281 ms | 6.0832 ms | 535881 | 281994 | 250096 |
| postcard | 797.33 us | 3.5766 ms | 356311 | 213270 | 199012 |
| prost | <span title="populate + encode">*4.6071 ms\**</span> <span title="encode">*1.6830 ms\**</span> | 4.9262 ms | 596811 | 306728 | 270153 |
| rkyv | 482.04 us | <span title="unvalidated">*2.1921 ms\**</span> <span title="validated upfront with error">*2.7561 ms\**</span> | 596952 | 241952 | 208634 |
| rmp | 1.7659 ms | 4.2343 ms | 418025 | 244771 | 225969 |
| ron | 11.007 ms | 17.413 ms | 1465229 | 439673 | 345438 |
| serde_json | 4.3134 ms | 27.489 ms | 1623197 | 472162 | 361701 |
| speedy | 445.41 us | 2.5104 ms | 449595 | 235136 | 210904 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*54.133 us\**</span> | <span title="unvalidated">*55.392 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*164.90 ns\**</span> | <span title="validated on-demand with error">*5.6618 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.5801 ns\**</span> <span title="validated upfront with error">*2.5865 ms\**</span> | <span title="unvalidated">*3.8691 us\**</span> <span title="validated upfront with error">*2.5757 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4017 ns\**</span> <span title="validated upfront with error">*534.35 us\**</span> | <span title="unvalidated">*158.39 ns\**</span> <span title="validated upfront with error">*535.21 us\**</span> | 4.6731 us |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*91.69%\**</span> | 27.61% | 54.47% | 60.25% |
| bincode | 50.04% | 72.31% | 62.51% | 88.53% | 85.45% |
| borsh | 51.20% | 74.15% | 79.78% | 90.99% | 94.49% |
| capnp | 42.52% | † | 44.32% | 63.35% | 70.68% |
| cbor | 17.46% | 28.71% | 32.11% | 61.36% | 72.32% |
| flatbuffers | 1.06% | † | 42.21% | 61.47% | 67.39% |
| nachricht | 5.13% | 36.04% | 66.49% | 75.63% | 79.57% |
| postcard | 43.97% | 61.29% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*7.61%\**</span> <span title="encode">*20.83%\**</span> | 44.50% | 59.70% | 69.53% | 73.67% |
| rkyv | 72.73% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.54%\**</span> | 59.69% | 88.15% | 95.39% |
| rmp | 19.85% | 51.77% | 85.24% | 87.13% | 88.07% |
| ron | 3.19% | 12.59% | 24.32% | 48.51% | 57.61% |
| serde_json | 8.13% | 7.97% | 21.95% | 45.17% | 55.02% |
| speedy | 78.71% | 87.32% | 79.25% | 90.70% | 94.36% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.29%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.85%\**</span> | <span title="validated on-demand with error">*2.80%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*54.33%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*4.09%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
