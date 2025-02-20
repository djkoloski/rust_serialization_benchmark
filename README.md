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

## Last updated: 2025-02-20 18:31:20

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.87.0-nightly (f280acf4c 2025-02-19)
binary: rustc
commit-hash: f280acf4c743806abbbbcfe65050ac52ec4bdec0
commit-date: 2025-02-19
host: x86_64-unknown-linux-gnu
release: 1.87.0-nightly
LLVM version: 20.1.0
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
| [bilrost 0.1012.2][bilrost] | <span title="encode">*474.94 µs\**</span> <span title="prepend">*446.29 µs\**</span> | 2.5734 ms | 857.11 µs | 804955 | 328941 | 285485 | 4.6030 ms |
| [bincode 2.0.0-rc][bincode] | 301.23 µs | 2.4302 ms | † | 741295 | 303944 | 257153 | 3.8977 ms |
| [bincode 1.3.3][bincode1] | 548.53 µs | 2.0657 ms | 589.28 µs | 1045784 | 373127 | 311761 | 4.8078 ms |
| [bitcode 0.6.4][bitcode] | 138.44 µs | 1.4687 ms | 62.308 µs | 703710 | 288826 | 229755 | 2.4965 ms |
| [borsh 1.5.3][borsh] | 548.64 µs | 2.1746 ms | † | 885780 | 362204 | 286514 | 4.4255 ms |
| [capnp 0.20.3][capnp] | 504.31 µs | † | † | 1443216 | 513986 | 428649 | 6.3161 ms |
| [cbor4ii 0.3.3][cbor4ii] | 655.06 µs | 5.1435 ms | 3.3678 ms | 1407835 | 403440 | 324081 | 4.7380 ms |
| [ciborium 0.2.2][ciborium] | 4.0283 ms | 12.237 ms | † | 1407835 | 403440 | 324081 | 4.7938 ms |
| [databuf 0.5.0][databuf] | 263.62 µs | 2.0038 ms | 630.72 µs | 765778 | 311715 | 264630 | 3.9106 ms |
| [dlhn 0.1.7][dlhn] | 730.71 µs | 2.5579 ms | † | 724953 | 301446 | 253629 | 3.6063 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0180 ms | † | † | 1276368 | 468539 | 388832 | 5.1587 ms |
| [minicbor 0.25.1][minicbor] | 606.71 µs | 2.9800 ms | 1.3321 ms | 817830 | 332671 | 284548 | 4.2994 ms |
| [msgpacker 0.4.5][msgpacker] | 1.2386 ms | 2.5076 ms | † | 764996 | 315291 | 264898 | 3.8831 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4001 ms | 4.3079 ms | 2.7711 ms | 818669 | 332556 | 285514 | 4.3047 ms |
| [nanoserde 0.1.37][nanoserde] | 242.83 µs | 2.0592 ms | † | 1045784 | 373127 | 311761 | 4.4943 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 670.17 µs | 2.1830 ms | † | 765778 | 311743 | 264518 | 3.8225 ms |
| [postcard 1.1.1][postcard] | 437.40 µs | 2.2670 ms | 926.47 µs | 724953 | 302399 | 253747 | 3.6052 ms |
| [pot 3.0.1][pot] | 2.4701 ms | 6.3558 ms | 4.8783 ms | 971922 | 372513 | 304122 | 4.6443 ms |
| [prost 0.13.4][prost] | <span title="encode">*936.13 µs\**</span> <span title="populate + encode">*2.4318 ms\**</span> | 3.2750 ms | † | 884628 | 363130 | 315494 | 5.0847 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.3641 ms\**</span> <span title="populate + encode">*3.1401 ms\**</span> | 3.7692 ms | † | 884628 | 363130 | 315494 | 4.8062 ms |
| [rkyv 0.8.9][rkyv] | 244.35 µs | <span title="unvalidated">*1.5483 ms\**</span> <span title="validated upfront with error">*1.9339 ms\**</span> | † | 1011488 | 393526 | 326517 | 4.9700 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3658 ms | 3.0526 ms | 1.4006 ms | 784997 | 325384 | 278219 | 4.1741 ms |
| [ron 0.8.1][ron] | 11.807 ms | 15.168 ms | 13.231 ms | 1607459 | 449158 | 349713 | 5.7508 ms |
| [savefile 0.18.5][savefile] | 191.99 µs | 2.1507 ms | † | 1045800 | 373139 | 311761 | 4.5057 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5459 ms | 4.9937 ms | 3.0843 ms | 1584946 | 413733 | 341439 | 4.9845 ms |
| [serde_bare 0.5.0][serde_bare] | 690.81 µs | 2.0715 ms | † | 765778 | 311715 | 264630 | 3.8299 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0715 ms | 4.8660 ms | 3.2975 ms | 1407835 | 403440 | 324081 | 4.7289 ms |
| [serde_json 1.0.134][serde_json] | 3.8698 ms | 5.8965 ms | † | 1827461 | 470560 | 361090 | 5.6681 ms |
| [simd-json 0.14.3][simd-json] | 2.1452 ms | 4.6952 ms | † | 1827461 | 470560 | 361090 | 5.6488 ms |
| [speedy 0.8.7][speedy] | 196.91 µs | 1.7452 ms | 366.22 µs | 885780 | 362204 | 286514 | 4.2836 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.556 ns\**</span> | <span title="validated on-demand with error">*172.25 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.5002 ns\**</span> <span title="validated upfront with error">*1.9583 ms\**</span> | <span title="unvalidated">*49.342 µs\**</span> <span title="validated upfront with error">*2.0607 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2427 ns\**</span> <span title="validated upfront with error">*382.90 µs\**</span> | <span title="unvalidated">*10.486 µs\**</span> <span title="validated upfront with error">*392.90 µs\**</span> | <span title="unvalidated">*7.4965 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*29.15%\**</span> <span title="prepend">*31.02%\**</span> | 57.07% | 7.27% | 87.42% | 87.80% | 80.48% | 54.24% |
| [bincode 2.0.0-rc][bincode] | 45.96% | 60.44% | † | 94.93% | 95.03% | 89.35% | 64.05% |
| [bincode 1.3.3][bincode1] | 25.24% | 71.10% | 10.57% | 67.29% | 77.41% | 73.70% | 51.93% |
| [bitcode 0.6.4][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 25.23% | 67.54% | † | 79.45% | 79.74% | 80.19% | 56.41% |
| [capnp 0.20.3][capnp] | 27.45% | † | † | 48.76% | 56.19% | 53.60% | 39.53% |
| [cbor4ii 0.3.3][cbor4ii] | 21.13% | 28.55% | 1.85% | 49.99% | 71.59% | 70.89% | 52.69% |
| [ciborium 0.2.2][ciborium] | 3.44% | 12.00% | † | 49.99% | 71.59% | 70.89% | 52.08% |
| [databuf 0.5.0][databuf] | 52.51% | 73.30% | 9.88% | 91.89% | 92.66% | 86.82% | 63.84% |
| [dlhn 0.1.7][dlhn] | 18.95% | 57.42% | † | 97.07% | 95.81% | 90.59% | 69.23% |
| [flatbuffers 24.12.23][flatbuffers] | 13.60% | † | † | 55.13% | 61.64% | 59.09% | 48.39% |
| [minicbor 0.25.1][minicbor] | 22.82% | 49.29% | 4.68% | 86.05% | 86.82% | 80.74% | 58.07% |
| [msgpacker 0.4.5][msgpacker] | 11.18% | 58.57% | † | 91.99% | 91.61% | 86.73% | 64.29% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.56% | 34.09% | 2.25% | 85.96% | 86.85% | 80.47% | 57.99% |
| [nanoserde 0.1.37][nanoserde] | 57.01% | 71.32% | † | 67.29% | 77.41% | 73.70% | 55.55% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.66% | 67.28% | † | 91.89% | 92.65% | 86.86% | 65.31% |
| [postcard 1.1.1][postcard] | 31.65% | 64.79% | 6.73% | 97.07% | 95.51% | 90.54% | 69.25% |
| [pot 3.0.1][pot] | 5.60% | 23.11% | 1.28% | 72.40% | 77.53% | 75.55% | 53.75% |
| [prost 0.13.4][prost] | <span title="encode">*14.79%\**</span> <span title="populate + encode">*5.69%\**</span> | 44.85% | † | 79.55% | 79.54% | 72.82% | 49.10% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*10.15%\**</span> <span title="populate + encode">*4.41%\**</span> | 38.97% | † | 79.55% | 79.54% | 72.82% | 51.94% |
| [rkyv 0.8.9][rkyv] | 56.66% | <span title="unvalidated">*94.86%\**</span> <span title="validated upfront with error">*75.94%\**</span> | † | 69.57% | 73.39% | 70.37% | 50.23% |
| [rmp-serde 1.3.0][rmp-serde] | 10.14% | 48.11% | 4.45% | 89.64% | 88.76% | 82.58% | 59.81% |
| [ron 0.8.1][ron] | 1.17% | 9.68% | 0.47% | 43.78% | 64.30% | 65.70% | 43.41% |
| [savefile 0.18.5][savefile] | 72.11% | 68.29% | † | 67.29% | 77.40% | 73.70% | 55.41% |
| [serde-brief 0.1.0][serde-brief] | 8.96% | 29.41% | 2.02% | 44.40% | 69.81% | 67.29% | 50.09% |
| [serde_bare 0.5.0][serde_bare] | 20.04% | 70.90% | † | 91.89% | 92.66% | 86.82% | 65.18% |
| [serde_cbor 0.11.2][serde_cbor] | 6.68% | 30.18% | 1.89% | 49.99% | 71.59% | 70.89% | 52.79% |
| [serde_json 1.0.134][serde_json] | 3.58% | 24.91% | † | 38.51% | 61.38% | 63.63% | 44.04% |
| [simd-json 0.14.3][simd-json] | 6.45% | 31.28% | † | 38.51% | 61.38% | 63.63% | 44.20% |
| [speedy 0.8.7][speedy] | 70.31% | 84.16% | 17.01% | 79.45% | 79.74% | 80.19% | 58.28% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*6.09%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.70%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.25%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.67%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*6.7233 ms\**</span> <span title="prepend">*8.9657 ms\**</span> | 8.3714 ms | 8625005 | 6443961 | 6231572 | 68.839 ms |
| [bincode 2.0.0-rc][bincode] | 2.4117 ms | 1.0223 ms | 6000005 | 5378497 | 5345897 | 7.5271 ms |
| [bincode 1.3.3][bincode1] | 5.1520 ms | 5.8994 ms | 6000008 | 5378500 | 5345890 | 7.0102 ms |
| [bitcode 0.6.4][bitcode] | 1.3938 ms | 794.70 µs | 6000006 | 5182295 | 4923880 | 12.521 ms |
| [borsh 1.5.3][borsh] | 6.2914 ms | 4.2603 ms | 6000004 | 5378496 | 5345889 | 7.5135 ms |
| [capnp 0.20.3][capnp] | 6.2878 ms | † | 14000088 | 7130367 | 6051062 | 81.204 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.8812 ms | 50.827 ms | 13125016 | 7524114 | 6757967 | 88.959 ms |
| [ciborium 0.2.2][ciborium] | 69.880 ms | 120.86 ms | 13122324 | 7524660 | 6759658 | 89.379 ms |
| [databuf 0.5.0][databuf] | 2.4065 ms | 5.3528 ms | 6000003 | 5378495 | 5345900 | 7.5824 ms |
| [dlhn 0.1.7][dlhn] | 6.2063 ms | 6.7919 ms | 6000003 | 5378495 | 5345900 | 7.5545 ms |
| [flatbuffers 24.12.23][flatbuffers] | 872.37 µs | † | 6000024 | 5378434 | 5345910 | 7.5338 ms |
| [minicbor 0.25.1][minicbor] | 6.0573 ms | 11.850 ms | 8125006 | 6494907 | 6390894 | 69.074 ms |
| [msgpacker 0.4.5][msgpacker] | 19.564 ms | 5.2842 ms | 7500005 | 6058442 | 6014337 | 9.5015 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.57 ms | 32.982 ms | 8125037 | 6493484 | 6386940 | 69.735 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3898 ms | 1.1080 ms | 6000008 | 5378500 | 5345890 | 7.5527 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 4.7123 ms | 2.8215 ms | 6000004 | 5378496 | 5345889 | 7.3189 ms |
| [postcard 1.1.1][postcard] | 509.51 µs | 1.2481 ms | 6000003 | 5378495 | 5345900 | 7.6260 ms |
| [pot 3.0.1][pot] | 40.094 ms | 70.894 ms | 10122342 | 6814618 | 6852251 | 81.385 ms |
| [prost 0.13.4][prost] | <span title="encode">*7.7933 ms\**</span> <span title="populate + encode">*8.4843 ms\**</span> | 16.070 ms | 8750000 | 6665735 | 6421871 | 71.486 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*14.336 ms\**</span> <span title="populate + encode">*30.963 ms\**</span> | 28.770 ms | 8750000 | 6665735 | 6421871 | 77.149 ms |
| [rkyv 0.8.9][rkyv] | 147.70 µs | <span title="unvalidated">*158.09 µs\**</span> <span title="validated upfront with error">*196.86 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.5323 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.397 ms | 17.781 ms | 8125006 | 6494876 | 6391037 | 69.993 ms |
| [ron 0.8.1][ron] | 172.40 ms | 234.81 ms | 22192885 | 8970395 | 8138755 | 148.72 ms |
| [savefile 0.18.5][savefile] | 197.80 µs | 197.41 µs | 6000024 | 5378519 | 5345892 | 7.4873 ms |
| [serde-brief 0.1.0][serde-brief] | 23.322 ms | 35.475 ms | 15750015 | 8024540 | 6816643 | 93.191 ms |
| [serde_bare 0.5.0][serde_bare] | 5.1886 ms | 4.8217 ms | 6000003 | 5378495 | 5345900 | 7.5254 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.937 ms | 46.933 ms | 13122324 | 7524660 | 6759658 | 89.208 ms |
| [serde_json 1.0.134][serde_json] | 89.388 ms | 87.406 ms | 26192883 | 9566084 | 8586741 | 154.77 ms |
| [simd-json 0.14.3][simd-json] | 51.567 ms | 69.332 ms | 26192883 | 9566084 | 8586741 | 155.24 ms |
| [speedy 0.8.7][speedy] | 196.96 µs | 197.48 µs | 6000004 | 5378496 | 5345889 | 7.4693 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*101.53 ns\**</span> | <span title="validated on-demand with error">*2.2888 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4752 ns\**</span> <span title="validated upfront with error">*40.482 ns\**</span> | <span title="unvalidated">*52.537 µs\**</span> <span title="validated upfront with error">*77.803 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*5.2934 ns\**</span> | <span title="unvalidated">*38.873 µs\**</span> <span title="validated upfront with error">*38.889 µs\**</span> | <span title="unvalidated">*99.368 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*2.20%\**</span> <span title="prepend">*1.65%\**</span> | 1.89% | 69.57% | 80.42% | 79.02% | 10.18% |
| [bincode 2.0.0-rc][bincode] | 6.12% | 15.46% | 100.00% | 96.35% | 92.11% | 93.13% |
| [bincode 1.3.3][bincode1] | 2.87% | 2.68% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bitcode 0.6.4][bitcode] | 10.60% | 19.89% | 100.00% | 100.00% | 100.00% | 55.99% |
| [borsh 1.5.3][borsh] | 2.35% | 3.71% | 100.00% | 96.35% | 92.11% | 93.30% |
| [capnp 0.20.3][capnp] | 2.35% | † | 42.86% | 72.68% | 81.37% | 8.63% |
| [cbor4ii 0.3.3][cbor4ii] | 1.49% | 0.31% | 45.71% | 68.88% | 72.86% | 7.88% |
| [ciborium 0.2.2][ciborium] | 0.21% | 0.13% | 45.72% | 68.87% | 72.84% | 7.84% |
| [databuf 0.5.0][databuf] | 6.14% | 2.95% | 100.00% | 96.35% | 92.11% | 92.45% |
| [dlhn 0.1.7][dlhn] | 2.38% | 2.33% | 100.00% | 96.35% | 92.11% | 92.80% |
| [flatbuffers 24.12.23][flatbuffers] | 16.93% | † | 100.00% | 96.35% | 92.11% | 93.05% |
| [minicbor 0.25.1][minicbor] | 2.44% | 1.33% | 73.85% | 79.79% | 77.05% | 10.15% |
| [msgpacker 0.4.5][msgpacker] | 0.75% | 2.99% | 80.00% | 85.54% | 81.87% | 73.78% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.48% | 73.85% | 79.81% | 77.09% | 10.05% |
| [nanoserde 0.1.37][nanoserde] | 10.63% | 14.27% | 100.00% | 96.35% | 92.11% | 92.82% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.13% | 5.60% | 100.00% | 96.35% | 92.11% | 95.78% |
| [postcard 1.1.1][postcard] | 28.99% | 12.67% | 100.00% | 96.35% | 92.11% | 91.92% |
| [pot 3.0.1][pot] | 0.37% | 0.22% | 59.27% | 76.05% | 71.86% | 8.61% |
| [prost 0.13.4][prost] | <span title="encode">*1.90%\**</span> <span title="populate + encode">*1.74%\**</span> | 0.98% | 68.57% | 77.75% | 76.67% | 9.81% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.03%\**</span> <span title="populate + encode">*0.48%\**</span> | 0.55% | 68.57% | 77.75% | 76.67% | 9.09% |
| [rkyv 0.8.9][rkyv] | 100.00% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*80.31%\**</span> | 100.00% | 96.35% | 92.11% | 93.07% |
| [rmp-serde 1.3.0][rmp-serde] | 0.96% | 0.89% | 73.85% | 79.79% | 77.04% | 10.02% |
| [ron 0.8.1][ron] | 0.09% | 0.07% | 27.04% | 57.77% | 60.50% | 4.71% |
| [savefile 0.18.5][savefile] | 74.67% | 80.08% | 100.00% | 96.35% | 92.11% | 93.63% |
| [serde-brief 0.1.0][serde-brief] | 0.63% | 0.45% | 38.10% | 64.58% | 72.23% | 7.52% |
| [serde_bare 0.5.0][serde_bare] | 2.85% | 3.28% | 100.00% | 96.35% | 92.11% | 93.15% |
| [serde_cbor 0.11.2][serde_cbor] | 0.41% | 0.34% | 45.72% | 68.87% | 72.84% | 7.86% |
| [serde_json 1.0.134][serde_json] | 0.17% | 0.18% | 22.91% | 54.17% | 57.34% | 4.53% |
| [simd-json 0.14.3][simd-json] | 0.29% | 0.23% | 22.91% | 54.17% | 57.34% | 4.52% |
| [speedy 0.8.7][speedy] | 74.99% | 80.05% | 100.00% | 96.35% | 92.11% | 93.85% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.22%\**</span> | <span title="validated on-demand with error">*1.70%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.22%\**</span> <span title="validated upfront with error">*3.07%\**</span> | <span title="unvalidated">*73.99%\**</span> <span title="validated upfront with error">*49.96%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*23.48%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.96%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*957.76 µs\**</span> <span title="prepend">*837.03 µs\**</span> | 3.0846 ms | 1.7399 ms | 489348 | 281173 | 249546 | 3.1024 ms |
| [bincode 2.0.0-rc][bincode] | 287.08 µs | 2.0797 ms | † | 367413 | 221291 | 206273 | 2.4914 ms |
| [bincode 1.3.3][bincode1] | 602.67 µs | 1.8132 ms | 857.32 µs | 569975 | 240525 | 232423 | 2.9112 ms |
| [bitcode 0.6.4][bitcode] | 126.09 µs | 1.2560 ms | 169.26 µs | 327688 | 200947 | 182736 | 754.36 µs |
| [borsh 1.5.3][borsh] | 562.31 µs | 1.8343 ms | † | 446595 | 234236 | 210008 | 2.4856 ms |
| [capnp 0.20.3][capnp] | 439.80 µs | † | † | 803896 | 335606 | 280851 | 3.9318 ms |
| [cbor4ii 0.3.3][cbor4ii] | 806.52 µs | 4.8122 ms | 3.5182 ms | 1109831 | 344745 | 274514 | 3.8310 ms |
| [ciborium 0.2.2][ciborium] | 3.7447 ms | 10.537 ms | † | 1109821 | 344751 | 274526 | 3.8201 ms |
| [databuf 0.5.0][databuf] | 318.04 µs | 1.7296 ms | 780.68 µs | 356311 | 213062 | 198488 | 2.4366 ms |
| [dlhn 0.1.7][dlhn] | 779.31 µs | 2.6022 ms | † | 366496 | 220600 | 205683 | 2.5168 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.2402 ms | † | † | 844168 | 345696 | 294015 | 3.8600 ms |
| [minicbor 0.25.1][minicbor] | 553.88 µs | 3.3289 ms | 1.8685 ms | 428773 | 249857 | 228741 | 2.7303 ms |
| [msgpacker 0.4.5][msgpacker] | 1.0039 ms | 2.8355 ms | † | 391251 | 236877 | 220476 | 2.7116 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1026 ms | 4.1376 ms | 2.9329 ms | 449745 | 252432 | 231110 | 2.7702 ms |
| [nanoserde 0.1.37][nanoserde] | 268.97 µs | 1.9349 ms | † | 567975 | 239930 | 232419 | 2.9129 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 636.40 µs | 1.9453 ms | † | 356311 | 212976 | 198524 | 2.4160 ms |
| [postcard 1.1.1][postcard] | 448.28 µs | 2.0913 ms | 816.37 µs | 367489 | 221913 | 207344 | 2.4995 ms |
| [pot 3.0.1][pot] | 2.4218 ms | 6.3198 ms | 5.0233 ms | 599125 | 299158 | 247693 | 3.1413 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.2812 ms\**</span> <span title="populate + encode">*2.9837 ms\**</span> | 3.4317 ms | † | 596811 | 305319 | 269310 | 3.4355 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.0429 ms\**</span> <span title="populate + encode">*2.9872 ms\**</span> | 3.8237 ms | † | 596811 | 305319 | 269310 | 3.4404 ms |
| [rkyv 0.8.9][rkyv] | 347.50 µs | <span title="unvalidated">*1.5043 ms\**</span> <span title="validated upfront with error">*1.8900 ms\**</span> | † | 603776 | 254776 | 220087 | 2.7488 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4933 ms | 3.0326 ms | 1.6933 ms | 424533 | 245214 | 226188 | 2.7246 ms |
| [ron 0.8.1][ron] | 7.3571 ms | 17.125 ms | 15.276 ms | 1465223 | 434935 | 343338 | 5.8321 ms |
| [savefile 0.18.5][savefile] | 214.69 µs | 1.8398 ms | † | 566991 | 239362 | 232010 | 2.9051 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3407 ms | 5.3868 ms | 3.8023 ms | 1276014 | 373898 | 293679 | 4.0413 ms |
| [serde_bare 0.5.0][serde_bare] | 742.90 µs | 2.3338 ms | † | 356311 | 213062 | 198488 | 2.4574 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8694 ms | 4.8686 ms | 3.4467 ms | 1109821 | 344751 | 274526 | 3.8391 ms |
| [serde_json 1.0.134][serde_json] | 3.6633 ms | 6.6995 ms | † | 1623191 | 466527 | 359623 | 6.0276 ms |
| [simd-json 0.14.3][simd-json] | 2.2546 ms | 4.5958 ms | † | 1623191 | 466527 | 359623 | 6.0347 ms |
| [speedy 0.8.7][speedy] | 261.66 µs | 1.6987 ms | 565.80 µs | 449595 | 234970 | 210361 | 2.5060 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*71.828 ns\**</span> | <span title="validated on-demand with error">*489.93 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4853 ns\**</span> <span title="validated upfront with error">*2.1843 ms\**</span> | <span title="unvalidated">*1.3521 µs\**</span> <span title="validated upfront with error">*2.1927 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*378.82 µs\**</span> | <span title="unvalidated">*240.21 ns\**</span> <span title="validated upfront with error">*379.59 µs\**</span> | <span title="unvalidated">*768.48 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*13.17%\**</span> <span title="prepend">*15.06%\**</span> | 40.72% | 9.73% | 66.96% | 71.47% | 73.23% | 24.32% |
| [bincode 2.0.0-rc][bincode] | 43.92% | 60.39% | † | 89.19% | 90.81% | 88.59% | 30.28% |
| [bincode 1.3.3][bincode1] | 20.92% | 69.27% | 19.74% | 57.49% | 83.55% | 78.62% | 25.91% |
| [bitcode 0.6.4][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 22.42% | 68.47% | † | 73.37% | 85.79% | 87.01% | 30.35% |
| [capnp 0.20.3][capnp] | 28.67% | † | † | 40.76% | 59.88% | 65.07% | 19.19% |
| [cbor4ii 0.3.3][cbor4ii] | 15.63% | 26.10% | 4.81% | 29.53% | 58.29% | 66.57% | 19.69% |
| [ciborium 0.2.2][ciborium] | 3.37% | 11.92% | † | 29.53% | 58.29% | 66.56% | 19.75% |
| [databuf 0.5.0][databuf] | 39.65% | 72.62% | 21.68% | 91.97% | 94.31% | 92.06% | 30.96% |
| [dlhn 0.1.7][dlhn] | 16.18% | 48.27% | † | 89.41% | 91.09% | 88.84% | 29.97% |
| [flatbuffers 24.12.23][flatbuffers] | 3.89% | † | † | 38.82% | 58.13% | 62.15% | 19.54% |
| [minicbor 0.25.1][minicbor] | 22.76% | 37.73% | 9.06% | 76.42% | 80.42% | 79.89% | 27.63% |
| [msgpacker 0.4.5][msgpacker] | 12.56% | 44.30% | † | 83.75% | 84.83% | 82.88% | 27.82% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.47% | 30.36% | 5.77% | 72.86% | 79.60% | 79.07% | 27.23% |
| [nanoserde 0.1.37][nanoserde] | 46.88% | 64.91% | † | 57.69% | 83.75% | 78.62% | 25.90% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 19.81% | 64.57% | † | 91.97% | 94.35% | 92.05% | 31.22% |
| [postcard 1.1.1][postcard] | 28.13% | 60.06% | 20.73% | 89.17% | 90.55% | 88.13% | 30.18% |
| [pot 3.0.1][pot] | 5.21% | 19.87% | 3.37% | 54.69% | 67.17% | 73.78% | 24.01% |
| [prost 0.13.4][prost] | <span title="encode">*9.84%\**</span> <span title="populate + encode">*4.23%\**</span> | 36.60% | † | 54.91% | 65.82% | 67.85% | 21.96% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*12.09%\**</span> <span title="populate + encode">*4.22%\**</span> | 32.85% | † | 54.91% | 65.82% | 67.85% | 21.93% |
| [rkyv 0.8.9][rkyv] | 36.28% | <span title="unvalidated">*83.49%\**</span> <span title="validated upfront with error">*66.46%\**</span> | † | 54.27% | 78.87% | 83.03% | 27.44% |
| [rmp-serde 1.3.0][rmp-serde] | 8.44% | 41.42% | 10.00% | 77.19% | 81.95% | 80.79% | 27.69% |
| [ron 0.8.1][ron] | 1.71% | 7.33% | 1.11% | 22.36% | 46.20% | 53.22% | 12.93% |
| [savefile 0.18.5][savefile] | 58.73% | 68.27% | † | 57.79% | 83.95% | 78.76% | 25.97% |
| [serde-brief 0.1.0][serde-brief] | 9.40% | 23.32% | 4.45% | 25.68% | 53.74% | 62.22% | 18.67% |
| [serde_bare 0.5.0][serde_bare] | 16.97% | 53.82% | † | 91.97% | 94.31% | 92.06% | 30.70% |
| [serde_cbor 0.11.2][serde_cbor] | 6.74% | 25.80% | 4.91% | 29.53% | 58.29% | 66.56% | 19.65% |
| [serde_json 1.0.134][serde_json] | 3.44% | 18.75% | † | 20.19% | 43.07% | 50.81% | 12.52% |
| [simd-json 0.14.3][simd-json] | 5.59% | 27.33% | † | 20.19% | 43.07% | 50.81% | 12.50% |
| [speedy 0.8.7][speedy] | 48.19% | 73.94% | 29.92% | 72.89% | 85.52% | 86.87% | 30.10% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.73%\**</span> | <span title="validated on-demand with error">*49.03%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.77%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*4.5154 ms\**</span> <span title="prepend">*2.5849 ms\**</span> | 8.3828 ms | 1704643 | 1294259 | 1245607 | 11.339 ms |
| [bincode 2.0.0-rc][bincode] | 1.1828 ms | 4.0441 ms | 1406257 | 1117802 | 1062238 | 9.3659 ms |
| [bincode 1.3.3][bincode1] | 3.9069 ms | 4.1993 ms | 1854234 | 1141994 | 1050351 | 10.136 ms |
| [bitcode 0.6.4][bitcode] | 723.85 µs | 2.3341 ms | 971318 | 878034 | 855922 | 3.3702 ms |
| [borsh 1.5.3][borsh] | 2.9286 ms | 2.8630 ms | 1521989 | 1108471 | 1038408 | 9.7511 ms |
| [capnp 0.20.3][capnp] | 2.2130 ms | † | 2724288 | 1546992 | 1240354 | 14.655 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.0635 ms | 18.504 ms | 6012539 | 1695215 | 1467194 | 21.515 ms |
| [ciborium 0.2.2][ciborium] | 22.365 ms | 56.454 ms | 6012373 | 1695146 | 1467435 | 21.491 ms |
| [databuf 0.5.0][databuf] | 1.3063 ms | 3.8033 ms | 1319999 | 1062631 | 1007898 | 8.7308 ms |
| [dlhn 0.1.7][dlhn] | 4.8017 ms | 6.5491 ms | 1311281 | 1077520 | 1045571 | 8.5357 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.1369 ms | † | 2325620 | 1440289 | 1265148 | 13.054 ms |
| [minicbor 0.25.1][minicbor] | 2.5029 ms | 11.182 ms | 1777386 | 1276218 | 1252036 | 12.081 ms |
| [msgpacker 0.4.5][msgpacker] | 2.3152 ms | 6.7999 ms | 1458773 | 1156055 | 1137194 | 9.5273 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.029 ms | 16.708 ms | 1770060 | 1277755 | 1263142 | 12.150 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3016 ms | 2.9434 ms | 1812404 | 1134820 | 1054758 | 10.308 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.1511 ms | 3.1993 ms | 1319999 | 1064380 | 1010284 | 8.9829 ms |
| [postcard 1.1.1][postcard] | 2.0100 ms | 4.2296 ms | 1311281 | 1083900 | 1041114 | 8.5351 ms |
| [pot 3.0.1][pot] | 14.163 ms | 29.703 ms | 2604812 | 1482233 | 1299952 | 15.475 ms |
| [prost 0.13.4][prost] | <span title="encode">*5.4198 ms\**</span> <span title="populate + encode">*10.751 ms\**</span> | 9.1616 ms | 1859886 | 1338076 | 1295497 | 11.927 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*5.4879 ms\**</span> <span title="populate + encode">*12.576 ms\**</span> | 11.907 ms | 1859886 | 1338076 | 1295497 | 12.015 ms |
| [rkyv 0.8.9][rkyv] | 929.06 µs | <span title="unvalidated">*2.1854 ms\**</span> <span title="validated upfront with error">*2.6161 ms\**</span> | 2075936 | 1383779 | 1211892 | 12.738 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.610 ms | 11.035 ms | 1745322 | 1261627 | 1228902 | 11.187 ms |
| [ron 0.8.1][ron] | 37.095 ms | 96.248 ms | 8677703 | 2233642 | 1827843 | 33.948 ms |
| [savefile 0.18.5][savefile] | 846.24 µs | 2.7291 ms | 1791505 | 1128012 | 1052757 | 10.280 ms |
| [serde-brief 0.1.0][serde-brief] | 6.4119 ms | 22.071 ms | 6951772 | 1796265 | 1570903 | 23.327 ms |
| [serde_bare 0.5.0][serde_bare] | 5.4359 ms | 4.8226 ms | 1319999 | 1062645 | 1007918 | 8.7265 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.244 ms | 21.925 ms | 6012373 | 1695146 | 1467435 | 21.241 ms |
| [serde_json 1.0.134][serde_json] | 20.788 ms | 31.659 ms | 9390461 | 2391679 | 1843922 | 34.785 ms |
| [simd-json 0.14.3][simd-json] | 11.569 ms | 25.736 ms | 9390461 | 2391679 | 1843922 | 34.346 ms |
| [speedy 0.8.7][speedy] | 772.93 µs | 2.4656 ms | 1584734 | 1119837 | 1038012 | 9.9897 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.312 ns\**</span> | <span title="validated on-demand with error">*851.27 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4854 ns\**</span> <span title="validated upfront with error">*5.0891 ms\**</span> | <span title="unvalidated">*2.5988 µs\**</span> <span title="validated upfront with error">*5.3314 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*451.46 µs\**</span> | <span title="unvalidated">*409.07 ns\**</span> <span title="validated upfront with error">*451.38 µs\**</span> | <span title="unvalidated">*235.04 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*16.03%\**</span> <span title="prepend">*28.00%\**</span> | 26.07% | 56.98% | 67.84% | 68.72% | 29.72% |
| [bincode 2.0.0-rc][bincode] | 61.20% | 54.04% | 69.07% | 78.55% | 80.58% | 35.98% |
| [bincode 1.3.3][bincode1] | 18.53% | 52.04% | 52.38% | 76.89% | 81.49% | 33.25% |
| [bitcode 0.6.4][bitcode] | 100.00% | 93.63% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.72% | 76.33% | 63.82% | 79.21% | 82.43% | 34.56% |
| [capnp 0.20.3][capnp] | 32.71% | † | 35.65% | 56.76% | 69.01% | 23.00% |
| [cbor4ii 0.3.3][cbor4ii] | 23.63% | 11.81% | 16.15% | 51.79% | 58.34% | 15.66% |
| [ciborium 0.2.2][ciborium] | 3.24% | 3.87% | 16.16% | 51.80% | 58.33% | 15.68% |
| [databuf 0.5.0][databuf] | 55.41% | 57.46% | 73.58% | 82.63% | 84.92% | 38.60% |
| [dlhn 0.1.7][dlhn] | 15.07% | 33.37% | 74.07% | 81.49% | 81.86% | 39.48% |
| [flatbuffers 24.12.23][flatbuffers] | 14.09% | † | 41.77% | 60.96% | 67.65% | 25.82% |
| [minicbor 0.25.1][minicbor] | 28.92% | 19.54% | 54.65% | 68.80% | 68.36% | 27.90% |
| [msgpacker 0.4.5][msgpacker] | 31.27% | 32.14% | 66.58% | 75.95% | 75.27% | 35.37% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.41% | 13.08% | 54.87% | 68.72% | 67.76% | 27.74% |
| [nanoserde 0.1.37][nanoserde] | 55.61% | 74.25% | 53.59% | 77.37% | 81.15% | 32.70% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.97% | 68.31% | 73.58% | 82.49% | 84.72% | 37.52% |
| [postcard 1.1.1][postcard] | 36.01% | 51.67% | 74.07% | 81.01% | 82.21% | 39.49% |
| [pot 3.0.1][pot] | 5.11% | 7.36% | 37.29% | 59.24% | 65.84% | 21.78% |
| [prost 0.13.4][prost] | <span title="encode">*13.36%\**</span> <span title="populate + encode">*6.73%\**</span> | 23.85% | 52.22% | 65.62% | 66.07% | 28.26% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*13.19%\**</span> <span title="populate + encode">*5.76%\**</span> | 18.35% | 52.22% | 65.62% | 66.07% | 28.05% |
| [rkyv 0.8.9][rkyv] | 77.91% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.54%\**</span> | 46.79% | 63.45% | 70.63% | 26.46% |
| [rmp-serde 1.3.0][rmp-serde] | 6.82% | 19.80% | 55.65% | 69.60% | 69.65% | 30.13% |
| [ron 0.8.1][ron] | 1.95% | 2.27% | 11.19% | 39.31% | 46.83% | 9.93% |
| [savefile 0.18.5][savefile] | 85.54% | 80.08% | 54.22% | 77.84% | 81.30% | 32.78% |
| [serde-brief 0.1.0][serde-brief] | 11.29% | 9.90% | 13.97% | 48.88% | 54.49% | 14.45% |
| [serde_bare 0.5.0][serde_bare] | 13.32% | 45.32% | 73.58% | 82.63% | 84.92% | 38.62% |
| [serde_cbor 0.11.2][serde_cbor] | 7.07% | 9.97% | 16.16% | 51.80% | 58.33% | 15.87% |
| [serde_json 1.0.134][serde_json] | 3.48% | 6.90% | 10.34% | 36.71% | 46.42% | 9.69% |
| [simd-json 0.14.3][simd-json] | 6.26% | 8.49% | 10.34% | 36.71% | 46.42% | 9.81% |
| [speedy 0.8.7][speedy] | 93.65% | 88.64% | 61.29% | 78.41% | 82.46% | 33.74% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*48.05%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*15.74%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1012.2
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.4
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
[protobuf]: https://crates.io/crates/protobuf/3.7.1
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
