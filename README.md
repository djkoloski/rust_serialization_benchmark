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

## Last updated: 2025-1-3 0:55:6

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.85.0-nightly (45d11e51b 2025-01-01)
binary: rustc
commit-hash: 45d11e51bb66c2deb63a006fe3953c4b6fbc50c2
commit-date: 2025-01-01
host: x86_64-unknown-linux-gnu
release: 1.85.0-nightly
LLVM version: 19.1.6
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
| [bilrost 0.1011.0][bilrost] | <span title="encode">*700.13 µs\**</span> <span title="prepend">*620.71 µs\**</span> | 3.1345 ms | 874632 | 355446 | 311723 | 5.4662 ms |
| [bincode 2.0.0-rc][bincode] | 321.13 µs | 2.4929 ms | 741295 | 303944 | 257153 | 3.9473 ms |
| [bincode 1.3.3][bincode1] | 523.39 µs | 2.0015 ms | 1045784 | 373127 | 311761 | 4.8398 ms |
| [bitcode 0.6.3][bitcode] | 136.53 µs | 1.4783 ms | 703710 | 288826 | 229755 | 2.5235 ms |
| [borsh 1.5.3][borsh] | 547.16 µs | 2.1931 ms | 885780 | 362204 | 286514 | 4.5190 ms |
| [capnp 0.20.3][capnp] | 479.61 µs | † | 1443216 | 513986 | 428649 | 6.7749 ms |
| [cbor4ii 0.3.3][cbor4ii] | 603.20 µs | 4.8263 ms | 1407835 | 403440 | 324081 | 5.0118 ms |
| [ciborium 0.2.2][ciborium] | 3.2060 ms | 13.294 ms | 1407835 | 403440 | 324081 | 5.0049 ms |
| [databuf 0.5.0][databuf] | 260.97 µs | 2.0782 ms | 765778 | 311715 | 264630 | 4.1127 ms |
| [dlhn 0.1.7][dlhn] | 718.16 µs | 2.5361 ms | 724953 | 301446 | 253629 | 3.7734 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0059 ms | † | 1276368 | 468539 | 388832 | 5.4741 ms |
| [msgpacker 0.4.5][msgpacker] | 1.2512 ms | 2.6023 ms | 764996 | 315291 | 264898 | 4.1775 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.6264 ms | 4.0802 ms | 818669 | 332556 | 285514 | 4.6241 ms |
| [nanoserde 0.1.37][nanoserde] | 265.22 µs | 2.1157 ms | 1045784 | 373127 | 311761 | 4.8521 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 645.00 µs | 2.2289 ms | 765778 | 311743 | 264518 | 4.1115 ms |
| [postcard 1.1.1][postcard] | 427.06 µs | 2.1670 ms | 724953 | 302399 | 253747 | 3.7322 ms |
| [pot 3.0.1][pot] | 2.3709 ms | 6.4643 ms | 971922 | 372513 | 304122 | 4.9860 ms |
| [prost 0.13.4][prost] | <span title="encode">*889.26 µs\**</span> <span title="populate + encode">*2.4307 ms\**</span> | 3.3715 ms | 884628 | 363130 | 315494 | 5.0611 ms |
| [rkyv 0.8.9][rkyv] | 249.95 µs | <span title="unvalidated">*1.5921 ms\**</span> <span title="validated upfront with error">*2.1891 ms\**</span> | 1011488 | 393526 | 326517 | 5.1918 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3588 ms | 3.1562 ms | 784997 | 325384 | 278219 | 4.4220 ms |
| [ron 0.8.1][ron] | 11.435 ms | 15.236 ms | 1607459 | 449158 | 349713 | 6.1846 ms |
| [savefile 0.18.5][savefile] | 189.23 µs | 2.2189 ms | 1045800 | 373139 | 311761 | 4.8111 ms |
| [serde-brief 0.1.0][serde-brief] | 1.4917 ms | 4.8641 ms | 1584946 | 413733 | 341439 | 5.1853 ms |
| [serde_bare 0.5.0][serde_bare] | 700.77 µs | 2.0961 ms | 765778 | 311715 | 264630 | 4.1647 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0125 ms | 4.7509 ms | 1407835 | 403440 | 324081 | 5.0899 ms |
| [serde_json 1.0.128][serde_json] | 4.0691 ms | 5.4481 ms | 1827461 | 470560 | 361090 | 6.0782 ms |
| [simd-json 0.14.3][simd-json] | 2.0884 ms | 4.6822 ms | 1827461 | 470560 | 361090 | 5.8915 ms |
| [speedy 0.8.7][speedy] | 203.19 µs | 1.7929 ms | 885780 | 362204 | 286514 | 4.5336 ms |
| [wiring 0.2.2][wiring] | 191.34 µs | 1.9977 ms | 1045784 | 337930 | 276188 | 4.1892 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.674 ns\**</span> | <span title="validated on-demand with error">*167.78 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4743 ns\**</span> <span title="validated upfront with error">*2.0794 ms\**</span> | <span title="unvalidated">*51.641 µs\**</span> <span title="validated upfront with error">*2.1566 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2374 ns\**</span> <span title="validated upfront with error">*595.01 µs\**</span> | <span title="unvalidated">*10.557 µs\**</span> <span title="validated upfront with error">*605.00 µs\**</span> | <span title="unvalidated">*7.5727 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*19.50%\**</span> <span title="prepend">*22.00%\**</span> | 47.16% | 80.46% | 81.26% | 73.70% | 46.17% |
| [bincode 2.0.0-rc][bincode] | 42.52% | 59.30% | 94.93% | 95.03% | 89.35% | 63.93% |
| [bincode 1.3.3][bincode1] | 26.09% | 73.86% | 67.29% | 77.41% | 73.70% | 52.14% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.95% | 67.41% | 79.45% | 79.74% | 80.19% | 55.84% |
| [capnp 0.20.3][capnp] | 28.47% | † | 48.76% | 56.19% | 53.60% | 37.25% |
| [cbor4ii 0.3.3][cbor4ii] | 22.63% | 30.63% | 49.99% | 71.59% | 70.89% | 50.35% |
| [ciborium 0.2.2][ciborium] | 4.26% | 11.12% | 49.99% | 71.59% | 70.89% | 50.42% |
| [databuf 0.5.0][databuf] | 52.32% | 71.13% | 91.89% | 92.66% | 86.82% | 61.36% |
| [dlhn 0.1.7][dlhn] | 19.01% | 58.29% | 97.07% | 95.81% | 90.59% | 66.88% |
| [flatbuffers 24.12.23][flatbuffers] | 13.57% | † | 55.13% | 61.64% | 59.09% | 46.10% |
| [msgpacker 0.4.5][msgpacker] | 10.91% | 56.81% | 91.99% | 91.61% | 86.73% | 60.41% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.43% | 36.23% | 85.96% | 86.85% | 80.47% | 54.57% |
| [nanoserde 0.1.37][nanoserde] | 51.48% | 69.87% | 67.29% | 77.41% | 73.70% | 52.01% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.17% | 66.32% | 91.89% | 92.65% | 86.86% | 61.38% |
| [postcard 1.1.1][postcard] | 31.97% | 68.22% | 97.07% | 95.51% | 90.54% | 67.61% |
| [pot 3.0.1][pot] | 5.76% | 22.87% | 72.40% | 77.53% | 75.55% | 50.61% |
| [prost 0.13.4][prost] | <span title="encode">*15.35%\**</span> <span title="populate + encode">*5.62%\**</span> | 43.85% | 79.55% | 79.54% | 72.82% | 49.86% |
| [rkyv 0.8.9][rkyv] | 54.62% | <span title="unvalidated">*92.85%\**</span> <span title="validated upfront with error">*67.53%\**</span> | 69.57% | 73.39% | 70.37% | 48.61% |
| [rmp-serde 1.3.0][rmp-serde] | 10.05% | 46.84% | 89.64% | 88.76% | 82.58% | 57.07% |
| [ron 0.8.1][ron] | 1.19% | 9.70% | 43.78% | 64.30% | 65.70% | 40.80% |
| [savefile 0.18.5][savefile] | 72.15% | 66.62% | 67.29% | 77.40% | 73.70% | 52.45% |
| [serde-brief 0.1.0][serde-brief] | 9.15% | 30.39% | 44.40% | 69.81% | 67.29% | 48.67% |
| [serde_bare 0.5.0][serde_bare] | 19.48% | 70.53% | 91.89% | 92.66% | 86.82% | 60.59% |
| [serde_cbor 0.11.2][serde_cbor] | 6.78% | 31.12% | 49.99% | 71.59% | 70.89% | 49.58% |
| [serde_json 1.0.128][serde_json] | 3.36% | 27.13% | 38.51% | 61.38% | 63.63% | 41.52% |
| [simd-json 0.14.3][simd-json] | 6.54% | 31.57% | 38.51% | 61.38% | 63.63% | 42.83% |
| [speedy 0.8.7][speedy] | 67.19% | 82.45% | 79.45% | 79.74% | 80.19% | 55.66% |
| [wiring 0.2.2][wiring] | 71.35% | 74.00% | 67.29% | 85.47% | 83.19% | 60.24% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*6.29%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.44%\**</span> <span title="validated upfront with error">*0.49%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.74%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*6.6888 ms\**</span> <span title="prepend">*8.7435 ms\**</span> | 9.6007 ms | 8625005 | 6443961 | 6231572 | 74.124 ms |
| [bincode 2.0.0-rc][bincode] | 2.3978 ms | 1.0176 ms | 6000005 | 5378497 | 5345897 | 7.5601 ms |
| [bincode 1.3.3][bincode1] | 5.1554 ms | 5.4500 ms | 6000008 | 5378500 | 5345890 | 7.4176 ms |
| [bitcode 0.6.3][bitcode] | 1.4109 ms | 791.83 µs | 6000006 | 5182295 | 4923880 | 13.568 ms |
| [borsh 1.5.3][borsh] | 6.3513 ms | 4.5724 ms | 6000004 | 5378496 | 5345889 | 8.0771 ms |
| [capnp 0.20.3][capnp] | 6.0746 ms | † | 14000088 | 7130367 | 6051062 | 78.571 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.9435 ms | 49.630 ms | 13125016 | 7524114 | 6757967 | 90.392 ms |
| [ciborium 0.2.2][ciborium] | 67.922 ms | 118.62 ms | 13122324 | 7524660 | 6759658 | 90.207 ms |
| [databuf 0.5.0][databuf] | 2.3953 ms | 5.3075 ms | 6000003 | 5378495 | 5345900 | 8.1312 ms |
| [dlhn 0.1.7][dlhn] | 6.3582 ms | 6.9132 ms | 6000003 | 5378495 | 5345900 | 8.0469 ms |
| [flatbuffers 24.12.23][flatbuffers] | 874.55 µs | † | 6000024 | 5378434 | 5345910 | 7.6990 ms |
| [msgpacker 0.4.5][msgpacker] | 18.313 ms | 5.1424 ms | 7500005 | 6058442 | 6014337 | 10.223 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.16 ms | 33.719 ms | 8125037 | 6493484 | 6386940 | 68.475 ms |
| [nanoserde 0.1.37][nanoserde] | 1.5560 ms | 1.0993 ms | 6000008 | 5378500 | 5345890 | 8.0552 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1138 ms | 3.9807 ms | 6000004 | 5378496 | 5345889 | 8.0463 ms |
| [postcard 1.1.1][postcard] | 489.74 µs | 1.7265 ms | 6000003 | 5378495 | 5345900 | 8.0091 ms |
| [pot 3.0.1][pot] | 40.171 ms | 72.932 ms | 10122342 | 6814618 | 6852251 | 79.156 ms |
| [prost 0.13.4][prost] | <span title="encode">*7.7218 ms\**</span> <span title="populate + encode">*8.5576 ms\**</span> | 15.449 ms | 8750000 | 6665735 | 6421871 | 71.262 ms |
| [rkyv 0.8.9][rkyv] | 237.14 µs | <span title="unvalidated">*148.15 µs\**</span> <span title="validated upfront with error">*148.15 µs\**</span> | 6000008 | 5378500 | 5345892 | 8.0166 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.186 ms | 18.221 ms | 8125006 | 6494876 | 6391037 | 66.978 ms |
| [ron 0.8.1][ron] | 171.94 ms | 236.83 ms | 22192885 | 8970395 | 8138755 | 149.03 ms |
| [savefile 0.18.5][savefile] | 237.36 µs | 238.43 µs | 6000024 | 5378519 | 5345892 | 7.8059 ms |
| [serde-brief 0.1.0][serde-brief] | 22.150 ms | 37.331 ms | 15750015 | 8024540 | 6816643 | 91.887 ms |
| [serde_bare 0.5.0][serde_bare] | 6.5270 ms | 4.7290 ms | 6000003 | 5378495 | 5345900 | 8.1571 ms |
| [serde_cbor 0.11.2][serde_cbor] | 34.956 ms | 47.373 ms | 13122324 | 7524660 | 6759658 | 89.412 ms |
| [serde_json 1.0.128][serde_json] | 88.206 ms | 85.051 ms | 26192883 | 9566084 | 8586741 | 152.18 ms |
| [simd-json 0.14.3][simd-json] | 53.177 ms | 68.934 ms | 26192883 | 9566084 | 8586741 | 151.79 ms |
| [speedy 0.8.7][speedy] | 238.00 µs | 238.52 µs | 6000004 | 5378496 | 5345889 | 8.0244 ms |
| [wiring 0.2.2][wiring] | 197.16 µs | 351.71 µs | 6000008 | 5378952 | 5345894 | 7.7279 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*104.88 ns\**</span> | <span title="validated on-demand with error">*2.1361 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4741 ns\**</span> <span title="validated upfront with error">*39.886 ns\**</span> | <span title="unvalidated">*54.009 µs\**</span> <span title="validated upfront with error">*77.753 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2376 ns\**</span> <span title="validated upfront with error">*5.5732 ns\**</span> | <span title="unvalidated">*48.391 µs\**</span> <span title="validated upfront with error">*38.734 µs\**</span> | <span title="unvalidated">*77.444 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*2.95%\**</span> <span title="prepend">*2.25%\**</span> | 1.54% | 69.57% | 80.42% | 79.02% | 10.01% |
| [bincode 2.0.0-rc][bincode] | 8.22% | 14.56% | 100.00% | 96.35% | 92.11% | 98.12% |
| [bincode 1.3.3][bincode1] | 3.82% | 2.72% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bitcode 0.6.3][bitcode] | 13.97% | 18.71% | 100.00% | 100.00% | 100.00% | 54.67% |
| [borsh 1.5.3][borsh] | 3.10% | 3.24% | 100.00% | 96.35% | 92.11% | 91.83% |
| [capnp 0.20.3][capnp] | 3.25% | † | 42.86% | 72.68% | 81.37% | 9.44% |
| [cbor4ii 0.3.3][cbor4ii] | 1.98% | 0.30% | 45.71% | 68.88% | 72.86% | 8.21% |
| [ciborium 0.2.2][ciborium] | 0.29% | 0.12% | 45.72% | 68.87% | 72.84% | 8.22% |
| [databuf 0.5.0][databuf] | 8.23% | 2.79% | 100.00% | 96.35% | 92.11% | 91.22% |
| [dlhn 0.1.7][dlhn] | 3.10% | 2.14% | 100.00% | 96.35% | 92.11% | 92.18% |
| [flatbuffers 24.12.23][flatbuffers] | 22.54% | † | 100.00% | 96.35% | 92.11% | 96.34% |
| [msgpacker 0.4.5][msgpacker] | 1.08% | 2.88% | 80.00% | 85.54% | 81.87% | 72.56% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.44% | 73.85% | 79.81% | 77.09% | 10.83% |
| [nanoserde 0.1.37][nanoserde] | 12.67% | 13.48% | 100.00% | 96.35% | 92.11% | 92.08% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.86% | 3.72% | 100.00% | 96.35% | 92.11% | 92.19% |
| [postcard 1.1.1][postcard] | 40.26% | 8.58% | 100.00% | 96.35% | 92.11% | 92.61% |
| [pot 3.0.1][pot] | 0.49% | 0.20% | 59.27% | 76.05% | 71.86% | 9.37% |
| [prost 0.13.4][prost] | <span title="encode">*2.55%\**</span> <span title="populate + encode">*2.30%\**</span> | 0.96% | 68.57% | 77.75% | 76.67% | 10.41% |
| [rkyv 0.8.9][rkyv] | 83.14% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 92.53% |
| [rmp-serde 1.3.0][rmp-serde] | 1.30% | 0.81% | 73.85% | 79.79% | 77.04% | 11.07% |
| [ron 0.8.1][ron] | 0.11% | 0.06% | 27.04% | 57.77% | 60.50% | 4.98% |
| [savefile 0.18.5][savefile] | 83.06% | 62.14% | 100.00% | 96.35% | 92.11% | 95.03% |
| [serde-brief 0.1.0][serde-brief] | 0.89% | 0.40% | 38.10% | 64.58% | 72.23% | 8.07% |
| [serde_bare 0.5.0][serde_bare] | 3.02% | 3.13% | 100.00% | 96.35% | 92.11% | 90.93% |
| [serde_cbor 0.11.2][serde_cbor] | 0.56% | 0.31% | 45.72% | 68.87% | 72.84% | 8.30% |
| [serde_json 1.0.128][serde_json] | 0.22% | 0.17% | 22.91% | 54.17% | 57.34% | 4.87% |
| [simd-json 0.14.3][simd-json] | 0.37% | 0.21% | 22.91% | 54.17% | 57.34% | 4.89% |
| [speedy 0.8.7][speedy] | 82.84% | 62.11% | 100.00% | 96.35% | 92.11% | 92.44% |
| [wiring 0.2.2][wiring] | 100.00% | 42.12% | 100.00% | 96.34% | 92.11% | 95.98% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*3.10%\**</span> | <span title="unvalidated">*71.72%\**</span> <span title="validated upfront with error">*49.82%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*22.21%\**</span> | <span title="unvalidated">*80.04%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*944.56 µs\**</span> <span title="prepend">*841.26 µs\**</span> | 3.1501 ms | 489348 | 281173 | 249546 | 3.0308 ms |
| [bincode 2.0.0-rc][bincode] | 319.80 µs | 2.0877 ms | 367413 | 221291 | 206273 | 2.5081 ms |
| [bincode 1.3.3][bincode1] | 598.41 µs | 1.8458 ms | 569975 | 240525 | 232423 | 2.8660 ms |
| [bitcode 0.6.3][bitcode] | 143.20 µs | 1.2719 ms | 327688 | 200947 | 182736 | 763.30 µs |
| [borsh 1.5.3][borsh] | 556.81 µs | 1.7979 ms | 446595 | 234236 | 210008 | 2.5012 ms |
| [capnp 0.20.3][capnp] | 476.78 µs | † | 803896 | 335606 | 280851 | 3.9064 ms |
| [cbor4ii 0.3.3][cbor4ii] | 790.89 µs | 4.6235 ms | 1109831 | 344745 | 274514 | 3.8221 ms |
| [ciborium 0.2.2][ciborium] | 3.7277 ms | 10.752 ms | 1109821 | 344751 | 274526 | 3.8157 ms |
| [databuf 0.5.0][databuf] | 320.60 µs | 1.7361 ms | 356311 | 213062 | 198488 | 2.3717 ms |
| [dlhn 0.1.7][dlhn] | 788.19 µs | 2.6162 ms | 366496 | 220600 | 205683 | 2.4692 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.2273 ms | † | 844168 | 345696 | 294015 | 3.8091 ms |
| [msgpacker 0.4.5][msgpacker] | 953.56 µs | 2.8456 ms | 391251 | 236877 | 220476 | 2.7740 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3494 ms | 3.9748 ms | 449745 | 252432 | 231110 | 2.7933 ms |
| [nanoserde 0.1.37][nanoserde] | 273.98 µs | 1.9170 ms | 567975 | 239930 | 232419 | 2.8540 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 613.22 µs | 1.9650 ms | 356311 | 212976 | 198524 | 2.3721 ms |
| [postcard 1.1.1][postcard] | 448.51 µs | 1.9944 ms | 367489 | 221913 | 207344 | 2.4644 ms |
| [pot 3.0.1][pot] | 2.4699 ms | 5.9653 ms | 599125 | 299158 | 247693 | 3.2065 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.0811 ms\**</span> <span title="populate + encode">*2.7446 ms\**</span> | 3.4650 ms | 596811 | 305319 | 269310 | 3.4411 ms |
| [rkyv 0.8.9][rkyv] | 333.05 µs | <span title="unvalidated">*1.4995 ms\**</span> <span title="validated upfront with error">*2.0417 ms\**</span> | 603776 | 254776 | 220087 | 2.6981 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4469 ms | 3.0229 ms | 424533 | 245214 | 226188 | 2.6900 ms |
| [ron 0.8.1][ron] | 7.0886 ms | 17.195 ms | 1465223 | 434935 | 343338 | 5.8685 ms |
| [savefile 0.18.5][savefile] | 208.23 µs | 1.8278 ms | 566991 | 239362 | 232010 | 2.8841 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3527 ms | 5.3449 ms | 1276014 | 373898 | 293679 | 4.0899 ms |
| [serde_bare 0.5.0][serde_bare] | 733.59 µs | 2.3201 ms | 356311 | 213062 | 198488 | 2.4755 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8778 ms | 4.7025 ms | 1109821 | 344751 | 274526 | 3.8362 ms |
| [serde_json 1.0.128][serde_json] | 3.8522 ms | 6.4889 ms | 1623191 | 466527 | 359623 | 6.0847 ms |
| [simd-json 0.14.3][simd-json] | 2.2273 ms | 4.6012 ms | 1623191 | 466527 | 359623 | 6.1046 ms |
| [speedy 0.8.7][speedy] | 268.87 µs | 1.5928 ms | 449595 | 234970 | 210361 | 2.4977 ms |
| [wiring 0.2.2][wiring] | 205.31 µs | 1.8254 ms | 566975 | 247810 | 225259 | 2.9360 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.796 ns\**</span> | <span title="validated on-demand with error">*409.57 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4746 ns\**</span> <span title="validated upfront with error">*2.2388 ms\**</span> | <span title="unvalidated">*1.3561 µs\**</span> <span title="validated upfront with error">*2.2011 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2371 ns\**</span> <span title="validated upfront with error">*526.76 µs\**</span> | <span title="unvalidated">*163.23 ns\**</span> <span title="validated upfront with error">*518.89 µs\**</span> | <span title="unvalidated">*715.33 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*15.16%\**</span> <span title="prepend">*17.02%\**</span> | 40.38% | 66.96% | 71.47% | 73.23% | 25.18% |
| [bincode 2.0.0-rc][bincode] | 44.78% | 60.92% | 89.19% | 90.81% | 88.59% | 30.43% |
| [bincode 1.3.3][bincode1] | 23.93% | 68.91% | 57.49% | 83.55% | 78.62% | 26.63% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 25.72% | 70.74% | 73.37% | 85.79% | 87.01% | 30.52% |
| [capnp 0.20.3][capnp] | 30.03% | † | 40.76% | 59.88% | 65.07% | 19.54% |
| [cbor4ii 0.3.3][cbor4ii] | 18.11% | 27.51% | 29.53% | 58.29% | 66.57% | 19.97% |
| [ciborium 0.2.2][ciborium] | 3.84% | 11.83% | 29.53% | 58.29% | 66.56% | 20.00% |
| [databuf 0.5.0][databuf] | 44.67% | 73.26% | 91.97% | 94.31% | 92.06% | 32.18% |
| [dlhn 0.1.7][dlhn] | 18.17% | 48.62% | 89.41% | 91.09% | 88.84% | 30.91% |
| [flatbuffers 24.12.23][flatbuffers] | 4.44% | † | 38.82% | 58.13% | 62.15% | 20.04% |
| [msgpacker 0.4.5][msgpacker] | 15.02% | 44.70% | 83.75% | 84.83% | 82.88% | 27.52% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.68% | 32.00% | 72.86% | 79.60% | 79.07% | 27.33% |
| [nanoserde 0.1.37][nanoserde] | 52.27% | 66.35% | 57.69% | 83.75% | 78.62% | 26.74% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 23.35% | 64.73% | 91.97% | 94.35% | 92.05% | 32.18% |
| [postcard 1.1.1][postcard] | 31.93% | 63.77% | 89.17% | 90.55% | 88.13% | 30.97% |
| [pot 3.0.1][pot] | 5.80% | 21.32% | 54.69% | 67.17% | 73.78% | 23.80% |
| [prost 0.13.4][prost] | <span title="encode">*13.25%\**</span> <span title="populate + encode">*5.22%\**</span> | 36.71% | 54.91% | 65.82% | 67.85% | 22.18% |
| [rkyv 0.8.9][rkyv] | 43.00% | <span title="unvalidated">*84.82%\**</span> <span title="validated upfront with error">*62.30%\**</span> | 54.27% | 78.87% | 83.03% | 28.29% |
| [rmp-serde 1.3.0][rmp-serde] | 9.90% | 42.08% | 77.19% | 81.95% | 80.79% | 28.38% |
| [ron 0.8.1][ron] | 2.02% | 7.40% | 22.36% | 46.20% | 53.22% | 13.01% |
| [savefile 0.18.5][savefile] | 68.77% | 69.59% | 57.79% | 83.95% | 78.76% | 26.47% |
| [serde-brief 0.1.0][serde-brief] | 10.59% | 23.80% | 25.68% | 53.74% | 62.22% | 18.66% |
| [serde_bare 0.5.0][serde_bare] | 19.52% | 54.82% | 91.97% | 94.31% | 92.06% | 30.83% |
| [serde_cbor 0.11.2][serde_cbor] | 7.63% | 27.05% | 29.53% | 58.29% | 66.56% | 19.90% |
| [serde_json 1.0.128][serde_json] | 3.72% | 19.60% | 20.19% | 43.07% | 50.81% | 12.54% |
| [simd-json 0.14.3][simd-json] | 6.43% | 27.64% | 20.19% | 43.07% | 50.81% | 12.50% |
| [speedy 0.8.7][speedy] | 53.26% | 79.85% | 72.89% | 85.52% | 86.87% | 30.56% |
| [wiring 0.2.2][wiring] | 69.75% | 69.68% | 57.80% | 81.09% | 81.12% | 26.00% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*39.85%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.04%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*4.4539 ms\**</span> <span title="prepend">*2.4912 ms\**</span> | 8.1901 ms | 1664428 | 1264167 | 1216472 | 11.047 ms |
| [bincode 2.0.0-rc][bincode] | 1.1967 ms | 4.1850 ms | 1372381 | 1091486 | 1037296 | 9.0580 ms |
| [bincode 1.3.3][bincode1] | 3.8918 ms | 4.0699 ms | 1811011 | 1115281 | 1025627 | 9.8469 ms |
| [bitcode 0.6.3][bitcode] | 699.03 µs | 2.3095 ms | 948499 | 857321 | 837658 | 3.0095 ms |
| [borsh 1.5.3][borsh] | 2.9251 ms | 2.8131 ms | 1486162 | 1082357 | 1013550 | 9.6187 ms |
| [capnp 0.20.3][capnp] | 2.3696 ms | † | 2664040 | 1511895 | 1212087 | 14.007 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.2157 ms | 18.005 ms | 5878791 | 1655835 | 1431390 | 20.935 ms |
| [ciborium 0.2.2][ciborium] | 23.074 ms | 53.974 ms | 5878653 | 1655791 | 1431560 | 21.013 ms |
| [databuf 0.5.0][databuf] | 1.2539 ms | 3.7374 ms | 1288257 | 1037579 | 984337 | 8.3754 ms |
| [dlhn 0.1.7][dlhn] | 4.9739 ms | 6.7913 ms | 1279599 | 1052061 | 1021161 | 8.2018 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.1126 ms | † | 2273740 | 1408408 | 1235566 | 12.610 ms |
| [msgpacker 0.4.5][msgpacker] | 2.2666 ms | 6.7114 ms | 1424043 | 1128758 | 1110156 | 9.0698 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.338 ms | 17.926 ms | 1728519 | 1247642 | 1233323 | 11.521 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2752 ms | 2.8912 ms | 1770477 | 1108304 | 1029947 | 9.6629 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.7773 ms | 3.1035 ms | 1288257 | 1039269 | 986510 | 8.3478 ms |
| [postcard 1.1.1][postcard] | 2.0984 ms | 4.1400 ms | 1279599 | 1058243 | 1016738 | 8.3077 ms |
| [pot 3.0.1][pot] | 13.916 ms | 30.080 ms | 2544810 | 1447453 | 1268390 | 15.114 ms |
| [prost 0.13.4][prost] | <span title="encode">*4.8770 ms\**</span> <span title="populate + encode">*8.7255 ms\**</span> | 8.8930 ms | 1818378 | 1307777 | 1266311 | 11.378 ms |
| [rkyv 0.8.9][rkyv] | 984.69 µs | <span title="unvalidated">*2.1599 ms\**</span> <span title="validated upfront with error">*2.5815 ms\**</span> | 2029080 | 1351984 | 1183990 | 12.604 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.082 ms | 10.620 ms | 1703813 | 1231892 | 1200208 | 10.801 ms |
| [ron 0.8.1][ron] | 36.273 ms | 86.537 ms | 8476284 | 2181196 | 1783971 | 33.106 ms |
| [savefile 0.18.5][savefile] | 830.87 µs | 2.7561 ms | 1750226 | 1101682 | 1027828 | 9.9352 ms |
| [serde-brief 0.1.0][serde-brief] | 6.7263 ms | 21.320 ms | 6796949 | 1754624 | 1533223 | 23.122 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9342 ms | 4.7217 ms | 1288257 | 1037597 | 984356 | 8.4425 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.6202 ms | 20.534 ms | 5878653 | 1655791 | 1431560 | 20.984 ms |
| [serde_json 1.0.128][serde_json] | 21.782 ms | 29.113 ms | 9175594 | 2334253 | 1800713 | 33.619 ms |
| [simd-json 0.14.3][simd-json] | 11.384 ms | 24.865 ms | 9175594 | 2334253 | 1800713 | 33.640 ms |
| [speedy 0.8.7][speedy] | 749.45 µs | 2.4399 ms | 1546963 | 1093532 | 1013443 | 9.4517 ms |
| [wiring 0.2.2][wiring] | 630.13 µs | 2.7263 ms | 1750210 | 1129857 | 1058906 | 10.074 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*73.628 ns\**</span> | <span title="validated on-demand with error">*953.68 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4748 ns\**</span> <span title="validated upfront with error">*5.2701 ms\**</span> | <span title="unvalidated">*2.6285 µs\**</span> <span title="validated upfront with error">*5.1861 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2378 ns\**</span> <span title="validated upfront with error">*423.47 µs\**</span> | <span title="unvalidated">*438.35 ns\**</span> <span title="validated upfront with error">*424.84 µs\**</span> | <span title="unvalidated">*237.29 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*14.15%\**</span> <span title="prepend">*25.29%\**</span> | 26.37% | 56.99% | 67.82% | 68.86% | 27.24% |
| [bincode 2.0.0-rc][bincode] | 52.66% | 51.61% | 69.11% | 78.55% | 80.75% | 33.22% |
| [bincode 1.3.3][bincode1] | 16.19% | 53.07% | 52.37% | 76.87% | 81.67% | 30.56% |
| [bitcode 0.6.3][bitcode] | 90.14% | 93.52% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 21.54% | 76.78% | 63.82% | 79.21% | 82.65% | 31.29% |
| [capnp 0.20.3][capnp] | 26.59% | † | 35.60% | 56.71% | 69.11% | 21.49% |
| [cbor4ii 0.3.3][cbor4ii] | 19.60% | 12.00% | 16.13% | 51.78% | 58.52% | 14.38% |
| [ciborium 0.2.2][ciborium] | 2.73% | 4.00% | 16.13% | 51.78% | 58.51% | 14.32% |
| [databuf 0.5.0][databuf] | 50.25% | 57.79% | 73.63% | 82.63% | 85.10% | 35.93% |
| [dlhn 0.1.7][dlhn] | 12.67% | 31.80% | 74.12% | 81.49% | 82.03% | 36.69% |
| [flatbuffers 24.12.23][flatbuffers] | 12.33% | † | 41.72% | 60.87% | 67.80% | 23.87% |
| [msgpacker 0.4.5][msgpacker] | 27.80% | 32.18% | 66.61% | 75.95% | 75.45% | 33.18% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.08% | 12.05% | 54.87% | 68.72% | 67.92% | 26.12% |
| [nanoserde 0.1.37][nanoserde] | 49.41% | 74.71% | 53.57% | 77.35% | 81.33% | 31.14% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.69% | 69.60% | 73.63% | 82.49% | 84.91% | 36.05% |
| [postcard 1.1.1][postcard] | 30.03% | 52.17% | 74.12% | 81.01% | 82.39% | 36.23% |
| [pot 3.0.1][pot] | 4.53% | 7.18% | 37.27% | 59.23% | 66.04% | 19.91% |
| [prost 0.13.4][prost] | <span title="encode">*12.92%\**</span> <span title="populate + encode">*7.22%\**</span> | 24.29% | 52.16% | 65.56% | 66.15% | 26.45% |
| [rkyv 0.8.9][rkyv] | 63.99% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.67%\**</span> | 46.75% | 63.41% | 70.75% | 23.88% |
| [rmp-serde 1.3.0][rmp-serde] | 6.25% | 20.34% | 55.67% | 69.59% | 69.79% | 27.86% |
| [ron 0.8.1][ron] | 1.74% | 2.50% | 11.19% | 39.31% | 46.95% | 9.09% |
| [savefile 0.18.5][savefile] | 75.84% | 78.37% | 54.19% | 77.82% | 81.50% | 30.29% |
| [serde-brief 0.1.0][serde-brief] | 9.37% | 10.13% | 13.95% | 48.86% | 54.63% | 13.02% |
| [serde_bare 0.5.0][serde_bare] | 12.77% | 45.74% | 73.63% | 82.63% | 85.10% | 35.65% |
| [serde_cbor 0.11.2][serde_cbor] | 6.55% | 10.52% | 16.13% | 51.78% | 58.51% | 14.34% |
| [serde_json 1.0.128][serde_json] | 2.89% | 7.42% | 10.34% | 36.73% | 46.52% | 8.95% |
| [simd-json 0.14.3][simd-json] | 5.54% | 8.69% | 10.34% | 36.73% | 46.52% | 8.95% |
| [speedy 0.8.7][speedy] | 84.08% | 88.52% | 61.31% | 78.40% | 82.65% | 31.84% |
| [wiring 0.2.2][wiring] | 100.00% | 79.22% | 54.19% | 75.88% | 79.11% | 29.87% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*45.96%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.68%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1011.0
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
[serde_json]: https://crates.io/crates/serde_json/1.0.128
[simd-json]: https://crates.io/crates/simd-json/0.14.3
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.2.2


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
