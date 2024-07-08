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

## Last updated: 2024-7-8 3:16:38

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.81.0-nightly (20ae37c18 2024-07-07)
binary: rustc
commit-hash: 20ae37c18df95f9246c019b04957d23b4164bf7a
commit-date: 2024-07-07
host: x86_64-unknown-linux-gnu
release: 1.81.0-nightly
LLVM version: 18.1.7
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
BogoMIPS:                           4890.86
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
| [abomonation 0.7.3][abomonation] | 434.14 µs | <span title="unvalidated">*1.4366 ms\**</span> | 1705800 | 520079 | 413396 | 6.8097 ms |
| [alkahest 0.1.5][alkahest] | 202.86 µs | † | 1045784 | 454157 | 389424 | 6.0510 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*716.46 µs\**</span> <span title="prepend">*625.95 µs\**</span> | 3.2681 ms | 874632 | 355446 | 311723 | 5.0782 ms |
| [bincode 2.0.0-rc][bincode] | 287.13 µs | 2.6124 ms | 741295 | 303944 | 257153 | 3.9479 ms |
| [bincode 1.3.3][bincode1] | 519.88 µs | 2.3828 ms | 1045784 | 373127 | 311761 | 4.8703 ms |
| [bitcode 0.6.0][bitcode] | 144.08 µs | 1.5421 ms | 703710 | 288826 | 229755 | 2.4237 ms |
| [borsh 1.5.1][borsh] | 542.96 µs | 2.2365 ms | 885780 | 362204 | 286514 | 4.1982 ms |
| [bson 2.9.0][bson] | 2.0519 ms | 7.8208 ms | 1924682 | 532821 | 376270 | 5.6268 ms |
| [capnp 0.19.6][capnp] | 526.93 µs | † | 1443216 | 513986 | 428649 | 6.3817 ms |
| [cbor4ii 0.3.2][cbor4ii] | 595.47 µs | 4.9230 ms | 1407835 | 403440 | 324081 | 4.8287 ms |
| [ciborium 0.2.2][ciborium] | 3.2282 ms | 11.988 ms | 1407835 | 403440 | 324081 | 4.8582 ms |
| [databuf 0.5.0][databuf] | 261.01 µs | 2.1137 ms | 765778 | 311715 | 264630 | 3.8404 ms |
| [dlhn 0.1.7][dlhn] | 803.81 µs | 2.5559 ms | 724953 | 301446 | 253629 | 3.5427 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0168 ms | † | 1276368 | 468539 | 388832 | 5.1337 ms |
| [msgpacker 0.4.3][msgpacker] | 1.2438 ms | 2.6380 ms | 764996 | 315291 | 264898 | 3.9196 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.6028 ms | 4.4148 ms | 818669 | 332556 | 285514 | 4.6435 ms |
| [nanoserde 0.1.37][nanoserde] | 241.59 µs | 2.1442 ms | 1045784 | 373127 | 311761 | 4.8473 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 649.57 µs | 2.2948 ms | 765778 | 311743 | 264518 | 3.8366 ms |
| [postcard 1.0.8][postcard] | 415.86 µs | 2.2995 ms | 724953 | 302399 | 253747 | 3.6047 ms |
| [pot 3.0.0][pot] | 2.1502 ms | 6.5262 ms | 971922 | 372513 | 304122 | 4.9410 ms |
| [prost 0.12.6][prost] | <span title="encode">*964.10 µs\**</span> <span title="populate + encode">*2.4942 ms\**</span> | 3.1931 ms | 884628 | 363130 | 315494 | 4.8164 ms |
| [rkyv 0.7.44][rkyv] | 222.01 µs | <span title="unvalidated">*1.4525 ms\**</span> <span title="validated upfront with error">*1.9868 ms\**</span> | 1011488 | 383862 | 333545 | 4.9174 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3835 ms | 3.2098 ms | 784997 | 325384 | 278219 | 4.1658 ms |
| [ron 0.8.1][ron] | 12.716 ms | 15.757 ms | 1607459 | 449158 | 349713 | 6.1199 ms |
| [savefile 0.17.6][savefile] | 188.40 µs | 2.2075 ms | 1045800 | 373140 | 311777 | 4.8180 ms |
| [serde_bare 0.5.0][serde_bare] | 672.29 µs | 2.2422 ms | 765778 | 311715 | 264630 | 4.1144 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0338 ms | 4.8280 ms | 1407835 | 403440 | 324081 | 5.0587 ms |
| [serde_json 1.0.120][serde_json] | 4.0039 ms | 5.7836 ms | 1827461 | 470560 | 361090 | 6.0894 ms |
| [simd-json 0.13.10][simd-json] | 2.0645 ms | 4.6941 ms | 1827461 | 470560 | 361090 | 5.6429 ms |
| [speedy 0.8.7][speedy] | 201.09 µs | 1.7693 ms | 885780 | 362204 | 286514 | 4.2094 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.293 µs\**</span> | <span title="unvalidated">*38.143 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8555 ns\**</span> | <span title="validated on-demand with panic">*24.904 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*73.579 ns\**</span> | <span title="validated on-demand with error">*166.99 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4729 ns\**</span> <span title="validated upfront with error">*2.0396 ms\**</span> | <span title="unvalidated">*50.668 µs\**</span> <span title="validated upfront with error">*2.0518 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2376 ns\**</span> <span title="validated upfront with error">*524.37 µs\**</span> | <span title="unvalidated">*10.847 µs\**</span> <span title="validated upfront with error">*534.73 µs\**</span> | 9.8159 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 33.19% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 55.54% | 55.58% | 35.59% |
| [alkahest 0.1.5][alkahest] | 71.02% | † | 67.29% | 63.60% | 59.00% | 40.05% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*20.11%\**</span> <span title="prepend">*23.02%\**</span> | 43.96% | 80.46% | 81.26% | 73.70% | 47.73% |
| [bincode 2.0.0-rc][bincode] | 50.18% | 54.99% | 94.93% | 95.03% | 89.35% | 61.39% |
| [bincode 1.3.3][bincode1] | 27.71% | 60.29% | 67.29% | 77.41% | 73.70% | 49.76% |
| [bitcode 0.6.0][bitcode] | 100.00% | 93.16% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 26.54% | 64.23% | 79.45% | 79.74% | 80.19% | 57.73% |
| [bson 2.9.0][bson] | 7.02% | 18.37% | 36.56% | 54.21% | 61.06% | 43.07% |
| [capnp 0.19.6][capnp] | 27.34% | † | 48.76% | 56.19% | 53.60% | 37.98% |
| [cbor4ii 0.3.2][cbor4ii] | 24.20% | 29.18% | 49.99% | 71.59% | 70.89% | 50.19% |
| [ciborium 0.2.2][ciborium] | 4.46% | 11.98% | 49.99% | 71.59% | 70.89% | 49.89% |
| [databuf 0.5.0][databuf] | 55.20% | 67.97% | 91.89% | 92.66% | 86.82% | 63.11% |
| [dlhn 0.1.7][dlhn] | 17.92% | 56.21% | 97.07% | 95.81% | 90.59% | 68.41% |
| [flatbuffers 24.3.25][flatbuffers] | 14.17% | † | 55.13% | 61.64% | 59.09% | 47.21% |
| [msgpacker 0.4.3][msgpacker] | 11.58% | 54.46% | 91.99% | 91.61% | 86.73% | 61.84% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.57% | 32.54% | 85.96% | 86.85% | 80.47% | 52.20% |
| [nanoserde 0.1.37][nanoserde] | 59.64% | 67.00% | 67.29% | 77.41% | 73.70% | 50.00% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.18% | 62.60% | 91.89% | 92.65% | 86.86% | 63.17% |
| [postcard 1.0.8][postcard] | 34.65% | 62.47% | 97.07% | 95.51% | 90.54% | 67.24% |
| [pot 3.0.0][pot] | 6.70% | 22.01% | 72.40% | 77.53% | 75.55% | 49.05% |
| [prost 0.12.6][prost] | <span title="encode">*14.94%\**</span> <span title="populate + encode">*5.78%\**</span> | 44.99% | 79.55% | 79.54% | 72.82% | 50.32% |
| [rkyv 0.7.44][rkyv] | 64.90% | <span title="unvalidated">*98.91%\**</span> <span title="validated upfront with error">*72.31%\**</span> | 69.57% | 75.24% | 68.88% | 49.29% |
| [rmp-serde 1.3.0][rmp-serde] | 10.41% | 44.76% | 89.64% | 88.76% | 82.58% | 58.18% |
| [ron 0.8.1][ron] | 1.13% | 9.12% | 43.78% | 64.30% | 65.70% | 39.60% |
| [savefile 0.17.6][savefile] | 76.48% | 65.08% | 67.29% | 77.40% | 73.69% | 50.31% |
| [serde_bare 0.5.0][serde_bare] | 21.43% | 64.07% | 91.89% | 92.66% | 86.82% | 58.91% |
| [serde_cbor 0.11.2][serde_cbor] | 7.08% | 29.76% | 49.99% | 71.59% | 70.89% | 47.91% |
| [serde_json 1.0.120][serde_json] | 3.60% | 24.84% | 38.51% | 61.38% | 63.63% | 39.80% |
| [simd-json 0.13.10][simd-json] | 6.98% | 30.60% | 38.51% | 61.38% | 63.63% | 42.95% |
| [speedy 0.8.7][speedy] | 71.65% | 81.20% | 79.45% | 79.74% | 80.19% | 57.58% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*28.44%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.70%\**</span> | <span title="validated on-demand with panic">*43.56%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*6.50%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.05%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.41%\**</span> <span title="validated upfront with error">*0.53%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.03%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 239.11 µs | <span title="unvalidated">*238.74 µs\**</span> | 6000024 | 5378513 | 5345890 | 7.6023 ms |
| [alkahest 0.1.5][alkahest] | 200.68 µs | † | 6000008 | 5378500 | 5345890 | 7.8514 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6183 ms\**</span> <span title="prepend">*8.7040 ms\**</span> | 10.100 ms | 8625005 | 6443961 | 6231572 | 74.066 ms |
| [bincode 2.0.0-rc][bincode] | 503.31 µs | 822.27 µs | 6000005 | 5378497 | 5345897 | 7.7747 ms |
| [bincode 1.3.3][bincode1] | 5.6653 ms | 4.3926 ms | 6000008 | 5378500 | 5345890 | 7.7793 ms |
| [bitcode 0.6.0][bitcode] | 1.4249 ms | 619.27 µs | 6000006 | 5182295 | 4923880 | 12.948 ms |
| [borsh 1.5.1][borsh] | 6.3580 ms | 4.6019 ms | 6000004 | 5378496 | 5345889 | 8.1388 ms |
| [bson 2.9.0][bson] | 46.097 ms | 90.327 ms | 23013911 | 9212089 | 7497811 | 108.73 ms |
| [capnp 0.19.6][capnp] | 5.5832 ms | † | 14000088 | 7130367 | 6051062 | 79.834 ms |
| [cbor4ii 0.3.2][cbor4ii] | 9.2010 ms | 48.145 ms | 13125016 | 7524114 | 6757967 | 90.332 ms |
| [ciborium 0.2.2][ciborium] | 68.843 ms | 120.76 ms | 13122324 | 7524660 | 6759658 | 89.597 ms |
| [databuf 0.5.0][databuf] | 2.3963 ms | 5.2529 ms | 6000003 | 5378495 | 5345900 | 8.2325 ms |
| [dlhn 0.1.7][dlhn] | 7.6165 ms | 6.7680 ms | 6000003 | 5378495 | 5345900 | 8.0464 ms |
| [flatbuffers 24.3.25][flatbuffers] | 881.27 µs | † | 6000024 | 5378434 | 5345910 | 7.5243 ms |
| [msgpacker 0.4.3][msgpacker] | 19.356 ms | 5.2501 ms | 7500005 | 6058442 | 6014337 | 9.6496 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.29 ms | 32.317 ms | 8125037 | 6493484 | 6386940 | 70.192 ms |
| [nanoserde 0.1.37][nanoserde] | 1.9001 ms | 1.0938 ms | 6000008 | 5378500 | 5345890 | 7.5260 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.0924 ms | 4.0605 ms | 6000004 | 5378496 | 5345889 | 7.7805 ms |
| [postcard 1.0.8][postcard] | 477.82 µs | 1.8014 ms | 6000003 | 5378495 | 5345900 | 7.9155 ms |
| [pot 3.0.0][pot] | 37.108 ms | 72.991 ms | 10122342 | 6814618 | 6852251 | 79.478 ms |
| [prost 0.12.6][prost] | <span title="encode">*7.6854 ms\**</span> <span title="populate + encode">*8.7502 ms\**</span> | 13.691 ms | 8750000 | 6665735 | 6421871 | 70.537 ms |
| [rkyv 0.7.44][rkyv] | 186.82 µs | <span title="unvalidated">*148.82 µs\**</span> <span title="validated upfront with error">*165.50 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.9358 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.359 ms | 17.985 ms | 8125006 | 6494876 | 6391037 | 68.425 ms |
| [ron 0.8.1][ron] | 171.43 ms | 245.25 ms | 22192885 | 8970395 | 8138755 | 147.36 ms |
| [savefile 0.17.6][savefile] | 238.73 µs | 239.06 µs | 6000024 | 5378513 | 5345893 | 7.8431 ms |
| [serde_bare 0.5.0][serde_bare] | 6.5230 ms | 4.1797 ms | 6000003 | 5378495 | 5345900 | 7.6281 ms |
| [serde_cbor 0.11.2][serde_cbor] | 36.644 ms | 48.459 ms | 13122324 | 7524660 | 6759658 | 89.129 ms |
| [serde_json 1.0.120][serde_json] | 88.188 ms | 83.345 ms | 26192883 | 9566084 | 8586741 | 152.73 ms |
| [simd-json 0.13.10][simd-json] | 53.102 ms | 72.679 ms | 26192883 | 9566084 | 8586741 | 153.34 ms |
| [speedy 0.8.7][speedy] | 238.18 µs | 238.59 µs | 6000004 | 5378496 | 5345889 | 7.8512 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1649 ns\**</span> | <span title="unvalidated">*141.46 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8552 ns\**</span> | <span title="validated on-demand with panic">*77.328 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*104.34 ns\**</span> | <span title="validated on-demand with error">*2.1397 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4736 ns\**</span> <span title="validated upfront with error">*40.029 ns\**</span> | <span title="unvalidated">*77.390 µs\**</span> <span title="validated upfront with error">*77.427 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2368 ns\**</span> <span title="validated upfront with error">*8.9869 ns\**</span> | <span title="unvalidated">*48.445 µs\**</span> <span title="validated upfront with error">*77.440 µs\**</span> | 97.271 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 78.13% | <span title="unvalidated">*62.34%\**</span> | 100.00% | 96.35% | 92.11% | 98.97% |
| [alkahest 0.1.5][alkahest] | 93.09% | † | 100.00% | 96.35% | 92.11% | 95.83% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.82%\**</span> <span title="prepend">*2.15%\**</span> | 1.47% | 69.57% | 80.42% | 79.02% | 10.16% |
| [bincode 2.0.0-rc][bincode] | 37.12% | 18.10% | 100.00% | 96.35% | 92.11% | 96.78% |
| [bincode 1.3.3][bincode1] | 3.30% | 3.39% | 100.00% | 96.35% | 92.11% | 96.72% |
| [bitcode 0.6.0][bitcode] | 13.11% | 24.03% | 100.00% | 100.00% | 100.00% | 58.11% |
| [borsh 1.5.1][borsh] | 2.94% | 3.23% | 100.00% | 96.35% | 92.11% | 92.45% |
| [bson 2.9.0][bson] | 0.41% | 0.16% | 26.07% | 56.26% | 65.67% | 6.92% |
| [capnp 0.19.6][capnp] | 3.35% | † | 42.86% | 72.68% | 81.37% | 9.42% |
| [cbor4ii 0.3.2][cbor4ii] | 2.03% | 0.31% | 45.71% | 68.88% | 72.86% | 8.33% |
| [ciborium 0.2.2][ciborium] | 0.27% | 0.12% | 45.72% | 68.87% | 72.84% | 8.40% |
| [databuf 0.5.0][databuf] | 7.80% | 2.83% | 100.00% | 96.35% | 92.11% | 91.40% |
| [dlhn 0.1.7][dlhn] | 2.45% | 2.20% | 100.00% | 96.35% | 92.11% | 93.51% |
| [flatbuffers 24.3.25][flatbuffers] | 21.20% | † | 100.00% | 96.35% | 92.11% | 100.00% |
| [msgpacker 0.4.3][msgpacker] | 0.97% | 2.83% | 80.00% | 85.54% | 81.87% | 77.98% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.15% | 0.46% | 73.85% | 79.81% | 77.09% | 10.72% |
| [nanoserde 0.1.37][nanoserde] | 9.83% | 13.61% | 100.00% | 96.35% | 92.11% | 99.98% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.67% | 3.67% | 100.00% | 96.35% | 92.11% | 96.71% |
| [postcard 1.0.8][postcard] | 39.10% | 8.26% | 100.00% | 96.35% | 92.11% | 95.06% |
| [pot 3.0.0][pot] | 0.50% | 0.20% | 59.27% | 76.05% | 71.86% | 9.47% |
| [prost 0.12.6][prost] | <span title="encode">*2.43%\**</span> <span title="populate + encode">*2.14%\**</span> | 1.09% | 68.57% | 77.75% | 76.67% | 10.67% |
| [rkyv 0.7.44][rkyv] | 100.00% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*89.92%\**</span> | 100.00% | 96.35% | 92.11% | 94.81% |
| [rmp-serde 1.3.0][rmp-serde] | 1.22% | 0.83% | 73.85% | 79.79% | 77.04% | 11.00% |
| [ron 0.8.1][ron] | 0.11% | 0.06% | 27.04% | 57.77% | 60.50% | 5.11% |
| [savefile 0.17.6][savefile] | 78.26% | 62.25% | 100.00% | 96.35% | 92.11% | 95.94% |
| [serde_bare 0.5.0][serde_bare] | 2.86% | 3.56% | 100.00% | 96.35% | 92.11% | 98.64% |
| [serde_cbor 0.11.2][serde_cbor] | 0.51% | 0.31% | 45.72% | 68.87% | 72.84% | 8.44% |
| [serde_json 1.0.120][serde_json] | 0.21% | 0.18% | 22.91% | 54.17% | 57.34% | 4.93% |
| [simd-json 0.13.10][simd-json] | 0.35% | 0.20% | 22.91% | 54.17% | 57.34% | 4.91% |
| [speedy 0.8.7][speedy] | 78.44% | 62.37% | 100.00% | 96.35% | 92.11% | 95.84% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.13%\**</span> | <span title="unvalidated">*34.25%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*62.65%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.19%\**</span> | <span title="validated on-demand with error">*2.26%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*3.09%\**</span> | <span title="unvalidated">*62.60%\**</span> <span title="validated upfront with error">*62.57%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*13.76%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*62.56%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 192.68 µs | <span title="unvalidated">*1.2985 ms\**</span> | 1290592 | 399761 | 340930 | 4.9714 ms |
| [alkahest 0.1.5][alkahest] | 216.81 µs | † | 667570 | 325484 | 320452 | 3.9254 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*900.52 µs\**</span> <span title="prepend">*840.03 µs\**</span> | 3.2107 ms | 489348 | 281173 | 249546 | 3.0735 ms |
| [bincode 2.0.0-rc][bincode] | 310.77 µs | 2.0969 ms | 367413 | 221291 | 206273 | 2.4877 ms |
| [bincode 1.3.3][bincode1] | 598.35 µs | 1.9248 ms | 569975 | 240525 | 232423 | 2.8873 ms |
| [bitcode 0.6.0][bitcode] | 132.65 µs | 1.2689 ms | 327688 | 200947 | 182736 | 738.98 µs |
| [borsh 1.5.1][borsh] | 553.33 µs | 1.8453 ms | 446595 | 234236 | 210008 | 2.4839 ms |
| [bson 2.9.0][bson] | 2.8203 ms | 9.0222 ms | 1619653 | 502185 | 328399 | 4.8198 ms |
| [capnp 0.19.6][capnp] | 455.79 µs | † | 803896 | 335606 | 280851 | 3.9561 ms |
| [cbor4ii 0.3.2][cbor4ii] | 779.76 µs | 4.7928 ms | 1109831 | 344745 | 274514 | 3.8554 ms |
| [ciborium 0.2.2][ciborium] | 3.8547 ms | 10.277 ms | 1109821 | 344751 | 274526 | 3.8443 ms |
| [databuf 0.5.0][databuf] | 292.32 µs | 1.7500 ms | 356311 | 213062 | 198488 | 2.3976 ms |
| [dlhn 0.1.7][dlhn] | 804.57 µs | 2.6899 ms | 366496 | 220600 | 205683 | 2.4766 ms |
| [flatbuffers 24.3.25][flatbuffers] | 2.9070 ms | † | 844168 | 345696 | 294015 | 3.8419 ms |
| [msgpacker 0.4.3][msgpacker] | 923.56 µs | 2.8534 ms | 391251 | 236877 | 220476 | 2.6196 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2602 ms | 4.0586 ms | 449745 | 252432 | 231110 | 2.7765 ms |
| [nanoserde 0.1.37][nanoserde] | 271.88 µs | 1.9206 ms | 567975 | 239930 | 232419 | 2.9329 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 613.13 µs | 1.9811 ms | 356311 | 212976 | 198524 | 2.3648 ms |
| [postcard 1.0.8][postcard] | 444.20 µs | 2.0769 ms | 367489 | 221913 | 207344 | 2.5133 ms |
| [pot 3.0.0][pot] | 2.2972 ms | 5.9702 ms | 599125 | 299158 | 247693 | 3.1948 ms |
| [prost 0.12.6][prost] | <span title="encode">*1.2668 ms\**</span> <span title="populate + encode">*2.9230 ms\**</span> | 3.3382 ms | 596811 | 305319 | 269310 | 3.4671 ms |
| [rkyv 0.7.44][rkyv] | 303.39 µs | <span title="unvalidated">*1.2649 ms\**</span> <span title="validated upfront with error">*1.7357 ms\**</span> | 596952 | 253967 | 220706 | 2.6824 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4084 ms | 3.0402 ms | 424533 | 245214 | 226188 | 2.6871 ms |
| [ron 0.8.1][ron] | 7.4113 ms | 17.944 ms | 1465223 | 434935 | 343338 | 5.9539 ms |
| [savefile 0.17.6][savefile] | 206.01 µs | 1.8631 ms | 566991 | 239361 | 232013 | 2.8898 ms |
| [serde_bare 0.5.0][serde_bare] | 739.61 µs | 2.3402 ms | 356311 | 213062 | 198488 | 2.3829 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8008 ms | 4.6486 ms | 1109821 | 344751 | 274526 | 3.8293 ms |
| [serde_json 1.0.120][serde_json] | 3.8422 ms | 6.7736 ms | 1623191 | 466527 | 359623 | 6.0275 ms |
| [simd-json 0.13.10][simd-json] | 2.2192 ms | 5.0468 ms | 1623191 | 466527 | 359623 | 6.0322 ms |
| [speedy 0.8.7][speedy] | 290.49 µs | 1.5960 ms | 449595 | 234970 | 210361 | 2.4620 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*36.822 µs\**</span> | <span title="unvalidated">*37.385 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8553 ns\**</span> | <span title="validated on-demand with panic">*7.1969 µs\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*73.582 ns\**</span> | <span title="validated on-demand with error">*417.73 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4735 ns\**</span> <span title="validated upfront with error">*2.2113 ms\**</span> | <span title="unvalidated">*1.3494 µs\**</span> <span title="validated upfront with error">*2.1913 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2375 ns\**</span> <span title="validated upfront with error">*465.06 µs\**</span> | <span title="unvalidated">*239.01 ns\**</span> <span title="validated upfront with error">*471.05 µs\**</span> | 929.68 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 68.84% | <span title="unvalidated">*97.41%\**</span> | 25.39% | 50.27% | 53.60% | 14.86% |
| [alkahest 0.1.5][alkahest] | 61.18% | † | 49.09% | 61.74% | 57.02% | 18.83% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*14.73%\**</span> <span title="prepend">*15.79%\**</span> | 39.40% | 66.96% | 71.47% | 73.23% | 24.04% |
| [bincode 2.0.0-rc][bincode] | 42.68% | 60.32% | 89.19% | 90.81% | 88.59% | 29.71% |
| [bincode 1.3.3][bincode1] | 22.17% | 65.72% | 57.49% | 83.55% | 78.62% | 25.59% |
| [bitcode 0.6.0][bitcode] | 100.00% | 99.68% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 23.97% | 68.55% | 73.37% | 85.79% | 87.01% | 29.75% |
| [bson 2.9.0][bson] | 4.70% | 14.02% | 20.23% | 40.01% | 55.64% | 15.33% |
| [capnp 0.19.6][capnp] | 29.10% | † | 40.76% | 59.88% | 65.07% | 18.68% |
| [cbor4ii 0.3.2][cbor4ii] | 17.01% | 26.39% | 29.53% | 58.29% | 66.57% | 19.17% |
| [ciborium 0.2.2][ciborium] | 3.44% | 12.31% | 29.53% | 58.29% | 66.56% | 19.22% |
| [databuf 0.5.0][databuf] | 45.38% | 72.28% | 91.97% | 94.31% | 92.06% | 30.82% |
| [dlhn 0.1.7][dlhn] | 16.49% | 47.02% | 89.41% | 91.09% | 88.84% | 29.84% |
| [flatbuffers 24.3.25][flatbuffers] | 4.56% | † | 38.82% | 58.13% | 62.15% | 19.23% |
| [msgpacker 0.4.3][msgpacker] | 14.36% | 44.33% | 83.75% | 84.83% | 82.88% | 28.21% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.52% | 31.17% | 72.86% | 79.60% | 79.07% | 26.62% |
| [nanoserde 0.1.37][nanoserde] | 48.79% | 65.86% | 57.69% | 83.75% | 78.62% | 25.20% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.63% | 63.85% | 91.97% | 94.35% | 92.05% | 31.25% |
| [postcard 1.0.8][postcard] | 29.86% | 60.90% | 89.17% | 90.55% | 88.13% | 29.40% |
| [pot 3.0.0][pot] | 5.77% | 21.19% | 54.69% | 67.17% | 73.78% | 23.13% |
| [prost 0.12.6][prost] | <span title="encode">*10.47%\**</span> <span title="populate + encode">*4.54%\**</span> | 37.89% | 54.91% | 65.82% | 67.85% | 21.31% |
| [rkyv 0.7.44][rkyv] | 43.72% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*72.88%\**</span> | 54.89% | 79.12% | 82.80% | 27.55% |
| [rmp-serde 1.3.0][rmp-serde] | 9.42% | 41.61% | 77.19% | 81.95% | 80.79% | 27.50% |
| [ron 0.8.1][ron] | 1.79% | 7.05% | 22.36% | 46.20% | 53.22% | 12.41% |
| [savefile 0.17.6][savefile] | 64.39% | 67.89% | 57.79% | 83.95% | 78.76% | 25.57% |
| [serde_bare 0.5.0][serde_bare] | 17.94% | 54.05% | 91.97% | 94.31% | 92.06% | 31.01% |
| [serde_cbor 0.11.2][serde_cbor] | 7.37% | 27.21% | 29.53% | 58.29% | 66.56% | 19.30% |
| [serde_json 1.0.120][serde_json] | 3.45% | 18.67% | 20.19% | 43.07% | 50.81% | 12.26% |
| [simd-json 0.13.10][simd-json] | 5.98% | 25.06% | 20.19% | 43.07% | 50.81% | 12.25% |
| [speedy 0.8.7][speedy] | 45.66% | 79.25% | 72.89% | 85.52% | 86.87% | 30.02% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.64%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.70%\**</span> | <span title="validated on-demand with panic">*3.32%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*57.22%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.71%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 492.07 µs | <span title="unvalidated">*2.3383 ms\**</span> | 2984682 | 1411704 | 1273124 | 14.157 ms |
| [alkahest 0.1.5][alkahest] | 588.78 µs | † | 1863391 | 1234113 | 1202345 | 11.425 ms |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*4.6577 ms\**</span> <span title="prepend">*2.4662 ms\**</span> | 8.3904 ms | 1664428 | 1264167 | 1216472 | 10.944 ms |
| [bincode 2.0.0-rc][bincode] | 708.74 µs | 3.7965 ms | 1372381 | 1091486 | 1037296 | 9.0813 ms |
| [bincode 1.3.3][bincode1] | 3.8671 ms | 4.0861 ms | 1811011 | 1115281 | 1025627 | 9.7617 ms |
| [bitcode 0.6.0][bitcode] | 696.47 µs | 2.3319 ms | 948499 | 857321 | 837658 | 3.0511 ms |
| [borsh 1.5.1][borsh] | 2.8685 ms | 2.8502 ms | 1486162 | 1082357 | 1013550 | 9.4947 ms |
| [bson 2.9.0][bson] | 20.647 ms | 49.556 ms | 10030880 | 2833079 | 1600859 | 27.066 ms |
| [capnp 0.19.6][capnp] | 2.1816 ms | † | 2664040 | 1511895 | 1212087 | 14.023 ms |
| [cbor4ii 0.3.2][cbor4ii] | 3.2306 ms | 17.635 ms | 5878791 | 1655835 | 1431390 | 20.521 ms |
| [ciborium 0.2.2][ciborium] | 24.051 ms | 53.558 ms | 5878653 | 1655791 | 1431560 | 20.500 ms |
| [databuf 0.5.0][databuf] | 1.3338 ms | 3.7006 ms | 1288257 | 1037579 | 984337 | 8.6223 ms |
| [dlhn 0.1.7][dlhn] | 5.2124 ms | 7.8215 ms | 1279599 | 1052061 | 1021161 | 8.3538 ms |
| [flatbuffers 24.3.25][flatbuffers] | 4.8597 ms | † | 2273740 | 1408408 | 1235566 | 12.493 ms |
| [msgpacker 0.4.3][msgpacker] | 2.1711 ms | 6.4614 ms | 1424043 | 1128758 | 1110156 | 9.1858 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.415 ms | 16.870 ms | 1728519 | 1247642 | 1233323 | 12.846 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3459 ms | 3.2129 ms | 1770477 | 1108304 | 1029947 | 9.8676 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.9905 ms | 3.3989 ms | 1288257 | 1039269 | 986510 | 8.5516 ms |
| [postcard 1.0.8][postcard] | 2.0479 ms | 4.1207 ms | 1279599 | 1058243 | 1016738 | 8.3156 ms |
| [pot 3.0.0][pot] | 13.197 ms | 30.120 ms | 2544810 | 1447453 | 1268390 | 15.347 ms |
| [prost 0.12.6][prost] | <span title="encode">*5.3722 ms\**</span> <span title="populate + encode">*9.3295 ms\**</span> | 8.4705 ms | 1818378 | 1307777 | 1266311 | 11.614 ms |
| [rkyv 0.7.44][rkyv] | 1.3056 ms | <span title="unvalidated">*2.1493 ms\**</span> <span title="validated upfront with error">*2.6992 ms\**</span> | 2029080 | 1335117 | 1158855 | 11.901 ms |
| [rmp-serde 1.3.0][rmp-serde] | 9.7094 ms | 10.691 ms | 1703813 | 1231892 | 1200208 | 10.888 ms |
| [ron 0.8.1][ron] | 36.868 ms | 92.943 ms | 8476284 | 2181196 | 1783971 | 34.131 ms |
| [savefile 0.17.6][savefile] | 807.23 µs | 2.8231 ms | 1750226 | 1101682 | 1027828 | 9.8000 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7733 ms | 4.6999 ms | 1288257 | 1037597 | 984356 | 8.5785 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.6768 ms | 21.218 ms | 5878653 | 1655791 | 1431560 | 20.996 ms |
| [serde_json 1.0.120][serde_json] | 20.107 ms | 30.136 ms | 9175594 | 2334253 | 1800713 | 33.732 ms |
| [simd-json 0.13.10][simd-json] | 11.536 ms | 26.526 ms | 9175594 | 2334253 | 1800713 | 35.374 ms |
| [speedy 0.8.7][speedy] | 714.55 µs | 2.4314 ms | 1546963 | 1093532 | 1013443 | 9.9529 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.318 µs\**</span> | <span title="unvalidated">*67.137 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8559 ns\**</span> | <span title="validated on-demand with panic">*627.01 ns\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*72.806 ns\**</span> | <span title="validated on-demand with error">*707.06 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4731 ns\**</span> <span title="validated upfront with error">*4.6959 ms\**</span> | <span title="unvalidated">*2.6292 µs\**</span> <span title="validated upfront with error">*4.8402 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2374 ns\**</span> <span title="validated upfront with error">*533.97 µs\**</span> | <span title="unvalidated">*373.77 ns\**</span> <span title="validated upfront with error">*531.62 µs\**</span> | 502.99 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*91.92%\**</span> | 31.78% | 60.73% | 65.80% | 21.55% |
| [alkahest 0.1.5][alkahest] | 83.57% | † | 50.90% | 69.47% | 69.67% | 26.71% |
| [bilrost 0.1010.0][bilrost] | <span title="encode">*10.56%\**</span> <span title="prepend">*19.95%\**</span> | 25.62% | 56.99% | 67.82% | 68.86% | 27.88% |
| [bincode 2.0.0-rc][bincode] | 69.43% | 56.61% | 69.11% | 78.55% | 80.75% | 33.60% |
| [bincode 1.3.3][bincode1] | 12.72% | 52.60% | 52.37% | 76.87% | 81.67% | 31.26% |
| [bitcode 0.6.0][bitcode] | 70.65% | 92.17% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 17.15% | 75.41% | 63.82% | 79.21% | 82.65% | 32.13% |
| [bson 2.9.0][bson] | 2.38% | 4.34% | 9.46% | 30.26% | 52.33% | 11.27% |
| [capnp 0.19.6][capnp] | 22.56% | † | 35.60% | 56.71% | 69.11% | 21.76% |
| [cbor4ii 0.3.2][cbor4ii] | 15.23% | 12.19% | 16.13% | 51.78% | 58.52% | 14.87% |
| [ciborium 0.2.2][ciborium] | 2.05% | 4.01% | 16.13% | 51.78% | 58.51% | 14.88% |
| [databuf 0.5.0][databuf] | 36.89% | 58.08% | 73.63% | 82.63% | 85.10% | 35.39% |
| [dlhn 0.1.7][dlhn] | 9.44% | 27.48% | 74.12% | 81.49% | 82.03% | 36.52% |
| [flatbuffers 24.3.25][flatbuffers] | 10.13% | † | 41.72% | 60.87% | 67.80% | 24.42% |
| [msgpacker 0.4.3][msgpacker] | 22.66% | 33.26% | 66.61% | 75.95% | 75.45% | 33.22% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.62% | 12.74% | 54.87% | 68.72% | 67.92% | 23.75% |
| [nanoserde 0.1.37][nanoserde] | 36.56% | 66.90% | 53.57% | 77.35% | 81.33% | 30.92% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 16.45% | 63.24% | 73.63% | 82.49% | 84.91% | 35.68% |
| [postcard 1.0.8][postcard] | 24.03% | 52.16% | 74.12% | 81.01% | 82.39% | 36.69% |
| [pot 3.0.0][pot] | 3.73% | 7.14% | 37.27% | 59.23% | 66.04% | 19.88% |
| [prost 0.12.6][prost] | <span title="encode">*9.16%\**</span> <span title="populate + encode">*5.27%\**</span> | 25.37% | 52.16% | 65.56% | 66.15% | 26.27% |
| [rkyv 0.7.44][rkyv] | 37.69% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.63%\**</span> | 46.75% | 64.21% | 72.28% | 25.64% |
| [rmp-serde 1.3.0][rmp-serde] | 5.07% | 20.10% | 55.67% | 69.59% | 69.79% | 28.02% |
| [ron 0.8.1][ron] | 1.33% | 2.31% | 11.19% | 39.31% | 46.95% | 8.94% |
| [savefile 0.17.6][savefile] | 60.96% | 76.13% | 54.19% | 77.82% | 81.50% | 31.13% |
| [serde_bare 0.5.0][serde_bare] | 10.31% | 45.73% | 73.63% | 82.63% | 85.10% | 35.57% |
| [serde_cbor 0.11.2][serde_cbor] | 5.09% | 10.13% | 16.13% | 51.78% | 58.51% | 14.53% |
| [serde_json 1.0.120][serde_json] | 2.45% | 7.13% | 10.34% | 36.73% | 46.52% | 9.05% |
| [simd-json 0.13.10][simd-json] | 4.27% | 8.10% | 10.34% | 36.73% | 46.52% | 8.63% |
| [speedy 0.8.7][speedy] | 68.86% | 88.40% | 61.31% | 78.40% | 82.65% | 30.66% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.56%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*59.61%\**</span> | ‡ |
| [capnp 0.19.6][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*52.86%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*14.22%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.07%\**</span> | 100.00% |

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
[rkyv]: https://crates.io/crates/rkyv/0.7.44
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.17.6
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.120
[simd-json]: https://crates.io/crates/simd-json/0.13.10
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
