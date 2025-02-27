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

## Last updated: 2025-02-27 13:51:14

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.87.0-nightly (00f245915 2025-02-26)
binary: rustc
commit-hash: 00f245915b0c7839d42c26f9628220c4f1b93bf6
commit-date: 2025-02-26
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

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*495.86 µs\**</span> <span title="prepend">*430.99 µs\**</span> | 2.7176 ms | 890.66 µs | 804955 | 328941 | 284849 | 4.0743 ms |
| [bincode 2.0.0-rc][bincode] | 271.37 µs | 2.5758 ms | † | 741295 | 303944 | 256422 | 3.5886 ms |
| [bincode 1.3.3][bincode1] | 547.13 µs | 2.0366 ms | 626.73 µs | 1045784 | 373127 | 311553 | 4.4063 ms |
| [bitcode 0.6.4][bitcode] | 145.73 µs | 1.4502 ms | 60.293 µs | 703710 | 288826 | 227322 | 2.3355 ms |
| [borsh 1.5.3][borsh] | 554.92 µs | 2.1465 ms | † | 885780 | 362204 | 286248 | 3.8268 ms |
| [capnp 0.20.3][capnp] | 507.30 µs | † | † | 1443216 | 513986 | 426532 | 6.1444 ms |
| [cbor4ii 0.3.3][cbor4ii] | 707.38 µs | 5.1556 ms | 3.4459 ms | 1407835 | 403440 | 323561 | 5.1362 ms |
| [ciborium 0.2.2][ciborium] | 4.0563 ms | 12.004 ms | † | 1407835 | 403440 | 323561 | 4.9212 ms |
| [databuf 0.5.0][databuf] | 262.59 µs | 2.0634 ms | 676.77 µs | 765778 | 311715 | 263914 | 3.4331 ms |
| [dlhn 0.1.7][dlhn] | 745.50 µs | 2.5618 ms | † | 724953 | 301446 | 253056 | 3.2165 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0246 ms | † | † | 1276368 | 468539 | 388381 | 4.7610 ms |
| [minicbor 0.25.1][minicbor] | 653.92 µs | 2.9574 ms | 1.4493 ms | 817830 | 332671 | 284034 | 4.3264 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4001 ms | 4.0805 ms | 2.6535 ms | 818669 | 332556 | 284797 | 4.1119 ms |
| [nanoserde 0.1.37][nanoserde] | 245.08 µs | 2.0927 ms | † | 1045784 | 373127 | 311553 | 4.2014 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 655.68 µs | 2.1794 ms | † | 765778 | 311743 | 263822 | 3.4889 ms |
| [postcard 1.1.1][postcard] | 428.30 µs | 2.2649 ms | 898.78 µs | 724953 | 302399 | 252968 | 3.1628 ms |
| [pot 3.0.1][pot] | 2.3786 ms | 6.5230 ms | 4.9104 ms | 971922 | 372513 | 303636 | 4.2738 ms |
| [prost 0.13.4][prost] | <span title="encode">*938.84 µs\**</span> <span title="populate + encode">*2.4224 ms\**</span> | 3.5434 ms | † | 884628 | 363130 | 314959 | 4.3771 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.1456 ms\**</span> <span title="populate + encode">*2.9176 ms\**</span> | 3.9356 ms | † | 884628 | 363130 | 314959 | 4.4967 ms |
| [rkyv 0.8.9][rkyv] | 242.07 µs | <span title="unvalidated">*1.5483 ms\**</span> <span title="validated upfront with error">*1.9351 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5393 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5068 ms | 3.0875 ms | 1.4426 ms | 784997 | 325384 | 277608 | 3.7450 ms |
| [ron 0.8.1][ron] | 9.6519 ms | 14.979 ms | 13.839 ms | 1607459 | 449158 | 349324 | 5.4926 ms |
| [savefile 0.18.5][savefile] | 188.80 µs | 2.1601 ms | † | 1045800 | 373139 | 311562 | 4.2967 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5285 ms | 4.6560 ms | 3.0139 ms | 1584946 | 413733 | 339964 | 4.8636 ms |
| [serde_bare 0.5.0][serde_bare] | 696.08 µs | 2.1128 ms | † | 765778 | 311715 | 263914 | 3.5798 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0778 ms | 4.5459 ms | 3.2040 ms | 1407835 | 403440 | 323561 | 4.7145 ms |
| [serde_json 1.0.134][serde_json] | 3.7889 ms | 5.7985 ms | † | 1827461 | 470560 | 360727 | 5.4685 ms |
| [simd-json 0.14.3][simd-json] | 2.0828 ms | 4.6716 ms | † | 1827461 | 470560 | 360727 | 5.5009 ms |
| [speedy 0.8.7][speedy] | 197.69 µs | 1.8436 ms | 366.71 µs | 885780 | 362204 | 286248 | 3.9028 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*75.264 ns\**</span> | <span title="validated on-demand with error">*172.44 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4861 ns\**</span> <span title="validated upfront with error">*2.0654 ms\**</span> | <span title="unvalidated">*51.151 µs\**</span> <span title="validated upfront with error">*2.0634 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*376.75 µs\**</span> | <span title="unvalidated">*10.517 µs\**</span> <span title="validated upfront with error">*388.98 µs\**</span> | <span title="unvalidated">*7.5718 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*29.39%\**</span> <span title="prepend">*33.81%\**</span> | 53.36% | 6.77% | 87.42% | 87.80% | 79.80% | 57.32% |
| [bincode 2.0.0-rc][bincode] | 53.70% | 56.30% | † | 94.93% | 95.03% | 88.65% | 65.08% |
| [bincode 1.3.3][bincode1] | 26.64% | 71.21% | 9.62% | 67.29% | 77.41% | 72.96% | 53.00% |
| [bitcode 0.6.4][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 26.26% | 67.56% | † | 79.45% | 79.74% | 79.41% | 61.03% |
| [capnp 0.20.3][capnp] | 28.73% | † | † | 48.76% | 56.19% | 53.30% | 38.01% |
| [cbor4ii 0.3.3][cbor4ii] | 20.60% | 28.13% | 1.75% | 49.99% | 71.59% | 70.26% | 45.47% |
| [ciborium 0.2.2][ciborium] | 3.59% | 12.08% | † | 49.99% | 71.59% | 70.26% | 47.46% |
| [databuf 0.5.0][databuf] | 55.50% | 70.28% | 8.91% | 91.89% | 92.66% | 86.13% | 68.03% |
| [dlhn 0.1.7][dlhn] | 19.55% | 56.61% | † | 97.07% | 95.81% | 89.83% | 72.61% |
| [flatbuffers 24.12.23][flatbuffers] | 14.22% | † | † | 55.13% | 61.64% | 58.53% | 49.05% |
| [minicbor 0.25.1][minicbor] | 22.29% | 49.04% | 4.16% | 86.05% | 86.82% | 80.03% | 53.98% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.70% | 35.54% | 2.27% | 85.96% | 86.85% | 79.82% | 56.80% |
| [nanoserde 0.1.37][nanoserde] | 59.46% | 69.30% | † | 67.29% | 77.41% | 72.96% | 55.59% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.23% | 66.54% | † | 91.89% | 92.65% | 86.16% | 66.94% |
| [postcard 1.1.1][postcard] | 34.03% | 64.03% | 6.71% | 97.07% | 95.51% | 89.86% | 73.84% |
| [pot 3.0.1][pot] | 6.13% | 22.23% | 1.23% | 72.40% | 77.53% | 74.87% | 54.65% |
| [prost 0.13.4][prost] | <span title="encode">*15.52%\**</span> <span title="populate + encode">*6.02%\**</span> | 40.93% | † | 79.55% | 79.54% | 72.18% | 53.36% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*12.72%\**</span> <span title="populate + encode">*4.99%\**</span> | 36.85% | † | 79.55% | 79.54% | 72.18% | 51.94% |
| [rkyv 0.8.9][rkyv] | 60.20% | <span title="unvalidated">*93.66%\**</span> <span title="validated upfront with error">*74.94%\**</span> | † | 69.57% | 73.39% | 69.74% | 51.45% |
| [rmp-serde 1.3.0][rmp-serde] | 9.67% | 46.97% | 4.18% | 89.64% | 88.76% | 81.89% | 62.36% |
| [ron 0.8.1][ron] | 1.51% | 9.68% | 0.44% | 43.78% | 64.30% | 65.07% | 42.52% |
| [savefile 0.18.5][savefile] | 77.19% | 67.14% | † | 67.29% | 77.40% | 72.96% | 54.36% |
| [serde-brief 0.1.0][serde-brief] | 9.53% | 31.15% | 2.00% | 44.40% | 69.81% | 66.87% | 48.02% |
| [serde_bare 0.5.0][serde_bare] | 20.94% | 68.64% | † | 91.89% | 92.66% | 86.13% | 65.24% |
| [serde_cbor 0.11.2][serde_cbor] | 7.01% | 31.90% | 1.88% | 49.99% | 71.59% | 70.26% | 49.54% |
| [serde_json 1.0.134][serde_json] | 3.85% | 25.01% | † | 38.51% | 61.38% | 63.02% | 42.71% |
| [simd-json 0.14.3][simd-json] | 7.00% | 31.04% | † | 38.51% | 61.38% | 63.02% | 42.46% |
| [speedy 0.8.7][speedy] | 73.72% | 78.66% | 16.44% | 79.45% | 79.74% | 79.41% | 59.84% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.65%\**</span> | <span title="validated on-demand with error">*6.10%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.56%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.70%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*6.7440 ms\**</span> <span title="prepend">*8.8321 ms\**</span> | 7.9798 ms | 8625005 | 6443961 | 6231572 | 73.581 ms |
| [bincode 2.0.0-rc][bincode] | 2.4127 ms | 1.0256 ms | 6000005 | 5378497 | 5346882 | 8.7841 ms |
| [bincode 1.3.3][bincode1] | 5.1636 ms | 964.07 µs | 6000008 | 5378500 | 5346908 | 8.8169 ms |
| [bitcode 0.6.4][bitcode] | 1.4644 ms | 801.42 µs | 6000006 | 5182295 | 4921841 | 14.244 ms |
| [borsh 1.5.3][borsh] | 5.9375 ms | 4.2362 ms | 6000004 | 5378496 | 5346866 | 8.9259 ms |
| [capnp 0.20.3][capnp] | 6.7056 ms | † | 14000088 | 7130367 | 6046182 | 83.690 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.9244 ms | 52.301 ms | 13125016 | 7524114 | 6757437 | 93.567 ms |
| [ciborium 0.2.2][ciborium] | 73.969 ms | 119.77 ms | 13122324 | 7524660 | 6759128 | 93.292 ms |
| [databuf 0.5.0][databuf] | 2.4128 ms | 5.2888 ms | 6000003 | 5378495 | 5346897 | 8.9158 ms |
| [dlhn 0.1.7][dlhn] | 6.2987 ms | 7.0106 ms | 6000003 | 5378495 | 5346897 | 8.9180 ms |
| [flatbuffers 24.12.23][flatbuffers] | 883.50 µs | † | 6000024 | 5378434 | 5346878 | 9.0914 ms |
| [minicbor 0.25.1][minicbor] | 6.0506 ms | 11.486 ms | 8125006 | 6494907 | 6390894 | 73.011 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 117.63 ms | 32.356 ms | 8125037 | 6493484 | 6386940 | 72.069 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2684 ms | 1.1092 ms | 6000008 | 5378500 | 5346908 | 8.8954 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1146 ms | 3.9843 ms | 6000004 | 5378496 | 5346866 | 8.9412 ms |
| [postcard 1.1.1][postcard] | 479.27 µs | 1.1614 ms | 6000003 | 5378495 | 5346897 | 8.9327 ms |
| [pot 3.0.1][pot] | 39.229 ms | 70.947 ms | 10122342 | 6814618 | 6852252 | 83.894 ms |
| [prost 0.13.4][prost] | <span title="encode">*7.8194 ms\**</span> <span title="populate + encode">*8.4732 ms\**</span> | 14.608 ms | 8750000 | 6665735 | 6421877 | 74.493 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*14.055 ms\**</span> <span title="populate + encode">*30.336 ms\**</span> | 32.070 ms | 8750000 | 6665735 | 6421877 | 80.981 ms |
| [rkyv 0.8.9][rkyv] | 195.87 µs | <span title="unvalidated">*200.96 µs\**</span> <span title="validated upfront with error">*201.04 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.7485 ms |
| [rmp-serde 1.3.0][rmp-serde] | 18.441 ms | 17.417 ms | 8125006 | 6494876 | 6391037 | 80.576 ms |
| [ron 0.8.1][ron] | 175.08 ms | 239.64 ms | 22192885 | 8970395 | 8137334 | 151.19 ms |
| [savefile 0.18.5][savefile] | 187.13 µs | 199.59 µs | 6000024 | 5378519 | 5346896 | 8.9666 ms |
| [serde-brief 0.1.0][serde-brief] | 22.743 ms | 40.922 ms | 15750015 | 8024540 | 6813667 | 95.600 ms |
| [serde_bare 0.5.0][serde_bare] | 5.1929 ms | 4.8207 ms | 6000003 | 5378495 | 5346897 | 8.7445 ms |
| [serde_cbor 0.11.2][serde_cbor] | 36.307 ms | 45.598 ms | 13122324 | 7524660 | 6759128 | 92.092 ms |
| [serde_json 1.0.134][serde_json] | 87.767 ms | 90.218 ms | 26192883 | 9566084 | 8584671 | 159.64 ms |
| [simd-json 0.14.3][simd-json] | 51.707 ms | 70.099 ms | 26192883 | 9566084 | 8584671 | 159.60 ms |
| [speedy 0.8.7][speedy] | 190.99 µs | 184.08 µs | 6000004 | 5378496 | 5346866 | 8.6737 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*104.33 ns\**</span> | <span title="validated on-demand with error">*2.3215 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4940 ns\**</span> <span title="validated upfront with error">*40.120 ns\**</span> | <span title="unvalidated">*52.541 µs\**</span> <span title="validated upfront with error">*77.872 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2466 ns\**</span> <span title="validated upfront with error">*4.9903 ns\**</span> | <span title="unvalidated">*58.426 µs\**</span> <span title="validated upfront with error">*38.940 µs\**</span> | <span title="unvalidated">*101.64 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*2.77%\**</span> <span title="prepend">*2.12%\**</span> | 2.31% | 69.57% | 80.42% | 78.98% | 11.79% |
| [bincode 2.0.0-rc][bincode] | 7.76% | 17.95% | 100.00% | 96.35% | 92.05% | 98.74% |
| [bincode 1.3.3][bincode1] | 3.62% | 19.09% | 100.00% | 96.35% | 92.05% | 98.38% |
| [bitcode 0.6.4][bitcode] | 12.78% | 22.97% | 100.00% | 100.00% | 100.00% | 60.89% |
| [borsh 1.5.3][borsh] | 3.15% | 4.35% | 100.00% | 96.35% | 92.05% | 97.17% |
| [capnp 0.20.3][capnp] | 2.79% | † | 42.86% | 72.68% | 81.40% | 10.36% |
| [cbor4ii 0.3.3][cbor4ii] | 1.89% | 0.35% | 45.71% | 68.88% | 72.84% | 9.27% |
| [ciborium 0.2.2][ciborium] | 0.25% | 0.15% | 45.72% | 68.87% | 72.82% | 9.30% |
| [databuf 0.5.0][databuf] | 7.76% | 3.48% | 100.00% | 96.35% | 92.05% | 97.28% |
| [dlhn 0.1.7][dlhn] | 2.97% | 2.63% | 100.00% | 96.35% | 92.05% | 97.26% |
| [flatbuffers 24.12.23][flatbuffers] | 21.18% | † | 100.00% | 96.35% | 92.05% | 95.41% |
| [minicbor 0.25.1][minicbor] | 3.09% | 1.60% | 73.85% | 79.79% | 77.01% | 11.88% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.57% | 73.85% | 79.81% | 77.06% | 12.04% |
| [nanoserde 0.1.37][nanoserde] | 14.75% | 16.60% | 100.00% | 96.35% | 92.05% | 97.51% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.66% | 4.62% | 100.00% | 96.35% | 92.05% | 97.01% |
| [postcard 1.1.1][postcard] | 39.04% | 15.85% | 100.00% | 96.35% | 92.05% | 97.10% |
| [pot 3.0.1][pot] | 0.48% | 0.26% | 59.27% | 76.05% | 71.83% | 10.34% |
| [prost 0.13.4][prost] | <span title="encode">*2.39%\**</span> <span title="populate + encode">*2.21%\**</span> | 1.26% | 68.57% | 77.75% | 76.64% | 11.64% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.33%\**</span> <span title="populate + encode">*0.62%\**</span> | 0.57% | 68.57% | 77.75% | 76.64% | 10.71% |
| [rkyv 0.8.9][rkyv] | 95.54% | <span title="unvalidated">*91.60%\**</span> <span title="validated upfront with error">*91.56%\**</span> | 100.00% | 96.35% | 92.05% | 99.14% |
| [rmp-serde 1.3.0][rmp-serde] | 1.01% | 1.06% | 73.85% | 79.79% | 77.01% | 10.76% |
| [ron 0.8.1][ron] | 0.11% | 0.08% | 27.04% | 57.77% | 60.48% | 5.74% |
| [savefile 0.18.5][savefile] | 100.00% | 92.23% | 100.00% | 96.35% | 92.05% | 96.73% |
| [serde-brief 0.1.0][serde-brief] | 0.82% | 0.45% | 38.10% | 64.58% | 72.23% | 9.07% |
| [serde_bare 0.5.0][serde_bare] | 3.60% | 3.82% | 100.00% | 96.35% | 92.05% | 99.19% |
| [serde_cbor 0.11.2][serde_cbor] | 0.52% | 0.40% | 45.72% | 68.87% | 72.82% | 9.42% |
| [serde_json 1.0.134][serde_json] | 0.21% | 0.20% | 22.91% | 54.17% | 57.33% | 5.43% |
| [simd-json 0.14.3][simd-json] | 0.36% | 0.26% | 22.91% | 54.17% | 57.33% | 5.43% |
| [speedy 0.8.7][speedy] | 97.98% | 100.00% | 100.00% | 96.35% | 92.05% | 100.00% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.19%\**</span> | <span title="validated on-demand with error">*1.68%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*3.11%\**</span> | <span title="unvalidated">*74.11%\**</span> <span title="validated upfront with error">*50.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.98%\**</span> | <span title="unvalidated">*66.65%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*922.41 µs\**</span> <span title="prepend">*839.49 µs\**</span> | 3.2147 ms | 1.7642 ms | 489348 | 281173 | 249360 | 2.6808 ms |
| [bincode 2.0.0-rc][bincode] | 263.19 µs | 2.1820 ms | † | 367413 | 221291 | 206242 | 2.0275 ms |
| [bincode 1.3.3][bincode1] | 600.10 µs | 1.8118 ms | 873.31 µs | 569975 | 240525 | 231884 | 2.5055 ms |
| [bitcode 0.6.4][bitcode] | 128.29 µs | 1.2565 ms | 170.99 µs | 327688 | 200947 | 182040 | 749.08 µs |
| [borsh 1.5.3][borsh] | 544.22 µs | 1.8135 ms | † | 446595 | 234236 | 209834 | 2.0675 ms |
| [capnp 0.20.3][capnp] | 440.01 µs | † | † | 803896 | 335606 | 280744 | 3.5363 ms |
| [cbor4ii 0.3.3][cbor4ii] | 810.44 µs | 4.9176 ms | 3.6375 ms | 1109831 | 344745 | 274333 | 3.4829 ms |
| [ciborium 0.2.2][ciborium] | 3.7110 ms | 10.293 ms | † | 1109821 | 344751 | 274345 | 3.6470 ms |
| [databuf 0.5.0][databuf] | 310.99 µs | 1.7067 ms | 775.89 µs | 356311 | 213062 | 198403 | 2.0211 ms |
| [dlhn 0.1.7][dlhn] | 771.43 µs | 2.5964 ms | † | 366496 | 220600 | 205586 | 2.0707 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.2332 ms | † | † | 844168 | 345696 | 293916 | 3.6245 ms |
| [minicbor 0.25.1][minicbor] | 557.15 µs | 3.4117 ms | 1.9195 ms | 428773 | 249857 | 228630 | 2.3833 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0277 ms | 4.1354 ms | 2.9855 ms | 449745 | 252432 | 230965 | 2.4173 ms |
| [nanoserde 0.1.37][nanoserde] | 278.02 µs | 1.9562 ms | † | 567975 | 239930 | 231872 | 2.5355 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 619.76 µs | 1.9510 ms | † | 356311 | 212976 | 198423 | 1.9418 ms |
| [postcard 1.1.1][postcard] | 447.49 µs | 2.1026 ms | 815.94 µs | 367489 | 221913 | 207244 | 2.1160 ms |
| [pot 3.0.1][pot] | 2.3910 ms | 6.1911 ms | 5.1398 ms | 599125 | 299158 | 247675 | 2.7483 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.2821 ms\**</span> <span title="populate + encode">*3.0142 ms\**</span> | 3.7261 ms | † | 596811 | 305319 | 268737 | 3.0301 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.0062 ms\**</span> <span title="populate + encode">*2.9517 ms\**</span> | 3.9549 ms | † | 596811 | 305319 | 268737 | 2.9920 ms |
| [rkyv 0.8.9][rkyv] | 347.42 µs | <span title="unvalidated">*1.5014 ms\**</span> <span title="validated upfront with error">*1.8877 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3100 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5081 ms | 3.0763 ms | 1.7122 ms | 424533 | 245214 | 226077 | 2.3436 ms |
| [ron 0.8.1][ron] | 6.9034 ms | 16.881 ms | 15.232 ms | 1465223 | 434935 | 342907 | 5.5727 ms |
| [savefile 0.18.5][savefile] | 207.10 µs | 1.8372 ms | † | 566991 | 239362 | 231478 | 2.9373 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3503 ms | 5.2883 ms | 3.7186 ms | 1276014 | 373898 | 293384 | 3.6465 ms |
| [serde_bare 0.5.0][serde_bare] | 760.42 µs | 2.4126 ms | † | 356311 | 213062 | 198403 | 1.9320 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8864 ms | 4.7545 ms | 3.3615 ms | 1109821 | 344751 | 274345 | 3.4375 ms |
| [serde_json 1.0.134][serde_json] | 3.6937 ms | 6.6450 ms | † | 1623191 | 466527 | 359157 | 5.7557 ms |
| [simd-json 0.14.3][simd-json] | 2.2126 ms | 4.6060 ms | † | 1623191 | 466527 | 359157 | 5.7381 ms |
| [speedy 0.8.7][speedy] | 263.50 µs | 1.6074 ms | 583.68 µs | 449595 | 234970 | 210192 | 2.3608 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.625 ns\**</span> | <span title="validated on-demand with error">*564.15 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4859 ns\**</span> <span title="validated upfront with error">*2.2064 ms\**</span> | <span title="unvalidated">*1.3627 µs\**</span> <span title="validated upfront with error">*2.2169 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2433 ns\**</span> <span title="validated upfront with error">*371.28 µs\**</span> | <span title="unvalidated">*164.36 ns\**</span> <span title="validated upfront with error">*370.29 µs\**</span> | <span title="unvalidated">*776.21 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*13.91%\**</span> <span title="prepend">*15.28%\**</span> | 39.09% | 9.69% | 66.96% | 71.47% | 73.00% | 27.94% |
| [bincode 2.0.0-rc][bincode] | 48.74% | 57.58% | † | 89.19% | 90.81% | 88.27% | 36.95% |
| [bincode 1.3.3][bincode1] | 21.38% | 69.35% | 19.58% | 57.49% | 83.55% | 78.50% | 29.90% |
| [bitcode 0.6.4][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 23.57% | 69.29% | † | 73.37% | 85.79% | 86.75% | 36.23% |
| [capnp 0.20.3][capnp] | 29.16% | † | † | 40.76% | 59.88% | 64.84% | 21.18% |
| [cbor4ii 0.3.3][cbor4ii] | 15.83% | 25.55% | 4.70% | 29.53% | 58.29% | 66.36% | 21.51% |
| [ciborium 0.2.2][ciborium] | 3.46% | 12.21% | † | 29.53% | 58.29% | 66.35% | 20.54% |
| [databuf 0.5.0][databuf] | 41.25% | 73.62% | 22.04% | 91.97% | 94.31% | 91.75% | 37.06% |
| [dlhn 0.1.7][dlhn] | 16.63% | 48.39% | † | 89.41% | 91.09% | 88.55% | 36.18% |
| [flatbuffers 24.12.23][flatbuffers] | 3.97% | † | † | 38.82% | 58.13% | 61.94% | 20.67% |
| [minicbor 0.25.1][minicbor] | 23.03% | 36.83% | 8.91% | 76.42% | 80.42% | 79.62% | 31.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.55% | 30.38% | 5.73% | 72.86% | 79.60% | 78.82% | 30.99% |
| [nanoserde 0.1.37][nanoserde] | 46.14% | 64.23% | † | 57.69% | 83.75% | 78.51% | 29.54% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.70% | 64.40% | † | 91.97% | 94.35% | 91.74% | 38.58% |
| [postcard 1.1.1][postcard] | 28.67% | 59.76% | 20.96% | 89.17% | 90.55% | 87.84% | 35.40% |
| [pot 3.0.1][pot] | 5.37% | 20.30% | 3.33% | 54.69% | 67.17% | 73.50% | 27.26% |
| [prost 0.13.4][prost] | <span title="encode">*10.01%\**</span> <span title="populate + encode">*4.26%\**</span> | 33.72% | † | 54.91% | 65.82% | 67.74% | 24.72% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*12.75%\**</span> <span title="populate + encode">*4.35%\**</span> | 31.77% | † | 54.91% | 65.82% | 67.74% | 25.04% |
| [rkyv 0.8.9][rkyv] | 36.93% | <span title="unvalidated">*83.69%\**</span> <span title="validated upfront with error">*66.56%\**</span> | † | 54.27% | 78.87% | 82.96% | 32.43% |
| [rmp-serde 1.3.0][rmp-serde] | 8.51% | 40.84% | 9.99% | 77.19% | 81.95% | 80.52% | 31.96% |
| [ron 0.8.1][ron] | 1.86% | 7.44% | 1.12% | 22.36% | 46.20% | 53.09% | 13.44% |
| [savefile 0.18.5][savefile] | 61.95% | 68.39% | † | 57.79% | 83.95% | 78.64% | 25.50% |
| [serde-brief 0.1.0][serde-brief] | 9.50% | 23.76% | 4.60% | 25.68% | 53.74% | 62.05% | 20.54% |
| [serde_bare 0.5.0][serde_bare] | 16.87% | 52.08% | † | 91.97% | 94.31% | 91.75% | 38.77% |
| [serde_cbor 0.11.2][serde_cbor] | 6.80% | 26.43% | 5.09% | 29.53% | 58.29% | 66.35% | 21.79% |
| [serde_json 1.0.134][serde_json] | 3.47% | 18.91% | † | 20.19% | 43.07% | 50.69% | 13.01% |
| [simd-json 0.14.3][simd-json] | 5.80% | 27.28% | † | 20.19% | 43.07% | 50.69% | 13.05% |
| [speedy 0.8.7][speedy] | 48.69% | 78.17% | 29.30% | 72.89% | 85.52% | 86.61% | 31.73% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*29.13%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.06%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.04%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*4.5458 ms\**</span> <span title="prepend">*2.5840 ms\**</span> | 8.4303 ms | 1704643 | 1294259 | 1245668 | 11.977 ms |
| [bincode 2.0.0-rc][bincode] | 1.1952 ms | 3.9661 ms | 1406257 | 1117802 | 1062438 | 9.5806 ms |
| [bincode 1.3.3][bincode1] | 4.0075 ms | 4.2227 ms | 1854234 | 1141994 | 1048745 | 10.495 ms |
| [bitcode 0.6.4][bitcode] | 719.41 µs | 2.3554 ms | 971318 | 878034 | 850340 | 2.9472 ms |
| [borsh 1.5.3][borsh] | 2.8405 ms | 2.8933 ms | 1521989 | 1108471 | 1038528 | 10.131 ms |
| [capnp 0.20.3][capnp] | 2.2697 ms | † | 2724288 | 1546992 | 1239111 | 14.824 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.0693 ms | 19.186 ms | 6012539 | 1695215 | 1464951 | 22.672 ms |
| [ciborium 0.2.2][ciborium] | 23.549 ms | 56.427 ms | 6012373 | 1695146 | 1465025 | 22.057 ms |
| [databuf 0.5.0][databuf] | 1.3199 ms | 3.7914 ms | 1319999 | 1062631 | 1008334 | 9.7791 ms |
| [dlhn 0.1.7][dlhn] | 4.8855 ms | 6.8251 ms | 1311281 | 1077520 | 1046095 | 8.8510 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.1979 ms | † | 2325620 | 1440289 | 1264800 | 14.304 ms |
| [minicbor 0.25.1][minicbor] | 2.2901 ms | 11.434 ms | 1777386 | 1276218 | 1252558 | 13.305 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.502 ms | 17.882 ms | 1770060 | 1277755 | 1263362 | 12.756 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2871 ms | 2.9366 ms | 1812404 | 1134820 | 1053109 | 10.285 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.1536 ms | 3.1395 ms | 1319999 | 1064380 | 1010708 | 9.2079 ms |
| [postcard 1.1.1][postcard] | 1.9281 ms | 4.2525 ms | 1311281 | 1083900 | 1041434 | 8.8301 ms |
| [pot 3.0.1][pot] | 14.149 ms | 29.813 ms | 2604812 | 1482233 | 1298928 | 16.430 ms |
| [prost 0.13.4][prost] | <span title="encode">*5.4633 ms\**</span> <span title="populate + encode">*9.4155 ms\**</span> | 9.5378 ms | 1859886 | 1338076 | 1295351 | 12.627 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*5.4896 ms\**</span> <span title="populate + encode">*12.913 ms\**</span> | 12.712 ms | 1859886 | 1338076 | 1295351 | 12.754 ms |
| [rkyv 0.8.9][rkyv] | 968.04 µs | <span title="unvalidated">*2.1693 ms\**</span> <span title="validated upfront with error">*2.5915 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.383 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.202 ms | 11.274 ms | 1745322 | 1261627 | 1228923 | 11.953 ms |
| [ron 0.8.1][ron] | 38.074 ms | 88.243 ms | 8677703 | 2233642 | 1826180 | 35.104 ms |
| [savefile 0.18.5][savefile] | 850.85 µs | 2.7432 ms | 1791505 | 1128012 | 1051153 | 10.466 ms |
| [serde-brief 0.1.0][serde-brief] | 6.7928 ms | 21.484 ms | 6951772 | 1796265 | 1567819 | 24.285 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9216 ms | 4.8011 ms | 1319999 | 1062645 | 1008349 | 9.1808 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.177 ms | 21.309 ms | 6012373 | 1695146 | 1465025 | 22.325 ms |
| [serde_json 1.0.134][serde_json] | 20.341 ms | 30.861 ms | 9390461 | 2391679 | 1842767 | 34.753 ms |
| [simd-json 0.14.3][simd-json] | 11.507 ms | 25.793 ms | 9390461 | 2391679 | 1842767 | 34.628 ms |
| [speedy 0.8.7][speedy] | 776.26 µs | 2.3935 ms | 1584734 | 1119837 | 1037992 | 9.9536 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.087 ns\**</span> | <span title="validated on-demand with error">*716.94 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4947 ns\**</span> <span title="validated upfront with error">*5.1888 ms\**</span> | <span title="unvalidated">*2.6002 µs\**</span> <span title="validated upfront with error">*5.0817 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2435 ns\**</span> <span title="validated upfront with error">*414.73 µs\**</span> | <span title="unvalidated">*420.93 ns\**</span> <span title="validated upfront with error">*415.51 µs\**</span> | <span title="unvalidated">*233.81 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.2][bilrost] | <span title="encode">*15.83%\**</span> <span title="prepend">*27.84%\**</span> | 25.73% | 56.98% | 67.84% | 68.26% | 24.61% |
| [bincode 2.0.0-rc][bincode] | 60.19% | 54.70% | 69.07% | 78.55% | 80.04% | 30.76% |
| [bincode 1.3.3][bincode1] | 17.95% | 51.37% | 52.38% | 76.89% | 81.08% | 28.08% |
| [bitcode 0.6.4][bitcode] | 100.00% | 92.10% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 25.33% | 74.98% | 63.82% | 79.21% | 81.88% | 29.09% |
| [capnp 0.20.3][capnp] | 31.70% | † | 35.65% | 56.76% | 68.63% | 19.88% |
| [cbor4ii 0.3.3][cbor4ii] | 23.44% | 11.31% | 16.15% | 51.79% | 58.05% | 13.00% |
| [ciborium 0.2.2][ciborium] | 3.05% | 3.84% | 16.16% | 51.80% | 58.04% | 13.36% |
| [databuf 0.5.0][databuf] | 54.50% | 57.22% | 73.58% | 82.63% | 84.33% | 30.14% |
| [dlhn 0.1.7][dlhn] | 14.73% | 31.78% | 74.07% | 81.49% | 81.29% | 33.30% |
| [flatbuffers 24.12.23][flatbuffers] | 13.84% | † | 41.77% | 60.96% | 67.23% | 20.60% |
| [minicbor 0.25.1][minicbor] | 31.41% | 18.97% | 54.65% | 68.80% | 67.89% | 22.15% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.44% | 12.13% | 54.87% | 68.72% | 67.31% | 23.10% |
| [nanoserde 0.1.37][nanoserde] | 55.89% | 73.87% | 53.59% | 77.37% | 80.75% | 28.66% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.81% | 69.10% | 73.58% | 82.49% | 84.13% | 32.01% |
| [postcard 1.1.1][postcard] | 37.31% | 51.01% | 74.07% | 81.01% | 81.65% | 33.38% |
| [pot 3.0.1][pot] | 5.08% | 7.28% | 37.29% | 59.24% | 65.46% | 17.94% |
| [prost 0.13.4][prost] | <span title="encode">*13.17%\**</span> <span title="populate + encode">*7.64%\**</span> | 22.74% | 52.22% | 65.62% | 65.65% | 23.34% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*13.10%\**</span> <span title="populate + encode">*5.57%\**</span> | 17.06% | 52.22% | 65.62% | 65.65% | 23.11% |
| [rkyv 0.8.9][rkyv] | 74.32% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.71%\**</span> | 46.79% | 63.45% | 70.25% | 22.02% |
| [rmp-serde 1.3.0][rmp-serde] | 7.05% | 19.24% | 55.65% | 69.60% | 69.19% | 24.66% |
| [ron 0.8.1][ron] | 1.89% | 2.46% | 11.19% | 39.31% | 46.56% | 8.40% |
| [savefile 0.18.5][savefile] | 84.55% | 79.08% | 54.22% | 77.84% | 80.90% | 28.16% |
| [serde-brief 0.1.0][serde-brief] | 10.59% | 10.10% | 13.97% | 48.88% | 54.24% | 12.14% |
| [serde_bare 0.5.0][serde_bare] | 14.62% | 45.18% | 73.58% | 82.63% | 84.33% | 32.10% |
| [serde_cbor 0.11.2][serde_cbor] | 7.07% | 10.18% | 16.16% | 51.80% | 58.04% | 13.20% |
| [serde_json 1.0.134][serde_json] | 3.54% | 7.03% | 10.34% | 36.71% | 46.14% | 8.48% |
| [simd-json 0.14.3][simd-json] | 6.25% | 8.41% | 10.34% | 36.71% | 46.14% | 8.51% |
| [speedy 0.8.7][speedy] | 92.68% | 90.63% | 61.29% | 78.41% | 81.92% | 29.61% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*58.71%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.85%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.19%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
