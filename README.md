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

## Last updated: 2024-9-16 22:40:8

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.83.0-nightly (04a318787 2024-09-15)
binary: rustc
commit-hash: 04a318787b39732e306faf5ef6dc584990f4f417
commit-date: 2024-09-15
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
BogoMIPS:                           4890.85
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
| [abomonation 0.7.3][abomonation] | 201.81 µs | <span title="unvalidated">*1.4153 ms\**</span> | 1705800 | 520076 | 413395 | 6.7826 ms |
| [alkahest 0.1.5][alkahest] | 197.64 µs | † | 1045784 | 454157 | 389424 | 5.9897 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*718.80 µs\**</span> <span title="prepend">*622.29 µs\**</span> | 3.2078 ms | 874632 | 355446 | 311723 | 5.0475 ms |
| [bincode 2.0.0-rc][bincode] | 287.66 µs | 2.4988 ms | 741295 | 303944 | 257153 | 4.0066 ms |
| [bincode 1.3.3][bincode1] | 523.19 µs | 2.2482 ms | 1045784 | 373127 | 311761 | 4.8117 ms |
| [bitcode 0.6.0][bitcode] | 146.18 µs | 1.4952 ms | 703710 | 288826 | 229755 | 2.4939 ms |
| [borsh 1.5.1][borsh] | 541.87 µs | 2.2144 ms | 885780 | 362204 | 286514 | 4.5252 ms |
| [bson 2.9.0][bson] | 2.0077 ms | 7.8461 ms | 1924682 | 532821 | 376270 | 5.8014 ms |
| [capnp 0.19.6][capnp] | 497.39 µs | † | 1443216 | 513986 | 428649 | 6.7943 ms |
| [cbor4ii 0.3.2][cbor4ii] | 605.24 µs | 4.9353 ms | 1407835 | 403440 | 324081 | 4.9407 ms |
| [ciborium 0.2.2][ciborium] | 3.2087 ms | 12.110 ms | 1407835 | 403440 | 324081 | 4.8617 ms |
| [databuf 0.5.0][databuf] | 280.60 µs | 2.0646 ms | 765778 | 311715 | 264630 | 4.0920 ms |
| [dlhn 0.1.7][dlhn] | 739.00 µs | 2.6028 ms | 724953 | 301446 | 253629 | 3.7978 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0177 ms | † | 1276368 | 468539 | 388832 | 5.5691 ms |
| [msgpacker 0.4.3][msgpacker] | 1.0209 ms | 2.5923 ms | 764996 | 315291 | 264898 | 4.0416 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3820 ms | 4.1082 ms | 818669 | 332556 | 285514 | 4.4346 ms |
| [nanoserde 0.1.37][nanoserde] | 277.85 µs | 2.1045 ms | 1045784 | 373127 | 311761 | 4.5766 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 648.63 µs | 2.2467 ms | 765778 | 311743 | 264518 | 4.1592 ms |
| [postcard 1.0.8][postcard] | 420.93 µs | 2.2944 ms | 724953 | 302399 | 253747 | 3.8033 ms |
| [pot 3.0.0][pot] | 2.2966 ms | 6.4874 ms | 971922 | 372513 | 304122 | 4.7205 ms |
| [prost 0.12.6][prost] | <span title="encode">*929.52 µs\**</span> <span title="populate + encode">*2.4810 ms\**</span> | 3.2726 ms | 884628 | 363130 | 315494 | 5.0505 ms |
| [rkyv 0.8.5][rkyv] | 254.03 µs | <span title="unvalidated">*1.5951 ms\**</span> <span title="validated upfront with error">*2.1840 ms\**</span> | 1011488 | 393526 | 326517 | 4.9187 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4481 ms | 3.1752 ms | 784997 | 325384 | 278219 | 4.1034 ms |
| [ron 0.8.1][ron] | 11.510 ms | 15.322 ms | 1607459 | 449158 | 349713 | 5.6592 ms |
| [savefile 0.17.6][savefile] | 188.70 µs | 2.2168 ms | 1045800 | 373140 | 311777 | 4.5462 ms |
| [serde_bare 0.5.0][serde_bare] | 703.72 µs | 2.1070 ms | 765778 | 311715 | 264630 | 3.8714 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7341 ms | 4.8918 ms | 1407835 | 403440 | 324081 | 4.8425 ms |
| [serde_json 1.0.120][serde_json] | 3.8709 ms | 5.6871 ms | 1827461 | 470560 | 361090 | 5.5912 ms |
| [simd-json 0.13.10][simd-json] | 2.0373 ms | 4.6321 ms | 1827461 | 470560 | 361090 | 5.6225 ms |
| [speedy 0.8.7][speedy] | 194.61 µs | 1.7965 ms | 885780 | 362204 | 286514 | 4.2494 ms |
| [wiring 0.2.2][wiring] | 195.40 µs | 1.9898 ms | 1045784 | 337930 | 276188 | 3.9616 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.740 µs\**</span> | <span title="unvalidated">*40.134 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8547 ns\**</span> | <span title="validated on-demand with panic">*24.881 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*73.731 ns\**</span> | <span title="validated on-demand with error">*166.67 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4741 ns\**</span> <span title="validated upfront with error">*2.0575 ms\**</span> | <span title="unvalidated">*50.388 µs\**</span> <span title="validated upfront with error">*2.1339 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*591.63 µs\**</span> | <span title="unvalidated">*10.278 µs\**</span> <span title="validated upfront with error">*593.71 µs\**</span> | <span title="unvalidated">*7.5308 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 72.43% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 55.54% | 55.58% | 36.77% |
| [alkahest 0.1.5][alkahest] | 73.96% | † | 67.29% | 63.60% | 59.00% | 41.64% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*20.34%\**</span> <span title="prepend">*23.49%\**</span> | 44.12% | 80.46% | 81.26% | 73.70% | 49.41% |
| [bincode 2.0.0-rc][bincode] | 50.82% | 56.64% | 94.93% | 95.03% | 89.35% | 62.24% |
| [bincode 1.3.3][bincode1] | 27.94% | 62.95% | 67.29% | 77.41% | 73.70% | 51.83% |
| [bitcode 0.6.0][bitcode] | 100.00% | 94.66% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 26.98% | 63.91% | 79.45% | 79.74% | 80.19% | 55.11% |
| [bson 2.9.0][bson] | 7.28% | 18.04% | 36.56% | 54.21% | 61.06% | 42.99% |
| [capnp 0.19.6][capnp] | 29.39% | † | 48.76% | 56.19% | 53.60% | 36.71% |
| [cbor4ii 0.3.2][cbor4ii] | 24.15% | 28.68% | 49.99% | 71.59% | 70.89% | 50.48% |
| [ciborium 0.2.2][ciborium] | 4.56% | 11.69% | 49.99% | 71.59% | 70.89% | 51.30% |
| [databuf 0.5.0][databuf] | 52.10% | 68.55% | 91.89% | 92.66% | 86.82% | 60.95% |
| [dlhn 0.1.7][dlhn] | 19.78% | 54.38% | 97.07% | 95.81% | 90.59% | 65.67% |
| [flatbuffers 24.3.25][flatbuffers] | 14.36% | † | 55.13% | 61.64% | 59.09% | 44.78% |
| [msgpacker 0.4.3][msgpacker] | 14.32% | 54.60% | 91.99% | 91.61% | 86.73% | 61.71% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.72% | 34.45% | 85.96% | 86.85% | 80.47% | 56.24% |
| [nanoserde 0.1.37][nanoserde] | 52.61% | 67.25% | 67.29% | 77.41% | 73.70% | 54.49% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.54% | 62.99% | 91.89% | 92.65% | 86.86% | 59.96% |
| [postcard 1.0.8][postcard] | 34.73% | 61.68% | 97.07% | 95.51% | 90.54% | 65.57% |
| [pot 3.0.0][pot] | 6.37% | 21.82% | 72.40% | 77.53% | 75.55% | 52.83% |
| [prost 0.12.6][prost] | <span title="encode">*15.73%\**</span> <span title="populate + encode">*5.89%\**</span> | 43.25% | 79.55% | 79.54% | 72.82% | 49.38% |
| [rkyv 0.8.5][rkyv] | 57.54% | <span title="unvalidated">*88.73%\**</span> <span title="validated upfront with error">*64.80%\**</span> | 69.57% | 73.39% | 70.37% | 50.70% |
| [rmp-serde 1.3.0][rmp-serde] | 10.09% | 44.57% | 89.64% | 88.76% | 82.58% | 60.78% |
| [ron 0.8.1][ron] | 1.27% | 9.24% | 43.78% | 64.30% | 65.70% | 44.07% |
| [savefile 0.17.6][savefile] | 77.47% | 63.84% | 67.29% | 77.40% | 73.69% | 54.86% |
| [serde_bare 0.5.0][serde_bare] | 20.77% | 67.17% | 91.89% | 92.66% | 86.82% | 64.42% |
| [serde_cbor 0.11.2][serde_cbor] | 8.43% | 28.93% | 49.99% | 71.59% | 70.89% | 51.50% |
| [serde_json 1.0.120][serde_json] | 3.78% | 24.89% | 38.51% | 61.38% | 63.63% | 44.60% |
| [simd-json 0.13.10][simd-json] | 7.18% | 30.55% | 38.51% | 61.38% | 63.63% | 44.36% |
| [speedy 0.8.7][speedy] | 75.11% | 78.78% | 79.45% | 79.74% | 80.19% | 58.69% |
| [wiring 0.2.2][wiring] | 74.81% | 71.13% | 67.29% | 85.47% | 83.19% | 62.95% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*25.61%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.70%\**</span> | <span title="validated on-demand with panic">*41.31%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*6.17%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.40%\**</span> <span title="validated upfront with error">*0.48%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.73%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 237.06 µs | <span title="unvalidated">*237.98 µs\**</span> | 6000024 | 5378514 | 5345890 | 7.5382 ms |
| [alkahest 0.1.5][alkahest] | 197.56 µs | † | 6000008 | 5378500 | 5345890 | 7.4013 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6858 ms\**</span> <span title="prepend">*8.9355 ms\**</span> | 8.8641 ms | 8625005 | 6443961 | 6231572 | 71.421 ms |
| [bincode 2.0.0-rc][bincode] | 507.90 µs | 821.89 µs | 6000005 | 5378497 | 5345897 | 7.4159 ms |
| [bincode 1.3.3][bincode1] | 5.1936 ms | 5.6053 ms | 6000008 | 5378500 | 5345890 | 7.3466 ms |
| [bitcode 0.6.0][bitcode] | 1.3997 ms | 805.76 µs | 6000006 | 5182295 | 4923880 | 12.819 ms |
| [borsh 1.5.1][borsh] | 6.0566 ms | 4.4110 ms | 6000004 | 5378496 | 5345889 | 7.5874 ms |
| [bson 2.9.0][bson] | 38.202 ms | 90.090 ms | 23013911 | 9212089 | 7497811 | 107.65 ms |
| [capnp 0.19.6][capnp] | 5.5677 ms | † | 14000088 | 7130367 | 6051062 | 80.774 ms |
| [cbor4ii 0.3.2][cbor4ii] | 9.0650 ms | 48.771 ms | 13125016 | 7524114 | 6757967 | 89.695 ms |
| [ciborium 0.2.2][ciborium] | 68.635 ms | 117.64 ms | 13122324 | 7524660 | 6759658 | 89.710 ms |
| [databuf 0.5.0][databuf] | 2.3963 ms | 5.3115 ms | 6000003 | 5378495 | 5345900 | 7.6415 ms |
| [dlhn 0.1.7][dlhn] | 6.1894 ms | 6.8636 ms | 6000003 | 5378495 | 5345900 | 7.6368 ms |
| [flatbuffers 24.3.25][flatbuffers] | 849.32 µs | † | 6000024 | 5378434 | 5345910 | 7.6414 ms |
| [msgpacker 0.4.3][msgpacker] | 3.1351 ms | 5.0738 ms | 7500005 | 6058442 | 6014337 | 9.7696 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.87 ms | 31.962 ms | 8125037 | 6493484 | 6386940 | 68.088 ms |
| [nanoserde 0.1.37][nanoserde] | 2.3925 ms | 1.0681 ms | 6000008 | 5378500 | 5345890 | 7.5645 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.0115 ms | 4.0210 ms | 6000004 | 5378496 | 5345889 | 7.4135 ms |
| [postcard 1.0.8][postcard] | 477.72 µs | 1.4626 ms | 6000003 | 5378495 | 5345900 | 7.7079 ms |
| [pot 3.0.0][pot] | 40.182 ms | 73.667 ms | 10122342 | 6814618 | 6852251 | 80.488 ms |
| [prost 0.12.6][prost] | <span title="encode">*7.7355 ms\**</span> <span title="populate + encode">*8.7242 ms\**</span> | 14.666 ms | 8750000 | 6665735 | 6421871 | 71.976 ms |
| [rkyv 0.8.5][rkyv] | 237.40 µs | <span title="unvalidated">*198.60 µs\**</span> <span title="validated upfront with error">*149.80 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.4101 ms |
| [rmp-serde 1.3.0][rmp-serde] | 18.225 ms | 17.626 ms | 8125006 | 6494876 | 6391037 | 69.239 ms |
| [ron 0.8.1][ron] | 181.37 ms | 237.64 ms | 22192885 | 8970395 | 8138755 | 149.98 ms |
| [savefile 0.17.6][savefile] | 238.03 µs | 240.23 µs | 6000024 | 5378513 | 5345893 | 7.4065 ms |
| [serde_bare 0.5.0][serde_bare] | 6.4479 ms | 3.8948 ms | 6000003 | 5378495 | 5345900 | 7.6398 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.156 ms | 47.191 ms | 13122324 | 7524660 | 6759658 | 88.861 ms |
| [serde_json 1.0.120][serde_json] | 89.709 ms | 87.276 ms | 26192883 | 9566084 | 8586741 | 156.15 ms |
| [simd-json 0.13.10][simd-json] | 52.539 ms | 72.689 ms | 26192883 | 9566084 | 8586741 | 154.45 ms |
| [speedy 0.8.7][speedy] | 237.90 µs | 239.40 µs | 6000004 | 5378496 | 5345889 | 7.3709 ms |
| [wiring 0.2.2][wiring] | 148.53 µs | 319.07 µs | 6000008 | 5378952 | 5345894 | 7.4764 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1732 ns\**</span> | <span title="unvalidated">*141.88 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8554 ns\**</span> | <span title="validated on-demand with panic">*77.476 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*107.56 ns\**</span> | <span title="validated on-demand with error">*2.1371 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4739 ns\**</span> <span title="validated upfront with error">*40.053 ns\**</span> | <span title="unvalidated">*54.187 µs\**</span> <span title="validated upfront with error">*77.396 µs\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2389 ns\**</span> <span title="validated upfront with error">*4.9605 ns\**</span> | <span title="unvalidated">*48.351 µs\**</span> <span title="validated upfront with error">*77.423 µs\**</span> | <span title="unvalidated">*100.49 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 62.66% | <span title="unvalidated">*62.95%\**</span> | 100.00% | 96.35% | 92.11% | 97.46% |
| [alkahest 0.1.5][alkahest] | 75.18% | † | 100.00% | 96.35% | 92.11% | 99.26% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.22%\**</span> <span title="prepend">*1.66%\**</span> | 1.69% | 69.57% | 80.42% | 79.02% | 10.29% |
| [bincode 2.0.0-rc][bincode] | 29.24% | 18.23% | 100.00% | 96.35% | 92.11% | 99.07% |
| [bincode 1.3.3][bincode1] | 2.86% | 2.67% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bitcode 0.6.0][bitcode] | 10.61% | 18.59% | 100.00% | 100.00% | 100.00% | 57.31% |
| [borsh 1.5.1][borsh] | 2.45% | 3.40% | 100.00% | 96.35% | 92.11% | 96.83% |
| [bson 2.9.0][bson] | 0.39% | 0.17% | 26.07% | 56.26% | 65.67% | 6.82% |
| [capnp 0.19.6][capnp] | 2.67% | † | 42.86% | 72.68% | 81.37% | 9.10% |
| [cbor4ii 0.3.2][cbor4ii] | 1.64% | 0.31% | 45.71% | 68.88% | 72.86% | 8.19% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.84% | 8.19% |
| [databuf 0.5.0][databuf] | 6.20% | 2.82% | 100.00% | 96.35% | 92.11% | 96.14% |
| [dlhn 0.1.7][dlhn] | 2.40% | 2.18% | 100.00% | 96.35% | 92.11% | 96.20% |
| [flatbuffers 24.3.25][flatbuffers] | 17.49% | † | 100.00% | 96.35% | 92.11% | 96.14% |
| [msgpacker 0.4.3][msgpacker] | 4.74% | 2.95% | 80.00% | 85.54% | 81.87% | 75.20% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.47% | 73.85% | 79.81% | 77.09% | 10.79% |
| [nanoserde 0.1.37][nanoserde] | 6.21% | 14.02% | 100.00% | 96.35% | 92.11% | 97.12% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.96% | 3.73% | 100.00% | 96.35% | 92.11% | 99.10% |
| [postcard 1.0.8][postcard] | 31.09% | 10.24% | 100.00% | 96.35% | 92.11% | 95.31% |
| [pot 3.0.0][pot] | 0.37% | 0.20% | 59.27% | 76.05% | 71.86% | 9.13% |
| [prost 0.12.6][prost] | <span title="encode">*1.92%\**</span> <span title="populate + encode">*1.70%\**</span> | 1.02% | 68.57% | 77.75% | 76.67% | 10.21% |
| [rkyv 0.8.5][rkyv] | 62.57% | <span title="unvalidated">*75.43%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 99.14% |
| [rmp-serde 1.3.0][rmp-serde] | 0.81% | 0.85% | 73.85% | 79.79% | 77.04% | 10.61% |
| [ron 0.8.1][ron] | 0.08% | 0.06% | 27.04% | 57.77% | 60.50% | 4.90% |
| [savefile 0.17.6][savefile] | 62.40% | 62.36% | 100.00% | 96.35% | 92.11% | 99.19% |
| [serde_bare 0.5.0][serde_bare] | 2.30% | 3.85% | 100.00% | 96.35% | 92.11% | 96.16% |
| [serde_cbor 0.11.2][serde_cbor] | 0.42% | 0.32% | 45.72% | 68.87% | 72.84% | 8.27% |
| [serde_json 1.0.120][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.70% |
| [simd-json 0.13.10][simd-json] | 0.28% | 0.21% | 22.91% | 54.17% | 57.34% | 4.76% |
| [speedy 0.8.7][speedy] | 62.43% | 62.57% | 100.00% | 96.35% | 92.11% | 99.67% |
| [wiring 0.2.2][wiring] | 100.00% | 46.95% | 100.00% | 96.34% | 92.11% | 98.26% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.01%\**</span> | <span title="unvalidated">*34.08%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.77%\**</span> | <span title="validated on-demand with panic">*62.41%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.15%\**</span> | <span title="validated on-demand with error">*2.26%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.08%\**</span> <span title="validated upfront with error">*3.09%\**</span> | <span title="unvalidated">*89.23%\**</span> <span title="validated upfront with error">*62.47%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.98%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*62.45%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 192.53 µs | <span title="unvalidated">*1.3061 ms\**</span> | 1290592 | 397700 | 339999 | 4.9486 ms |
| [alkahest 0.1.5][alkahest] | 214.11 µs | † | 667570 | 325484 | 320452 | 3.9047 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*940.92 µs\**</span> <span title="prepend">*846.72 µs\**</span> | 3.2007 ms | 489348 | 281173 | 249546 | 3.0772 ms |
| [bincode 2.0.0-rc][bincode] | 308.91 µs | 2.1054 ms | 367413 | 221291 | 206273 | 2.4684 ms |
| [bincode 1.3.3][bincode1] | 598.65 µs | 1.8552 ms | 569975 | 240525 | 232423 | 2.8975 ms |
| [bitcode 0.6.0][bitcode] | 127.96 µs | 1.2814 ms | 327688 | 200947 | 182736 | 743.56 µs |
| [borsh 1.5.1][borsh] | 551.97 µs | 1.8388 ms | 446595 | 234236 | 210008 | 2.4930 ms |
| [bson 2.9.0][bson] | 2.8753 ms | 9.0487 ms | 1619653 | 502185 | 328399 | 4.7974 ms |
| [capnp 0.19.6][capnp] | 461.59 µs | † | 803896 | 335606 | 280851 | 3.9318 ms |
| [cbor4ii 0.3.2][cbor4ii] | 774.71 µs | 4.6873 ms | 1109831 | 344745 | 274514 | 3.8247 ms |
| [ciborium 0.2.2][ciborium] | 3.7921 ms | 10.387 ms | 1109821 | 344751 | 274526 | 3.8262 ms |
| [databuf 0.5.0][databuf] | 294.44 µs | 1.7503 ms | 356311 | 213062 | 198488 | 2.4101 ms |
| [dlhn 0.1.7][dlhn] | 787.38 µs | 2.6711 ms | 366496 | 220600 | 205683 | 2.4866 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.2658 ms | † | 844168 | 345696 | 294015 | 3.8652 ms |
| [msgpacker 0.4.3][msgpacker] | 753.08 µs | 2.8223 ms | 391251 | 236877 | 220476 | 2.6184 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1899 ms | 3.9629 ms | 449745 | 252432 | 231110 | 2.7606 ms |
| [nanoserde 0.1.37][nanoserde] | 271.42 µs | 1.9360 ms | 567975 | 239930 | 232419 | 2.8769 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 609.81 µs | 2.0025 ms | 356311 | 212976 | 198524 | 2.3801 ms |
| [postcard 1.0.8][postcard] | 443.76 µs | 2.0966 ms | 367489 | 221913 | 207344 | 2.5026 ms |
| [pot 3.0.0][pot] | 2.4299 ms | 6.0319 ms | 599125 | 299158 | 247693 | 3.1772 ms |
| [prost 0.12.6][prost] | <span title="encode">*1.2437 ms\**</span> <span title="populate + encode">*2.9008 ms\**</span> | 3.4969 ms | 596811 | 305319 | 269310 | 3.4216 ms |
| [rkyv 0.8.5][rkyv] | 337.18 µs | <span title="unvalidated">*1.5111 ms\**</span> <span title="validated upfront with error">*2.0261 ms\**</span> | 603776 | 254776 | 220087 | 2.7484 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5564 ms | 3.0464 ms | 424533 | 245214 | 226188 | 2.6761 ms |
| [ron 0.8.1][ron] | 7.4507 ms | 17.566 ms | 1465223 | 434935 | 343338 | 5.8207 ms |
| [savefile 0.17.6][savefile] | 209.65 µs | 1.8599 ms | 566991 | 239361 | 232013 | 2.8677 ms |
| [serde_bare 0.5.0][serde_bare] | 768.47 µs | 2.3537 ms | 356311 | 213062 | 198488 | 2.3606 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7104 ms | 4.8284 ms | 1109821 | 344751 | 274526 | 3.8401 ms |
| [serde_json 1.0.120][serde_json] | 3.6029 ms | 6.8631 ms | 1623191 | 466527 | 359623 | 6.0181 ms |
| [simd-json 0.13.10][simd-json] | 2.2018 ms | 4.5635 ms | 1623191 | 466527 | 359623 | 5.9864 ms |
| [speedy 0.8.7][speedy] | 281.17 µs | 1.6121 ms | 449595 | 234970 | 210361 | 2.4924 ms |
| [wiring 0.2.2][wiring] | 219.35 µs | 1.8427 ms | 566975 | 247810 | 225259 | 2.8902 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.268 µs\**</span> | <span title="unvalidated">*37.242 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8548 ns\**</span> | <span title="validated on-demand with panic">*7.5208 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*73.812 ns\**</span> | <span title="validated on-demand with error">*408.36 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4739 ns\**</span> <span title="validated upfront with error">*2.1857 ms\**</span> | <span title="unvalidated">*1.3716 µs\**</span> <span title="validated upfront with error">*2.1849 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2366 ns\**</span> <span title="validated upfront with error">*498.51 µs\**</span> | <span title="unvalidated">*163.03 ns\**</span> <span title="validated upfront with error">*498.92 µs\**</span> | <span title="unvalidated">*702.90 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 66.46% | <span title="unvalidated">*98.11%\**</span> | 25.39% | 50.53% | 53.75% | 15.03% |
| [alkahest 0.1.5][alkahest] | 59.76% | † | 49.09% | 61.74% | 57.02% | 19.04% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*13.60%\**</span> <span title="prepend">*15.11%\**</span> | 40.03% | 66.96% | 71.47% | 73.23% | 24.16% |
| [bincode 2.0.0-rc][bincode] | 41.42% | 60.86% | 89.19% | 90.81% | 88.59% | 30.12% |
| [bincode 1.3.3][bincode1] | 21.37% | 69.07% | 57.49% | 83.55% | 78.62% | 25.66% |
| [bitcode 0.6.0][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 23.18% | 69.69% | 73.37% | 85.79% | 87.01% | 29.83% |
| [bson 2.9.0][bson] | 4.45% | 14.16% | 20.23% | 40.01% | 55.64% | 15.50% |
| [capnp 0.19.6][capnp] | 27.72% | † | 40.76% | 59.88% | 65.07% | 18.91% |
| [cbor4ii 0.3.2][cbor4ii] | 16.52% | 27.34% | 29.53% | 58.29% | 66.57% | 19.44% |
| [ciborium 0.2.2][ciborium] | 3.37% | 12.34% | 29.53% | 58.29% | 66.56% | 19.43% |
| [databuf 0.5.0][databuf] | 43.46% | 73.21% | 91.97% | 94.31% | 92.06% | 30.85% |
| [dlhn 0.1.7][dlhn] | 16.25% | 47.97% | 89.41% | 91.09% | 88.84% | 29.90% |
| [flatbuffers 24.3.25][flatbuffers] | 3.92% | † | 38.82% | 58.13% | 62.15% | 19.24% |
| [msgpacker 0.4.3][msgpacker] | 16.99% | 45.40% | 83.75% | 84.83% | 82.88% | 28.40% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.47% | 32.33% | 72.86% | 79.60% | 79.07% | 26.93% |
| [nanoserde 0.1.37][nanoserde] | 47.14% | 66.19% | 57.69% | 83.75% | 78.62% | 25.85% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.98% | 63.99% | 91.97% | 94.35% | 92.05% | 31.24% |
| [postcard 1.0.8][postcard] | 28.84% | 61.12% | 89.17% | 90.55% | 88.13% | 29.71% |
| [pot 3.0.0][pot] | 5.27% | 21.24% | 54.69% | 67.17% | 73.78% | 23.40% |
| [prost 0.12.6][prost] | <span title="encode">*10.29%\**</span> <span title="populate + encode">*4.41%\**</span> | 36.64% | 54.91% | 65.82% | 67.85% | 21.73% |
| [rkyv 0.8.5][rkyv] | 37.95% | <span title="unvalidated">*84.80%\**</span> <span title="validated upfront with error">*63.24%\**</span> | 54.27% | 78.87% | 83.03% | 27.05% |
| [rmp-serde 1.3.0][rmp-serde] | 8.22% | 42.06% | 77.19% | 81.95% | 80.79% | 27.79% |
| [ron 0.8.1][ron] | 1.72% | 7.29% | 22.36% | 46.20% | 53.22% | 12.77% |
| [savefile 0.17.6][savefile] | 61.04% | 68.90% | 57.79% | 83.95% | 78.76% | 25.93% |
| [serde_bare 0.5.0][serde_bare] | 16.65% | 54.44% | 91.97% | 94.31% | 92.06% | 31.50% |
| [serde_cbor 0.11.2][serde_cbor] | 7.48% | 26.54% | 29.53% | 58.29% | 66.56% | 19.36% |
| [serde_json 1.0.120][serde_json] | 3.55% | 18.67% | 20.19% | 43.07% | 50.81% | 12.36% |
| [simd-json 0.13.10][simd-json] | 5.81% | 28.08% | 20.19% | 43.07% | 50.81% | 12.42% |
| [speedy 0.8.7][speedy] | 45.51% | 79.49% | 72.89% | 85.52% | 86.87% | 29.83% |
| [wiring 0.2.2][wiring] | 58.34% | 69.54% | 57.80% | 81.09% | 81.12% | 25.73% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.44%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*2.17%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*39.92%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.89%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 485.46 µs | <span title="unvalidated">*2.3052 ms\**</span> | 2984682 | 1398800 | 1264717 | 13.895 ms |
| [alkahest 0.1.5][alkahest] | 601.88 µs | † | 1863391 | 1234113 | 1202345 | 11.813 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*4.6554 ms\**</span> <span title="prepend">*2.5004 ms\**</span> | 8.5122 ms | 1664428 | 1264167 | 1216472 | 11.117 ms |
| [bincode 2.0.0-rc][bincode] | 700.50 µs | 3.8992 ms | 1372381 | 1091486 | 1037296 | 8.9246 ms |
| [bincode 1.3.3][bincode1] | 3.7112 ms | 3.9832 ms | 1811011 | 1115281 | 1025627 | 9.8478 ms |
| [bitcode 0.6.0][bitcode] | 706.86 µs | 2.3023 ms | 948499 | 857321 | 837658 | 3.0123 ms |
| [borsh 1.5.1][borsh] | 2.8870 ms | 2.8438 ms | 1486162 | 1082357 | 1013550 | 9.4914 ms |
| [bson 2.9.0][bson] | 20.589 ms | 50.343 ms | 10030880 | 2833079 | 1600859 | 27.392 ms |
| [capnp 0.19.6][capnp] | 2.2289 ms | † | 2664040 | 1511895 | 1212087 | 13.963 ms |
| [cbor4ii 0.3.2][cbor4ii] | 3.2094 ms | 17.479 ms | 5878791 | 1655835 | 1431390 | 21.299 ms |
| [ciborium 0.2.2][ciborium] | 23.455 ms | 53.719 ms | 5878653 | 1655791 | 1431560 | 20.565 ms |
| [databuf 0.5.0][databuf] | 1.2504 ms | 3.7695 ms | 1288257 | 1037579 | 984337 | 8.5225 ms |
| [dlhn 0.1.7][dlhn] | 4.9227 ms | 6.5952 ms | 1279599 | 1052061 | 1021161 | 8.2318 ms |
| [flatbuffers 24.3.25][flatbuffers] | 5.3291 ms | † | 2273740 | 1408408 | 1235566 | 12.663 ms |
| [msgpacker 0.4.3][msgpacker] | 1.5165 ms | 6.1217 ms | 1424043 | 1128758 | 1110156 | 10.284 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.296 ms | 17.603 ms | 1728519 | 1247642 | 1233323 | 11.624 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2516 ms | 2.8888 ms | 1770477 | 1108304 | 1029947 | 9.9231 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.0483 ms | 3.2623 ms | 1288257 | 1039269 | 986510 | 8.4407 ms |
| [postcard 1.0.8][postcard] | 2.1095 ms | 4.0585 ms | 1279599 | 1058243 | 1016738 | 8.2602 ms |
| [pot 3.0.0][pot] | 13.799 ms | 30.000 ms | 2544810 | 1447453 | 1268390 | 15.041 ms |
| [prost 0.12.6][prost] | <span title="encode">*5.2664 ms\**</span> <span title="populate + encode">*9.0715 ms\**</span> | 8.7800 ms | 1818378 | 1307777 | 1266311 | 11.558 ms |
| [rkyv 0.8.5][rkyv] | 1.0276 ms | <span title="unvalidated">*2.1572 ms\**</span> <span title="validated upfront with error">*2.5570 ms\**</span> | 2029080 | 1351984 | 1183990 | 12.213 ms |
| [rmp-serde 1.3.0][rmp-serde] | 9.8763 ms | 10.702 ms | 1703813 | 1231892 | 1200208 | 11.167 ms |
| [ron 0.8.1][ron] | 38.530 ms | 88.810 ms | 8476284 | 2181196 | 1783971 | 33.459 ms |
| [savefile 0.17.6][savefile] | 821.51 µs | 2.7168 ms | 1750226 | 1101682 | 1027828 | 9.7205 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8009 ms | 5.0284 ms | 1288257 | 1037597 | 984356 | 8.5896 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.4724 ms | 21.417 ms | 5878653 | 1655791 | 1431560 | 20.684 ms |
| [serde_json 1.0.120][serde_json] | 19.879 ms | 30.371 ms | 9175594 | 2334253 | 1800713 | 33.499 ms |
| [simd-json 0.13.10][simd-json] | 11.563 ms | 25.741 ms | 9175594 | 2334253 | 1800713 | 33.956 ms |
| [speedy 0.8.7][speedy] | 706.72 µs | 2.4026 ms | 1546963 | 1093532 | 1013443 | 9.6034 ms |
| [wiring 0.2.2][wiring] | 700.99 µs | 2.7232 ms | 1750210 | 1129857 | 1058906 | 10.075 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*65.767 µs\**</span> | <span title="unvalidated">*66.951 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8552 ns\**</span> | <span title="validated on-demand with panic">*658.01 ns\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*72.836 ns\**</span> | <span title="validated on-demand with error">*704.62 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4736 ns\**</span> <span title="validated upfront with error">*4.9211 ms\**</span> | <span title="unvalidated">*2.6305 µs\**</span> <span title="validated upfront with error">*4.9393 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2375 ns\**</span> <span title="validated upfront with error">*417.00 µs\**</span> | <span title="unvalidated">*429.11 ns\**</span> <span title="validated upfront with error">*416.31 µs\**</span> | <span title="unvalidated">*233.27 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.58%\**</span> | 31.78% | 61.29% | 66.23% | 21.68% |
| [alkahest 0.1.5][alkahest] | 80.66% | † | 50.90% | 69.47% | 69.67% | 25.50% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*10.43%\**</span> <span title="prepend">*19.42%\**</span> | 25.34% | 56.99% | 67.82% | 68.86% | 27.10% |
| [bincode 2.0.0-rc][bincode] | 69.30% | 55.32% | 69.11% | 78.55% | 80.75% | 33.75% |
| [bincode 1.3.3][bincode1] | 13.08% | 54.16% | 52.37% | 76.87% | 81.67% | 30.59% |
| [bitcode 0.6.0][bitcode] | 68.68% | 93.70% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 16.82% | 75.86% | 63.82% | 79.21% | 82.65% | 31.74% |
| [bson 2.9.0][bson] | 2.36% | 4.29% | 9.46% | 30.26% | 52.33% | 11.00% |
| [capnp 0.19.6][capnp] | 21.78% | † | 35.60% | 56.71% | 69.11% | 21.57% |
| [cbor4ii 0.3.2][cbor4ii] | 15.13% | 12.34% | 16.13% | 51.78% | 58.52% | 14.14% |
| [ciborium 0.2.2][ciborium] | 2.07% | 4.02% | 16.13% | 51.78% | 58.51% | 14.65% |
| [databuf 0.5.0][databuf] | 38.82% | 57.23% | 73.63% | 82.63% | 85.10% | 35.35% |
| [dlhn 0.1.7][dlhn] | 9.86% | 32.71% | 74.12% | 81.49% | 82.03% | 36.59% |
| [flatbuffers 24.3.25][flatbuffers] | 9.11% | † | 41.72% | 60.87% | 67.80% | 23.79% |
| [msgpacker 0.4.3][msgpacker] | 32.01% | 35.24% | 66.61% | 75.95% | 75.45% | 29.29% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.66% | 12.25% | 54.87% | 68.72% | 67.92% | 25.91% |
| [nanoserde 0.1.37][nanoserde] | 38.79% | 74.67% | 53.57% | 77.35% | 81.33% | 30.36% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 15.93% | 66.13% | 73.63% | 82.49% | 84.91% | 35.69% |
| [postcard 1.0.8][postcard] | 23.01% | 53.15% | 74.12% | 81.01% | 82.39% | 36.47% |
| [pot 3.0.0][pot] | 3.52% | 7.19% | 37.27% | 59.23% | 66.04% | 20.03% |
| [prost 0.12.6][prost] | <span title="encode">*9.22%\**</span> <span title="populate + encode">*5.35%\**</span> | 24.57% | 52.16% | 65.56% | 66.15% | 26.06% |
| [rkyv 0.8.5][rkyv] | 47.24% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*84.36%\**</span> | 46.75% | 63.41% | 70.75% | 24.66% |
| [rmp-serde 1.3.0][rmp-serde] | 4.92% | 20.16% | 55.67% | 69.59% | 69.79% | 26.97% |
| [ron 0.8.1][ron] | 1.26% | 2.43% | 11.19% | 39.31% | 46.95% | 9.00% |
| [savefile 0.17.6][savefile] | 59.09% | 79.40% | 54.19% | 77.82% | 81.50% | 30.99% |
| [serde_bare 0.5.0][serde_bare] | 10.11% | 42.90% | 73.63% | 82.63% | 85.10% | 35.07% |
| [serde_cbor 0.11.2][serde_cbor] | 5.12% | 10.07% | 16.13% | 51.78% | 58.51% | 14.56% |
| [serde_json 1.0.120][serde_json] | 2.44% | 7.10% | 10.34% | 36.73% | 46.52% | 8.99% |
| [simd-json 0.13.10][simd-json] | 4.20% | 8.38% | 10.34% | 36.73% | 46.52% | 8.87% |
| [speedy 0.8.7][speedy] | 68.69% | 89.79% | 61.31% | 78.40% | 82.65% | 31.37% |
| [wiring 0.2.2][wiring] | 69.25% | 79.22% | 54.19% | 75.88% | 79.11% | 29.90% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.64%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.70%\**</span> | <span title="validated on-demand with panic">*65.21%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*60.90%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.31%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
[rkyv]: https://crates.io/crates/rkyv/0.8.5
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
