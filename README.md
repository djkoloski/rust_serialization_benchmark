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

## Last updated: 2024-4-27 2:47:3

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.77.0-nightly (11f32b73e 2024-01-31)
binary: rustc
commit-hash: 11f32b73e0dc9287e305b5b9980d24aecdc8c17f
commit-date: 2024-01-31
host: x86_64-unknown-linux-gnu
release: 1.77.0-nightly
LLVM version: 17.0.6
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
Vulnerability Spectre v2:           Mitigation; Retpolines, STIBP disabled, RSB filling, PBRSB-eIBRS Not affected
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
| [abomonation 0.7.3][abomonation] | 404.20 µs | <span title="unvalidated">*1.4556 ms\**</span> | 1705800 | 520075 | 413391 | 6.7116 ms |
| [alkahest 0.1.5][alkahest] | 190.01 µs | † | 1045784 | 454157 | 389424 | 5.9993 ms |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*774.75 µs\**</span> <span title="prepend">*742.67 µs\**</span> | 3.2446 ms | 874632 | 355446 | 311723 | 5.1215 ms |
| [bincode 2.0.0-rc][bincode] | 210.78 µs | 2.4866 ms | 741295 | 303944 | 257153 | 3.9701 ms |
| [bincode 1.3.3][bincode1] | 522.55 µs | 1.9951 ms | 1045784 | 373127 | 311761 | 4.8023 ms |
| [bitcode 0.6.0][bitcode] | 145.83 µs | 1.4946 ms | 703710 | 288826 | 229755 | 2.4382 ms |
| [borsh 1.3.1][borsh] | 543.56 µs | 2.2135 ms | 885780 | 362204 | 286514 | 4.5604 ms |
| [bson 2.9.0][bson] | 2.2664 ms | 7.5434 ms | 1924682 | 532821 | 376270 | 5.6473 ms |
| [capnp 0.18.13][capnp] | 538.48 µs | † | 1443216 | 513986 | 428649 | 6.4076 ms |
| [cbor4ii 0.3.2][cbor4ii] | 904.83 µs | 5.0411 ms | 1407835 | 403440 | 324081 | 4.8045 ms |
| [ciborium 0.2.2][ciborium] | 3.9793 ms | 10.073 ms | 1407835 | 403440 | 324081 | 4.7849 ms |
| [databuf 0.5.0][databuf] | 264.82 µs | 2.0499 ms | 765778 | 311715 | 264630 | 3.8687 ms |
| [dlhn 0.1.6][dlhn] | 793.55 µs | 2.4765 ms | 724953 | 301446 | 253629 | 3.6260 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.5435 ms | † | 1276368 | 468539 | 388832 | 5.1872 ms |
| [msgpacker 0.4.3][msgpacker] | 1.0879 ms | 2.6638 ms | 764996 | 315291 | 264898 | 3.8810 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4287 ms | 4.1432 ms | 818669 | 332556 | 285514 | 4.3278 ms |
| [nanoserde 0.1.37][nanoserde] | 260.96 µs | 2.0592 ms | 1045784 | 373127 | 311761 | 4.5639 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 677.11 µs | 2.1913 ms | 765778 | 311743 | 264518 | 4.0906 ms |
| [postcard 1.0.8][postcard] | 416.14 µs | 2.2123 ms | 724953 | 302399 | 253747 | 3.7715 ms |
| [pot 3.0.0][pot] | 2.3256 ms | 6.7306 ms | 971922 | 372513 | 304122 | 4.6365 ms |
| [prost 0.12.4][prost] | <span title="encode">*955.17 µs\**</span> <span title="populate + encode">*2.7734 ms\**</span> | 3.4192 ms | 884628 | 363130 | 315494 | 5.0868 ms |
| [rkyv 0.7.44][rkyv] | 219.42 µs | <span title="unvalidated">*1.4470 ms\**</span> <span title="validated upfront with error">*1.9654 ms\**</span> | 1011488 | 383862 | 333545 | 4.8737 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.2250 ms | 3.5011 ms | 784997 | 325384 | 278219 | 4.1597 ms |
| [ron 0.8.1][ron] | 14.104 ms | 16.543 ms | 1607459 | 449158 | 349713 | 5.7352 ms |
| [savefile 0.16.5][savefile] | 203.37 µs | 2.1325 ms | 1045800 | 373139 | 311755 | 4.5862 ms |
| [serde_bare 0.5.0][serde_bare] | 674.90 µs | 2.1043 ms | 765778 | 311715 | 264630 | 3.8726 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8754 ms | 5.0411 ms | 1407835 | 403440 | 324081 | 4.8686 ms |
| [serde_json 1.0.115][serde_json] | 3.8432 ms | 5.7844 ms | 1827461 | 470560 | 361090 | 5.5719 ms |
| [simd-json 0.13.9][simd-json] | 2.0833 ms | 4.6666 ms | 1827461 | 470560 | 361090 | 5.6335 ms |
| [speedy 0.8.7][speedy] | 199.95 µs | 1.7506 ms | 885780 | 362204 | 286514 | 4.2705 ms |
| [wiring 0.2.1][wiring] | 203.68 µs | 1.9740 ms | 2091568 | 674994 | 276295 | 4.4498 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.768 µs\**</span> | <span title="unvalidated">*40.329 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8551 ns\**</span> | <span title="validated on-demand with panic">*25.075 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*72.975 ns\**</span> | <span title="validated on-demand with error">*168.93 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4734 ns\**</span> <span title="validated upfront with error">*1.9691 ms\**</span> | <span title="unvalidated">*51.905 µs\**</span> <span title="validated upfront with error">*1.9784 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2379 ns\**</span> <span title="validated upfront with error">*516.60 µs\**</span> | <span title="unvalidated">*10.449 µs\**</span> <span title="validated upfront with error">*522.74 µs\**</span> | 10.318 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 36.08% | <span title="unvalidated">*99.41%\**</span> | 41.25% | 55.54% | 55.58% | 36.33% |
| [alkahest 0.1.5][alkahest] | 76.75% | † | 67.29% | 63.60% | 59.00% | 40.64% |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*18.82%\**</span> <span title="prepend">*19.64%\**</span> | 44.60% | 80.46% | 81.26% | 73.70% | 47.61% |
| [bincode 2.0.0-rc][bincode] | 69.19% | 58.19% | 94.93% | 95.03% | 89.35% | 61.41% |
| [bincode 1.3.3][bincode1] | 27.91% | 72.53% | 67.29% | 77.41% | 73.70% | 50.77% |
| [bitcode 0.6.0][bitcode] | 100.00% | 96.82% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 26.83% | 65.37% | 79.45% | 79.74% | 80.19% | 53.46% |
| [bson 2.9.0][bson] | 6.43% | 19.18% | 36.56% | 54.21% | 61.06% | 43.17% |
| [capnp 0.18.13][capnp] | 27.08% | † | 48.76% | 56.19% | 53.60% | 38.05% |
| [cbor4ii 0.3.2][cbor4ii] | 16.12% | 28.70% | 49.99% | 71.59% | 70.89% | 50.75% |
| [ciborium 0.2.2][ciborium] | 3.66% | 14.37% | 49.99% | 71.59% | 70.89% | 50.96% |
| [databuf 0.5.0][databuf] | 55.07% | 70.59% | 91.89% | 92.66% | 86.82% | 63.02% |
| [dlhn 0.1.6][dlhn] | 18.38% | 58.43% | 97.07% | 95.81% | 90.59% | 67.24% |
| [flatbuffers 23.5.26][flatbuffers] | 9.45% | † | 55.13% | 61.64% | 59.09% | 47.00% |
| [msgpacker 0.4.3][msgpacker] | 13.40% | 54.32% | 91.99% | 91.61% | 86.73% | 62.82% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.69% | 34.92% | 85.96% | 86.85% | 80.47% | 56.34% |
| [nanoserde 0.1.37][nanoserde] | 55.88% | 70.27% | 67.29% | 77.41% | 73.70% | 53.42% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 21.54% | 66.03% | 91.89% | 92.65% | 86.86% | 59.60% |
| [postcard 1.0.8][postcard] | 35.04% | 65.41% | 97.07% | 95.51% | 90.54% | 64.65% |
| [pot 3.0.0][pot] | 6.27% | 21.50% | 72.40% | 77.53% | 75.55% | 52.59% |
| [prost 0.12.4][prost] | <span title="encode">*15.27%\**</span> <span title="populate + encode">*5.26%\**</span> | 42.32% | 79.55% | 79.54% | 72.82% | 47.93% |
| [rkyv 0.7.44][rkyv] | 66.46% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.62%\**</span> | 69.57% | 75.24% | 68.88% | 50.03% |
| [rmp-serde 1.1.2][rmp-serde] | 11.90% | 41.33% | 89.64% | 88.76% | 82.58% | 58.61% |
| [ron 0.8.1][ron] | 1.03% | 8.75% | 43.78% | 64.30% | 65.70% | 42.51% |
| [savefile 0.16.5][savefile] | 71.71% | 67.85% | 67.29% | 77.40% | 73.70% | 53.16% |
| [serde_bare 0.5.0][serde_bare] | 21.61% | 68.76% | 91.89% | 92.66% | 86.82% | 62.96% |
| [serde_cbor 0.11.2][serde_cbor] | 7.78% | 28.70% | 49.99% | 71.59% | 70.89% | 50.08% |
| [serde_json 1.0.115][serde_json] | 3.79% | 25.02% | 38.51% | 61.38% | 63.63% | 43.76% |
| [simd-json 0.13.9][simd-json] | 7.00% | 31.01% | 38.51% | 61.38% | 63.63% | 43.28% |
| [speedy 0.8.7][speedy] | 72.93% | 82.66% | 79.45% | 79.74% | 80.19% | 57.09% |
| [wiring 0.2.1][wiring] | 71.60% | 73.30% | 33.65% | 42.79% | 83.16% | 54.79% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*25.91%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.73%\**</span> | <span title="validated on-demand with panic">*41.67%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*6.19%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.05%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.13%\**</span> <span title="validated upfront with error">*0.53%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.00%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 259.51 µs | <span title="unvalidated">*259.46 µs\**</span> | 6000024 | 5378513 | 5345891 | 7.6915 ms |
| [alkahest 0.1.5][alkahest] | 149.11 µs | † | 6000008 | 5378500 | 5345890 | 7.4508 ms |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*6.6333 ms\**</span> <span title="prepend">*8.4724 ms\**</span> | 10.176 ms | 8625005 | 6443961 | 6231572 | 75.208 ms |
| [bincode 2.0.0-rc][bincode] | 428.38 µs | 828.68 µs | 6000005 | 5378497 | 5345897 | 7.5808 ms |
| [bincode 1.3.3][bincode1] | 5.1214 ms | 5.0041 ms | 6000008 | 5378500 | 5345890 | 7.9182 ms |
| [bitcode 0.6.0][bitcode] | 1.4071 ms | 603.40 µs | 6000006 | 5182295 | 4923880 | 12.656 ms |
| [borsh 1.3.1][borsh] | 5.7328 ms | 4.2189 ms | 6000004 | 5378496 | 5345889 | 7.7574 ms |
| [bson 2.9.0][bson] | 47.113 ms | 78.793 ms | 23013911 | 9212089 | 7497811 | 108.02 ms |
| [capnp 0.18.13][capnp] | 5.4664 ms | † | 14000088 | 7130367 | 6051062 | 78.727 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.664 ms | 47.453 ms | 13125016 | 7524114 | 6757967 | 89.957 ms |
| [ciborium 0.2.2][ciborium] | 67.483 ms | 100.29 ms | 13122324 | 7524660 | 6759658 | 90.820 ms |
| [databuf 0.5.0][databuf] | 2.4037 ms | 5.3266 ms | 6000003 | 5378495 | 5345900 | 7.7063 ms |
| [dlhn 0.1.6][dlhn] | 6.5021 ms | 5.7998 ms | 6000003 | 5378495 | 5345900 | 7.6843 ms |
| [flatbuffers 23.5.26][flatbuffers] | 658.91 µs | † | 6000024 | 5378434 | 5345910 | 7.9528 ms |
| [msgpacker 0.4.3][msgpacker] | 19.103 ms | 8.7800 ms | 7500005 | 6058442 | 6014337 | 9.8224 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.70 ms | 26.564 ms | 8125037 | 6493484 | 6386940 | 68.278 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2874 ms | 905.50 µs | 6000008 | 5378500 | 5345890 | 7.6305 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.7395 ms | 4.5450 ms | 6000004 | 5378496 | 5345889 | 7.7538 ms |
| [postcard 1.0.8][postcard] | 518.33 µs | 1.2686 ms | 6000003 | 5378495 | 5345900 | 7.7188 ms |
| [pot 3.0.0][pot] | 40.349 ms | 72.468 ms | 10122342 | 6814618 | 6852251 | 80.278 ms |
| [prost 0.12.4][prost] | <span title="encode">*7.8975 ms\**</span> <span title="populate + encode">*9.9675 ms\**</span> | 13.009 ms | 8750000 | 6665735 | 6421871 | 70.630 ms |
| [rkyv 0.7.44][rkyv] | 206.01 µs | <span title="unvalidated">*198.08 µs\**</span> <span title="validated upfront with error">*148.66 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.6746 ms |
| [rmp-serde 1.1.2][rmp-serde] | 13.661 ms | 18.716 ms | 8125006 | 6494876 | 6391037 | 68.358 ms |
| [ron 0.8.1][ron] | 174.83 ms | 252.20 ms | 22192885 | 8970395 | 8138755 | 148.55 ms |
| [savefile 0.16.5][savefile] | 259.41 µs | 259.69 µs | 6000024 | 5378518 | 5345893 | 7.7235 ms |
| [serde_bare 0.5.0][serde_bare] | 6.4898 ms | 4.4998 ms | 6000003 | 5378495 | 5345900 | 7.5731 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.957 ms | 46.526 ms | 13122324 | 7524660 | 6759658 | 89.529 ms |
| [serde_json 1.0.115][serde_json] | 87.074 ms | 88.219 ms | 26192883 | 9566084 | 8586741 | 154.91 ms |
| [simd-json 0.13.9][simd-json] | 53.804 ms | 78.079 ms | 26192883 | 9566084 | 8586741 | 153.22 ms |
| [speedy 0.8.7][speedy] | 260.46 µs | 259.52 µs | 6000004 | 5378496 | 5345889 | 7.9508 ms |
| [wiring 0.2.1][wiring] | 148.86 µs | 352.93 µs | 12000016 | 10757697 | 10691743 | 15.110 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1651 ns\**</span> | <span title="unvalidated">*141.90 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8547 ns\**</span> | <span title="validated on-demand with panic">*77.369 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*104.82 ns\**</span> | <span title="validated on-demand with error">*2.1465 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4890 ns\**</span> <span title="validated upfront with error">*38.132 ns\**</span> | <span title="unvalidated">*54.131 µs\**</span> <span title="validated upfront with error">*77.410 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2365 ns\**</span> <span title="validated upfront with error">*11.146 ns\**</span> | <span title="unvalidated">*46.072 µs\**</span> <span title="validated upfront with error">*38.683 µs\**</span> | 107.68 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 57.36% | <span title="unvalidated">*57.30%\**</span> | 100.00% | 96.35% | 92.11% | 96.87% |
| [alkahest 0.1.5][alkahest] | 99.83% | † | 100.00% | 96.35% | 92.11% | 100.00% |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*2.24%\**</span> <span title="prepend">*1.76%\**</span> | 1.46% | 69.57% | 80.42% | 79.02% | 9.91% |
| [bincode 2.0.0-rc][bincode] | 34.75% | 17.94% | 100.00% | 96.35% | 92.11% | 98.29% |
| [bincode 1.3.3][bincode1] | 2.91% | 2.97% | 100.00% | 96.35% | 92.11% | 94.10% |
| [bitcode 0.6.0][bitcode] | 10.58% | 24.64% | 100.00% | 100.00% | 100.00% | 58.87% |
| [borsh 1.3.1][borsh] | 2.60% | 3.52% | 100.00% | 96.35% | 92.11% | 96.05% |
| [bson 2.9.0][bson] | 0.32% | 0.19% | 26.07% | 56.26% | 65.67% | 6.90% |
| [capnp 0.18.13][capnp] | 2.72% | † | 42.86% | 72.68% | 81.37% | 9.46% |
| [cbor4ii 0.3.2][cbor4ii] | 1.40% | 0.31% | 45.71% | 68.88% | 72.86% | 8.28% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.15% | 45.72% | 68.87% | 72.84% | 8.20% |
| [databuf 0.5.0][databuf] | 6.19% | 2.79% | 100.00% | 96.35% | 92.11% | 96.68% |
| [dlhn 0.1.6][dlhn] | 2.29% | 2.56% | 100.00% | 96.35% | 92.11% | 96.96% |
| [flatbuffers 23.5.26][flatbuffers] | 22.59% | † | 100.00% | 96.35% | 92.11% | 93.69% |
| [msgpacker 0.4.3][msgpacker] | 0.78% | 1.69% | 80.00% | 85.54% | 81.87% | 75.86% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.56% | 73.85% | 79.81% | 77.09% | 10.91% |
| [nanoserde 0.1.37][nanoserde] | 11.56% | 16.42% | 100.00% | 96.35% | 92.11% | 97.64% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.14% | 3.27% | 100.00% | 96.35% | 92.11% | 96.09% |
| [postcard 1.0.8][postcard] | 28.72% | 11.72% | 100.00% | 96.35% | 92.11% | 96.53% |
| [pot 3.0.0][pot] | 0.37% | 0.21% | 59.27% | 76.05% | 71.86% | 9.28% |
| [prost 0.12.4][prost] | <span title="encode">*1.88%\**</span> <span title="populate + encode">*1.49%\**</span> | 1.14% | 68.57% | 77.75% | 76.67% | 10.55% |
| [rkyv 0.7.44][rkyv] | 72.26% | <span title="unvalidated">*75.05%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 97.08% |
| [rmp-serde 1.1.2][rmp-serde] | 1.09% | 0.79% | 73.85% | 79.79% | 77.04% | 10.90% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 5.02% |
| [savefile 0.16.5][savefile] | 57.38% | 57.25% | 100.00% | 96.35% | 92.11% | 96.47% |
| [serde_bare 0.5.0][serde_bare] | 2.29% | 3.30% | 100.00% | 96.35% | 92.11% | 98.39% |
| [serde_cbor 0.11.2][serde_cbor] | 0.41% | 0.32% | 45.72% | 68.87% | 72.84% | 8.32% |
| [serde_json 1.0.115][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.81% |
| [simd-json 0.13.9][simd-json] | 0.28% | 0.19% | 22.91% | 54.17% | 57.34% | 4.86% |
| [speedy 0.8.7][speedy] | 57.15% | 57.28% | 100.00% | 96.35% | 92.11% | 93.71% |
| [wiring 0.2.1][wiring] | 100.00% | 42.12% | 50.00% | 48.17% | 46.05% | 49.31% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.11%\**</span> | <span title="unvalidated">*27.26%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*50.00%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.80%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.68%\**</span> <span title="validated upfront with error">*3.24%\**</span> | <span title="unvalidated">*71.46%\**</span> <span title="validated upfront with error">*49.97%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*11.09%\**</span> | <span title="unvalidated">*83.96%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 190.67 µs | <span title="unvalidated">*1.3229 ms\**</span> | 1290592 | 396594 | 340493 | 4.9360 ms |
| [alkahest 0.1.5][alkahest] | 218.57 µs | † | 667570 | 325484 | 320452 | 3.9778 ms |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*940.59 µs\**</span> <span title="prepend">*930.28 µs\**</span> | 3.2623 ms | 489348 | 281173 | 249546 | 3.1489 ms |
| [bincode 2.0.0-rc][bincode] | 266.93 µs | 2.0995 ms | 367413 | 221291 | 206273 | 2.5259 ms |
| [bincode 1.3.3][bincode1] | 571.55 µs | 1.8125 ms | 569975 | 240525 | 232423 | 2.8760 ms |
| [bitcode 0.6.0][bitcode] | 130.50 µs | 1.2737 ms | 327688 | 200947 | 182736 | 741.10 µs |
| [borsh 1.3.1][borsh] | 561.25 µs | 1.8360 ms | 446595 | 234236 | 210008 | 2.4951 ms |
| [bson 2.9.0][bson] | 2.8590 ms | 8.2882 ms | 1619653 | 502185 | 328399 | 4.8892 ms |
| [capnp 0.18.13][capnp] | 456.72 µs | † | 803896 | 335606 | 280851 | 3.9690 ms |
| [cbor4ii 0.3.2][cbor4ii] | 791.05 µs | 4.8386 ms | 1109831 | 344745 | 274514 | 3.8242 ms |
| [ciborium 0.2.2][ciborium] | 3.7774 ms | 9.6206 ms | 1109821 | 344751 | 274526 | 3.8217 ms |
| [databuf 0.5.0][databuf] | 320.26 µs | 1.7438 ms | 356311 | 213062 | 198488 | 2.4160 ms |
| [dlhn 0.1.6][dlhn] | 795.13 µs | 2.5449 ms | 366496 | 220600 | 205683 | 2.4564 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.3331 ms | † | 844168 | 345696 | 294015 | 3.8596 ms |
| [msgpacker 0.4.3][msgpacker] | 862.98 µs | 2.8289 ms | 391251 | 236877 | 220476 | 2.6418 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1737 ms | 3.9612 ms | 449745 | 252432 | 231110 | 2.7832 ms |
| [nanoserde 0.1.37][nanoserde] | 263.94 µs | 1.9043 ms | 567975 | 239930 | 232419 | 2.9145 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 666.34 µs | 2.0062 ms | 356311 | 212976 | 198524 | 2.4064 ms |
| [postcard 1.0.8][postcard] | 434.37 µs | 1.9669 ms | 367489 | 221913 | 207344 | 2.5067 ms |
| [pot 3.0.0][pot] | 2.2570 ms | 6.0393 ms | 599125 | 299158 | 247693 | 3.1794 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.1237 ms\**</span> <span title="populate + encode">*2.8090 ms\**</span> | 3.5524 ms | 596811 | 305319 | 269310 | 3.4464 ms |
| [rkyv 0.7.44][rkyv] | 297.35 µs | <span title="unvalidated">*1.2746 ms\**</span> <span title="validated upfront with error">*1.7872 ms\**</span> | 596952 | 253967 | 220706 | 2.7136 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3452 ms | 3.0101 ms | 424533 | 245214 | 226188 | 2.7138 ms |
| [ron 0.8.1][ron] | 8.4003 ms | 16.941 ms | 1465223 | 434935 | 343338 | 5.8564 ms |
| [savefile 0.16.5][savefile] | 218.09 µs | 1.8430 ms | 566991 | 239361 | 232010 | 2.8986 ms |
| [serde_bare 0.5.0][serde_bare] | 737.47 µs | 2.2609 ms | 356311 | 213062 | 198488 | 2.4042 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7565 ms | 4.7824 ms | 1109821 | 344751 | 274526 | 3.8323 ms |
| [serde_json 1.0.115][serde_json] | 3.7094 ms | 6.8987 ms | 1623191 | 466527 | 359623 | 6.0570 ms |
| [simd-json 0.13.9][simd-json] | 2.2289 ms | 4.5318 ms | 1623191 | 466527 | 359623 | 6.1258 ms |
| [speedy 0.8.7][speedy] | 274.36 µs | 1.6521 ms | 449595 | 234970 | 210361 | 2.4973 ms |
| [wiring 0.2.1][wiring] | 203.53 µs | 1.8533 ms | 1133950 | 495111 | 225317 | 3.0221 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.400 µs\**</span> | <span title="unvalidated">*38.230 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8547 ns\**</span> | <span title="validated on-demand with panic">*4.7747 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*72.965 ns\**</span> | <span title="validated on-demand with error">*436.86 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4733 ns\**</span> <span title="validated upfront with error">*2.1915 ms\**</span> | <span title="unvalidated">*1.3774 µs\**</span> <span title="validated upfront with error">*2.1915 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2366 ns\**</span> <span title="validated upfront with error">*511.46 µs\**</span> | <span title="unvalidated">*163.22 ns\**</span> <span title="validated upfront with error">*515.68 µs\**</span> | 923.84 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 68.44% | <span title="unvalidated">*96.28%\**</span> | 25.39% | 50.67% | 53.67% | 15.01% |
| [alkahest 0.1.5][alkahest] | 59.71% | † | 49.09% | 61.74% | 57.02% | 18.63% |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*13.87%\**</span> <span title="prepend">*14.03%\**</span> | 39.04% | 66.96% | 71.47% | 73.23% | 23.54% |
| [bincode 2.0.0-rc][bincode] | 48.89% | 60.67% | 89.19% | 90.81% | 88.59% | 29.34% |
| [bincode 1.3.3][bincode1] | 22.83% | 70.27% | 57.49% | 83.55% | 78.62% | 25.77% |
| [bitcode 0.6.0][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 23.25% | 69.37% | 73.37% | 85.79% | 87.01% | 29.70% |
| [bson 2.9.0][bson] | 4.56% | 15.37% | 20.23% | 40.01% | 55.64% | 15.16% |
| [capnp 0.18.13][capnp] | 28.57% | † | 40.76% | 59.88% | 65.07% | 18.67% |
| [cbor4ii 0.3.2][cbor4ii] | 16.50% | 26.32% | 29.53% | 58.29% | 66.57% | 19.38% |
| [ciborium 0.2.2][ciborium] | 3.45% | 13.24% | 29.53% | 58.29% | 66.56% | 19.39% |
| [databuf 0.5.0][databuf] | 40.75% | 73.04% | 91.97% | 94.31% | 92.06% | 30.67% |
| [dlhn 0.1.6][dlhn] | 16.41% | 50.05% | 89.41% | 91.09% | 88.84% | 30.17% |
| [flatbuffers 23.5.26][flatbuffers] | 3.92% | † | 38.82% | 58.13% | 62.15% | 19.20% |
| [msgpacker 0.4.3][msgpacker] | 15.12% | 45.02% | 83.75% | 84.83% | 82.88% | 28.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.52% | 32.15% | 72.86% | 79.60% | 79.07% | 26.63% |
| [nanoserde 0.1.37][nanoserde] | 49.44% | 66.89% | 57.69% | 83.75% | 78.62% | 25.43% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 19.58% | 63.49% | 91.97% | 94.35% | 92.05% | 30.80% |
| [postcard 1.0.8][postcard] | 30.04% | 64.76% | 89.17% | 90.55% | 88.13% | 29.56% |
| [pot 3.0.0][pot] | 5.78% | 21.09% | 54.69% | 67.17% | 73.78% | 23.31% |
| [prost 0.12.4][prost] | <span title="encode">*11.61%\**</span> <span title="populate + encode">*4.65%\**</span> | 35.85% | 54.91% | 65.82% | 67.85% | 21.50% |
| [rkyv 0.7.44][rkyv] | 43.89% | <span title="unvalidated">*99.93%\**</span> <span title="validated upfront with error">*71.27%\**</span> | 54.89% | 79.12% | 82.80% | 27.31% |
| [rmp-serde 1.1.2][rmp-serde] | 9.70% | 42.31% | 77.19% | 81.95% | 80.79% | 27.31% |
| [ron 0.8.1][ron] | 1.55% | 7.52% | 22.36% | 46.20% | 53.22% | 12.65% |
| [savefile 0.16.5][savefile] | 59.84% | 69.11% | 57.79% | 83.95% | 78.76% | 25.57% |
| [serde_bare 0.5.0][serde_bare] | 17.70% | 56.34% | 91.97% | 94.31% | 92.06% | 30.83% |
| [serde_cbor 0.11.2][serde_cbor] | 7.43% | 26.63% | 29.53% | 58.29% | 66.56% | 19.34% |
| [serde_json 1.0.115][serde_json] | 3.52% | 18.46% | 20.19% | 43.07% | 50.81% | 12.24% |
| [simd-json 0.13.9][simd-json] | 5.85% | 28.11% | 20.19% | 43.07% | 50.81% | 12.10% |
| [speedy 0.8.7][speedy] | 47.57% | 77.10% | 72.89% | 85.52% | 86.87% | 29.68% |
| [wiring 0.2.1][wiring] | 64.12% | 68.73% | 28.90% | 40.59% | 81.10% | 24.52% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.43%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*3.42%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*37.36%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.85%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 482.32 µs | <span title="unvalidated">*2.3284 ms\**</span> | 2984682 | 1406961 | 1270192 | 14.301 ms |
| [alkahest 0.1.5][alkahest] | 736.51 µs | † | 1863391 | 1234113 | 1202345 | 11.312 ms |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*5.0006 ms\**</span> <span title="prepend">*2.7598 ms\**</span> | 8.4334 ms | 1664428 | 1264167 | 1216472 | 10.857 ms |
| [bincode 2.0.0-rc][bincode] | 699.44 µs | 3.6804 ms | 1372381 | 1091486 | 1037296 | 9.1567 ms |
| [bincode 1.3.3][bincode1] | 3.7319 ms | 3.9542 ms | 1811011 | 1115281 | 1025627 | 9.7603 ms |
| [bitcode 0.6.0][bitcode] | 717.16 µs | 2.3078 ms | 948499 | 857321 | 837658 | 3.0227 ms |
| [borsh 1.3.1][borsh] | 2.8571 ms | 2.8815 ms | 1486162 | 1082357 | 1013550 | 9.5002 ms |
| [bson 2.9.0][bson] | 22.148 ms | 44.495 ms | 10030880 | 2833079 | 1600859 | 27.052 ms |
| [capnp 0.18.13][capnp] | 2.1592 ms | † | 2664040 | 1511895 | 1212087 | 14.049 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.2589 ms | 17.767 ms | 5878791 | 1655835 | 1431390 | 20.765 ms |
| [ciborium 0.2.2][ciborium] | 22.514 ms | 46.526 ms | 5878653 | 1655791 | 1431560 | 20.939 ms |
| [databuf 0.5.0][databuf] | 1.7986 ms | 3.5878 ms | 1288257 | 1037579 | 984337 | 8.4365 ms |
| [dlhn 0.1.6][dlhn] | 5.2001 ms | 6.3241 ms | 1279599 | 1052061 | 1021161 | 8.2566 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.1948 ms | † | 2273740 | 1408408 | 1235566 | 12.628 ms |
| [msgpacker 0.4.3][msgpacker] | 1.8722 ms | 4.5823 ms | 1424043 | 1128758 | 1110156 | 9.1625 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.976 ms | 15.791 ms | 1728519 | 1247642 | 1233323 | 11.664 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3339 ms | 2.8741 ms | 1770477 | 1108304 | 1029947 | 10.192 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.0986 ms | 3.0492 ms | 1288257 | 1039269 | 986510 | 8.4126 ms |
| [postcard 1.0.8][postcard] | 1.7818 ms | 3.9616 ms | 1279599 | 1058243 | 1016738 | 8.2033 ms |
| [pot 3.0.0][pot] | 13.575 ms | 31.287 ms | 2544810 | 1447453 | 1268390 | 15.104 ms |
| [prost 0.12.4][prost] | <span title="encode">*5.1630 ms\**</span> <span title="populate + encode">*9.1979 ms\**</span> | 9.5114 ms | 1818378 | 1307777 | 1266311 | 11.535 ms |
| [rkyv 0.7.44][rkyv] | 1.2991 ms | <span title="unvalidated">*2.1569 ms\**</span> <span title="validated upfront with error">*2.7572 ms\**</span> | 2029080 | 1335117 | 1158855 | 11.794 ms |
| [rmp-serde 1.1.2][rmp-serde] | 8.7881 ms | 12.279 ms | 1703813 | 1231892 | 1200208 | 10.871 ms |
| [ron 0.8.1][ron] | 37.611 ms | 95.670 ms | 8476284 | 2181196 | 1783971 | 33.582 ms |
| [savefile 0.16.5][savefile] | 1.0063 ms | 2.6395 ms | 1750226 | 1101682 | 1027827 | 9.7453 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7971 ms | 4.4928 ms | 1288257 | 1037597 | 984356 | 8.3936 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.1090 ms | 21.438 ms | 5878653 | 1655791 | 1431560 | 20.678 ms |
| [serde_json 1.0.115][serde_json] | 20.088 ms | 31.041 ms | 9175594 | 2334253 | 1800713 | 33.370 ms |
| [simd-json 0.13.9][simd-json] | 11.249 ms | 26.588 ms | 9175594 | 2334253 | 1800713 | 33.795 ms |
| [speedy 0.8.7][speedy] | 740.51 µs | 2.5192 ms | 1546963 | 1093532 | 1013443 | 9.7125 ms |
| [wiring 0.2.1][wiring] | 836.99 µs | 2.7274 ms | 3500420 | 2259435 | 1061377 | 10.579 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.033 µs\**</span> | <span title="unvalidated">*67.127 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8558 ns\**</span> | <span title="validated on-demand with panic">*626.66 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*73.108 ns\**</span> | <span title="validated on-demand with error">*710.35 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4736 ns\**</span> <span title="validated upfront with error">*4.7525 ms\**</span> | <span title="unvalidated">*2.6516 µs\**</span> <span title="validated upfront with error">*4.7557 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2368 ns\**</span> <span title="validated upfront with error">*603.27 µs\**</span> | <span title="unvalidated">*343.70 ns\**</span> <span title="validated upfront with error">*602.82 µs\**</span> | 502.52 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.63%\**</span> | 31.78% | 60.93% | 65.95% | 21.14% |
| [alkahest 0.1.5][alkahest] | 65.49% | † | 50.90% | 69.47% | 69.67% | 26.72% |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*9.65%\**</span> <span title="prepend">*17.48%\**</span> | 25.58% | 56.99% | 67.82% | 68.86% | 27.84% |
| [bincode 2.0.0-rc][bincode] | 68.96% | 58.61% | 69.11% | 78.55% | 80.75% | 33.01% |
| [bincode 1.3.3][bincode1] | 12.92% | 54.55% | 52.37% | 76.87% | 81.67% | 30.97% |
| [bitcode 0.6.0][bitcode] | 67.25% | 93.46% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 16.88% | 74.85% | 63.82% | 79.21% | 82.65% | 31.82% |
| [bson 2.9.0][bson] | 2.18% | 4.85% | 9.46% | 30.26% | 52.33% | 11.17% |
| [capnp 0.18.13][capnp] | 22.34% | † | 35.60% | 56.71% | 69.11% | 21.52% |
| [cbor4ii 0.3.2][cbor4ii] | 11.32% | 12.14% | 16.13% | 51.78% | 58.52% | 14.56% |
| [ciborium 0.2.2][ciborium] | 2.14% | 4.64% | 16.13% | 51.78% | 58.51% | 14.44% |
| [databuf 0.5.0][databuf] | 26.82% | 60.12% | 73.63% | 82.63% | 85.10% | 35.83% |
| [dlhn 0.1.6][dlhn] | 9.28% | 34.11% | 74.12% | 81.49% | 82.03% | 36.61% |
| [flatbuffers 23.5.26][flatbuffers] | 9.28% | † | 41.72% | 60.87% | 67.80% | 23.94% |
| [msgpacker 0.4.3][msgpacker] | 25.76% | 47.07% | 66.61% | 75.95% | 75.45% | 32.99% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.61% | 13.66% | 54.87% | 68.72% | 67.92% | 25.91% |
| [nanoserde 0.1.37][nanoserde] | 36.16% | 75.05% | 53.57% | 77.35% | 81.33% | 29.66% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 15.57% | 70.74% | 73.63% | 82.49% | 84.91% | 35.93% |
| [postcard 1.0.8][postcard] | 27.07% | 54.45% | 74.12% | 81.01% | 82.39% | 36.85% |
| [pot 3.0.0][pot] | 3.55% | 6.89% | 37.27% | 59.23% | 66.04% | 20.01% |
| [prost 0.12.4][prost] | <span title="encode">*9.34%\**</span> <span title="populate + encode">*5.24%\**</span> | 22.68% | 52.16% | 65.56% | 66.15% | 26.21% |
| [rkyv 0.7.44][rkyv] | 37.13% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.23%\**</span> | 46.75% | 64.21% | 72.28% | 25.63% |
| [rmp-serde 1.1.2][rmp-serde] | 5.49% | 17.57% | 55.67% | 69.59% | 69.79% | 27.81% |
| [ron 0.8.1][ron] | 1.28% | 2.25% | 11.19% | 39.31% | 46.95% | 9.00% |
| [savefile 0.16.5][savefile] | 47.93% | 81.72% | 54.19% | 77.82% | 81.50% | 31.02% |
| [serde_bare 0.5.0][serde_bare] | 10.05% | 48.01% | 73.63% | 82.63% | 85.10% | 36.01% |
| [serde_cbor 0.11.2][serde_cbor] | 5.29% | 10.06% | 16.13% | 51.78% | 58.51% | 14.62% |
| [serde_json 1.0.115][serde_json] | 2.40% | 6.95% | 10.34% | 36.73% | 46.52% | 9.06% |
| [simd-json 0.13.9][simd-json] | 4.29% | 8.11% | 10.34% | 36.73% | 46.52% | 8.94% |
| [speedy 0.8.7][speedy] | 65.13% | 85.62% | 61.31% | 78.40% | 82.65% | 31.12% |
| [wiring 0.2.1][wiring] | 57.63% | 79.08% | 27.10% | 37.94% | 78.92% | 28.57% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.51%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.65%\**</span> | <span title="validated on-demand with panic">*54.85%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.69%\**</span> | <span title="validated on-demand with error">*48.38%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.96%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bilrost]: https://crates.io/crates/bilrost/0.1006.0
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.0
[borsh]: https://crates.io/crates/borsh/1.3.1
[bson]: https://crates.io/crates/bson/2.9.0
[capnp]: https://crates.io/crates/capnp/0.18.13
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.2
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.9
[postcard]: https://crates.io/crates/postcard/1.0.8
[pot]: https://crates.io/crates/pot/3.0.0
[prost]: https://crates.io/crates/prost/0.12.4
[rkyv]: https://crates.io/crates/rkyv/0.7.44
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.16.5
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.115
[simd-json]: https://crates.io/crates/simd-json/0.13.9
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.2.1


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
