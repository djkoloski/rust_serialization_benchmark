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

## Last updated: 2025-03-10 16:45:32

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.87.0-nightly (3ea711f17 2025-03-09)
binary: rustc
commit-hash: 3ea711f17e3946ac3f4df11691584e2c56b4b0cf
commit-date: 2025-03-09
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
| [bilrost 0.1012.3][bilrost] | <span title="encode">*474.78 µs\**</span> <span title="prepend">*432.46 µs\**</span> | 2.5898 ms | 887.10 µs | 804955 | 328941 | 284849 | 4.0817 ms |
| [bincode 2.0.0][bincode] | 344.43 µs | 2.2471 ms | 696.20 µs | 741295 | 303944 | 256422 | 3.7448 ms |
| [bincode 1.3.3][bincode1] | 543.96 µs | 2.0485 ms | 596.33 µs | 1045784 | 373127 | 311553 | 4.4170 ms |
| [bitcode 0.6.5][bitcode] | 146.62 µs | 1.4514 ms | 63.191 µs | 703710 | 288826 | 227322 | 2.4178 ms |
| [borsh 1.5.5][borsh] | 551.68 µs | 2.1524 ms | † | 885780 | 362204 | 286248 | 3.9165 ms |
| [capnp 0.20.6][capnp] | 522.87 µs | † | † | 1443216 | 513986 | 426532 | 6.1556 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 657.42 µs | 5.1121 ms | 3.3417 ms | 1407835 | 403440 | 323561 | 4.6588 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.1002 ms | 12.531 ms | † | 1407835 | 403440 | 323561 | 4.7594 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 2.2367 ms | 4.9218 ms | 3.2110 ms | 1407835 | 403440 | 323561 | 4.6590 ms |
| [databuf 0.5.0][databuf] | 275.17 µs | 2.0404 ms | 654.77 µs | 765778 | 311715 | 263914 | 3.4779 ms |
| [dlhn 0.1.7][dlhn] | 730.93 µs | 2.4914 ms | † | 724953 | 301446 | 253056 | 3.1845 ms |
| [flatbuffers 25.2.10][flatbuffers] | 1.0277 ms | † | † | 1276368 | 468539 | 388381 | 4.6961 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.7600 ms | 5.7437 ms | † | 1827461 | 470560 | 360727 | 5.4609 ms |
| json:<br> [simd-json 0.14.3][simd-json] | 2.0878 ms | 4.7200 ms | † | 1827461 | 470560 | 360727 | 5.7669 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.3722 ms | 3.0486 ms | 1.4874 ms | 784997 | 325384 | 277608 | 3.7237 ms |
| [minicbor 0.26.1][minicbor] | 532.20 µs | 2.9605 ms | 1.3664 ms | 817830 | 332671 | 284034 | 3.8753 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4505 ms | 4.2322 ms | 2.7238 ms | 818669 | 332556 | 284797 | 4.0432 ms |
| [nanoserde 0.1.37][nanoserde] | 244.57 µs | 2.0521 ms | † | 1045784 | 373127 | 311553 | 4.1227 ms |
| [postcard 1.1.1][postcard] | 423.37 µs | 2.2130 ms | 841.98 µs | 724953 | 302399 | 252968 | 3.5830 ms |
| [pot 3.0.1][pot] | 2.2645 ms | 6.6024 ms | 4.9531 ms | 971922 | 372513 | 303636 | 4.2505 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*936.15 µs\**</span> <span title="populate + encode">*2.4225 ms\**</span> | 3.3724 ms | † | 884628 | 363130 | 314959 | 4.3859 ms |
| protobuf:<br> [protobuf 3.7.1][protobuf] | <span title="encode">*1.1710 ms\**</span> <span title="populate + encode">*2.9564 ms\**</span> | 3.6954 ms | † | 884628 | 363130 | 314959 | 4.3759 ms |
| [rkyv 0.8.10][rkyv] | 246.65 µs | <span title="unvalidated">*1.5481 ms\**</span> <span title="validated upfront with error">*1.9324 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6936 ms |
| [ron 0.8.1][ron] | 11.632 ms | 15.843 ms | 14.233 ms | 1607459 | 449158 | 349324 | 5.5707 ms |
| [savefile 0.18.5][savefile] | 191.86 µs | 2.1689 ms | † | 1045800 | 373139 | 311562 | 4.1881 ms |
| scale:<br> [parity-scale-codec 3.7.4][parity-scale-codec] | 659.90 µs | 2.3729 ms | † | 765778 | 311743 | 263822 | 3.4967 ms |
| [serde-brief 0.1.1][serde-brief] | 1.5652 ms | 4.7886 ms | 3.0844 ms | 1584946 | 413733 | 339964 | 4.8635 ms |
| [serde_bare 0.5.0][serde_bare] | 704.00 µs | 2.0327 ms | † | 765778 | 311715 | 263914 | 3.4647 ms |
| [speedy 0.8.7][speedy] | 199.32 µs | 1.7537 ms | 362.78 µs | 885780 | 362204 | 286248 | 3.8196 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*73.357 ns\**</span> | <span title="validated on-demand with error">*170.47 µs\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4860 ns\**</span> <span title="validated upfront with error">*2.1491 ms\**</span> | <span title="unvalidated">*51.856 µs\**</span> <span title="validated upfront with error">*2.2298 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2433 ns\**</span> <span title="validated upfront with error">*375.90 µs\**</span> | <span title="unvalidated">*10.511 µs\**</span> <span title="validated upfront with error">*388.36 µs\**</span> | <span title="unvalidated">*7.4942 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*30.88%\**</span> <span title="prepend">*33.90%\**</span> | 56.04% | 7.12% | 87.42% | 87.80% | 79.80% | 59.24% |
| [bincode 2.0.0][bincode] | 42.57% | 64.59% | 9.08% | 94.93% | 95.03% | 88.65% | 64.56% |
| [bincode 1.3.3][bincode1] | 26.95% | 70.85% | 10.60% | 67.29% | 77.41% | 72.96% | 54.74% |
| [bitcode 0.6.5][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 26.58% | 67.43% | † | 79.45% | 79.74% | 79.41% | 61.73% |
| [capnp 0.20.6][capnp] | 28.04% | † | † | 48.76% | 56.19% | 53.30% | 39.28% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 22.30% | 28.39% | 1.89% | 49.99% | 71.59% | 70.26% | 51.90% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.58% | 11.58% | † | 49.99% | 71.59% | 70.26% | 50.80% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.56% | 29.49% | 1.97% | 49.99% | 71.59% | 70.26% | 51.90% |
| [databuf 0.5.0][databuf] | 53.28% | 71.13% | 9.65% | 91.89% | 92.66% | 86.13% | 69.52% |
| [dlhn 0.1.7][dlhn] | 20.06% | 58.26% | † | 97.07% | 95.81% | 89.83% | 75.92% |
| [flatbuffers 25.2.10][flatbuffers] | 14.27% | † | † | 55.13% | 61.64% | 58.53% | 51.49% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.90% | 25.27% | † | 38.51% | 61.38% | 63.02% | 44.27% |
| json:<br> [simd-json 0.14.3][simd-json] | 7.02% | 30.75% | † | 38.51% | 61.38% | 63.02% | 41.93% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.69% | 47.61% | 4.25% | 89.64% | 88.76% | 81.89% | 64.93% |
| [minicbor 0.26.1][minicbor] | 27.55% | 49.03% | 4.62% | 86.05% | 86.82% | 80.03% | 62.39% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.69% | 34.29% | 2.32% | 85.96% | 86.85% | 79.82% | 59.80% |
| [nanoserde 0.1.37][nanoserde] | 59.95% | 70.73% | † | 67.29% | 77.41% | 72.96% | 58.65% |
| [postcard 1.1.1][postcard] | 34.63% | 65.59% | 7.51% | 97.07% | 95.51% | 89.86% | 67.48% |
| [pot 3.0.1][pot] | 6.47% | 21.98% | 1.28% | 72.40% | 77.53% | 74.87% | 56.88% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*15.66%\**</span> <span title="populate + encode">*6.05%\**</span> | 43.04% | † | 79.55% | 79.54% | 72.18% | 55.13% |
| protobuf:<br> [protobuf 3.7.1][protobuf] | <span title="encode">*12.52%\**</span> <span title="populate + encode">*4.96%\**</span> | 39.28% | † | 79.55% | 79.54% | 72.18% | 55.25% |
| [rkyv 0.8.10][rkyv] | 59.44% | <span title="unvalidated">*93.75%\**</span> <span title="validated upfront with error">*75.11%\**</span> | † | 69.57% | 73.39% | 69.74% | 51.51% |
| [ron 0.8.1][ron] | 1.26% | 9.16% | 0.44% | 43.78% | 64.30% | 65.07% | 43.40% |
| [savefile 0.18.5][savefile] | 76.42% | 66.92% | † | 67.29% | 77.40% | 72.96% | 57.73% |
| scale:<br> [parity-scale-codec 3.7.4][parity-scale-codec] | 22.22% | 61.17% | † | 91.89% | 92.65% | 86.16% | 69.15% |
| [serde-brief 0.1.1][serde-brief] | 9.37% | 30.31% | 2.05% | 44.40% | 69.81% | 66.87% | 49.71% |
| [serde_bare 0.5.0][serde_bare] | 20.83% | 71.40% | † | 91.89% | 92.66% | 86.13% | 69.78% |
| [speedy 0.8.7][speedy] | 73.56% | 82.76% | 17.42% | 79.45% | 79.74% | 79.41% | 63.30% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*6.17%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.27%\**</span> <span title="validated upfront with error">*0.47%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.71%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*6.7461 ms\**</span> <span title="prepend">*8.6989 ms\**</span> | 8.0537 ms | 8625005 | 6443961 | 6231572 | 72.780 ms |
| [bincode 2.0.0][bincode] | 2.4170 ms | 1.0241 ms | 6000005 | 5378497 | 5346882 | 8.6136 ms |
| [bincode 1.3.3][bincode1] | 5.2258 ms | 4.6165 ms | 6000008 | 5378500 | 5346908 | 9.1810 ms |
| [bitcode 0.6.5][bitcode] | 1.4758 ms | 799.17 µs | 6000006 | 5182295 | 4921841 | 13.466 ms |
| [borsh 1.5.5][borsh] | 6.1708 ms | 4.3554 ms | 6000004 | 5378496 | 5346866 | 8.6049 ms |
| [capnp 0.20.6][capnp] | 6.3875 ms | † | 14000088 | 7130367 | 6046182 | 81.221 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 10.062 ms | 53.649 ms | 13125016 | 7524114 | 6757437 | 91.921 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 70.902 ms | 139.64 ms | 13122324 | 7524660 | 6759128 | 92.963 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 35.577 ms | 47.073 ms | 13122324 | 7524660 | 6759128 | 92.931 ms |
| [databuf 0.5.0][databuf] | 2.4209 ms | 5.3578 ms | 6000003 | 5378495 | 5346897 | 8.9352 ms |
| [dlhn 0.1.7][dlhn] | 6.0994 ms | 8.0085 ms | 6000003 | 5378495 | 5346897 | 8.7592 ms |
| [flatbuffers 25.2.10][flatbuffers] | 865.16 µs | † | 6000024 | 5378434 | 5346878 | 8.6093 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 89.235 ms | 89.447 ms | 26192883 | 9566084 | 8584671 | 156.97 ms |
| json:<br> [simd-json 0.14.3][simd-json] | 51.676 ms | 70.148 ms | 26192883 | 9566084 | 8584671 | 156.25 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 14.953 ms | 16.456 ms | 8125006 | 6494876 | 6391037 | 71.335 ms |
| [minicbor 0.26.1][minicbor] | 6.0593 ms | 12.230 ms | 8125006 | 6494907 | 6390894 | 71.000 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 123.77 ms | 26.818 ms | 8125037 | 6493484 | 6386940 | 70.319 ms |
| [nanoserde 0.1.37][nanoserde] | 1.4382 ms | 1.1079 ms | 6000008 | 5378500 | 5346908 | 8.5391 ms |
| [postcard 1.1.1][postcard] | 478.77 µs | 1.2706 ms | 6000003 | 5378495 | 5346897 | 8.7050 ms |
| [pot 3.0.1][pot] | 40.993 ms | 73.700 ms | 10122342 | 6814618 | 6852252 | 83.720 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*7.8238 ms\**</span> <span title="populate + encode">*8.8196 ms\**</span> | 16.186 ms | 8750000 | 6665735 | 6421877 | 73.871 ms |
| protobuf:<br> [protobuf 3.7.1][protobuf] | <span title="encode">*15.837 ms\**</span> <span title="populate + encode">*32.268 ms\**</span> | 28.935 ms | 8750000 | 6665735 | 6421877 | 80.015 ms |
| [rkyv 0.8.10][rkyv] | 148.54 µs | <span title="unvalidated">*186.72 µs\**</span> <span title="validated upfront with error">*206.25 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.7714 ms |
| [ron 0.8.1][ron] | 172.67 ms | 237.15 ms | 22192885 | 8970395 | 8137334 | 151.44 ms |
| [savefile 0.18.5][savefile] | 198.07 µs | 189.59 µs | 6000024 | 5378519 | 5346896 | 8.6275 ms |
| scale:<br> [parity-scale-codec 3.7.4][parity-scale-codec] | 5.1154 ms | 4.8728 ms | 6000004 | 5378496 | 5346866 | 8.8727 ms |
| [serde-brief 0.1.1][serde-brief] | 22.837 ms | 38.056 ms | 15750015 | 8024540 | 6813667 | 94.556 ms |
| [serde_bare 0.5.0][serde_bare] | 5.1859 ms | 4.8683 ms | 6000003 | 5378495 | 5346897 | 8.6688 ms |
| [speedy 0.8.7][speedy] | 148.48 µs | 148.73 µs | 6000004 | 5378496 | 5346866 | 8.8207 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*102.62 ns\**</span> | <span title="validated on-demand with error">*2.2706 ms\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4989 ns\**</span> <span title="validated upfront with error">*45.030 ns\**</span> | <span title="unvalidated">*52.360 µs\**</span> <span title="validated upfront with error">*77.739 µs\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*5.6165 ns\**</span> | <span title="unvalidated">*38.893 µs\**</span> <span title="validated upfront with error">*38.866 µs\**</span> | <span title="unvalidated">*100.25 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*2.20%\**</span> <span title="prepend">*1.71%\**</span> | 1.85% | 69.57% | 80.42% | 78.98% | 11.73% |
| [bincode 2.0.0][bincode] | 6.14% | 14.52% | 100.00% | 96.35% | 92.05% | 99.14% |
| [bincode 1.3.3][bincode1] | 2.84% | 3.22% | 100.00% | 96.35% | 92.05% | 93.01% |
| [bitcode 0.6.5][bitcode] | 10.06% | 18.61% | 100.00% | 100.00% | 100.00% | 63.41% |
| [borsh 1.5.5][borsh] | 2.41% | 3.41% | 100.00% | 96.35% | 92.05% | 99.24% |
| [capnp 0.20.6][capnp] | 2.32% | † | 42.86% | 72.68% | 81.40% | 10.51% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.48% | 0.28% | 45.71% | 68.88% | 72.84% | 9.29% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.21% | 0.11% | 45.72% | 68.87% | 72.82% | 9.19% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.42% | 0.32% | 45.72% | 68.87% | 72.82% | 9.19% |
| [databuf 0.5.0][databuf] | 6.13% | 2.78% | 100.00% | 96.35% | 92.05% | 95.57% |
| [dlhn 0.1.7][dlhn] | 2.43% | 1.86% | 100.00% | 96.35% | 92.05% | 97.49% |
| [flatbuffers 25.2.10][flatbuffers] | 17.16% | † | 100.00% | 96.35% | 92.05% | 99.18% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.33% | 5.44% |
| json:<br> [simd-json 0.14.3][simd-json] | 0.29% | 0.21% | 22.91% | 54.17% | 57.33% | 5.46% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.99% | 0.90% | 73.85% | 79.79% | 77.01% | 11.97% |
| [minicbor 0.26.1][minicbor] | 2.45% | 1.22% | 73.85% | 79.79% | 77.01% | 12.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.55% | 73.85% | 79.81% | 77.06% | 12.14% |
| [nanoserde 0.1.37][nanoserde] | 10.32% | 13.42% | 100.00% | 96.35% | 92.05% | 100.00% |
| [postcard 1.1.1][postcard] | 31.01% | 11.71% | 100.00% | 96.35% | 92.05% | 98.09% |
| [pot 3.0.1][pot] | 0.36% | 0.20% | 59.27% | 76.05% | 71.83% | 10.20% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.90%\**</span> <span title="populate + encode">*1.68%\**</span> | 0.92% | 68.57% | 77.75% | 76.64% | 11.56% |
| protobuf:<br> [protobuf 3.7.1][protobuf] | <span title="encode">*0.94%\**</span> <span title="populate + encode">*0.46%\**</span> | 0.51% | 68.57% | 77.75% | 76.64% | 10.67% |
| [rkyv 0.8.10][rkyv] | 99.96% | <span title="unvalidated">*79.65%\**</span> <span title="validated upfront with error">*72.11%\**</span> | 100.00% | 96.35% | 92.05% | 97.35% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.48% | 5.64% |
| [savefile 0.18.5][savefile] | 74.96% | 78.45% | 100.00% | 96.35% | 92.05% | 98.98% |
| scale:<br> [parity-scale-codec 3.7.4][parity-scale-codec] | 2.90% | 3.05% | 100.00% | 96.35% | 92.05% | 96.24% |
| [serde-brief 0.1.1][serde-brief] | 0.65% | 0.39% | 38.10% | 64.58% | 72.23% | 9.03% |
| [serde_bare 0.5.0][serde_bare] | 2.86% | 3.06% | 100.00% | 96.35% | 92.05% | 98.50% |
| [speedy 0.8.7][speedy] | 100.00% | 100.00% | 100.00% | 96.35% | 92.05% | 96.81% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.21%\**</span> | <span title="validated on-demand with error">*1.71%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.74%\**</span> <span title="validated upfront with error">*2.76%\**</span> | <span title="unvalidated">*74.23%\**</span> <span title="validated upfront with error">*50.00%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*22.13%\**</span> | <span title="unvalidated">*99.93%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*918.79 µs\**</span> <span title="prepend">*836.88 µs\**</span> | 3.0783 ms | 1.7592 ms | 489348 | 281173 | 249360 | 2.7072 ms |
| [bincode 2.0.0][bincode] | 334.61 µs | 1.8611 ms | 811.37 µs | 367413 | 221291 | 206242 | 2.1900 ms |
| [bincode 1.3.3][bincode1] | 601.96 µs | 1.8219 ms | 848.15 µs | 569975 | 240525 | 231884 | 2.5585 ms |
| [bitcode 0.6.5][bitcode] | 127.65 µs | 1.2498 ms | 170.47 µs | 327688 | 200947 | 182040 | 756.25 µs |
| [borsh 1.5.5][borsh] | 561.13 µs | 1.8320 ms | † | 446595 | 234236 | 209834 | 2.0394 ms |
| [capnp 0.20.6][capnp] | 453.61 µs | † | † | 803896 | 335606 | 280744 | 3.5611 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 806.32 µs | 4.8142 ms | 3.4792 ms | 1109831 | 344745 | 274333 | 3.4609 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.6996 ms | 10.194 ms | † | 1109821 | 344751 | 274345 | 3.4738 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8630 ms | 4.7309 ms | 3.4148 ms | 1109821 | 344751 | 274345 | 3.5001 ms |
| [databuf 0.5.0][databuf] | 326.84 µs | 1.7158 ms | 813.42 µs | 356311 | 213062 | 198403 | 2.0169 ms |
| [dlhn 0.1.7][dlhn] | 774.72 µs | 2.6043 ms | † | 366496 | 220600 | 205586 | 1.9825 ms |
| [flatbuffers 25.2.10][flatbuffers] | 3.2289 ms | † | † | 844168 | 345696 | 293916 | 3.5656 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.7085 ms | 6.7250 ms | † | 1623191 | 466527 | 359157 | 5.9366 ms |
| json:<br> [simd-json 0.14.3][simd-json] | 2.2169 ms | 4.6288 ms | † | 1623191 | 466527 | 359157 | 5.7904 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4922 ms | 3.0034 ms | 1.6984 ms | 424533 | 245214 | 226077 | 2.2436 ms |
| [minicbor 0.26.1][minicbor] | 565.63 µs | 3.3547 ms | 1.8748 ms | 428773 | 249857 | 228630 | 2.2738 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1252 ms | 4.1078 ms | 2.9588 ms | 449745 | 252432 | 230965 | 2.3291 ms |
| [nanoserde 0.1.37][nanoserde] | 275.26 µs | 1.9403 ms | † | 567975 | 239930 | 231872 | 2.4468 ms |
| [postcard 1.1.1][postcard] | 449.58 µs | 2.0591 ms | 815.73 µs | 367489 | 221913 | 207244 | 2.0421 ms |
| [pot 3.0.1][pot] | 2.3722 ms | 6.3431 ms | 5.2535 ms | 599125 | 299158 | 247675 | 2.7239 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.2847 ms\**</span> <span title="populate + encode">*3.0183 ms\**</span> | 3.5265 ms | † | 596811 | 305319 | 268737 | 3.0989 ms |
| protobuf:<br> [protobuf 3.7.1][protobuf] | <span title="encode">*1.0569 ms\**</span> <span title="populate + encode">*3.0225 ms\**</span> | 3.7696 ms | † | 596811 | 305319 | 268737 | 3.0421 ms |
| [rkyv 0.8.10][rkyv] | 346.96 µs | <span title="unvalidated">*1.5033 ms\**</span> <span title="validated upfront with error">*1.8538 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3134 ms |
| [ron 0.8.1][ron] | 7.3445 ms | 17.492 ms | 15.720 ms | 1465223 | 434935 | 342907 | 5.7423 ms |
| [savefile 0.18.5][savefile] | 211.90 µs | 1.8192 ms | † | 566991 | 239362 | 231478 | 2.4943 ms |
| scale:<br> [parity-scale-codec 3.7.4][parity-scale-codec] | 623.83 µs | 2.1282 ms | † | 356311 | 212976 | 198423 | 1.9635 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3649 ms | 5.3684 ms | 3.7296 ms | 1276014 | 373898 | 293384 | 3.6533 ms |
| [serde_bare 0.5.0][serde_bare] | 769.88 µs | 2.3105 ms | † | 356311 | 213062 | 198403 | 1.9285 ms |
| [speedy 0.8.7][speedy] | 264.86 µs | 1.6247 ms | 564.07 µs | 449595 | 234970 | 210192 | 2.2807 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*73.887 ns\**</span> | <span title="validated on-demand with error">*421.91 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4875 ns\**</span> <span title="validated upfront with error">*2.4285 ms\**</span> | <span title="unvalidated">*1.3542 µs\**</span> <span title="validated upfront with error">*2.4305 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2426 ns\**</span> <span title="validated upfront with error">*340.66 µs\**</span> | <span title="unvalidated">*240.24 ns\**</span> <span title="validated upfront with error">*345.74 µs\**</span> | <span title="unvalidated">*746.67 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*13.89%\**</span> <span title="prepend">*15.25%\**</span> | 40.60% | 9.69% | 66.96% | 71.47% | 73.00% | 27.93% |
| [bincode 2.0.0][bincode] | 38.15% | 67.15% | 21.01% | 89.19% | 90.81% | 88.27% | 34.53% |
| [bincode 1.3.3][bincode1] | 21.21% | 68.60% | 20.10% | 57.49% | 83.55% | 78.50% | 29.56% |
| [bitcode 0.6.5][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 22.75% | 68.22% | † | 73.37% | 85.79% | 86.75% | 37.08% |
| [capnp 0.20.6][capnp] | 28.14% | † | † | 40.76% | 59.88% | 64.84% | 21.24% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 15.83% | 25.96% | 4.90% | 29.53% | 58.29% | 66.36% | 21.85% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.45% | 12.26% | † | 29.53% | 58.29% | 66.35% | 21.77% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.85% | 26.42% | 4.99% | 29.53% | 58.29% | 66.35% | 21.61% |
| [databuf 0.5.0][databuf] | 39.06% | 72.84% | 20.96% | 91.97% | 94.31% | 91.75% | 37.50% |
| [dlhn 0.1.7][dlhn] | 16.48% | 47.99% | † | 89.41% | 91.09% | 88.55% | 38.15% |
| [flatbuffers 25.2.10][flatbuffers] | 3.95% | † | † | 38.82% | 58.13% | 61.94% | 21.21% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.44% | 18.58% | † | 20.19% | 43.07% | 50.69% | 12.74% |
| json:<br> [simd-json 0.14.3][simd-json] | 5.76% | 27.00% | † | 20.19% | 43.07% | 50.69% | 13.06% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.55% | 41.61% | 10.04% | 77.19% | 81.95% | 80.52% | 33.71% |
| [minicbor 0.26.1][minicbor] | 22.57% | 37.26% | 9.09% | 76.42% | 80.42% | 79.62% | 33.26% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.49% | 30.43% | 5.76% | 72.86% | 79.60% | 78.82% | 32.47% |
| [nanoserde 0.1.37][nanoserde] | 46.37% | 64.41% | † | 57.69% | 83.75% | 78.51% | 30.91% |
| [postcard 1.1.1][postcard] | 28.39% | 60.70% | 20.90% | 89.17% | 90.55% | 87.84% | 37.03% |
| [pot 3.0.1][pot] | 5.38% | 19.70% | 3.24% | 54.69% | 67.17% | 73.50% | 27.76% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*9.94%\**</span> <span title="populate + encode">*4.23%\**</span> | 35.44% | † | 54.91% | 65.82% | 67.74% | 24.40% |
| protobuf:<br> [protobuf 3.7.1][protobuf] | <span title="encode">*12.08%\**</span> <span title="populate + encode">*4.22%\**</span> | 33.15% | † | 54.91% | 65.82% | 67.74% | 24.86% |
| [rkyv 0.8.10][rkyv] | 36.79% | <span title="unvalidated">*83.14%\**</span> <span title="validated upfront with error">*67.42%\**</span> | † | 54.27% | 78.87% | 82.96% | 32.69% |
| [ron 0.8.1][ron] | 1.74% | 7.14% | 1.08% | 22.36% | 46.20% | 53.09% | 13.17% |
| [savefile 0.18.5][savefile] | 60.24% | 68.70% | † | 57.79% | 83.95% | 78.64% | 30.32% |
| scale:<br> [parity-scale-codec 3.7.4][parity-scale-codec] | 20.46% | 58.73% | † | 91.97% | 94.35% | 91.74% | 38.52% |
| [serde-brief 0.1.1][serde-brief] | 9.35% | 23.28% | 4.57% | 25.68% | 53.74% | 62.05% | 20.70% |
| [serde_bare 0.5.0][serde_bare] | 16.58% | 54.09% | † | 91.97% | 94.31% | 91.75% | 39.21% |
| [speedy 0.8.7][speedy] | 48.20% | 76.92% | 30.22% | 72.89% | 85.52% | 86.61% | 33.16% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*56.94%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.74%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.07%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*4.5672 ms\**</span> <span title="prepend">*2.5584 ms\**</span> | 8.3589 ms | 1704643 | 1294259 | 1245668 | 11.757 ms |
| [bincode 2.0.0][bincode] | 1.2190 ms | 3.7032 ms | 1406257 | 1117802 | 1062438 | 9.7202 ms |
| [bincode 1.3.3][bincode1] | 3.9414 ms | 4.1942 ms | 1854234 | 1141994 | 1048745 | 10.996 ms |
| [bitcode 0.6.5][bitcode] | 718.45 µs | 2.3422 ms | 971318 | 878034 | 850340 | 2.9277 ms |
| [borsh 1.5.5][borsh] | 2.9199 ms | 2.8871 ms | 1521989 | 1108471 | 1038528 | 9.9964 ms |
| [capnp 0.20.6][capnp] | 2.2469 ms | † | 2724288 | 1546992 | 1239111 | 15.034 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.0795 ms | 17.955 ms | 6012539 | 1695215 | 1464951 | 21.561 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.045 ms | 57.583 ms | 6012373 | 1695146 | 1465025 | 21.019 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.593 ms | 21.895 ms | 6012373 | 1695146 | 1465025 | 21.168 ms |
| [databuf 0.5.0][databuf] | 1.3238 ms | 3.7473 ms | 1319999 | 1062631 | 1008334 | 8.9097 ms |
| [dlhn 0.1.7][dlhn] | 4.9708 ms | 6.6144 ms | 1311281 | 1077520 | 1046095 | 8.7287 ms |
| [flatbuffers 25.2.10][flatbuffers] | 5.2611 ms | † | 2325620 | 1440289 | 1264800 | 13.394 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.042 ms | 30.846 ms | 9390461 | 2391679 | 1842767 | 34.907 ms |
| json:<br> [simd-json 0.14.3][simd-json] | 11.635 ms | 25.900 ms | 9390461 | 2391679 | 1842767 | 35.759 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.687 ms | 11.359 ms | 1745322 | 1261627 | 1228923 | 11.770 ms |
| [minicbor 0.26.1][minicbor] | 2.3403 ms | 11.391 ms | 1777386 | 1276218 | 1252558 | 13.181 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.716 ms | 17.759 ms | 1770060 | 1277755 | 1263362 | 12.764 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3124 ms | 2.9335 ms | 1812404 | 1134820 | 1053109 | 10.177 ms |
| [postcard 1.1.1][postcard] | 1.9828 ms | 4.2152 ms | 1311281 | 1083900 | 1041434 | 9.0511 ms |
| [pot 3.0.1][pot] | 13.573 ms | 31.130 ms | 2604812 | 1482233 | 1298928 | 16.224 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*5.5177 ms\**</span> <span title="populate + encode">*9.4315 ms\**</span> | 9.0767 ms | 1859886 | 1338076 | 1295351 | 12.264 ms |
| protobuf:<br> [protobuf 3.7.1][protobuf] | <span title="encode">*5.5208 ms\**</span> <span title="populate + encode">*13.034 ms\**</span> | 12.039 ms | 1859886 | 1338076 | 1295351 | 12.904 ms |
| [rkyv 0.8.10][rkyv] | 918.82 µs | <span title="unvalidated">*2.1694 ms\**</span> <span title="validated upfront with error">*2.6219 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.710 ms |
| [ron 0.8.1][ron] | 37.412 ms | 92.620 ms | 8677703 | 2233642 | 1826180 | 34.895 ms |
| [savefile 0.18.5][savefile] | 864.44 µs | 2.7406 ms | 1791505 | 1128012 | 1051153 | 10.587 ms |
| scale:<br> [parity-scale-codec 3.7.4][parity-scale-codec] | 3.1837 ms | 3.3607 ms | 1319999 | 1064380 | 1010708 | 8.9163 ms |
| [serde-brief 0.1.1][serde-brief] | 6.7014 ms | 22.272 ms | 6951772 | 1796265 | 1567819 | 23.309 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8676 ms | 4.8446 ms | 1319999 | 1062645 | 1008349 | 8.8948 ms |
| [speedy 0.8.7][speedy] | 762.32 µs | 2.3920 ms | 1584734 | 1119837 | 1037992 | 10.226 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*73.797 ns\**</span> | <span title="validated on-demand with error">*713.79 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4850 ns\**</span> <span title="validated upfront with error">*5.8039 ms\**</span> | <span title="unvalidated">*2.6029 µs\**</span> <span title="validated upfront with error">*5.8111 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*438.73 µs\**</span> | <span title="unvalidated">*416.50 ns\**</span> <span title="validated upfront with error">*439.31 µs\**</span> | <span title="unvalidated">*234.00 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*15.73%\**</span> <span title="prepend">*28.08%\**</span> | 25.95% | 56.98% | 67.84% | 68.26% | 24.90% |
| [bincode 2.0.0][bincode] | 58.94% | 58.58% | 69.07% | 78.55% | 80.04% | 30.12% |
| [bincode 1.3.3][bincode1] | 18.23% | 51.72% | 52.38% | 76.89% | 81.08% | 26.63% |
| [bitcode 0.6.5][bitcode] | 100.00% | 92.62% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 24.61% | 75.14% | 63.82% | 79.21% | 81.88% | 29.29% |
| [capnp 0.20.6][capnp] | 31.98% | † | 35.65% | 56.76% | 68.63% | 19.47% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 23.33% | 12.08% | 16.15% | 51.79% | 58.05% | 13.58% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.12% | 3.77% | 16.16% | 51.80% | 58.04% | 13.93% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.78% | 9.91% | 16.16% | 51.80% | 58.04% | 13.83% |
| [databuf 0.5.0][databuf] | 54.27% | 57.89% | 73.58% | 82.63% | 84.33% | 32.86% |
| [dlhn 0.1.7][dlhn] | 14.45% | 32.80% | 74.07% | 81.49% | 81.29% | 33.54% |
| [flatbuffers 25.2.10][flatbuffers] | 13.66% | † | 41.77% | 60.96% | 67.23% | 21.86% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.58% | 7.03% | 10.34% | 36.71% | 46.14% | 8.39% |
| json:<br> [simd-json 0.14.3][simd-json] | 6.17% | 8.38% | 10.34% | 36.71% | 46.14% | 8.19% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 6.72% | 19.10% | 55.65% | 69.60% | 69.19% | 24.88% |
| [minicbor 0.26.1][minicbor] | 30.70% | 19.04% | 54.65% | 68.80% | 67.89% | 22.21% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.42% | 12.22% | 54.87% | 68.72% | 67.31% | 22.94% |
| [nanoserde 0.1.37][nanoserde] | 54.74% | 73.95% | 53.59% | 77.37% | 80.75% | 28.77% |
| [postcard 1.1.1][postcard] | 36.23% | 51.47% | 74.07% | 81.01% | 81.65% | 32.35% |
| [pot 3.0.1][pot] | 5.29% | 6.97% | 37.29% | 59.24% | 65.46% | 18.05% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*13.02%\**</span> <span title="populate + encode">*7.62%\**</span> | 23.90% | 52.22% | 65.62% | 65.65% | 23.87% |
| protobuf:<br> [protobuf 3.7.1][protobuf] | <span title="encode">*13.01%\**</span> <span title="populate + encode">*5.51%\**</span> | 18.02% | 52.22% | 65.62% | 65.65% | 22.69% |
| [rkyv 0.8.10][rkyv] | 78.19% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*82.74%\**</span> | 46.79% | 63.45% | 70.25% | 21.36% |
| [ron 0.8.1][ron] | 1.92% | 2.34% | 11.19% | 39.31% | 46.56% | 8.39% |
| [savefile 0.18.5][savefile] | 83.11% | 79.16% | 54.22% | 77.84% | 80.90% | 27.65% |
| scale:<br> [parity-scale-codec 3.7.4][parity-scale-codec] | 22.57% | 64.55% | 73.58% | 82.49% | 84.13% | 32.84% |
| [serde-brief 0.1.1][serde-brief] | 10.72% | 9.74% | 13.97% | 48.88% | 54.24% | 12.56% |
| [serde_bare 0.5.0][serde_bare] | 14.76% | 44.78% | 73.58% | 82.63% | 84.33% | 32.91% |
| [speedy 0.8.7][speedy] | 94.25% | 90.69% | 61.29% | 78.41% | 81.92% | 28.63% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*58.35%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.00%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1012.3
[bincode]: https://crates.io/crates/bincode/2.0.0
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
