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

## Last updated: 6/20/2022

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 206.27 us | <span title="unvalidated">*2.9411 ms\**</span> | 1705800 | 515090 | 406498 |
| bare | 732.77 us | 4.4407 ms | 765778 | 312739 | 265462 |
| bincode | 564.72 us | 3.6424 ms | 1045784 | 374305 | 312658 |
| borsh | 477.96 us | 3.7272 ms | 885780 | 363280 | 286948 |
| bson | 3.2133 ms | 11.432 ms | 1924682 | 537661 | 377489 |
| capnp | 1.1955 ms | † | 1443216 | 509618 | 429784 |
| cbor | 2.0142 ms | 8.8629 ms | 1407835 | 407372 | 325016 |
| flatbuffers | 2.9258 ms | † | 1276368 | 469962 | 389885 |
| nachricht | 9.4611 ms | 9.9223 ms | 926221 | 365209 | 304177 |
| postcard | 728.75 us | 4.5128 ms | 724953 | 303462 | 254408 |
| prost | <span title="populate + encode">*6.5794 ms\**</span> <span title="encode">*842.49 us\**</span> | 8.5569 ms | 764951 | 269811 | 228807 |
| rkyv | 278.86 us | <span title="unvalidated">*2.7780 ms\**</span> <span title="validated upfront with error">*3.4473 ms\**</span> | 1011488 | 392809 | 332689 |
| rmp | 1.4008 ms | 4.6204 ms | 784997 | 326654 | 278933 |
| ron | 16.245 ms | 15.086 ms | 1607459 | 452648 | 351604 |
| serde_json | 4.0430 ms | 8.7244 ms | 1827461 | 474358 | 364177 |
| simd-json | 3.9252 ms | 9.5618 ms | 1827461 | 474358 | 364177 |
| speedy | 729.59 us | 4.2750 ms | 885780 | 363280 | 286948 |
| alkahest | 361.56 us | † | 1045784 | 454748 | 389910 |
| dlhn | 1.2252 ms | 5.2271 ms | 724953 | 302512 | 254382 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*34.503 us\**</span> | <span title="unvalidated">*54.890 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*163.14 ns\**</span> | <span title="validated on-demand with error">*588.92 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.4658 ns\**</span> <span title="validated upfront with error">*2.5279 ms\**</span> | <span title="unvalidated">*159.89 us\**</span> <span title="validated upfront with error">*2.8982 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.3385 ns\**</span> <span title="validated upfront with error">*674.29 us\**</span> | <span title="unvalidated">*15.896 us\**</span> <span title="validated upfront with error">*689.44 us\**</span> | 62.167 us |
| alkahest | <span title="validated on-demand with panic">*2.4288 ns\**</span> | <span title="validated on-demand with panic">*103.43 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*94.45%\**</span> | 42.50% | 52.38% | 56.29% |
| bare | 28.15% | 62.56% | 94.67% | 86.27% | 86.19% |
| bincode | 36.53% | 76.27% | 69.32% | 72.08% | 73.18% |
| borsh | 43.16% | 74.53% | 81.84% | 74.27% | 79.74% |
| bson | 6.42% | 24.30% | 37.67% | 50.18% | 60.61% |
| capnp | 17.25% | † | 50.23% | 52.94% | 53.24% |
| cbor | 10.24% | 31.34% | 51.49% | 66.23% | 70.40% |
| flatbuffers | 7.05% | † | 56.80% | 57.41% | 58.69% |
| nachricht | 2.18% | 28.00% | 78.27% | 73.88% | 75.22% |
| postcard | 28.30% | 61.56% | 100.00% | 88.91% | 89.94% |
| prost | <span title="populate + encode">*3.14%\**</span> <span title="encode">*24.48%\**</span> | 32.47% | 94.77% | 100.00% | 100.00% |
| rkyv | 73.97% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*80.58%\**</span> | 71.67% | 68.69% | 68.78% |
| rmp | 14.73% | 60.12% | 92.35% | 82.60% | 82.03% |
| ron | 1.27% | 18.41% | 45.10% | 59.61% | 65.08% |
| serde_json | 5.10% | 31.84% | 39.67% | 56.88% | 62.83% |
| simd-json | 5.26% | 29.05% | 39.67% | 56.88% | 62.83% |
| speedy | 28.27% | 64.98% | 81.84% | 74.27% | 79.74% |
| alkahest | 57.05% | † | 69.32% | 59.33% | 58.68% |
| dlhn | 16.84% | 53.15% | 100.00% | 89.19% | 89.95% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*28.96%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.82%\**</span> | <span title="validated on-demand with error">*2.70%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*38.62%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*9.94%\**</span> <span title="validated upfront with error">*0.55%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.31%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*55.11%\**</span> | <span title="validated on-demand with panic">*15.37%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 335.48 us | <span title="unvalidated">*1.6834 ms\**</span> | 6000024 | 5380836 | 5345893 |
| bare | 6.2228 ms | 14.383 ms | 6000003 | 5380817 | 5345893 |
| bincode | 9.2662 ms | 9.3830 ms | 6000008 | 5380823 | 5345885 |
| borsh | 9.8756 ms | 10.768 ms | 6000004 | 5380818 | 5345888 |
| bson | 74.425 ms | 158.07 ms | 23013911 | 9211138 | 7501230 |
| capnp | 25.993 ms | † | 14000088 | 6729881 | 6051413 |
| cbor | 53.611 ms | 81.163 ms | 13122324 | 7527423 | 6761276 |
| flatbuffers | 3.1408 ms | † | 6000024 | 5380800 | 5345925 |
| nachricht | 91.385 ms | 48.868 ms | 10125030 | 7160144 | 6777986 |
| postcard | 1.9607 ms | 7.7435 ms | 6000003 | 5380817 | 5345893 |
| prost | <span title="populate + encode">*30.841 ms\**</span> <span title="encode">*18.936 ms\**</span> | 28.271 ms | 8750000 | 6683814 | 6421721 |
| rkyv | 1.0314 ms | <span title="unvalidated">*2.0368 ms\**</span> <span title="validated upfront with error">*2.0824 ms\**</span> | 6000008 | 5380822 | 5345887 |
| rmp | 30.852 ms | 25.560 ms | 8125006 | 6496879 | 6391025 |
| ron | 278.43 ms | 296.14 ms | 22192885 | 9009575 | 8147524 |
| serde_json | 150.80 ms | 99.862 ms | 26192883 | 9612105 | 8595089 |
| simd-json | 147.87 ms | 140.41 ms | 39152823 | 16587283 | 14555990 |
| speedy | 882.38 us | 2.6292 ms | 6000004 | 5380818 | 5345888 |
| alkahest | 501.32 us | † | 6000008 | 5380823 | 5345885 |
| dlhn | 7.9378 ms | 15.058 ms | 6000003 | 5380817 | 5345893 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.6173 ns\**</span> | <span title="unvalidated">*294.78 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*313.83 ns\**</span> | <span title="validated on-demand with error">*9.5382 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.3848 ns\**</span> <span title="validated upfront with error">*43.346 ns\**</span> | <span title="unvalidated">*165.93 us\**</span> <span title="validated upfront with error">*177.54 us\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.6973 ns\**</span> <span title="validated upfront with error">*16.271 ns\**</span> | <span title="unvalidated">*50.612 us\**</span> <span title="validated upfront with error">*47.230 us\**</span> | 745.11 us |
| alkahest | <span title="validated on-demand with panic">*2.3742 ns\**</span> | <span title="validated on-demand with panic">*86.893 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 100.00% | 100.00% |
| bare | 5.39% | 11.70% | 100.00% | 100.00% | 100.00% |
| bincode | 3.62% | 17.94% | 100.00% | 100.00% | 100.00% |
| borsh | 3.40% | 15.63% | 100.00% | 100.00% | 100.00% |
| bson | 0.45% | 1.06% | 26.07% | 58.42% | 71.27% |
| capnp | 1.29% | † | 42.86% | 79.95% | 88.34% |
| cbor | 0.63% | 2.07% | 45.72% | 71.48% | 79.07% |
| flatbuffers | 10.68% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.37% | 3.44% | 59.26% | 75.15% | 78.87% |
| postcard | 17.11% | 21.74% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*1.09%\**</span> <span title="encode">*1.77%\**</span> | 5.95% | 68.57% | 80.50% | 83.25% |
| rkyv | 32.53% | <span title="unvalidated">*82.65%\**</span> <span title="validated upfront with error">*80.84%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 1.09% | 6.59% | 73.85% | 82.82% | 83.65% |
| ron | 0.12% | 0.57% | 27.04% | 59.72% | 65.61% |
| serde_json | 0.22% | 1.69% | 22.91% | 55.98% | 62.20% |
| simd-json | 0.23% | 1.20% | 15.32% | 32.44% | 36.73% |
| speedy | 38.02% | 64.03% | 100.00% | 100.00% | 100.00% |
| alkahest | 66.92% | † | 100.00% | 100.00% | 100.00% |
| dlhn | 4.23% | 11.18% | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*64.85%\**</span> | <span title="unvalidated">*16.02%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.54%\**</span> | <span title="validated on-demand with error">*0.50%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*50.14%\**</span> <span title="validated upfront with error">*3.92%\**</span> | <span title="unvalidated">*28.46%\**</span> <span title="validated upfront with error">*26.60%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.43%\**</span> | <span title="unvalidated">*93.32%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*71.49%\**</span> | <span title="validated on-demand with panic">*54.35%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 495.21 us | <span title="unvalidated">*2.9267 ms\**</span> | 1290592 | 390628 | 329822 |
| bare | 1.6024 ms | 5.2197 ms | 356311 | 213270 | 199012 |
| bincode | 1.4247 ms | 3.9795 ms | 569975 | 240897 | 232904 |
| borsh | 1.3690 ms | 3.8030 ms | 446595 | 234395 | 210623 |
| bson | 8.9111 ms | 17.072 ms | 1619653 | 506953 | 328780 |
| capnp | 1.0548 ms | † | 803896 | 336655 | 281557 |
| cbor | 3.9529 ms | 9.0985 ms | 1109821 | 347812 | 275365 |
| flatbuffers | 39.963 ms | † | 844168 | 346957 | 295321 |
| nachricht | 7.7520 ms | 7.6250 ms | 535875 | 282125 | 250252 |
| postcard | 984.60 us | 4.2912 ms | 367489 | 222144 | 207844 |
| prost | <span title="populate + encode">*7.0903 ms\**</span> <span title="encode">*2.1849 ms\**</span> | 9.5588 ms | 596811 | 306728 | 270153 |
| rkyv | 804.28 us | <span title="unvalidated">*2.3283 ms\**</span> <span title="validated upfront with error">*3.0605 ms\**</span> | 596952 | 254571 | 220523 |
| rmp | 2.0642 ms | 4.8106 ms | 418025 | 244869 | 226158 |
| ron | 11.671 ms | 17.051 ms | 1465223 | 439761 | 345579 |
| serde_json | 5.0867 ms | 10.716 ms | 1623191 | 472275 | 362019 |
| simd-json | 5.6466 ms | 9.5609 ms | 1663769 | 496401 | 386208 |
| speedy | 517.97 us | 2.5761 ms | 449595 | 235136 | 210904 |
| alkahest | 438.80 us | † | 667570 | 325536 | 320888 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*51.214 us\**</span> | <span title="unvalidated">*48.362 us\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*211.53 ns\**</span> | <span title="validated on-demand with error">*6.3068 us\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.3949 ns\**</span> <span title="validated upfront with error">*2.7556 ms\**</span> | <span title="unvalidated">*4.1169 us\**</span> <span title="validated upfront with error">*2.7745 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.8186 ns\**</span> <span title="validated upfront with error">*718.49 us\**</span> | <span title="unvalidated">*209.12 ns\**</span> <span title="validated upfront with error">*663.16 us\**</span> | 7.2345 us |
| alkahest | <span title="validated on-demand with panic">*2.1388 ns\**</span> | <span title="validated on-demand with panic">*30.571 us\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 88.61% | <span title="unvalidated">*79.55%\**</span> | 27.61% | 54.60% | 60.34% |
| bare | 27.38% | 44.61% | 100.00% | 100.00% | 100.00% |
| bincode | 30.80% | 58.51% | 62.51% | 88.53% | 85.45% |
| borsh | 32.05% | 61.22% | 79.78% | 90.99% | 94.49% |
| bson | 4.92% | 13.64% | 22.00% | 42.07% | 60.53% |
| capnp | 41.60% | † | 44.32% | 63.35% | 70.68% |
| cbor | 11.10% | 25.59% | 32.11% | 61.32% | 72.27% |
| flatbuffers | 1.10% | † | 42.21% | 61.47% | 67.39% |
| nachricht | 5.66% | 30.54% | 66.49% | 75.59% | 79.52% |
| postcard | 44.57% | 54.26% | 96.96% | 96.01% | 95.75% |
| prost | <span title="populate + encode">*6.19%\**</span> <span title="encode">*20.08%\**</span> | 24.36% | 59.70% | 69.53% | 73.67% |
| rkyv | 54.56% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*76.08%\**</span> | 59.69% | 83.78% | 90.25% |
| rmp | 21.26% | 48.40% | 85.24% | 87.10% | 88.00% |
| ron | 3.76% | 13.65% | 24.32% | 48.50% | 57.59% |
| serde_json | 8.63% | 21.73% | 21.95% | 45.16% | 54.97% |
| simd-json | 7.77% | 24.35% | 21.42% | 42.96% | 51.53% |
| speedy | 84.72% | 90.38% | 79.25% | 90.70% | 94.36% |
| alkahest | 100.00% | † | 53.37% | 65.51% | 62.02% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.43%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.86%\**</span> | <span title="validated on-demand with error">*3.32%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*53.57%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*5.08%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*85.03%\**</span> | <span title="validated on-demand with panic">*0.68%\**</span> | ‡ |

## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
