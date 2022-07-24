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

## Last updated: 2022-7-24

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 294.24 µs | <span title="unvalidated">*2.6656 ms\**</span> | 1705800 | 506933 | 404238 |
| bare | 704.87 µs | 3.4983 ms | 765778 | 312739 | 264630 |
| bincode | 586.93 µs | 3.6365 ms | 1045784 | 374305 | 311761 |
| borsh | 507.42 µs | 3.4736 ms | 885780 | 363280 | 286514 |
| bson | 3.1261 ms | 11.444 ms | 1924682 | 537661 | 376270 |
| capnp | 1.3144 ms | † | 1443216 | 509618 | 428649 |
| cbor | 2.0059 ms | 6.2495 ms | 1407835 | 407372 | 324081 |
| flatbuffers | 1.9725 ms | † | 1276368 | 469962 | 388832 |
| nachricht | 7.5272 ms | 5.9315 ms | 818669 | 334639 | 285514 |
| postcard | 405.10 µs | 3.2539 ms | 724953 | 303462 | 253747 |
| prost | <span title="populate + encode">*3.1138 ms\**</span> <span title="encode">*582.69 µs\**</span> | 3.9637 ms | 764951 | 269811 | 227947 |
| rkyv | 383.40 µs | <span title="unvalidated">*2.8086 ms\**</span> <span title="validated upfront with error">*3.9313 ms\**</span> | 1011488 | 392809 | 331932 |
| rmp | 1.5228 ms | 4.7238 ms | 784997 | 326654 | 278219 |
| ron | 18.174 ms | 19.748 ms | 1607459 | 452648 | 349713 |
| serde_json | 4.7979 ms | 9.0660 ms | 1827461 | 474358 | 361090 |
| simd-json | 4.5000 ms | 7.4627 ms | 1827461 | 474358 | 361090 |
| speedy | 371.19 µs | 3.4305 ms | 885780 | 363280 | 286514 |
| alkahest | 287.99 µs | † | 1045784 | 454748 | 389424 |
| dlhn | 854.45 µs | 4.4512 ms | 724953 | 302512 | 253629 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*44.347 µs\**</span> | <span title="unvalidated">*71.832 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*123.74 ns\**</span> | <span title="validated on-demand with error">*485.91 µs\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.2465 ns\**</span> <span title="validated upfront with error">*2.6021 ms\**</span> | <span title="unvalidated">*97.539 µs\**</span> <span title="validated upfront with error">*2.6439 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.8134 ns\**</span> <span title="validated upfront with error">*934.58 µs\**</span> | <span title="unvalidated">*17.190 µs\**</span> <span title="validated upfront with error">*893.62 µs\**</span> | 14.968 µs |
| alkahest | <span title="validated on-demand with panic">*2.7619 ns\**</span> | <span title="validated on-demand with panic">*63.281 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 97.88% | <span title="unvalidated">*100.00%\**</span> | 42.50% | 53.22% | 56.39% |
| bare | 40.86% | 76.20% | 94.67% | 86.27% | 86.14% |
| bincode | 49.07% | 73.30% | 69.32% | 72.08% | 73.12% |
| borsh | 56.76% | 76.74% | 81.84% | 74.27% | 79.56% |
| bson | 9.21% | 23.29% | 37.67% | 50.18% | 60.58% |
| capnp | 21.91% | † | 50.23% | 52.94% | 53.18% |
| cbor | 14.36% | 42.65% | 51.49% | 66.23% | 70.34% |
| flatbuffers | 14.60% | † | 56.80% | 57.41% | 58.62% |
| nachricht | 3.83% | 44.94% | 88.55% | 80.63% | 79.84% |
| postcard | 71.09% | 81.92% | 100.00% | 88.91% | 89.83% |
| prost | <span title="populate + encode">*9.25%\**</span> <span title="encode">*49.42%\**</span> | 67.25% | 94.77% | 100.00% | 100.00% |
| rkyv | 75.11% | <span title="unvalidated">*94.91%\**</span> <span title="validated upfront with error">*67.80%\**</span> | 71.67% | 68.69% | 68.67% |
| rmp | 18.91% | 56.43% | 92.35% | 82.60% | 81.93% |
| ron | 1.58% | 13.50% | 45.10% | 59.61% | 65.18% |
| serde_json | 6.00% | 29.40% | 39.67% | 56.88% | 63.13% |
| simd-json | 6.40% | 35.72% | 39.67% | 56.88% | 63.13% |
| speedy | 77.59% | 77.70% | 81.84% | 74.27% | 79.56% |
| alkahest | 100.00% | † | 69.32% | 59.33% | 58.53% |
| dlhn | 33.70% | 59.88% | 100.00% | 89.19% | 89.87% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*23.93%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.47%\**</span> | <span title="validated on-demand with error">*3.54%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*55.86%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.62%\**</span> <span title="validated upfront with error">*0.65%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.92%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*65.66%\**</span> | <span title="validated on-demand with panic">*27.16%\**</span> | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 534.19 µs | <span title="unvalidated">*526.09 µs\**</span> | 6000024 | 5380836 | 5345889 |
| bare | 8.2287 ms | 6.8687 ms | 6000003 | 5380817 | 5345900 |
| bincode | 6.4404 ms | 6.2545 ms | 6000008 | 5380823 | 5345890 |
| borsh | 8.8123 ms | 6.6706 ms | 6000004 | 5380818 | 5345889 |
| bson | 65.038 ms | 155.44 ms | 23013911 | 9211138 | 7497811 |
| capnp | 16.565 ms | † | 14000088 | 6729881 | 6051062 |
| cbor | 52.548 ms | 62.742 ms | 13122324 | 7527423 | 6759658 |
| flatbuffers | 1.1173 ms | † | 6000024 | 5380800 | 5345910 |
| nachricht | 182.65 ms | 52.550 ms | 8125037 | 6495174 | 6386940 |
| postcard | 1.1384 ms | 1.7790 ms | 6000003 | 5380817 | 5345900 |
| prost | <span title="populate + encode">*14.555 ms\**</span> <span title="encode">*11.223 ms\**</span> | 20.870 ms | 8750000 | 6683814 | 6421871 |
| rkyv | 588.74 µs | <span title="unvalidated">*508.27 µs\**</span> <span title="validated upfront with error">*511.63 µs\**</span> | 6000008 | 5380822 | 5345892 |
| rmp | 19.776 ms | 20.499 ms | 8125006 | 6496879 | 6391037 |
| ron | 237.80 ms | 313.07 ms | 22192885 | 9009575 | 8138755 |
| serde_json | 111.61 ms | 96.813 ms | 26192883 | 9612105 | 8586741 |
| simd-json | 106.46 ms | 150.97 ms | 39152823 | 16587283 | 14549214 |
| speedy | 1.8451 ms | 992.83 µs | 6000004 | 5380818 | 5345889 |
| alkahest | 521.43 µs | † | 6000008 | 5380823 | 5345890 |
| dlhn | 7.2275 ms | 7.9229 ms | 6000003 | 5380817 | 5345900 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*3.1660 ns\**</span> | <span title="unvalidated">*288.88 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*228.39 ns\**</span> | <span title="validated on-demand with error">*7.6195 ms\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.9283 ns\**</span> <span title="validated upfront with error">*76.794 ns\**</span> | <span title="unvalidated">*56.468 µs\**</span> <span title="validated upfront with error">*48.350 µs\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.6431 ns\**</span> <span title="validated upfront with error">*19.362 ns\**</span> | <span title="unvalidated">*46.623 µs\**</span> <span title="validated upfront with error">*41.336 µs\**</span> | 258.63 µs |
| alkahest | <span title="validated on-demand with panic">*2.4242 ns\**</span> | <span title="validated on-demand with panic">*88.944 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 97.61% | <span title="unvalidated">*96.61%\**</span> | 100.00% | 100.00% | 100.00% |
| bare | 6.34% | 7.40% | 100.00% | 100.00% | 100.00% |
| bincode | 8.10% | 8.13% | 100.00% | 100.00% | 100.00% |
| borsh | 5.92% | 7.62% | 100.00% | 100.00% | 100.00% |
| bson | 0.80% | 0.33% | 26.07% | 58.42% | 71.30% |
| capnp | 3.15% | † | 42.86% | 79.95% | 88.35% |
| cbor | 0.99% | 0.81% | 45.72% | 71.48% | 79.09% |
| flatbuffers | 46.67% | † | 100.00% | 100.00% | 100.00% |
| nachricht | 0.29% | 0.97% | 73.85% | 82.84% | 83.70% |
| postcard | 45.80% | 28.57% | 100.00% | 100.00% | 100.00% |
| prost | <span title="populate + encode">*3.58%\**</span> <span title="encode">*4.65%\**</span> | 2.44% | 68.57% | 80.50% | 83.25% |
| rkyv | 88.57% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.34%\**</span> | 100.00% | 100.00% | 100.00% |
| rmp | 2.64% | 2.48% | 73.85% | 82.82% | 83.65% |
| ron | 0.22% | 0.16% | 27.04% | 59.72% | 65.68% |
| serde_json | 0.47% | 0.53% | 22.91% | 55.98% | 62.26% |
| simd-json | 0.49% | 0.34% | 15.32% | 32.44% | 36.74% |
| speedy | 28.26% | 51.19% | 100.00% | 100.00% | 100.00% |
| alkahest | 100.00% | † | 100.00% | 100.00% | 100.00% |
| dlhn | 7.21% | 6.42% | 100.00% | 100.00% | 100.00% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*51.90%\**</span> | <span title="unvalidated">*14.31%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*0.72%\**</span> | <span title="validated on-demand with error">*0.54%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*41.83%\**</span> <span title="validated upfront with error">*2.14%\**</span> | <span title="unvalidated">*73.20%\**</span> <span title="validated upfront with error">*85.49%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*8.49%\**</span> | <span title="unvalidated">*88.66%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*67.78%\**</span> | <span title="validated on-demand with panic">*46.47%\**</span> | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 286.23 µs | <span title="unvalidated">*2.0057 ms\**</span> | 1290592 | 394008 | 330146 |
| bare | 908.82 µs | 3.3590 ms | 356311 | 213270 | 198488 |
| bincode | 721.90 µs | 2.6703 ms | 569975 | 240897 | 232423 |
| borsh | 617.53 µs | 2.6380 ms | 446595 | 234395 | 210008 |
| bson | 4.5307 ms | 11.877 ms | 1619653 | 506953 | 328399 |
| capnp | 896.45 µs | † | 803896 | 336655 | 280851 |
| cbor | 2.0712 ms | 6.2772 ms | 1109821 | 347812 | 274526 |
| flatbuffers | 3.6071 ms | † | 844168 | 346957 | 294015 |
| nachricht | 7.0567 ms | 5.6754 ms | 449745 | 252743 | 231110 |
| postcard | 487.82 µs | 2.9694 ms | 367489 | 222144 | 207344 |
| prost | <span title="populate + encode">*4.1798 ms\**</span> <span title="encode">*1.4675 ms\**</span> | 5.0108 ms | 596811 | 306728 | 269310 |
| rkyv | 500.03 µs | <span title="unvalidated">*1.8361 ms\**</span> <span title="validated upfront with error">*2.7131 ms\**</span> | 596952 | 254571 | 219976 |
| rmp | 1.6617 ms | 3.9549 ms | 424533 | 245594 | 226188 |
| ron | 9.9961 ms | 19.224 ms | 1465223 | 439761 | 343338 |
| serde_json | 4.8292 ms | 11.032 ms | 1623191 | 472275 | 359623 |
| simd-json | 4.6787 ms | 6.7544 ms | 1663769 | 496401 | 383682 |
| speedy | 450.65 µs | 2.4272 ms | 449595 | 235136 | 210361 |
| alkahest | 334.23 µs | † | 667570 | 325536 | 320452 |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*49.561 µs\**</span> | <span title="unvalidated">*52.204 µs\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*117.69 ns\**</span> | <span title="validated on-demand with error">*837.94 ns\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*3.1576 ns\**</span> <span title="validated upfront with error">*2.8539 ms\**</span> | <span title="unvalidated">*2.9315 µs\**</span> <span title="validated upfront with error">*2.9533 ms\**</span> | ‡ |
| rkyv | <span title="unvalidated">*1.7161 ns\**</span> <span title="validated upfront with error">*683.50 µs\**</span> | <span title="unvalidated">*193.03 ns\**</span> <span title="validated upfront with error">*681.60 µs\**</span> | 1.5292 µs |
| alkahest | <span title="validated on-demand with panic">*2.5830 ns\**</span> | <span title="validated on-demand with panic">*46.844 µs\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Format / Lib | Serialize | Deserialize | Size | Zlib | Zstd |
|---|--:|--:|--:|--:|--:|
| abomonation | 100.00% | <span title="unvalidated">*91.54%\**</span> | 27.61% | 54.13% | 60.12% |
| bare | 31.49% | 54.66% | 100.00% | 100.00% | 100.00% |
| bincode | 39.65% | 68.76% | 62.51% | 88.53% | 85.40% |
| borsh | 46.35% | 69.60% | 79.78% | 90.99% | 94.51% |
| bson | 6.32% | 15.46% | 22.00% | 42.07% | 60.44% |
| capnp | 31.93% | † | 44.32% | 63.35% | 70.67% |
| cbor | 13.82% | 29.25% | 32.11% | 61.32% | 72.30% |
| flatbuffers | 7.94% | † | 42.21% | 61.47% | 67.51% |
| nachricht | 4.06% | 32.35% | 79.23% | 84.38% | 85.88% |
| postcard | 58.68% | 61.83% | 96.96% | 96.01% | 95.73% |
| prost | <span title="populate + encode">*6.85%\**</span> <span title="encode">*19.50%\**</span> | 36.64% | 59.70% | 69.53% | 73.70% |
| rkyv | 57.24% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*67.68%\**</span> | 59.69% | 83.78% | 90.23% |
| rmp | 17.23% | 46.43% | 83.93% | 86.84% | 87.75% |
| ron | 2.86% | 9.55% | 24.32% | 48.50% | 57.81% |
| serde_json | 5.93% | 16.64% | 21.95% | 45.16% | 55.19% |
| simd-json | 6.12% | 27.18% | 21.42% | 42.96% | 51.73% |
| speedy | 63.51% | 75.65% | 79.25% | 90.70% | 94.36% |
| alkahest | 85.64% | † | 53.37% | 65.51% | 61.94% |

#### Zero-copy deserialization speed

| Format / Lib | Access | Read | Update |
|---|--:|--:|--:|
| abomonation | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.37%\**</span> | ‡ |
| capnp | <span title="validated on-demand with error">*1.46%\**</span> | <span title="validated on-demand with error">*23.04%\**</span> | ‡ |
| flatbuffers | <span title="unvalidated">*54.35%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*6.58%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| rkyv | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |
| alkahest | <span title="validated on-demand with panic">*66.44%\**</span> | <span title="validated on-demand with panic">*0.41%\**</span> | ‡ |



## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
