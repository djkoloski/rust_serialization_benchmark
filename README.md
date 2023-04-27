<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

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

## Last updated: 2023-4-27

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 255.73 µs | <span title="unvalidated">*2.4131 ms\**</span> | 1705800 | 502519 | 414534 |
| bare | 749.70 µs | 3.2960 ms | 765778 | 312739 | 264630 |
| bincode | 438.00 µs | 3.1375 ms | 1045784 | 374305 | 311761 |
| bitcode | 822.94 µs | 3.5460 ms | 703664 | 320922 | 273622 |
| borsh | 481.13 µs | 3.3216 ms | 885780 | 363280 | 286514 |
| bson | 2.9499 ms | 11.287 ms | 1924682 | 537661 | 376270 |
| capnp | 718.58 µs | † | 1443216 | 509618 | 428649 |
| cbor | 1.9718 ms | 6.5596 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 1.9748 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 8.3143 ms | 5.5411 ms | 818669 | 334639 | 285514 |
| postcard | 422.52 µs | 3.2858 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*3.1304 ms\**</span> <span title="encode">*513.94 µs\**</span> | 3.8797 ms | 764951 | 269811 | 227947 |
| rkyv | 312.59 µs | <span title="unvalidated">*2.4088 ms\**</span> <span title="validated upfront with error">*3.3440 ms\**</span> | 1011488 | 384478 | 333545 |
| rmp | 1.5732 ms | 4.5187 ms | 784997 | 326654 | 278219 |
| ron | 15.654 ms | 20.488 ms | 1607459 | 452648 | 349713 |
| scale | 595.68 µs | 3.2740 ms | 765778 | 312771 | 264518 |
| serde_json | 3.8706 ms | 8.7641 ms | 1827461 | 474358 | 361090 |
| simd-json | 4.2916 ms | 6.4168 ms | 1827461 | 474358 | 361090 |
| speedy | 250.79 µs | 2.8089 ms | 885780 | 363280 | 286514 |
| alkahest | 248.29 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 773.81 µs | 3.6881 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*37.376 µs\**</span> | <span title="unvalidated">*55.771 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*120.57 ns\**</span> | <span title="validated on-demand with error">*380.59 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2714 ns\**</span> <span title="validated upfront with error">*2.0983 ms\**</span> | <span title="unvalidated">*96.302 µs\**</span> <span title="validated upfront with error">*2.1076 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.5224 ns\**</span> <span title="validated upfront with error">*924.74 µs\**</span> | <span title="unvalidated">*16.388 µs\**</span> <span title="validated upfront with error">*941.27 µs\**</span> | 19.085 µs |
| alkahest | <span title="validated on-demand with panic">*2.7684 ns\**</span> | <span title="validated on-demand with panic">*35.235 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 97.09% | <span title="unvalidated">*99.82%\**</span> | 41.25% | 53.69% | 54.99% |
| bare | 33.12% | 73.08% | 91.89% | 86.27% | 86.14% |
| bincode | 56.69% | 76.77% | 67.29% | 72.08% | 73.12% |
| bitcode | 30.17% | 67.93% | 100.00% | 84.07% | 83.31% |
| borsh | 51.61% | 72.52% | 79.44% | 74.27% | 79.56% |
| bson | 8.42% | 21.34% | 36.56% | 50.18% | 60.58% |
| capnp | 34.55% | † | 48.76% | 52.94% | 53.18% |
| cbor | 12.59% | 36.72% | 49.98% | 66.23% | 70.34% |
| flatbuffers | 12.57% | † | 55.13% | 57.41% | 58.62% |
| nachricht | 2.99% | 43.47% | 85.95% | 80.63% | 79.84% |
| postcard | 58.76% | 73.31% | 97.06% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*7.93%\**</span> <span title="encode">*48.31%\**</span> | 62.09% | 91.99% | 100.00% | 100.00% |
| rkyv | 79.43% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.03%\**</span> | 69.57% | 70.18% | 68.34% |
| rmp | 15.78% | 53.31% | 89.64% | 82.60% | 81.93% |
| ron | 1.59% | 11.76% | 43.77% | 59.61% | 65.18% |
| scale | 41.68% | 73.57% | 91.89% | 86.26% | 86.17% |
| serde_json | 6.41% | 27.48% | 38.51% | 56.88% | 63.13% |
| simd-json | 5.79% | 37.54% | 38.51% | 56.88% | 63.13% |
| speedy | 99.00% | 85.76% | 79.44% | 74.27% | 79.56% |
| alkahest | 100.00% | † | 67.29% | 59.33% | 58.53% |
| dlhn | 32.09% | 65.31% | 97.06% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*29.38%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.26%\**</span> | <span title="validated on-demand with error">*4.31%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.54%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.02%\**</span> <span title="validated upfront with error">*0.78%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.74%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*54.99%\**</span> | <span title="validated on-demand with panic">*46.51%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 466.14 µs | <span title="unvalidated">*466.93 µs\**</span> | 6000024 | 5380836 | 5345891 |
| bare | 6.3551 ms | 5.1707 ms | 6000003 | 5380817 | 5345900 |
| bincode | 4.4342 ms | 5.3139 ms | 6000008 | 5380823 | 5345890 |
| bitcode | 10.443 ms | 16.990 ms | 4737624 | 4740613 | 4737741 |
| borsh | 5.5368 ms | 5.9258 ms | 6000004 | 5380818 | 5345889 |
| bson | 44.491 ms | 125.15 ms | 23013911 | 9211138 | 7497811 |
| capnp | 9.6573 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 43.805 ms | 57.594 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 958.95 µs | † | 6000024 | 5380800 | 5345910 |
| nachricht | 211.22 ms | 37.930 ms | 8125037 | 6495174 | 6386940 |
| postcard | 709.00 µs | 1.4455 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*8.7458 ms\**</span> <span title="encode">*5.9466 ms\**</span> | 15.996 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 520.70 µs | <span title="unvalidated">*466.00 µs\**</span> <span title="validated upfront with error">*466.05 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 19.670 ms | 22.744 ms | 8125006 | 6496879 | 6391037 |
| ron | 225.75 ms | 359.37 ms | 22192885 | 9009575 | 8138755 |
| scale | 4.2517 ms | 6.0973 ms | 6000004 | 5380818 | 5345889 |
| serde_json | 106.33 ms | 98.016 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 106.13 ms | 156.06 ms | 39152823 | 16587283 | 14549214 |
| speedy | 464.33 µs | 464.46 µs | 6000004 | 5380818 | 5345889 |
| alkahest | 466.21 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 7.2680 ms | 7.6543 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*2.4772 ns\**</span> | <span title="unvalidated">*252.15 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*194.17 ns\**</span> | <span title="validated on-demand with error">*5.4935 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2710 ns\**</span> <span title="validated upfront with error">*40.810 ns\**</span> | <span title="unvalidated">*83.698 µs\**</span> <span title="validated upfront with error">*83.778 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.5228 ns\**</span> <span title="validated upfront with error">*8.7642 ns\**</span> | <span title="unvalidated">*47.122 µs\**</span> <span title="validated upfront with error">*47.199 µs\**</span> | 239.68 µs |
| alkahest | <span title="validated on-demand with panic">*2.7684 ns\**</span> | <span title="validated on-demand with panic">*83.717 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 99.61% | <span title="unvalidated">*99.47%\**</span> | 78.96% | 88.10% | 88.62% |
| bare | 7.31% | 8.98% | 78.96% | 88.10% | 88.62% |
| bincode | 10.47% | 8.74% | 78.96% | 88.10% | 88.62% |
| bitcode | 4.45% | 2.73% | 100.00% | 100.00% | 100.00% |
| borsh | 8.39% | 7.84% | 78.96% | 88.10% | 88.62% |
| bson | 1.04% | 0.37% | 20.59% | 51.47% | 63.19% |
| capnp | 4.81% | † | 33.84% | 70.44% | 78.30% |
| cbor | 1.06% | 0.81% | 36.10% | 62.98% | 70.09% |
| flatbuffers | 48.42% | † | 78.96% | 88.10% | 88.62% |
| nachricht | 0.22% | 1.22% | 58.31% | 72.99% | 74.18% |
| postcard | 65.49% | 32.13% | 78.96% | 88.10% | 88.62% |
| prost | <span title="populate + encode">*5.31%\**</span> <span title="encode">*7.81%\**</span> | 2.90% | 54.14% | 70.93% | 73.78% |
| rkyv | 89.17% | <span title="unvalidated">*99.67%\**</span> <span title="validated upfront with error">*99.66%\**</span> | 78.96% | 88.10% | 88.62% |
| rmp | 2.36% | 2.04% | 58.31% | 72.97% | 74.13% |
| ron | 0.21% | 0.13% | 21.35% | 52.62% | 58.21% |
| scale | 10.92% | 7.62% | 78.96% | 88.10% | 88.62% |
| serde_json | 0.44% | 0.47% | 18.09% | 49.32% | 55.18% |
| simd-json | 0.44% | 0.30% | 12.10% | 28.58% | 32.56% |
| speedy | 100.00% | 100.00% | 78.96% | 88.10% | 88.62% |
| alkahest | 99.60% | † | 78.96% | 88.10% | 88.62% |
| dlhn | 6.39% | 6.07% | 78.96% | 88.10% | 88.62% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*61.47%\**</span> | <span title="unvalidated">*18.69%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.78%\**</span> | <span title="validated on-demand with error">*0.86%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.55%\**</span> <span title="validated upfront with error">*3.73%\**</span> | <span title="unvalidated">*56.30%\**</span> <span title="validated upfront with error">*56.25%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*17.38%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.84%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*55.01%\**</span> | <span title="validated on-demand with panic">*56.29%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 249.90 µs | <span title="unvalidated">*1.9190 ms\**</span> | 1290592 | 396381 | 339894 |
| bare | 829.02 µs | 3.1684 ms | 356311 | 213270 | 198488 |
| bincode | 626.57 µs | 2.6036 ms | 569975 | 240897 | 232423 |
| bitcode | 704.67 µs | 2.7926 ms | 323111 | 215477 | 201612 |
| borsh | 565.52 µs | 2.6798 ms | 446595 | 234395 | 210008 |
| bson | 4.1565 ms | 11.993 ms | 1619653 | 506953 | 328399 |
| capnp | 613.80 µs | † | 803896 | 336655 | 280851 |
| cbor | 2.0447 ms | 6.5225 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 3.3178 ms | † | 844168 | 346957 | 294015 |
| nachricht | 7.6225 ms | 5.1286 ms | 449745 | 252743 | 231110 |
| postcard | 481.80 µs | 2.6819 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*3.7943 ms\**</span> <span title="encode">*1.3597 ms\**</span> | 4.5558 ms | 596811 | 306728 | 269310 |
| rkyv | 409.03 µs | <span title="unvalidated">*1.8243 ms\**</span> <span title="validated upfront with error">*2.5618 ms\**</span> | 596952 | 254523 | 220366 |
| rmp | 1.7741 ms | 3.9883 ms | 424533 | 245594 | 226188 |
| ron | 9.3046 ms | 20.998 ms | 1465223 | 439761 | 343338 |
| scale | 653.26 µs | 2.6960 ms | 356311 | 213188 | 198524 |
| serde_json | 4.2514 ms | 10.021 ms | 1623191 | 472275 | 359623 |
| simd-json | 4.1299 ms | 6.0857 ms | 1663769 | 496401 | 383682 |
| speedy | 395.36 µs | 2.2960 ms | 449595 | 235136 | 210361 |
| alkahest | 286.37 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*61.247 µs\**</span> | <span title="unvalidated">*61.937 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*124.33 ns\**</span> | <span title="validated on-demand with error">*666.92 ns\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2810 ns\**</span> <span title="validated upfront with error">*2.2481 ms\**</span> | <span title="unvalidated">*2.8935 µs\**</span> <span title="validated upfront with error">*2.2555 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.5219 ns\**</span> <span title="validated upfront with error">*721.29 µs\**</span> | <span title="unvalidated">*189.01 ns\**</span> <span title="validated upfront with error">*723.03 µs\**</span> | 1.5317 µs |
| alkahest | <span title="validated on-demand with panic">*2.7691 ns\**</span> | <span title="validated on-demand with panic">*13.329 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*95.07%\**</span> | 25.04% | 53.78% | 58.40% |
| bare | 30.14% | 57.58% | 90.68% | 99.96% | 100.00% |
| bincode | 39.88% | 70.07% | 56.69% | 88.50% | 85.40% |
| bitcode | 35.46% | 65.33% | 100.00% | 98.94% | 98.45% |
| borsh | 44.19% | 68.08% | 72.35% | 90.95% | 94.51% |
| bson | 6.01% | 15.21% | 19.95% | 42.05% | 60.44% |
| capnp | 40.71% | † | 40.19% | 63.33% | 70.67% |
| cbor | 12.22% | 27.97% | 29.11% | 61.29% | 72.30% |
| flatbuffers | 7.53% | † | 38.28% | 61.45% | 67.51% |
| nachricht | 3.28% | 35.57% | 71.84% | 84.35% | 85.88% |
| postcard | 51.87% | 68.02% | 87.92% | 95.97% | 95.73% |
| prost | <span title="populate + encode">*6.59%\**</span> <span title="encode">*18.38%\**</span> | 40.04% | 54.14% | 69.50% | 73.70% |
| rkyv | 61.10% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.21%\**</span> | 54.13% | 83.76% | 90.07% |
| rmp | 14.09% | 45.74% | 76.11% | 86.81% | 87.75% |
| ron | 2.69% | 8.69% | 22.05% | 48.48% | 57.81% |
| scale | 38.25% | 67.67% | 90.68% | 100.00% | 99.98% |
| serde_json | 5.88% | 18.20% | 19.91% | 45.14% | 55.19% |
| simd-json | 6.05% | 29.98% | 19.42% | 42.95% | 51.73% |
| speedy | 63.21% | 79.46% | 71.87% | 90.67% | 94.36% |
| alkahest | 87.26% | † | 48.40% | 65.49% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.31%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.22%\**</span> | <span title="validated on-demand with error">*28.34%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*46.39%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.53%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*54.96%\**</span> | <span title="validated on-demand with panic">*1.42%\**</span> | ‡ |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
