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
* **Size**: the size of the buffer when serialized
* **Zlib**: the size of the buffer after zlib compression
* **Zstd**: the size of the buffer after zstd compression
* **Zstd Time**: the time taken to compress the serialized buffer with zstd

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2024-9-16 1:32:57

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.83.0-nightly (9b72238eb 2024-09-14)
binary: rustc
commit-hash: 9b72238eb813e9d06e9e9d270168512fbffd7ee7
commit-date: 2024-09-14
host: x86_64-unknown-linux-gnu
release: 1.83.0-nightly
LLVM version: 19.1.0
```

### CPU info

```
Architecture:                       x86_64
CPU op-mode(s):                     32-bit, 64-bit
Address sizes:                      48 bits physical, 48 bits virtual
Byte Order:                         Little Endian
CPU(s):                             4
On-line CPU(s) list:                0-3
Vendor ID:                          AuthenticAMD
Model name:                         AMD EPYC 7763 64-Core Processor
CPU family:                         25
Model:                              1
Thread(s) per core:                 2
Core(s) per socket:                 2
Socket(s):                          1
Stepping:                           1
BogoMIPS:                           4890.86
Flags:                              fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
Virtualization:                     AMD-V
Hypervisor vendor:                  Microsoft
Virtualization type:                full
L1d cache:                          64 KiB (2 instances)
L1i cache:                          64 KiB (2 instances)
L2 cache:                           1 MiB (2 instances)
L3 cache:                           32 MiB (1 instance)
NUMA node(s):                       1
NUMA node0 CPU(s):                  0-3
Vulnerability Gather data sampling: Not affected
Vulnerability Itlb multihit:        Not affected
Vulnerability L1tf:                 Not affected
Vulnerability Mds:                  Not affected
Vulnerability Meltdown:             Not affected
Vulnerability Mmio stale data:      Not affected
Vulnerability Retbleed:             Not affected
Vulnerability Spec rstack overflow: Vulnerable: Safe RET, no microcode
Vulnerability Spec store bypass:    Vulnerable
Vulnerability Spectre v1:           Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:           Mitigation; Retpolines; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
Vulnerability Srbds:                Not affected
Vulnerability Tsx async abort:      Not affected
```

</details>

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 205.79 µs | <span title="unvalidated">*1.4156 ms\**</span> | 1705800 | 520080 | 413611 | 6.7984 ms |
| [alkahest 0.1.5][alkahest] | 202.50 µs | † | 1045784 | 454157 | 389424 | 6.0286 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*695.60 µs\**</span> <span title="prepend">*629.53 µs\**</span> | 3.2261 ms | 874632 | 355446 | 311723 | 5.1451 ms |
| [bincode 2.0.0-rc][bincode] | 287.10 µs | 2.5108 ms | 741295 | 303944 | 257153 | 3.7656 ms |
| [bincode 1.3.3][bincode1] | 518.34 µs | 2.0205 ms | 1045784 | 373127 | 311761 | 4.8399 ms |
| [bitcode 0.6.0][bitcode] | 137.70 µs | 1.5076 ms | 703710 | 288826 | 229755 | 2.4292 ms |
| [borsh 1.5.1][borsh] | 549.49 µs | 2.2169 ms | 885780 | 362204 | 286514 | 4.5350 ms |
| [bson 2.9.0][bson] | 2.0130 ms | 7.7425 ms | 1924682 | 532821 | 376270 | 5.7868 ms |
| [capnp 0.19.6][capnp] | 494.32 µs | † | 1443216 | 513986 | 428649 | 6.6912 ms |
| [cbor4ii 0.3.2][cbor4ii] | 601.76 µs | 4.9636 ms | 1407835 | 403440 | 324081 | 4.8505 ms |
| [ciborium 0.2.2][ciborium] | 3.1429 ms | 11.600 ms | 1407835 | 403440 | 324081 | 4.9070 ms |
| [databuf 0.5.0][databuf] | 256.18 µs | 2.0674 ms | 765778 | 311715 | 264630 | 4.1894 ms |
| [dlhn 0.1.7][dlhn] | 741.32 µs | 2.6412 ms | 724953 | 301446 | 253629 | 3.8501 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0253 ms | † | 1276368 | 468539 | 388832 | 5.1907 ms |
| [msgpacker 0.4.3][msgpacker] | 1.0528 ms | 2.7142 ms | 764996 | 315291 | 264898 | 3.9662 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.6204 ms | 4.1194 ms | 818669 | 332556 | 285514 | 4.3614 ms |
| [nanoserde 0.1.37][nanoserde] | 243.93 µs | 2.0850 ms | 1045784 | 373127 | 311761 | 4.5564 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 657.48 µs | 2.2725 ms | 765778 | 311743 | 264518 | 4.1806 ms |
| [postcard 1.0.8][postcard] | 413.97 µs | 2.3033 ms | 724953 | 302399 | 253747 | 3.8563 ms |
| [pot 3.0.0][pot] | 2.2411 ms | 6.5860 ms | 971922 | 372513 | 304122 | 4.6555 ms |
| [prost 0.12.6][prost] | <span title="encode">*933.45 µs\**</span> <span title="populate + encode">*2.6332 ms\**</span> | 3.3592 ms | 884628 | 363130 | 315494 | 5.1339 ms |
| [rkyv 0.8.3][rkyv] | 286.28 µs | <span title="unvalidated">*1.5957 ms\**</span> <span title="validated upfront with error">*2.1627 ms\**</span> | 1011488 | 393526 | 326517 | 4.9384 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3425 ms | 3.2122 ms | 784997 | 325384 | 278219 | 4.1045 ms |
| [ron 0.8.1][ron] | 12.508 ms | 15.536 ms | 1607459 | 449158 | 349713 | 5.6935 ms |
| [savefile 0.17.6][savefile] | 188.98 µs | 2.1855 ms | 1045800 | 373140 | 311777 | 4.5674 ms |
| [serde_bare 0.5.0][serde_bare] | 679.61 µs | 2.0824 ms | 765778 | 311715 | 264630 | 3.8804 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8727 ms | 5.0345 ms | 1407835 | 403440 | 324081 | 4.8545 ms |
| [serde_json 1.0.120][serde_json] | 3.8159 ms | 5.8727 ms | 1827461 | 470560 | 361090 | 5.6363 ms |
| [simd-json 0.13.10][simd-json] | 2.0767 ms | 4.6491 ms | 1827461 | 470560 | 361090 | 5.7076 ms |
| [speedy 0.8.7][speedy] | 201.85 µs | 1.7910 ms | 885780 | 362204 | 286514 | 4.2540 ms |
| [wiring 0.2.2][wiring] | 193.14 µs | 1.9929 ms | 1045784 | 337930 | 276188 | 3.9464 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*21.993 µs\**</span> | <span title="unvalidated">*37.296 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8618 ns\**</span> | <span title="validated on-demand with panic">*24.925 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*73.519 ns\**</span> | <span title="validated on-demand with error">*167.58 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4746 ns\**</span> <span title="validated upfront with error">*2.0073 ms\**</span> | <span title="unvalidated">*51.630 µs\**</span> <span title="validated upfront with error">*2.0578 ms\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*1.2398 ns\**</span> <span title="validated upfront with error">*601.39 µs\**</span> | <span title="unvalidated">*10.516 µs\**</span> <span title="validated upfront with error">*610.01 µs\**</span> | <span title="unvalidated">*7.3716 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 66.91% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 55.53% | 55.55% | 35.73% |
| [alkahest 0.1.5][alkahest] | 68.00% | † | 67.29% | 63.60% | 59.00% | 40.29% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*19.80%\**</span> <span title="prepend">*21.87%\**</span> | 43.88% | 80.46% | 81.26% | 73.70% | 47.21% |
| [bincode 2.0.0-rc][bincode] | 47.96% | 56.38% | 94.93% | 95.03% | 89.35% | 64.51% |
| [bincode 1.3.3][bincode1] | 26.57% | 70.06% | 67.29% | 77.41% | 73.70% | 50.19% |
| [bitcode 0.6.0][bitcode] | 100.00% | 93.90% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 25.06% | 63.85% | 79.45% | 79.74% | 80.19% | 53.57% |
| [bson 2.9.0][bson] | 6.84% | 18.28% | 36.56% | 54.21% | 61.06% | 41.98% |
| [capnp 0.19.6][capnp] | 27.86% | † | 48.76% | 56.19% | 53.60% | 36.30% |
| [cbor4ii 0.3.2][cbor4ii] | 22.88% | 28.52% | 49.99% | 71.59% | 70.89% | 50.08% |
| [ciborium 0.2.2][ciborium] | 4.38% | 12.20% | 49.99% | 71.59% | 70.89% | 49.50% |
| [databuf 0.5.0][databuf] | 53.75% | 68.47% | 91.89% | 92.66% | 86.82% | 57.98% |
| [dlhn 0.1.7][dlhn] | 18.57% | 53.60% | 97.07% | 95.81% | 90.59% | 63.09% |
| [flatbuffers 24.3.25][flatbuffers] | 13.43% | † | 55.13% | 61.64% | 59.09% | 46.80% |
| [msgpacker 0.4.3][msgpacker] | 13.08% | 52.16% | 91.99% | 91.61% | 86.73% | 61.25% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.45% | 34.36% | 85.96% | 86.85% | 80.47% | 55.70% |
| [nanoserde 0.1.37][nanoserde] | 56.45% | 67.89% | 67.29% | 77.41% | 73.70% | 53.31% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.94% | 62.29% | 91.89% | 92.65% | 86.86% | 58.11% |
| [postcard 1.0.8][postcard] | 33.26% | 61.46% | 97.07% | 95.51% | 90.54% | 62.99% |
| [pot 3.0.0][pot] | 6.14% | 21.49% | 72.40% | 77.53% | 75.55% | 52.18% |
| [prost 0.12.6][prost] | <span title="encode">*14.75%\**</span> <span title="populate + encode">*5.23%\**</span> | 42.14% | 79.55% | 79.54% | 72.82% | 47.32% |
| [rkyv 0.8.3][rkyv] | 48.10% | <span title="unvalidated">*88.71%\**</span> <span title="validated upfront with error">*65.46%\**</span> | 69.57% | 73.39% | 70.37% | 49.19% |
| [rmp-serde 1.3.0][rmp-serde] | 10.26% | 44.07% | 89.64% | 88.76% | 82.58% | 59.18% |
| [ron 0.8.1][ron] | 1.10% | 9.11% | 43.78% | 64.30% | 65.70% | 42.67% |
| [savefile 0.17.6][savefile] | 72.86% | 64.77% | 67.29% | 77.40% | 73.69% | 53.19% |
| [serde_bare 0.5.0][serde_bare] | 20.26% | 67.98% | 91.89% | 92.66% | 86.82% | 62.60% |
| [serde_cbor 0.11.2][serde_cbor] | 7.35% | 28.12% | 49.99% | 71.59% | 70.89% | 50.04% |
| [serde_json 1.0.120][serde_json] | 3.61% | 24.10% | 38.51% | 61.38% | 63.63% | 43.10% |
| [simd-json 0.13.10][simd-json] | 6.63% | 30.45% | 38.51% | 61.38% | 63.63% | 42.56% |
| [speedy 0.8.7][speedy] | 68.22% | 79.04% | 79.45% | 79.74% | 80.19% | 57.10% |
| [wiring 0.2.2][wiring] | 71.30% | 71.03% | 67.29% | 85.47% | 83.19% | 61.55% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*28.20%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.59%\**</span> | <span title="validated on-demand with panic">*42.19%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*6.28%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.10%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.37%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.72%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 237.63 µs | <span title="unvalidated">*237.90 µs\**</span> | 6000024 | 5378514 | 5345891 | 7.4494 ms |
| [alkahest 0.1.5][alkahest] | 148.15 µs | † | 6000008 | 5378500 | 5345890 | 7.4827 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.7244 ms\**</span> <span title="prepend">*8.9160 ms\**</span> | 9.0205 ms | 8625005 | 6443961 | 6231572 | 70.169 ms |
| [bincode 2.0.0-rc][bincode] | 501.00 µs | 818.99 µs | 6000005 | 5378497 | 5345897 | 7.4418 ms |
| [bincode 1.3.3][bincode1] | 5.5884 ms | 5.6455 ms | 6000008 | 5378500 | 5345890 | 7.5070 ms |
| [bitcode 0.6.0][bitcode] | 1.4068 ms | 802.12 µs | 6000006 | 5182295 | 4923880 | 12.590 ms |
| [borsh 1.5.1][borsh] | 6.1988 ms | 4.2391 ms | 6000004 | 5378496 | 5345889 | 7.5149 ms |
| [bson 2.9.0][bson] | 34.195 ms | 88.989 ms | 23013911 | 9212089 | 7497811 | 106.86 ms |
| [capnp 0.19.6][capnp] | 6.5545 ms | † | 14000088 | 7130367 | 6051062 | 78.859 ms |
| [cbor4ii 0.3.2][cbor4ii] | 9.0981 ms | 47.254 ms | 13125016 | 7524114 | 6757967 | 90.682 ms |
| [ciborium 0.2.2][ciborium] | 66.506 ms | 125.09 ms | 13122324 | 7524660 | 6759658 | 90.546 ms |
| [databuf 0.5.0][databuf] | 2.3934 ms | 5.3569 ms | 6000003 | 5378495 | 5345900 | 7.7825 ms |
| [dlhn 0.1.7][dlhn] | 6.0979 ms | 6.8357 ms | 6000003 | 5378495 | 5345900 | 7.6415 ms |
| [flatbuffers 24.3.25][flatbuffers] | 856.12 µs | † | 6000024 | 5378434 | 5345910 | 7.8340 ms |
| [msgpacker 0.4.3][msgpacker] | 3.1352 ms | 5.1367 ms | 7500005 | 6058442 | 6014337 | 10.220 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 130.25 ms | 31.999 ms | 8125037 | 6493484 | 6386940 | 68.648 ms |
| [nanoserde 0.1.37][nanoserde] | 1.4483 ms | 1.0630 ms | 6000008 | 5378500 | 5345890 | 7.7076 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.0765 ms | 4.0228 ms | 6000004 | 5378496 | 5345889 | 7.5301 ms |
| [postcard 1.0.8][postcard] | 474.39 µs | 1.3316 ms | 6000003 | 5378495 | 5345900 | 7.7503 ms |
| [pot 3.0.0][pot] | 39.285 ms | 71.389 ms | 10122342 | 6814618 | 6852251 | 80.228 ms |
| [prost 0.12.6][prost] | <span title="encode">*7.7164 ms\**</span> <span title="populate + encode">*8.8529 ms\**</span> | 14.708 ms | 8750000 | 6665735 | 6421871 | 70.986 ms |
| [rkyv 0.8.3][rkyv] | 270.68 µs | <span title="unvalidated">*148.08 µs\**</span> <span title="validated upfront with error">*149.73 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.9661 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.546 ms | 17.424 ms | 8125006 | 6494876 | 6391037 | 68.460 ms |
| [ron 0.8.1][ron] | 172.23 ms | 241.39 ms | 22192885 | 8970395 | 8138755 | 150.27 ms |
| [savefile 0.17.6][savefile] | 918.14 µs | 237.98 µs | 6000024 | 5378513 | 5345893 | 7.5382 ms |
| [serde_bare 0.5.0][serde_bare] | 6.0249 ms | 3.8892 ms | 6000003 | 5378495 | 5345900 | 7.6666 ms |
| [serde_cbor 0.11.2][serde_cbor] | 32.964 ms | 50.402 ms | 13122324 | 7524660 | 6759658 | 90.279 ms |
| [serde_json 1.0.120][serde_json] | 87.178 ms | 86.891 ms | 26192883 | 9566084 | 8586741 | 153.56 ms |
| [simd-json 0.13.10][simd-json] | 52.847 ms | 71.568 ms | 26192883 | 9566084 | 8586741 | 153.64 ms |
| [speedy 0.8.7][speedy] | 237.65 µs | 569.87 µs | 6000004 | 5378496 | 5345889 | 7.4455 ms |
| [wiring 0.2.2][wiring] | 147.50 µs | 320.51 µs | 6000008 | 5378952 | 5345894 | 7.8309 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1638 ns\**</span> | <span title="unvalidated">*140.80 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8557 ns\**</span> | <span title="validated on-demand with panic">*77.360 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*104.01 ns\**</span> | <span title="validated on-demand with error">*2.1957 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4736 ns\**</span> <span title="validated upfront with error">*40.258 ns\**</span> | <span title="unvalidated">*77.387 µs\**</span> <span title="validated upfront with error">*77.459 µs\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*1.2368 ns\**</span> <span title="validated upfront with error">*4.9542 ns\**</span> | <span title="unvalidated">*48.412 µs\**</span> <span title="validated upfront with error">*77.463 µs\**</span> | <span title="unvalidated">*79.600 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 62.07% | <span title="unvalidated">*62.24%\**</span> | 100.00% | 96.35% | 92.11% | 99.90% |
| [alkahest 0.1.5][alkahest] | 99.56% | † | 100.00% | 96.35% | 92.11% | 99.45% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.19%\**</span> <span title="prepend">*1.65%\**</span> | 1.64% | 69.57% | 80.42% | 79.02% | 10.61% |
| [bincode 2.0.0-rc][bincode] | 29.44% | 18.08% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bincode 1.3.3][bincode1] | 2.64% | 2.62% | 100.00% | 96.35% | 92.11% | 99.13% |
| [bitcode 0.6.0][bitcode] | 10.48% | 18.46% | 100.00% | 100.00% | 100.00% | 59.11% |
| [borsh 1.5.1][borsh] | 2.38% | 3.49% | 100.00% | 96.35% | 92.11% | 99.03% |
| [bson 2.9.0][bson] | 0.43% | 0.17% | 26.07% | 56.26% | 65.67% | 6.96% |
| [capnp 0.19.6][capnp] | 2.25% | † | 42.86% | 72.68% | 81.37% | 9.44% |
| [cbor4ii 0.3.2][cbor4ii] | 1.62% | 0.31% | 45.71% | 68.88% | 72.86% | 8.21% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.12% | 45.72% | 68.87% | 72.84% | 8.22% |
| [databuf 0.5.0][databuf] | 6.16% | 2.76% | 100.00% | 96.35% | 92.11% | 95.62% |
| [dlhn 0.1.7][dlhn] | 2.42% | 2.17% | 100.00% | 96.35% | 92.11% | 97.39% |
| [flatbuffers 24.3.25][flatbuffers] | 17.23% | † | 100.00% | 96.35% | 92.11% | 94.99% |
| [msgpacker 0.4.3][msgpacker] | 4.70% | 2.88% | 80.00% | 85.54% | 81.87% | 72.82% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.11% | 0.46% | 73.85% | 79.81% | 77.09% | 10.84% |
| [nanoserde 0.1.37][nanoserde] | 10.18% | 13.93% | 100.00% | 96.35% | 92.11% | 96.55% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.91% | 3.68% | 100.00% | 96.35% | 92.11% | 98.83% |
| [postcard 1.0.8][postcard] | 31.09% | 11.12% | 100.00% | 96.35% | 92.11% | 96.02% |
| [pot 3.0.0][pot] | 0.38% | 0.21% | 59.27% | 76.05% | 71.86% | 9.28% |
| [prost 0.12.6][prost] | <span title="encode">*1.91%\**</span> <span title="populate + encode">*1.67%\**</span> | 1.01% | 68.57% | 77.75% | 76.67% | 10.48% |
| [rkyv 0.8.3][rkyv] | 54.49% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*98.90%\**</span> | 100.00% | 96.35% | 92.11% | 93.42% |
| [rmp-serde 1.3.0][rmp-serde] | 0.95% | 0.85% | 73.85% | 79.79% | 77.04% | 10.87% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 4.95% |
| [savefile 0.17.6][savefile] | 16.07% | 62.22% | 100.00% | 96.35% | 92.11% | 98.72% |
| [serde_bare 0.5.0][serde_bare] | 2.45% | 3.81% | 100.00% | 96.35% | 92.11% | 97.07% |
| [serde_cbor 0.11.2][serde_cbor] | 0.45% | 0.29% | 45.72% | 68.87% | 72.84% | 8.24% |
| [serde_json 1.0.120][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.85% |
| [simd-json 0.13.10][simd-json] | 0.28% | 0.21% | 22.91% | 54.17% | 57.34% | 4.84% |
| [speedy 0.8.7][speedy] | 62.07% | 25.98% | 100.00% | 96.35% | 92.11% | 99.95% |
| [wiring 0.2.2][wiring] | 100.00% | 46.20% | 100.00% | 96.34% | 92.11% | 95.03% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.16%\**</span> | <span title="unvalidated">*34.38%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.65%\**</span> | <span title="validated on-demand with panic">*62.58%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.19%\**</span> | <span title="validated on-demand with error">*2.20%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*3.07%\**</span> | <span title="unvalidated">*62.56%\**</span> <span title="validated upfront with error">*62.50%\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.96%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*62.50%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 190.63 µs | <span title="unvalidated">*1.3096 ms\**</span> | 1290592 | 397893 | 338798 | 4.9243 ms |
| [alkahest 0.1.5][alkahest] | 217.74 µs | † | 667570 | 325484 | 320452 | 3.9239 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*928.47 µs\**</span> <span title="prepend">*844.02 µs\**</span> | 3.2504 ms | 489348 | 281173 | 249546 | 3.1343 ms |
| [bincode 2.0.0-rc][bincode] | 304.66 µs | 2.1168 ms | 367413 | 221291 | 206273 | 2.5180 ms |
| [bincode 1.3.3][bincode1] | 567.07 µs | 1.8662 ms | 569975 | 240525 | 232423 | 2.9059 ms |
| [bitcode 0.6.0][bitcode] | 139.15 µs | 1.2792 ms | 327688 | 200947 | 182736 | 742.15 µs |
| [borsh 1.5.1][borsh] | 560.38 µs | 1.8532 ms | 446595 | 234236 | 210008 | 2.5101 ms |
| [bson 2.9.0][bson] | 2.7911 ms | 9.1285 ms | 1619653 | 502185 | 328399 | 4.8868 ms |
| [capnp 0.19.6][capnp] | 458.83 µs | † | 803896 | 335606 | 280851 | 4.0636 ms |
| [cbor4ii 0.3.2][cbor4ii] | 784.05 µs | 4.7841 ms | 1109831 | 344745 | 274514 | 3.8622 ms |
| [ciborium 0.2.2][ciborium] | 3.7105 ms | 10.494 ms | 1109821 | 344751 | 274526 | 3.8712 ms |
| [databuf 0.5.0][databuf] | 296.60 µs | 1.7402 ms | 356311 | 213062 | 198488 | 2.4122 ms |
| [dlhn 0.1.7][dlhn] | 789.75 µs | 2.6395 ms | 366496 | 220600 | 205683 | 2.5222 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.3379 ms | † | 844168 | 345696 | 294015 | 3.8559 ms |
| [msgpacker 0.4.3][msgpacker] | 749.40 µs | 2.8209 ms | 391251 | 236877 | 220476 | 2.6559 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4368 ms | 3.9656 ms | 449745 | 252432 | 231110 | 2.7990 ms |
| [nanoserde 0.1.37][nanoserde] | 269.36 µs | 1.9249 ms | 567975 | 239930 | 232419 | 2.9232 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 613.77 µs | 1.9997 ms | 356311 | 212976 | 198524 | 2.3901 ms |
| [postcard 1.0.8][postcard] | 439.45 µs | 2.0816 ms | 367489 | 221913 | 207344 | 2.4976 ms |
| [pot 3.0.0][pot] | 2.5070 ms | 6.1020 ms | 599125 | 299158 | 247693 | 3.1674 ms |
| [prost 0.12.6][prost] | <span title="encode">*1.2563 ms\**</span> <span title="populate + encode">*2.8968 ms\**</span> | 3.4224 ms | 596811 | 305319 | 269310 | 3.4521 ms |
| [rkyv 0.8.3][rkyv] | 427.73 µs | <span title="unvalidated">*1.5192 ms\**</span> <span title="validated upfront with error">*2.0584 ms\**</span> | 603776 | 254776 | 220087 | 3.7005 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4358 ms | 3.0344 ms | 424533 | 245214 | 226188 | 2.7428 ms |
| [ron 0.8.1][ron] | 7.4067 ms | 17.150 ms | 1465223 | 434935 | 343338 | 5.8241 ms |
| [savefile 0.17.6][savefile] | 210.03 µs | 1.8309 ms | 566991 | 239361 | 232013 | 2.9755 ms |
| [serde_bare 0.5.0][serde_bare] | 735.50 µs | 2.2985 ms | 356311 | 213062 | 198488 | 2.4150 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8162 ms | 4.8698 ms | 1109821 | 344751 | 274526 | 3.8675 ms |
| [serde_json 1.0.120][serde_json] | 3.6437 ms | 6.8662 ms | 1623191 | 466527 | 359623 | 6.2168 ms |
| [simd-json 0.13.10][simd-json] | 2.2590 ms | 4.6296 ms | 1623191 | 466527 | 359623 | 5.9938 ms |
| [speedy 0.8.7][speedy] | 283.85 µs | 1.6005 ms | 449595 | 234970 | 210361 | 2.5173 ms |
| [wiring 0.2.2][wiring] | 219.43 µs | 1.8415 ms | 566975 | 247810 | 225259 | 3.0185 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.579 µs\**</span> | <span title="unvalidated">*38.416 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8544 ns\**</span> | <span title="validated on-demand with panic">*7.0236 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*72.719 ns\**</span> | <span title="validated on-demand with error">*417.15 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4753 ns\**</span> <span title="validated upfront with error">*2.1952 ms\**</span> | <span title="unvalidated">*1.3517 µs\**</span> <span title="validated upfront with error">*2.2184 ms\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*1.2367 ns\**</span> <span title="validated upfront with error">*535.44 µs\**</span> | <span title="unvalidated">*163.49 ns\**</span> <span title="validated upfront with error">*535.49 µs\**</span> | <span title="unvalidated">*725.10 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 72.99% | <span title="unvalidated">*97.68%\**</span> | 25.39% | 50.50% | 53.94% | 15.07% |
| [alkahest 0.1.5][alkahest] | 63.91% | † | 49.09% | 61.74% | 57.02% | 18.91% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*14.99%\**</span> <span title="prepend">*16.49%\**</span> | 39.36% | 66.96% | 71.47% | 73.23% | 23.68% |
| [bincode 2.0.0-rc][bincode] | 45.67% | 60.43% | 89.19% | 90.81% | 88.59% | 29.47% |
| [bincode 1.3.3][bincode1] | 24.54% | 68.55% | 57.49% | 83.55% | 78.62% | 25.54% |
| [bitcode 0.6.0][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 24.83% | 69.03% | 73.37% | 85.79% | 87.01% | 29.57% |
| [bson 2.9.0][bson] | 4.99% | 14.01% | 20.23% | 40.01% | 55.64% | 15.19% |
| [capnp 0.19.6][capnp] | 30.33% | † | 40.76% | 59.88% | 65.07% | 18.26% |
| [cbor4ii 0.3.2][cbor4ii] | 17.75% | 26.74% | 29.53% | 58.29% | 66.57% | 19.22% |
| [ciborium 0.2.2][ciborium] | 3.75% | 12.19% | 29.53% | 58.29% | 66.56% | 19.17% |
| [databuf 0.5.0][databuf] | 46.92% | 73.51% | 91.97% | 94.31% | 92.06% | 30.77% |
| [dlhn 0.1.7][dlhn] | 17.62% | 48.46% | 89.41% | 91.09% | 88.84% | 29.42% |
| [flatbuffers 24.3.25][flatbuffers] | 4.17% | † | 38.82% | 58.13% | 62.15% | 19.25% |
| [msgpacker 0.4.3][msgpacker] | 18.57% | 45.35% | 83.75% | 84.83% | 82.88% | 27.94% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.56% | 32.26% | 72.86% | 79.60% | 79.07% | 26.51% |
| [nanoserde 0.1.37][nanoserde] | 51.66% | 66.46% | 57.69% | 83.75% | 78.62% | 25.39% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.67% | 63.97% | 91.97% | 94.35% | 92.05% | 31.05% |
| [postcard 1.0.8][postcard] | 31.66% | 61.45% | 89.17% | 90.55% | 88.13% | 29.71% |
| [pot 3.0.0][pot] | 5.55% | 20.96% | 54.69% | 67.17% | 73.78% | 23.43% |
| [prost 0.12.6][prost] | <span title="encode">*11.08%\**</span> <span title="populate + encode">*4.80%\**</span> | 37.38% | 54.91% | 65.82% | 67.85% | 21.50% |
| [rkyv 0.8.3][rkyv] | 32.53% | <span title="unvalidated">*84.20%\**</span> <span title="validated upfront with error">*62.15%\**</span> | 54.27% | 78.87% | 83.03% | 20.06% |
| [rmp-serde 1.3.0][rmp-serde] | 9.69% | 42.16% | 77.19% | 81.95% | 80.79% | 27.06% |
| [ron 0.8.1][ron] | 1.88% | 7.46% | 22.36% | 46.20% | 53.22% | 12.74% |
| [savefile 0.17.6][savefile] | 66.25% | 69.87% | 57.79% | 83.95% | 78.76% | 24.94% |
| [serde_bare 0.5.0][serde_bare] | 18.92% | 55.65% | 91.97% | 94.31% | 92.06% | 30.73% |
| [serde_cbor 0.11.2][serde_cbor] | 7.66% | 26.27% | 29.53% | 58.29% | 66.56% | 19.19% |
| [serde_json 1.0.120][serde_json] | 3.82% | 18.63% | 20.19% | 43.07% | 50.81% | 11.94% |
| [simd-json 0.13.10][simd-json] | 6.16% | 27.63% | 20.19% | 43.07% | 50.81% | 12.38% |
| [speedy 0.8.7][speedy] | 49.02% | 79.93% | 72.89% | 85.52% | 86.87% | 29.48% |
| [wiring 0.2.2][wiring] | 63.41% | 69.47% | 57.80% | 81.09% | 81.12% | 24.59% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.43%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.69%\**</span> | <span title="validated on-demand with panic">*2.33%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*39.19%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.10%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 484.63 µs | <span title="unvalidated">*2.3007 ms\**</span> | 2984682 | 1398717 | 1264267 | 14.064 ms |
| [alkahest 0.1.5][alkahest] | 616.87 µs | † | 1863391 | 1234113 | 1202345 | 11.278 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*4.6668 ms\**</span> <span title="prepend">*2.4807 ms\**</span> | 8.4617 ms | 1664428 | 1264167 | 1216472 | 11.023 ms |
| [bincode 2.0.0-rc][bincode] | 698.76 µs | 4.1986 ms | 1372381 | 1091486 | 1037296 | 9.1114 ms |
| [bincode 1.3.3][bincode1] | 3.7678 ms | 3.9871 ms | 1811011 | 1115281 | 1025627 | 10.244 ms |
| [bitcode 0.6.0][bitcode] | 704.74 µs | 2.2923 ms | 948499 | 857321 | 837658 | 3.0248 ms |
| [borsh 1.5.1][borsh] | 2.8693 ms | 2.9228 ms | 1486162 | 1082357 | 1013550 | 9.5634 ms |
| [bson 2.9.0][bson] | 19.966 ms | 49.363 ms | 10030880 | 2833079 | 1600859 | 27.471 ms |
| [capnp 0.19.6][capnp] | 2.1786 ms | † | 2664040 | 1511895 | 1212087 | 14.065 ms |
| [cbor4ii 0.3.2][cbor4ii] | 3.1995 ms | 17.841 ms | 5878791 | 1655835 | 1431390 | 20.831 ms |
| [ciborium 0.2.2][ciborium] | 22.277 ms | 54.823 ms | 5878653 | 1655791 | 1431560 | 20.901 ms |
| [databuf 0.5.0][databuf] | 1.2466 ms | 3.6746 ms | 1288257 | 1037579 | 984337 | 8.4020 ms |
| [dlhn 0.1.7][dlhn] | 4.7814 ms | 6.6287 ms | 1279599 | 1052061 | 1021161 | 8.2337 ms |
| [flatbuffers 24.3.25][flatbuffers] | 5.5103 ms | † | 2273740 | 1408408 | 1235566 | 13.120 ms |
| [msgpacker 0.4.3][msgpacker] | 1.5733 ms | 6.1468 ms | 1424043 | 1128758 | 1110156 | 9.2898 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.975 ms | 17.370 ms | 1728519 | 1247642 | 1233323 | 12.579 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3130 ms | 2.8837 ms | 1770477 | 1108304 | 1029947 | 9.9106 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.0580 ms | 3.2414 ms | 1288257 | 1039269 | 986510 | 8.3716 ms |
| [postcard 1.0.8][postcard] | 2.0904 ms | 4.0415 ms | 1279599 | 1058243 | 1016738 | 8.2395 ms |
| [pot 3.0.0][pot] | 13.390 ms | 30.341 ms | 2544810 | 1447453 | 1268390 | 15.492 ms |
| [prost 0.12.6][prost] | <span title="encode">*5.2978 ms\**</span> <span title="populate + encode">*9.1364 ms\**</span> | 8.3498 ms | 1818378 | 1307777 | 1266311 | 11.373 ms |
| [rkyv 0.8.3][rkyv] | 865.12 µs | <span title="unvalidated">*2.1445 ms\**</span> <span title="validated upfront with error">*2.5724 ms\**</span> | 2029080 | 1351984 | 1183990 | 12.517 ms |
| [rmp-serde 1.3.0][rmp-serde] | 9.9502 ms | 10.622 ms | 1703813 | 1231892 | 1200208 | 11.114 ms |
| [ron 0.8.1][ron] | 38.139 ms | 87.271 ms | 8476284 | 2181196 | 1783971 | 33.259 ms |
| [savefile 0.17.6][savefile] | 795.37 µs | 2.7096 ms | 1750226 | 1101682 | 1027828 | 9.8776 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8856 ms | 4.9893 ms | 1288257 | 1037597 | 984356 | 8.7195 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.031 ms | 21.918 ms | 5878653 | 1655791 | 1431560 | 20.752 ms |
| [serde_json 1.0.120][serde_json] | 21.108 ms | 30.919 ms | 9175594 | 2334253 | 1800713 | 33.404 ms |
| [simd-json 0.13.10][simd-json] | 11.637 ms | 26.698 ms | 9175594 | 2334253 | 1800713 | 34.685 ms |
| [speedy 0.8.7][speedy] | 713.68 µs | 2.4214 ms | 1546963 | 1093532 | 1013443 | 9.7299 ms |
| [wiring 0.2.2][wiring] | 694.05 µs | 2.7141 ms | 1750210 | 1129857 | 1058906 | 10.328 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.146 µs\**</span> | <span title="unvalidated">*66.973 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8553 ns\**</span> | <span title="validated on-demand with panic">*625.96 ns\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*72.326 ns\**</span> | <span title="validated on-demand with error">*711.10 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4732 ns\**</span> <span title="validated upfront with error">*4.9947 ms\**</span> | <span title="unvalidated">*2.6238 µs\**</span> <span title="validated upfront with error">*5.2393 ms\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*1.2367 ns\**</span> <span title="validated upfront with error">*417.92 µs\**</span> | <span title="unvalidated">*432.93 ns\**</span> <span title="validated upfront with error">*420.58 µs\**</span> | <span title="unvalidated">*233.80 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.21%\**</span> | 31.78% | 61.29% | 66.26% | 21.51% |
| [alkahest 0.1.5][alkahest] | 78.56% | † | 50.90% | 69.47% | 69.67% | 26.82% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*10.38%\**</span> <span title="prepend">*19.54%\**</span> | 25.34% | 56.99% | 67.82% | 68.86% | 27.44% |
| [bincode 2.0.0-rc][bincode] | 69.36% | 51.08% | 69.11% | 78.55% | 80.75% | 33.20% |
| [bincode 1.3.3][bincode1] | 12.86% | 53.79% | 52.37% | 76.87% | 81.67% | 29.53% |
| [bitcode 0.6.0][bitcode] | 68.77% | 93.55% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 16.89% | 73.37% | 63.82% | 79.21% | 82.65% | 31.63% |
| [bson 2.9.0][bson] | 2.43% | 4.34% | 9.46% | 30.26% | 52.33% | 11.01% |
| [capnp 0.19.6][capnp] | 22.25% | † | 35.60% | 56.71% | 69.11% | 21.51% |
| [cbor4ii 0.3.2][cbor4ii] | 15.15% | 12.02% | 16.13% | 51.78% | 58.52% | 14.52% |
| [ciborium 0.2.2][ciborium] | 2.18% | 3.91% | 16.13% | 51.78% | 58.51% | 14.47% |
| [databuf 0.5.0][databuf] | 38.88% | 58.36% | 73.63% | 82.63% | 85.10% | 36.00% |
| [dlhn 0.1.7][dlhn] | 10.14% | 32.35% | 74.12% | 81.49% | 82.03% | 36.74% |
| [flatbuffers 24.3.25][flatbuffers] | 8.79% | † | 41.72% | 60.87% | 67.80% | 23.05% |
| [msgpacker 0.4.3][msgpacker] | 30.80% | 34.89% | 66.61% | 75.95% | 75.45% | 32.56% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.62% | 12.35% | 54.87% | 68.72% | 67.92% | 24.05% |
| [nanoserde 0.1.37][nanoserde] | 36.91% | 74.37% | 53.57% | 77.35% | 81.33% | 30.52% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 15.85% | 66.16% | 73.63% | 82.49% | 84.91% | 36.13% |
| [postcard 1.0.8][postcard] | 23.18% | 53.06% | 74.12% | 81.01% | 82.39% | 36.71% |
| [pot 3.0.0][pot] | 3.62% | 7.07% | 37.27% | 59.23% | 66.04% | 19.52% |
| [prost 0.12.6][prost] | <span title="encode">*9.15%\**</span> <span title="populate + encode">*5.30%\**</span> | 25.68% | 52.16% | 65.56% | 66.15% | 26.60% |
| [rkyv 0.8.3][rkyv] | 56.02% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.37%\**</span> | 46.75% | 63.41% | 70.75% | 24.17% |
| [rmp-serde 1.3.0][rmp-serde] | 4.87% | 20.19% | 55.67% | 69.59% | 69.79% | 27.22% |
| [ron 0.8.1][ron] | 1.27% | 2.46% | 11.19% | 39.31% | 46.95% | 9.09% |
| [savefile 0.17.6][savefile] | 60.93% | 79.14% | 54.19% | 77.82% | 81.50% | 30.62% |
| [serde_bare 0.5.0][serde_bare] | 9.92% | 42.98% | 73.63% | 82.63% | 85.10% | 34.69% |
| [serde_cbor 0.11.2][serde_cbor] | 4.83% | 9.78% | 16.13% | 51.78% | 58.51% | 14.58% |
| [serde_json 1.0.120][serde_json] | 2.30% | 6.94% | 10.34% | 36.73% | 46.52% | 9.06% |
| [simd-json 0.13.10][simd-json] | 4.16% | 8.03% | 10.34% | 36.73% | 46.52% | 8.72% |
| [speedy 0.8.7][speedy] | 67.91% | 88.56% | 61.31% | 78.40% | 82.65% | 31.09% |
| [wiring 0.2.2][wiring] | 69.83% | 79.01% | 54.19% | 75.88% | 79.11% | 29.29% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.65%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*69.16%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*60.88%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.50%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bilrost]: https://crates.io/crates/bilrost/0.1010.0
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.0
[borsh]: https://crates.io/crates/borsh/1.5.1
[bson]: https://crates.io/crates/bson/2.9.0
[capnp]: https://crates.io/crates/capnp/0.19.6
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.2
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/24.3.25
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.12
[postcard]: https://crates.io/crates/postcard/1.0.8
[pot]: https://crates.io/crates/pot/3.0.0
[prost]: https://crates.io/crates/prost/0.12.6
[rkyv]: https://crates.io/crates/rkyv/0.8.3
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.17.6
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.120
[simd-json]: https://crates.io/crates/simd-json/0.13.10
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.2.2


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
