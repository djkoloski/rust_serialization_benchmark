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

## Last updated: 2024-9-16 0:10:26

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
| [abomonation 0.7.3][abomonation] | 218.62 µs | <span title="unvalidated">*1.4170 ms\**</span> | 1705800 | 520080 | 413422 | 6.9581 ms |
| [alkahest 0.1.5][alkahest] | 195.77 µs | † | 1045784 | 454157 | 389424 | 6.0857 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*707.79 µs\**</span> <span title="prepend">*624.20 µs\**</span> | 3.2387 ms | 874632 | 355446 | 311723 | 5.0462 ms |
| [bincode 2.0.0-rc][bincode] | 327.33 µs | 2.5209 ms | 741295 | 303944 | 257153 | 3.7629 ms |
| [bincode 1.3.3][bincode1] | 522.48 µs | 2.1567 ms | 1045784 | 373127 | 311761 | 4.5793 ms |
| [bitcode 0.6.0][bitcode] | 148.47 µs | 1.5025 ms | 703710 | 288826 | 229755 | 2.6461 ms |
| [borsh 1.5.1][borsh] | 546.45 µs | 2.2538 ms | 885780 | 362204 | 286514 | 4.5282 ms |
| [bson 2.9.0][bson] | 2.1141 ms | 7.8802 ms | 1924682 | 532821 | 376270 | 5.9253 ms |
| [capnp 0.19.6][capnp] | 817.34 µs | † | 1443216 | 513986 | 428649 | 6.7222 ms |
| [cbor4ii 0.3.2][cbor4ii] | 590.05 µs | 4.9639 ms | 1407835 | 403440 | 324081 | 4.9692 ms |
| [ciborium 0.2.2][ciborium] | 3.2381 ms | 11.752 ms | 1407835 | 403440 | 324081 | 5.0060 ms |
| [databuf 0.5.0][databuf] | 256.99 µs | 2.0473 ms | 765778 | 311715 | 264630 | 4.0997 ms |
| [dlhn 0.1.7][dlhn] | 729.59 µs | 2.6106 ms | 724953 | 301446 | 253629 | 3.7326 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0089 ms | † | 1276368 | 468539 | 388832 | 5.5276 ms |
| [msgpacker 0.4.3][msgpacker] | 1.2266 ms | 2.5983 ms | 764996 | 315291 | 264898 | 4.2048 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4101 ms | 4.4569 ms | 818669 | 332556 | 285514 | 4.5698 ms |
| [nanoserde 0.1.37][nanoserde] | 276.74 µs | 2.0929 ms | 1045784 | 373127 | 311761 | 4.5288 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 647.12 µs | 2.2727 ms | 765778 | 311743 | 264518 | 4.1534 ms |
| [postcard 1.0.8][postcard] | 427.25 µs | 2.2949 ms | 724953 | 302399 | 253747 | 3.8230 ms |
| [pot 3.0.0][pot] | 2.0848 ms | 6.4373 ms | 971922 | 372513 | 304122 | 4.6238 ms |
| [prost 0.12.6][prost] | <span title="encode">*936.07 µs\**</span> <span title="populate + encode">*2.4344 ms\**</span> | 3.3176 ms | 884628 | 363130 | 315494 | 4.7604 ms |
| [rkyv 0.8.3][rkyv] | 298.54 µs | <span title="unvalidated">*1.5918 ms\**</span> <span title="validated upfront with error">*2.1709 ms\**</span> | 1011488 | 393526 | 326517 | 4.9382 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3506 ms | 3.1776 ms | 784997 | 325384 | 278219 | 4.1102 ms |
| [ron 0.8.1][ron] | 11.278 ms | 14.933 ms | 1607459 | 449158 | 349713 | 5.7416 ms |
| [savefile 0.17.6][savefile] | 188.09 µs | 2.1909 ms | 1045800 | 373140 | 311777 | 4.5780 ms |
| [serde_bare 0.5.0][serde_bare] | 669.47 µs | 2.1616 ms | 765778 | 311715 | 264630 | 3.8273 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9284 ms | 5.0444 ms | 1407835 | 403440 | 324081 | 4.7678 ms |
| [serde_json 1.0.120][serde_json] | 3.8451 ms | 5.8060 ms | 1827461 | 470560 | 361090 | 5.6703 ms |
| [simd-json 0.13.10][simd-json] | 2.0400 ms | 4.6794 ms | 1827461 | 470560 | 361090 | 5.8459 ms |
| [speedy 0.8.7][speedy] | 201.33 µs | 1.8069 ms | 885780 | 362204 | 286514 | 4.2068 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.760 µs\**</span> | <span title="unvalidated">*41.117 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8555 ns\**</span> | <span title="validated on-demand with panic">*24.915 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*72.860 ns\**</span> | <span title="validated on-demand with error">*178.19 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4749 ns\**</span> <span title="validated upfront with error">*1.9589 ms\**</span> | <span title="unvalidated">*49.039 µs\**</span> <span title="validated upfront with error">*2.0556 ms\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*613.26 µs\**</span> | <span title="unvalidated">*10.411 µs\**</span> <span title="validated upfront with error">*611.20 µs\**</span> | <span title="unvalidated">*7.3676 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 67.91% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 55.53% | 55.57% | 38.03% |
| [alkahest 0.1.5][alkahest] | 75.84% | † | 67.29% | 63.60% | 59.00% | 43.48% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*20.98%\**</span> <span title="prepend">*23.79%\**</span> | 43.75% | 80.46% | 81.26% | 73.70% | 52.44% |
| [bincode 2.0.0-rc][bincode] | 45.36% | 56.21% | 94.93% | 95.03% | 89.35% | 70.32% |
| [bincode 1.3.3][bincode1] | 28.42% | 65.70% | 67.29% | 77.41% | 73.70% | 57.78% |
| [bitcode 0.6.0][bitcode] | 100.00% | 94.31% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 27.17% | 62.87% | 79.45% | 79.74% | 80.19% | 58.44% |
| [bson 2.9.0][bson] | 7.02% | 17.98% | 36.56% | 54.21% | 61.06% | 44.66% |
| [capnp 0.19.6][capnp] | 18.17% | † | 48.76% | 56.19% | 53.60% | 39.36% |
| [cbor4ii 0.3.2][cbor4ii] | 25.16% | 28.55% | 49.99% | 71.59% | 70.89% | 53.25% |
| [ciborium 0.2.2][ciborium] | 4.59% | 12.06% | 49.99% | 71.59% | 70.89% | 52.86% |
| [databuf 0.5.0][databuf] | 57.77% | 69.21% | 91.89% | 92.66% | 86.82% | 64.54% |
| [dlhn 0.1.7][dlhn] | 20.35% | 54.28% | 97.07% | 95.81% | 90.59% | 70.89% |
| [flatbuffers 24.3.25][flatbuffers] | 14.72% | † | 55.13% | 61.64% | 59.09% | 47.87% |
| [msgpacker 0.4.3][msgpacker] | 12.10% | 54.54% | 91.99% | 91.61% | 86.73% | 62.93% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.74% | 31.79% | 85.96% | 86.85% | 80.47% | 57.90% |
| [nanoserde 0.1.37][nanoserde] | 53.65% | 67.71% | 67.29% | 77.41% | 73.70% | 58.43% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.94% | 62.35% | 91.89% | 92.65% | 86.86% | 63.71% |
| [postcard 1.0.8][postcard] | 34.75% | 61.75% | 97.07% | 95.51% | 90.54% | 69.22% |
| [pot 3.0.0][pot] | 7.12% | 22.01% | 72.40% | 77.53% | 75.55% | 57.23% |
| [prost 0.12.6][prost] | <span title="encode">*15.86%\**</span> <span title="populate + encode">*6.10%\**</span> | 42.71% | 79.55% | 79.54% | 72.82% | 55.59% |
| [rkyv 0.8.3][rkyv] | 49.73% | <span title="unvalidated">*89.02%\**</span> <span title="validated upfront with error">*65.27%\**</span> | 69.57% | 73.39% | 70.37% | 53.58% |
| [rmp-serde 1.3.0][rmp-serde] | 10.99% | 44.59% | 89.64% | 88.76% | 82.58% | 64.38% |
| [ron 0.8.1][ron] | 1.32% | 9.49% | 43.78% | 64.30% | 65.70% | 46.09% |
| [savefile 0.17.6][savefile] | 78.94% | 64.68% | 67.29% | 77.40% | 73.69% | 57.80% |
| [serde_bare 0.5.0][serde_bare] | 22.18% | 65.55% | 91.89% | 92.66% | 86.82% | 69.14% |
| [serde_cbor 0.11.2][serde_cbor] | 7.70% | 28.09% | 49.99% | 71.59% | 70.89% | 55.50% |
| [serde_json 1.0.120][serde_json] | 3.86% | 24.41% | 38.51% | 61.38% | 63.63% | 46.67% |
| [simd-json 0.13.10][simd-json] | 7.28% | 30.28% | 38.51% | 61.38% | 63.63% | 45.26% |
| [speedy 0.8.7][speedy] | 73.74% | 78.42% | 79.45% | 79.74% | 80.19% | 62.90% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*25.32%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*41.79%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*5.84%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.23%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.70%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 906.41 µs | <span title="unvalidated">*287.93 µs\**</span> | 6000024 | 5378513 | 5345890 | 7.6492 ms |
| [alkahest 0.1.5][alkahest] | 149.72 µs | † | 6000008 | 5378500 | 5345890 | 7.7890 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6532 ms\**</span> <span title="prepend">*8.6732 ms\**</span> | 9.6561 ms | 8625005 | 6443961 | 6231572 | 71.753 ms |
| [bincode 2.0.0-rc][bincode] | 2.3967 ms | 1.1556 ms | 6000005 | 5378497 | 5345897 | 8.2882 ms |
| [bincode 1.3.3][bincode1] | 5.6724 ms | 6.7799 ms | 6000008 | 5378500 | 5345890 | 8.3530 ms |
| [bitcode 0.6.0][bitcode] | 1.4091 ms | 802.05 µs | 6000006 | 5182295 | 4923880 | 12.669 ms |
| [borsh 1.5.1][borsh] | 6.1419 ms | 4.4464 ms | 6000004 | 5378496 | 5345889 | 7.4149 ms |
| [bson 2.9.0][bson] | 38.742 ms | 88.732 ms | 23013911 | 9212089 | 7497811 | 111.03 ms |
| [capnp 0.19.6][capnp] | 6.2926 ms | † | 14000088 | 7130367 | 6051062 | 79.996 ms |
| [cbor4ii 0.3.2][cbor4ii] | 9.3917 ms | 47.896 ms | 13125016 | 7524114 | 6757967 | 90.993 ms |
| [ciborium 0.2.2][ciborium] | 66.994 ms | 117.73 ms | 13122324 | 7524660 | 6759658 | 92.080 ms |
| [databuf 0.5.0][databuf] | 2.3997 ms | 5.3224 ms | 6000003 | 5378495 | 5345900 | 7.6161 ms |
| [dlhn 0.1.7][dlhn] | 6.0495 ms | 7.3197 ms | 6000003 | 5378495 | 5345900 | 7.5453 ms |
| [flatbuffers 24.3.25][flatbuffers] | 873.07 µs | † | 6000024 | 5378434 | 5345910 | 7.8812 ms |
| [msgpacker 0.4.3][msgpacker] | 20.220 ms | 5.7960 ms | 7500005 | 6058442 | 6014337 | 10.133 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 126.71 ms | 33.433 ms | 8125037 | 6493484 | 6386940 | 69.550 ms |
| [nanoserde 0.1.37][nanoserde] | 1.9231 ms | 1.0644 ms | 6000008 | 5378500 | 5345890 | 7.9187 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1222 ms | 2.8068 ms | 6000004 | 5378496 | 5345889 | 8.6024 ms |
| [postcard 1.0.8][postcard] | 489.02 µs | 1.4108 ms | 6000003 | 5378495 | 5345900 | 7.7577 ms |
| [pot 3.0.0][pot] | 38.754 ms | 73.054 ms | 10122342 | 6814618 | 6852251 | 81.622 ms |
| [prost 0.12.6][prost] | <span title="encode">*7.7579 ms\**</span> <span title="populate + encode">*9.0197 ms\**</span> | 13.743 ms | 8750000 | 6665735 | 6421871 | 71.934 ms |
| [rkyv 0.8.3][rkyv] | 309.12 µs | <span title="unvalidated">*148.13 µs\**</span> <span title="validated upfront with error">*207.52 µs\**</span> | 6000008 | 5378500 | 5345892 | 8.7963 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.337 ms | 18.303 ms | 8125006 | 6494876 | 6391037 | 67.086 ms |
| [ron 0.8.1][ron] | 168.10 ms | 242.18 ms | 22192885 | 8970395 | 8138755 | 152.45 ms |
| [savefile 0.17.6][savefile] | 287.66 µs | 288.58 µs | 6000024 | 5378513 | 5345893 | 8.0112 ms |
| [serde_bare 0.5.0][serde_bare] | 6.6044 ms | 5.4389 ms | 6000003 | 5378495 | 5345900 | 7.9180 ms |
| [serde_cbor 0.11.2][serde_cbor] | 34.587 ms | 48.058 ms | 13122324 | 7524660 | 6759658 | 91.184 ms |
| [serde_json 1.0.120][serde_json] | 89.367 ms | 87.079 ms | 26192883 | 9566084 | 8586741 | 157.34 ms |
| [simd-json 0.13.10][simd-json] | 53.596 ms | 74.053 ms | 26192883 | 9566084 | 8586741 | 157.88 ms |
| [speedy 0.8.7][speedy] | 1.0344 ms | 288.83 µs | 6000004 | 5378496 | 5345889 | 8.5805 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1649 ns\**</span> | <span title="unvalidated">*142.48 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8597 ns\**</span> | <span title="validated on-demand with panic">*77.485 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*105.04 ns\**</span> | <span title="validated on-demand with error">*2.1853 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4743 ns\**</span> <span title="validated upfront with error">*39.219 ns\**</span> | <span title="unvalidated">*54.016 µs\**</span> <span title="validated upfront with error">*77.382 µs\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*1.2372 ns\**</span> <span title="validated upfront with error">*5.5796 ns\**</span> | <span title="unvalidated">*48.375 µs\**</span> <span title="validated upfront with error">*38.683 µs\**</span> | <span title="unvalidated">*76.295 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 16.52% | <span title="unvalidated">*51.45%\**</span> | 100.00% | 96.35% | 92.11% | 96.94% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 95.20% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.25%\**</span> <span title="prepend">*1.73%\**</span> | 1.53% | 69.57% | 80.42% | 79.02% | 10.33% |
| [bincode 2.0.0-rc][bincode] | 6.25% | 12.82% | 100.00% | 96.35% | 92.11% | 89.46% |
| [bincode 1.3.3][bincode1] | 2.64% | 2.18% | 100.00% | 96.35% | 92.11% | 88.77% |
| [bitcode 0.6.0][bitcode] | 10.63% | 18.47% | 100.00% | 100.00% | 100.00% | 58.53% |
| [borsh 1.5.1][borsh] | 2.44% | 3.33% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bson 2.9.0][bson] | 0.39% | 0.17% | 26.07% | 56.26% | 65.67% | 6.68% |
| [capnp 0.19.6][capnp] | 2.38% | † | 42.86% | 72.68% | 81.37% | 9.27% |
| [cbor4ii 0.3.2][cbor4ii] | 1.59% | 0.31% | 45.71% | 68.88% | 72.86% | 8.15% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.84% | 8.05% |
| [databuf 0.5.0][databuf] | 6.24% | 2.78% | 100.00% | 96.35% | 92.11% | 97.36% |
| [dlhn 0.1.7][dlhn] | 2.47% | 2.02% | 100.00% | 96.35% | 92.11% | 98.27% |
| [flatbuffers 24.3.25][flatbuffers] | 17.15% | † | 100.00% | 96.35% | 92.11% | 94.08% |
| [msgpacker 0.4.3][msgpacker] | 0.74% | 2.56% | 80.00% | 85.54% | 81.87% | 73.18% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.44% | 73.85% | 79.81% | 77.09% | 10.66% |
| [nanoserde 0.1.37][nanoserde] | 7.79% | 13.92% | 100.00% | 96.35% | 92.11% | 93.64% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.92% | 5.28% | 100.00% | 96.35% | 92.11% | 86.20% |
| [postcard 1.0.8][postcard] | 30.62% | 10.50% | 100.00% | 96.35% | 92.11% | 95.58% |
| [pot 3.0.0][pot] | 0.39% | 0.20% | 59.27% | 76.05% | 71.86% | 9.08% |
| [prost 0.12.6][prost] | <span title="encode">*1.93%\**</span> <span title="populate + encode">*1.66%\**</span> | 1.08% | 68.57% | 77.75% | 76.67% | 10.31% |
| [rkyv 0.8.3][rkyv] | 48.43% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.38%\**</span> | 100.00% | 96.35% | 92.11% | 84.30% |
| [rmp-serde 1.3.0][rmp-serde] | 0.98% | 0.81% | 73.85% | 79.79% | 77.04% | 11.05% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 4.86% |
| [savefile 0.17.6][savefile] | 52.05% | 51.33% | 100.00% | 96.35% | 92.11% | 92.56% |
| [serde_bare 0.5.0][serde_bare] | 2.27% | 2.72% | 100.00% | 96.35% | 92.11% | 93.65% |
| [serde_cbor 0.11.2][serde_cbor] | 0.43% | 0.31% | 45.72% | 68.87% | 72.84% | 8.13% |
| [serde_json 1.0.120][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.71% |
| [simd-json 0.13.10][simd-json] | 0.28% | 0.20% | 22.91% | 54.17% | 57.34% | 4.70% |
| [speedy 0.8.7][speedy] | 14.47% | 51.29% | 100.00% | 96.35% | 92.11% | 86.42% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.15%\**</span> | <span title="unvalidated">*27.15%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.53%\**</span> | <span title="validated on-demand with panic">*49.92%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.77%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*3.15%\**</span> | <span title="unvalidated">*71.61%\**</span> <span title="validated upfront with error">*49.99%\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*22.17%\**</span> | <span title="unvalidated">*79.96%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 192.90 µs | <span title="unvalidated">*1.3055 ms\**</span> | 1290592 | 397877 | 339132 | 4.9189 ms |
| [alkahest 0.1.5][alkahest] | 216.22 µs | † | 667570 | 325484 | 320452 | 3.9188 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*948.17 µs\**</span> <span title="prepend">*842.22 µs\**</span> | 3.2520 ms | 489348 | 281173 | 249546 | 3.0930 ms |
| [bincode 2.0.0-rc][bincode] | 304.71 µs | 2.0903 ms | 367413 | 221291 | 206273 | 2.4589 ms |
| [bincode 1.3.3][bincode1] | 597.43 µs | 1.8596 ms | 569975 | 240525 | 232423 | 2.8537 ms |
| [bitcode 0.6.0][bitcode] | 131.92 µs | 1.2811 ms | 327688 | 200947 | 182736 | 790.98 µs |
| [borsh 1.5.1][borsh] | 554.59 µs | 1.8417 ms | 446595 | 234236 | 210008 | 2.4688 ms |
| [bson 2.9.0][bson] | 2.8652 ms | 8.9176 ms | 1619653 | 502185 | 328399 | 4.7906 ms |
| [capnp 0.19.6][capnp] | 643.16 µs | † | 803896 | 335606 | 280851 | 3.9031 ms |
| [cbor4ii 0.3.2][cbor4ii] | 776.62 µs | 4.8281 ms | 1109831 | 344745 | 274514 | 3.8063 ms |
| [ciborium 0.2.2][ciborium] | 3.7975 ms | 10.423 ms | 1109821 | 344751 | 274526 | 3.9143 ms |
| [databuf 0.5.0][databuf] | 293.70 µs | 1.7355 ms | 356311 | 213062 | 198488 | 2.4039 ms |
| [dlhn 0.1.7][dlhn] | 761.77 µs | 2.6315 ms | 366496 | 220600 | 205683 | 2.5518 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.2521 ms | † | 844168 | 345696 | 294015 | 3.8258 ms |
| [msgpacker 0.4.3][msgpacker] | 933.30 µs | 2.8223 ms | 391251 | 236877 | 220476 | 2.6092 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1458 ms | 3.9284 ms | 449745 | 252432 | 231110 | 2.7630 ms |
| [nanoserde 0.1.37][nanoserde] | 278.01 µs | 1.9337 ms | 567975 | 239930 | 232419 | 2.8919 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 612.11 µs | 2.0181 ms | 356311 | 212976 | 198524 | 2.4367 ms |
| [postcard 1.0.8][postcard] | 441.10 µs | 2.0719 ms | 367489 | 221913 | 207344 | 2.5012 ms |
| [pot 3.0.0][pot] | 2.3790 ms | 6.1687 ms | 599125 | 299158 | 247693 | 3.1681 ms |
| [prost 0.12.6][prost] | <span title="encode">*1.2628 ms\**</span> <span title="populate + encode">*2.9460 ms\**</span> | 3.4367 ms | 596811 | 305319 | 269310 | 3.4578 ms |
| [rkyv 0.8.3][rkyv] | 428.67 µs | <span title="unvalidated">*1.5110 ms\**</span> <span title="validated upfront with error">*2.0644 ms\**</span> | 603776 | 254776 | 220087 | 2.7446 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4523 ms | 3.0277 ms | 424533 | 245214 | 226188 | 2.6748 ms |
| [ron 0.8.1][ron] | 7.2515 ms | 17.233 ms | 1465223 | 434935 | 343338 | 5.8657 ms |
| [savefile 0.17.6][savefile] | 210.49 µs | 1.8499 ms | 566991 | 239361 | 232013 | 2.8320 ms |
| [serde_bare 0.5.0][serde_bare] | 737.57 µs | 2.3492 ms | 356311 | 213062 | 198488 | 2.3931 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8402 ms | 4.7960 ms | 1109821 | 344751 | 274526 | 3.7962 ms |
| [serde_json 1.0.120][serde_json] | 3.8795 ms | 6.8272 ms | 1623191 | 466527 | 359623 | 6.1197 ms |
| [simd-json 0.13.10][simd-json] | 2.2021 ms | 4.5552 ms | 1623191 | 466527 | 359623 | 6.0187 ms |
| [speedy 0.8.7][speedy] | 281.16 µs | 1.6177 ms | 449595 | 234970 | 210361 | 2.4694 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.345 µs\**</span> | <span title="unvalidated">*37.907 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8556 ns\**</span> | <span title="validated on-demand with panic">*7.1946 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*72.170 ns\**</span> | <span title="validated on-demand with error">*414.15 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4740 ns\**</span> <span title="validated upfront with error">*2.1966 ms\**</span> | <span title="unvalidated">*1.3584 µs\**</span> <span title="validated upfront with error">*2.1862 ms\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*1.2381 ns\**</span> <span title="validated upfront with error">*543.73 µs\**</span> | <span title="unvalidated">*239.08 ns\**</span> <span title="validated upfront with error">*541.55 µs\**</span> | <span title="unvalidated">*714.03 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 68.39% | <span title="unvalidated">*98.13%\**</span> | 25.39% | 50.50% | 53.88% | 16.08% |
| [alkahest 0.1.5][alkahest] | 61.01% | † | 49.09% | 61.74% | 57.02% | 20.18% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*13.91%\**</span> <span title="prepend">*15.66%\**</span> | 39.39% | 66.96% | 71.47% | 73.23% | 25.57% |
| [bincode 2.0.0-rc][bincode] | 43.29% | 61.29% | 89.19% | 90.81% | 88.59% | 32.17% |
| [bincode 1.3.3][bincode1] | 22.08% | 68.89% | 57.49% | 83.55% | 78.62% | 27.72% |
| [bitcode 0.6.0][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 23.79% | 69.56% | 73.37% | 85.79% | 87.01% | 32.04% |
| [bson 2.9.0][bson] | 4.60% | 14.37% | 20.23% | 40.01% | 55.64% | 16.51% |
| [capnp 0.19.6][capnp] | 20.51% | † | 40.76% | 59.88% | 65.07% | 20.27% |
| [cbor4ii 0.3.2][cbor4ii] | 16.99% | 26.53% | 29.53% | 58.29% | 66.57% | 20.78% |
| [ciborium 0.2.2][ciborium] | 3.47% | 12.29% | 29.53% | 58.29% | 66.56% | 20.21% |
| [databuf 0.5.0][databuf] | 44.92% | 73.82% | 91.97% | 94.31% | 92.06% | 32.90% |
| [dlhn 0.1.7][dlhn] | 17.32% | 48.68% | 89.41% | 91.09% | 88.84% | 31.00% |
| [flatbuffers 24.3.25][flatbuffers] | 4.06% | † | 38.82% | 58.13% | 62.15% | 20.67% |
| [msgpacker 0.4.3][msgpacker] | 14.13% | 45.39% | 83.75% | 84.83% | 82.88% | 30.32% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.56% | 32.61% | 72.86% | 79.60% | 79.07% | 28.63% |
| [nanoserde 0.1.37][nanoserde] | 47.45% | 66.25% | 57.69% | 83.75% | 78.62% | 27.35% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.55% | 63.48% | 91.97% | 94.35% | 92.05% | 32.46% |
| [postcard 1.0.8][postcard] | 29.91% | 61.83% | 89.17% | 90.55% | 88.13% | 31.62% |
| [pot 3.0.0][pot] | 5.55% | 20.77% | 54.69% | 67.17% | 73.78% | 24.97% |
| [prost 0.12.6][prost] | <span title="encode">*10.45%\**</span> <span title="populate + encode">*4.48%\**</span> | 37.28% | 54.91% | 65.82% | 67.85% | 22.88% |
| [rkyv 0.8.3][rkyv] | 30.77% | <span title="unvalidated">*84.78%\**</span> <span title="validated upfront with error">*62.06%\**</span> | 54.27% | 78.87% | 83.03% | 28.82% |
| [rmp-serde 1.3.0][rmp-serde] | 9.08% | 42.31% | 77.19% | 81.95% | 80.79% | 29.57% |
| [ron 0.8.1][ron] | 1.82% | 7.43% | 22.36% | 46.20% | 53.22% | 13.48% |
| [savefile 0.17.6][savefile] | 62.67% | 69.25% | 57.79% | 83.95% | 78.76% | 27.93% |
| [serde_bare 0.5.0][serde_bare] | 17.89% | 54.53% | 91.97% | 94.31% | 92.06% | 33.05% |
| [serde_cbor 0.11.2][serde_cbor] | 7.17% | 26.71% | 29.53% | 58.29% | 66.56% | 20.84% |
| [serde_json 1.0.120][serde_json] | 3.40% | 18.76% | 20.19% | 43.07% | 50.81% | 12.93% |
| [simd-json 0.13.10][simd-json] | 5.99% | 28.12% | 20.19% | 43.07% | 50.81% | 13.14% |
| [speedy 0.8.7][speedy] | 46.92% | 79.19% | 72.89% | 85.52% | 86.87% | 32.03% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.63%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.72%\**</span> | <span title="validated on-demand with panic">*3.32%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*57.73%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.04%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.60%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.04%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 493.36 µs | <span title="unvalidated">*2.3110 ms\**</span> | 2984682 | 1416983 | 1280132 | 14.503 ms |
| [alkahest 0.1.5][alkahest] | 626.25 µs | † | 1863391 | 1234113 | 1202345 | 11.585 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*4.6650 ms\**</span> <span title="prepend">*2.4798 ms\**</span> | 8.5772 ms | 1664428 | 1264167 | 1216472 | 11.195 ms |
| [bincode 2.0.0-rc][bincode] | 1.1978 ms | 4.0471 ms | 1372381 | 1091486 | 1037296 | 9.5120 ms |
| [bincode 1.3.3][bincode1] | 3.8572 ms | 4.0998 ms | 1811011 | 1115281 | 1025627 | 10.130 ms |
| [bitcode 0.6.0][bitcode] | 710.34 µs | 2.3299 ms | 948499 | 857321 | 837658 | 3.1267 ms |
| [borsh 1.5.1][borsh] | 2.8296 ms | 2.8253 ms | 1486162 | 1082357 | 1013550 | 9.5673 ms |
| [bson 2.9.0][bson] | 21.480 ms | 50.074 ms | 10030880 | 2833079 | 1600859 | 27.390 ms |
| [capnp 0.19.6][capnp] | 2.4524 ms | † | 2664040 | 1511895 | 1212087 | 14.157 ms |
| [cbor4ii 0.3.2][cbor4ii] | 3.2712 ms | 18.184 ms | 5878791 | 1655835 | 1431390 | 21.028 ms |
| [ciborium 0.2.2][ciborium] | 23.300 ms | 53.745 ms | 5878653 | 1655791 | 1431560 | 20.979 ms |
| [databuf 0.5.0][databuf] | 1.2710 ms | 3.7168 ms | 1288257 | 1037579 | 984337 | 9.1726 ms |
| [dlhn 0.1.7][dlhn] | 4.9750 ms | 6.7128 ms | 1279599 | 1052061 | 1021161 | 8.5006 ms |
| [flatbuffers 24.3.25][flatbuffers] | 5.3784 ms | † | 2273740 | 1408408 | 1235566 | 13.239 ms |
| [msgpacker 0.4.3][msgpacker] | 2.1918 ms | 6.2463 ms | 1424043 | 1128758 | 1110156 | 9.3555 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.224 ms | 18.122 ms | 1728519 | 1247642 | 1233323 | 11.877 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3344 ms | 2.8912 ms | 1770477 | 1108304 | 1029947 | 10.118 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.0068 ms | 3.1407 ms | 1288257 | 1039269 | 986510 | 8.5818 ms |
| [postcard 1.0.8][postcard] | 1.9090 ms | 4.0265 ms | 1279599 | 1058243 | 1016738 | 8.3806 ms |
| [pot 3.0.0][pot] | 13.254 ms | 30.257 ms | 2544810 | 1447453 | 1268390 | 15.588 ms |
| [prost 0.12.6][prost] | <span title="encode">*5.2941 ms\**</span> <span title="populate + encode">*9.4302 ms\**</span> | 8.6520 ms | 1818378 | 1307777 | 1266311 | 11.515 ms |
| [rkyv 0.8.3][rkyv] | 870.25 µs | <span title="unvalidated">*2.1547 ms\**</span> <span title="validated upfront with error">*2.5921 ms\**</span> | 2029080 | 1351984 | 1183990 | 12.598 ms |
| [rmp-serde 1.3.0][rmp-serde] | 9.8254 ms | 10.471 ms | 1703813 | 1231892 | 1200208 | 11.357 ms |
| [ron 0.8.1][ron] | 36.516 ms | 86.644 ms | 8476284 | 2181196 | 1783971 | 34.467 ms |
| [savefile 0.17.6][savefile] | 810.14 µs | 2.7112 ms | 1750226 | 1101682 | 1027828 | 9.8033 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8814 ms | 4.7736 ms | 1288257 | 1037597 | 984356 | 8.6162 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.5284 ms | 21.119 ms | 5878653 | 1655791 | 1431560 | 21.102 ms |
| [serde_json 1.0.120][serde_json] | 20.096 ms | 30.630 ms | 9175594 | 2334253 | 1800713 | 33.966 ms |
| [simd-json 0.13.10][simd-json] | 11.356 ms | 26.700 ms | 9175594 | 2334253 | 1800713 | 33.899 ms |
| [speedy 0.8.7][speedy] | 708.08 µs | 2.4133 ms | 1546963 | 1093532 | 1013443 | 9.6258 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*65.638 µs\**</span> | <span title="unvalidated">*66.539 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8568 ns\**</span> | <span title="validated on-demand with panic">*626.79 ns\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*72.380 ns\**</span> | <span title="validated on-demand with error">*710.89 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4750 ns\**</span> <span title="validated upfront with error">*4.7989 ms\**</span> | <span title="unvalidated">*2.6261 µs\**</span> <span title="validated upfront with error">*5.0748 ms\**</span> | ‡ |
| [rkyv 0.8.3][rkyv] | <span title="unvalidated">*1.2375 ns\**</span> <span title="validated upfront with error">*425.21 µs\**</span> | <span title="unvalidated">*431.06 ns\**</span> <span title="validated upfront with error">*426.14 µs\**</span> | <span title="unvalidated">*234.95 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.24%\**</span> | 31.78% | 60.50% | 65.44% | 21.56% |
| [alkahest 0.1.5][alkahest] | 78.78% | † | 50.90% | 69.47% | 69.67% | 26.99% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*10.58%\**</span> <span title="prepend">*19.90%\**</span> | 25.12% | 56.99% | 67.82% | 68.86% | 27.93% |
| [bincode 2.0.0-rc][bincode] | 41.19% | 53.24% | 69.11% | 78.55% | 80.75% | 32.87% |
| [bincode 1.3.3][bincode1] | 12.79% | 52.56% | 52.37% | 76.87% | 81.67% | 30.86% |
| [bitcode 0.6.0][bitcode] | 69.45% | 92.48% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 17.44% | 76.26% | 63.82% | 79.21% | 82.65% | 32.68% |
| [bson 2.9.0][bson] | 2.30% | 4.30% | 9.46% | 30.26% | 52.33% | 11.42% |
| [capnp 0.19.6][capnp] | 20.12% | † | 35.60% | 56.71% | 69.11% | 22.09% |
| [cbor4ii 0.3.2][cbor4ii] | 15.08% | 11.85% | 16.13% | 51.78% | 58.52% | 14.87% |
| [ciborium 0.2.2][ciborium] | 2.12% | 4.01% | 16.13% | 51.78% | 58.51% | 14.90% |
| [databuf 0.5.0][databuf] | 38.82% | 57.97% | 73.63% | 82.63% | 85.10% | 34.09% |
| [dlhn 0.1.7][dlhn] | 9.92% | 32.10% | 74.12% | 81.49% | 82.03% | 36.78% |
| [flatbuffers 24.3.25][flatbuffers] | 9.17% | † | 41.72% | 60.87% | 67.80% | 23.62% |
| [msgpacker 0.4.3][msgpacker] | 22.51% | 34.50% | 66.61% | 75.95% | 75.45% | 33.42% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.69% | 11.89% | 54.87% | 68.72% | 67.92% | 26.33% |
| [nanoserde 0.1.37][nanoserde] | 36.97% | 74.53% | 53.57% | 77.35% | 81.33% | 30.90% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 16.41% | 68.61% | 73.63% | 82.49% | 84.91% | 36.43% |
| [postcard 1.0.8][postcard] | 25.84% | 53.51% | 74.12% | 81.01% | 82.39% | 37.31% |
| [pot 3.0.0][pot] | 3.72% | 7.12% | 37.27% | 59.23% | 66.04% | 20.06% |
| [prost 0.12.6][prost] | <span title="encode">*9.32%\**</span> <span title="populate + encode">*5.23%\**</span> | 24.90% | 52.16% | 65.56% | 66.15% | 27.15% |
| [rkyv 0.8.3][rkyv] | 56.69% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.13%\**</span> | 46.75% | 63.41% | 70.75% | 24.82% |
| [rmp-serde 1.3.0][rmp-serde] | 5.02% | 20.58% | 55.67% | 69.59% | 69.79% | 27.53% |
| [ron 0.8.1][ron] | 1.35% | 2.49% | 11.19% | 39.31% | 46.95% | 9.07% |
| [savefile 0.17.6][savefile] | 60.90% | 79.47% | 54.19% | 77.82% | 81.50% | 31.89% |
| [serde_bare 0.5.0][serde_bare] | 10.11% | 45.14% | 73.63% | 82.63% | 85.10% | 36.29% |
| [serde_cbor 0.11.2][serde_cbor] | 5.18% | 10.20% | 16.13% | 51.78% | 58.51% | 14.82% |
| [serde_json 1.0.120][serde_json] | 2.46% | 7.03% | 10.34% | 36.73% | 46.52% | 9.21% |
| [simd-json 0.13.10][simd-json] | 4.34% | 8.07% | 10.34% | 36.73% | 46.52% | 9.22% |
| [speedy 0.8.7][speedy] | 69.68% | 89.28% | 61.31% | 78.40% | 82.65% | 32.48% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.65%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.65%\**</span> | <span title="validated on-demand with panic">*68.77%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*60.64%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.41%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
