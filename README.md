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

## Last updated: 11/29/2021

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 239.45 us | <span title="unvalidated">*2.9763 ms\**</span> | 1705800 | 507150 | 405443 |
| bincode | 497.62 us | 3.7935 ms | 1045784 | 374305 | 312658 |
| borsh | 512.29 us | 3.7558 ms | 885780 | 363280 | 286948 |
| bson | 3.2398 ms | 11.466 ms | 1924682 | 537661 | 377489 |
| capnp | 1.4190 ms | † | 1443216 | 509618 | 429784 |
| cbor | 1.8215 ms | 7.8008 ms | 1407835 | 407372 | 325016 |
| flatbuffers | 2.4512 ms | † | 1276368 | 469962 | 389885 |
| nachricht | 4.2831 ms | 6.5536 ms | 926221 | 365209 | 304177 |
| postcard | 645.02 us | 4.3221 ms | 765778 | 312739 | 265462 |
| prost | <span title="populate + encode">*4.4750 ms\**</span> <span title="encode">*798.69 us\**</span> | 8.8317 ms | 764951 | 269811 | 228807 |
| rkyv | 276.82 us | <span title="unvalidated">*2.7300 ms\**</span> <span title="validated upfront with error">*3.4442 ms\**</span> | 1011488 | 392809 | 332689 |
| rmp | 1.8461 ms | 4.7317 ms | 784997 | 326654 | 278933 |
| ron | 23.034 ms | 16.148 ms | 1607459 | 452648 | 351604 |
| serde_json | 4.2477 ms | 9.1914 ms | 1827461 | 474358 | 364177 |
| simd-json | 4.0963 ms | 8.9160 ms | 1827461 | 474358 | 364177 |
| speedy | 244.56 us | 3.1769 ms | 885780 | 363280 | 286948 |
| alkahest | 232.09 us | † | 1045784 | 454748 | 389910 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*50.281 us\**</span> | <span title="unvalidated">*75.922 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*170.11 ns\**</span> | <span title="validated on-demand with error">*581.31 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.8251 ns\**</span> <span title="validated upfront with error">*2.3512 ms\**</span> | <span title="unvalidated">*155.53 us\**</span> <span title="validated upfront with error">*2.4411 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.2624 ns\**</span> <span title="validated upfront with error">*658.19 us\**</span> | <span title="unvalidated">*15.972 us\**</span> <span title="validated upfront with error">*674.73 us\**</span> | 55.444 us |
| alkahest | <span title="validated on-demand with panic">*1.9109 ns\**</span> | <span title="validated on-demand with panic">*70.966 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 96.93% | <span title="unvalidated">*91.72%\**</span> | 44.84% | 53.20% | 56.43% |
| bincode | 46.64% | 71.97% | 73.15% | 72.08% | 73.18% |
| borsh | 45.30% | 72.69% | 86.36% | 74.27% | 79.74% |
| bson | 7.16% | 23.81% | 39.74% | 50.18% | 60.61% |
| capnp | 16.36% | † | 53.00% | 52.94% | 53.24% |
| cbor | 12.74% | 35.00% | 54.34% | 66.23% | 70.40% |
| flatbuffers | 9.47% | † | 59.93% | 57.41% | 58.69% |
| nachricht | 5.42% | 41.66% | 82.59% | 73.88% | 75.22% |
| postcard | 35.98% | 63.16% | 99.89% | 86.27% | 86.19% |
| prost | <span title="populate + encode">*5.19%\**</span> <span title="encode">*29.06%\**</span> | 30.91% | 100.00% | 100.00% | 100.00% |
| rkyv | 83.84% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.26%\**</span> | 75.63% | 68.69% | 68.78% |
| rmp | 12.57% | 57.70% | 97.45% | 82.60% | 82.03% |
| ron | 1.01% | 16.91% | 47.59% | 59.61% | 65.08% |
| serde_json | 5.46% | 29.70% | 41.86% | 56.88% | 62.83% |
| simd-json | 5.67% | 30.62% | 41.86% | 56.88% | 62.83% |
| speedy | 94.90% | 85.93% | 86.36% | 74.27% | 79.74% |
| alkahest | 100.00% | † | 73.15% | 59.33% | 58.68% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*21.04%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.74%\**</span> | <span title="validated on-demand with error">*2.75%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*44.69%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.27%\**</span> <span title="validated upfront with error">*0.65%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.37%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*66.06%\**</span> | <span title="validated on-demand with panic">*22.51%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 357.48 us | <span title="unvalidated">*1.6575 ms\**</span> | 6000024 | 5380836 | 5345893 |
| bincode | 4.6646 ms | 5.9235 ms | 6000008 | 5380823 | 5345885 |
| borsh | 6.9330 ms | 8.6247 ms | 6000004 | 5380818 | 5345888 |
| bson | 59.888 ms | 123.20 ms | 23013911 | 9211138 | 7501230 |
| capnp | 21.611 ms | † | 14000088 | 6729881 | 6051413 |
| cbor | 40.682 ms | 66.170 ms | 13122324 | 7527423 | 6761276 |
| flatbuffers | 2.1025 ms | † | 6000024 | 5380800 | 5345925 |
| nachricht | 62.391 ms | 36.754 ms | 10125030 | 7160144 | 6777986 |
| postcard | 5.8788 ms | 10.183 ms | 6000003 | 5380817 | 5345893 |
| prost | <span title="populate + encode">*21.310 ms\**</span> <span title="encode">*15.139 ms\**</span> | 20.325 ms | 8750000 | 6683814 | 6421721 |
| rkyv | 718.53 us | <span title="unvalidated">*1.8408 ms\**</span> <span title="validated upfront with error">*1.8406 ms\**</span> | 6000008 | 5380822 | 5345887 |
| rmp | 16.159 ms | 20.422 ms | 8125006 | 6496879 | 6391025 |
| ron | 211.17 ms | 258.24 ms | 22192885 | 9009575 | 8147524 |
| serde_json | 95.967 ms | 88.608 ms | 26192883 | 9612105 | 8595089 |
| simd-json | 99.210 ms | 120.84 ms | 39152823 | 16587283 | 14555990 |
| speedy | 651.91 us | 2.2203 ms | 6000004 | 5380818 | 5345888 |
| alkahest | 614.17 us | † | 6000008 | 5380823 | 5345885 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.3751 ns\**</span> | <span title="unvalidated">*175.23 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*252.78 ns\**</span> | <span title="validated on-demand with error">*7.8701 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.5732 ns\**</span> <span title="validated upfront with error">*42.283 ns\**</span> | <span title="unvalidated">*139.52 us\**</span> <span title="validated upfront with error">*139.67 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.2620 ns\**</span> <span title="validated upfront with error">*14.443 ns\**</span> | <span title="unvalidated">*34.877 us\**</span> <span title="validated upfront with error">*35.092 us\**</span> | 531.48 us |
| alkahest | <span title="validated on-demand with panic">*1.8985 ns\**</span> | <span title="validated on-demand with panic">*69.758 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 100.00% | 100.00% |
| bincode | 7.66% | 27.98% | 100.00% | 100.00% | 100.00% |
| borsh | 5.16% | 19.22% | 100.00% | 100.00% | 100.00% |
| bson | 0.60% | 1.35% | 26.07% | 58.42% | 71.27% |
| capnp | 1.65% | † | 42.86% | 79.95% | 88.34% |
| cbor | 0.88% | 2.50% | 45.72% | 71.48% | 79.07% |
| flatbuffers | 17.00% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.57% | 4.51% | 59.26% | 75.15% | 78.87% |
| postcard | 6.08% | 16.28% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*1.68%\**</span> <span title="encode">*2.36%\**</span> | 8.15% | 68.57% | 80.50% | 83.25% |
| rkyv | 49.75% | <span title="unvalidated">*90.04%\**</span> <span title="validated upfront with error">*90.05%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 2.21% | 8.12% | 73.85% | 82.82% | 83.65% |
| ron | 0.17% | 0.64% | 27.04% | 59.72% | 65.61% |
| serde_json | 0.37% | 1.87% | 22.91% | 55.98% | 62.20% |
| simd-json | 0.36% | 1.37% | 15.32% | 32.44% | 36.73% |
| speedy | 54.84% | 74.65% | 100.00% | 100.00% | 100.00% |
| alkahest | 58.21% | † | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*53.13%\**</span> | <span title="unvalidated">*19.90%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.50%\**</span> | <span title="validated on-demand with error">*0.44%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*49.04%\**</span> <span title="validated upfront with error">*2.98%\**</span> | <span title="unvalidated">*25.00%\**</span> <span title="validated upfront with error">*24.97%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*8.74%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.39%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*66.47%\**</span> | <span title="validated on-demand with panic">*50.00%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 330.28 us | <span title="unvalidated">*2.3437 ms\**</span> | 1290592 | 391277 | 329474 |
| bincode | 647.92 us | 3.0044 ms | 569975 | 240897 | 232904 |
| borsh | 670.69 us | 2.8793 ms | 446595 | 234395 | 210623 |
| bson | 6.0056 ms | 12.582 ms | 1619653 | 506953 | 328780 |
| capnp | 840.18 us | † | 803896 | 336655 | 281557 |
| cbor | 1.8329 ms | 7.7217 ms | 1109821 | 347812 | 275365 |
| flatbuffers | 32.055 ms | † | 844168 | 346957 | 295321 |
| nachricht | 5.3026 ms | 6.0314 ms | 535875 | 282125 | 250252 |
| postcard | 696.65 us | 3.5101 ms | 356311 | 213270 | 199012 |
| prost | <span title="populate + encode">*4.5820 ms\**</span> <span title="encode">*1.6590 ms\**</span> | 7.8249 ms | 596811 | 306728 | 270153 |
| rkyv | 443.69 us | <span title="unvalidated">*2.1900 ms\**</span> <span title="validated upfront with error">*2.7479 ms\**</span> | 596952 | 254571 | 220523 |
| rmp | 1.6200 ms | 4.0374 ms | 418025 | 244869 | 226158 |
| ron | 10.805 ms | 16.400 ms | 1465223 | 439761 | 345579 |
| serde_json | 4.3212 ms | 9.6048 ms | 1623191 | 472275 | 362019 |
| simd-json | 4.4106 ms | 8.1604 ms | 1663769 | 496401 | 386208 |
| speedy | 412.83 us | 2.4766 ms | 449595 | 235136 | 210904 |
| alkahest | 357.67 us | † | 667570 | 325536 | 320888 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*53.922 us\**</span> | <span title="unvalidated">*55.274 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*191.42 ns\**</span> | <span title="validated on-demand with error">*5.6267 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.5716 ns\**</span> <span title="validated upfront with error">*2.5015 ms\**</span> | <span title="unvalidated">*3.7170 us\**</span> <span title="validated upfront with error">*2.5026 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.2619 ns\**</span> <span title="validated upfront with error">*547.53 us\**</span> | <span title="unvalidated">*148.03 ns\**</span> <span title="validated upfront with error">*548.16 us\**</span> | 4.7230 us |
| alkahest | <span title="validated on-demand with panic">*1.8977 ns\**</span> | <span title="validated on-demand with panic">*29.846 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*93.44%\**</span> | 27.61% | 54.51% | 60.40% |
| bincode | 50.98% | 72.89% | 62.51% | 88.53% | 85.45% |
| borsh | 49.24% | 76.06% | 79.78% | 90.99% | 94.49% |
| bson | 5.50% | 17.41% | 22.00% | 42.07% | 60.53% |
| capnp | 39.31% | † | 44.32% | 63.35% | 70.68% |
| cbor | 18.02% | 28.36% | 32.11% | 61.32% | 72.27% |
| flatbuffers | 1.03% | † | 42.21% | 61.47% | 67.39% |
| nachricht | 6.23% | 36.31% | 66.49% | 75.59% | 79.52% |
| postcard | 47.41% | 62.39% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*7.21%\**</span> <span title="encode">*19.91%\**</span> | 27.99% | 59.70% | 69.53% | 73.67% |
| rkyv | 74.44% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.70%\**</span> | 59.69% | 83.78% | 90.25% |
| rmp | 20.39% | 54.24% | 85.24% | 87.10% | 88.00% |
| ron | 3.06% | 13.35% | 24.32% | 48.50% | 57.59% |
| serde_json | 7.64% | 22.80% | 21.95% | 45.16% | 54.97% |
| simd-json | 7.49% | 26.84% | 21.42% | 42.96% | 51.53% |
| speedy | 80.00% | 88.43% | 79.25% | 90.70% | 94.36% |
| alkahest | 92.34% | † | 53.37% | 65.51% | 62.02% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.27%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.66%\**</span> | <span title="validated on-demand with error">*2.63%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*49.07%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*3.98%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*66.50%\**</span> | <span title="validated on-demand with panic">*0.50%\**</span> | ‡ |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
