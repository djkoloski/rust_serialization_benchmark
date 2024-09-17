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

## Last updated: 2024-9-17 2:50:58

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.83.0-nightly (c52c23b6f 2024-09-16)
binary: rustc
commit-hash: c52c23b6f44cd19718721a5e3b2eeb169e9c96ff
commit-date: 2024-09-16
host: x86_64-unknown-linux-gnu
release: 1.83.0-nightly
LLVM version: 19.1.0
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
| [bilrost 0.1010.0][bilrost] | <span title="encode">*696.30 µs\**</span> <span title="prepend">*627.02 µs\**</span> | 3.2225 ms | 874632 | 355446 | 311723 | 5.9176 ms |
| [bincode 2.0.0-rc][bincode] | 327.62 µs | 2.7430 ms | 741295 | 303944 | 257153 | 3.9956 ms |
| [bincode 1.3.3][bincode1] | 522.15 µs | 2.3722 ms | 1045784 | 373127 | 311761 | 4.9219 ms |
| [bitcode 0.6.3][bitcode] | 146.14 µs | 1.4838 ms | 703710 | 288826 | 229755 | 2.4565 ms |
| [borsh 1.5.1][borsh] | 545.86 µs | 2.1871 ms | 885780 | 362204 | 286514 | 4.5638 ms |
| [capnp 0.19.7][capnp] | 478.98 µs | † | 1443216 | 513986 | 428649 | 6.8509 ms |
| [cbor4ii 0.3.3][cbor4ii] | 595.77 µs | 5.2779 ms | 1407835 | 403440 | 324081 | 4.9208 ms |
| [ciborium 0.2.2][ciborium] | 4.3400 ms | 12.113 ms | 1407835 | 403440 | 324081 | 4.8952 ms |
| [databuf 0.5.0][databuf] | 255.15 µs | 2.0632 ms | 765778 | 311715 | 264630 | 4.1894 ms |
| [dlhn 0.1.7][dlhn] | 711.81 µs | 2.8752 ms | 724953 | 301446 | 253629 | 3.8404 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0259 ms | † | 1276368 | 468539 | 388832 | 5.5752 ms |
| [msgpacker 0.4.3][msgpacker] | 943.65 µs | 2.7212 ms | 764996 | 315291 | 264898 | 4.2210 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.8087 ms | 4.5676 ms | 818669 | 332556 | 285514 | 4.6654 ms |
| [nanoserde 0.1.37][nanoserde] | 261.02 µs | 2.0712 ms | 1045784 | 373127 | 311761 | 4.5550 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 657.11 µs | 2.2587 ms | 765778 | 311743 | 264518 | 3.9943 ms |
| [postcard 1.0.10][postcard] | 398.35 µs | 2.1727 ms | 724953 | 302399 | 253747 | 3.6706 ms |
| [pot 3.0.1][pot] | 2.3759 ms | 6.8393 ms | 971922 | 372513 | 304122 | 5.0875 ms |
| [prost 0.13.2][prost] | <span title="encode">*951.17 µs\**</span> <span title="populate + encode">*2.5158 ms\**</span> | 3.3263 ms | 884628 | 363130 | 315494 | 4.9086 ms |
| [rkyv 0.8.5][rkyv] | 243.10 µs | <span title="unvalidated">*1.5883 ms\**</span> <span title="validated upfront with error">*2.1743 ms\**</span> | 1011488 | 393526 | 326517 | 5.3054 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3409 ms | 3.1621 ms | 784997 | 325384 | 278219 | 4.2725 ms |
| [ron 0.8.1][ron] | 11.389 ms | 14.878 ms | 1607459 | 449158 | 349713 | 5.7919 ms |
| [savefile 0.17.7][savefile] | 189.56 µs | 2.8373 ms | 1045800 | 373140 | 311777 | 4.5761 ms |
| [serde_bare 0.5.0][serde_bare] | 694.93 µs | 2.1020 ms | 765778 | 311715 | 264630 | 3.8792 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9622 ms | 4.9327 ms | 1407835 | 403440 | 324081 | 4.8214 ms |
| [serde_json 1.0.128][serde_json] | 3.8112 ms | 5.5805 ms | 1827461 | 470560 | 361090 | 5.6848 ms |
| [simd-json 0.13.10][simd-json] | 2.1634 ms | 4.6308 ms | 1827461 | 470560 | 361090 | 5.6499 ms |
| [speedy 0.8.7][speedy] | 204.20 µs | 1.7854 ms | 885780 | 362204 | 286514 | 4.2299 ms |
| [wiring 0.2.2][wiring] | 190.75 µs | 1.9875 ms | 1045784 | 337930 | 276188 | 3.9679 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.554 ns\**</span> | <span title="validated on-demand with error">*167.55 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4730 ns\**</span> <span title="validated upfront with error">*1.9811 ms\**</span> | <span title="unvalidated">*51.742 µs\**</span> <span title="validated upfront with error">*2.0308 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*591.11 µs\**</span> | <span title="unvalidated">*10.332 µs\**</span> <span title="validated upfront with error">*601.85 µs\**</span> | <span title="unvalidated">*7.7403 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*20.99%\**</span> <span title="prepend">*23.31%\**</span> | 46.04% | 80.46% | 81.26% | 73.70% | 41.51% |
| [bincode 2.0.0-rc][bincode] | 44.61% | 54.09% | 94.93% | 95.03% | 89.35% | 61.48% |
| [bincode 1.3.3][bincode1] | 27.99% | 62.55% | 67.29% | 77.41% | 73.70% | 49.91% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 26.77% | 67.84% | 79.45% | 79.74% | 80.19% | 53.83% |
| [capnp 0.19.7][capnp] | 30.51% | † | 48.76% | 56.19% | 53.60% | 35.86% |
| [cbor4ii 0.3.3][cbor4ii] | 24.53% | 28.11% | 49.99% | 71.59% | 70.89% | 49.92% |
| [ciborium 0.2.2][ciborium] | 3.37% | 12.25% | 49.99% | 71.59% | 70.89% | 50.18% |
| [databuf 0.5.0][databuf] | 57.28% | 71.92% | 91.89% | 92.66% | 86.82% | 58.64% |
| [dlhn 0.1.7][dlhn] | 20.53% | 51.61% | 97.07% | 95.81% | 90.59% | 63.96% |
| [flatbuffers 24.3.25][flatbuffers] | 14.25% | † | 55.13% | 61.64% | 59.09% | 44.06% |
| [msgpacker 0.4.3][msgpacker] | 15.49% | 54.53% | 91.99% | 91.61% | 86.73% | 58.20% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.52% | 32.49% | 85.96% | 86.85% | 80.47% | 52.65% |
| [nanoserde 0.1.37][nanoserde] | 55.99% | 71.64% | 67.29% | 77.41% | 73.70% | 53.93% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.24% | 65.69% | 91.89% | 92.65% | 86.86% | 61.50% |
| [postcard 1.0.10][postcard] | 36.69% | 68.29% | 97.07% | 95.51% | 90.54% | 66.92% |
| [pot 3.0.1][pot] | 6.15% | 21.70% | 72.40% | 77.53% | 75.55% | 48.29% |
| [prost 0.13.2][prost] | <span title="encode">*15.36%\**</span> <span title="populate + encode">*5.81%\**</span> | 44.61% | 79.55% | 79.54% | 72.82% | 50.04% |
| [rkyv 0.8.5][rkyv] | 60.12% | <span title="unvalidated">*93.42%\**</span> <span title="validated upfront with error">*68.24%\**</span> | 69.57% | 73.39% | 70.37% | 46.30% |
| [rmp-serde 1.3.0][rmp-serde] | 10.90% | 46.92% | 89.64% | 88.76% | 82.58% | 57.50% |
| [ron 0.8.1][ron] | 1.28% | 9.97% | 43.78% | 64.30% | 65.70% | 42.41% |
| [savefile 0.17.7][savefile] | 77.09% | 52.30% | 67.29% | 77.40% | 73.69% | 53.68% |
| [serde_bare 0.5.0][serde_bare] | 21.03% | 70.59% | 91.89% | 92.66% | 86.82% | 63.32% |
| [serde_cbor 0.11.2][serde_cbor] | 7.45% | 30.08% | 49.99% | 71.59% | 70.89% | 50.95% |
| [serde_json 1.0.128][serde_json] | 3.83% | 26.59% | 38.51% | 61.38% | 63.63% | 43.21% |
| [simd-json 0.13.10][simd-json] | 6.76% | 32.04% | 38.51% | 61.38% | 63.63% | 43.48% |
| [speedy 0.8.7][speedy] | 71.57% | 83.11% | 79.45% | 79.74% | 80.19% | 58.07% |
| [wiring 0.2.2][wiring] | 76.61% | 74.66% | 67.29% | 85.47% | 83.19% | 61.91% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*6.17%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*19.97%\**</span> <span title="validated upfront with error">*0.51%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.72%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6861 ms\**</span> <span title="prepend">*8.7410 ms\**</span> | 9.1053 ms | 8625005 | 6443961 | 6231572 | 70.340 ms |
| [bincode 2.0.0-rc][bincode] | 2.3960 ms | 1.0187 ms | 6000005 | 5378497 | 5345897 | 7.4417 ms |
| [bincode 1.3.3][bincode1] | 5.1944 ms | 4.8484 ms | 6000008 | 5378500 | 5345890 | 7.6828 ms |
| [bitcode 0.6.3][bitcode] | 1.4137 ms | 810.24 µs | 6000006 | 5182295 | 4923880 | 12.973 ms |
| [borsh 1.5.1][borsh] | 6.0069 ms | 4.2985 ms | 6000004 | 5378496 | 5345889 | 7.4086 ms |
| [capnp 0.19.7][capnp] | 5.8357 ms | † | 14000088 | 7130367 | 6051062 | 81.231 ms |
| [cbor4ii 0.3.3][cbor4ii] | 9.8444 ms | 47.813 ms | 13125016 | 7524114 | 6757967 | 92.802 ms |
| [ciborium 0.2.2][ciborium] | 66.080 ms | 117.85 ms | 13122324 | 7524660 | 6759658 | 93.222 ms |
| [databuf 0.5.0][databuf] | 2.3956 ms | 5.3056 ms | 6000003 | 5378495 | 5345900 | 7.4623 ms |
| [dlhn 0.1.7][dlhn] | 5.8545 ms | 8.6152 ms | 6000003 | 5378495 | 5345900 | 7.5253 ms |
| [flatbuffers 24.3.25][flatbuffers] | 875.69 µs | † | 6000024 | 5378434 | 5345910 | 7.4619 ms |
| [msgpacker 0.4.3][msgpacker] | 2.0967 ms | 4.9552 ms | 7500005 | 6058442 | 6014337 | 9.5418 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 129.35 ms | 30.905 ms | 8125037 | 6493484 | 6386940 | 68.601 ms |
| [nanoserde 0.1.37][nanoserde] | 1.4626 ms | 1.1056 ms | 6000008 | 5378500 | 5345890 | 7.4216 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 4.7459 ms | 4.0426 ms | 6000004 | 5378496 | 5345889 | 7.5883 ms |
| [postcard 1.0.10][postcard] | 489.15 µs | 1.3439 ms | 6000003 | 5378495 | 5345900 | 7.4920 ms |
| [pot 3.0.1][pot] | 40.466 ms | 73.252 ms | 10122342 | 6814618 | 6852251 | 84.290 ms |
| [prost 0.13.2][prost] | <span title="encode">*7.7218 ms\**</span> <span title="populate + encode">*8.5504 ms\**</span> | 12.166 ms | 8750000 | 6665735 | 6421871 | 70.330 ms |
| [rkyv 0.8.5][rkyv] | 258.31 µs | <span title="unvalidated">*197.34 µs\**</span> <span title="validated upfront with error">*197.43 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.5712 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.256 ms | 18.012 ms | 8125006 | 6494876 | 6391037 | 67.462 ms |
| [ron 0.8.1][ron] | 168.81 ms | 226.14 ms | 22192885 | 8970395 | 8138755 | 149.77 ms |
| [savefile 0.17.7][savefile] | 258.63 µs | 258.66 µs | 6000024 | 5378513 | 5345893 | 7.4224 ms |
| [serde_bare 0.5.0][serde_bare] | 6.4722 ms | 4.0653 ms | 6000003 | 5378495 | 5345900 | 7.4078 ms |
| [serde_cbor 0.11.2][serde_cbor] | 33.845 ms | 46.538 ms | 13122324 | 7524660 | 6759658 | 92.784 ms |
| [serde_json 1.0.128][serde_json] | 86.365 ms | 87.482 ms | 26192883 | 9566084 | 8586741 | 154.09 ms |
| [simd-json 0.13.10][simd-json] | 52.315 ms | 72.231 ms | 26192883 | 9566084 | 8586741 | 154.05 ms |
| [speedy 0.8.7][speedy] | 258.56 µs | 258.73 µs | 6000004 | 5378496 | 5345889 | 7.5257 ms |
| [wiring 0.2.2][wiring] | 197.53 µs | 352.47 µs | 6000008 | 5378952 | 5345894 | 7.6508 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*105.40 ns\**</span> | <span title="validated on-demand with error">*2.2050 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4743 ns\**</span> <span title="validated upfront with error">*39.646 ns\**</span> | <span title="unvalidated">*54.208 µs\**</span> <span title="validated upfront with error">*77.487 µs\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2367 ns\**</span> <span title="validated upfront with error">*4.6464 ns\**</span> | <span title="unvalidated">*48.411 µs\**</span> <span title="validated upfront with error">*38.742 µs\**</span> | <span title="unvalidated">*99.753 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.95%\**</span> <span title="prepend">*2.26%\**</span> | 2.17% | 69.57% | 80.42% | 79.02% | 10.53% |
| [bincode 2.0.0-rc][bincode] | 8.24% | 19.37% | 100.00% | 96.35% | 92.11% | 99.54% |
| [bincode 1.3.3][bincode1] | 3.80% | 4.07% | 100.00% | 96.35% | 92.11% | 96.42% |
| [bitcode 0.6.3][bitcode] | 13.97% | 24.36% | 100.00% | 100.00% | 100.00% | 57.10% |
| [borsh 1.5.1][borsh] | 3.29% | 4.59% | 100.00% | 96.35% | 92.11% | 99.99% |
| [capnp 0.19.7][capnp] | 3.38% | † | 42.86% | 72.68% | 81.37% | 9.12% |
| [cbor4ii 0.3.3][cbor4ii] | 2.01% | 0.41% | 45.71% | 68.88% | 72.86% | 7.98% |
| [ciborium 0.2.2][ciborium] | 0.30% | 0.17% | 45.72% | 68.87% | 72.84% | 7.95% |
| [databuf 0.5.0][databuf] | 8.25% | 3.72% | 100.00% | 96.35% | 92.11% | 99.27% |
| [dlhn 0.1.7][dlhn] | 3.37% | 2.29% | 100.00% | 96.35% | 92.11% | 98.44% |
| [flatbuffers 24.3.25][flatbuffers] | 22.56% | † | 100.00% | 96.35% | 92.11% | 99.27% |
| [msgpacker 0.4.3][msgpacker] | 9.42% | 3.98% | 80.00% | 85.54% | 81.87% | 77.64% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.15% | 0.64% | 73.85% | 79.81% | 77.09% | 10.80% |
| [nanoserde 0.1.37][nanoserde] | 13.51% | 17.85% | 100.00% | 96.35% | 92.11% | 99.81% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 4.16% | 4.88% | 100.00% | 96.35% | 92.11% | 97.62% |
| [postcard 1.0.10][postcard] | 40.38% | 14.68% | 100.00% | 96.35% | 92.11% | 98.88% |
| [pot 3.0.1][pot] | 0.49% | 0.27% | 59.27% | 76.05% | 71.86% | 8.79% |
| [prost 0.13.2][prost] | <span title="encode">*2.56%\**</span> <span title="populate + encode">*2.31%\**</span> | 1.62% | 68.57% | 77.75% | 76.67% | 10.53% |
| [rkyv 0.8.5][rkyv] | 76.47% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.95%\**</span> | 100.00% | 96.35% | 92.11% | 97.84% |
| [rmp-serde 1.3.0][rmp-serde] | 1.29% | 1.10% | 73.85% | 79.79% | 77.04% | 10.98% |
| [ron 0.8.1][ron] | 0.12% | 0.09% | 27.04% | 57.77% | 60.50% | 4.95% |
| [savefile 0.17.7][savefile] | 76.38% | 76.29% | 100.00% | 96.35% | 92.11% | 99.80% |
| [serde_bare 0.5.0][serde_bare] | 3.05% | 4.85% | 100.00% | 96.35% | 92.11% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 0.58% | 0.42% | 45.72% | 68.87% | 72.84% | 7.98% |
| [serde_json 1.0.128][serde_json] | 0.23% | 0.23% | 22.91% | 54.17% | 57.34% | 4.81% |
| [simd-json 0.13.10][simd-json] | 0.38% | 0.27% | 22.91% | 54.17% | 57.34% | 4.81% |
| [speedy 0.8.7][speedy] | 76.40% | 76.27% | 100.00% | 96.35% | 92.11% | 98.43% |
| [wiring 0.2.2][wiring] | 100.00% | 55.99% | 100.00% | 96.34% | 92.11% | 96.82% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.17%\**</span> | <span title="validated on-demand with error">*1.76%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*3.12%\**</span> | <span title="unvalidated">*71.47%\**</span> <span title="validated upfront with error">*50.00%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*26.62%\**</span> | <span title="unvalidated">*80.03%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*926.62 µs\**</span> <span title="prepend">*840.37 µs\**</span> | 3.1952 ms | 489348 | 281173 | 249546 | 3.0782 ms |
| [bincode 2.0.0-rc][bincode] | 303.41 µs | 2.1171 ms | 367413 | 221291 | 206273 | 2.4782 ms |
| [bincode 1.3.3][bincode1] | 596.82 µs | 1.8650 ms | 569975 | 240525 | 232423 | 2.9188 ms |
| [bitcode 0.6.3][bitcode] | 133.89 µs | 1.2710 ms | 327688 | 200947 | 182736 | 749.67 µs |
| [borsh 1.5.1][borsh] | 518.11 µs | 1.8416 ms | 446595 | 234236 | 210008 | 2.4672 ms |
| [capnp 0.19.7][capnp] | 476.19 µs | † | 803896 | 335606 | 280851 | 3.9121 ms |
| [cbor4ii 0.3.3][cbor4ii] | 786.05 µs | 4.6978 ms | 1109831 | 344745 | 274514 | 3.8789 ms |
| [ciborium 0.2.2][ciborium] | 3.8069 ms | 10.319 ms | 1109821 | 344751 | 274526 | 3.8492 ms |
| [databuf 0.5.0][databuf] | 302.01 µs | 1.7295 ms | 356311 | 213062 | 198488 | 2.3832 ms |
| [dlhn 0.1.7][dlhn] | 769.97 µs | 2.6426 ms | 366496 | 220600 | 205683 | 2.4684 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.2657 ms | † | 844168 | 345696 | 294015 | 3.8239 ms |
| [msgpacker 0.4.3][msgpacker] | 705.01 µs | 2.8260 ms | 391251 | 236877 | 220476 | 2.6182 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4630 ms | 3.8996 ms | 449745 | 252432 | 231110 | 2.7578 ms |
| [nanoserde 0.1.37][nanoserde] | 261.40 µs | 1.9023 ms | 567975 | 239930 | 232419 | 2.8706 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 614.86 µs | 2.0959 ms | 356311 | 212976 | 198524 | 2.3974 ms |
| [postcard 1.0.10][postcard] | 446.84 µs | 2.0228 ms | 367489 | 221913 | 207344 | 2.4842 ms |
| [pot 3.0.1][pot] | 2.3896 ms | 6.2034 ms | 599125 | 299158 | 247693 | 3.1954 ms |
| [prost 0.13.2][prost] | <span title="encode">*1.2594 ms\**</span> <span title="populate + encode">*2.9066 ms\**</span> | 3.4984 ms | 596811 | 305319 | 269310 | 3.4975 ms |
| [rkyv 0.8.5][rkyv] | 346.04 µs | <span title="unvalidated">*1.5295 ms\**</span> <span title="validated upfront with error">*2.0385 ms\**</span> | 603776 | 254776 | 220087 | 2.7585 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4175 ms | 3.0097 ms | 424533 | 245214 | 226188 | 2.7246 ms |
| [ron 0.8.1][ron] | 7.1574 ms | 16.991 ms | 1465223 | 434935 | 343338 | 5.9812 ms |
| [savefile 0.17.7][savefile] | 205.85 µs | 1.8438 ms | 566991 | 239361 | 232013 | 2.8602 ms |
| [serde_bare 0.5.0][serde_bare] | 712.59 µs | 2.3776 ms | 356311 | 213062 | 198488 | 2.4120 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8081 ms | 4.7379 ms | 1109821 | 344751 | 274526 | 3.8515 ms |
| [serde_json 1.0.128][serde_json] | 3.6719 ms | 6.4289 ms | 1623191 | 466527 | 359623 | 6.0749 ms |
| [simd-json 0.13.10][simd-json] | 2.2279 ms | 4.5460 ms | 1623191 | 466527 | 359623 | 6.0855 ms |
| [speedy 0.8.7][speedy] | 278.90 µs | 1.5939 ms | 449595 | 234970 | 210361 | 2.5436 ms |
| [wiring 0.2.2][wiring] | 222.12 µs | 1.8121 ms | 566975 | 247810 | 225259 | 2.9275 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.463 ns\**</span> | <span title="validated on-demand with error">*424.88 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4781 ns\**</span> <span title="validated upfront with error">*2.1849 ms\**</span> | <span title="unvalidated">*1.3575 µs\**</span> <span title="validated upfront with error">*2.2118 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2371 ns\**</span> <span title="validated upfront with error">*500.73 µs\**</span> | <span title="unvalidated">*163.21 ns\**</span> <span title="validated upfront with error">*500.39 µs\**</span> | <span title="unvalidated">*775.85 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*14.45%\**</span> <span title="prepend">*15.93%\**</span> | 39.78% | 66.96% | 71.47% | 73.23% | 24.35% |
| [bincode 2.0.0-rc][bincode] | 44.13% | 60.03% | 89.19% | 90.81% | 88.59% | 30.25% |
| [bincode 1.3.3][bincode1] | 22.43% | 68.15% | 57.49% | 83.55% | 78.62% | 25.68% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 25.84% | 69.02% | 73.37% | 85.79% | 87.01% | 30.39% |
| [capnp 0.19.7][capnp] | 28.12% | † | 40.76% | 59.88% | 65.07% | 19.16% |
| [cbor4ii 0.3.3][cbor4ii] | 17.03% | 27.06% | 29.53% | 58.29% | 66.57% | 19.33% |
| [ciborium 0.2.2][ciborium] | 3.52% | 12.32% | 29.53% | 58.29% | 66.56% | 19.48% |
| [databuf 0.5.0][databuf] | 44.33% | 73.49% | 91.97% | 94.31% | 92.06% | 31.46% |
| [dlhn 0.1.7][dlhn] | 17.39% | 48.10% | 89.41% | 91.09% | 88.84% | 30.37% |
| [flatbuffers 24.3.25][flatbuffers] | 4.10% | † | 38.82% | 58.13% | 62.15% | 19.60% |
| [msgpacker 0.4.3][msgpacker] | 18.99% | 44.98% | 83.75% | 84.83% | 82.88% | 28.63% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.45% | 32.59% | 72.86% | 79.60% | 79.07% | 27.18% |
| [nanoserde 0.1.37][nanoserde] | 51.22% | 66.81% | 57.69% | 83.75% | 78.62% | 26.12% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.78% | 60.64% | 91.97% | 94.35% | 92.05% | 31.27% |
| [postcard 1.0.10][postcard] | 29.96% | 62.83% | 89.17% | 90.55% | 88.13% | 30.18% |
| [pot 3.0.1][pot] | 5.60% | 20.49% | 54.69% | 67.17% | 73.78% | 23.46% |
| [prost 0.13.2][prost] | <span title="encode">*10.63%\**</span> <span title="populate + encode">*4.61%\**</span> | 36.33% | 54.91% | 65.82% | 67.85% | 21.43% |
| [rkyv 0.8.5][rkyv] | 38.69% | <span title="unvalidated">*83.10%\**</span> <span title="validated upfront with error">*62.35%\**</span> | 54.27% | 78.87% | 83.03% | 27.18% |
| [rmp-serde 1.3.0][rmp-serde] | 9.45% | 42.23% | 77.19% | 81.95% | 80.79% | 27.52% |
| [ron 0.8.1][ron] | 1.87% | 7.48% | 22.36% | 46.20% | 53.22% | 12.53% |
| [savefile 0.17.7][savefile] | 65.04% | 68.93% | 57.79% | 83.95% | 78.76% | 26.21% |
| [serde_bare 0.5.0][serde_bare] | 18.79% | 53.46% | 91.97% | 94.31% | 92.06% | 31.08% |
| [serde_cbor 0.11.2][serde_cbor] | 7.41% | 26.83% | 29.53% | 58.29% | 66.56% | 19.46% |
| [serde_json 1.0.128][serde_json] | 3.65% | 19.77% | 20.19% | 43.07% | 50.81% | 12.34% |
| [simd-json 0.13.10][simd-json] | 6.01% | 27.96% | 20.19% | 43.07% | 50.81% | 12.32% |
| [speedy 0.8.7][speedy] | 48.01% | 79.74% | 72.89% | 85.52% | 86.87% | 29.47% |
| [wiring 0.2.2][wiring] | 60.28% | 70.14% | 57.80% | 81.09% | 81.12% | 25.61% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*38.41%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.92%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.02%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*4.6437 ms\**</span> <span title="prepend">*2.4776 ms\**</span> | 8.5670 ms | 1664428 | 1264167 | 1216472 | 10.967 ms |
| [bincode 2.0.0-rc][bincode] | 1.1938 ms | 4.0660 ms | 1372381 | 1091486 | 1037296 | 8.9313 ms |
| [bincode 1.3.3][bincode1] | 3.5734 ms | 4.2899 ms | 1811011 | 1115281 | 1025627 | 9.8752 ms |
| [bitcode 0.6.3][bitcode] | 709.45 µs | 2.3099 ms | 948499 | 857321 | 837658 | 3.0476 ms |
| [borsh 1.5.1][borsh] | 2.8702 ms | 2.8592 ms | 1486162 | 1082357 | 1013550 | 9.4805 ms |
| [capnp 0.19.7][capnp] | 2.3754 ms | † | 2664040 | 1511895 | 1212087 | 13.837 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.2689 ms | 17.251 ms | 5878791 | 1655835 | 1431390 | 20.705 ms |
| [ciborium 0.2.2][ciborium] | 22.529 ms | 56.529 ms | 5878653 | 1655791 | 1431560 | 21.485 ms |
| [databuf 0.5.0][databuf] | 1.2579 ms | 4.0254 ms | 1288257 | 1037579 | 984337 | 8.4869 ms |
| [dlhn 0.1.7][dlhn] | 4.7265 ms | 7.7631 ms | 1279599 | 1052061 | 1021161 | 8.1670 ms |
| [flatbuffers 24.3.25][flatbuffers] | 5.3193 ms | † | 2273740 | 1408408 | 1235566 | 12.655 ms |
| [msgpacker 0.4.3][msgpacker] | 1.6126 ms | 6.2763 ms | 1424043 | 1128758 | 1110156 | 9.1673 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.081 ms | 17.321 ms | 1728519 | 1247642 | 1233323 | 12.463 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2712 ms | 2.8730 ms | 1770477 | 1108304 | 1029947 | 9.9180 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.9800 ms | 3.7833 ms | 1288257 | 1039269 | 986510 | 8.4109 ms |
| [postcard 1.0.10][postcard] | 2.0733 ms | 4.1699 ms | 1279599 | 1058243 | 1016738 | 8.3081 ms |
| [pot 3.0.1][pot] | 13.843 ms | 31.587 ms | 2544810 | 1447453 | 1268390 | 15.334 ms |
| [prost 0.13.2][prost] | <span title="encode">*5.2988 ms\**</span> <span title="populate + encode">*9.1610 ms\**</span> | 8.4523 ms | 1818378 | 1307777 | 1266311 | 11.539 ms |
| [rkyv 0.8.5][rkyv] | 904.02 µs | <span title="unvalidated">*2.1523 ms\**</span> <span title="validated upfront with error">*2.5731 ms\**</span> | 2029080 | 1351984 | 1183990 | 12.349 ms |
| [rmp-serde 1.3.0][rmp-serde] | 9.9240 ms | 11.401 ms | 1703813 | 1231892 | 1200208 | 11.004 ms |
| [ron 0.8.1][ron] | 36.830 ms | 87.014 ms | 8476284 | 2181196 | 1783971 | 33.706 ms |
| [savefile 0.17.7][savefile] | 800.62 µs | 2.9594 ms | 1750226 | 1101682 | 1027828 | 9.7791 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7583 ms | 4.9962 ms | 1288257 | 1037597 | 984356 | 8.5924 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.4197 ms | 20.736 ms | 5878653 | 1655791 | 1431560 | 20.822 ms |
| [serde_json 1.0.128][serde_json] | 20.031 ms | 29.215 ms | 9175594 | 2334253 | 1800713 | 34.056 ms |
| [simd-json 0.13.10][simd-json] | 11.410 ms | 26.500 ms | 9175594 | 2334253 | 1800713 | 35.277 ms |
| [speedy 0.8.7][speedy] | 708.74 µs | 2.4172 ms | 1546963 | 1093532 | 1013443 | 9.5776 ms |
| [wiring 0.2.2][wiring] | 691.52 µs | 2.7131 ms | 1750210 | 1129857 | 1058906 | 10.159 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.523 ns\**</span> | <span title="validated on-demand with error">*709.61 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4749 ns\**</span> <span title="validated upfront with error">*5.0178 ms\**</span> | <span title="unvalidated">*2.6273 µs\**</span> <span title="validated upfront with error">*5.0853 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2372 ns\**</span> <span title="validated upfront with error">*425.37 µs\**</span> | <span title="unvalidated">*424.35 ns\**</span> <span title="validated upfront with error">*429.22 µs\**</span> | <span title="unvalidated">*234.83 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*14.89%\**</span> <span title="prepend">*27.91%\**</span> | 25.12% | 56.99% | 67.82% | 68.86% | 27.79% |
| [bincode 2.0.0-rc][bincode] | 57.93% | 52.93% | 69.11% | 78.55% | 80.75% | 34.12% |
| [bincode 1.3.3][bincode1] | 19.35% | 50.17% | 52.37% | 76.87% | 81.67% | 30.86% |
| [bitcode 0.6.3][bitcode] | 97.47% | 93.18% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 24.09% | 75.28% | 63.82% | 79.21% | 82.65% | 32.15% |
| [capnp 0.19.7][capnp] | 29.11% | † | 35.60% | 56.71% | 69.11% | 22.03% |
| [cbor4ii 0.3.3][cbor4ii] | 21.15% | 12.48% | 16.13% | 51.78% | 58.52% | 14.72% |
| [ciborium 0.2.2][ciborium] | 3.07% | 3.81% | 16.13% | 51.78% | 58.51% | 14.18% |
| [databuf 0.5.0][databuf] | 54.97% | 53.47% | 73.63% | 82.63% | 85.10% | 35.91% |
| [dlhn 0.1.7][dlhn] | 14.63% | 27.72% | 74.12% | 81.49% | 82.03% | 37.32% |
| [flatbuffers 24.3.25][flatbuffers] | 13.00% | † | 41.72% | 60.87% | 67.80% | 24.08% |
| [msgpacker 0.4.3][msgpacker] | 42.88% | 34.29% | 66.61% | 75.95% | 75.45% | 33.24% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.30% | 12.43% | 54.87% | 68.72% | 67.92% | 24.45% |
| [nanoserde 0.1.37][nanoserde] | 54.40% | 74.91% | 53.57% | 77.35% | 81.33% | 30.73% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 23.21% | 56.89% | 73.63% | 82.49% | 84.91% | 36.23% |
| [postcard 1.0.10][postcard] | 33.35% | 51.62% | 74.12% | 81.01% | 82.39% | 36.68% |
| [pot 3.0.1][pot] | 5.00% | 6.81% | 37.27% | 59.23% | 66.04% | 19.88% |
| [prost 0.13.2][prost] | <span title="encode">*13.05%\**</span> <span title="populate + encode">*7.55%\**</span> | 25.46% | 52.16% | 65.56% | 66.15% | 26.41% |
| [rkyv 0.8.5][rkyv] | 76.49% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.65%\**</span> | 46.75% | 63.41% | 70.75% | 24.68% |
| [rmp-serde 1.3.0][rmp-serde] | 6.97% | 18.88% | 55.67% | 69.59% | 69.79% | 27.70% |
| [ron 0.8.1][ron] | 1.88% | 2.47% | 11.19% | 39.31% | 46.95% | 9.04% |
| [savefile 0.17.7][savefile] | 86.37% | 72.73% | 54.19% | 77.82% | 81.50% | 31.16% |
| [serde_bare 0.5.0][serde_bare] | 14.53% | 43.08% | 73.63% | 82.63% | 85.10% | 35.47% |
| [serde_cbor 0.11.2][serde_cbor] | 7.34% | 10.38% | 16.13% | 51.78% | 58.51% | 14.64% |
| [serde_json 1.0.128][serde_json] | 3.45% | 7.37% | 10.34% | 36.73% | 46.52% | 8.95% |
| [simd-json 0.13.10][simd-json] | 6.06% | 8.12% | 10.34% | 36.73% | 46.52% | 8.64% |
| [speedy 0.8.7][speedy] | 97.57% | 89.04% | 61.31% | 78.40% | 82.65% | 31.82% |
| [wiring 0.2.2][wiring] | 100.00% | 79.33% | 54.19% | 75.88% | 79.11% | 30.00% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.71%\**</span> | <span title="validated on-demand with error">*59.80%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.15%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1010.0
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.3
[borsh]: https://crates.io/crates/borsh/1.5.1
[capnp]: https://crates.io/crates/capnp/0.19.7
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.3
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/24.3.25
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.12
[postcard]: https://crates.io/crates/postcard/1.0.10
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.13.2
[rkyv]: https://crates.io/crates/rkyv/0.8.5
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.17.7
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.128
[simd-json]: https://crates.io/crates/simd-json/0.13.10
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.2.2


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
