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

## Last updated: 2025-1-23 18:8:19

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
| [bilrost 0.1012.1][bilrost] | <span title="encode">*486.26 µs\**</span> <span title="prepend">*432.32 µs\**</span> | 2.6344 ms | 804955 | 328941 | 285485 | 4.6551 ms |
| [bincode 2.0.0-rc][bincode] | 328.94 µs | 2.4312 ms | 741295 | 303944 | 257153 | 4.0378 ms |
| [bincode 1.3.3][bincode1] | 531.32 µs | 2.0146 ms | 1045784 | 373127 | 311761 | 4.8571 ms |
| [bitcode 0.6.3][bitcode] | 139.26 µs | 1.4400 ms | 703710 | 288826 | 229755 | 2.4389 ms |
| [borsh 1.5.3][borsh] | 545.55 µs | 2.1616 ms | 885780 | 362204 | 286514 | 4.7054 ms |
| [capnp 0.20.3][capnp] | 522.48 µs | † | 1443216 | 513986 | 428649 | 6.5633 ms |
| [cbor4ii 0.3.3][cbor4ii] | 591.77 µs | 4.9448 ms | 1407835 | 403440 | 324081 | 4.8401 ms |
| [ciborium 0.2.2][ciborium] | 4.0241 ms | 12.735 ms | 1407835 | 403440 | 324081 | 4.9056 ms |
| [databuf 0.5.0][databuf] | 267.86 µs | 2.0153 ms | 765778 | 311715 | 264630 | 3.9539 ms |
| [dlhn 0.1.7][dlhn] | 749.15 µs | 2.5706 ms | 724953 | 301446 | 253629 | 3.6523 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0649 ms | † | 1276368 | 468539 | 388832 | 5.2655 ms |
| [minicbor 0.25.1][minicbor] | 744.82 µs | 3.0007 ms | 817830 | 332671 | 284548 | 4.3070 ms |
| [msgpacker 0.4.5][msgpacker] | 1.3765 ms | 2.5673 ms | 764996 | 315291 | 264898 | 3.8845 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3563 ms | 4.1866 ms | 818669 | 332556 | 285514 | 4.2778 ms |
| [nanoserde 0.1.37][nanoserde] | 252.58 µs | 2.0463 ms | 1045784 | 373127 | 311761 | 4.5426 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 659.71 µs | 2.1886 ms | 765778 | 311743 | 264518 | 4.1410 ms |
| [postcard 1.1.1][postcard] | 428.96 µs | 2.1668 ms | 724953 | 302399 | 253747 | 3.7788 ms |
| [pot 3.0.1][pot] | 2.4579 ms | 6.3195 ms | 971922 | 372513 | 304122 | 4.5952 ms |
| [prost 0.13.4][prost] | <span title="encode">*966.61 µs\**</span> <span title="populate + encode">*2.4350 ms\**</span> | 3.2408 ms | 884628 | 363130 | 315494 | 5.0680 ms |
| [rkyv 0.8.9][rkyv] | 247.50 µs | <span title="unvalidated">*1.5475 ms\**</span> <span title="validated upfront with error">*2.1503 ms\**</span> | 1011488 | 393526 | 326517 | 4.9336 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4228 ms | 3.2005 ms | 784997 | 325384 | 278219 | 4.1093 ms |
| [ron 0.8.1][ron] | 11.210 ms | 15.293 ms | 1607459 | 449158 | 349713 | 5.7938 ms |
| [savefile 0.18.5][savefile] | 190.36 µs | 2.1482 ms | 1045800 | 373139 | 311761 | 4.5217 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5080 ms | 5.0370 ms | 1584946 | 413733 | 341439 | 4.9457 ms |
| [serde_bare 0.5.0][serde_bare] | 692.14 µs | 2.0714 ms | 765778 | 311715 | 264630 | 3.8459 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0229 ms | 4.8920 ms | 1407835 | 403440 | 324081 | 4.7349 ms |
| [serde_json 1.0.134][serde_json] | 4.1179 ms | 6.0103 ms | 1827461 | 470560 | 361090 | 5.5315 ms |
| [simd-json 0.14.3][simd-json] | 2.1573 ms | 4.6363 ms | 1827461 | 470560 | 361090 | 5.8553 ms |
| [speedy 0.8.7][speedy] | 199.42 µs | 1.7446 ms | 885780 | 362204 | 286514 | 4.2011 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*71.961 ns\**</span> | <span title="validated on-demand with error">*168.74 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4966 ns\**</span> <span title="validated upfront with error">*1.9781 ms\**</span> | <span title="unvalidated">*51.002 µs\**</span> <span title="validated upfront with error">*2.0598 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2435 ns\**</span> <span title="validated upfront with error">*592.50 µs\**</span> | <span title="unvalidated">*10.462 µs\**</span> <span title="validated upfront with error">*598.55 µs\**</span> | <span title="unvalidated">*7.3752 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*28.64%\**</span> <span title="prepend">*32.21%\**</span> | 54.66% | 87.42% | 87.80% | 80.48% | 52.39% |
| [bincode 2.0.0-rc][bincode] | 42.34% | 59.23% | 94.93% | 95.03% | 89.35% | 60.40% |
| [bincode 1.3.3][bincode1] | 26.21% | 71.48% | 67.29% | 77.41% | 73.70% | 50.21% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 25.53% | 66.62% | 79.45% | 79.74% | 80.19% | 51.83% |
| [capnp 0.20.3][capnp] | 26.65% | † | 48.76% | 56.19% | 53.60% | 37.16% |
| [cbor4ii 0.3.3][cbor4ii] | 23.53% | 29.12% | 49.99% | 71.59% | 70.89% | 50.39% |
| [ciborium 0.2.2][ciborium] | 3.46% | 11.31% | 49.99% | 71.59% | 70.89% | 49.72% |
| [databuf 0.5.0][databuf] | 51.99% | 71.45% | 91.89% | 92.66% | 86.82% | 61.68% |
| [dlhn 0.1.7][dlhn] | 18.59% | 56.02% | 97.07% | 95.81% | 90.59% | 66.78% |
| [flatbuffers 24.12.23][flatbuffers] | 13.08% | † | 55.13% | 61.64% | 59.09% | 46.32% |
| [minicbor 0.25.1][minicbor] | 18.70% | 47.99% | 86.05% | 86.82% | 80.74% | 56.63% |
| [msgpacker 0.4.5][msgpacker] | 10.12% | 56.09% | 91.99% | 91.61% | 86.73% | 62.79% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.60% | 34.40% | 85.96% | 86.85% | 80.47% | 57.01% |
| [nanoserde 0.1.37][nanoserde] | 55.14% | 70.37% | 67.29% | 77.41% | 73.70% | 53.69% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.11% | 65.80% | 91.89% | 92.65% | 86.86% | 58.90% |
| [postcard 1.1.1][postcard] | 32.46% | 66.46% | 97.07% | 95.51% | 90.54% | 64.54% |
| [pot 3.0.1][pot] | 5.67% | 22.79% | 72.40% | 77.53% | 75.55% | 53.07% |
| [prost 0.13.4][prost] | <span title="encode">*14.41%\**</span> <span title="populate + encode">*5.72%\**</span> | 44.43% | 79.55% | 79.54% | 72.82% | 48.12% |
| [rkyv 0.8.9][rkyv] | 56.27% | <span title="unvalidated">*93.05%\**</span> <span title="validated upfront with error">*66.97%\**</span> | 69.57% | 73.39% | 70.37% | 49.43% |
| [rmp-serde 1.3.0][rmp-serde] | 9.79% | 44.99% | 89.64% | 88.76% | 82.58% | 59.35% |
| [ron 0.8.1][ron] | 1.24% | 9.42% | 43.78% | 64.30% | 65.70% | 42.09% |
| [savefile 0.18.5][savefile] | 73.16% | 67.03% | 67.29% | 77.40% | 73.70% | 53.94% |
| [serde-brief 0.1.0][serde-brief] | 9.23% | 28.59% | 44.40% | 69.81% | 67.29% | 49.31% |
| [serde_bare 0.5.0][serde_bare] | 20.12% | 69.52% | 91.89% | 92.66% | 86.82% | 63.42% |
| [serde_cbor 0.11.2][serde_cbor] | 6.88% | 29.44% | 49.99% | 71.59% | 70.89% | 51.51% |
| [serde_json 1.0.134][serde_json] | 3.38% | 23.96% | 38.51% | 61.38% | 63.63% | 44.09% |
| [simd-json 0.14.3][simd-json] | 6.46% | 31.06% | 38.51% | 61.38% | 63.63% | 41.65% |
| [speedy 0.8.7][speedy] | 69.83% | 82.54% | 79.45% | 79.74% | 80.19% | 58.05% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.73%\**</span> | <span title="validated on-demand with error">*6.20%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.81%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.51%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.75%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*6.7324 ms\**</span> <span title="prepend">*8.8941 ms\**</span> | 8.3364 ms | 8625005 | 6443961 | 6231572 | 71.068 ms |
| [bincode 2.0.0-rc][bincode] | 2.4122 ms | 1.0236 ms | 6000005 | 5378497 | 5345897 | 7.4322 ms |
| [bincode 1.3.3][bincode1] | 5.2046 ms | 4.8436 ms | 6000008 | 5378500 | 5345890 | 7.3269 ms |
| [bitcode 0.6.3][bitcode] | 1.4087 ms | 799.71 µs | 6000006 | 5182295 | 4923880 | 12.428 ms |
| [borsh 1.5.3][borsh] | 6.2469 ms | 4.1573 ms | 6000004 | 5378496 | 5345889 | 7.7945 ms |
| [capnp 0.20.3][capnp] | 5.7805 ms | † | 14000088 | 7130367 | 6051062 | 79.955 ms |
| [cbor4ii 0.3.3][cbor4ii] | 10.558 ms | 50.120 ms | 13125016 | 7524114 | 6757967 | 89.139 ms |
| [ciborium 0.2.2][ciborium] | 70.040 ms | 118.72 ms | 13122324 | 7524660 | 6759658 | 88.237 ms |
| [databuf 0.5.0][databuf] | 2.4113 ms | 5.3486 ms | 6000003 | 5378495 | 5345900 | 7.4257 ms |
| [dlhn 0.1.7][dlhn] | 6.4105 ms | 6.7591 ms | 6000003 | 5378495 | 5345900 | 7.4644 ms |
| [flatbuffers 24.12.23][flatbuffers] | 884.63 µs | † | 6000024 | 5378434 | 5345910 | 7.7058 ms |
| [minicbor 0.25.1][minicbor] | 5.1980 ms | 11.673 ms | 8125006 | 6494907 | 6390894 | 68.894 ms |
| [msgpacker 0.4.5][msgpacker] | 18.538 ms | 5.1868 ms | 7500005 | 6058442 | 6014337 | 9.6521 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.86 ms | 32.477 ms | 8125037 | 6493484 | 6386940 | 69.541 ms |
| [nanoserde 0.1.37][nanoserde] | 1.6523 ms | 1.1119 ms | 6000008 | 5378500 | 5345890 | 7.3812 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1570 ms | 4.0156 ms | 6000004 | 5378496 | 5345889 | 7.3500 ms |
| [postcard 1.1.1][postcard] | 480.08 µs | 1.2157 ms | 6000003 | 5378495 | 5345900 | 7.5632 ms |
| [pot 3.0.1][pot] | 38.530 ms | 68.492 ms | 10122342 | 6814618 | 6852251 | 80.805 ms |
| [prost 0.13.4][prost] | <span title="encode">*7.8240 ms\**</span> <span title="populate + encode">*8.3168 ms\**</span> | 14.247 ms | 8750000 | 6665735 | 6421871 | 73.799 ms |
| [rkyv 0.8.9][rkyv] | 199.70 µs | <span title="unvalidated">*212.37 µs\**</span> <span title="validated upfront with error">*149.23 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.5641 ms |
| [rmp-serde 1.3.0][rmp-serde] | 18.603 ms | 18.263 ms | 8125006 | 6494876 | 6391037 | 69.162 ms |
| [ron 0.8.1][ron] | 173.19 ms | 238.24 ms | 22192885 | 8970395 | 8138755 | 146.24 ms |
| [savefile 0.18.5][savefile] | 148.02 µs | 148.16 µs | 6000024 | 5378519 | 5345892 | 7.5800 ms |
| [serde-brief 0.1.0][serde-brief] | 22.672 ms | 47.520 ms | 15750015 | 8024540 | 6816643 | 91.797 ms |
| [serde_bare 0.5.0][serde_bare] | 6.3803 ms | 4.7613 ms | 6000003 | 5378495 | 5345900 | 7.3894 ms |
| [serde_cbor 0.11.2][serde_cbor] | 32.104 ms | 47.100 ms | 13122324 | 7524660 | 6759658 | 89.560 ms |
| [serde_json 1.0.134][serde_json] | 88.742 ms | 88.430 ms | 26192883 | 9566084 | 8586741 | 154.44 ms |
| [simd-json 0.14.3][simd-json] | 51.804 ms | 70.731 ms | 26192883 | 9566084 | 8586741 | 154.86 ms |
| [speedy 0.8.7][speedy] | 199.16 µs | 198.44 µs | 6000004 | 5378496 | 5345889 | 7.4009 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*104.07 ns\**</span> | <span title="validated on-demand with error">*2.1829 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4854 ns\**</span> <span title="validated upfront with error">*39.822 ns\**</span> | <span title="unvalidated">*54.389 µs\**</span> <span title="validated upfront with error">*78.182 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2458 ns\**</span> <span title="validated upfront with error">*4.9763 ns\**</span> | <span title="unvalidated">*48.709 µs\**</span> <span title="validated upfront with error">*77.721 µs\**</span> | <span title="unvalidated">*103.11 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*2.20%\**</span> <span title="prepend">*1.66%\**</span> | 1.78% | 69.57% | 80.42% | 79.02% | 10.31% |
| [bincode 2.0.0-rc][bincode] | 6.14% | 14.47% | 100.00% | 96.35% | 92.11% | 98.58% |
| [bincode 1.3.3][bincode1] | 2.84% | 3.06% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bitcode 0.6.3][bitcode] | 10.51% | 18.53% | 100.00% | 100.00% | 100.00% | 58.95% |
| [borsh 1.5.3][borsh] | 2.37% | 3.56% | 100.00% | 96.35% | 92.11% | 94.00% |
| [capnp 0.20.3][capnp] | 2.56% | † | 42.86% | 72.68% | 81.37% | 9.16% |
| [cbor4ii 0.3.3][cbor4ii] | 1.40% | 0.30% | 45.71% | 68.88% | 72.86% | 8.22% |
| [ciborium 0.2.2][ciborium] | 0.21% | 0.12% | 45.72% | 68.87% | 72.84% | 8.30% |
| [databuf 0.5.0][databuf] | 6.14% | 2.77% | 100.00% | 96.35% | 92.11% | 98.67% |
| [dlhn 0.1.7][dlhn] | 2.31% | 2.19% | 100.00% | 96.35% | 92.11% | 98.16% |
| [flatbuffers 24.12.23][flatbuffers] | 16.73% | † | 100.00% | 96.35% | 92.11% | 95.08% |
| [minicbor 0.25.1][minicbor] | 2.85% | 1.27% | 73.85% | 79.79% | 77.05% | 10.64% |
| [msgpacker 0.4.5][msgpacker] | 0.80% | 2.86% | 80.00% | 85.54% | 81.87% | 75.91% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.46% | 73.85% | 79.81% | 77.09% | 10.54% |
| [nanoserde 0.1.37][nanoserde] | 8.96% | 13.32% | 100.00% | 96.35% | 92.11% | 99.26% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.87% | 3.69% | 100.00% | 96.35% | 92.11% | 99.69% |
| [postcard 1.1.1][postcard] | 30.83% | 12.19% | 100.00% | 96.35% | 92.11% | 96.88% |
| [pot 3.0.1][pot] | 0.38% | 0.22% | 59.27% | 76.05% | 71.86% | 9.07% |
| [prost 0.13.4][prost] | <span title="encode">*1.89%\**</span> <span title="populate + encode">*1.78%\**</span> | 1.04% | 68.57% | 77.75% | 76.67% | 9.93% |
| [rkyv 0.8.9][rkyv] | 74.12% | <span title="unvalidated">*69.77%\**</span> <span title="validated upfront with error">*99.28%\**</span> | 100.00% | 96.35% | 92.11% | 96.86% |
| [rmp-serde 1.3.0][rmp-serde] | 0.80% | 0.81% | 73.85% | 79.79% | 77.04% | 10.59% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 5.01% |
| [savefile 0.18.5][savefile] | 100.00% | 100.00% | 100.00% | 96.35% | 92.11% | 96.66% |
| [serde-brief 0.1.0][serde-brief] | 0.65% | 0.31% | 38.10% | 64.58% | 72.23% | 7.98% |
| [serde_bare 0.5.0][serde_bare] | 2.32% | 3.11% | 100.00% | 96.35% | 92.11% | 99.15% |
| [serde_cbor 0.11.2][serde_cbor] | 0.46% | 0.31% | 45.72% | 68.87% | 72.84% | 8.18% |
| [serde_json 1.0.134][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.74% |
| [simd-json 0.14.3][simd-json] | 0.29% | 0.21% | 22.91% | 54.17% | 57.34% | 4.73% |
| [speedy 0.8.7][speedy] | 74.32% | 74.66% | 100.00% | 96.35% | 92.11% | 99.00% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.20%\**</span> | <span title="validated on-demand with error">*2.23%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.12%\**</span> <span title="validated upfront with error">*3.13%\**</span> | <span title="unvalidated">*89.56%\**</span> <span title="validated upfront with error">*62.30%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*25.03%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*62.67%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*941.30 µs\**</span> <span title="prepend">*838.78 µs\**</span> | 3.3996 ms | 489348 | 281173 | 249546 | 3.1046 ms |
| [bincode 2.0.0-rc][bincode] | 309.83 µs | 2.0710 ms | 367413 | 221291 | 206273 | 2.4506 ms |
| [bincode 1.3.3][bincode1] | 595.67 µs | 1.8126 ms | 569975 | 240525 | 232423 | 2.9010 ms |
| [bitcode 0.6.3][bitcode] | 129.03 µs | 1.2313 ms | 327688 | 200947 | 182736 | 728.19 µs |
| [borsh 1.5.3][borsh] | 554.04 µs | 1.8363 ms | 446595 | 234236 | 210008 | 2.4549 ms |
| [capnp 0.20.3][capnp] | 450.77 µs | † | 803896 | 335606 | 280851 | 4.0027 ms |
| [cbor4ii 0.3.3][cbor4ii] | 780.40 µs | 4.6921 ms | 1109831 | 344745 | 274514 | 3.7848 ms |
| [ciborium 0.2.2][ciborium] | 3.7341 ms | 10.294 ms | 1109821 | 344751 | 274526 | 3.8029 ms |
| [databuf 0.5.0][databuf] | 320.34 µs | 1.7056 ms | 356311 | 213062 | 198488 | 2.3817 ms |
| [dlhn 0.1.7][dlhn] | 761.81 µs | 2.6112 ms | 366496 | 220600 | 205683 | 2.4759 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.2034 ms | † | 844168 | 345696 | 294015 | 3.8248 ms |
| [minicbor 0.25.1][minicbor] | 540.51 µs | 3.3439 ms | 428773 | 249857 | 228741 | 2.6948 ms |
| [msgpacker 0.4.5][msgpacker] | 981.91 µs | 2.8412 ms | 391251 | 236877 | 220476 | 2.6424 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1256 ms | 3.9391 ms | 449745 | 252432 | 231110 | 2.8348 ms |
| [nanoserde 0.1.37][nanoserde] | 270.42 µs | 1.8694 ms | 567975 | 239930 | 232419 | 2.8616 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 619.25 µs | 1.9579 ms | 356311 | 212976 | 198524 | 2.3678 ms |
| [postcard 1.1.1][postcard] | 438.31 µs | 1.9586 ms | 367489 | 221913 | 207344 | 2.5530 ms |
| [pot 3.0.1][pot] | 2.3772 ms | 5.9477 ms | 599125 | 299158 | 247693 | 3.1902 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.2850 ms\**</span> <span title="populate + encode">*2.9288 ms\**</span> | 3.3713 ms | 596811 | 305319 | 269310 | 3.4184 ms |
| [rkyv 0.8.9][rkyv] | 339.20 µs | <span title="unvalidated">*1.4730 ms\**</span> <span title="validated upfront with error">*1.9975 ms\**</span> | 603776 | 254776 | 220087 | 2.7153 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5736 ms | 3.0162 ms | 424533 | 245214 | 226188 | 2.6889 ms |
| [ron 0.8.1][ron] | 7.1106 ms | 16.932 ms | 1465223 | 434935 | 343338 | 5.7726 ms |
| [savefile 0.18.5][savefile] | 210.05 µs | 1.7835 ms | 566991 | 239362 | 232010 | 2.8460 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3373 ms | 5.3455 ms | 1276014 | 373898 | 293679 | 3.9796 ms |
| [serde_bare 0.5.0][serde_bare] | 761.22 µs | 2.3198 ms | 356311 | 213062 | 198488 | 2.4030 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8439 ms | 4.7177 ms | 1109821 | 344751 | 274526 | 3.7698 ms |
| [serde_json 1.0.134][serde_json] | 3.8460 ms | 6.8187 ms | 1623191 | 466527 | 359623 | 5.9963 ms |
| [simd-json 0.14.3][simd-json] | 2.2691 ms | 4.5985 ms | 1623191 | 466527 | 359623 | 5.9644 ms |
| [speedy 0.8.7][speedy] | 262.30 µs | 1.5671 ms | 449595 | 234970 | 210361 | 2.5004 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.038 ns\**</span> | <span title="validated on-demand with error">*410.19 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4860 ns\**</span> <span title="validated upfront with error">*2.2572 ms\**</span> | <span title="unvalidated">*1.3637 µs\**</span> <span title="validated upfront with error">*2.2707 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*500.94 µs\**</span> | <span title="unvalidated">*240.26 ns\**</span> <span title="validated upfront with error">*501.03 µs\**</span> | <span title="unvalidated">*760.20 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*13.71%\**</span> <span title="prepend">*15.38%\**</span> | 36.22% | 66.96% | 71.47% | 73.23% | 23.46% |
| [bincode 2.0.0-rc][bincode] | 41.65% | 59.45% | 89.19% | 90.81% | 88.59% | 29.71% |
| [bincode 1.3.3][bincode1] | 21.66% | 67.93% | 57.49% | 83.55% | 78.62% | 25.10% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 23.29% | 67.05% | 73.37% | 85.79% | 87.01% | 29.66% |
| [capnp 0.20.3][capnp] | 28.62% | † | 40.76% | 59.88% | 65.07% | 18.19% |
| [cbor4ii 0.3.3][cbor4ii] | 16.53% | 26.24% | 29.53% | 58.29% | 66.57% | 19.24% |
| [ciborium 0.2.2][ciborium] | 3.46% | 11.96% | 29.53% | 58.29% | 66.56% | 19.15% |
| [databuf 0.5.0][databuf] | 40.28% | 72.19% | 91.97% | 94.31% | 92.06% | 30.57% |
| [dlhn 0.1.7][dlhn] | 16.94% | 47.15% | 89.41% | 91.09% | 88.84% | 29.41% |
| [flatbuffers 24.12.23][flatbuffers] | 4.03% | † | 38.82% | 58.13% | 62.15% | 19.04% |
| [minicbor 0.25.1][minicbor] | 23.87% | 36.82% | 76.42% | 80.42% | 79.89% | 27.02% |
| [msgpacker 0.4.5][msgpacker] | 13.14% | 43.34% | 83.75% | 84.83% | 82.88% | 27.56% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.52% | 31.26% | 72.86% | 79.60% | 79.07% | 25.69% |
| [nanoserde 0.1.37][nanoserde] | 47.71% | 65.87% | 57.69% | 83.75% | 78.62% | 25.45% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 20.84% | 62.89% | 91.97% | 94.35% | 92.05% | 30.75% |
| [postcard 1.1.1][postcard] | 29.44% | 62.87% | 89.17% | 90.55% | 88.13% | 28.52% |
| [pot 3.0.1][pot] | 5.43% | 20.70% | 54.69% | 67.17% | 73.78% | 22.83% |
| [prost 0.13.4][prost] | <span title="encode">*10.04%\**</span> <span title="populate + encode">*4.41%\**</span> | 36.52% | 54.91% | 65.82% | 67.85% | 21.30% |
| [rkyv 0.8.9][rkyv] | 38.04% | <span title="unvalidated">*83.59%\**</span> <span title="validated upfront with error">*61.64%\**</span> | 54.27% | 78.87% | 83.03% | 26.82% |
| [rmp-serde 1.3.0][rmp-serde] | 8.20% | 40.82% | 77.19% | 81.95% | 80.79% | 27.08% |
| [ron 0.8.1][ron] | 1.81% | 7.27% | 22.36% | 46.20% | 53.22% | 12.61% |
| [savefile 0.18.5][savefile] | 61.43% | 69.04% | 57.79% | 83.95% | 78.76% | 25.59% |
| [serde-brief 0.1.0][serde-brief] | 9.65% | 23.03% | 25.68% | 53.74% | 62.22% | 18.30% |
| [serde_bare 0.5.0][serde_bare] | 16.95% | 53.08% | 91.97% | 94.31% | 92.06% | 30.30% |
| [serde_cbor 0.11.2][serde_cbor] | 7.00% | 26.10% | 29.53% | 58.29% | 66.56% | 19.32% |
| [serde_json 1.0.134][serde_json] | 3.35% | 18.06% | 20.19% | 43.07% | 50.81% | 12.14% |
| [simd-json 0.14.3][simd-json] | 5.69% | 26.78% | 20.19% | 43.07% | 50.81% | 12.21% |
| [speedy 0.8.7][speedy] | 49.19% | 78.57% | 72.89% | 85.52% | 86.87% | 29.12% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*58.57%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.62%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*4.5537 ms\**</span> <span title="prepend">*2.5960 ms\**</span> | 8.3601 ms | 1704643 | 1294259 | 1245607 | 11.426 ms |
| [bincode 2.0.0-rc][bincode] | 1.2004 ms | 3.9012 ms | 1406257 | 1117802 | 1062238 | 9.4091 ms |
| [bincode 1.3.3][bincode1] | 4.0703 ms | 4.2230 ms | 1854234 | 1141994 | 1050351 | 10.238 ms |
| [bitcode 0.6.3][bitcode] | 710.19 µs | 2.3510 ms | 971318 | 878034 | 855922 | 3.3940 ms |
| [borsh 1.5.3][borsh] | 2.9329 ms | 2.8897 ms | 1521989 | 1108471 | 1038408 | 9.8472 ms |
| [capnp 0.20.3][capnp] | 2.2104 ms | † | 2724288 | 1546992 | 1240354 | 14.388 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.2991 ms | 18.412 ms | 6012539 | 1695215 | 1467194 | 21.437 ms |
| [ciborium 0.2.2][ciborium] | 22.751 ms | 54.152 ms | 6012373 | 1695146 | 1467435 | 21.408 ms |
| [databuf 0.5.0][databuf] | 1.3026 ms | 3.8136 ms | 1319999 | 1062631 | 1007898 | 8.7904 ms |
| [dlhn 0.1.7][dlhn] | 4.9459 ms | 8.4472 ms | 1311281 | 1077520 | 1045571 | 9.0143 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.1539 ms | † | 2325620 | 1440289 | 1265148 | 13.239 ms |
| [minicbor 0.25.1][minicbor] | 2.1405 ms | 11.867 ms | 1777386 | 1276218 | 1252036 | 12.328 ms |
| [msgpacker 0.4.5][msgpacker] | 2.3341 ms | 6.8557 ms | 1458773 | 1156055 | 1137194 | 9.6744 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.668 ms | 18.296 ms | 1770060 | 1277755 | 1263142 | 12.297 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2980 ms | 2.9342 ms | 1812404 | 1134820 | 1054758 | 10.727 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.1617 ms | 3.1989 ms | 1319999 | 1064380 | 1010284 | 8.7632 ms |
| [postcard 1.1.1][postcard] | 2.1551 ms | 4.2828 ms | 1311281 | 1083900 | 1041114 | 8.7277 ms |
| [pot 3.0.1][pot] | 13.887 ms | 29.901 ms | 2604812 | 1482233 | 1299952 | 15.579 ms |
| [prost 0.13.4][prost] | <span title="encode">*5.4450 ms\**</span> <span title="populate + encode">*9.2988 ms\**</span> | 8.6866 ms | 1859886 | 1338076 | 1295497 | 11.930 ms |
| [rkyv 0.8.9][rkyv] | 1.0022 ms | <span title="unvalidated">*2.1910 ms\**</span> <span title="validated upfront with error">*2.6337 ms\**</span> | 2075936 | 1383779 | 1211892 | 12.816 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.345 ms | 11.245 ms | 1745322 | 1261627 | 1228902 | 11.728 ms |
| [ron 0.8.1][ron] | 37.898 ms | 97.033 ms | 8677703 | 2233642 | 1827843 | 34.639 ms |
| [savefile 0.18.5][savefile] | 852.26 µs | 2.7557 ms | 1791505 | 1128012 | 1052757 | 10.343 ms |
| [serde-brief 0.1.0][serde-brief] | 6.4518 ms | 22.549 ms | 6951772 | 1796265 | 1570903 | 23.902 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7096 ms | 4.7600 ms | 1319999 | 1062645 | 1007918 | 8.8947 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.189 ms | 21.755 ms | 6012373 | 1695146 | 1467435 | 21.584 ms |
| [serde_json 1.0.134][serde_json] | 20.675 ms | 31.007 ms | 9390461 | 2391679 | 1843922 | 34.751 ms |
| [simd-json 0.14.3][simd-json] | 11.930 ms | 25.834 ms | 9390461 | 2391679 | 1843922 | 34.502 ms |
| [speedy 0.8.7][speedy] | 782.45 µs | 2.4930 ms | 1584734 | 1119837 | 1038012 | 10.293 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*74.653 ns\**</span> | <span title="validated on-demand with error">*710.71 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4929 ns\**</span> <span title="validated upfront with error">*5.4606 ms\**</span> | <span title="unvalidated">*2.6211 µs\**</span> <span title="validated upfront with error">*5.2234 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*442.30 µs\**</span> | <span title="unvalidated">*437.37 ns\**</span> <span title="validated upfront with error">*442.03 µs\**</span> | <span title="unvalidated">*235.19 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.1][bilrost] | <span title="encode">*15.60%\**</span> <span title="prepend">*27.36%\**</span> | 26.21% | 56.98% | 67.84% | 68.72% | 29.70% |
| [bincode 2.0.0-rc][bincode] | 59.16% | 56.16% | 69.07% | 78.55% | 80.58% | 36.07% |
| [bincode 1.3.3][bincode1] | 17.45% | 51.88% | 52.38% | 76.89% | 81.49% | 33.15% |
| [bitcode 0.6.3][bitcode] | 100.00% | 93.19% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.21% | 75.82% | 63.82% | 79.21% | 82.43% | 34.47% |
| [capnp 0.20.3][capnp] | 32.13% | † | 35.65% | 56.76% | 69.01% | 23.59% |
| [cbor4ii 0.3.3][cbor4ii] | 21.53% | 11.90% | 16.15% | 51.79% | 58.34% | 15.83% |
| [ciborium 0.2.2][ciborium] | 3.12% | 4.05% | 16.16% | 51.80% | 58.33% | 15.85% |
| [databuf 0.5.0][databuf] | 54.52% | 57.45% | 73.58% | 82.63% | 84.92% | 38.61% |
| [dlhn 0.1.7][dlhn] | 14.36% | 25.94% | 74.07% | 81.49% | 81.86% | 37.65% |
| [flatbuffers 24.12.23][flatbuffers] | 13.78% | † | 41.77% | 60.96% | 67.65% | 25.64% |
| [minicbor 0.25.1][minicbor] | 33.18% | 18.46% | 54.65% | 68.80% | 68.36% | 27.53% |
| [msgpacker 0.4.5][msgpacker] | 30.43% | 31.96% | 66.58% | 75.95% | 75.27% | 35.08% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.39% | 11.98% | 54.87% | 68.72% | 67.76% | 27.60% |
| [nanoserde 0.1.37][nanoserde] | 54.71% | 74.67% | 53.59% | 77.37% | 81.15% | 31.64% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.46% | 68.49% | 73.58% | 82.49% | 84.72% | 38.73% |
| [postcard 1.1.1][postcard] | 32.95% | 51.16% | 74.07% | 81.01% | 82.21% | 38.89% |
| [pot 3.0.1][pot] | 5.11% | 7.33% | 37.29% | 59.24% | 65.84% | 21.79% |
| [prost 0.13.4][prost] | <span title="encode">*13.04%\**</span> <span title="populate + encode">*7.64%\**</span> | 25.22% | 52.22% | 65.62% | 66.07% | 28.45% |
| [rkyv 0.8.9][rkyv] | 70.86% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.19%\**</span> | 46.79% | 63.45% | 70.63% | 26.48% |
| [rmp-serde 1.3.0][rmp-serde] | 6.87% | 19.48% | 55.65% | 69.60% | 69.65% | 28.94% |
| [ron 0.8.1][ron] | 1.87% | 2.26% | 11.19% | 39.31% | 46.83% | 9.80% |
| [savefile 0.18.5][savefile] | 83.33% | 79.51% | 54.22% | 77.84% | 81.30% | 32.82% |
| [serde-brief 0.1.0][serde-brief] | 11.01% | 9.72% | 13.97% | 48.88% | 54.49% | 14.20% |
| [serde_bare 0.5.0][serde_bare] | 15.08% | 46.03% | 73.58% | 82.63% | 84.92% | 38.16% |
| [serde_cbor 0.11.2][serde_cbor] | 6.97% | 10.07% | 16.16% | 51.80% | 58.33% | 15.72% |
| [serde_json 1.0.134][serde_json] | 3.44% | 7.07% | 10.34% | 36.71% | 46.42% | 9.77% |
| [simd-json 0.14.3][simd-json] | 5.95% | 8.48% | 10.34% | 36.71% | 46.42% | 9.84% |
| [speedy 0.8.7][speedy] | 90.76% | 87.89% | 61.29% | 78.41% | 82.46% | 32.97% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.67%\**</span> | <span title="validated on-demand with error">*61.54%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.86%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.69%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
