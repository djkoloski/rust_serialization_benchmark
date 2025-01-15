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

## Last updated: 2025-1-15 15:37:15

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.86.0-nightly (8361aef0d 2025-01-14)
binary: rustc
commit-hash: 8361aef0d7c29b1501a316a208ed84cd8a2ae5da
commit-date: 2025-01-14
host: x86_64-unknown-linux-gnu
release: 1.86.0-nightly
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
| [bilrost 0.1011.0][bilrost] | <span title="encode">*706.78 µs\**</span> <span title="prepend">*617.28 µs\**</span> | 3.2943 ms | 874632 | 355446 | 311723 | 5.6407 ms |
| [bincode 2.0.0-rc][bincode] | 324.57 µs | 2.5190 ms | 741295 | 303944 | 257153 | 4.0131 ms |
| [bincode 1.3.3][bincode1] | 522.23 µs | 2.1210 ms | 1045784 | 373127 | 311761 | 4.9215 ms |
| [bitcode 0.6.3][bitcode] | 145.29 µs | 1.4813 ms | 703710 | 288826 | 229755 | 2.6221 ms |
| [borsh 1.5.3][borsh] | 546.79 µs | 2.2028 ms | 885780 | 362204 | 286514 | 4.5344 ms |
| [capnp 0.20.3][capnp] | 528.78 µs | † | 1443216 | 513986 | 428649 | 6.8550 ms |
| [cbor4ii 0.3.3][cbor4ii] | 589.71 µs | 4.9563 ms | 1407835 | 403440 | 324081 | 5.1224 ms |
| [ciborium 0.2.2][ciborium] | 4.1083 ms | 12.128 ms | 1407835 | 403440 | 324081 | 5.0771 ms |
| [databuf 0.5.0][databuf] | 255.61 µs | 2.1682 ms | 765778 | 311715 | 264630 | 4.1830 ms |
| [dlhn 0.1.7][dlhn] | 759.95 µs | 2.6335 ms | 724953 | 301446 | 253629 | 3.8243 ms |
| [flatbuffers 24.12.23][flatbuffers] | 1.0170 ms | † | 1276368 | 468539 | 388832 | 5.5459 ms |
| [msgpacker 0.4.5][msgpacker] | 1.2644 ms | 2.6346 ms | 764996 | 315291 | 264898 | 4.2230 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3015 ms | 4.2109 ms | 818669 | 332556 | 285514 | 4.7350 ms |
| [nanoserde 0.1.37][nanoserde] | 257.22 µs | 2.1394 ms | 1045784 | 373127 | 311761 | 4.7695 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 661.54 µs | 2.3234 ms | 765778 | 311743 | 264518 | 4.1816 ms |
| [postcard 1.1.1][postcard] | 427.92 µs | 2.2319 ms | 724953 | 302399 | 253747 | 3.8274 ms |
| [pot 3.0.1][pot] | 2.2684 ms | 6.5011 ms | 971922 | 372513 | 304122 | 5.4831 ms |
| [prost 0.13.4][prost] | <span title="encode">*953.77 µs\**</span> <span title="populate + encode">*2.5000 ms\**</span> | 3.3305 ms | 884628 | 363130 | 315494 | 5.1571 ms |
| [rkyv 0.8.9][rkyv] | 261.88 µs | <span title="unvalidated">*1.6027 ms\**</span> <span title="validated upfront with error">*2.1938 ms\**</span> | 1011488 | 393526 | 326517 | 5.2511 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4911 ms | 3.3776 ms | 784997 | 325384 | 278219 | 4.3710 ms |
| [ron 0.8.1][ron] | 11.308 ms | 15.326 ms | 1607459 | 449158 | 349713 | 6.0885 ms |
| [savefile 0.18.5][savefile] | 190.98 µs | 2.2667 ms | 1045800 | 373139 | 311761 | 4.8662 ms |
| [serde-brief 0.1.0][serde-brief] | 1.4990 ms | 5.0227 ms | 1584946 | 413733 | 341439 | 5.2553 ms |
| [serde_bare 0.5.0][serde_bare] | 686.35 µs | 2.1786 ms | 765778 | 311715 | 264630 | 4.1210 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0486 ms | 4.8260 ms | 1407835 | 403440 | 324081 | 5.0796 ms |
| [serde_json 1.0.134][serde_json] | 3.8738 ms | 5.9795 ms | 1827461 | 470560 | 361090 | 6.0497 ms |
| [simd-json 0.14.3][simd-json] | 2.1214 ms | 4.6893 ms | 1827461 | 470560 | 361090 | 6.0496 ms |
| [speedy 0.8.7][speedy] | 203.16 µs | 1.8006 ms | 885780 | 362204 | 286514 | 4.5017 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*71.959 ns\**</span> | <span title="validated on-demand with error">*167.96 µs\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4734 ns\**</span> <span title="validated upfront with error">*2.0462 ms\**</span> | <span title="unvalidated">*52.002 µs\**</span> <span title="validated upfront with error">*2.0185 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2368 ns\**</span> <span title="validated upfront with error">*582.96 µs\**</span> | <span title="unvalidated">*10.515 µs\**</span> <span title="validated upfront with error">*584.52 µs\**</span> | <span title="unvalidated">*7.5826 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*20.56%\**</span> <span title="prepend">*23.54%\**</span> | 44.97% | 80.46% | 81.26% | 73.70% | 46.49% |
| [bincode 2.0.0-rc][bincode] | 44.76% | 58.81% | 94.93% | 95.03% | 89.35% | 65.34% |
| [bincode 1.3.3][bincode1] | 27.82% | 69.84% | 67.29% | 77.41% | 73.70% | 53.28% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 26.57% | 67.25% | 79.45% | 79.74% | 80.19% | 57.83% |
| [capnp 0.20.3][capnp] | 27.48% | † | 48.76% | 56.19% | 53.60% | 38.25% |
| [cbor4ii 0.3.3][cbor4ii] | 24.64% | 29.89% | 49.99% | 71.59% | 70.89% | 51.19% |
| [ciborium 0.2.2][ciborium] | 3.54% | 12.21% | 49.99% | 71.59% | 70.89% | 51.65% |
| [databuf 0.5.0][databuf] | 56.84% | 68.32% | 91.89% | 92.66% | 86.82% | 62.68% |
| [dlhn 0.1.7][dlhn] | 19.12% | 56.25% | 97.07% | 95.81% | 90.59% | 68.56% |
| [flatbuffers 24.12.23][flatbuffers] | 14.29% | † | 55.13% | 61.64% | 59.09% | 47.28% |
| [msgpacker 0.4.5][msgpacker] | 11.49% | 56.22% | 91.99% | 91.61% | 86.73% | 62.09% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.74% | 35.18% | 85.96% | 86.85% | 80.47% | 55.38% |
| [nanoserde 0.1.37][nanoserde] | 56.48% | 69.24% | 67.29% | 77.41% | 73.70% | 54.98% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.96% | 63.76% | 91.89% | 92.65% | 86.86% | 62.71% |
| [postcard 1.1.1][postcard] | 33.95% | 66.37% | 97.07% | 95.51% | 90.54% | 68.51% |
| [pot 3.0.1][pot] | 6.40% | 22.79% | 72.40% | 77.53% | 75.55% | 47.82% |
| [prost 0.13.4][prost] | <span title="encode">*15.23%\**</span> <span title="populate + encode">*5.81%\**</span> | 44.48% | 79.55% | 79.54% | 72.82% | 50.84% |
| [rkyv 0.8.9][rkyv] | 55.48% | <span title="unvalidated">*92.43%\**</span> <span title="validated upfront with error">*67.52%\**</span> | 69.57% | 73.39% | 70.37% | 49.93% |
| [rmp-serde 1.3.0][rmp-serde] | 9.74% | 43.86% | 89.64% | 88.76% | 82.58% | 59.99% |
| [ron 0.8.1][ron] | 1.28% | 9.67% | 43.78% | 64.30% | 65.70% | 43.07% |
| [savefile 0.18.5][savefile] | 76.08% | 65.35% | 67.29% | 77.40% | 73.70% | 53.88% |
| [serde-brief 0.1.0][serde-brief] | 9.69% | 29.49% | 44.40% | 69.81% | 67.29% | 49.89% |
| [serde_bare 0.5.0][serde_bare] | 21.17% | 67.99% | 91.89% | 92.66% | 86.82% | 63.63% |
| [serde_cbor 0.11.2][serde_cbor] | 7.09% | 30.69% | 49.99% | 71.59% | 70.89% | 51.62% |
| [serde_json 1.0.134][serde_json] | 3.75% | 24.77% | 38.51% | 61.38% | 63.63% | 43.34% |
| [simd-json 0.14.3][simd-json] | 6.85% | 31.59% | 38.51% | 61.38% | 63.63% | 43.34% |
| [speedy 0.8.7][speedy] | 71.52% | 82.27% | 79.45% | 79.74% | 80.19% | 58.25% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*6.26%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.22%\**</span> <span title="validated upfront with error">*0.52%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.80%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*6.6519 ms\**</span> <span title="prepend">*8.8697 ms\**</span> | 9.1158 ms | 8625005 | 6443961 | 6231572 | 77.328 ms |
| [bincode 2.0.0-rc][bincode] | 2.8741 ms | 1.0202 ms | 6000005 | 5378497 | 5345897 | 7.6421 ms |
| [bincode 1.3.3][bincode1] | 5.1587 ms | 6.1548 ms | 6000008 | 5378500 | 5345890 | 7.7636 ms |
| [bitcode 0.6.3][bitcode] | 1.4330 ms | 792.22 µs | 6000006 | 5182295 | 4923880 | 14.700 ms |
| [borsh 1.5.3][borsh] | 6.2352 ms | 4.7872 ms | 6000004 | 5378496 | 5345889 | 7.9011 ms |
| [capnp 0.20.3][capnp] | 7.0329 ms | † | 14000088 | 7130367 | 6051062 | 83.795 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.8699 ms | 52.859 ms | 13125016 | 7524114 | 6757967 | 91.822 ms |
| [ciborium 0.2.2][ciborium] | 69.803 ms | 121.49 ms | 13122324 | 7524660 | 6759658 | 91.644 ms |
| [databuf 0.5.0][databuf] | 2.3965 ms | 5.2761 ms | 6000003 | 5378495 | 5345900 | 7.8517 ms |
| [dlhn 0.1.7][dlhn] | 6.1723 ms | 6.9908 ms | 6000003 | 5378495 | 5345900 | 7.9965 ms |
| [flatbuffers 24.12.23][flatbuffers] | 876.20 µs | † | 6000024 | 5378434 | 5345910 | 7.3575 ms |
| [msgpacker 0.4.5][msgpacker] | 18.767 ms | 5.2130 ms | 7500005 | 6058442 | 6014337 | 9.7495 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.75 ms | 32.660 ms | 8125037 | 6493484 | 6386940 | 72.461 ms |
| [nanoserde 0.1.37][nanoserde] | 2.1329 ms | 1.1019 ms | 6000008 | 5378500 | 5345890 | 7.8717 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.1209 ms | 4.5401 ms | 6000004 | 5378496 | 5345889 | 7.3546 ms |
| [postcard 1.1.1][postcard] | 478.29 µs | 1.8012 ms | 6000003 | 5378495 | 5345900 | 7.8547 ms |
| [pot 3.0.1][pot] | 36.775 ms | 68.783 ms | 10122342 | 6814618 | 6852251 | 80.961 ms |
| [prost 0.13.4][prost] | <span title="encode">*7.7495 ms\**</span> <span title="populate + encode">*8.5337 ms\**</span> | 14.615 ms | 8750000 | 6665735 | 6421871 | 71.520 ms |
| [rkyv 0.8.9][rkyv] | 237.44 µs | <span title="unvalidated">*204.66 µs\**</span> <span title="validated upfront with error">*201.97 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.3320 ms |
| [rmp-serde 1.3.0][rmp-serde] | 18.299 ms | 18.353 ms | 8125006 | 6494876 | 6391037 | 68.408 ms |
| [ron 0.8.1][ron] | 173.55 ms | 232.05 ms | 22192885 | 8970395 | 8138755 | 151.61 ms |
| [savefile 0.18.5][savefile] | 237.37 µs | 238.22 µs | 6000024 | 5378519 | 5345892 | 7.3950 ms |
| [serde-brief 0.1.0][serde-brief] | 22.840 ms | 41.288 ms | 15750015 | 8024540 | 6816643 | 95.687 ms |
| [serde_bare 0.5.0][serde_bare] | 6.6291 ms | 4.6975 ms | 6000003 | 5378495 | 5345900 | 8.1701 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.654 ms | 47.359 ms | 13122324 | 7524660 | 6759658 | 90.580 ms |
| [serde_json 1.0.134][serde_json] | 87.594 ms | 89.307 ms | 26192883 | 9566084 | 8586741 | 156.42 ms |
| [simd-json 0.14.3][simd-json] | 52.107 ms | 70.431 ms | 26192883 | 9566084 | 8586741 | 156.37 ms |
| [speedy 0.8.7][speedy] | 237.28 µs | 238.29 µs | 6000004 | 5378496 | 5345889 | 7.3144 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*103.55 ns\**</span> | <span title="validated on-demand with error">*2.1402 ms\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4733 ns\**</span> <span title="validated upfront with error">*40.066 ns\**</span> | <span title="unvalidated">*54.002 µs\**</span> <span title="validated upfront with error">*77.395 µs\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2362 ns\**</span> <span title="validated upfront with error">*4.9575 ns\**</span> | <span title="unvalidated">*48.409 µs\**</span> <span title="validated upfront with error">*38.738 µs\**</span> | <span title="unvalidated">*101.85 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*3.57%\**</span> <span title="prepend">*2.68%\**</span> | 2.22% | 69.57% | 80.42% | 79.02% | 9.46% |
| [bincode 2.0.0-rc][bincode] | 8.26% | 19.80% | 100.00% | 96.35% | 92.11% | 95.71% |
| [bincode 1.3.3][bincode1] | 4.60% | 3.28% | 100.00% | 96.35% | 92.11% | 94.21% |
| [bitcode 0.6.3][bitcode] | 16.56% | 25.49% | 100.00% | 100.00% | 100.00% | 49.76% |
| [borsh 1.5.3][borsh] | 3.81% | 4.22% | 100.00% | 96.35% | 92.11% | 92.57% |
| [capnp 0.20.3][capnp] | 3.37% | † | 42.86% | 72.68% | 81.37% | 8.73% |
| [cbor4ii 0.3.3][cbor4ii] | 2.40% | 0.38% | 45.71% | 68.88% | 72.86% | 7.97% |
| [ciborium 0.2.2][ciborium] | 0.34% | 0.17% | 45.72% | 68.87% | 72.84% | 7.98% |
| [databuf 0.5.0][databuf] | 9.90% | 3.83% | 100.00% | 96.35% | 92.11% | 93.16% |
| [dlhn 0.1.7][dlhn] | 3.84% | 2.89% | 100.00% | 96.35% | 92.11% | 91.47% |
| [flatbuffers 24.12.23][flatbuffers] | 27.08% | † | 100.00% | 96.35% | 92.11% | 99.41% |
| [msgpacker 0.4.5][msgpacker] | 1.26% | 3.87% | 80.00% | 85.54% | 81.87% | 75.02% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.19% | 0.62% | 73.85% | 79.81% | 77.09% | 10.09% |
| [nanoserde 0.1.37][nanoserde] | 11.12% | 18.33% | 100.00% | 96.35% | 92.11% | 92.92% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 4.63% | 4.45% | 100.00% | 96.35% | 92.11% | 99.45% |
| [postcard 1.1.1][postcard] | 49.61% | 11.21% | 100.00% | 96.35% | 92.11% | 93.12% |
| [pot 3.0.1][pot] | 0.65% | 0.29% | 59.27% | 76.05% | 71.86% | 9.03% |
| [prost 0.13.4][prost] | <span title="encode">*3.06%\**</span> <span title="populate + encode">*2.78%\**</span> | 1.38% | 68.57% | 77.75% | 76.67% | 10.23% |
| [rkyv 0.8.9][rkyv] | 99.93% | <span title="unvalidated">*98.69%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 99.76% |
| [rmp-serde 1.3.0][rmp-serde] | 1.30% | 1.10% | 73.85% | 79.79% | 77.04% | 10.69% |
| [ron 0.8.1][ron] | 0.14% | 0.09% | 27.04% | 57.77% | 60.50% | 4.82% |
| [savefile 0.18.5][savefile] | 99.96% | 84.78% | 100.00% | 96.35% | 92.11% | 98.91% |
| [serde-brief 0.1.0][serde-brief] | 1.04% | 0.49% | 38.10% | 64.58% | 72.23% | 7.64% |
| [serde_bare 0.5.0][serde_bare] | 3.58% | 4.30% | 100.00% | 96.35% | 92.11% | 89.53% |
| [serde_cbor 0.11.2][serde_cbor] | 0.67% | 0.43% | 45.72% | 68.87% | 72.84% | 8.08% |
| [serde_json 1.0.134][serde_json] | 0.27% | 0.23% | 22.91% | 54.17% | 57.34% | 4.68% |
| [simd-json 0.14.3][simd-json] | 0.46% | 0.29% | 22.91% | 54.17% | 57.34% | 4.68% |
| [speedy 0.8.7][speedy] | 100.00% | 84.76% | 100.00% | 96.35% | 92.11% | 100.00% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.19%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*3.09%\**</span> | <span title="unvalidated">*71.73%\**</span> <span title="validated upfront with error">*50.05%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.94%\**</span> | <span title="unvalidated">*80.02%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*937.35 µs\**</span> <span title="prepend">*831.38 µs\**</span> | 3.1782 ms | 489348 | 281173 | 249546 | 3.0990 ms |
| [bincode 2.0.0-rc][bincode] | 311.35 µs | 2.0987 ms | 367413 | 221291 | 206273 | 2.5016 ms |
| [bincode 1.3.3][bincode1] | 591.29 µs | 1.8371 ms | 569975 | 240525 | 232423 | 2.8811 ms |
| [bitcode 0.6.3][bitcode] | 130.12 µs | 1.2711 ms | 327688 | 200947 | 182736 | 744.58 µs |
| [borsh 1.5.3][borsh] | 528.52 µs | 1.8434 ms | 446595 | 234236 | 210008 | 2.4669 ms |
| [capnp 0.20.3][capnp] | 494.14 µs | † | 803896 | 335606 | 280851 | 3.9459 ms |
| [cbor4ii 0.3.3][cbor4ii] | 794.69 µs | 4.6858 ms | 1109831 | 344745 | 274514 | 3.8322 ms |
| [ciborium 0.2.2][ciborium] | 3.6777 ms | 10.002 ms | 1109821 | 344751 | 274526 | 3.8682 ms |
| [databuf 0.5.0][databuf] | 305.19 µs | 1.7391 ms | 356311 | 213062 | 198488 | 2.3867 ms |
| [dlhn 0.1.7][dlhn] | 777.41 µs | 2.6586 ms | 366496 | 220600 | 205683 | 2.4869 ms |
| [flatbuffers 24.12.23][flatbuffers] | 3.2680 ms | † | 844168 | 345696 | 294015 | 3.8623 ms |
| [msgpacker 0.4.5][msgpacker] | 958.39 µs | 2.8770 ms | 391251 | 236877 | 220476 | 2.6256 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0725 ms | 3.9794 ms | 449745 | 252432 | 231110 | 2.7771 ms |
| [nanoserde 0.1.37][nanoserde] | 278.87 µs | 1.9306 ms | 567975 | 239930 | 232419 | 2.9293 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 615.29 µs | 1.9809 ms | 356311 | 212976 | 198524 | 2.4458 ms |
| [postcard 1.1.1][postcard] | 441.53 µs | 2.0025 ms | 367489 | 221913 | 207344 | 2.4895 ms |
| [pot 3.0.1][pot] | 2.3593 ms | 5.9633 ms | 599125 | 299158 | 247693 | 3.1897 ms |
| [prost 0.13.4][prost] | <span title="encode">*1.2480 ms\**</span> <span title="populate + encode">*2.9430 ms\**</span> | 3.5357 ms | 596811 | 305319 | 269310 | 3.4836 ms |
| [rkyv 0.8.9][rkyv] | 334.35 µs | <span title="unvalidated">*1.5025 ms\**</span> <span title="validated upfront with error">*2.0051 ms\**</span> | 603776 | 254776 | 220087 | 2.7221 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5291 ms | 3.0620 ms | 424533 | 245214 | 226188 | 2.6957 ms |
| [ron 0.8.1][ron] | 7.4316 ms | 16.796 ms | 1465223 | 434935 | 343338 | 6.0148 ms |
| [savefile 0.18.5][savefile] | 216.01 µs | 1.8976 ms | 566991 | 239362 | 232010 | 2.8830 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3314 ms | 5.4039 ms | 1276014 | 373898 | 293679 | 4.0561 ms |
| [serde_bare 0.5.0][serde_bare] | 739.63 µs | 2.3376 ms | 356311 | 213062 | 198488 | 2.3878 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8702 ms | 4.6533 ms | 1109821 | 344751 | 274526 | 3.8591 ms |
| [serde_json 1.0.134][serde_json] | 3.6634 ms | 6.6993 ms | 1623191 | 466527 | 359623 | 6.1012 ms |
| [simd-json 0.14.3][simd-json] | 2.2252 ms | 4.6059 ms | 1623191 | 466527 | 359623 | 6.1941 ms |
| [speedy 0.8.7][speedy] | 260.10 µs | 1.6065 ms | 449595 | 234970 | 210361 | 2.4971 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*72.140 ns\**</span> | <span title="validated on-demand with error">*568.56 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4740 ns\**</span> <span title="validated upfront with error">*2.2006 ms\**</span> | <span title="unvalidated">*1.3485 µs\**</span> <span title="validated upfront with error">*2.2160 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2361 ns\**</span> <span title="validated upfront with error">*527.82 µs\**</span> | <span title="unvalidated">*240.39 ns\**</span> <span title="validated upfront with error">*536.13 µs\**</span> | <span title="unvalidated">*756.00 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*13.88%\**</span> <span title="prepend">*15.65%\**</span> | 39.99% | 66.96% | 71.47% | 73.23% | 24.03% |
| [bincode 2.0.0-rc][bincode] | 41.79% | 60.57% | 89.19% | 90.81% | 88.59% | 29.76% |
| [bincode 1.3.3][bincode1] | 22.01% | 69.19% | 57.49% | 83.55% | 78.62% | 25.84% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.62% | 68.95% | 73.37% | 85.79% | 87.01% | 30.18% |
| [capnp 0.20.3][capnp] | 26.33% | † | 40.76% | 59.88% | 65.07% | 18.87% |
| [cbor4ii 0.3.3][cbor4ii] | 16.37% | 27.13% | 29.53% | 58.29% | 66.57% | 19.43% |
| [ciborium 0.2.2][ciborium] | 3.54% | 12.71% | 29.53% | 58.29% | 66.56% | 19.25% |
| [databuf 0.5.0][databuf] | 42.64% | 73.09% | 91.97% | 94.31% | 92.06% | 31.20% |
| [dlhn 0.1.7][dlhn] | 16.74% | 47.81% | 89.41% | 91.09% | 88.84% | 29.94% |
| [flatbuffers 24.12.23][flatbuffers] | 3.98% | † | 38.82% | 58.13% | 62.15% | 19.28% |
| [msgpacker 0.4.5][msgpacker] | 13.58% | 44.18% | 83.75% | 84.83% | 82.88% | 28.36% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.57% | 31.94% | 72.86% | 79.60% | 79.07% | 26.81% |
| [nanoserde 0.1.37][nanoserde] | 46.66% | 65.84% | 57.69% | 83.75% | 78.62% | 25.42% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.15% | 64.17% | 91.97% | 94.35% | 92.05% | 30.44% |
| [postcard 1.1.1][postcard] | 29.47% | 63.48% | 89.17% | 90.55% | 88.13% | 29.91% |
| [pot 3.0.1][pot] | 5.52% | 21.32% | 54.69% | 67.17% | 73.78% | 23.34% |
| [prost 0.13.4][prost] | <span title="encode">*10.43%\**</span> <span title="populate + encode">*4.42%\**</span> | 35.95% | 54.91% | 65.82% | 67.85% | 21.37% |
| [rkyv 0.8.9][rkyv] | 38.92% | <span title="unvalidated">*84.60%\**</span> <span title="validated upfront with error">*63.39%\**</span> | 54.27% | 78.87% | 83.03% | 27.35% |
| [rmp-serde 1.3.0][rmp-serde] | 8.51% | 41.51% | 77.19% | 81.95% | 80.79% | 27.62% |
| [ron 0.8.1][ron] | 1.75% | 7.57% | 22.36% | 46.20% | 53.22% | 12.38% |
| [savefile 0.18.5][savefile] | 60.24% | 66.98% | 57.79% | 83.95% | 78.76% | 25.83% |
| [serde-brief 0.1.0][serde-brief] | 9.77% | 23.52% | 25.68% | 53.74% | 62.22% | 18.36% |
| [serde_bare 0.5.0][serde_bare] | 17.59% | 54.38% | 91.97% | 94.31% | 92.06% | 31.18% |
| [serde_cbor 0.11.2][serde_cbor] | 6.96% | 27.32% | 29.53% | 58.29% | 66.56% | 19.29% |
| [serde_json 1.0.134][serde_json] | 3.55% | 18.97% | 20.19% | 43.07% | 50.81% | 12.20% |
| [simd-json 0.14.3][simd-json] | 5.85% | 27.60% | 20.19% | 43.07% | 50.81% | 12.02% |
| [speedy 0.8.7][speedy] | 50.03% | 79.12% | 72.89% | 85.52% | 86.87% | 29.82% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*42.28%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.83%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.04%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*4.5392 ms\**</span> <span title="prepend">*2.5830 ms\**</span> | 8.3454 ms | 1704643 | 1294259 | 1245607 | 11.190 ms |
| [bincode 2.0.0-rc][bincode] | 1.4074 ms | 3.7901 ms | 1406257 | 1117802 | 1062238 | 9.2003 ms |
| [bincode 1.3.3][bincode1] | 3.9651 ms | 4.3945 ms | 1854234 | 1141994 | 1050351 | 10.202 ms |
| [bitcode 0.6.3][bitcode] | 711.82 µs | 2.3679 ms | 971318 | 878034 | 855922 | 3.3927 ms |
| [borsh 1.5.3][borsh] | 2.8930 ms | 2.8428 ms | 1521989 | 1108471 | 1038408 | 9.7372 ms |
| [capnp 0.20.3][capnp] | 2.1940 ms | † | 2724288 | 1546992 | 1240354 | 15.053 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.2782 ms | 17.719 ms | 6012539 | 1695215 | 1467194 | 21.385 ms |
| [ciborium 0.2.2][ciborium] | 23.445 ms | 54.233 ms | 6012373 | 1695146 | 1467435 | 21.459 ms |
| [databuf 0.5.0][databuf] | 1.2907 ms | 3.8139 ms | 1319999 | 1062631 | 1007898 | 8.7548 ms |
| [dlhn 0.1.7][dlhn] | 5.0193 ms | 8.5484 ms | 1311281 | 1077520 | 1045571 | 8.5799 ms |
| [flatbuffers 24.12.23][flatbuffers] | 5.1134 ms | † | 2325620 | 1440289 | 1265148 | 13.309 ms |
| [msgpacker 0.4.5][msgpacker] | 2.3282 ms | 6.9956 ms | 1458773 | 1156055 | 1137194 | 9.6667 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.056 ms | 17.809 ms | 1770060 | 1277755 | 1263142 | 12.202 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3491 ms | 2.9915 ms | 1812404 | 1134820 | 1054758 | 10.086 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 3.1421 ms | 3.2958 ms | 1319999 | 1064380 | 1010284 | 8.8405 ms |
| [postcard 1.1.1][postcard] | 1.9884 ms | 4.3336 ms | 1311281 | 1083900 | 1041114 | 8.6424 ms |
| [pot 3.0.1][pot] | 13.406 ms | 30.076 ms | 2604812 | 1482233 | 1299952 | 15.733 ms |
| [prost 0.13.4][prost] | <span title="encode">*5.4148 ms\**</span> <span title="populate + encode">*9.4298 ms\**</span> | 8.7034 ms | 1859886 | 1338076 | 1295497 | 12.117 ms |
| [rkyv 0.8.9][rkyv] | 1.0222 ms | <span title="unvalidated">*2.2018 ms\**</span> <span title="validated upfront with error">*2.6304 ms\**</span> | 2075936 | 1383779 | 1211892 | 12.609 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.106 ms | 11.180 ms | 1745322 | 1261627 | 1228902 | 11.232 ms |
| [ron 0.8.1][ron] | 38.052 ms | 90.966 ms | 8677703 | 2233642 | 1827843 | 34.553 ms |
| [savefile 0.18.5][savefile] | 839.37 µs | 2.8262 ms | 1791505 | 1128012 | 1052757 | 10.164 ms |
| [serde-brief 0.1.0][serde-brief] | 6.5334 ms | 22.240 ms | 6951772 | 1796265 | 1570903 | 23.617 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9511 ms | 4.9270 ms | 1319999 | 1062645 | 1007918 | 8.9053 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.505 ms | 20.992 ms | 6012373 | 1695146 | 1467435 | 21.328 ms |
| [serde_json 1.0.134][serde_json] | 20.349 ms | 30.774 ms | 9390461 | 2391679 | 1843922 | 34.299 ms |
| [simd-json 0.14.3][simd-json] | 11.989 ms | 26.277 ms | 9390461 | 2391679 | 1843922 | 34.296 ms |
| [speedy 0.8.7][speedy] | 772.42 µs | 2.4938 ms | 1584734 | 1119837 | 1038012 | 10.051 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*72.116 ns\**</span> | <span title="validated on-demand with error">*706.01 ns\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*2.4740 ns\**</span> <span title="validated upfront with error">*5.2432 ms\**</span> | <span title="unvalidated">*2.6208 µs\**</span> <span title="validated upfront with error">*5.2783 ms\**</span> | ‡ |
| [rkyv 0.8.9][rkyv] | <span title="unvalidated">*1.2365 ns\**</span> <span title="validated upfront with error">*421.49 µs\**</span> | <span title="unvalidated">*435.02 ns\**</span> <span title="validated upfront with error">*422.10 µs\**</span> | <span title="unvalidated">*235.29 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1011.0][bilrost] | <span title="encode">*15.68%\**</span> <span title="prepend">*27.56%\**</span> | 26.38% | 56.98% | 67.84% | 68.72% | 30.32% |
| [bincode 2.0.0-rc][bincode] | 50.58% | 58.09% | 69.07% | 78.55% | 80.58% | 36.88% |
| [bincode 1.3.3][bincode1] | 17.95% | 50.10% | 52.38% | 76.89% | 81.49% | 33.26% |
| [bitcode 0.6.3][bitcode] | 100.00% | 92.99% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.3][borsh] | 24.60% | 77.45% | 63.82% | 79.21% | 82.43% | 34.84% |
| [capnp 0.20.3][capnp] | 32.44% | † | 35.65% | 56.76% | 69.01% | 22.54% |
| [cbor4ii 0.3.3][cbor4ii] | 21.71% | 12.43% | 16.15% | 51.79% | 58.34% | 15.86% |
| [ciborium 0.2.2][ciborium] | 3.04% | 4.06% | 16.16% | 51.80% | 58.33% | 15.81% |
| [databuf 0.5.0][databuf] | 55.15% | 57.73% | 73.58% | 82.63% | 84.92% | 38.75% |
| [dlhn 0.1.7][dlhn] | 14.18% | 25.76% | 74.07% | 81.49% | 81.86% | 39.54% |
| [flatbuffers 24.12.23][flatbuffers] | 13.92% | † | 41.77% | 60.96% | 67.65% | 25.49% |
| [msgpacker 0.4.5][msgpacker] | 30.57% | 31.47% | 66.58% | 75.95% | 75.27% | 35.10% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.37% | 12.36% | 54.87% | 68.72% | 67.76% | 27.80% |
| [nanoserde 0.1.37][nanoserde] | 52.76% | 73.60% | 53.59% | 77.37% | 81.15% | 33.64% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.65% | 66.81% | 73.58% | 82.49% | 84.72% | 38.38% |
| [postcard 1.1.1][postcard] | 35.80% | 50.81% | 74.07% | 81.01% | 82.21% | 39.26% |
| [pot 3.0.1][pot] | 5.31% | 7.32% | 37.29% | 59.24% | 65.84% | 21.56% |
| [prost 0.13.4][prost] | <span title="encode">*13.15%\**</span> <span title="populate + encode">*7.55%\**</span> | 25.30% | 52.22% | 65.62% | 66.07% | 28.00% |
| [rkyv 0.8.9][rkyv] | 69.64% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.71%\**</span> | 46.79% | 63.45% | 70.63% | 26.91% |
| [rmp-serde 1.3.0][rmp-serde] | 7.04% | 19.69% | 55.65% | 69.60% | 69.65% | 30.20% |
| [ron 0.8.1][ron] | 1.87% | 2.42% | 11.19% | 39.31% | 46.83% | 9.82% |
| [savefile 0.18.5][savefile] | 84.80% | 77.91% | 54.22% | 77.84% | 81.30% | 33.38% |
| [serde-brief 0.1.0][serde-brief] | 10.90% | 9.90% | 13.97% | 48.88% | 54.49% | 14.37% |
| [serde_bare 0.5.0][serde_bare] | 14.38% | 44.69% | 73.58% | 82.63% | 84.92% | 38.10% |
| [serde_cbor 0.11.2][serde_cbor] | 6.78% | 10.49% | 16.16% | 51.80% | 58.33% | 15.91% |
| [serde_json 1.0.134][serde_json] | 3.50% | 7.15% | 10.34% | 36.71% | 46.42% | 9.89% |
| [simd-json 0.14.3][simd-json] | 5.94% | 8.38% | 10.34% | 36.71% | 46.42% | 9.89% |
| [speedy 0.8.7][speedy] | 92.15% | 88.29% | 61.29% | 78.41% | 82.46% | 33.75% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.3][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*61.62%\**</span> | ‡ |
| [flatbuffers 24.12.23][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.60%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
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
[serde_json]: https://crates.io/crates/serde_json/1.0.134
[simd-json]: https://crates.io/crates/simd-json/0.14.3
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
