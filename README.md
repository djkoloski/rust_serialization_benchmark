<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

## [Interactive site](https://djkoloski.github.io/rust_serialization_benchmark/)

Calculate the number of messages per second that can be sent/received with various rust serialization frameworks and compression libraries.
[Documentation](pages/README.md)

## Format

All tests benchmark the following properties (time or size):

* **Serialize**: serialize data into a buffer
* **Deserialize**: deserializes a buffer into a normal rust object
* **Borrow**: deserializes a buffer into a rust object that borrows string data from the input, with lifetime
* **Size**: the size of the buffer when serialized
* **Zlib**: the size of the buffer after zlib compression
* **Zstd**: the size of the buffer after zstd compression
* **Zstd Time**: the time taken to compress the serialized buffer with zstd

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2025-02-08 03:48:13

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.86.0-nightly (a9e7b3048 2025-02-07)
binary: rustc
commit-hash: a9e7b30487235621751cc628f170c0f15fb215c4
commit-date: 2025-02-07
host: x86_64-unknown-linux-gnu
release: 1.86.0-nightly
LLVM version: 19.1.7
```

### CPU info

```
Architecture:                         x86_64
CPU op-mode(s):                       32-bit, 64-bit
Address sizes:                        48 bits physical, 48 bits virtual
Byte Order:                           Little Endian
CPU(s):                               4
On-line CPU(s) list:                  0-3
Vendor ID:                            AuthenticAMD
Model name:                           AMD EPYC 7763 64-Core Processor
CPU family:                           25
Model:                                1
Thread(s) per core:                   2
Core(s) per socket:                   2
Socket(s):                            1
Stepping:                             1
BogoMIPS:                             4890.85
Flags:                                fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves user_shstk clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
Virtualization:                       AMD-V
Hypervisor vendor:                    Microsoft
Virtualization type:                  full
L1d cache:                            64 KiB (2 instances)
L1i cache:                            64 KiB (2 instances)
L2 cache:                             1 MiB (2 instances)
L3 cache:                             32 MiB (1 instance)
NUMA node(s):                         1
NUMA node0 CPU(s):                    0-3
Vulnerability Gather data sampling:   Not affected
Vulnerability Itlb multihit:          Not affected
Vulnerability L1tf:                   Not affected
Vulnerability Mds:                    Not affected
Vulnerability Meltdown:               Not affected
Vulnerability Mmio stale data:        Not affected
Vulnerability Reg file data sampling: Not affected
Vulnerability Retbleed:               Not affected
Vulnerability Spec rstack overflow:   Vulnerable: Safe RET, no microcode
Vulnerability Spec store bypass:      Vulnerable
Vulnerability Spectre v1:             Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:             Mitigation; Retpolines; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
Vulnerability Srbds:                  Not affected
Vulnerability Tsx async abort:        Not affected
```

</details>

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*507.62 µs\**</span> <span title="prepend">*432.44 µs\**</span> | 2.5585 ms | 932.03 µs | 804955 | 328941 | 285485 | 4.4763 ms |
| [bincode 2.0.0-rc][bincode] | 323.49 µs | 2.4564 ms | † | 741295 | 303944 | 257153 | 3.9385 ms |
| [bincode 1.3.3][bincode1] | 523.59 µs | 2.0803 ms | 603.34 µs | 1045784 | 373127 | 311761 | 4.7526 ms |
| [bitcode 0.6.3][bitcode] | 136.85 µs | 1.4377 ms | 62.384 µs | 703710 | 288826 | 229755 | 2.3815 ms |
| [borsh 1.5.3][borsh] | 547.26 µs | 2.1383 ms | † | 885780 | 362204 | 286514 | 4.4694 ms |
| [capnp 0.20.3][capnp] | 492.99 µs | † | † | 1443216 | 513986 | 428649 | 6.2507 ms |
| [cbor4ii 0.3.3][cbor4ii] | 702.48 µs | 4.9251 ms | 3.4455 ms | 1407835 | 403440 | 324081 | 5.0143 ms |
| [ciborium 0.2.2][ciborium] | 4.1718 ms | 11.735 ms | † | 1407835 | 403440 | 324081 | 4.9814 ms |
| [databuf 0.5.0][databuf] | 273.56 µs | 2.0054 ms | 674.02 µs | 765778 | 311715 | 264630 | 3.7937 ms |
| [dlhn 0.1.7][dlhn] | 769.88 µs | 2.5530 ms | † | 724953 | 301446 | 253629 | 3.5126 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0092 ms | † | † | 1276368 | 468539 | 388832 | 5.1318 ms |
| [minicbor 0.25.1][minicbor] | 745.21 µs | 3.0365 ms | 1.4117 ms | 817830 | 332671 | 284548 | 4.2865 ms |
| [msgpacker 0.4.5][msgpacker] | 1.2230 ms | 2.5674 ms | † | 764996 | 315291 | 264898 | 3.9026 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.6284 ms | 4.0610 ms | 2.6010 ms | 818669 | 332556 | 285514 | 4.5704 ms |
| [nanoserde 0.1.37][nanoserde] | 251.16 µs | 2.0498 ms | † | 1045784 | 373127 | 311761 | 4.4532 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 670.12 µs | 2.1818 ms | † | 765778 | 311743 | 264518 | 4.0557 ms |
| [postcard 1.1.1][postcard] | 415.56 µs | 2.1662 ms | 647.69 µs | 724953 | 302399 | 253747 | 3.7714 ms |
| [pot 3.0.1][pot] | 2.4300 ms | 6.5086 ms | 4.8249 ms | 971922 | 372513 | 304122 | 4.5096 ms |
| [prost 0.13.4][prost] | <span title="encode">*976.06 µs\**</span> <span title="populate + encode">*2.4663 ms\**</span> | 3.1770 ms | † | 884628 | 363130 | 315494 | 4.7010 ms |
| [rkyv 0.8.9][rkyv] | 255.74 µs | <span title="unvalidated">*1.5577 ms\**</span> <span title="validated upfront with error">*2.2058 ms\**</span> | † | 1011488 | 393526 | 326517 | 4.8500 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4626 ms | 3.1887 ms | 1.4003 ms | 784997 | 325384 | 278219 | 4.0618 ms |
| [ron 0.8.1][ron] | 11.377 ms | 15.126 ms | 13.557 ms | 1607459 | 449158 | 349713 | 5.6450 ms |
| [savefile 0.18.5][savefile] | 193.49 µs | 2.1501 ms | † | 1045800 | 373139 | 311761 | 4.5146 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5275 ms | 4.7821 ms | 3.1021 ms | 1584946 | 413733 | 341439 | 4.8508 ms |
| [serde_bare 0.5.0][serde_bare] | 700.64 µs | 2.0834 ms | † | 765778 | 311715 | 264630 | 3.8218 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0642 ms | 4.7414 ms | 3.1808 ms | 1407835 | 403440 | 324081 | 5.0216 ms |
| [serde_json 1.0.134][serde_json] | 3.7855 ms | 6.0413 ms | † | 1827461 | 470560 | 361090 | 5.4860 ms |
| [simd-json 0.14.3][simd-json] | 2.1505 ms | 4.6232 ms | † | 1827461 | 470560 | 361090 | 5.9753 ms |
| [speedy 0.8.7][speedy] | 198.69 µs | 1.7371 ms | 370.43 µs | 885780 | 362204 | 286514 | 4.1801 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.942 ns\**</span> | <span title="validated on-demand with error">*169.28 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4871 ns\**</span> <span title="validated upfront with error">*1.9946 ms\**</span> | <span title="unvalidated">*51.691 µs\**</span> <span title="validated upfront with error">*2.3313 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2492 ns\**</span> <span title="validated upfront with error">*656.54 µs\**</span> | <span title="unvalidated">*10.338 µs\**</span> <span title="validated upfront with error">*665.15 µs\**</span> | <span title="unvalidated">*7.5907 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*26.96%\**</span> <span title="prepend">*31.65%\**</span> | 56.19% | 6.69% | 87.42% | 87.80% | 80.48% | 53.20% |
| [bincode 2.0.0-rc][bincode] | 42.30% | 58.53% | † | 94.93% | 95.03% | 89.35% | 60.47% |
| [bincode 1.3.3][bincode1] | 26.14% | 69.11% | 10.34% | 67.29% | 77.41% | 73.70% | 50.11% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 25.01% | 67.24% | † | 79.45% | 79.74% | 80.19% | 53.28% |
| [capnp 0.20.3][capnp] | 27.76% | † | † | 48.76% | 56.19% | 53.60% | 38.10% |
| [cbor4ii 0.3.3][cbor4ii] | 19.48% | 29.19% | 1.81% | 49.99% | 71.59% | 70.89% | 47.49% |
| [ciborium 0.2.2][ciborium] | 3.28% | 12.25% | † | 49.99% | 71.59% | 70.89% | 47.81% |
| [databuf 0.5.0][databuf] | 50.03% | 71.69% | 9.26% | 91.89% | 92.66% | 86.82% | 62.78% |
| [dlhn 0.1.7][dlhn] | 17.78% | 56.31% | † | 97.07% | 95.81% | 90.59% | 67.80% |
| [flatbuffers 24.12.23][flatbuffers] | 13.56% | † | † | 55.13% | 61.64% | 59.09% | 46.41% |
| [minicbor 0.25.1][minicbor] | 18.36% | 47.35% | 4.42% | 86.05% | 86.82% | 80.74% | 55.56% |
| [msgpacker 0.4.5][msgpacker] | 11.19% | 56.00% | † | 91.99% | 91.61% | 86.73% | 61.02% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.43% | 35.40% | 2.40% | 85.96% | 86.85% | 80.47% | 52.11% |
| [nanoserde 0.1.37][nanoserde] | 54.49% | 70.14% | † | 67.29% | 77.41% | 73.70% | 53.48% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.42% | 65.90% | † | 91.89% | 92.65% | 86.86% | 58.72% |
| [postcard 1.1.1][postcard] | 32.93% | 66.37% | 9.63% | 97.07% | 95.51% | 90.54% | 63.15% |
| [pot 3.0.1][pot] | 5.63% | 22.09% | 1.29% | 72.40% | 77.53% | 75.55% | 52.81% |
| [prost 0.13.4][prost] | <span title="encode">*14.02%\**</span> <span title="populate + encode">*5.55%\**</span> | 45.25% | † | 79.55% | 79.54% | 72.82% | 50.66% |
| [rkyv 0.8.9][rkyv] | 53.51% | <span title="unvalidated">*92.30%\**</span> <span title="validated upfront with error">*65.18%\**</span> | † | 69.57% | 73.39% | 70.37% | 49.10% |
| [rmp-serde 1.3.0][rmp-serde] | 9.36% | 45.09% | 4.46% | 89.64% | 88.76% | 82.58% | 58.63% |
| [ron 0.8.1][ron] | 1.20% | 9.50% | 0.46% | 43.78% | 64.30% | 65.70% | 42.19% |
| [savefile 0.18.5][savefile] | 70.73% | 66.87% | † | 67.29% | 77.40% | 73.70% | 52.75% |
| [serde-brief 0.1.0][serde-brief] | 8.96% | 30.06% | 2.01% | 44.40% | 69.81% | 67.29% | 49.09% |
| [serde_bare 0.5.0][serde_bare] | 19.53% | 69.01% | † | 91.89% | 92.66% | 86.82% | 62.31% |
| [serde_cbor 0.11.2][serde_cbor] | 6.63% | 30.32% | 1.96% | 49.99% | 71.59% | 70.89% | 47.43% |
| [serde_json 1.0.134][serde_json] | 3.62% | 23.80% | † | 38.51% | 61.38% | 63.63% | 43.41% |
| [simd-json 0.14.3][simd-json] | 6.36% | 31.10% | † | 38.51% | 61.38% | 63.63% | 39.86% |
| [speedy 0.8.7][speedy] | 68.88% | 82.76% | 16.84% | 79.45% | 79.74% | 80.19% | 56.97% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*6.11%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.23%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.00%\**</span> <span title="validated upfront with error">*0.44%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.55%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*6.6781 ms\**</span> <span title="prepend">*8.8716 ms\**</span> | 8.3848 ms | 8625005 | 6443961 | 6231572 | 76.286 ms |
| [bincode 2.0.0-rc][bincode] | 2.8778 ms | 1.0592 ms | 6000005 | 5378497 | 5345897 | 7.5114 ms |
| [bincode 1.3.3][bincode1] | 5.1860 ms | 4.0957 ms | 6000008 | 5378500 | 5345890 | 7.8103 ms |
| [bitcode 0.6.3][bitcode] | 1.4198 ms | 796.13 µs | 6000006 | 5182295 | 4923880 | 12.614 ms |
| [borsh 1.5.3][borsh] | 6.1743 ms | 4.1925 ms | 6000004 | 5378496 | 5345889 | 7.8471 ms |
| [capnp 0.20.3][capnp] | 5.6347 ms | † | 14000088 | 7130367 | 6051062 | 79.325 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.9051 ms | 48.535 ms | 13125016 | 7524114 | 6757967 | 92.627 ms |
| [ciborium 0.2.2][ciborium] | 67.077 ms | 118.67 ms | 13122324 | 7524660 | 6759658 | 92.663 ms |
| [databuf 0.5.0][databuf] | 2.4099 ms | 5.3410 ms | 6000003 | 5378495 | 5345900 | 7.4797 ms |
| [dlhn 0.1.7][dlhn] | 6.3681 ms | 6.8074 ms | 6000003 | 5378495 | 5345900 | 7.4620 ms |
| [flatbuffers 24.12.23][flatbuffers] | 880.05 µs | † | 6000024 | 5378434 | 5345910 | 7.9832 ms |
| [minicbor 0.25.1][minicbor] | 5.1962 ms | 11.412 ms | 8125006 | 6494907 | 6390894 | 69.402 ms |
| [msgpacker 0.4.5][msgpacker] | 18.711 ms | 5.2359 ms | 7500005 | 6058442 | 6014337 | 9.6818 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.27 ms | 32.507 ms | 8125037 | 6493484 | 6386940 | 72.908 ms |
| [nanoserde 0.1.37][nanoserde] | 1.6516 ms | 1.1046 ms | 6000008 | 5378500 | 5345890 | 7.7785 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1452 ms | 4.5441 ms | 6000004 | 5378496 | 5345889 | 7.8848 ms |
| [postcard 1.1.1][postcard] | 510.41 µs | 1.1721 ms | 6000003 | 5378495 | 5345900 | 7.5265 ms |
| [pot 3.0.1][pot] | 39.592 ms | 69.727 ms | 10122342 | 6814618 | 6852251 | 81.712 ms |
| [prost 0.13.4][prost] | <span title="encode">*7.8464 ms\**</span> <span title="populate + encode">*8.3534 ms\**</span> | 13.141 ms | 8750000 | 6665735 | 6421871 | 75.607 ms |
| [rkyv 0.8.9][rkyv] | 197.52 µs | <span title="unvalidated">*148.52 µs\**</span> <span title="validated upfront with error">*148.50 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.8908 ms |
| [rmp-serde 1.3.0][rmp-serde] | 19.030 ms | 17.828 ms | 8125006 | 6494876 | 6391037 | 72.438 ms |
| [ron 0.8.1][ron] | 170.86 ms | 241.37 ms | 22192885 | 8970395 | 8138755 | 147.21 ms |
| [savefile 0.18.5][savefile] | 147.72 µs | 197.81 µs | 6000024 | 5378519 | 5345892 | 7.9572 ms |
| [serde-brief 0.1.0][serde-brief] | 22.670 ms | 37.556 ms | 15750015 | 8024540 | 6816643 | 92.416 ms |
| [serde_bare 0.5.0][serde_bare] | 5.2043 ms | 4.7041 ms | 6000003 | 5378495 | 5345900 | 7.5497 ms |
| [serde_cbor 0.11.2][serde_cbor] | 33.538 ms | 46.894 ms | 13122324 | 7524660 | 6759658 | 93.599 ms |
| [serde_json 1.0.134][serde_json] | 88.016 ms | 92.665 ms | 26192883 | 9566084 | 8586741 | 153.93 ms |
| [simd-json 0.14.3][simd-json] | 51.682 ms | 70.183 ms | 26192883 | 9566084 | 8586741 | 153.85 ms |
| [speedy 0.8.7][speedy] | 197.33 µs | 198.09 µs | 6000004 | 5378496 | 5345889 | 7.9183 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*112.61 ns\**</span> | <span title="validated on-demand with error">*2.1912 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4877 ns\**</span> <span title="validated upfront with error">*39.453 ns\**</span> | <span title="unvalidated">*54.369 µs\**</span> <span title="validated upfront with error">*77.765 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2428 ns\**</span> <span title="validated upfront with error">*4.9769 ns\**</span> | <span title="unvalidated">*48.573 µs\**</span> <span title="validated upfront with error">*38.891 µs\**</span> | <span title="unvalidated">*99.552 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*2.21%\**</span> <span title="prepend">*1.67%\**</span> | 1.77% | 69.57% | 80.42% | 79.02% | 9.78% |
| [bincode 2.0.0-rc][bincode] | 5.13% | 14.02% | 100.00% | 96.35% | 92.11% | 99.34% |
| [bincode 1.3.3][bincode1] | 2.85% | 3.63% | 100.00% | 96.35% | 92.11% | 95.54% |
| [bitcode 0.6.3][bitcode] | 10.40% | 18.65% | 100.00% | 100.00% | 100.00% | 59.16% |
| [borsh 1.5.3][borsh] | 2.39% | 3.54% | 100.00% | 96.35% | 92.11% | 95.09% |
| [capnp 0.20.3][capnp] | 2.62% | † | 42.86% | 72.68% | 81.37% | 9.41% |
| [cbor4ii 0.3.3][cbor4ii] | 1.49% | 0.31% | 45.71% | 68.88% | 72.86% | 8.06% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.84% | 8.05% |
| [databuf 0.5.0][databuf] | 6.13% | 2.78% | 100.00% | 96.35% | 92.11% | 99.76% |
| [dlhn 0.1.7][dlhn] | 2.32% | 2.18% | 100.00% | 96.35% | 92.11% | 100.00% |
| [flatbuffers 24.12.23][flatbuffers] | 16.79% | † | 100.00% | 96.35% | 92.11% | 93.47% |
| [minicbor 0.25.1][minicbor] | 2.84% | 1.30% | 73.85% | 79.79% | 77.05% | 10.75% |
| [msgpacker 0.4.5][msgpacker] | 0.79% | 2.84% | 80.00% | 85.54% | 81.87% | 77.07% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.46% | 73.85% | 79.81% | 77.09% | 10.23% |
| [nanoserde 0.1.37][nanoserde] | 8.94% | 13.44% | 100.00% | 96.35% | 92.11% | 95.93% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.87% | 3.27% | 100.00% | 96.35% | 92.11% | 94.64% |
| [postcard 1.1.1][postcard] | 28.94% | 12.67% | 100.00% | 96.35% | 92.11% | 99.14% |
| [pot 3.0.1][pot] | 0.37% | 0.21% | 59.27% | 76.05% | 71.86% | 9.13% |
| [prost 0.13.4][prost] | <span title="encode">*1.88%\**</span> <span title="populate + encode">*1.77%\**</span> | 1.13% | 68.57% | 77.75% | 76.67% | 9.87% |
| [rkyv 0.8.9][rkyv] | 74.79% | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 94.57% |
| [rmp-serde 1.3.0][rmp-serde] | 0.78% | 0.83% | 73.85% | 79.79% | 77.04% | 10.30% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 5.07% |
| [savefile 0.18.5][savefile] | 100.00% | 75.07% | 100.00% | 96.35% | 92.11% | 93.78% |
| [serde-brief 0.1.0][serde-brief] | 0.65% | 0.40% | 38.10% | 64.58% | 72.23% | 8.07% |
| [serde_bare 0.5.0][serde_bare] | 2.84% | 3.16% | 100.00% | 96.35% | 92.11% | 98.84% |
| [serde_cbor 0.11.2][serde_cbor] | 0.44% | 0.32% | 45.72% | 68.87% | 72.84% | 7.97% |
| [serde_json 1.0.134][serde_json] | 0.17% | 0.16% | 22.91% | 54.17% | 57.34% | 4.85% |
| [simd-json 0.14.3][simd-json] | 0.29% | 0.21% | 22.91% | 54.17% | 57.34% | 4.85% |
| [speedy 0.8.7][speedy] | 74.86% | 74.97% | 100.00% | 96.35% | 92.11% | 94.24% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.10%\**</span> | <span title="validated on-demand with error">*1.77%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*3.15%\**</span> | <span title="unvalidated">*71.53%\**</span> <span title="validated upfront with error">*50.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.97%\**</span> | <span title="unvalidated">*80.07%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*948.52 µs\**</span> <span title="prepend">*838.35 µs\**</span> | 3.1292 ms | 1.7240 ms | 489348 | 281173 | 249546 | 3.0738 ms |
| [bincode 2.0.0-rc][bincode] | 312.57 µs | 2.0799 ms | † | 367413 | 221291 | 206273 | 2.4693 ms |
| [bincode 1.3.3][bincode1] | 599.34 µs | 1.7971 ms | 838.76 µs | 569975 | 240525 | 232423 | 2.8513 ms |
| [bitcode 0.6.3][bitcode] | 128.46 µs | 1.2473 ms | 170.77 µs | 327688 | 200947 | 182736 | 752.52 µs |
| [borsh 1.5.3][borsh] | 561.31 µs | 1.7819 ms | † | 446595 | 234236 | 210008 | 2.4540 ms |
| [capnp 0.20.3][capnp] | 467.97 µs | † | † | 803896 | 335606 | 280851 | 3.8858 ms |
| [cbor4ii 0.3.3][cbor4ii] | 821.35 µs | 4.7556 ms | 3.5473 ms | 1109831 | 344745 | 274514 | 3.7899 ms |
| [ciborium 0.2.2][ciborium] | 3.7284 ms | 9.8718 ms | † | 1109821 | 344751 | 274526 | 3.7747 ms |
| [databuf 0.5.0][databuf] | 324.40 µs | 1.7333 ms | 780.14 µs | 356311 | 213062 | 198488 | 2.3784 ms |
| [dlhn 0.1.7][dlhn] | 790.04 µs | 2.5957 ms | † | 366496 | 220600 | 205683 | 2.4786 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.1904 ms | † | † | 844168 | 345696 | 294015 | 3.8250 ms |
| [minicbor 0.25.1][minicbor] | 526.76 µs | 3.3432 ms | 1.8872 ms | 428773 | 249857 | 228741 | 2.7186 ms |
| [msgpacker 0.4.5][msgpacker] | 961.10 µs | 2.8636 ms | † | 391251 | 236877 | 220476 | 2.6130 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2207 ms | 4.0727 ms | 2.9797 ms | 449745 | 252432 | 231110 | 2.7406 ms |
| [nanoserde 0.1.37][nanoserde] | 269.56 µs | 1.9113 ms | † | 567975 | 239930 | 232419 | 2.8608 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 624.31 µs | 1.9534 ms | † | 356311 | 212976 | 198524 | 2.3905 ms |
| [postcard 1.1.1][postcard] | 444.15 µs | 2.0077 ms | 829.73 µs | 367489 | 221913 | 207344 | 2.4867 ms |
| [pot 3.0.1][pot] | 2.3941 ms | 6.0544 ms | 5.1153 ms | 599125 | 299158 | 247693 | 3.1576 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.2820 ms\**</span> <span title="populate + encode">*2.9598 ms\**</span> | 3.3143 ms | † | 596811 | 305319 | 269310 | 3.4586 ms |
| [rkyv 0.8.9][rkyv] | 334.71 µs | <span title="unvalidated">*1.4893 ms\**</span> <span title="validated upfront with error">*2.0309 ms\**</span> | † | 603776 | 254776 | 220087 | 2.7036 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5975 ms | 3.0377 ms | 1.7336 ms | 424533 | 245214 | 226188 | 2.6790 ms |
| [ron 0.8.1][ron] | 7.3104 ms | 17.192 ms | 15.494 ms | 1465223 | 434935 | 343338 | 5.7357 ms |
| [savefile 0.18.5][savefile] | 221.40 µs | 1.8076 ms | † | 566991 | 239362 | 232010 | 2.8588 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3493 ms | 5.3904 ms | 3.7794 ms | 1276014 | 373898 | 293679 | 4.0572 ms |
| [serde_bare 0.5.0][serde_bare] | 756.61 µs | 2.3320 ms | † | 356311 | 213062 | 198488 | 2.3747 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8635 ms | 4.7407 ms | 3.5904 ms | 1109821 | 344751 | 274526 | 3.7701 ms |
| [serde_json 1.0.134][serde_json] | 3.6769 ms | 6.8725 ms | † | 1623191 | 466527 | 359623 | 5.8867 ms |
| [simd-json 0.14.3][simd-json] | 2.2265 ms | 4.6012 ms | † | 1623191 | 466527 | 359623 | 5.9044 ms |
| [speedy 0.8.7][speedy] | 257.76 µs | 1.6137 ms | 562.48 µs | 449595 | 234970 | 210361 | 2.4804 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.923 ns\**</span> | <span title="validated on-demand with error">*409.87 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4858 ns\**</span> <span title="validated upfront with error">*2.2381 ms\**</span> | <span title="unvalidated">*1.3668 µs\**</span> <span title="validated upfront with error">*2.2620 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*537.43 µs\**</span> | <span title="unvalidated">*240.21 ns\**</span> <span title="validated upfront with error">*523.31 µs\**</span> | <span title="unvalidated">*753.89 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*13.54%\**</span> <span title="prepend">*15.32%\**</span> | 39.86% | 9.91% | 66.96% | 71.47% | 73.23% | 24.48% |
| [bincode 2.0.0-rc][bincode] | 41.10% | 59.97% | † | 89.19% | 90.81% | 88.59% | 30.47% |
| [bincode 1.3.3][bincode1] | 21.43% | 69.41% | 20.36% | 57.49% | 83.55% | 78.62% | 26.39% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 22.89% | 70.00% | † | 73.37% | 85.79% | 87.01% | 30.66% |
| [capnp 0.20.3][capnp] | 27.45% | † | † | 40.76% | 59.88% | 65.07% | 19.37% |
| [cbor4ii 0.3.3][cbor4ii] | 15.64% | 26.23% | 4.81% | 29.53% | 58.29% | 66.57% | 19.86% |
| [ciborium 0.2.2][ciborium] | 3.45% | 12.63% | † | 29.53% | 58.29% | 66.56% | 19.94% |
| [databuf 0.5.0][databuf] | 39.60% | 71.96% | 21.89% | 91.97% | 94.31% | 92.06% | 31.64% |
| [dlhn 0.1.7][dlhn] | 16.26% | 48.05% | † | 89.41% | 91.09% | 88.84% | 30.36% |
| [flatbuffers 24.12.23][flatbuffers] | 4.03% | † | † | 38.82% | 58.13% | 62.15% | 19.67% |
| [minicbor 0.25.1][minicbor] | 24.39% | 37.31% | 9.05% | 76.42% | 80.42% | 79.89% | 27.68% |
| [msgpacker 0.4.5][msgpacker] | 13.37% | 43.56% | † | 83.75% | 84.83% | 82.88% | 28.80% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.46% | 30.63% | 5.73% | 72.86% | 79.60% | 79.07% | 27.46% |
| [nanoserde 0.1.37][nanoserde] | 47.66% | 65.26% | † | 57.69% | 83.75% | 78.62% | 26.30% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.58% | 63.85% | † | 91.97% | 94.35% | 92.05% | 31.48% |
| [postcard 1.1.1][postcard] | 28.92% | 62.13% | 20.58% | 89.17% | 90.55% | 88.13% | 30.26% |
| [pot 3.0.1][pot] | 5.37% | 20.60% | 3.34% | 54.69% | 67.17% | 73.78% | 23.83% |
| [prost 0.13.4][prost] | <span title="encode">*10.02%\**</span> <span title="populate + encode">*4.34%\**</span> | 37.63% | † | 54.91% | 65.82% | 67.85% | 21.76% |
| [rkyv 0.8.9][rkyv] | 38.38% | <span title="unvalidated">*83.75%\**</span> <span title="validated upfront with error">*61.42%\**</span> | † | 54.27% | 78.87% | 83.03% | 27.83% |
| [rmp-serde 1.3.0][rmp-serde] | 8.04% | 41.06% | 9.85% | 77.19% | 81.95% | 80.79% | 28.09% |
| [ron 0.8.1][ron] | 1.76% | 7.26% | 1.10% | 22.36% | 46.20% | 53.22% | 13.12% |
| [savefile 0.18.5][savefile] | 58.02% | 69.00% | † | 57.79% | 83.95% | 78.76% | 26.32% |
| [serde-brief 0.1.0][serde-brief] | 9.52% | 23.14% | 4.52% | 25.68% | 53.74% | 62.22% | 18.55% |
| [serde_bare 0.5.0][serde_bare] | 16.98% | 53.49% | † | 91.97% | 94.31% | 92.06% | 31.69% |
| [serde_cbor 0.11.2][serde_cbor] | 6.89% | 26.31% | 4.76% | 29.53% | 58.29% | 66.56% | 19.96% |
| [serde_json 1.0.134][serde_json] | 3.49% | 18.15% | † | 20.19% | 43.07% | 50.81% | 12.78% |
| [simd-json 0.14.3][simd-json] | 5.77% | 27.11% | † | 20.19% | 43.07% | 50.81% | 12.75% |
| [speedy 0.8.7][speedy] | 49.84% | 77.29% | 30.36% | 72.89% | 85.52% | 86.87% | 30.34% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*58.61%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.57%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*4.5483 ms\**</span> <span title="prepend">*2.5521 ms\**</span> | 8.3385 ms | 1704643 | 1294259 | 1245607 | 11.443 ms |
| [bincode 2.0.0-rc][bincode] | 1.4147 ms | 4.1292 ms | 1406257 | 1117802 | 1062238 | 9.3331 ms |
| [bincode 1.3.3][bincode1] | 4.0686 ms | 4.1091 ms | 1854234 | 1141994 | 1050351 | 10.174 ms |
| [bitcode 0.6.3][bitcode] | 707.69 µs | 2.3115 ms | 971318 | 878034 | 855922 | 3.3442 ms |
| [borsh 1.5.3][borsh] | 2.9882 ms | 2.8349 ms | 1521989 | 1108471 | 1038408 | 9.8066 ms |
| [capnp 0.20.3][capnp] | 2.2543 ms | † | 2724288 | 1546992 | 1240354 | 14.354 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.2982 ms | 18.087 ms | 6012539 | 1695215 | 1467194 | 21.421 ms |
| [ciborium 0.2.2][ciborium] | 22.648 ms | 53.864 ms | 6012373 | 1695146 | 1467435 | 21.286 ms |
| [databuf 0.5.0][databuf] | 1.2878 ms | 3.8181 ms | 1319999 | 1062631 | 1007898 | 8.7301 ms |
| [dlhn 0.1.7][dlhn] | 4.9308 ms | 6.6380 ms | 1311281 | 1077520 | 1045571 | 8.5955 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.2153 ms | † | 2325620 | 1440289 | 1265148 | 13.163 ms |
| [minicbor 0.25.1][minicbor] | 2.0780 ms | 11.576 ms | 1777386 | 1276218 | 1252036 | 12.268 ms |
| [msgpacker 0.4.5][msgpacker] | 2.3267 ms | 6.8283 ms | 1458773 | 1156055 | 1137194 | 9.5720 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.531 ms | 18.411 ms | 1770060 | 1277755 | 1263142 | 12.177 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2795 ms | 2.9235 ms | 1812404 | 1134820 | 1054758 | 10.242 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.1126 ms | 3.2693 ms | 1319999 | 1064380 | 1010284 | 8.7652 ms |
| [postcard 1.1.1][postcard] | 1.9869 ms | 4.3324 ms | 1311281 | 1083900 | 1041114 | 8.6427 ms |
| [pot 3.0.1][pot] | 14.097 ms | 30.083 ms | 2604812 | 1482233 | 1299952 | 15.741 ms |
| [prost 0.13.4][prost] | <span title="encode">*5.4805 ms\**</span> <span title="populate + encode">*9.3849 ms\**</span> | 8.6739 ms | 1859886 | 1338076 | 1295497 | 11.871 ms |
| [rkyv 0.8.9][rkyv] | 1.0170 ms | <span title="unvalidated">*2.1920 ms\**</span> <span title="validated upfront with error">*2.6199 ms\**</span> | 2075936 | 1383779 | 1211892 | 12.671 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.340 ms | 11.181 ms | 1745322 | 1261627 | 1228902 | 11.224 ms |
| [ron 0.8.1][ron] | 38.585 ms | 90.299 ms | 8677703 | 2233642 | 1827843 | 33.831 ms |
| [savefile 0.18.5][savefile] | 844.43 µs | 2.7570 ms | 1791505 | 1128012 | 1052757 | 10.164 ms |
| [serde-brief 0.1.0][serde-brief] | 6.8464 ms | 22.294 ms | 6951772 | 1796265 | 1570903 | 23.288 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8490 ms | 4.7888 ms | 1319999 | 1062645 | 1007918 | 8.7334 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.650 ms | 21.251 ms | 6012373 | 1695146 | 1467435 | 22.072 ms |
| [serde_json 1.0.134][serde_json] | 20.691 ms | 30.590 ms | 9390461 | 2391679 | 1843922 | 33.895 ms |
| [simd-json 0.14.3][simd-json] | 11.557 ms | 25.749 ms | 9390461 | 2391679 | 1843922 | 35.992 ms |
| [speedy 0.8.7][speedy] | 777.33 µs | 2.4575 ms | 1584734 | 1119837 | 1038012 | 9.8976 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.195 ns\**</span> | <span title="validated on-demand with error">*712.78 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4865 ns\**</span> <span title="validated upfront with error">*5.2213 ms\**</span> | <span title="unvalidated">*2.6165 µs\**</span> <span title="validated upfront with error">*5.2294 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2428 ns\**</span> <span title="validated upfront with error">*424.85 µs\**</span> | <span title="unvalidated">*437.15 ns\**</span> <span title="validated upfront with error">*423.05 µs\**</span> | <span title="unvalidated">*233.74 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*15.56%\**</span> <span title="prepend">*27.73%\**</span> | 26.29% | 56.98% | 67.84% | 68.72% | 29.22% |
| [bincode 2.0.0-rc][bincode] | 50.02% | 53.09% | 69.07% | 78.55% | 80.58% | 35.83% |
| [bincode 1.3.3][bincode1] | 17.39% | 53.35% | 52.38% | 76.89% | 81.49% | 32.87% |
| [bitcode 0.6.3][bitcode] | 100.00% | 94.83% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 23.68% | 77.32% | 63.82% | 79.21% | 82.43% | 34.10% |
| [capnp 0.20.3][capnp] | 31.39% | † | 35.65% | 56.76% | 69.01% | 23.30% |
| [cbor4ii 0.3.3][cbor4ii] | 21.46% | 12.12% | 16.15% | 51.79% | 58.34% | 15.61% |
| [ciborium 0.2.2][ciborium] | 3.12% | 4.07% | 16.16% | 51.80% | 58.33% | 15.71% |
| [databuf 0.5.0][databuf] | 54.95% | 57.41% | 73.58% | 82.63% | 84.92% | 38.31% |
| [dlhn 0.1.7][dlhn] | 14.35% | 33.02% | 74.07% | 81.49% | 81.86% | 38.91% |
| [flatbuffers 24.12.23][flatbuffers] | 13.57% | † | 41.77% | 60.96% | 67.65% | 25.41% |
| [minicbor 0.25.1][minicbor] | 34.06% | 18.94% | 54.65% | 68.80% | 68.36% | 27.26% |
| [msgpacker 0.4.5][msgpacker] | 30.42% | 32.10% | 66.58% | 75.95% | 75.27% | 34.94% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.32% | 11.91% | 54.87% | 68.72% | 67.76% | 27.46% |
| [nanoserde 0.1.37][nanoserde] | 55.31% | 74.98% | 53.59% | 77.37% | 81.15% | 32.65% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.74% | 67.05% | 73.58% | 82.49% | 84.72% | 38.15% |
| [postcard 1.1.1][postcard] | 35.62% | 50.60% | 74.07% | 81.01% | 82.21% | 38.69% |
| [pot 3.0.1][pot] | 5.02% | 7.29% | 37.29% | 59.24% | 65.84% | 21.25% |
| [prost 0.13.4][prost] | <span title="encode">*12.91%\**</span> <span title="populate + encode">*7.54%\**</span> | 25.27% | 52.22% | 65.62% | 66.07% | 28.17% |
| [rkyv 0.8.9][rkyv] | 69.59% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.67%\**</span> | 46.79% | 63.45% | 70.63% | 26.39% |
| [rmp-serde 1.3.0][rmp-serde] | 6.84% | 19.60% | 55.65% | 69.60% | 69.65% | 29.79% |
| [ron 0.8.1][ron] | 1.83% | 2.43% | 11.19% | 39.31% | 46.83% | 9.89% |
| [savefile 0.18.5][savefile] | 83.81% | 79.51% | 54.22% | 77.84% | 81.30% | 32.90% |
| [serde-brief 0.1.0][serde-brief] | 10.34% | 9.83% | 13.97% | 48.88% | 54.49% | 14.36% |
| [serde_bare 0.5.0][serde_bare] | 14.59% | 45.77% | 73.58% | 82.63% | 84.92% | 38.29% |
| [serde_cbor 0.11.2][serde_cbor] | 6.64% | 10.31% | 16.16% | 51.80% | 58.33% | 15.15% |
| [serde_json 1.0.134][serde_json] | 3.42% | 7.17% | 10.34% | 36.71% | 46.42% | 9.87% |
| [simd-json 0.14.3][simd-json] | 6.12% | 8.51% | 10.34% | 36.71% | 46.42% | 9.29% |
| [speedy 0.8.7][speedy] | 91.04% | 89.20% | 61.29% | 78.41% | 82.46% | 33.79% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*61.33%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.71%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1012.2
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.3
[borsh]: https://crates.io/crates/borsh/1.5.3
[capnp]: https://crates.io/crates/capnp/0.20.3
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.3
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/24.12.23
[minicbor]: https://crates.io/crates/minicbor/0.25.1
[msgpacker]: https://crates.io/crates/msgpacker/0.4.5
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.12
[postcard]: https://crates.io/crates/postcard/1.1.1
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.13.4
[rkyv]: https://crates.io/crates/rkyv/0.8.9
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.18.5
[serde-brief]: https://crates.io/crates/serde-brief/0.1.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.134
[simd-json]: https://crates.io/crates/simd-json/0.14.3
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
