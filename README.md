<!-- EDIT /home/runner/work/rust_serialization_benchmark/rust_serialization_benchmark/README.md -->

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

## Last updated: 2022-07-23

<!-- EDIT /home/runner/work/rust_serialization_benchmark/rust_serialization_benchmark/result.md -->

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 243.20 µs | <span title="unvalidated">*2.2884 ms\**</span> | 1705800 | 506926 | 403581 |
| bare | 646.01 µs | 3.2905 ms | 765778 | 312739 | 264630 |
| bincode | 545.49 µs | 3.0581 ms | 1045784 | 374305 | 311761 |
| borsh | 462.31 µs | 3.3105 ms | 885780 | 363280 | 286514 |
| bson | 2.8756 ms | 9.9132 ms | 1924682 | 537661 | 376270 |
| capnp | 1.2134 ms | † | 1443216 | 509618 | 428649 |
| cbor | 1.9857 ms | 6.6749 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 1.9654 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 6.3844 ms | 5.1478 ms | 818669 | 334639 | 285514 |
| postcard | 402.84 µs | 3.4262 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*2.8991 ms\**</span> <span title="encode">*584.89 µs\**</span> | 4.1361 ms | 764951 | 269811 | 227947 |
| rkyv | 346.72 µs | <span title="unvalidated">*2.5411 ms\**</span> <span title="validated upfront with error">*3.3821 ms\**</span> | 1011488 | 392809 | 331932 |
| rmp | 1.4812 ms | 4.5934 ms | 784997 | 326654 | 278219 |
| ron | 17.515 ms | 14.874 ms | 1607459 | 452648 | 349713 |
| serde_json | 4.0448 ms | 8.7980 ms | 1827461 | 474358 | 361090 |
| simd-json | 4.3596 ms | 6.1991 ms | 1827461 | 474358 | 361090 |
| speedy | 309.59 µs | 2.5626 ms | 885780 | 363280 | 286514 |
| alkahest | 242.91 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 656.08 µs | 3.8339 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*38.361 µs\**</span> | <span title="unvalidated">*63.111 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*121.29 ns\**</span> | <span title="validated on-demand with error">*457.02 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2714 ns\**</span> <span title="validated upfront with error">*2.1077 ms\**</span> | <span title="unvalidated">*97.178 µs\**</span> <span title="validated upfront with error">*2.1803 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.4935 ns\**</span> <span title="validated upfront with error">*830.31 µs\**</span> | <span title="unvalidated">*16.504 µs\**</span> <span title="validated upfront with error">*846.97 µs\**</span> | 14.252 µs |
| alkahest | <span title="validated on-demand with panic">*2.4049 ns\**</span> | <span title="validated on-demand with panic">*47.276 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 99.88% | <span title="unvalidated">*100.00%\**</span> | 42.50% | 53.22% | 56.48% |
| bare | 37.60% | 69.55% | 94.67% | 86.27% | 86.14% |
| bincode | 44.53% | 74.83% | 69.32% | 72.08% | 73.12% |
| borsh | 52.54% | 69.13% | 81.84% | 74.27% | 79.56% |
| bson | 8.45% | 23.08% | 37.67% | 50.18% | 60.58% |
| capnp | 20.02% | † | 50.23% | 52.94% | 53.18% |
| cbor | 12.23% | 34.28% | 51.49% | 66.23% | 70.34% |
| flatbuffers | 12.36% | † | 56.80% | 57.41% | 58.62% |
| nachricht | 3.80% | 44.45% | 88.55% | 80.63% | 79.84% |
| postcard | 60.30% | 66.79% | 100.00% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*8.38%\**</span> <span title="encode">*41.53%\**</span> | 55.33% | 94.77% | 100.00% | 100.00% |
| rkyv | 70.06% | <span title="unvalidated">*90.06%\**</span> <span title="validated upfront with error">*67.66%\**</span> | 71.67% | 68.69% | 68.67% |
| rmp | 16.40% | 49.82% | 92.35% | 82.60% | 81.93% |
| ron | 1.39% | 15.39% | 45.10% | 59.61% | 65.18% |
| serde_json | 6.01% | 26.01% | 39.67% | 56.88% | 63.13% |
| simd-json | 5.57% | 36.92% | 39.67% | 56.88% | 63.13% |
| speedy | 78.46% | 89.30% | 81.84% | 74.27% | 79.56% |
| alkahest | 100.00% | † | 69.32% | 59.33% | 58.53% |
| dlhn | 37.02% | 59.69% | 100.00% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*26.15%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.23%\**</span> | <span title="validated on-demand with error">*3.61%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*45.65%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.98%\**</span> <span title="validated upfront with error">*0.76%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.95%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*62.10%\**</span> | <span title="validated on-demand with panic">*34.91%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 476.02 µs | <span title="unvalidated">*452.11 µs\**</span> | 6000024 | 5380837 | 5345891 |
| bare | 6.5464 ms | 5.4215 ms | 6000003 | 5380817 | 5345900 |
| bincode | 4.9416 ms | 4.9795 ms | 6000008 | 5380823 | 5345890 |
| borsh | 6.7839 ms | 5.8009 ms | 6000004 | 5380818 | 5345889 |
| bson | 54.550 ms | 130.39 ms | 23013911 | 9211138 | 7497811 |
| capnp | 10.658 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 43.828 ms | 54.944 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 848.28 µs | † | 6000024 | 5380800 | 5345910 |
| nachricht | 153.00 ms | 46.422 ms | 8125037 | 6495174 | 6386940 |
| postcard | 1.0140 ms | 1.5693 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*13.560 ms\**</span> <span title="encode">*9.5961 ms\**</span> | 21.464 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 554.13 µs | <span title="unvalidated">*464.25 µs\**</span> <span title="validated upfront with error">*452.51 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 18.575 ms | 16.129 ms | 8125006 | 6496879 | 6391037 |
| ron | 201.40 ms | 258.19 ms | 22192885 | 9009575 | 8138755 |
| serde_json | 102.78 ms | 82.855 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 91.790 ms | 136.05 ms | 39152823 | 16587283 | 14549214 |
| speedy | 1.8158 ms | 1.0111 ms | 6000004 | 5380818 | 5345889 |
| alkahest | 455.03 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 7.2609 ms | 7.4576 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*3.0785 ns\**</span> | <span title="unvalidated">*236.17 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*188.59 ns\**</span> | <span title="validated on-demand with error">*6.4194 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2737 ns\**</span> <span title="validated upfront with error">*38.305 ns\**</span> | <span title="unvalidated">*41.648 µs\**</span> <span title="validated upfront with error">*37.070 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.6909 ns\**</span> <span title="validated upfront with error">*15.790 ns\**</span> | <span title="unvalidated">*47.148 µs\**</span> <span title="validated upfront with error">*41.627 µs\**</span> | 244.08 µs |
| alkahest | <span title="validated on-demand with panic">*2.4041 ns\**</span> | <span title="validated on-demand with panic">*83.739 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 95.59% | <span title="unvalidated">*100.00%\**</span> | 100.00% | 100.00% | 100.00% |
| bare | 6.95% | 8.34% | 100.00% | 100.00% | 100.00% |
| bincode | 9.21% | 9.08% | 100.00% | 100.00% | 100.00% |
| borsh | 6.71% | 7.79% | 100.00% | 100.00% | 100.00% |
| bson | 0.83% | 0.35% | 26.07% | 58.42% | 71.30% |
| capnp | 4.27% | † | 42.86% | 79.95% | 88.35% |
| cbor | 1.04% | 0.82% | 45.72% | 71.48% | 79.09% |
| flatbuffers | 53.64% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.30% | 0.97% | 73.85% | 82.84% | 83.70% |
| postcard | 44.87% | 28.81% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*3.36%\**</span> <span title="encode">*4.74%\**</span> | 2.11% | 68.57% | 80.50% | 83.25% |
| rkyv | 82.12% | <span title="unvalidated">*97.39%\**</span> <span title="validated upfront with error">*99.91%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 2.45% | 2.80% | 73.85% | 82.82% | 83.65% |
| ron | 0.23% | 0.18% | 27.04% | 59.72% | 65.68% |
| serde_json | 0.44% | 0.55% | 22.91% | 55.98% | 62.26% |
| simd-json | 0.50% | 0.33% | 15.32% | 32.44% | 36.74% |
| speedy | 25.06% | 44.71% | 100.00% | 100.00% | 100.00% |
| alkahest | 100.00% | † | 100.00% | 100.00% | 100.00% |
| dlhn | 6.27% | 6.06% | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*54.93%\**</span> | <span title="unvalidated">*15.70%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.90%\**</span> | <span title="validated on-demand with error">*0.58%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*51.65%\**</span> <span title="validated upfront with error">*4.41%\**</span> | <span title="unvalidated">*89.01%\**</span> <span title="validated upfront with error">*100.00%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*10.71%\**</span> | <span title="unvalidated">*78.62%\**</span> <span title="validated upfront with error">*89.05%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*70.33%\**</span> | <span title="validated on-demand with panic">*44.27%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 232.53 µs | <span title="unvalidated">*1.7070 ms\**</span> | 1290592 | 394115 | 330468 |
| bare | 830.04 µs | 2.9674 ms | 356311 | 213270 | 198488 |
| bincode | 711.90 µs | 2.7134 ms | 569975 | 240897 | 232423 |
| borsh | 539.62 µs | 2.3411 ms | 446595 | 234395 | 210008 |
| bson | 4.2173 ms | 12.360 ms | 1619653 | 506953 | 328399 |
| capnp | 891.03 µs | † | 803896 | 336655 | 280851 |
| cbor | 1.9840 ms | 6.4790 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 3.6046 ms | † | 844168 | 346957 | 294015 |
| nachricht | 6.6591 ms | 5.4333 ms | 449745 | 252743 | 231110 |
| postcard | 467.17 µs | 2.8373 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*4.0024 ms\**</span> <span title="encode">*1.4604 ms\**</span> | 4.1889 ms | 596811 | 306728 | 269310 |
| rkyv | 412.09 µs | <span title="unvalidated">*1.7013 ms\**</span> <span title="validated upfront with error">*2.2466 ms\**</span> | 596952 | 254571 | 219976 |
| rmp | 1.5191 ms | 3.5754 ms | 424533 | 245594 | 226188 |
| ron | 9.3493 ms | 17.451 ms | 1465223 | 439761 | 343338 |
| serde_json | 4.7781 ms | 10.090 ms | 1623191 | 472275 | 359623 |
| simd-json | 4.3278 ms | 5.3403 ms | 1663769 | 496401 | 383682 |
| speedy | 369.53 µs | 2.0181 ms | 449595 | 235136 | 210361 |
| alkahest | 297.15 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*45.896 µs\**</span> | <span title="unvalidated">*47.171 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*107.52 ns\**</span> | <span title="validated on-demand with error">*706.94 ns\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*2.8897 ns\**</span> <span title="validated upfront with error">*2.1405 ms\**</span> | <span title="unvalidated">*2.4471 µs\**</span> <span title="validated upfront with error">*2.1330 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.6841 ns\**</span> <span title="validated upfront with error">*675.72 µs\**</span> | <span title="unvalidated">*188.71 ns\**</span> <span title="validated upfront with error">*677.17 µs\**</span> | 1.4986 µs |
| alkahest | <span title="validated on-demand with panic">*2.1239 ns\**</span> | <span title="validated on-demand with panic">*44.104 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*99.67%\**</span> | 27.61% | 54.11% | 60.06% |
| bare | 28.01% | 57.33% | 100.00% | 100.00% | 100.00% |
| bincode | 32.66% | 62.70% | 62.51% | 88.53% | 85.40% |
| borsh | 43.09% | 72.67% | 79.78% | 90.99% | 94.51% |
| bson | 5.51% | 13.76% | 22.00% | 42.07% | 60.44% |
| capnp | 26.10% | † | 44.32% | 63.35% | 70.67% |
| cbor | 11.72% | 26.26% | 32.11% | 61.32% | 72.30% |
| flatbuffers | 6.45% | † | 42.21% | 61.47% | 67.51% |
| nachricht | 3.49% | 31.31% | 79.23% | 84.38% | 85.88% |
| postcard | 49.77% | 59.96% | 96.96% | 96.01% | 95.73% |
| prost | <span title="populate + encode">*5.81%\**</span> <span title="encode">*15.92%\**</span> | 40.61% | 59.70% | 69.53% | 73.70% |
| rkyv | 56.43% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*75.73%\**</span> | 59.69% | 83.78% | 90.23% |
| rmp | 15.31% | 47.58% | 83.93% | 86.84% | 87.75% |
| ron | 2.49% | 9.75% | 24.32% | 48.50% | 57.81% |
| serde_json | 4.87% | 16.86% | 21.95% | 45.16% | 55.19% |
| simd-json | 5.37% | 31.86% | 21.42% | 42.96% | 51.73% |
| speedy | 62.93% | 84.30% | 79.25% | 90.70% | 94.36% |
| alkahest | 78.25% | † | 53.37% | 65.51% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.40%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.57%\**</span> | <span title="validated on-demand with error">*26.69%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*58.28%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.71%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*79.29%\**</span> | <span title="validated on-demand with panic">*0.43%\**</span> | ‡ |

