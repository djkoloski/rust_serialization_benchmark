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

## Last updated: 2025-03-09 01:40:56

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.87.0-nightly (f5a1ef712 2025-03-07)
binary: rustc
commit-hash: f5a1ef7121ad661b5a21a1d02941c8064d54ee0b
commit-date: 2025-03-07
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
| [bilrost 0.1012.3][bilrost] | <span title="encode">*482.28 µs\**</span> <span title="prepend">*432.39 µs\**</span> | 2.5779 ms | 884.75 µs | 804955 | 328941 | 284849 | 4.1081 ms |
| [bincode 2.0.0-rc][bincode] | 302.35 µs | 2.2225 ms | 701.24 µs | 741295 | 303944 | 256422 | 3.6037 ms |
| [bincode 1.3.3][bincode1] | 559.62 µs | 2.0721 ms | 618.44 µs | 1045784 | 373127 | 311553 | 4.4943 ms |
| [bitcode 0.6.5][bitcode] | 144.96 µs | 1.4437 ms | 62.457 µs | 703710 | 288826 | 227322 | 2.4754 ms |
| [borsh 1.5.5][borsh] | 548.45 µs | 2.1554 ms | † | 885780 | 362204 | 286248 | 4.1350 ms |
| [capnp 0.20.6][capnp] | 520.11 µs | † | † | 1443216 | 513986 | 426532 | 6.0999 ms |
| [cbor4ii 1.0.0][cbor4ii] | 655.09 µs | 5.1315 ms | 3.3668 ms | 1407835 | 403440 | 323561 | 4.7575 ms |
| [ciborium 0.2.2][ciborium] | 4.1057 ms | 12.013 ms | † | 1407835 | 403440 | 323561 | 4.6918 ms |
| [databuf 0.5.0][databuf] | 256.86 µs | 1.9917 ms | 667.76 µs | 765778 | 311715 | 263914 | 3.4495 ms |
| [dlhn 0.1.7][dlhn] | 693.77 µs | 2.5231 ms | † | 724953 | 301446 | 253056 | 3.1692 ms |
| [flatbuffers 25.2.10][flatbuffers] | 1.0148 ms | † | † | 1276368 | 468539 | 388381 | 4.7349 ms |
| [minicbor 0.26.1][minicbor] | 626.08 µs | 2.9477 ms | 1.4003 ms | 817830 | 332671 | 284034 | 3.9455 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2337 ms | 4.1297 ms | 2.5839 ms | 818669 | 332556 | 284797 | 3.9478 ms |
| [nanoserde 0.1.37][nanoserde] | 260.81 µs | 2.0623 ms | † | 1045784 | 373127 | 311553 | 4.1651 ms |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 672.26 µs | 2.3791 ms | † | 765778 | 311743 | 263822 | 3.4844 ms |
| [postcard 1.1.1][postcard] | 426.85 µs | 2.2871 ms | 867.19 µs | 724953 | 302399 | 252968 | 3.1679 ms |
| [pot 3.0.1][pot] | 2.4076 ms | 6.5105 ms | 4.8875 ms | 971922 | 372513 | 303636 | 4.3768 ms |
| [prost 0.13.5][prost] | <span title="encode">*940.75 µs\**</span> <span title="populate + encode">*2.4640 ms\**</span> | 3.3046 ms | † | 884628 | 363130 | 314959 | 4.3210 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.1777 ms\**</span> <span title="populate + encode">*2.9871 ms\**</span> | 3.7661 ms | † | 884628 | 363130 | 314959 | 4.3713 ms |
| [rkyv 0.8.10][rkyv] | 244.33 µs | <span title="unvalidated">*1.5372 ms\**</span> <span title="validated upfront with error">*1.9312 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5607 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4052 ms | 3.0992 ms | 1.4344 ms | 784997 | 325384 | 277608 | 3.7320 ms |
| [ron 0.8.1][ron] | 11.592 ms | 15.939 ms | 13.534 ms | 1607459 | 449158 | 349324 | 5.5680 ms |
| [savefile 0.18.5][savefile] | 191.15 µs | 2.1648 ms | † | 1045800 | 373139 | 311562 | 4.1727 ms |
| [serde-brief 0.1.1][serde-brief] | 1.5443 ms | 4.7525 ms | 3.0344 ms | 1584946 | 413733 | 339964 | 4.8360 ms |
| [serde_bare 0.5.0][serde_bare] | 707.69 µs | 2.0805 ms | † | 765778 | 311715 | 263914 | 3.4908 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0538 ms | 4.9998 ms | 3.2505 ms | 1407835 | 403440 | 323561 | 4.6851 ms |
| [serde_json 1.0.140][serde_json] | 3.7113 ms | 5.9547 ms | † | 1827461 | 470560 | 360727 | 5.4644 ms |
| [simd-json 0.14.3][simd-json] | 2.0718 ms | 4.6308 ms | † | 1827461 | 470560 | 360727 | 5.4855 ms |
| [speedy 0.8.7][speedy] | 198.87 µs | 1.7331 ms | 377.63 µs | 885780 | 362204 | 286248 | 3.8417 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*73.870 ns\**</span> | <span title="validated on-demand with error">*169.01 µs\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4865 ns\**</span> <span title="validated upfront with error">*2.0416 ms\**</span> | <span title="unvalidated">*51.761 µs\**</span> <span title="validated upfront with error">*2.0948 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2432 ns\**</span> <span title="validated upfront with error">*381.38 µs\**</span> | <span title="unvalidated">*10.642 µs\**</span> <span title="validated upfront with error">*392.56 µs\**</span> | <span title="unvalidated">*7.4575 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*30.06%\**</span> <span title="prepend">*33.53%\**</span> | 56.00% | 7.06% | 87.42% | 87.80% | 79.80% | 60.26% |
| [bincode 2.0.0-rc][bincode] | 47.94% | 64.96% | 8.91% | 94.93% | 95.03% | 88.65% | 68.69% |
| [bincode 1.3.3][bincode1] | 25.90% | 69.67% | 10.10% | 67.29% | 77.41% | 72.96% | 55.08% |
| [bitcode 0.6.5][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 26.43% | 66.98% | † | 79.45% | 79.74% | 79.41% | 59.86% |
| [capnp 0.20.6][capnp] | 27.87% | † | † | 48.76% | 56.19% | 53.30% | 40.58% |
| [cbor4ii 1.0.0][cbor4ii] | 22.13% | 28.13% | 1.86% | 49.99% | 71.59% | 70.26% | 52.03% |
| [ciborium 0.2.2][ciborium] | 3.53% | 12.02% | † | 49.99% | 71.59% | 70.26% | 52.76% |
| [databuf 0.5.0][databuf] | 56.44% | 72.49% | 9.35% | 91.89% | 92.66% | 86.13% | 71.76% |
| [dlhn 0.1.7][dlhn] | 20.89% | 57.22% | † | 97.07% | 95.81% | 89.83% | 78.11% |
| [flatbuffers 25.2.10][flatbuffers] | 14.28% | † | † | 55.13% | 61.64% | 58.53% | 52.28% |
| [minicbor 0.26.1][minicbor] | 23.15% | 48.98% | 4.46% | 86.05% | 86.82% | 80.03% | 62.74% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.77% | 34.96% | 2.42% | 85.96% | 86.85% | 79.82% | 62.70% |
| [nanoserde 0.1.37][nanoserde] | 55.58% | 70.00% | † | 67.29% | 77.41% | 72.96% | 59.43% |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 21.56% | 60.68% | † | 91.89% | 92.65% | 86.16% | 71.04% |
| [postcard 1.1.1][postcard] | 33.96% | 63.12% | 7.20% | 97.07% | 95.51% | 89.86% | 78.14% |
| [pot 3.0.1][pot] | 6.02% | 22.17% | 1.28% | 72.40% | 77.53% | 74.87% | 56.56% |
| [prost 0.13.5][prost] | <span title="encode">*15.41%\**</span> <span title="populate + encode">*5.88%\**</span> | 43.69% | † | 79.55% | 79.54% | 72.18% | 57.29% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*12.31%\**</span> <span title="populate + encode">*4.85%\**</span> | 38.33% | † | 79.55% | 79.54% | 72.18% | 56.63% |
| [rkyv 0.8.10][rkyv] | 59.33% | <span title="unvalidated">*93.92%\**</span> <span title="validated upfront with error">*74.76%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.28% |
| [rmp-serde 1.3.0][rmp-serde] | 10.32% | 46.58% | 4.35% | 89.64% | 88.76% | 81.89% | 66.33% |
| [ron 0.8.1][ron] | 1.25% | 9.06% | 0.46% | 43.78% | 64.30% | 65.07% | 44.46% |
| [savefile 0.18.5][savefile] | 75.84% | 66.69% | † | 67.29% | 77.40% | 72.96% | 59.32% |
| [serde-brief 0.1.1][serde-brief] | 9.39% | 30.38% | 2.06% | 44.40% | 69.81% | 66.87% | 51.19% |
| [serde_bare 0.5.0][serde_bare] | 20.48% | 69.39% | † | 91.89% | 92.66% | 86.13% | 70.91% |
| [serde_cbor 0.11.2][serde_cbor] | 7.06% | 28.88% | 1.92% | 49.99% | 71.59% | 70.26% | 52.84% |
| [serde_json 1.0.140][serde_json] | 3.91% | 24.24% | † | 38.51% | 61.38% | 63.02% | 45.30% |
| [simd-json 0.14.3][simd-json] | 7.00% | 31.18% | † | 38.51% | 61.38% | 63.02% | 45.13% |
| [speedy 0.8.7][speedy] | 72.89% | 83.30% | 16.54% | 79.45% | 79.74% | 79.41% | 64.44% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*6.30%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.56%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.71%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*6.6813 ms\**</span> <span title="prepend">*8.8029 ms\**</span> | 8.2760 ms | 8625005 | 6443961 | 6231572 | 71.959 ms |
| [bincode 2.0.0-rc][bincode] | 2.8810 ms | 1.3916 ms | 6000005 | 5378497 | 5346882 | 8.4585 ms |
| [bincode 1.3.3][bincode1] | 4.6893 ms | 896.65 µs | 6000008 | 5378500 | 5346908 | 8.4343 ms |
| [bitcode 0.6.5][bitcode] | 1.3952 ms | 799.75 µs | 6000006 | 5182295 | 4921841 | 13.387 ms |
| [borsh 1.5.5][borsh] | 6.1176 ms | 4.2939 ms | 6000004 | 5378496 | 5346866 | 8.5963 ms |
| [capnp 0.20.6][capnp] | 5.7716 ms | † | 14000088 | 7130367 | 6046182 | 83.963 ms |
| [cbor4ii 1.0.0][cbor4ii] | 10.020 ms | 51.179 ms | 13125016 | 7524114 | 6757437 | 90.886 ms |
| [ciborium 0.2.2][ciborium] | 68.945 ms | 118.72 ms | 13122324 | 7524660 | 6759128 | 90.782 ms |
| [databuf 0.5.0][databuf] | 2.4094 ms | 5.4106 ms | 6000003 | 5378495 | 5346897 | 8.5153 ms |
| [dlhn 0.1.7][dlhn] | 6.3216 ms | 6.8725 ms | 6000003 | 5378495 | 5346897 | 8.5105 ms |
| [flatbuffers 25.2.10][flatbuffers] | 876.14 µs | † | 6000024 | 5378434 | 5346878 | 8.5276 ms |
| [minicbor 0.26.1][minicbor] | 5.1837 ms | 11.628 ms | 8125006 | 6494907 | 6390894 | 69.311 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 118.48 ms | 32.897 ms | 8125037 | 6493484 | 6386940 | 70.591 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2563 ms | 1.1095 ms | 6000008 | 5378500 | 5346908 | 8.3511 ms |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 5.1132 ms | 4.3886 ms | 6000004 | 5378496 | 5346866 | 8.4053 ms |
| [postcard 1.1.1][postcard] | 478.66 µs | 1.3917 ms | 6000003 | 5378495 | 5346897 | 8.5073 ms |
| [pot 3.0.1][pot] | 39.121 ms | 69.111 ms | 10122342 | 6814618 | 6852252 | 81.812 ms |
| [prost 0.13.5][prost] | <span title="encode">*7.7619 ms\**</span> <span title="populate + encode">*8.3018 ms\**</span> | 15.228 ms | 8750000 | 6665735 | 6421877 | 72.160 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*15.693 ms\**</span> <span title="populate + encode">*31.689 ms\**</span> | 29.520 ms | 8750000 | 6665735 | 6421877 | 79.171 ms |
| [rkyv 0.8.10][rkyv] | 198.63 µs | <span title="unvalidated">*197.65 µs\**</span> <span title="validated upfront with error">*197.77 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.4125 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.881 ms | 17.686 ms | 8125006 | 6494876 | 6391037 | 69.665 ms |
| [ron 0.8.1][ron] | 173.13 ms | 237.64 ms | 22192885 | 8970395 | 8137334 | 151.32 ms |
| [savefile 0.18.5][savefile] | 197.41 µs | 198.17 µs | 6000024 | 5378519 | 5346896 | 8.6171 ms |
| [serde-brief 0.1.1][serde-brief] | 23.257 ms | 39.013 ms | 15750015 | 8024540 | 6813667 | 92.852 ms |
| [serde_bare 0.5.0][serde_bare] | 5.1892 ms | 4.7878 ms | 6000003 | 5378495 | 5346897 | 8.4717 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.729 ms | 48.501 ms | 13122324 | 7524660 | 6759128 | 90.716 ms |
| [serde_json 1.0.140][serde_json] | 87.801 ms | 88.388 ms | 26192883 | 9566084 | 8584671 | 156.29 ms |
| [simd-json 0.14.3][simd-json] | 51.825 ms | 69.449 ms | 26192883 | 9566084 | 8584671 | 156.14 ms |
| [speedy 0.8.7][speedy] | 197.71 µs | 198.16 µs | 6000004 | 5378496 | 5346866 | 8.3548 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*102.90 ns\**</span> | <span title="validated on-demand with error">*2.2731 ms\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4861 ns\**</span> <span title="validated upfront with error">*46.018 ns\**</span> | <span title="unvalidated">*77.856 µs\**</span> <span title="validated upfront with error">*78.149 µs\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*5.2927 ns\**</span> | <span title="unvalidated">*58.530 µs\**</span> <span title="validated upfront with error">*38.883 µs\**</span> | <span title="unvalidated">*98.739 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*2.95%\**</span> <span title="prepend">*2.24%\**</span> | 2.39% | 69.57% | 80.42% | 78.98% | 11.61% |
| [bincode 2.0.0-rc][bincode] | 6.85% | 14.20% | 100.00% | 96.35% | 92.05% | 98.73% |
| [bincode 1.3.3][bincode1] | 4.21% | 22.04% | 100.00% | 96.35% | 92.05% | 99.01% |
| [bitcode 0.6.5][bitcode] | 14.15% | 24.71% | 100.00% | 100.00% | 100.00% | 62.38% |
| [borsh 1.5.5][borsh] | 3.23% | 4.60% | 100.00% | 96.35% | 92.05% | 97.15% |
| [capnp 0.20.6][capnp] | 3.42% | † | 42.86% | 72.68% | 81.40% | 9.95% |
| [cbor4ii 1.0.0][cbor4ii] | 1.97% | 0.39% | 45.71% | 68.88% | 72.84% | 9.19% |
| [ciborium 0.2.2][ciborium] | 0.29% | 0.17% | 45.72% | 68.87% | 72.82% | 9.20% |
| [databuf 0.5.0][databuf] | 8.19% | 3.65% | 100.00% | 96.35% | 92.05% | 98.07% |
| [dlhn 0.1.7][dlhn] | 3.12% | 2.88% | 100.00% | 96.35% | 92.05% | 98.13% |
| [flatbuffers 25.2.10][flatbuffers] | 22.53% | † | 100.00% | 96.35% | 92.05% | 97.93% |
| [minicbor 0.26.1][minicbor] | 3.81% | 1.70% | 73.85% | 79.79% | 77.01% | 12.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.17% | 0.60% | 73.85% | 79.81% | 77.06% | 11.83% |
| [nanoserde 0.1.37][nanoserde] | 15.71% | 17.81% | 100.00% | 96.35% | 92.05% | 100.00% |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 3.86% | 4.50% | 100.00% | 96.35% | 92.05% | 99.36% |
| [postcard 1.1.1][postcard] | 41.24% | 14.20% | 100.00% | 96.35% | 92.05% | 98.16% |
| [pot 3.0.1][pot] | 0.50% | 0.29% | 59.27% | 76.05% | 71.83% | 10.21% |
| [prost 0.13.5][prost] | <span title="encode">*2.54%\**</span> <span title="populate + encode">*2.38%\**</span> | 1.30% | 68.57% | 77.75% | 76.64% | 11.57% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.26%\**</span> <span title="populate + encode">*0.62%\**</span> | 0.67% | 68.57% | 77.75% | 76.64% | 10.55% |
| [rkyv 0.8.10][rkyv] | 99.39% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.94%\**</span> | 100.00% | 96.35% | 92.05% | 99.27% |
| [rmp-serde 1.3.0][rmp-serde] | 1.24% | 1.12% | 73.85% | 79.79% | 77.01% | 11.99% |
| [ron 0.8.1][ron] | 0.11% | 0.08% | 27.04% | 57.77% | 60.48% | 5.52% |
| [savefile 0.18.5][savefile] | 100.00% | 99.74% | 100.00% | 96.35% | 92.05% | 96.91% |
| [serde-brief 0.1.1][serde-brief] | 0.85% | 0.51% | 38.10% | 64.58% | 72.23% | 8.99% |
| [serde_bare 0.5.0][serde_bare] | 3.80% | 4.13% | 100.00% | 96.35% | 92.05% | 98.58% |
| [serde_cbor 0.11.2][serde_cbor] | 0.55% | 0.41% | 45.72% | 68.87% | 72.82% | 9.21% |
| [serde_json 1.0.140][serde_json] | 0.22% | 0.22% | 22.91% | 54.17% | 57.33% | 5.34% |
| [simd-json 0.14.3][simd-json] | 0.38% | 0.28% | 22.91% | 54.17% | 57.33% | 5.35% |
| [speedy 0.8.7][speedy] | 99.85% | 99.74% | 100.00% | 96.35% | 92.05% | 99.96% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.21%\**</span> | <span title="validated on-demand with error">*1.71%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*2.70%\**</span> | <span title="unvalidated">*49.94%\**</span> <span title="validated upfront with error">*49.75%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*23.49%\**</span> | <span title="unvalidated">*66.43%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*925.37 µs\**</span> <span title="prepend">*835.95 µs\**</span> | 3.1063 ms | 1.7539 ms | 489348 | 281173 | 249360 | 2.5892 ms |
| [bincode 2.0.0-rc][bincode] | 268.33 µs | 1.8644 ms | 809.89 µs | 367413 | 221291 | 206242 | 2.0253 ms |
| [bincode 1.3.3][bincode1] | 601.63 µs | 1.8069 ms | 854.41 µs | 569975 | 240525 | 231884 | 2.4295 ms |
| [bitcode 0.6.5][bitcode] | 145.90 µs | 1.2532 ms | 169.99 µs | 327688 | 200947 | 182040 | 736.28 µs |
| [borsh 1.5.5][borsh] | 536.98 µs | 1.8076 ms | † | 446595 | 234236 | 209834 | 2.0536 ms |
| [capnp 0.20.6][capnp] | 451.06 µs | † | † | 803896 | 335606 | 280744 | 3.5265 ms |
| [cbor4ii 1.0.0][cbor4ii] | 817.57 µs | 4.8467 ms | 3.5298 ms | 1109831 | 344745 | 274333 | 3.4167 ms |
| [ciborium 0.2.2][ciborium] | 3.7109 ms | 10.415 ms | † | 1109821 | 344751 | 274345 | 3.4405 ms |
| [databuf 0.5.0][databuf] | 309.82 µs | 1.7381 ms | 805.74 µs | 356311 | 213062 | 198403 | 1.9301 ms |
| [dlhn 0.1.7][dlhn] | 768.42 µs | 2.5824 ms | † | 366496 | 220600 | 205586 | 2.0005 ms |
| [flatbuffers 25.2.10][flatbuffers] | 3.2311 ms | † | † | 844168 | 345696 | 293916 | 3.4352 ms |
| [minicbor 0.26.1][minicbor] | 566.04 µs | 3.3783 ms | 1.8794 ms | 428773 | 249857 | 228630 | 2.2540 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0074 ms | 4.1168 ms | 2.9932 ms | 449745 | 252432 | 230965 | 2.2927 ms |
| [nanoserde 0.1.37][nanoserde] | 273.09 µs | 1.9309 ms | † | 567975 | 239930 | 231872 | 2.4902 ms |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 636.63 µs | 2.1455 ms | † | 356311 | 212976 | 198423 | 1.9372 ms |
| [postcard 1.1.1][postcard] | 451.57 µs | 2.0842 ms | 818.89 µs | 367489 | 221913 | 207244 | 2.0050 ms |
| [pot 3.0.1][pot] | 2.3831 ms | 6.1643 ms | 5.0340 ms | 599125 | 299158 | 247675 | 2.7257 ms |
| [prost 0.13.5][prost] | <span title="encode">*1.1110 ms\**</span> <span title="populate + encode">*2.8603 ms\**</span> | 3.4545 ms | † | 596811 | 305319 | 268737 | 2.9623 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.0255 ms\**</span> <span title="populate + encode">*2.9747 ms\**</span> | 3.7322 ms | † | 596811 | 305319 | 268737 | 3.0105 ms |
| [rkyv 0.8.10][rkyv] | 348.79 µs | <span title="unvalidated">*1.5088 ms\**</span> <span title="validated upfront with error">*1.8886 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3479 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5070 ms | 3.1090 ms | 1.6996 ms | 424533 | 245214 | 226077 | 2.2448 ms |
| [ron 0.8.1][ron] | 7.2287 ms | 17.043 ms | 15.258 ms | 1465223 | 434935 | 342907 | 5.5139 ms |
| [savefile 0.18.5][savefile] | 213.42 µs | 1.8417 ms | † | 566991 | 239362 | 231478 | 2.4416 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3738 ms | 5.4107 ms | 3.8081 ms | 1276014 | 373898 | 293384 | 3.6116 ms |
| [serde_bare 0.5.0][serde_bare] | 764.54 µs | 2.3547 ms | † | 356311 | 213062 | 198403 | 1.9476 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8722 ms | 4.7509 ms | 3.6251 ms | 1109821 | 344751 | 274345 | 3.4220 ms |
| [serde_json 1.0.140][serde_json] | 3.6687 ms | 6.7913 ms | † | 1623191 | 466527 | 359157 | 5.7067 ms |
| [simd-json 0.14.3][simd-json] | 2.2034 ms | 4.6191 ms | † | 1623191 | 466527 | 359157 | 5.6864 ms |
| [speedy 0.8.7][speedy] | 256.98 µs | 1.6850 ms | 560.40 µs | 449595 | 234970 | 210192 | 2.0774 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*73.907 ns\**</span> | <span title="validated on-demand with error">*411.87 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4860 ns\**</span> <span title="validated upfront with error">*2.4021 ms\**</span> | <span title="unvalidated">*1.3626 µs\**</span> <span title="validated upfront with error">*2.4012 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2440 ns\**</span> <span title="validated upfront with error">*373.11 µs\**</span> | <span title="unvalidated">*240.35 ns\**</span> <span title="validated upfront with error">*375.08 µs\**</span> | <span title="unvalidated">*744.31 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*15.77%\**</span> <span title="prepend">*17.45%\**</span> | 40.34% | 9.69% | 66.96% | 71.47% | 73.00% | 28.44% |
| [bincode 2.0.0-rc][bincode] | 54.37% | 67.22% | 20.99% | 89.19% | 90.81% | 88.27% | 36.35% |
| [bincode 1.3.3][bincode1] | 24.25% | 69.36% | 19.90% | 57.49% | 83.55% | 78.50% | 30.31% |
| [bitcode 0.6.5][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 27.17% | 69.33% | † | 73.37% | 85.79% | 86.75% | 35.85% |
| [capnp 0.20.6][capnp] | 32.35% | † | † | 40.76% | 59.88% | 64.84% | 20.88% |
| [cbor4ii 1.0.0][cbor4ii] | 17.85% | 25.86% | 4.82% | 29.53% | 58.29% | 66.36% | 21.55% |
| [ciborium 0.2.2][ciborium] | 3.93% | 12.03% | † | 29.53% | 58.29% | 66.35% | 21.40% |
| [databuf 0.5.0][databuf] | 47.09% | 72.10% | 21.10% | 91.97% | 94.31% | 91.75% | 38.15% |
| [dlhn 0.1.7][dlhn] | 18.99% | 48.53% | † | 89.41% | 91.09% | 88.55% | 36.80% |
| [flatbuffers 25.2.10][flatbuffers] | 4.52% | † | † | 38.82% | 58.13% | 61.94% | 21.43% |
| [minicbor 0.26.1][minicbor] | 25.78% | 37.10% | 9.04% | 76.42% | 80.42% | 79.62% | 32.67% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.91% | 30.44% | 5.68% | 72.86% | 79.60% | 78.82% | 32.11% |
| [nanoserde 0.1.37][nanoserde] | 53.43% | 64.90% | † | 57.69% | 83.75% | 78.51% | 29.57% |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 22.92% | 58.41% | † | 91.97% | 94.35% | 91.74% | 38.01% |
| [postcard 1.1.1][postcard] | 32.31% | 60.13% | 20.76% | 89.17% | 90.55% | 87.84% | 36.72% |
| [pot 3.0.1][pot] | 6.12% | 20.33% | 3.38% | 54.69% | 67.17% | 73.50% | 27.01% |
| [prost 0.13.5][prost] | <span title="encode">*13.13%\**</span> <span title="populate + encode">*5.10%\**</span> | 36.28% | † | 54.91% | 65.82% | 67.74% | 24.86% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*14.23%\**</span> <span title="populate + encode">*4.90%\**</span> | 33.58% | † | 54.91% | 65.82% | 67.74% | 24.46% |
| [rkyv 0.8.10][rkyv] | 41.83% | <span title="unvalidated">*83.06%\**</span> <span title="validated upfront with error">*66.36%\**</span> | † | 54.27% | 78.87% | 82.96% | 31.36% |
| [rmp-serde 1.3.0][rmp-serde] | 9.68% | 40.31% | 10.00% | 77.19% | 81.95% | 80.52% | 32.80% |
| [ron 0.8.1][ron] | 2.02% | 7.35% | 1.11% | 22.36% | 46.20% | 53.09% | 13.35% |
| [savefile 0.18.5][savefile] | 68.36% | 68.05% | † | 57.79% | 83.95% | 78.64% | 30.16% |
| [serde-brief 0.1.1][serde-brief] | 10.62% | 23.16% | 4.46% | 25.68% | 53.74% | 62.05% | 20.39% |
| [serde_bare 0.5.0][serde_bare] | 19.08% | 53.22% | † | 91.97% | 94.31% | 91.75% | 37.80% |
| [serde_cbor 0.11.2][serde_cbor] | 7.79% | 26.38% | 4.69% | 29.53% | 58.29% | 66.35% | 21.52% |
| [serde_json 1.0.140][serde_json] | 3.98% | 18.45% | † | 20.19% | 43.07% | 50.69% | 12.90% |
| [simd-json 0.14.3][simd-json] | 6.62% | 27.13% | † | 20.19% | 43.07% | 50.69% | 12.95% |
| [speedy 0.8.7][speedy] | 56.77% | 74.37% | 30.33% | 72.89% | 85.52% | 86.61% | 35.44% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*58.36%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.04%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.64%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*4.5372 ms\**</span> <span title="prepend">*2.5866 ms\**</span> | 8.3713 ms | 1704643 | 1294259 | 1245668 | 11.699 ms |
| [bincode 2.0.0-rc][bincode] | 1.4138 ms | 3.8050 ms | 1406257 | 1117802 | 1062438 | 9.6633 ms |
| [bincode 1.3.3][bincode1] | 3.9396 ms | 4.1531 ms | 1854234 | 1141994 | 1048745 | 10.371 ms |
| [bitcode 0.6.5][bitcode] | 726.86 µs | 2.3509 ms | 971318 | 878034 | 850340 | 2.9521 ms |
| [borsh 1.5.5][borsh] | 2.8709 ms | 2.8689 ms | 1521989 | 1108471 | 1038528 | 9.9788 ms |
| [capnp 0.20.6][capnp] | 2.1663 ms | † | 2724288 | 1546992 | 1239111 | 14.558 ms |
| [cbor4ii 1.0.0][cbor4ii] | 3.0555 ms | 18.453 ms | 6012539 | 1695215 | 1464951 | 22.096 ms |
| [ciborium 0.2.2][ciborium] | 23.302 ms | 55.434 ms | 6012373 | 1695146 | 1465025 | 21.324 ms |
| [databuf 0.5.0][databuf] | 1.3166 ms | 3.7797 ms | 1319999 | 1062631 | 1008334 | 9.0268 ms |
| [dlhn 0.1.7][dlhn] | 4.8135 ms | 6.2164 ms | 1311281 | 1077520 | 1046095 | 8.6683 ms |
| [flatbuffers 25.2.10][flatbuffers] | 5.1508 ms | † | 2325620 | 1440289 | 1264800 | 13.552 ms |
| [minicbor 0.26.1][minicbor] | 2.4211 ms | 11.258 ms | 1777386 | 1276218 | 1252558 | 12.607 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.268 ms | 17.981 ms | 1770060 | 1277755 | 1263362 | 12.591 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2757 ms | 2.9164 ms | 1812404 | 1134820 | 1053109 | 10.410 ms |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 3.1404 ms | 3.3743 ms | 1319999 | 1064380 | 1010708 | 8.9068 ms |
| [postcard 1.1.1][postcard] | 1.9609 ms | 4.2390 ms | 1311281 | 1083900 | 1041434 | 8.8095 ms |
| [pot 3.0.1][pot] | 14.084 ms | 30.755 ms | 2604812 | 1482233 | 1298928 | 16.110 ms |
| [prost 0.13.5][prost] | <span title="encode">*5.2057 ms\**</span> <span title="populate + encode">*9.0035 ms\**</span> | 8.6147 ms | 1859886 | 1338076 | 1295351 | 12.303 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*5.4557 ms\**</span> <span title="populate + encode">*12.618 ms\**</span> | 12.153 ms | 1859886 | 1338076 | 1295351 | 12.173 ms |
| [rkyv 0.8.10][rkyv] | 932.20 µs | <span title="unvalidated">*2.1565 ms\**</span> <span title="validated upfront with error">*2.6030 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.081 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.990 ms | 10.961 ms | 1745322 | 1261627 | 1228923 | 11.560 ms |
| [ron 0.8.1][ron] | 37.824 ms | 88.279 ms | 8677703 | 2233642 | 1826180 | 34.489 ms |
| [savefile 0.18.5][savefile] | 865.97 µs | 2.7307 ms | 1791505 | 1128012 | 1051153 | 10.349 ms |
| [serde-brief 0.1.1][serde-brief] | 6.9672 ms | 21.632 ms | 6951772 | 1796265 | 1567819 | 23.331 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8734 ms | 4.7577 ms | 1319999 | 1062645 | 1008349 | 8.8543 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.298 ms | 22.021 ms | 6012373 | 1695146 | 1465025 | 21.099 ms |
| [serde_json 1.0.140][serde_json] | 20.338 ms | 31.254 ms | 9390461 | 2391679 | 1842767 | 34.633 ms |
| [simd-json 0.14.3][simd-json] | 11.619 ms | 25.786 ms | 9390461 | 2391679 | 1842767 | 34.656 ms |
| [speedy 0.8.7][speedy] | 775.83 µs | 2.4443 ms | 1584734 | 1119837 | 1037992 | 10.109 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*74.785 ns\**</span> | <span title="validated on-demand with error">*713.03 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4876 ns\**</span> <span title="validated upfront with error">*5.6443 ms\**</span> | <span title="unvalidated">*2.6046 µs\**</span> <span title="validated upfront with error">*5.6446 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2432 ns\**</span> <span title="validated upfront with error">*441.04 µs\**</span> | <span title="unvalidated">*409.63 ns\**</span> <span title="validated upfront with error">*442.20 µs\**</span> | <span title="unvalidated">*237.03 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*16.02%\**</span> <span title="prepend">*28.10%\**</span> | 25.76% | 56.98% | 67.84% | 68.26% | 25.23% |
| [bincode 2.0.0-rc][bincode] | 51.41% | 56.68% | 69.07% | 78.55% | 80.04% | 30.55% |
| [bincode 1.3.3][bincode1] | 18.45% | 51.93% | 52.38% | 76.89% | 81.08% | 28.47% |
| [bitcode 0.6.5][bitcode] | 100.00% | 91.73% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 25.32% | 75.17% | 63.82% | 79.21% | 81.88% | 29.58% |
| [capnp 0.20.6][capnp] | 33.55% | † | 35.65% | 56.76% | 68.63% | 20.28% |
| [cbor4ii 1.0.0][cbor4ii] | 23.79% | 11.69% | 16.15% | 51.79% | 58.05% | 13.36% |
| [ciborium 0.2.2][ciborium] | 3.12% | 3.89% | 16.16% | 51.80% | 58.04% | 13.84% |
| [databuf 0.5.0][databuf] | 55.21% | 57.05% | 73.58% | 82.63% | 84.33% | 32.70% |
| [dlhn 0.1.7][dlhn] | 15.10% | 34.69% | 74.07% | 81.49% | 81.29% | 34.06% |
| [flatbuffers 25.2.10][flatbuffers] | 14.11% | † | 41.77% | 60.96% | 67.23% | 21.78% |
| [minicbor 0.26.1][minicbor] | 30.02% | 19.16% | 54.65% | 68.80% | 67.89% | 23.42% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.48% | 11.99% | 54.87% | 68.72% | 67.31% | 23.45% |
| [nanoserde 0.1.37][nanoserde] | 56.98% | 73.94% | 53.59% | 77.37% | 80.75% | 28.36% |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 23.15% | 63.91% | 73.58% | 82.49% | 84.13% | 33.14% |
| [postcard 1.1.1][postcard] | 37.07% | 50.87% | 74.07% | 81.01% | 81.65% | 33.51% |
| [pot 3.0.1][pot] | 5.16% | 7.01% | 37.29% | 59.24% | 65.46% | 18.32% |
| [prost 0.13.5][prost] | <span title="encode">*13.96%\**</span> <span title="populate + encode">*8.07%\**</span> | 25.03% | 52.22% | 65.62% | 65.65% | 23.99% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*13.32%\**</span> <span title="populate + encode">*5.76%\**</span> | 17.74% | 52.22% | 65.62% | 65.65% | 24.25% |
| [rkyv 0.8.10][rkyv] | 77.97% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*82.85%\**</span> | 46.79% | 63.45% | 70.25% | 22.57% |
| [rmp-serde 1.3.0][rmp-serde] | 6.61% | 19.67% | 55.65% | 69.60% | 69.19% | 25.54% |
| [ron 0.8.1][ron] | 1.92% | 2.44% | 11.19% | 39.31% | 46.56% | 8.56% |
| [savefile 0.18.5][savefile] | 83.94% | 78.97% | 54.22% | 77.84% | 80.90% | 28.53% |
| [serde-brief 0.1.1][serde-brief] | 10.43% | 9.97% | 13.97% | 48.88% | 54.24% | 12.65% |
| [serde_bare 0.5.0][serde_bare] | 14.91% | 45.33% | 73.58% | 82.63% | 84.33% | 33.34% |
| [serde_cbor 0.11.2][serde_cbor] | 7.06% | 9.79% | 16.16% | 51.80% | 58.04% | 13.99% |
| [serde_json 1.0.140][serde_json] | 3.57% | 6.90% | 10.34% | 36.71% | 46.14% | 8.52% |
| [simd-json 0.14.3][simd-json] | 6.26% | 8.36% | 10.34% | 36.71% | 46.14% | 8.52% |
| [speedy 0.8.7][speedy] | 93.69% | 88.23% | 61.29% | 78.41% | 81.92% | 29.20% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*57.45%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*15.73%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1012.3
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.5
[borsh]: https://crates.io/crates/borsh/1.5.5
[capnp]: https://crates.io/crates/capnp/0.20.6
[cbor4ii]: https://crates.io/crates/cbor4ii/1.0.0
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.2.10
[minicbor]: https://crates.io/crates/minicbor/0.26.1
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.7.4
[postcard]: https://crates.io/crates/postcard/1.1.1
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.13.5
[protobuf]: https://crates.io/crates/protobuf/3.7.1
[rkyv]: https://crates.io/crates/rkyv/0.8.10
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.18.5
[serde-brief]: https://crates.io/crates/serde-brief/0.1.1
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.140
[simd-json]: https://crates.io/crates/simd-json/0.14.3
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
