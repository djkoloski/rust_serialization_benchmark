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

## Last updated: 2025-1-23 16:13:45

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.86.0-nightly (649b995a9 2025-01-22)
binary: rustc
commit-hash: 649b995a9febd658b2570160703dff6fdc038ab2
commit-date: 2025-01-22
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

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*468.94 µs\**</span> <span title="prepend">*430.25 µs\**</span> | 2.6074 ms | 804955 | 328941 | 285485 | 4.4709 ms |
| [bincode 2.0.0-rc][bincode] | 271.11 µs | 2.4483 ms | 741295 | 303944 | 257153 | 3.7006 ms |
| [bincode 1.3.3][bincode1] | 522.62 µs | 2.0138 ms | 1045784 | 373127 | 311761 | 4.5014 ms |
| [bitcode 0.6.3][bitcode] | 138.91 µs | 1.4562 ms | 703710 | 288826 | 229755 | 2.5661 ms |
| [borsh 1.5.3][borsh] | 551.20 µs | 2.1874 ms | 885780 | 362204 | 286514 | 4.2188 ms |
| [capnp 0.20.3][capnp] | 576.77 µs | † | 1443216 | 513986 | 428649 | 6.3334 ms |
| [cbor4ii 0.3.3][cbor4ii] | 605.97 µs | 4.8471 ms | 1407835 | 403440 | 324081 | 4.7058 ms |
| [ciborium 0.2.2][ciborium] | 4.0368 ms | 12.364 ms | 1407835 | 403440 | 324081 | 4.7159 ms |
| [databuf 0.5.0][databuf] | 257.71 µs | 2.0267 ms | 765778 | 311715 | 264630 | 3.8414 ms |
| [dlhn 0.1.7][dlhn] | 760.49 µs | 2.5764 ms | 724953 | 301446 | 253629 | 3.5083 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0441 ms | † | 1276368 | 468539 | 388832 | 5.0906 ms |
| [minicbor 0.25.1][minicbor] | 711.17 µs | 3.0255 ms | 817830 | 332671 | 284548 | 4.3075 ms |
| [msgpacker 0.4.5][msgpacker] | 1.3962 ms | 2.6280 ms | 764996 | 315291 | 264898 | 4.1355 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3408 ms | 4.2166 ms | 818669 | 332556 | 285514 | 4.5957 ms |
| [nanoserde 0.1.37][nanoserde] | 255.67 µs | 2.0405 ms | 1045784 | 373127 | 311761 | 4.5107 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 668.92 µs | 2.2361 ms | 765778 | 311743 | 264518 | 4.1175 ms |
| [postcard 1.1.1][postcard] | 427.87 µs | 2.2262 ms | 724953 | 302399 | 253747 | 3.7701 ms |
| [pot 3.0.1][pot] | 2.2414 ms | 6.4921 ms | 971922 | 372513 | 304122 | 4.8781 ms |
| [prost 0.13.4][prost] | <span title="encode">*906.84 µs\**</span> <span title="populate + encode">*2.3668 ms\**</span> | 3.2061 ms | 884628 | 363130 | 315494 | 5.0483 ms |
| [rkyv 0.8.9][rkyv] | 246.33 µs | <span title="unvalidated">*1.4444 ms\**</span> <span title="validated upfront with error">*1.9898 ms\**</span> | 1011488 | 393526 | 326517 | 4.8623 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4538 ms | 3.2375 ms | 784997 | 325384 | 278219 | 4.0899 ms |
| [ron 0.8.1][ron] | 12.519 ms | 15.624 ms | 1607459 | 449158 | 349713 | 5.6055 ms |
| [savefile 0.18.5][savefile] | 191.32 µs | 2.1826 ms | 1045800 | 373139 | 311761 | 4.4556 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5460 ms | 4.7572 ms | 1584946 | 413733 | 341439 | 5.1766 ms |
| [serde_bare 0.5.0][serde_bare] | 681.15 µs | 2.0694 ms | 765778 | 311715 | 264630 | 3.8063 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0210 ms | 4.6091 ms | 1407835 | 403440 | 324081 | 4.7851 ms |
| [serde_json 1.0.134][serde_json] | 3.9134 ms | 5.9332 ms | 1827461 | 470560 | 361090 | 5.5058 ms |
| [simd-json 0.14.3][simd-json] | 2.1744 ms | 4.6913 ms | 1827461 | 470560 | 361090 | 5.6858 ms |
| [speedy 0.8.7][speedy] | 197.29 µs | 1.7349 ms | 885780 | 362204 | 286514 | 4.2131 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*72.984 ns\**</span> | <span title="validated on-demand with error">*168.75 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4868 ns\**</span> <span title="validated upfront with error">*1.9925 ms\**</span> | <span title="unvalidated">*50.852 µs\**</span> <span title="validated upfront with error">*2.0547 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*548.85 µs\**</span> | <span title="unvalidated">*10.425 µs\**</span> <span title="validated upfront with error">*588.22 µs\**</span> | <span title="unvalidated">*7.5791 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*29.62%\**</span> <span title="prepend">*32.29%\**</span> | 55.40% | 87.42% | 87.80% | 80.48% | 57.40% |
| [bincode 2.0.0-rc][bincode] | 51.24% | 59.00% | 94.93% | 95.03% | 89.35% | 69.34% |
| [bincode 1.3.3][bincode1] | 26.58% | 71.73% | 67.29% | 77.41% | 73.70% | 57.01% |
| [bitcode 0.6.3][bitcode] | 100.00% | 99.19% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 25.20% | 66.03% | 79.45% | 79.74% | 80.19% | 60.83% |
| [capnp 0.20.3][capnp] | 24.08% | † | 48.76% | 56.19% | 53.60% | 40.52% |
| [cbor4ii 0.3.3][cbor4ii] | 22.92% | 29.80% | 49.99% | 71.59% | 70.89% | 54.53% |
| [ciborium 0.2.2][ciborium] | 3.44% | 11.68% | 49.99% | 71.59% | 70.89% | 54.41% |
| [databuf 0.5.0][databuf] | 53.90% | 71.27% | 91.89% | 92.66% | 86.82% | 66.80% |
| [dlhn 0.1.7][dlhn] | 18.27% | 56.06% | 97.07% | 95.81% | 90.59% | 73.14% |
| [flatbuffers 24.12.23][flatbuffers] | 13.30% | † | 55.13% | 61.64% | 59.09% | 50.41% |
| [minicbor 0.25.1][minicbor] | 19.53% | 47.74% | 86.05% | 86.82% | 80.74% | 59.57% |
| [msgpacker 0.4.5][msgpacker] | 9.95% | 54.96% | 91.99% | 91.61% | 86.73% | 62.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.60% | 34.26% | 85.96% | 86.85% | 80.47% | 55.84% |
| [nanoserde 0.1.37][nanoserde] | 54.33% | 70.79% | 67.29% | 77.41% | 73.70% | 56.89% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.77% | 64.59% | 91.89% | 92.65% | 86.86% | 62.32% |
| [postcard 1.1.1][postcard] | 32.47% | 64.88% | 97.07% | 95.51% | 90.54% | 68.06% |
| [pot 3.0.1][pot] | 6.20% | 22.25% | 72.40% | 77.53% | 75.55% | 52.60% |
| [prost 0.13.4][prost] | <span title="encode">*15.32%\**</span> <span title="populate + encode">*5.87%\**</span> | 45.05% | 79.55% | 79.54% | 72.82% | 50.83% |
| [rkyv 0.8.9][rkyv] | 56.39% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.59%\**</span> | 69.57% | 73.39% | 70.37% | 52.78% |
| [rmp-serde 1.3.0][rmp-serde] | 9.55% | 44.61% | 89.64% | 88.76% | 82.58% | 62.74% |
| [ron 0.8.1][ron] | 1.11% | 9.24% | 43.78% | 64.30% | 65.70% | 45.78% |
| [savefile 0.18.5][savefile] | 72.61% | 66.18% | 67.29% | 77.40% | 73.70% | 57.59% |
| [serde-brief 0.1.0][serde-brief] | 8.99% | 30.36% | 44.40% | 69.81% | 67.29% | 49.57% |
| [serde_bare 0.5.0][serde_bare] | 20.39% | 69.80% | 91.89% | 92.66% | 86.82% | 67.42% |
| [serde_cbor 0.11.2][serde_cbor] | 6.87% | 31.34% | 49.99% | 71.59% | 70.89% | 53.63% |
| [serde_json 1.0.134][serde_json] | 3.55% | 24.34% | 38.51% | 61.38% | 63.63% | 46.61% |
| [simd-json 0.14.3][simd-json] | 6.39% | 30.79% | 38.51% | 61.38% | 63.63% | 45.13% |
| [speedy 0.8.7][speedy] | 70.41% | 83.26% | 79.45% | 79.74% | 80.19% | 60.91% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*6.18%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.50%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.77%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*6.7513 ms\**</span> <span title="prepend">*8.7737 ms\**</span> | 8.4603 ms | 8625005 | 6443961 | 6231572 | 75.996 ms |
| [bincode 2.0.0-rc][bincode] | 2.4104 ms | 1.0223 ms | 6000005 | 5378497 | 5345897 | 7.4003 ms |
| [bincode 1.3.3][bincode1] | 5.1898 ms | 4.7455 ms | 6000008 | 5378500 | 5345890 | 7.7514 ms |
| [bitcode 0.6.3][bitcode] | 1.4229 ms | 798.09 µs | 6000006 | 5182295 | 4923880 | 12.588 ms |
| [borsh 1.5.3][borsh] | 6.2427 ms | 4.3081 ms | 6000004 | 5378496 | 5345889 | 7.6148 ms |
| [capnp 0.20.3][capnp] | 6.0577 ms | † | 14000088 | 7130367 | 6051062 | 82.155 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.9928 ms | 53.941 ms | 13125016 | 7524114 | 6757967 | 91.880 ms |
| [ciborium 0.2.2][ciborium] | 71.487 ms | 119.05 ms | 13122324 | 7524660 | 6759658 | 90.909 ms |
| [databuf 0.5.0][databuf] | 2.4129 ms | 5.3977 ms | 6000003 | 5378495 | 5345900 | 7.6461 ms |
| [dlhn 0.1.7][dlhn] | 6.4464 ms | 7.0335 ms | 6000003 | 5378495 | 5345900 | 7.7044 ms |
| [flatbuffers 24.12.23][flatbuffers] | 871.82 µs | † | 6000024 | 5378434 | 5345910 | 7.8240 ms |
| [minicbor 0.25.1][minicbor] | 5.2343 ms | 11.598 ms | 8125006 | 6494907 | 6390894 | 70.931 ms |
| [msgpacker 0.4.5][msgpacker] | 18.427 ms | 5.2214 ms | 7500005 | 6058442 | 6014337 | 9.7305 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.37 ms | 33.041 ms | 8125037 | 6493484 | 6386940 | 71.026 ms |
| [nanoserde 0.1.37][nanoserde] | 1.5808 ms | 1.1046 ms | 6000008 | 5378500 | 5345890 | 7.4474 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1571 ms | 4.0075 ms | 6000004 | 5378496 | 5345889 | 7.4162 ms |
| [postcard 1.1.1][postcard] | 479.05 µs | 1.3239 ms | 6000003 | 5378495 | 5345900 | 7.4103 ms |
| [pot 3.0.1][pot] | 35.443 ms | 73.500 ms | 10122342 | 6814618 | 6852251 | 80.837 ms |
| [prost 0.13.4][prost] | <span title="encode">*7.8525 ms\**</span> <span title="populate + encode">*8.2353 ms\**</span> | 13.397 ms | 8750000 | 6665735 | 6421871 | 71.940 ms |
| [rkyv 0.8.9][rkyv] | 148.24 µs | <span title="unvalidated">*149.66 µs\**</span> <span title="validated upfront with error">*149.91 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.5636 ms |
| [rmp-serde 1.3.0][rmp-serde] | 18.475 ms | 17.711 ms | 8125006 | 6494876 | 6391037 | 68.527 ms |
| [ron 0.8.1][ron] | 177.53 ms | 243.05 ms | 22192885 | 8970395 | 8138755 | 146.63 ms |
| [savefile 0.18.5][savefile] | 148.22 µs | 148.29 µs | 6000024 | 5378519 | 5345892 | 7.6124 ms |
| [serde-brief 0.1.0][serde-brief] | 22.718 ms | 38.058 ms | 15750015 | 8024540 | 6816643 | 94.376 ms |
| [serde_bare 0.5.0][serde_bare] | 6.5413 ms | 4.7189 ms | 6000003 | 5378495 | 5345900 | 7.3928 ms |
| [serde_cbor 0.11.2][serde_cbor] | 33.446 ms | 44.603 ms | 13122324 | 7524660 | 6759658 | 89.534 ms |
| [serde_json 1.0.134][serde_json] | 87.406 ms | 90.661 ms | 26192883 | 9566084 | 8586741 | 152.46 ms |
| [simd-json 0.14.3][simd-json] | 51.036 ms | 69.339 ms | 26192883 | 9566084 | 8586741 | 152.09 ms |
| [speedy 0.8.7][speedy] | 148.14 µs | 148.16 µs | 6000004 | 5378496 | 5345889 | 7.3946 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*102.69 ns\**</span> | <span title="validated on-demand with error">*2.1607 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4851 ns\**</span> <span title="validated upfront with error">*39.245 ns\**</span> | <span title="unvalidated">*54.375 µs\**</span> <span title="validated upfront with error">*77.781 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2432 ns\**</span> <span title="validated upfront with error">*5.6468 ns\**</span> | <span title="unvalidated">*48.602 µs\**</span> <span title="validated upfront with error">*39.090 µs\**</span> | <span title="unvalidated">*77.542 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*2.19%\**</span> <span title="prepend">*1.69%\**</span> | 1.75% | 69.57% | 80.42% | 79.02% | 9.73% |
| [bincode 2.0.0-rc][bincode] | 6.15% | 14.49% | 100.00% | 96.35% | 92.11% | 99.90% |
| [bincode 1.3.3][bincode1] | 2.85% | 3.12% | 100.00% | 96.35% | 92.11% | 95.37% |
| [bitcode 0.6.3][bitcode] | 10.41% | 18.56% | 100.00% | 100.00% | 100.00% | 58.73% |
| [borsh 1.5.3][borsh] | 2.37% | 3.44% | 100.00% | 96.35% | 92.11% | 97.08% |
| [capnp 0.20.3][capnp] | 2.45% | † | 42.86% | 72.68% | 81.37% | 9.00% |
| [cbor4ii 0.3.3][cbor4ii] | 1.48% | 0.27% | 45.71% | 68.88% | 72.86% | 8.05% |
| [ciborium 0.2.2][ciborium] | 0.21% | 0.12% | 45.72% | 68.87% | 72.84% | 8.13% |
| [databuf 0.5.0][databuf] | 6.14% | 2.74% | 100.00% | 96.35% | 92.11% | 96.69% |
| [dlhn 0.1.7][dlhn] | 2.30% | 2.11% | 100.00% | 96.35% | 92.11% | 95.96% |
| [flatbuffers 24.12.23][flatbuffers] | 16.99% | † | 100.00% | 96.35% | 92.11% | 94.49% |
| [minicbor 0.25.1][minicbor] | 2.83% | 1.28% | 73.85% | 79.79% | 77.05% | 10.42% |
| [msgpacker 0.4.5][msgpacker] | 0.80% | 2.84% | 80.00% | 85.54% | 81.87% | 75.98% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.45% | 73.85% | 79.81% | 77.09% | 10.41% |
| [nanoserde 0.1.37][nanoserde] | 9.37% | 13.41% | 100.00% | 96.35% | 92.11% | 99.27% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.87% | 3.70% | 100.00% | 96.35% | 92.11% | 99.68% |
| [postcard 1.1.1][postcard] | 30.92% | 11.19% | 100.00% | 96.35% | 92.11% | 99.76% |
| [pot 3.0.1][pot] | 0.42% | 0.20% | 59.27% | 76.05% | 71.86% | 9.15% |
| [prost 0.13.4][prost] | <span title="encode">*1.89%\**</span> <span title="populate + encode">*1.80%\**</span> | 1.11% | 68.57% | 77.75% | 76.67% | 10.28% |
| [rkyv 0.8.9][rkyv] | 99.93% | <span title="unvalidated">*99.00%\**</span> <span title="validated upfront with error">*98.83%\**</span> | 100.00% | 96.35% | 92.11% | 97.74% |
| [rmp-serde 1.3.0][rmp-serde] | 0.80% | 0.84% | 73.85% | 79.79% | 77.04% | 10.79% |
| [ron 0.8.1][ron] | 0.08% | 0.06% | 27.04% | 57.77% | 60.50% | 5.04% |
| [savefile 0.18.5][savefile] | 99.95% | 99.91% | 100.00% | 96.35% | 92.11% | 97.12% |
| [serde-brief 0.1.0][serde-brief] | 0.65% | 0.39% | 38.10% | 64.58% | 72.23% | 7.83% |
| [serde_bare 0.5.0][serde_bare] | 2.26% | 3.14% | 100.00% | 96.35% | 92.11% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 0.44% | 0.33% | 45.72% | 68.87% | 72.84% | 8.26% |
| [serde_json 1.0.134][serde_json] | 0.17% | 0.16% | 22.91% | 54.17% | 57.34% | 4.85% |
| [simd-json 0.14.3][simd-json] | 0.29% | 0.21% | 22.91% | 54.17% | 57.34% | 4.86% |
| [speedy 0.8.7][speedy] | 100.00% | 100.00% | 100.00% | 96.35% | 92.11% | 99.98% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.21%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*3.17%\**</span> | <span title="unvalidated">*71.89%\**</span> <span title="validated upfront with error">*50.26%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*22.02%\**</span> | <span title="unvalidated">*80.43%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*932.98 µs\**</span> <span title="prepend">*867.67 µs\**</span> | 3.1590 ms | 489348 | 281173 | 249546 | 3.0569 ms |
| [bincode 2.0.0-rc][bincode] | 253.71 µs | 2.0490 ms | 367413 | 221291 | 206273 | 2.4751 ms |
| [bincode 1.3.3][bincode1] | 600.49 µs | 1.8158 ms | 569975 | 240525 | 232423 | 2.8511 ms |
| [bitcode 0.6.3][bitcode] | 135.69 µs | 1.2543 ms | 327688 | 200947 | 182736 | 735.51 µs |
| [borsh 1.5.3][borsh] | 562.23 µs | 1.7817 ms | 446595 | 234236 | 210008 | 2.5047 ms |
| [capnp 0.20.3][capnp] | 449.23 µs | † | 803896 | 335606 | 280851 | 3.8700 ms |
| [cbor4ii 0.3.3][cbor4ii] | 784.81 µs | 4.6529 ms | 1109831 | 344745 | 274514 | 3.7918 ms |
| [ciborium 0.2.2][ciborium] | 3.6234 ms | 9.9216 ms | 1109821 | 344751 | 274526 | 3.8291 ms |
| [databuf 0.5.0][databuf] | 319.02 µs | 1.7004 ms | 356311 | 213062 | 198488 | 2.9442 ms |
| [dlhn 0.1.7][dlhn] | 801.37 µs | 2.5909 ms | 366496 | 220600 | 205683 | 2.4555 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.2433 ms | † | 844168 | 345696 | 294015 | 3.8136 ms |
| [minicbor 0.25.1][minicbor] | 527.85 µs | 3.3854 ms | 428773 | 249857 | 228741 | 2.7410 ms |
| [msgpacker 0.4.5][msgpacker] | 983.67 µs | 2.8524 ms | 391251 | 236877 | 220476 | 2.6347 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2067 ms | 3.9527 ms | 449745 | 252432 | 231110 | 2.8061 ms |
| [nanoserde 0.1.37][nanoserde] | 268.39 µs | 1.9181 ms | 567975 | 239930 | 232419 | 2.8573 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 633.21 µs | 1.9191 ms | 356311 | 212976 | 198524 | 2.3939 ms |
| [postcard 1.1.1][postcard] | 450.63 µs | 1.9819 ms | 367489 | 221913 | 207344 | 2.4601 ms |
| [pot 3.0.1][pot] | 2.3758 ms | 5.9264 ms | 599125 | 299158 | 247693 | 3.1408 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.0931 ms\**</span> <span title="populate + encode">*2.7470 ms\**</span> | 3.4510 ms | 596811 | 305319 | 269310 | 3.4719 ms |
| [rkyv 0.8.9][rkyv] | 337.34 µs | <span title="unvalidated">*1.4507 ms\**</span> <span title="validated upfront with error">*1.9680 ms\**</span> | 603776 | 254776 | 220087 | 2.7889 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4835 ms | 3.0006 ms | 424533 | 245214 | 226188 | 2.6967 ms |
| [ron 0.8.1][ron] | 7.0992 ms | 17.213 ms | 1465223 | 434935 | 343338 | 5.7833 ms |
| [savefile 0.18.5][savefile] | 212.69 µs | 1.7952 ms | 566991 | 239362 | 232010 | 2.8483 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3621 ms | 5.3869 ms | 1276014 | 373898 | 293679 | 4.0309 ms |
| [serde_bare 0.5.0][serde_bare] | 746.94 µs | 2.3094 ms | 356311 | 213062 | 198488 | 2.3949 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8838 ms | 4.7265 ms | 1109821 | 344751 | 274526 | 3.8415 ms |
| [serde_json 1.0.134][serde_json] | 3.6948 ms | 6.8005 ms | 1623191 | 466527 | 359623 | 5.9752 ms |
| [simd-json 0.14.3][simd-json] | 2.2729 ms | 4.6392 ms | 1623191 | 466527 | 359623 | 5.9176 ms |
| [speedy 0.8.7][speedy] | 258.62 µs | 1.5684 ms | 449595 | 234970 | 210361 | 2.4820 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*72.925 ns\**</span> | <span title="validated on-demand with error">*411.02 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4865 ns\**</span> <span title="validated upfront with error">*2.1910 ms\**</span> | <span title="unvalidated">*1.3593 µs\**</span> <span title="validated upfront with error">*2.2006 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*504.31 µs\**</span> | <span title="unvalidated">*240.13 ns\**</span> <span title="validated upfront with error">*511.94 µs\**</span> | <span title="unvalidated">*751.98 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*14.54%\**</span> <span title="prepend">*15.64%\**</span> | 39.71% | 66.96% | 71.47% | 73.23% | 24.06% |
| [bincode 2.0.0-rc][bincode] | 53.48% | 61.22% | 89.19% | 90.81% | 88.59% | 29.72% |
| [bincode 1.3.3][bincode1] | 22.60% | 69.08% | 57.49% | 83.55% | 78.62% | 25.80% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.13% | 70.40% | 73.37% | 85.79% | 87.01% | 29.37% |
| [capnp 0.20.3][capnp] | 30.21% | † | 40.76% | 59.88% | 65.07% | 19.01% |
| [cbor4ii 0.3.3][cbor4ii] | 17.29% | 26.96% | 29.53% | 58.29% | 66.57% | 19.40% |
| [ciborium 0.2.2][ciborium] | 3.74% | 12.64% | 29.53% | 58.29% | 66.56% | 19.21% |
| [databuf 0.5.0][databuf] | 42.53% | 73.76% | 91.97% | 94.31% | 92.06% | 24.98% |
| [dlhn 0.1.7][dlhn] | 16.93% | 48.41% | 89.41% | 91.09% | 88.84% | 29.95% |
| [flatbuffers 24.12.23][flatbuffers] | 4.18% | † | 38.82% | 58.13% | 62.15% | 19.29% |
| [minicbor 0.25.1][minicbor] | 25.71% | 37.05% | 76.42% | 80.42% | 79.89% | 26.83% |
| [msgpacker 0.4.5][msgpacker] | 13.79% | 43.97% | 83.75% | 84.83% | 82.88% | 27.92% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.61% | 31.73% | 72.86% | 79.60% | 79.07% | 26.21% |
| [nanoserde 0.1.37][nanoserde] | 50.56% | 65.39% | 57.69% | 83.75% | 78.62% | 25.74% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.43% | 65.36% | 91.97% | 94.35% | 92.05% | 30.72% |
| [postcard 1.1.1][postcard] | 30.11% | 63.29% | 89.17% | 90.55% | 88.13% | 29.90% |
| [pot 3.0.1][pot] | 5.71% | 21.16% | 54.69% | 67.17% | 73.78% | 23.42% |
| [prost 0.13.4][prost] | <span title="encode">*12.41%\**</span> <span title="populate + encode">*4.94%\**</span> | 36.35% | 54.91% | 65.82% | 67.85% | 21.18% |
| [rkyv 0.8.9][rkyv] | 40.22% | <span title="unvalidated">*86.46%\**</span> <span title="validated upfront with error">*63.73%\**</span> | 54.27% | 78.87% | 83.03% | 26.37% |
| [rmp-serde 1.3.0][rmp-serde] | 9.15% | 41.80% | 77.19% | 81.95% | 80.79% | 27.27% |
| [ron 0.8.1][ron] | 1.91% | 7.29% | 22.36% | 46.20% | 53.22% | 12.72% |
| [savefile 0.18.5][savefile] | 63.80% | 69.87% | 57.79% | 83.95% | 78.76% | 25.82% |
| [serde-brief 0.1.0][serde-brief] | 9.96% | 23.28% | 25.68% | 53.74% | 62.22% | 18.25% |
| [serde_bare 0.5.0][serde_bare] | 18.17% | 54.31% | 91.97% | 94.31% | 92.06% | 30.71% |
| [serde_cbor 0.11.2][serde_cbor] | 7.20% | 26.54% | 29.53% | 58.29% | 66.56% | 19.15% |
| [serde_json 1.0.134][serde_json] | 3.67% | 18.44% | 20.19% | 43.07% | 50.81% | 12.31% |
| [simd-json 0.14.3][simd-json] | 5.97% | 27.04% | 20.19% | 43.07% | 50.81% | 12.43% |
| [speedy 0.8.7][speedy] | 52.47% | 79.97% | 72.89% | 85.52% | 86.87% | 29.63% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*58.42%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.67%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*4.5218 ms\**</span> <span title="prepend">*2.5898 ms\**</span> | 8.5106 ms | 1704643 | 1294259 | 1245607 | 11.322 ms |
| [bincode 2.0.0-rc][bincode] | 1.1968 ms | 3.7605 ms | 1406257 | 1117802 | 1062238 | 9.3091 ms |
| [bincode 1.3.3][bincode1] | 3.9715 ms | 4.1808 ms | 1854234 | 1141994 | 1050351 | 10.300 ms |
| [bitcode 0.6.3][bitcode] | 719.76 µs | 2.3301 ms | 971318 | 878034 | 855922 | 3.3335 ms |
| [borsh 1.5.3][borsh] | 2.9288 ms | 2.9116 ms | 1521989 | 1108471 | 1038408 | 9.7589 ms |
| [capnp 0.20.3][capnp] | 2.2213 ms | † | 2724288 | 1546992 | 1240354 | 14.619 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.3650 ms | 17.810 ms | 6012539 | 1695215 | 1467194 | 21.283 ms |
| [ciborium 0.2.2][ciborium] | 23.165 ms | 54.246 ms | 6012373 | 1695146 | 1467435 | 21.256 ms |
| [databuf 0.5.0][databuf] | 1.2917 ms | 3.7316 ms | 1319999 | 1062631 | 1007898 | 8.6940 ms |
| [dlhn 0.1.7][dlhn] | 4.9605 ms | 8.1453 ms | 1311281 | 1077520 | 1045571 | 8.6001 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.2152 ms | † | 2325620 | 1440289 | 1265148 | 13.257 ms |
| [msgpacker 0.4.5][msgpacker] | 2.3418 ms | 6.7814 ms | 1458773 | 1156055 | 1137194 | 9.6924 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.535 ms | 18.249 ms | 1770060 | 1277755 | 1263142 | 13.075 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2995 ms | 2.9379 ms | 1812404 | 1134820 | 1054758 | 10.174 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.1710 ms | 3.1871 ms | 1319999 | 1064380 | 1010284 | 8.8310 ms |
| [postcard 1.1.1][postcard] | 2.1697 ms | 4.3291 ms | 1311281 | 1083900 | 1041114 | 8.7760 ms |
| [pot 3.0.1][pot] | 13.448 ms | 30.521 ms | 2604812 | 1482233 | 1299952 | 15.831 ms |
| [prost 0.13.4][prost] | <span title="encode">*4.9321 ms\**</span> <span title="populate + encode">*8.8695 ms\**</span> | 9.1713 ms | 1859886 | 1338076 | 1295497 | 11.899 ms |
| [rkyv 0.8.9][rkyv] | 1.0205 ms | <span title="unvalidated">*2.1853 ms\**</span> <span title="validated upfront with error">*2.6170 ms\**</span> | 2075936 | 1383779 | 1211892 | 12.789 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.050 ms | 11.041 ms | 1745322 | 1261627 | 1228902 | 11.198 ms |
| [ron 0.8.1][ron] | 37.641 ms | 91.662 ms | 8677703 | 2233642 | 1827843 | 33.969 ms |
| [savefile 0.18.5][savefile] | 865.35 µs | 2.8060 ms | 1791505 | 1128012 | 1052757 | 10.268 ms |
| [serde-brief 0.1.0][serde-brief] | 6.4538 ms | 22.593 ms | 6951772 | 1796265 | 1570903 | 23.409 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7590 ms | 4.7887 ms | 1319999 | 1062645 | 1007918 | 8.8501 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.227 ms | 21.634 ms | 6012373 | 1695146 | 1467435 | 21.619 ms |
| [serde_json 1.0.134][serde_json] | 20.047 ms | 30.531 ms | 9390461 | 2391679 | 1843922 | 34.825 ms |
| [simd-json 0.14.3][simd-json] | 11.362 ms | 25.983 ms | 9390461 | 2391679 | 1843922 | 34.206 ms |
| [speedy 0.8.7][speedy] | 776.28 µs | 2.4789 ms | 1584734 | 1119837 | 1038012 | 9.9562 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*72.867 ns\**</span> | <span title="validated on-demand with error">*715.82 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4862 ns\**</span> <span title="validated upfront with error">*5.2352 ms\**</span> | <span title="unvalidated">*2.6094 µs\**</span> <span title="validated upfront with error">*5.2405 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2442 ns\**</span> <span title="validated upfront with error">*443.89 µs\**</span> | <span title="unvalidated">*442.22 ns\**</span> <span title="validated upfront with error">*442.55 µs\**</span> | <span title="unvalidated">*234.31 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*15.92%\**</span> <span title="prepend">*27.79%\**</span> | 25.68% | 56.98% | 67.84% | 68.72% | 29.44% |
| [bincode 2.0.0-rc][bincode] | 60.14% | 58.11% | 69.07% | 78.55% | 80.58% | 35.81% |
| [bincode 1.3.3][bincode1] | 18.12% | 52.27% | 52.38% | 76.89% | 81.49% | 32.37% |
| [bitcode 0.6.3][bitcode] | 100.00% | 93.79% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.58% | 75.05% | 63.82% | 79.21% | 82.43% | 34.16% |
| [capnp 0.20.3][capnp] | 32.40% | † | 35.65% | 56.76% | 69.01% | 22.80% |
| [cbor4ii 0.3.3][cbor4ii] | 21.39% | 12.27% | 16.15% | 51.79% | 58.34% | 15.66% |
| [ciborium 0.2.2][ciborium] | 3.11% | 4.03% | 16.16% | 51.80% | 58.33% | 15.68% |
| [databuf 0.5.0][databuf] | 55.72% | 58.56% | 73.58% | 82.63% | 84.92% | 38.34% |
| [dlhn 0.1.7][dlhn] | 14.51% | 26.83% | 74.07% | 81.49% | 81.86% | 38.76% |
| [flatbuffers 24.12.23][flatbuffers] | 13.80% | † | 41.77% | 60.96% | 67.65% | 25.15% |
| [msgpacker 0.4.5][msgpacker] | 30.74% | 32.22% | 66.58% | 75.95% | 75.27% | 34.39% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.36% | 11.97% | 54.87% | 68.72% | 67.76% | 25.50% |
| [nanoserde 0.1.37][nanoserde] | 55.39% | 74.38% | 53.59% | 77.37% | 81.15% | 32.77% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.70% | 68.57% | 73.58% | 82.49% | 84.72% | 37.75% |
| [postcard 1.1.1][postcard] | 33.17% | 50.48% | 74.07% | 81.01% | 82.21% | 37.98% |
| [pot 3.0.1][pot] | 5.35% | 7.16% | 37.29% | 59.24% | 65.84% | 21.06% |
| [prost 0.13.4][prost] | <span title="encode">*14.59%\**</span> <span title="populate + encode">*8.12%\**</span> | 23.83% | 52.22% | 65.62% | 66.07% | 28.02% |
| [rkyv 0.8.9][rkyv] | 70.53% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.50%\**</span> | 46.79% | 63.45% | 70.63% | 26.06% |
| [rmp-serde 1.3.0][rmp-serde] | 7.16% | 19.79% | 55.65% | 69.60% | 69.65% | 29.77% |
| [ron 0.8.1][ron] | 1.91% | 2.38% | 11.19% | 39.31% | 46.83% | 9.81% |
| [savefile 0.18.5][savefile] | 83.18% | 77.88% | 54.22% | 77.84% | 81.30% | 32.47% |
| [serde-brief 0.1.0][serde-brief] | 11.15% | 9.67% | 13.97% | 48.88% | 54.49% | 14.24% |
| [serde_bare 0.5.0][serde_bare] | 15.12% | 45.63% | 73.58% | 82.63% | 84.92% | 37.67% |
| [serde_cbor 0.11.2][serde_cbor] | 7.04% | 10.10% | 16.16% | 51.80% | 58.33% | 15.42% |
| [serde_json 1.0.134][serde_json] | 3.59% | 7.16% | 10.34% | 36.71% | 46.42% | 9.57% |
| [simd-json 0.14.3][simd-json] | 6.33% | 8.41% | 10.34% | 36.71% | 46.42% | 9.75% |
| [speedy 0.8.7][speedy] | 92.72% | 88.16% | 61.29% | 78.41% | 82.46% | 33.48% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*61.78%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.04%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.95%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
