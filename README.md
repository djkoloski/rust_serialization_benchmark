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

## Last updated: 3/14/2022

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 251.48 us | <span title="unvalidated">*3.0824 ms\**</span> | 1705800 | 515094 | 406212 |
| bare | 784.25 us | 4.3747 ms | 765778 | 312739 | 265462 |
| bincode | 876.22 us | 5.2536 ms | 1045784 | 374305 | 312658 |
| borsh | 535.59 us | 4.1569 ms | 885780 | 363280 | 286948 |
| bson | 3.9309 ms | 12.097 ms | 1924682 | 537661 | 377489 |
| capnp | 1.2680 ms | † | 1443216 | 509618 | 429784 |
| cbor | 2.0621 ms | 7.7788 ms | 1407835 | 407372 | 325016 |
| flatbuffers | 1.8381 ms | † | 1276368 | 469962 | 389885 |
| nachricht | 4.3497 ms | 6.9867 ms | 926221 | 365209 | 304177 |
| postcard | 675.07 us | 4.1708 ms | 765778 | 312739 | 265462 |
| prost | <span title="populate + encode">*4.4448 ms\**</span> <span title="encode">*729.83 us\**</span> | 8.8590 ms | 764951 | 269811 | 228807 |
| rkyv | 291.09 us | <span title="unvalidated">*2.8809 ms\**</span> <span title="validated upfront with error">*3.5597 ms\**</span> | 1011488 | 392809 | 332689 |
| rmp | 1.4057 ms | 4.9996 ms | 784997 | 326654 | 278933 |
| ron | 19.620 ms | 16.320 ms | 1607459 | 452648 | 351604 |
| serde_json | 4.2616 ms | 9.4564 ms | 1827461 | 474358 | 364177 |
| simd-json | 4.1413 ms | 8.9223 ms | 1827461 | 474358 | 364177 |
| speedy | 228.56 us | 3.2595 ms | 885780 | 363280 | 286948 |
| alkahest | 222.43 us | † | 1045784 | 454748 | 389910 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*38.632 us\**</span> | <span title="unvalidated">*57.425 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*154.13 ns\**</span> | <span title="validated on-demand with error">*690.86 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.9151 ns\**</span> <span title="validated upfront with error">*2.3605 ms\**</span> | <span title="unvalidated">*133.02 us\**</span> <span title="validated upfront with error">*2.5115 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4278 ns\**</span> <span title="validated upfront with error">*700.18 us\**</span> | <span title="unvalidated">*15.933 us\**</span> <span title="validated upfront with error">*719.59 us\**</span> | 67.792 us |
| alkahest | <span title="validated on-demand with panic">*2.0297 ns\**</span> | <span title="validated on-demand with panic">*75.394 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 88.45% | <span title="unvalidated">*93.46%\**</span> | 44.84% | 52.38% | 56.33% |
| bare | 28.36% | 65.85% | 99.89% | 86.27% | 86.19% |
| bincode | 25.39% | 54.84% | 73.15% | 72.08% | 73.18% |
| borsh | 41.53% | 69.30% | 86.36% | 74.27% | 79.74% |
| bson | 5.66% | 23.81% | 39.74% | 50.18% | 60.61% |
| capnp | 17.54% | † | 53.00% | 52.94% | 53.24% |
| cbor | 10.79% | 37.04% | 54.34% | 66.23% | 70.40% |
| flatbuffers | 12.10% | † | 59.93% | 57.41% | 58.69% |
| nachricht | 5.11% | 41.23% | 82.59% | 73.88% | 75.22% |
| postcard | 32.95% | 69.07% | 99.89% | 86.27% | 86.19% |
| prost | <span title="populate + encode">*5.00%\**</span> <span title="encode">*30.48%\**</span> | 32.52% | 100.00% | 100.00% | 100.00% |
| rkyv | 76.41% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*80.93%\**</span> | 75.63% | 68.69% | 68.78% |
| rmp | 15.82% | 57.62% | 97.45% | 82.60% | 82.03% |
| ron | 1.13% | 17.65% | 47.59% | 59.61% | 65.08% |
| serde_json | 5.22% | 30.47% | 41.86% | 56.88% | 62.83% |
| simd-json | 5.37% | 32.29% | 41.86% | 56.88% | 62.83% |
| speedy | 97.32% | 88.38% | 86.36% | 74.27% | 79.74% |
| alkahest | 100.00% | † | 73.15% | 59.33% | 58.68% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*27.75%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.93%\**</span> | <span title="validated on-demand with error">*2.31%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*48.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.98%\**</span> <span title="validated upfront with error">*0.63%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.21%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*70.35%\**</span> | <span title="validated on-demand with panic">*21.13%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 258.10 us | <span title="unvalidated">*1.5652 ms\**</span> | 6000024 | 5380837 | 5345893 |
| bare | 4.6105 ms | 11.967 ms | 6000003 | 5380817 | 5345893 |
| bincode | 6.8703 ms | 9.6184 ms | 6000008 | 5380823 | 5345885 |
| borsh | 5.8235 ms | 5.7295 ms | 6000004 | 5380818 | 5345888 |
| bson | 94.660 ms | 184.58 ms | 23013911 | 9211138 | 7501230 |
| capnp | 29.209 ms | † | 14000088 | 6729881 | 6051413 |
| cbor | 70.670 ms | 69.791 ms | 13122324 | 7527423 | 6761276 |
| flatbuffers | 2.8184 ms | † | 6000024 | 5380800 | 5345925 |
| nachricht | 96.749 ms | 57.486 ms | 10125030 | 7160144 | 6777986 |
| postcard | 8.3022 ms | 12.873 ms | 6000003 | 5380817 | 5345893 |
| prost | <span title="populate + encode">*21.239 ms\**</span> <span title="encode">*13.465 ms\**</span> | 22.087 ms | 8750000 | 6683814 | 6421721 |
| rkyv | 1.0816 ms | <span title="unvalidated">*2.2639 ms\**</span> <span title="validated upfront with error">*2.3904 ms\**</span> | 6000008 | 5380822 | 5345887 |
| rmp | 19.067 ms | 29.544 ms | 8125006 | 6496879 | 6391025 |
| ron | 242.39 ms | 313.36 ms | 22192885 | 9009575 | 8147524 |
| serde_json | 117.74 ms | 114.43 ms | 26192883 | 9612105 | 8595089 |
| simd-json | 118.11 ms | 146.50 ms | 39152823 | 16587283 | 14555990 |
| speedy | 900.34 us | 2.8591 ms | 6000004 | 5380818 | 5345888 |
| alkahest | 798.73 us | † | 6000008 | 5380823 | 5345885 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.3582 ns\**</span> | <span title="unvalidated">*182.96 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*366.46 ns\**</span> | <span title="validated on-demand with error">*11.676 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.3395 ns\**</span> <span title="validated upfront with error">*46.792 ns\**</span> | <span title="unvalidated">*215.65 us\**</span> <span title="validated upfront with error">*224.36 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.7624 ns\**</span> <span title="validated upfront with error">*16.180 ns\**</span> | <span title="unvalidated">*51.569 us\**</span> <span title="validated upfront with error">*50.550 us\**</span> | 852.28 us |
| alkahest | <span title="validated on-demand with panic">*2.3883 ns\**</span> | <span title="validated on-demand with panic">*91.426 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 100.00% | 100.00% |
| bare | 5.60% | 13.08% | 100.00% | 100.00% | 100.00% |
| bincode | 3.76% | 16.27% | 100.00% | 100.00% | 100.00% |
| borsh | 4.43% | 27.32% | 100.00% | 100.00% | 100.00% |
| bson | 0.27% | 0.85% | 26.07% | 58.42% | 71.27% |
| capnp | 0.88% | † | 42.86% | 79.95% | 88.34% |
| cbor | 0.37% | 2.24% | 45.72% | 71.48% | 79.07% |
| flatbuffers | 9.16% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.27% | 2.72% | 59.26% | 75.15% | 78.87% |
| postcard | 3.11% | 12.16% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*1.22%\**</span> <span title="encode">*1.92%\**</span> | 7.09% | 68.57% | 80.50% | 83.25% |
| rkyv | 23.86% | <span title="unvalidated">*69.14%\**</span> <span title="validated upfront with error">*65.48%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 1.35% | 5.30% | 73.85% | 82.82% | 83.65% |
| ron | 0.11% | 0.50% | 27.04% | 59.72% | 65.61% |
| serde_json | 0.22% | 1.37% | 22.91% | 55.98% | 62.20% |
| simd-json | 0.22% | 1.07% | 15.32% | 32.44% | 36.73% |
| speedy | 28.67% | 54.74% | 100.00% | 100.00% | 100.00% |
| alkahest | 32.31% | † | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*74.73%\**</span> | <span title="unvalidated">*27.63%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.48%\**</span> | <span title="validated on-demand with error">*0.43%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*52.77%\**</span> <span title="validated upfront with error">*3.77%\**</span> | <span title="unvalidated">*23.44%\**</span> <span title="validated upfront with error">*22.53%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.89%\**</span> | <span title="unvalidated">*98.02%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*73.79%\**</span> | <span title="validated on-demand with panic">*55.29%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 397.05 us | <span title="unvalidated">*2.7395 ms\**</span> | 1290592 | 423153 | 362130 |
| bare | 1.1952 ms | 4.7295 ms | 356311 | 213270 | 199012 |
| bincode | 841.71 us | 3.3924 ms | 569975 | 240897 | 232904 |
| borsh | 799.83 us | 3.1241 ms | 446595 | 234395 | 210623 |
| bson | 6.9416 ms | 14.494 ms | 1619653 | 506953 | 328780 |
| capnp | 901.83 us | † | 803896 | 336655 | 281557 |
| cbor | 2.2834 ms | 8.2242 ms | 1109821 | 347812 | 275365 |
| flatbuffers | 34.941 ms | † | 844168 | 346957 | 295321 |
| nachricht | 5.1483 ms | 6.1708 ms | 535875 | 282125 | 250252 |
| postcard | 846.16 us | 3.6627 ms | 356311 | 213270 | 199012 |
| prost | <span title="populate + encode">*5.0674 ms\**</span> <span title="encode">*1.8217 ms\**</span> | 8.1423 ms | 596811 | 306728 | 270153 |
| rkyv | 526.61 us | <span title="unvalidated">*2.3126 ms\**</span> <span title="validated upfront with error">*2.9381 ms\**</span> | 596952 | 254571 | 220523 |
| rmp | 1.9748 ms | 4.5638 ms | 418025 | 244869 | 226158 |
| ron | 10.658 ms | 17.779 ms | 1465223 | 439761 | 345579 |
| serde_json | 4.6502 ms | 10.922 ms | 1623191 | 472275 | 362019 |
| simd-json | 4.6189 ms | 8.7888 ms | 1663769 | 496401 | 386208 |
| speedy | 476.16 us | 2.6256 ms | 449595 | 235136 | 210904 |
| alkahest | 401.85 us | † | 667570 | 325536 | 320888 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*49.320 us\**</span> | <span title="unvalidated">*50.352 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*166.68 ns\**</span> | <span title="validated on-demand with error">*5.9148 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2020 ns\**</span> <span title="validated upfront with error">*2.6585 ms\**</span> | <span title="unvalidated">*3.9201 us\**</span> <span title="validated upfront with error">*2.6256 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.5082 ns\**</span> <span title="validated upfront with error">*614.01 us\**</span> | <span title="unvalidated">*175.77 ns\**</span> <span title="validated upfront with error">*614.06 us\**</span> | 6.7172 us |
| alkahest | <span title="validated on-demand with panic">*2.1620 ns\**</span> | <span title="validated on-demand with panic">*30.433 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*84.42%\**</span> | 27.61% | 50.40% | 54.96% |
| bare | 33.22% | 48.90% | 100.00% | 100.00% | 100.00% |
| bincode | 47.17% | 68.17% | 62.51% | 88.53% | 85.45% |
| borsh | 49.64% | 74.02% | 79.78% | 90.99% | 94.49% |
| bson | 5.72% | 15.96% | 22.00% | 42.07% | 60.53% |
| capnp | 44.03% | † | 44.32% | 63.35% | 70.68% |
| cbor | 17.39% | 28.12% | 32.11% | 61.32% | 72.27% |
| flatbuffers | 1.14% | † | 42.21% | 61.47% | 67.39% |
| nachricht | 7.71% | 37.48% | 66.49% | 75.59% | 79.52% |
| postcard | 46.92% | 63.14% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*7.84%\**</span> <span title="encode">*21.80%\**</span> | 28.40% | 59.70% | 69.53% | 73.67% |
| rkyv | 75.40% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.71%\**</span> | 59.69% | 83.78% | 90.25% |
| rmp | 20.11% | 50.67% | 85.24% | 87.10% | 88.00% |
| ron | 3.73% | 13.01% | 24.32% | 48.50% | 57.59% |
| serde_json | 8.54% | 21.17% | 21.95% | 45.16% | 54.97% |
| simd-json | 8.60% | 26.31% | 21.42% | 42.96% | 51.53% |
| speedy | 83.39% | 88.08% | 79.25% | 90.70% | 94.36% |
| alkahest | 98.81% | † | 53.37% | 65.51% | 62.02% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.35%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.90%\**</span> | <span title="validated on-demand with error">*2.97%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*47.10%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*4.48%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*69.76%\**</span> | <span title="validated on-demand with panic">*0.58%\**</span> | ‡ |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
