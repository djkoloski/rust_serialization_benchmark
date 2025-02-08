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

## Last updated: 2025-02-08 16:32:56

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
BogoMIPS:                             4890.84
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
| [bilrost 0.1012.2][bilrost] | <span title="encode">*507.00 µs\**</span> <span title="prepend">*431.82 µs\**</span> | 2.6229 ms | 913.85 µs | 804955 | 328941 | 285485 | 4.5234 ms |
| [bincode 2.0.0-rc][bincode] | 301.54 µs | 2.4894 ms | † | 741295 | 303944 | 257153 | 3.9394 ms |
| [bincode 1.3.3][bincode1] | 521.36 µs | 2.1148 ms | 651.01 µs | 1045784 | 373127 | 311761 | 4.7957 ms |
| [bitcode 0.6.3][bitcode] | 137.23 µs | 1.4437 ms | 60.100 µs | 703710 | 288826 | 229755 | 2.4035 ms |
| [borsh 1.5.3][borsh] | 554.37 µs | 2.1177 ms | † | 885780 | 362204 | 286514 | 4.5449 ms |
| [capnp 0.20.3][capnp] | 498.26 µs | † | † | 1443216 | 513986 | 428649 | 6.3658 ms |
| [cbor4ii 0.3.3][cbor4ii] | 706.14 µs | 5.1648 ms | 3.5460 ms | 1407835 | 403440 | 324081 | 5.0499 ms |
| [ciborium 0.2.2][ciborium] | 4.1721 ms | 12.090 ms | † | 1407835 | 403440 | 324081 | 5.0333 ms |
| [databuf 0.5.0][databuf] | 256.29 µs | 2.0000 ms | 660.31 µs | 765778 | 311715 | 264630 | 4.1131 ms |
| [dlhn 0.1.7][dlhn] | 731.94 µs | 2.5542 ms | † | 724953 | 301446 | 253629 | 3.7985 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0227 ms | † | † | 1276368 | 468539 | 388832 | 5.1492 ms |
| [minicbor 0.25.1][minicbor] | 674.71 µs | 2.9687 ms | 1.3845 ms | 817830 | 332671 | 284548 | 4.2732 ms |
| [msgpacker 0.4.5][msgpacker] | 1.2451 ms | 2.6760 ms | † | 764996 | 315291 | 264898 | 3.9037 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5788 ms | 4.0542 ms | 2.6119 ms | 818669 | 332556 | 285514 | 4.2848 ms |
| [nanoserde 0.1.37][nanoserde] | 255.99 µs | 2.0626 ms | † | 1045784 | 373127 | 311761 | 4.4850 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 712.38 µs | 2.1798 ms | † | 765778 | 311743 | 264518 | 3.8051 ms |
| [postcard 1.1.1][postcard] | 424.55 µs | 2.1889 ms | 649.17 µs | 724953 | 302399 | 253747 | 3.8281 ms |
| [pot 3.0.1][pot] | 2.3657 ms | 6.4752 ms | 4.8584 ms | 971922 | 372513 | 304122 | 4.5828 ms |
| [prost 0.13.4][prost] | <span title="encode">*970.24 µs\**</span> <span title="populate + encode">*2.4647 ms\**</span> | 3.2826 ms | † | 884628 | 363130 | 315494 | 4.8418 ms |
| [rkyv 0.8.9][rkyv] | 255.51 µs | <span title="unvalidated">*1.5664 ms\**</span> <span title="validated upfront with error">*2.0085 ms\**</span> | † | 1011488 | 393526 | 326517 | 4.8546 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4330 ms | 3.1692 ms | 1.3878 ms | 784997 | 325384 | 278219 | 4.0875 ms |
| [ron 0.8.1][ron] | 11.213 ms | 16.749 ms | 13.901 ms | 1607459 | 449158 | 349713 | 5.6294 ms |
| [savefile 0.18.5][savefile] | 193.44 µs | 2.1511 ms | † | 1045800 | 373139 | 311761 | 4.4951 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5313 ms | 5.0410 ms | 3.0739 ms | 1584946 | 413733 | 341439 | 4.8973 ms |
| [serde_bare 0.5.0][serde_bare] | 673.50 µs | 2.0884 ms | † | 765778 | 311715 | 264630 | 3.8172 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.1274 ms | 4.9707 ms | 3.2404 ms | 1407835 | 403440 | 324081 | 4.9886 ms |
| [serde_json 1.0.134][serde_json] | 3.7039 ms | 6.0440 ms | † | 1827461 | 470560 | 361090 | 5.5554 ms |
| [simd-json 0.14.3][simd-json] | 2.0815 ms | 4.6967 ms | † | 1827461 | 470560 | 361090 | 6.0026 ms |
| [speedy 0.8.7][speedy] | 194.01 µs | 1.7291 ms | 375.39 µs | 885780 | 362204 | 286514 | 4.2257 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.342 ns\**</span> | <span title="validated on-demand with error">*169.18 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4886 ns\**</span> <span title="validated upfront with error">*1.9727 ms\**</span> | <span title="unvalidated">*51.748 µs\**</span> <span title="validated upfront with error">*2.0329 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2435 ns\**</span> <span title="validated upfront with error">*445.63 µs\**</span> | <span title="unvalidated">*10.361 µs\**</span> <span title="validated upfront with error">*456.57 µs\**</span> | <span title="unvalidated">*7.4986 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*27.07%\**</span> <span title="prepend">*31.78%\**</span> | 55.04% | 6.58% | 87.42% | 87.80% | 80.48% | 53.13% |
| [bincode 2.0.0-rc][bincode] | 45.51% | 57.99% | † | 94.93% | 95.03% | 89.35% | 61.01% |
| [bincode 1.3.3][bincode1] | 26.32% | 68.27% | 9.23% | 67.29% | 77.41% | 73.70% | 50.12% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.75% | 68.17% | † | 79.45% | 79.74% | 80.19% | 52.88% |
| [capnp 0.20.3][capnp] | 27.54% | † | † | 48.76% | 56.19% | 53.60% | 37.76% |
| [cbor4ii 0.3.3][cbor4ii] | 19.43% | 27.95% | 1.69% | 49.99% | 71.59% | 70.89% | 47.60% |
| [ciborium 0.2.2][ciborium] | 3.29% | 11.94% | † | 49.99% | 71.59% | 70.89% | 47.75% |
| [databuf 0.5.0][databuf] | 53.54% | 72.19% | 9.10% | 91.89% | 92.66% | 86.82% | 58.44% |
| [dlhn 0.1.7][dlhn] | 18.75% | 56.52% | † | 97.07% | 95.81% | 90.59% | 63.27% |
| [flatbuffers 24.12.23][flatbuffers] | 13.42% | † | † | 55.13% | 61.64% | 59.09% | 46.68% |
| [minicbor 0.25.1][minicbor] | 20.34% | 48.63% | 4.34% | 86.05% | 86.82% | 80.74% | 56.25% |
| [msgpacker 0.4.5][msgpacker] | 11.02% | 53.95% | † | 91.99% | 91.61% | 86.73% | 61.57% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.46% | 35.61% | 2.30% | 85.96% | 86.85% | 80.47% | 56.09% |
| [nanoserde 0.1.37][nanoserde] | 53.61% | 69.99% | † | 67.29% | 77.41% | 73.70% | 53.59% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 19.26% | 66.23% | † | 91.89% | 92.65% | 86.86% | 63.17% |
| [postcard 1.1.1][postcard] | 32.32% | 65.96% | 9.26% | 97.07% | 95.51% | 90.54% | 62.79% |
| [pot 3.0.1][pot] | 5.80% | 22.30% | 1.24% | 72.40% | 77.53% | 75.55% | 52.45% |
| [prost 0.13.4][prost] | <span title="encode">*14.14%\**</span> <span title="populate + encode">*5.57%\**</span> | 43.98% | † | 79.55% | 79.54% | 72.82% | 49.64% |
| [rkyv 0.8.9][rkyv] | 53.71% | <span title="unvalidated">*92.17%\**</span> <span title="validated upfront with error">*71.88%\**</span> | † | 69.57% | 73.39% | 70.37% | 49.51% |
| [rmp-serde 1.3.0][rmp-serde] | 9.58% | 45.55% | 4.33% | 89.64% | 88.76% | 82.58% | 58.80% |
| [ron 0.8.1][ron] | 1.22% | 8.62% | 0.43% | 43.78% | 64.30% | 65.70% | 42.70% |
| [savefile 0.18.5][savefile] | 70.94% | 67.11% | † | 67.29% | 77.40% | 73.70% | 53.47% |
| [serde-brief 0.1.0][serde-brief] | 8.96% | 28.64% | 1.96% | 44.40% | 69.81% | 67.29% | 49.08% |
| [serde_bare 0.5.0][serde_bare] | 20.38% | 69.13% | † | 91.89% | 92.66% | 86.82% | 62.97% |
| [serde_cbor 0.11.2][serde_cbor] | 6.45% | 29.04% | 1.85% | 49.99% | 71.59% | 70.89% | 48.18% |
| [serde_json 1.0.134][serde_json] | 3.71% | 23.89% | † | 38.51% | 61.38% | 63.63% | 43.26% |
| [simd-json 0.14.3][simd-json] | 6.59% | 30.74% | † | 38.51% | 61.38% | 63.63% | 40.04% |
| [speedy 0.8.7][speedy] | 70.73% | 83.49% | 16.01% | 79.45% | 79.74% | 80.19% | 56.88% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*6.12%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.02%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.27%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*6.6822 ms\**</span> <span title="prepend">*8.8271 ms\**</span> | 8.3125 ms | 8625005 | 6443961 | 6231572 | 71.076 ms |
| [bincode 2.0.0-rc][bincode] | 2.8815 ms | 1.0205 ms | 6000005 | 5378497 | 5345897 | 7.4127 ms |
| [bincode 1.3.3][bincode1] | 5.1881 ms | 4.2670 ms | 6000008 | 5378500 | 5345890 | 7.3644 ms |
| [bitcode 0.6.3][bitcode] | 1.3973 ms | 797.76 µs | 6000006 | 5182295 | 4923880 | 13.037 ms |
| [borsh 1.5.3][borsh] | 6.2421 ms | 4.3495 ms | 6000004 | 5378496 | 5345889 | 7.5680 ms |
| [capnp 0.20.3][capnp] | 5.5188 ms | † | 14000088 | 7130367 | 6051062 | 79.232 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.8646 ms | 49.111 ms | 13125016 | 7524114 | 6757967 | 92.136 ms |
| [ciborium 0.2.2][ciborium] | 71.644 ms | 117.04 ms | 13122324 | 7524660 | 6759658 | 89.437 ms |
| [databuf 0.5.0][databuf] | 2.4101 ms | 5.3520 ms | 6000003 | 5378495 | 5345900 | 7.5973 ms |
| [dlhn 0.1.7][dlhn] | 6.2032 ms | 6.8224 ms | 6000003 | 5378495 | 5345900 | 7.5770 ms |
| [flatbuffers 24.12.23][flatbuffers] | 857.58 µs | † | 6000024 | 5378434 | 5345910 | 7.3083 ms |
| [minicbor 0.25.1][minicbor] | 5.1859 ms | 11.377 ms | 8125006 | 6494907 | 6390894 | 68.315 ms |
| [msgpacker 0.4.5][msgpacker] | 18.475 ms | 5.2212 ms | 7500005 | 6058442 | 6014337 | 9.5315 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 131.24 ms | 32.630 ms | 8125037 | 6493484 | 6386940 | 68.744 ms |
| [nanoserde 0.1.37][nanoserde] | 1.7206 ms | 1.1041 ms | 6000008 | 5378500 | 5345890 | 7.2903 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1498 ms | 4.5583 ms | 6000004 | 5378496 | 5345889 | 7.3718 ms |
| [postcard 1.1.1][postcard] | 493.60 µs | 1.3096 ms | 6000003 | 5378495 | 5345900 | 7.4893 ms |
| [pot 3.0.1][pot] | 36.701 ms | 72.146 ms | 10122342 | 6814618 | 6852251 | 79.944 ms |
| [prost 0.13.4][prost] | <span title="encode">*8.0778 ms\**</span> <span title="populate + encode">*8.7568 ms\**</span> | 15.521 ms | 8750000 | 6665735 | 6421871 | 70.922 ms |
| [rkyv 0.8.9][rkyv] | 147.81 µs | <span title="unvalidated">*148.59 µs\**</span> <span title="validated upfront with error">*198.18 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.3101 ms |
| [rmp-serde 1.3.0][rmp-serde] | 18.964 ms | 18.911 ms | 8125006 | 6494876 | 6391037 | 69.438 ms |
| [ron 0.8.1][ron] | 169.93 ms | 242.00 ms | 22192885 | 8970395 | 8138755 | 146.44 ms |
| [savefile 0.18.5][savefile] | 148.11 µs | 198.70 µs | 6000024 | 5378519 | 5345892 | 7.3584 ms |
| [serde-brief 0.1.0][serde-brief] | 23.153 ms | 41.364 ms | 15750015 | 8024540 | 6816643 | 95.692 ms |
| [serde_bare 0.5.0][serde_bare] | 5.1075 ms | 4.7112 ms | 6000003 | 5378495 | 5345900 | 7.3561 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.700 ms | 47.874 ms | 13122324 | 7524660 | 6759658 | 89.395 ms |
| [serde_json 1.0.134][serde_json] | 86.599 ms | 91.498 ms | 26192883 | 9566084 | 8586741 | 153.46 ms |
| [simd-json 0.14.3][simd-json] | 50.967 ms | 68.525 ms | 26192883 | 9566084 | 8586741 | 152.32 ms |
| [speedy 0.8.7][speedy] | 180.02 µs | 148.27 µs | 6000004 | 5378496 | 5345889 | 7.3673 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*102.51 ns\**</span> | <span title="validated on-demand with error">*2.2016 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4866 ns\**</span> <span title="validated upfront with error">*39.869 ns\**</span> | <span title="unvalidated">*54.287 µs\**</span> <span title="validated upfront with error">*77.880 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2434 ns\**</span> <span title="validated upfront with error">*5.2928 ns\**</span> | <span title="unvalidated">*48.647 µs\**</span> <span title="validated upfront with error">*77.804 µs\**</span> | <span title="unvalidated">*76.752 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*2.21%\**</span> <span title="prepend">*1.67%\**</span> | 1.78% | 69.57% | 80.42% | 79.02% | 10.26% |
| [bincode 2.0.0-rc][bincode] | 5.13% | 14.53% | 100.00% | 96.35% | 92.11% | 98.35% |
| [bincode 1.3.3][bincode1] | 2.85% | 3.47% | 100.00% | 96.35% | 92.11% | 98.99% |
| [bitcode 0.6.3][bitcode] | 10.58% | 18.59% | 100.00% | 100.00% | 100.00% | 55.92% |
| [borsh 1.5.3][borsh] | 2.37% | 3.41% | 100.00% | 96.35% | 92.11% | 96.33% |
| [capnp 0.20.3][capnp] | 2.68% | † | 42.86% | 72.68% | 81.37% | 9.20% |
| [cbor4ii 0.3.3][cbor4ii] | 1.50% | 0.30% | 45.71% | 68.88% | 72.86% | 7.91% |
| [ciborium 0.2.2][ciborium] | 0.21% | 0.13% | 45.72% | 68.87% | 72.84% | 8.15% |
| [databuf 0.5.0][databuf] | 6.13% | 2.77% | 100.00% | 96.35% | 92.11% | 95.96% |
| [dlhn 0.1.7][dlhn] | 2.38% | 2.17% | 100.00% | 96.35% | 92.11% | 96.22% |
| [flatbuffers 24.12.23][flatbuffers] | 17.24% | † | 100.00% | 96.35% | 92.11% | 99.75% |
| [minicbor 0.25.1][minicbor] | 2.85% | 1.30% | 73.85% | 79.79% | 77.05% | 10.67% |
| [msgpacker 0.4.5][msgpacker] | 0.80% | 2.84% | 80.00% | 85.54% | 81.87% | 76.49% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.11% | 0.45% | 73.85% | 79.81% | 77.09% | 10.60% |
| [nanoserde 0.1.37][nanoserde] | 8.59% | 13.43% | 100.00% | 96.35% | 92.11% | 100.00% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.87% | 3.25% | 100.00% | 96.35% | 92.11% | 98.89% |
| [postcard 1.1.1][postcard] | 29.95% | 11.32% | 100.00% | 96.35% | 92.11% | 97.34% |
| [pot 3.0.1][pot] | 0.40% | 0.21% | 59.27% | 76.05% | 71.86% | 9.12% |
| [prost 0.13.4][prost] | <span title="encode">*1.83%\**</span> <span title="populate + encode">*1.69%\**</span> | 0.96% | 68.57% | 77.75% | 76.67% | 10.28% |
| [rkyv 0.8.9][rkyv] | 100.00% | <span title="unvalidated">*99.78%\**</span> <span title="validated upfront with error">*74.82%\**</span> | 100.00% | 96.35% | 92.11% | 99.73% |
| [rmp-serde 1.3.0][rmp-serde] | 0.78% | 0.78% | 73.85% | 79.79% | 77.04% | 10.50% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 4.98% |
| [savefile 0.18.5][savefile] | 99.80% | 74.62% | 100.00% | 96.35% | 92.11% | 99.07% |
| [serde-brief 0.1.0][serde-brief] | 0.64% | 0.36% | 38.10% | 64.58% | 72.23% | 7.62% |
| [serde_bare 0.5.0][serde_bare] | 2.89% | 3.15% | 100.00% | 96.35% | 92.11% | 99.11% |
| [serde_cbor 0.11.2][serde_cbor] | 0.41% | 0.31% | 45.72% | 68.87% | 72.84% | 8.16% |
| [serde_json 1.0.134][serde_json] | 0.17% | 0.16% | 22.91% | 54.17% | 57.34% | 4.75% |
| [simd-json 0.14.3][simd-json] | 0.29% | 0.22% | 22.91% | 54.17% | 57.34% | 4.79% |
| [speedy 0.8.7][speedy] | 82.11% | 100.00% | 100.00% | 96.35% | 92.11% | 98.95% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.21%\**</span> | <span title="validated on-demand with error">*2.21%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*3.12%\**</span> | <span title="unvalidated">*89.61%\**</span> <span title="validated upfront with error">*62.46%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*23.49%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*62.53%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*944.67 µs\**</span> <span title="prepend">*854.59 µs\**</span> | 3.2193 ms | 1.7195 ms | 489348 | 281173 | 249546 | 3.0932 ms |
| [bincode 2.0.0-rc][bincode] | 279.21 µs | 2.0733 ms | † | 367413 | 221291 | 206273 | 2.4945 ms |
| [bincode 1.3.3][bincode1] | 594.61 µs | 1.7908 ms | 843.89 µs | 569975 | 240525 | 232423 | 2.8636 ms |
| [bitcode 0.6.3][bitcode] | 130.98 µs | 1.2417 ms | 169.20 µs | 327688 | 200947 | 182736 | 753.33 µs |
| [borsh 1.5.3][borsh] | 557.47 µs | 1.8012 ms | † | 446595 | 234236 | 210008 | 2.4725 ms |
| [capnp 0.20.3][capnp] | 452.61 µs | † | † | 803896 | 335606 | 280851 | 3.8683 ms |
| [cbor4ii 0.3.3][cbor4ii] | 818.82 µs | 4.7284 ms | 3.5834 ms | 1109831 | 344745 | 274514 | 3.7988 ms |
| [ciborium 0.2.2][ciborium] | 3.8172 ms | 10.011 ms | † | 1109821 | 344751 | 274526 | 3.8821 ms |
| [databuf 0.5.0][databuf] | 311.13 µs | 1.7370 ms | 784.48 µs | 356311 | 213062 | 198488 | 2.3804 ms |
| [dlhn 0.1.7][dlhn] | 768.40 µs | 2.6367 ms | † | 366496 | 220600 | 205683 | 2.4563 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.1924 ms | † | † | 844168 | 345696 | 294015 | 3.8105 ms |
| [minicbor 0.25.1][minicbor] | 542.35 µs | 3.3818 ms | 1.8988 ms | 428773 | 249857 | 228741 | 2.7248 ms |
| [msgpacker 0.4.5][msgpacker] | 970.57 µs | 3.0879 ms | † | 391251 | 236877 | 220476 | 2.6334 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1394 ms | 4.1092 ms | 2.9991 ms | 449745 | 252432 | 231110 | 2.7588 ms |
| [nanoserde 0.1.37][nanoserde] | 267.05 µs | 1.9445 ms | † | 567975 | 239930 | 232419 | 2.8450 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 614.62 µs | 1.9810 ms | † | 356311 | 212976 | 198524 | 2.3739 ms |
| [postcard 1.1.1][postcard] | 448.65 µs | 1.9908 ms | 830.35 µs | 367489 | 221913 | 207344 | 2.5214 ms |
| [pot 3.0.1][pot] | 2.3842 ms | 6.1155 ms | 4.9765 ms | 599125 | 299158 | 247693 | 3.1263 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.2363 ms\**</span> <span title="populate + encode">*2.9333 ms\**</span> | 3.4826 ms | † | 596811 | 305319 | 269310 | 3.4308 ms |
| [rkyv 0.8.9][rkyv] | 335.78 µs | <span title="unvalidated">*1.4930 ms\**</span> <span title="validated upfront with error">*1.8746 ms\**</span> | † | 603776 | 254776 | 220087 | 2.7311 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5853 ms | 3.0387 ms | 1.7343 ms | 424533 | 245214 | 226188 | 2.6912 ms |
| [ron 0.8.1][ron] | 7.3306 ms | 17.235 ms | 15.451 ms | 1465223 | 434935 | 343338 | 5.8076 ms |
| [savefile 0.18.5][savefile] | 208.00 µs | 1.8049 ms | † | 566991 | 239362 | 232010 | 2.8520 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3392 ms | 5.4935 ms | 3.7806 ms | 1276014 | 373898 | 293679 | 4.0368 ms |
| [serde_bare 0.5.0][serde_bare] | 740.46 µs | 2.3347 ms | † | 356311 | 213062 | 198488 | 2.4041 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8659 ms | 4.8371 ms | 3.5171 ms | 1109821 | 344751 | 274526 | 3.8119 ms |
| [serde_json 1.0.134][serde_json] | 3.6204 ms | 6.8746 ms | † | 1623191 | 466527 | 359623 | 5.9913 ms |
| [simd-json 0.14.3][simd-json] | 2.2204 ms | 4.5843 ms | † | 1623191 | 466527 | 359623 | 5.9659 ms |
| [speedy 0.8.7][speedy] | 256.38 µs | 1.6201 ms | 572.51 µs | 449595 | 234970 | 210361 | 2.4936 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*72.525 ns\**</span> | <span title="validated on-demand with error">*455.03 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4873 ns\**</span> <span title="validated upfront with error">*2.1922 ms\**</span> | <span title="unvalidated">*1.3571 µs\**</span> <span title="validated upfront with error">*2.2182 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2435 ns\**</span> <span title="validated upfront with error">*372.71 µs\**</span> | <span title="unvalidated">*163.81 ns\**</span> <span title="validated upfront with error">*372.34 µs\**</span> | <span title="unvalidated">*761.51 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*13.87%\**</span> <span title="prepend">*15.33%\**</span> | 38.57% | 9.84% | 66.96% | 71.47% | 73.23% | 24.35% |
| [bincode 2.0.0-rc][bincode] | 46.91% | 59.89% | † | 89.19% | 90.81% | 88.59% | 30.20% |
| [bincode 1.3.3][bincode1] | 22.03% | 69.34% | 20.05% | 57.49% | 83.55% | 78.62% | 26.31% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 23.50% | 68.94% | † | 73.37% | 85.79% | 87.01% | 30.47% |
| [capnp 0.20.3][capnp] | 28.94% | † | † | 40.76% | 59.88% | 65.07% | 19.47% |
| [cbor4ii 0.3.3][cbor4ii] | 16.00% | 26.26% | 4.72% | 29.53% | 58.29% | 66.57% | 19.83% |
| [ciborium 0.2.2][ciborium] | 3.43% | 12.40% | † | 29.53% | 58.29% | 66.56% | 19.41% |
| [databuf 0.5.0][databuf] | 42.10% | 71.49% | 21.57% | 91.97% | 94.31% | 92.06% | 31.65% |
| [dlhn 0.1.7][dlhn] | 17.05% | 47.09% | † | 89.41% | 91.09% | 88.84% | 30.67% |
| [flatbuffers 24.12.23][flatbuffers] | 4.10% | † | † | 38.82% | 58.13% | 62.15% | 19.77% |
| [minicbor 0.25.1][minicbor] | 24.15% | 36.72% | 8.91% | 76.42% | 80.42% | 79.89% | 27.65% |
| [msgpacker 0.4.5][msgpacker] | 13.50% | 40.21% | † | 83.75% | 84.83% | 82.88% | 28.61% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.55% | 30.22% | 5.64% | 72.86% | 79.60% | 79.07% | 27.31% |
| [nanoserde 0.1.37][nanoserde] | 49.05% | 63.86% | † | 57.69% | 83.75% | 78.62% | 26.48% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.31% | 62.68% | † | 91.97% | 94.35% | 92.05% | 31.73% |
| [postcard 1.1.1][postcard] | 29.19% | 62.37% | 20.38% | 89.17% | 90.55% | 88.13% | 29.88% |
| [pot 3.0.1][pot] | 5.49% | 20.30% | 3.40% | 54.69% | 67.17% | 73.78% | 24.10% |
| [prost 0.13.4][prost] | <span title="encode">*10.59%\**</span> <span title="populate + encode">*4.47%\**</span> | 35.65% | † | 54.91% | 65.82% | 67.85% | 21.96% |
| [rkyv 0.8.9][rkyv] | 39.01% | <span title="unvalidated">*83.17%\**</span> <span title="validated upfront with error">*66.24%\**</span> | † | 54.27% | 78.87% | 83.03% | 27.58% |
| [rmp-serde 1.3.0][rmp-serde] | 8.26% | 40.86% | 9.76% | 77.19% | 81.95% | 80.79% | 27.99% |
| [ron 0.8.1][ron] | 1.79% | 7.20% | 1.10% | 22.36% | 46.20% | 53.22% | 12.97% |
| [savefile 0.18.5][savefile] | 62.97% | 68.80% | † | 57.79% | 83.95% | 78.76% | 26.41% |
| [serde-brief 0.1.0][serde-brief] | 9.78% | 22.60% | 4.48% | 25.68% | 53.74% | 62.22% | 18.66% |
| [serde_bare 0.5.0][serde_bare] | 17.69% | 53.18% | † | 91.97% | 94.31% | 92.06% | 31.34% |
| [serde_cbor 0.11.2][serde_cbor] | 7.02% | 25.67% | 4.81% | 29.53% | 58.29% | 66.56% | 19.76% |
| [serde_json 1.0.134][serde_json] | 3.62% | 18.06% | † | 20.19% | 43.07% | 50.81% | 12.57% |
| [simd-json 0.14.3][simd-json] | 5.90% | 27.09% | † | 20.19% | 43.07% | 50.81% | 12.63% |
| [speedy 0.8.7][speedy] | 51.09% | 76.64% | 29.55% | 72.89% | 85.52% | 86.87% | 30.21% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*36.00%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.07%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.04%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*4.5655 ms\**</span> <span title="prepend">*2.5618 ms\**</span> | 8.3663 ms | 1704643 | 1294259 | 1245607 | 11.400 ms |
| [bincode 2.0.0-rc][bincode] | 1.1873 ms | 4.1461 ms | 1406257 | 1117802 | 1062238 | 9.3491 ms |
| [bincode 1.3.3][bincode1] | 4.0207 ms | 4.1450 ms | 1854234 | 1141994 | 1050351 | 10.134 ms |
| [bitcode 0.6.3][bitcode] | 719.09 µs | 2.3138 ms | 971318 | 878034 | 855922 | 3.3461 ms |
| [borsh 1.5.3][borsh] | 2.7805 ms | 2.8421 ms | 1521989 | 1108471 | 1038408 | 9.7360 ms |
| [capnp 0.20.3][capnp] | 2.1841 ms | † | 2724288 | 1546992 | 1240354 | 14.406 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.2974 ms | 18.508 ms | 6012539 | 1695215 | 1467194 | 21.423 ms |
| [ciborium 0.2.2][ciborium] | 22.835 ms | 54.302 ms | 6012373 | 1695146 | 1467435 | 21.250 ms |
| [databuf 0.5.0][databuf] | 1.2928 ms | 3.7473 ms | 1319999 | 1062631 | 1007898 | 8.7352 ms |
| [dlhn 0.1.7][dlhn] | 4.9003 ms | 6.7293 ms | 1311281 | 1077520 | 1045571 | 8.5437 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.1784 ms | † | 2325620 | 1440289 | 1265148 | 13.090 ms |
| [minicbor 0.25.1][minicbor] | 2.1802 ms | 11.307 ms | 1777386 | 1276218 | 1252036 | 12.266 ms |
| [msgpacker 0.4.5][msgpacker] | 2.3213 ms | 6.9928 ms | 1458773 | 1156055 | 1137194 | 9.7009 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.973 ms | 18.246 ms | 1770060 | 1277755 | 1263142 | 12.094 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2774 ms | 2.9284 ms | 1812404 | 1134820 | 1054758 | 10.124 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.9446 ms | 3.3065 ms | 1319999 | 1064380 | 1010284 | 8.8897 ms |
| [postcard 1.1.1][postcard] | 1.9797 ms | 4.2254 ms | 1311281 | 1083900 | 1041114 | 8.5484 ms |
| [pot 3.0.1][pot] | 13.363 ms | 29.823 ms | 2604812 | 1482233 | 1299952 | 15.437 ms |
| [prost 0.13.4][prost] | <span title="encode">*5.4292 ms\**</span> <span title="populate + encode">*9.2774 ms\**</span> | 8.9147 ms | 1859886 | 1338076 | 1295497 | 11.988 ms |
| [rkyv 0.8.9][rkyv] | 1.0213 ms | <span title="unvalidated">*2.1802 ms\**</span> <span title="validated upfront with error">*2.6084 ms\**</span> | 2075936 | 1383779 | 1211892 | 12.818 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.470 ms | 11.222 ms | 1745322 | 1261627 | 1228902 | 11.283 ms |
| [ron 0.8.1][ron] | 38.189 ms | 89.497 ms | 8677703 | 2233642 | 1827843 | 34.219 ms |
| [savefile 0.18.5][savefile] | 837.58 µs | 2.7762 ms | 1791505 | 1128012 | 1052757 | 10.155 ms |
| [serde-brief 0.1.0][serde-brief] | 6.7143 ms | 22.783 ms | 6951772 | 1796265 | 1570903 | 23.393 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7973 ms | 4.7895 ms | 1319999 | 1062645 | 1007918 | 8.8004 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.548 ms | 21.191 ms | 6012373 | 1695146 | 1467435 | 21.363 ms |
| [serde_json 1.0.134][serde_json] | 20.432 ms | 30.651 ms | 9390461 | 2391679 | 1843922 | 35.552 ms |
| [simd-json 0.14.3][simd-json] | 11.582 ms | 25.662 ms | 9390461 | 2391679 | 1843922 | 34.144 ms |
| [speedy 0.8.7][speedy] | 774.86 µs | 2.4788 ms | 1584734 | 1119837 | 1038012 | 9.9005 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*72.584 ns\**</span> | <span title="validated on-demand with error">*713.78 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4877 ns\**</span> <span title="validated upfront with error">*5.0844 ms\**</span> | <span title="unvalidated">*2.6182 µs\**</span> <span title="validated upfront with error">*5.1682 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2427 ns\**</span> <span title="validated upfront with error">*432.24 µs\**</span> | <span title="unvalidated">*439.58 ns\**</span> <span title="validated upfront with error">*432.86 µs\**</span> | <span title="unvalidated">*235.45 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*15.75%\**</span> <span title="prepend">*28.07%\**</span> | 26.06% | 56.98% | 67.84% | 68.72% | 29.35% |
| [bincode 2.0.0-rc][bincode] | 60.57% | 52.58% | 69.07% | 78.55% | 80.58% | 35.79% |
| [bincode 1.3.3][bincode1] | 17.88% | 52.60% | 52.38% | 76.89% | 81.49% | 33.02% |
| [bitcode 0.6.3][bitcode] | 100.00% | 94.23% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 25.86% | 76.71% | 63.82% | 79.21% | 82.43% | 34.37% |
| [capnp 0.20.3][capnp] | 32.92% | † | 35.65% | 56.76% | 69.01% | 23.23% |
| [cbor4ii 0.3.3][cbor4ii] | 21.81% | 11.78% | 16.15% | 51.79% | 58.34% | 15.62% |
| [ciborium 0.2.2][ciborium] | 3.15% | 4.01% | 16.16% | 51.80% | 58.33% | 15.75% |
| [databuf 0.5.0][databuf] | 55.62% | 58.18% | 73.58% | 82.63% | 84.92% | 38.31% |
| [dlhn 0.1.7][dlhn] | 14.67% | 32.40% | 74.07% | 81.49% | 81.86% | 39.16% |
| [flatbuffers 24.12.23][flatbuffers] | 13.89% | † | 41.77% | 60.96% | 67.65% | 25.56% |
| [minicbor 0.25.1][minicbor] | 32.98% | 19.28% | 54.65% | 68.80% | 68.36% | 27.28% |
| [msgpacker 0.4.5][msgpacker] | 30.98% | 31.18% | 66.58% | 75.95% | 75.27% | 34.49% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.40% | 11.95% | 54.87% | 68.72% | 67.76% | 27.67% |
| [nanoserde 0.1.37][nanoserde] | 56.29% | 74.45% | 53.59% | 77.37% | 81.15% | 33.05% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 24.42% | 65.94% | 73.58% | 82.49% | 84.72% | 37.64% |
| [postcard 1.1.1][postcard] | 36.32% | 51.60% | 74.07% | 81.01% | 82.21% | 39.14% |
| [pot 3.0.1][pot] | 5.38% | 7.31% | 37.29% | 59.24% | 65.84% | 21.68% |
| [prost 0.13.4][prost] | <span title="encode">*13.24%\**</span> <span title="populate + encode">*7.75%\**</span> | 24.46% | 52.22% | 65.62% | 66.07% | 27.91% |
| [rkyv 0.8.9][rkyv] | 70.41% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.58%\**</span> | 46.79% | 63.45% | 70.63% | 26.10% |
| [rmp-serde 1.3.0][rmp-serde] | 6.87% | 19.43% | 55.65% | 69.60% | 69.65% | 29.66% |
| [ron 0.8.1][ron] | 1.88% | 2.44% | 11.19% | 39.31% | 46.83% | 9.78% |
| [savefile 0.18.5][savefile] | 85.85% | 78.53% | 54.22% | 77.84% | 81.30% | 32.95% |
| [serde-brief 0.1.0][serde-brief] | 10.71% | 9.57% | 13.97% | 48.88% | 54.49% | 14.30% |
| [serde_bare 0.5.0][serde_bare] | 14.99% | 45.52% | 73.58% | 82.63% | 84.92% | 38.02% |
| [serde_cbor 0.11.2][serde_cbor] | 6.82% | 10.29% | 16.16% | 51.80% | 58.33% | 15.66% |
| [serde_json 1.0.134][serde_json] | 3.52% | 7.11% | 10.34% | 36.71% | 46.42% | 9.41% |
| [simd-json 0.14.3][simd-json] | 6.21% | 8.50% | 10.34% | 36.71% | 46.42% | 9.80% |
| [speedy 0.8.7][speedy] | 92.80% | 87.95% | 61.29% | 78.41% | 82.46% | 33.80% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*61.58%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.79%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
