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

## Last updated: 2025-1-22 13:58:12

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.86.0-nightly (ed43cbcb8 2025-01-21)
binary: rustc
commit-hash: ed43cbcb882e7c06870abdd9305dc1f17eb9bab9
commit-date: 2025-01-21
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
BogoMIPS:                             4890.86
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

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*501.78 µs\**</span> <span title="prepend">*434.43 µs\**</span> | 2.7714 ms | 804955 | 328941 | 285485 | 4.4652 ms |
| [bincode 2.0.0-rc][bincode] | 273.24 µs | 2.4420 ms | 741295 | 303944 | 257153 | 4.0225 ms |
| [bincode 1.3.3][bincode1] | 525.73 µs | 2.0229 ms | 1045784 | 373127 | 311761 | 4.8371 ms |
| [bitcode 0.6.3][bitcode] | 139.83 µs | 1.4519 ms | 703710 | 288826 | 229755 | 2.3766 ms |
| [borsh 1.5.3][borsh] | 566.49 µs | 2.2026 ms | 885780 | 362204 | 286514 | 4.5606 ms |
| [capnp 0.20.3][capnp] | 520.78 µs | † | 1443216 | 513986 | 428649 | 6.3476 ms |
| [cbor4ii 0.3.3][cbor4ii] | 607.89 µs | 4.9559 ms | 1407835 | 403440 | 324081 | 4.8043 ms |
| [ciborium 0.2.2][ciborium] | 4.1861 ms | 11.614 ms | 1407835 | 403440 | 324081 | 4.8125 ms |
| [databuf 0.5.0][databuf] | 258.97 µs | 2.0214 ms | 765778 | 311715 | 264630 | 3.8492 ms |
| [dlhn 0.1.7][dlhn] | 727.66 µs | 2.4795 ms | 724953 | 301446 | 253629 | 3.5524 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0333 ms | † | 1276368 | 468539 | 388832 | 5.3277 ms |
| [msgpacker 0.4.5][msgpacker] | 1.2763 ms | 2.6741 ms | 764996 | 315291 | 264898 | 3.9589 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5064 ms | 4.1470 ms | 818669 | 332556 | 285514 | 4.5804 ms |
| [nanoserde 0.1.37][nanoserde] | 234.55 µs | 2.1152 ms | 1045784 | 373127 | 311761 | 4.5700 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 657.18 µs | 2.2198 ms | 765778 | 311743 | 264518 | 4.0863 ms |
| [postcard 1.1.1][postcard] | 427.95 µs | 2.2838 ms | 724953 | 302399 | 253747 | 3.7990 ms |
| [pot 3.0.1][pot] | 2.2760 ms | 6.5624 ms | 971922 | 372513 | 304122 | 4.5223 ms |
| [prost 0.13.4][prost] | <span title="encode">*932.76 µs\**</span> <span title="populate + encode">*2.4093 ms\**</span> | 3.4041 ms | 884628 | 363130 | 315494 | 5.0684 ms |
| [rkyv 0.8.9][rkyv] | 252.73 µs | <span title="unvalidated">*1.5777 ms\**</span> <span title="validated upfront with error">*2.1879 ms\**</span> | 1011488 | 393526 | 326517 | 4.9528 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4735 ms | 3.2757 ms | 784997 | 325384 | 278219 | 4.0895 ms |
| [ron 0.8.1][ron] | 11.565 ms | 16.246 ms | 1607459 | 449158 | 349713 | 5.6106 ms |
| [savefile 0.18.5][savefile] | 191.36 µs | 2.1500 ms | 1045800 | 373139 | 311761 | 4.4893 ms |
| [serde-brief 0.1.0][serde-brief] | 1.6044 ms | 4.8976 ms | 1584946 | 413733 | 341439 | 4.9109 ms |
| [serde_bare 0.5.0][serde_bare] | 691.92 µs | 2.0713 ms | 765778 | 311715 | 264630 | 3.8799 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9975 ms | 5.1902 ms | 1407835 | 403440 | 324081 | 4.7748 ms |
| [serde_json 1.0.134][serde_json] | 3.6875 ms | 6.2971 ms | 1827461 | 470560 | 361090 | 5.6183 ms |
| [simd-json 0.14.3][simd-json] | 2.0765 ms | 4.7315 ms | 1827461 | 470560 | 361090 | 5.5418 ms |
| [speedy 0.8.7][speedy] | 199.66 µs | 1.7373 ms | 885780 | 362204 | 286514 | 4.3411 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*74.370 ns\**</span> | <span title="validated on-demand with error">*166.27 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4899 ns\**</span> <span title="validated upfront with error">*2.1506 ms\**</span> | <span title="unvalidated">*52.157 µs\**</span> <span title="validated upfront with error">*2.1999 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*644.71 µs\**</span> | <span title="unvalidated">*10.767 µs\**</span> <span title="validated upfront with error">*630.66 µs\**</span> | <span title="unvalidated">*7.5588 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*27.87%\**</span> <span title="prepend">*32.19%\**</span> | 52.39% | 87.42% | 87.80% | 80.48% | 53.22% |
| [bincode 2.0.0-rc][bincode] | 51.17% | 59.46% | 94.93% | 95.03% | 89.35% | 59.08% |
| [bincode 1.3.3][bincode1] | 26.60% | 71.77% | 67.29% | 77.41% | 73.70% | 49.13% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.68% | 65.92% | 79.45% | 79.74% | 80.19% | 52.11% |
| [capnp 0.20.3][capnp] | 26.85% | † | 48.76% | 56.19% | 53.60% | 37.44% |
| [cbor4ii 0.3.3][cbor4ii] | 23.00% | 29.30% | 49.99% | 71.59% | 70.89% | 49.47% |
| [ciborium 0.2.2][ciborium] | 3.34% | 12.50% | 49.99% | 71.59% | 70.89% | 49.38% |
| [databuf 0.5.0][databuf] | 53.99% | 71.83% | 91.89% | 92.66% | 86.82% | 61.74% |
| [dlhn 0.1.7][dlhn] | 19.22% | 58.56% | 97.07% | 95.81% | 90.59% | 66.90% |
| [flatbuffers 24.12.23][flatbuffers] | 13.53% | † | 55.13% | 61.64% | 59.09% | 44.61% |
| [msgpacker 0.4.5][msgpacker] | 10.96% | 54.29% | 91.99% | 91.61% | 86.73% | 60.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.54% | 35.01% | 85.96% | 86.85% | 80.47% | 51.89% |
| [nanoserde 0.1.37][nanoserde] | 59.62% | 68.64% | 67.29% | 77.41% | 73.70% | 52.00% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.28% | 65.41% | 91.89% | 92.65% | 86.86% | 58.16% |
| [postcard 1.1.1][postcard] | 32.67% | 63.57% | 97.07% | 95.51% | 90.54% | 62.56% |
| [pot 3.0.1][pot] | 6.14% | 22.12% | 72.40% | 77.53% | 75.55% | 52.55% |
| [prost 0.13.4][prost] | <span title="encode">*14.99%\**</span> <span title="populate + encode">*5.80%\**</span> | 42.65% | 79.55% | 79.54% | 72.82% | 46.89% |
| [rkyv 0.8.9][rkyv] | 55.33% | <span title="unvalidated">*92.03%\**</span> <span title="validated upfront with error">*66.36%\**</span> | 69.57% | 73.39% | 70.37% | 47.98% |
| [rmp-serde 1.3.0][rmp-serde] | 9.49% | 44.32% | 89.64% | 88.76% | 82.58% | 58.11% |
| [ron 0.8.1][ron] | 1.21% | 8.94% | 43.78% | 64.30% | 65.70% | 42.36% |
| [savefile 0.18.5][savefile] | 73.07% | 67.53% | 67.29% | 77.40% | 73.70% | 52.94% |
| [serde-brief 0.1.0][serde-brief] | 8.72% | 29.65% | 44.40% | 69.81% | 67.29% | 48.39% |
| [serde_bare 0.5.0][serde_bare] | 20.21% | 70.10% | 91.89% | 92.66% | 86.82% | 61.25% |
| [serde_cbor 0.11.2][serde_cbor] | 7.00% | 27.97% | 49.99% | 71.59% | 70.89% | 49.77% |
| [serde_json 1.0.134][serde_json] | 3.79% | 23.06% | 38.51% | 61.38% | 63.63% | 42.30% |
| [simd-json 0.14.3][simd-json] | 6.73% | 30.69% | 38.51% | 61.38% | 63.63% | 42.88% |
| [speedy 0.8.7][speedy] | 70.03% | 83.57% | 79.45% | 79.74% | 80.19% | 54.75% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.67%\**</span> | <span title="validated on-demand with error">*6.48%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.92%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.64%\**</span> <span title="validated upfront with error">*0.49%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.71%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*6.7597 ms\**</span> <span title="prepend">*8.9469 ms\**</span> | 8.5566 ms | 8625005 | 6443961 | 6231572 | 72.311 ms |
| [bincode 2.0.0-rc][bincode] | 2.4156 ms | 1.0267 ms | 6000005 | 5378497 | 5345897 | 7.7502 ms |
| [bincode 1.3.3][bincode1] | 5.1940 ms | 4.9235 ms | 6000008 | 5378500 | 5345890 | 7.5127 ms |
| [bitcode 0.6.3][bitcode] | 1.4821 ms | 798.02 µs | 6000006 | 5182295 | 4923880 | 13.146 ms |
| [borsh 1.5.3][borsh] | 6.2509 ms | 4.3658 ms | 6000004 | 5378496 | 5345889 | 7.9966 ms |
| [capnp 0.20.3][capnp] | 5.9091 ms | † | 14000088 | 7130367 | 6051062 | 80.239 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.9870 ms | 50.463 ms | 13125016 | 7524114 | 6757967 | 91.622 ms |
| [ciborium 0.2.2][ciborium] | 73.262 ms | 123.39 ms | 13122324 | 7524660 | 6759658 | 91.495 ms |
| [databuf 0.5.0][databuf] | 2.4211 ms | 5.3601 ms | 6000003 | 5378495 | 5345900 | 7.9281 ms |
| [dlhn 0.1.7][dlhn] | 6.2128 ms | 6.9286 ms | 6000003 | 5378495 | 5345900 | 7.6824 ms |
| [flatbuffers 24.12.23][flatbuffers] | 860.70 µs | † | 6000024 | 5378434 | 5345910 | 7.4038 ms |
| [msgpacker 0.4.5][msgpacker] | 18.514 ms | 5.2405 ms | 7500005 | 6058442 | 6014337 | 9.8654 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 124.03 ms | 32.899 ms | 8125037 | 6493484 | 6386940 | 71.227 ms |
| [nanoserde 0.1.37][nanoserde] | 1.8179 ms | 1.1046 ms | 6000008 | 5378500 | 5345890 | 7.5895 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1541 ms | 4.0472 ms | 6000004 | 5378496 | 5345889 | 7.5230 ms |
| [postcard 1.1.1][postcard] | 483.85 µs | 1.3169 ms | 6000003 | 5378495 | 5345900 | 7.6039 ms |
| [pot 3.0.1][pot] | 38.303 ms | 71.116 ms | 10122342 | 6814618 | 6852251 | 82.049 ms |
| [prost 0.13.4][prost] | <span title="encode">*8.0812 ms\**</span> <span title="populate + encode">*9.0772 ms\**</span> | 12.582 ms | 8750000 | 6665735 | 6421871 | 80.700 ms |
| [rkyv 0.8.9][rkyv] | 206.79 µs | <span title="unvalidated">*149.85 µs\**</span> <span title="validated upfront with error">*202.36 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.5526 ms |
| [rmp-serde 1.3.0][rmp-serde] | 18.857 ms | 17.980 ms | 8125006 | 6494876 | 6391037 | 70.540 ms |
| [ron 0.8.1][ron] | 173.85 ms | 240.18 ms | 22192885 | 8970395 | 8138755 | 149.52 ms |
| [savefile 0.18.5][savefile] | 148.69 µs | 148.50 µs | 6000024 | 5378519 | 5345892 | 7.5205 ms |
| [serde-brief 0.1.0][serde-brief] | 23.321 ms | 50.266 ms | 15750015 | 8024540 | 6816643 | 94.269 ms |
| [serde_bare 0.5.0][serde_bare] | 6.6354 ms | 4.7714 ms | 6000003 | 5378495 | 5345900 | 7.4549 ms |
| [serde_cbor 0.11.2][serde_cbor] | 36.596 ms | 45.876 ms | 13122324 | 7524660 | 6759658 | 91.212 ms |
| [serde_json 1.0.134][serde_json] | 86.741 ms | 88.993 ms | 26192883 | 9566084 | 8586741 | 162.25 ms |
| [simd-json 0.14.3][simd-json] | 51.444 ms | 70.392 ms | 26192883 | 9566084 | 8586741 | 154.42 ms |
| [speedy 0.8.7][speedy] | 202.29 µs | 149.50 µs | 6000004 | 5378496 | 5345889 | 8.4444 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*104.71 ns\**</span> | <span title="validated on-demand with error">*2.1527 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4852 ns\**</span> <span title="validated upfront with error">*39.904 ns\**</span> | <span title="unvalidated">*54.334 µs\**</span> <span title="validated upfront with error">*77.730 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2428 ns\**</span> <span title="validated upfront with error">*4.9812 ns\**</span> | <span title="unvalidated">*49.293 µs\**</span> <span title="validated upfront with error">*77.795 µs\**</span> | <span title="unvalidated">*76.683 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*2.20%\**</span> <span title="prepend">*1.66%\**</span> | 1.74% | 69.57% | 80.42% | 79.02% | 10.24% |
| [bincode 2.0.0-rc][bincode] | 6.16% | 14.46% | 100.00% | 96.35% | 92.11% | 95.53% |
| [bincode 1.3.3][bincode1] | 2.86% | 3.02% | 100.00% | 96.35% | 92.11% | 98.55% |
| [bitcode 0.6.3][bitcode] | 10.03% | 18.61% | 100.00% | 100.00% | 100.00% | 56.32% |
| [borsh 1.5.3][borsh] | 2.38% | 3.40% | 100.00% | 96.35% | 92.11% | 92.59% |
| [capnp 0.20.3][capnp] | 2.52% | † | 42.86% | 72.68% | 81.37% | 9.23% |
| [cbor4ii 0.3.3][cbor4ii] | 1.49% | 0.29% | 45.71% | 68.88% | 72.86% | 8.08% |
| [ciborium 0.2.2][ciborium] | 0.20% | 0.12% | 45.72% | 68.87% | 72.84% | 8.09% |
| [databuf 0.5.0][databuf] | 6.14% | 2.77% | 100.00% | 96.35% | 92.11% | 93.39% |
| [dlhn 0.1.7][dlhn] | 2.39% | 2.14% | 100.00% | 96.35% | 92.11% | 96.37% |
| [flatbuffers 24.12.23][flatbuffers] | 17.28% | † | 100.00% | 96.35% | 92.11% | 100.00% |
| [msgpacker 0.4.5][msgpacker] | 0.80% | 2.83% | 80.00% | 85.54% | 81.87% | 75.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.45% | 73.85% | 79.81% | 77.09% | 10.39% |
| [nanoserde 0.1.37][nanoserde] | 8.18% | 13.44% | 100.00% | 96.35% | 92.11% | 97.55% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.88% | 3.67% | 100.00% | 96.35% | 92.11% | 98.42% |
| [postcard 1.1.1][postcard] | 30.73% | 11.28% | 100.00% | 96.35% | 92.11% | 97.37% |
| [pot 3.0.1][pot] | 0.39% | 0.21% | 59.27% | 76.05% | 71.86% | 9.02% |
| [prost 0.13.4][prost] | <span title="encode">*1.84%\**</span> <span title="populate + encode">*1.64%\**</span> | 1.18% | 68.57% | 77.75% | 76.67% | 9.17% |
| [rkyv 0.8.9][rkyv] | 71.90% | <span title="unvalidated">*99.10%\**</span> <span title="validated upfront with error">*73.38%\**</span> | 100.00% | 96.35% | 92.11% | 98.03% |
| [rmp-serde 1.3.0][rmp-serde] | 0.79% | 0.83% | 73.85% | 79.79% | 77.04% | 10.50% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 4.95% |
| [savefile 0.18.5][savefile] | 100.00% | 100.00% | 100.00% | 96.35% | 92.11% | 98.45% |
| [serde-brief 0.1.0][serde-brief] | 0.64% | 0.30% | 38.10% | 64.58% | 72.23% | 7.85% |
| [serde_bare 0.5.0][serde_bare] | 2.24% | 3.11% | 100.00% | 96.35% | 92.11% | 99.31% |
| [serde_cbor 0.11.2][serde_cbor] | 0.41% | 0.32% | 45.72% | 68.87% | 72.84% | 8.12% |
| [serde_json 1.0.134][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.56% |
| [simd-json 0.14.3][simd-json] | 0.29% | 0.21% | 22.91% | 54.17% | 57.34% | 4.79% |
| [speedy 0.8.7][speedy] | 73.50% | 99.33% | 100.00% | 96.35% | 92.11% | 87.68% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.19%\**</span> | <span title="validated on-demand with error">*2.29%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*3.11%\**</span> | <span title="unvalidated">*90.72%\**</span> <span title="validated upfront with error">*63.42%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.95%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*63.36%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*928.91 µs\**</span> <span title="prepend">*836.74 µs\**</span> | 3.2391 ms | 489348 | 281173 | 249546 | 3.0765 ms |
| [bincode 2.0.0-rc][bincode] | 258.59 µs | 2.0604 ms | 367413 | 221291 | 206273 | 2.4684 ms |
| [bincode 1.3.3][bincode1] | 600.02 µs | 1.8225 ms | 569975 | 240525 | 232423 | 2.8812 ms |
| [bitcode 0.6.3][bitcode] | 132.42 µs | 1.2394 ms | 327688 | 200947 | 182736 | 739.92 µs |
| [borsh 1.5.3][borsh] | 551.97 µs | 1.8112 ms | 446595 | 234236 | 210008 | 2.5074 ms |
| [capnp 0.20.3][capnp] | 449.81 µs | † | 803896 | 335606 | 280851 | 3.8593 ms |
| [cbor4ii 0.3.3][cbor4ii] | 788.38 µs | 4.6826 ms | 1109831 | 344745 | 274514 | 3.8792 ms |
| [ciborium 0.2.2][ciborium] | 3.7877 ms | 9.9617 ms | 1109821 | 344751 | 274526 | 3.8084 ms |
| [databuf 0.5.0][databuf] | 305.54 µs | 1.6970 ms | 356311 | 213062 | 198488 | 2.3946 ms |
| [dlhn 0.1.7][dlhn] | 781.87 µs | 2.6028 ms | 366496 | 220600 | 205683 | 2.4754 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.2258 ms | † | 844168 | 345696 | 294015 | 3.8052 ms |
| [msgpacker 0.4.5][msgpacker] | 965.12 µs | 2.9357 ms | 391251 | 236877 | 220476 | 2.6469 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0899 ms | 4.0377 ms | 449745 | 252432 | 231110 | 2.8770 ms |
| [nanoserde 0.1.37][nanoserde] | 271.81 µs | 2.0290 ms | 567975 | 239930 | 232419 | 2.8810 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 615.08 µs | 1.9511 ms | 356311 | 212976 | 198524 | 2.3907 ms |
| [postcard 1.1.1][postcard] | 450.56 µs | 2.0094 ms | 367489 | 221913 | 207344 | 2.5027 ms |
| [pot 3.0.1][pot] | 2.4133 ms | 6.2246 ms | 599125 | 299158 | 247693 | 3.2331 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.0901 ms\**</span> <span title="populate + encode">*2.7592 ms\**</span> | 3.3859 ms | 596811 | 305319 | 269310 | 3.4422 ms |
| [rkyv 0.8.9][rkyv] | 338.67 µs | <span title="unvalidated">*1.4704 ms\**</span> <span title="validated upfront with error">*2.0078 ms\**</span> | 603776 | 254776 | 220087 | 2.7692 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5350 ms | 3.0348 ms | 424533 | 245214 | 226188 | 2.6929 ms |
| [ron 0.8.1][ron] | 7.2889 ms | 16.988 ms | 1465223 | 434935 | 343338 | 5.8021 ms |
| [savefile 0.18.5][savefile] | 213.75 µs | 1.7991 ms | 566991 | 239362 | 232010 | 2.8887 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3550 ms | 5.3884 ms | 1276014 | 373898 | 293679 | 4.0623 ms |
| [serde_bare 0.5.0][serde_bare] | 759.07 µs | 2.3182 ms | 356311 | 213062 | 198488 | 2.3782 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8376 ms | 4.7011 ms | 1109821 | 344751 | 274526 | 3.8162 ms |
| [serde_json 1.0.134][serde_json] | 3.6290 ms | 6.9617 ms | 1623191 | 466527 | 359623 | 6.0259 ms |
| [simd-json 0.14.3][simd-json] | 2.2193 ms | 4.6008 ms | 1623191 | 466527 | 359623 | 5.9468 ms |
| [speedy 0.8.7][speedy] | 269.01 µs | 1.5704 ms | 449595 | 234970 | 210361 | 2.4810 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*76.180 ns\**</span> | <span title="validated on-demand with error">*412.02 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4880 ns\**</span> <span title="validated upfront with error">*2.2088 ms\**</span> | <span title="unvalidated">*1.3652 µs\**</span> <span title="validated upfront with error">*2.1801 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2435 ns\**</span> <span title="validated upfront with error">*518.45 µs\**</span> | <span title="unvalidated">*164.21 ns\**</span> <span title="validated upfront with error">*526.81 µs\**</span> | <span title="unvalidated">*765.48 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*14.26%\**</span> <span title="prepend">*15.83%\**</span> | 38.26% | 66.96% | 71.47% | 73.23% | 24.05% |
| [bincode 2.0.0-rc][bincode] | 51.21% | 60.15% | 89.19% | 90.81% | 88.59% | 29.98% |
| [bincode 1.3.3][bincode1] | 22.07% | 68.01% | 57.49% | 83.55% | 78.62% | 25.68% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 23.99% | 68.43% | 73.37% | 85.79% | 87.01% | 29.51% |
| [capnp 0.20.3][capnp] | 29.44% | † | 40.76% | 59.88% | 65.07% | 19.17% |
| [cbor4ii 0.3.3][cbor4ii] | 16.80% | 26.47% | 29.53% | 58.29% | 66.57% | 19.07% |
| [ciborium 0.2.2][ciborium] | 3.50% | 12.44% | 29.53% | 58.29% | 66.56% | 19.43% |
| [databuf 0.5.0][databuf] | 43.34% | 73.03% | 91.97% | 94.31% | 92.06% | 30.90% |
| [dlhn 0.1.7][dlhn] | 16.94% | 47.62% | 89.41% | 91.09% | 88.84% | 29.89% |
| [flatbuffers 24.12.23][flatbuffers] | 4.11% | † | 38.82% | 58.13% | 62.15% | 19.45% |
| [msgpacker 0.4.5][msgpacker] | 13.72% | 42.22% | 83.75% | 84.83% | 82.88% | 27.95% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.60% | 30.70% | 72.86% | 79.60% | 79.07% | 25.72% |
| [nanoserde 0.1.37][nanoserde] | 48.72% | 61.08% | 57.69% | 83.75% | 78.62% | 25.68% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.53% | 63.52% | 91.97% | 94.35% | 92.05% | 30.95% |
| [postcard 1.1.1][postcard] | 29.39% | 61.68% | 89.17% | 90.55% | 88.13% | 29.56% |
| [pot 3.0.1][pot] | 5.49% | 19.91% | 54.69% | 67.17% | 73.78% | 22.89% |
| [prost 0.13.4][prost] | <span title="encode">*12.15%\**</span> <span title="populate + encode">*4.80%\**</span> | 36.60% | 54.91% | 65.82% | 67.85% | 21.50% |
| [rkyv 0.8.9][rkyv] | 39.10% | <span title="unvalidated">*84.29%\**</span> <span title="validated upfront with error">*61.73%\**</span> | 54.27% | 78.87% | 83.03% | 26.72% |
| [rmp-serde 1.3.0][rmp-serde] | 8.63% | 40.84% | 77.19% | 81.95% | 80.79% | 27.48% |
| [ron 0.8.1][ron] | 1.82% | 7.30% | 22.36% | 46.20% | 53.22% | 12.75% |
| [savefile 0.18.5][savefile] | 61.95% | 68.89% | 57.79% | 83.95% | 78.76% | 25.61% |
| [serde-brief 0.1.0][serde-brief] | 9.77% | 23.00% | 25.68% | 53.74% | 62.22% | 18.21% |
| [serde_bare 0.5.0][serde_bare] | 17.45% | 53.46% | 91.97% | 94.31% | 92.06% | 31.11% |
| [serde_cbor 0.11.2][serde_cbor] | 7.21% | 26.36% | 29.53% | 58.29% | 66.56% | 19.39% |
| [serde_json 1.0.134][serde_json] | 3.65% | 17.80% | 20.19% | 43.07% | 50.81% | 12.28% |
| [simd-json 0.14.3][simd-json] | 5.97% | 26.94% | 20.19% | 43.07% | 50.81% | 12.44% |
| [speedy 0.8.7][speedy] | 49.22% | 78.92% | 72.89% | 85.52% | 86.87% | 29.82% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.63%\**</span> | <span title="validated on-demand with error">*39.85%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.03%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*4.5791 ms\**</span> <span title="prepend">*2.5814 ms\**</span> | 8.3844 ms | 1704643 | 1294259 | 1245607 | 11.479 ms |
| [bincode 2.0.0-rc][bincode] | 1.1939 ms | 3.9018 ms | 1406257 | 1117802 | 1062238 | 9.4997 ms |
| [bincode 1.3.3][bincode1] | 4.0822 ms | 4.1737 ms | 1854234 | 1141994 | 1050351 | 10.286 ms |
| [bitcode 0.6.3][bitcode] | 723.73 µs | 2.3248 ms | 971318 | 878034 | 855922 | 3.3384 ms |
| [borsh 1.5.3][borsh] | 2.9460 ms | 2.8979 ms | 1521989 | 1108471 | 1038408 | 9.7777 ms |
| [capnp 0.20.3][capnp] | 2.2366 ms | † | 2724288 | 1546992 | 1240354 | 14.974 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.3310 ms | 18.547 ms | 6012539 | 1695215 | 1467194 | 21.793 ms |
| [ciborium 0.2.2][ciborium] | 24.589 ms | 54.657 ms | 6012373 | 1695146 | 1467435 | 21.337 ms |
| [databuf 0.5.0][databuf] | 1.3013 ms | 3.7358 ms | 1319999 | 1062631 | 1007898 | 8.9719 ms |
| [dlhn 0.1.7][dlhn] | 4.9342 ms | 8.1979 ms | 1311281 | 1077520 | 1045571 | 8.8089 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.0391 ms | † | 2325620 | 1440289 | 1265148 | 13.471 ms |
| [msgpacker 0.4.5][msgpacker] | 2.2358 ms | 6.8456 ms | 1458773 | 1156055 | 1137194 | 9.5984 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.563 ms | 18.090 ms | 1770060 | 1277755 | 1263142 | 13.134 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2652 ms | 2.9410 ms | 1812404 | 1134820 | 1054758 | 10.375 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.1414 ms | 3.1406 ms | 1319999 | 1064380 | 1010284 | 8.8117 ms |
| [postcard 1.1.1][postcard] | 2.1520 ms | 4.2509 ms | 1311281 | 1083900 | 1041114 | 8.7832 ms |
| [pot 3.0.1][pot] | 13.638 ms | 30.641 ms | 2604812 | 1482233 | 1299952 | 15.742 ms |
| [prost 0.13.4][prost] | <span title="encode">*4.9716 ms\**</span> <span title="populate + encode">*8.8483 ms\**</span> | 8.9905 ms | 1859886 | 1338076 | 1295497 | 11.874 ms |
| [rkyv 0.8.9][rkyv] | 1.0144 ms | <span title="unvalidated">*2.1786 ms\**</span> <span title="validated upfront with error">*2.6008 ms\**</span> | 2075936 | 1383779 | 1211892 | 12.671 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.250 ms | 11.141 ms | 1745322 | 1261627 | 1228902 | 11.091 ms |
| [ron 0.8.1][ron] | 37.967 ms | 91.543 ms | 8677703 | 2233642 | 1827843 | 34.532 ms |
| [savefile 0.18.5][savefile] | 844.11 µs | 2.7791 ms | 1791505 | 1128012 | 1052757 | 10.133 ms |
| [serde-brief 0.1.0][serde-brief] | 6.4551 ms | 22.109 ms | 6951772 | 1796265 | 1570903 | 23.662 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9354 ms | 4.7399 ms | 1319999 | 1062645 | 1007918 | 8.7667 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.408 ms | 21.068 ms | 6012373 | 1695146 | 1467435 | 21.394 ms |
| [serde_json 1.0.134][serde_json] | 20.624 ms | 31.060 ms | 9390461 | 2391679 | 1843922 | 34.662 ms |
| [simd-json 0.14.3][simd-json] | 11.745 ms | 26.263 ms | 9390461 | 2391679 | 1843922 | 37.100 ms |
| [speedy 0.8.7][speedy] | 785.24 µs | 2.4834 ms | 1584734 | 1119837 | 1038012 | 10.068 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*75.669 ns\**</span> | <span title="validated on-demand with error">*710.31 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4848 ns\**</span> <span title="validated upfront with error">*5.2578 ms\**</span> | <span title="unvalidated">*2.6178 µs\**</span> <span title="validated upfront with error">*5.1285 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2428 ns\**</span> <span title="validated upfront with error">*423.88 µs\**</span> | <span title="unvalidated">*434.25 ns\**</span> <span title="validated upfront with error">*424.91 µs\**</span> | <span title="unvalidated">*235.74 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*15.81%\**</span> <span title="prepend">*28.04%\**</span> | 25.98% | 56.98% | 67.84% | 68.72% | 29.08% |
| [bincode 2.0.0-rc][bincode] | 60.62% | 55.84% | 69.07% | 78.55% | 80.58% | 35.14% |
| [bincode 1.3.3][bincode1] | 17.73% | 52.20% | 52.38% | 76.89% | 81.49% | 32.46% |
| [bitcode 0.6.3][bitcode] | 100.00% | 93.71% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.57% | 75.18% | 63.82% | 79.21% | 82.43% | 34.14% |
| [capnp 0.20.3][capnp] | 32.36% | † | 35.65% | 56.76% | 69.01% | 22.30% |
| [cbor4ii 0.3.3][cbor4ii] | 21.73% | 11.75% | 16.15% | 51.79% | 58.34% | 15.32% |
| [ciborium 0.2.2][ciborium] | 2.94% | 3.99% | 16.16% | 51.80% | 58.33% | 15.65% |
| [databuf 0.5.0][databuf] | 55.62% | 58.32% | 73.58% | 82.63% | 84.92% | 37.21% |
| [dlhn 0.1.7][dlhn] | 14.67% | 26.58% | 74.07% | 81.49% | 81.86% | 37.90% |
| [flatbuffers 24.12.23][flatbuffers] | 14.36% | † | 41.77% | 60.96% | 67.65% | 24.78% |
| [msgpacker 0.4.5][msgpacker] | 32.37% | 31.82% | 66.58% | 75.95% | 75.27% | 34.78% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.37% | 12.04% | 54.87% | 68.72% | 67.76% | 25.42% |
| [nanoserde 0.1.37][nanoserde] | 57.20% | 74.08% | 53.59% | 77.37% | 81.15% | 32.18% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 23.04% | 69.37% | 73.58% | 82.49% | 84.72% | 37.89% |
| [postcard 1.1.1][postcard] | 33.63% | 51.25% | 74.07% | 81.01% | 82.21% | 38.01% |
| [pot 3.0.1][pot] | 5.31% | 7.11% | 37.29% | 59.24% | 65.84% | 21.21% |
| [prost 0.13.4][prost] | <span title="encode">*14.56%\**</span> <span title="populate + encode">*8.18%\**</span> | 24.23% | 52.22% | 65.62% | 66.07% | 28.11% |
| [rkyv 0.8.9][rkyv] | 71.35% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.77%\**</span> | 46.79% | 63.45% | 70.63% | 26.35% |
| [rmp-serde 1.3.0][rmp-serde] | 7.06% | 19.55% | 55.65% | 69.60% | 69.65% | 30.10% |
| [ron 0.8.1][ron] | 1.91% | 2.38% | 11.19% | 39.31% | 46.83% | 9.67% |
| [savefile 0.18.5][savefile] | 85.74% | 78.39% | 54.22% | 77.84% | 81.30% | 32.95% |
| [serde-brief 0.1.0][serde-brief] | 11.21% | 9.85% | 13.97% | 48.88% | 54.49% | 14.11% |
| [serde_bare 0.5.0][serde_bare] | 14.66% | 45.96% | 73.58% | 82.63% | 84.92% | 38.08% |
| [serde_cbor 0.11.2][serde_cbor] | 6.95% | 10.34% | 16.16% | 51.80% | 58.33% | 15.60% |
| [serde_json 1.0.134][serde_json] | 3.51% | 7.01% | 10.34% | 36.71% | 46.42% | 9.63% |
| [simd-json 0.14.3][simd-json] | 6.16% | 8.30% | 10.34% | 36.71% | 46.42% | 9.00% |
| [speedy 0.8.7][speedy] | 92.17% | 87.73% | 61.29% | 78.41% | 82.46% | 33.16% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.64%\**</span> | <span title="validated on-demand with error">*61.14%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.59%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1012.1
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

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
