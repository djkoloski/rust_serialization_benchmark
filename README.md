<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## Maintainers

These benchmarks are maintained by a small group of volunteers. Special thanks to:

- [djkoloski](https://github.com/djkoloski)
- [mumbleskates](https://github.com/mumbleskates)
- [finnbear](https://github.com/finnbear)

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

## Last updated: 2026-01-13 14:50:57

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.94.0-nightly (2f1bd3f37 2026-01-12)
binary: rustc
commit-hash: 2f1bd3f3781c90a8447e37d65a898442b8618895
commit-date: 2026-01-12
host: x86_64-unknown-linux-gnu
release: 1.94.0-nightly
LLVM version: 21.1.8
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
Flags:                                fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves user_shstk clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*447.76 µs\**</span> <span title="prepend">*410.14 µs\**</span> | 2.5949 ms | 865.44 µs | 804955 | 328941 | 284849 | 4.1406 ms |
| [bin-proto 0.12.3][bin-proto] | 4.3070 ms | 4.9015 ms | † | 1045784 | 373127 | 311553 | 4.4272 ms |
| [bincode 2.0.1][bincode] | 382.28 µs | 2.3543 ms | 681.72 µs | 741295 | 303944 | 256422 | 3.5544 ms |
| [bincode 1.3.3][bincode1] | 549.33 µs | 2.0543 ms | 592.61 µs | 1045784 | 373127 | 311553 | 4.4595 ms |
| [bitcode 0.6.6][bitcode] | 138.72 µs | 1.4388 ms | 59.374 µs | 703710 | 288826 | 227322 | 2.4964 ms |
| [borsh 1.5.7][borsh] | 551.54 µs | 2.2016 ms | † | 885780 | 362204 | 286248 | 3.8501 ms |
| [capnp 0.23.2][capnp] | 454.45 µs | † | † | 1443216 | 513986 | 426532 | 6.2140 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 662.50 µs | 4.9803 ms | 3.3815 ms | 1407835 | 403440 | 323561 | 5.0309 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.0231 ms | 11.364 ms | † | 1407835 | 403440 | 323561 | 5.0195 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 2.0991 ms | 4.9722 ms | 3.1112 ms | 1407835 | 403440 | 323561 | 4.7112 ms |
| [columnar 0.11.0][columnar] | 248.67 µs | 2.2355 ms <span title="copy_from">*810.57 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.2218 ms |
| [databuf 0.5.0][databuf] | 275.25 µs | 2.0345 ms | 668.40 µs | 765778 | 311715 | 263914 | 3.4533 ms |
| [dlhn 0.1.7][dlhn] | 730.31 µs | 2.5801 ms | † | 724953 | 301446 | 253056 | 3.1850 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0336 ms | † | † | 1276368 | 468539 | 388381 | 4.7909 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.2837 ms | 7.3589 ms | 5.6318 ms | 1829756 | 714318 | 691541 | 8.5473 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.5731 ms | 6.0853 ms | † | 1827461 | 470560 | 360727 | 5.4682 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2173 ms | 4.6245 ms | † | 1827461 | 470560 | 360727 | 5.4158 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.3892 ms | 2.5574 ms | † | 764996 | 315291 | 264212 | 3.5082 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.5240 ms | 3.1151 ms | 1.4319 ms | 784997 | 325384 | 277608 | 3.7357 ms |
| [minicbor 1.0.0][minicbor] | 473.61 µs | 3.0824 ms | 1.4233 ms | 817830 | 332671 | 284034 | 3.9569 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3605 ms | 4.5444 ms | 3.1091 ms | 818669 | 332556 | 284797 | 3.9303 ms |
| [nanoserde 0.2.1][nanoserde] | 258.86 µs | 2.1687 ms | † | 1045784 | 373127 | 311553 | 4.1504 ms |
| [nibblecode 0.1.0][nibblecode] | 186.88 µs | † | † | 1011487 | 473996 | 404668 | 5.2316 ms |
| [postcard 1.1.1][postcard] | 430.08 µs | 2.2392 ms | 618.23 µs | 724953 | 302399 | 252968 | 3.1634 ms |
| [pot 3.0.1][pot] | 2.2821 ms | 6.6605 ms | 4.9063 ms | 971922 | 372513 | 303636 | 4.3739 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*935.19 µs\**</span> <span title="populate + encode">*2.4439 ms\**</span> | 3.4643 ms | † | 884628 | 363130 | 314959 | 4.3609 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2074 ms\**</span> <span title="populate + encode">*3.0078 ms\**</span> | 3.8523 ms | † | 884628 | 363130 | 314959 | 4.7579 ms |
| [rkyv 0.8.10][rkyv] | 249.15 µs | <span title="unvalidated">*1.5422 ms\**</span> <span title="validated upfront with error">*1.9095 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5659 ms |
| [ron 0.10.1][ron] | 11.719 ms | 23.554 ms | 21.541 ms | 1607459 | 449158 | 349324 | 5.5170 ms |
| [savefile 0.18.6][savefile] | 193.40 µs | 2.1505 ms | † | 1045800 | 373139 | 311562 | 4.1885 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 671.17 µs | 2.4765 ms | † | 765778 | 311743 | 263822 | 3.4824 ms |
| [serde-brief 0.1.1][serde-brief] | 1.6621 ms | 4.8979 ms | 2.9857 ms | 1584946 | 413733 | 339964 | 4.8211 ms |
| [serde_bare 0.5.0][serde_bare] | 694.72 µs | 2.1798 ms | † | 765778 | 311715 | 263914 | 3.4826 ms |
| [speedy 0.8.7][speedy] | 200.59 µs | 1.7734 ms | 388.39 µs | 885780 | 362204 | 286248 | 3.8198 ms |
| [wincode 0.2.4][wincode] | 174.61 µs | 2.0056 ms | 465.75 µs | 1045784 | 373127 | 311553 | 4.2067 ms |
| [wiring 0.2.4][wiring] | 194.18 µs | 2.0299 ms | † | 1045784 | 337930 | 275808 | 3.6430 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*82.015 ns\**</span> | <span title="validated on-demand with error">*191.25 µs\**</span> | ‡ |
| [columnar 0.11.0][columnar] | 20.516 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4867 ns\**</span> <span title="validated upfront with error">*2.0986 ms\**</span> | <span title="unvalidated">*51.336 µs\**</span> <span title="validated upfront with error">*2.1150 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*266.03 µs\**</span> | <span title="unvalidated">*10.390 µs\**</span> <span title="validated upfront with error">*277.50 µs\**</span> | <span title="unvalidated">*7.4679 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2427 ns\**</span> <span title="validated upfront with error">*370.29 µs\**</span> | <span title="unvalidated">*10.334 µs\**</span> <span title="validated upfront with error">*380.31 µs\**</span> | <span title="unvalidated">*7.6027 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*30.98%\**</span> <span title="prepend">*33.82%\**</span> | 31.24% | 6.86% | 87.42% | 87.80% | 79.80% | 60.29% |
| [bin-proto 0.12.3][bin-proto] | 3.22% | 16.54% | † | 67.29% | 77.41% | 72.96% | 56.39% |
| [bincode 2.0.1][bincode] | 36.29% | 34.43% | 8.71% | 94.93% | 95.03% | 88.65% | 70.23% |
| [bincode 1.3.3][bincode1] | 25.25% | 39.46% | 10.02% | 67.29% | 77.41% | 72.96% | 55.98% |
| [bitcode 0.6.6][bitcode] | 100.00% | 56.34% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 25.15% | 36.82% | † | 79.45% | 79.74% | 79.41% | 64.84% |
| [capnp 0.23.2][capnp] | 30.52% | † | † | 48.76% | 56.19% | 53.30% | 40.17% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 20.94% | 16.28% | 1.76% | 49.99% | 71.59% | 70.26% | 49.62% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.45% | 7.13% | † | 49.99% | 71.59% | 70.26% | 49.73% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.61% | 16.30% | 1.91% | 49.99% | 71.59% | 70.26% | 52.99% |
| [columnar 0.11.0][columnar] | 55.78% | 36.26% <span title="copy_from">*100.00%\**</span> | † | 67.28% | 78.02% | 77.34% | 59.13% |
| [databuf 0.5.0][databuf] | 50.40% | 39.84% | 8.88% | 91.89% | 92.66% | 86.13% | 72.29% |
| [dlhn 0.1.7][dlhn] | 18.99% | 31.42% | † | 97.07% | 95.81% | 89.83% | 78.38% |
| [flatbuffers 25.12.19][flatbuffers] | 13.42% | † | † | 55.13% | 61.64% | 58.53% | 52.11% |
| [flexbuffers 25.2.10][flexbuffers] | 1.90% | 11.01% | 1.05% | 38.46% | 40.43% | 32.87% | 29.21% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.88% | 13.32% | † | 38.51% | 61.38% | 63.02% | 45.65% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.26% | 17.53% | † | 38.51% | 61.38% | 63.02% | 46.09% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 9.99% | 31.70% | † | 91.99% | 91.61% | 86.04% | 71.16% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.10% | 26.02% | 4.15% | 89.64% | 88.76% | 81.89% | 66.83% |
| [minicbor 1.0.0][minicbor] | 29.29% | 26.30% | 4.17% | 86.05% | 86.82% | 80.03% | 63.09% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.59% | 17.84% | 1.91% | 85.96% | 86.85% | 79.82% | 63.52% |
| [nanoserde 0.2.1][nanoserde] | 53.59% | 37.38% | † | 67.29% | 77.41% | 72.96% | 60.15% |
| [nibblecode 0.1.0][nibblecode] | 74.23% | † | † | 69.57% | 60.93% | 56.17% | 47.72% |
| [postcard 1.1.1][postcard] | 32.25% | 36.20% | 9.60% | 97.07% | 95.51% | 89.86% | 78.92% |
| [pot 3.0.1][pot] | 6.08% | 12.17% | 1.21% | 72.40% | 77.53% | 74.87% | 57.07% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*14.83%\**</span> <span title="populate + encode">*5.68%\**</span> | 23.40% | † | 79.55% | 79.54% | 72.18% | 57.25% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.49%\**</span> <span title="populate + encode">*4.61%\**</span> | 21.04% | † | 79.55% | 79.54% | 72.18% | 52.47% |
| [rkyv 0.8.10][rkyv] | 55.68% | <span title="unvalidated">*52.56%\**</span> <span title="validated upfront with error">*42.45%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.67% |
| [ron 0.10.1][ron] | 1.18% | 3.44% | 0.28% | 43.78% | 64.30% | 65.07% | 45.25% |
| [savefile 0.18.6][savefile] | 71.73% | 37.69% | † | 67.29% | 77.40% | 72.96% | 59.60% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.67% | 32.73% | † | 91.89% | 92.65% | 86.16% | 71.69% |
| [serde-brief 0.1.1][serde-brief] | 8.35% | 16.55% | 1.99% | 44.40% | 69.81% | 66.87% | 51.78% |
| [serde_bare 0.5.0][serde_bare] | 19.97% | 37.19% | † | 91.89% | 92.66% | 86.13% | 71.68% |
| [speedy 0.8.7][speedy] | 69.16% | 45.71% | 15.29% | 79.45% | 79.74% | 79.41% | 65.35% |
| [wincode 0.2.4][wincode] | 79.45% | 40.42% | 12.75% | 67.29% | 77.41% | 72.96% | 59.34% |
| [wiring 0.2.4][wiring] | 71.44% | 39.93% | † | 67.29% | 85.47% | 82.42% | 68.53% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.52%\**</span> | <span title="validated on-demand with error">*5.40%\**</span> | ‡ |
| [columnar 0.11.0][columnar] | 6.06% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.13%\**</span> <span title="validated upfront with error">*0.49%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.46%\**</span> <span title="validated upfront with error">*3.72%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.72%\**</span> | <span title="unvalidated">*98.23%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.4934 ms\**</span> <span title="prepend">*8.8466 ms\**</span> | 7.8549 ms | 8625005 | 6443961 | 6231572 | 73.250 ms |
| [bin-proto 0.12.3][bin-proto] | 8.4808 ms | 9.8667 ms | 6000008 | 5378500 | 5346908 | 8.4579 ms |
| [bincode 2.0.1][bincode] | 2.8844 ms | 983.75 µs | 6000005 | 5378497 | 5346882 | 8.4744 ms |
| [bincode 1.3.3][bincode1] | 5.1502 ms | 5.8509 ms | 6000008 | 5378500 | 5346908 | 8.5170 ms |
| [bitcode 0.6.6][bitcode] | 1.3164 ms | 799.57 µs | 6000006 | 5182295 | 4921841 | 13.361 ms |
| [borsh 1.5.7][borsh] | 6.2029 ms | 4.1343 ms | 6000004 | 5378496 | 5346866 | 8.4127 ms |
| [capnp 0.23.2][capnp] | 6.4883 ms | † | 14000088 | 7130367 | 6046182 | 81.798 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 10.102 ms | 47.030 ms | 13125016 | 7524114 | 6757437 | 91.284 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 66.362 ms | 113.48 ms | 13122324 | 7524660 | 6759128 | 90.203 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 34.186 ms | 41.764 ms | 13122324 | 7524660 | 6759128 | 89.621 ms |
| [columnar 0.11.0][columnar] | 1.7574 ms | 1.4266 ms <span title="copy_from">*670.27 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.6414 ms |
| [databuf 0.5.0][databuf] | 2.4128 ms | 5.3417 ms | 6000003 | 5378495 | 5346897 | 8.7137 ms |
| [dlhn 0.1.7][dlhn] | 6.0779 ms | 6.8965 ms | 6000003 | 5378495 | 5346897 | 8.6596 ms |
| [flatbuffers 25.12.19][flatbuffers] | 966.06 µs | † | 6000024 | 5378434 | 5346878 | 8.9064 ms |
| [flexbuffers 25.2.10][flexbuffers] | 101.48 ms | 77.166 ms | 26609424 | 11901040 | 12486322 | 153.78 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 86.698 ms | 98.961 ms | 26192883 | 9566084 | 8584671 | 157.12 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 53.316 ms | 70.053 ms | 26192883 | 9566084 | 8584671 | 155.24 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 19.253 ms | 5.0790 ms | 7500005 | 6058442 | 6014500 | 10.497 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 15.785 ms | 17.550 ms | 8125006 | 6494876 | 6391037 | 73.573 ms |
| [minicbor 1.0.0][minicbor] | 5.1852 ms | 11.456 ms | 8125006 | 6494907 | 6390894 | 69.334 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 120.24 ms | 30.579 ms | 8125037 | 6493484 | 6386940 | 69.183 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2317 ms | 892.71 µs | 6000008 | 5378500 | 5346908 | 8.4529 ms |
| [nibblecode 0.1.0][nibblecode] | 148.21 µs | † | 6000008 | 5378500 | 5346908 | 8.5513 ms |
| [postcard 1.1.1][postcard] | 477.61 µs | 1.2844 ms | 6000003 | 5378495 | 5346897 | 8.6367 ms |
| [pot 3.0.1][pot] | 38.260 ms | 77.356 ms | 10122342 | 6814618 | 6852252 | 81.321 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*7.7566 ms\**</span> <span title="populate + encode">*8.3991 ms\**</span> | 10.930 ms | 8750000 | 6665735 | 6421877 | 72.361 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*16.002 ms\**</span> <span title="populate + encode">*32.013 ms\**</span> | 29.259 ms | 8750000 | 6665735 | 6421877 | 80.622 ms |
| [rkyv 0.8.10][rkyv] | 155.15 µs | <span title="unvalidated">*152.49 µs\**</span> <span title="validated upfront with error">*152.60 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.5664 ms |
| [ron 0.10.1][ron] | 164.90 ms | 490.10 ms | 22192885 | 8970395 | 8137334 | 151.99 ms |
| [savefile 0.18.6][savefile] | 203.96 µs | 202.27 µs | 6000024 | 5378519 | 5346896 | 8.7030 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1089 ms | 4.3114 ms | 6000004 | 5378496 | 5346866 | 8.5227 ms |
| [serde-brief 0.1.1][serde-brief] | 22.173 ms | 40.323 ms | 15750015 | 8024540 | 6813667 | 92.518 ms |
| [serde_bare 0.5.0][serde_bare] | 5.9982 ms | 4.6774 ms | 6000003 | 5378495 | 5346897 | 8.4601 ms |
| [speedy 0.8.7][speedy] | 148.09 µs | 148.38 µs | 6000004 | 5378496 | 5346866 | 8.5195 ms |
| [wincode 0.2.4][wincode] | 150.53 µs | 148.67 µs | 6000008 | 5378500 | 5346908 | 8.7247 ms |
| [wiring 0.2.4][wiring] | 150.24 µs | 322.19 µs | 6000008 | 5378952 | 5346905 | 8.7099 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*117.20 ns\**</span> | <span title="validated on-demand with error">*1.9246 ms\**</span> | ‡ |
| [columnar 0.11.0][columnar] | 19.273 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4866 ns\**</span> <span title="validated upfront with error">*45.498 ns\**</span> | <span title="unvalidated">*77.764 µs\**</span> <span title="validated upfront with error">*77.747 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2493 ns\**</span> <span title="validated upfront with error">*1.5542 ns\**</span> | <span title="unvalidated">*38.839 µs\**</span> <span title="validated upfront with error">*38.855 µs\**</span> | <span title="unvalidated">*78.482 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2425 ns\**</span> <span title="validated upfront with error">*5.2922 ns\**</span> | <span title="unvalidated">*38.859 µs\**</span> <span title="validated upfront with error">*38.830 µs\**</span> | <span title="unvalidated">*76.247 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*1.98%\**</span> <span title="prepend">*1.67%\**</span> | 1.89% | 69.57% | 80.42% | 78.98% | 11.48% |
| [bin-proto 0.12.3][bin-proto] | 1.75% | 1.50% | 100.00% | 96.35% | 92.05% | 99.47% |
| [bincode 2.0.1][bincode] | 5.13% | 15.08% | 100.00% | 96.35% | 92.05% | 99.27% |
| [bincode 1.3.3][bincode1] | 2.88% | 2.54% | 100.00% | 96.35% | 92.05% | 98.78% |
| [bitcode 0.6.6][bitcode] | 11.25% | 18.56% | 100.00% | 100.00% | 100.00% | 62.97% |
| [borsh 1.5.7][borsh] | 2.39% | 3.59% | 100.00% | 96.35% | 92.05% | 100.00% |
| [capnp 0.23.2][capnp] | 2.28% | † | 42.86% | 72.68% | 81.40% | 10.28% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.47% | 0.32% | 45.71% | 68.88% | 72.84% | 9.22% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.82% | 9.33% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.43% | 0.36% | 45.72% | 68.87% | 72.82% | 9.39% |
| [columnar 0.11.0][columnar] | 8.43% | 10.40% <span title="copy_from">*22.14%\**</span> | 100.00% | 96.35% | 92.05% | 97.35% |
| [databuf 0.5.0][databuf] | 6.14% | 2.78% | 100.00% | 96.35% | 92.05% | 96.55% |
| [dlhn 0.1.7][dlhn] | 2.44% | 2.15% | 100.00% | 96.35% | 92.05% | 97.15% |
| [flatbuffers 25.12.19][flatbuffers] | 15.33% | † | 100.00% | 96.35% | 92.05% | 94.46% |
| [flexbuffers 25.2.10][flexbuffers] | 0.15% | 0.19% | 22.55% | 43.54% | 39.42% | 5.47% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.15% | 22.91% | 54.17% | 57.33% | 5.35% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.28% | 0.21% | 22.91% | 54.17% | 57.33% | 5.42% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 0.77% | 2.92% | 80.00% | 85.54% | 81.83% | 80.14% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.94% | 0.85% | 73.85% | 79.79% | 77.01% | 11.43% |
| [minicbor 1.0.0][minicbor] | 2.86% | 1.30% | 73.85% | 79.79% | 77.01% | 12.13% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.49% | 73.85% | 79.81% | 77.06% | 12.16% |
| [nanoserde 0.2.1][nanoserde] | 12.02% | 16.62% | 100.00% | 96.35% | 92.05% | 99.52% |
| [nibblecode 0.1.0][nibblecode] | 99.92% | † | 100.00% | 96.35% | 92.05% | 98.38% |
| [postcard 1.1.1][postcard] | 31.01% | 11.55% | 100.00% | 96.35% | 92.05% | 97.41% |
| [pot 3.0.1][pot] | 0.39% | 0.19% | 59.27% | 76.05% | 71.83% | 10.35% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.91%\**</span> <span title="populate + encode">*1.76%\**</span> | 1.36% | 68.57% | 77.75% | 76.64% | 11.63% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*0.93%\**</span> <span title="populate + encode">*0.46%\**</span> | 0.51% | 68.57% | 77.75% | 76.64% | 10.43% |
| [rkyv 0.8.10][rkyv] | 95.45% | <span title="unvalidated">*97.30%\**</span> <span title="validated upfront with error">*97.23%\**</span> | 100.00% | 96.35% | 92.05% | 98.21% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 27.04% | 57.77% | 60.48% | 5.54% |
| [savefile 0.18.6][savefile] | 72.61% | 73.36% | 100.00% | 96.35% | 92.05% | 96.66% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.90% | 3.44% | 100.00% | 96.35% | 92.05% | 98.71% |
| [serde-brief 0.1.1][serde-brief] | 0.67% | 0.37% | 38.10% | 64.58% | 72.23% | 9.09% |
| [serde_bare 0.5.0][serde_bare] | 2.47% | 3.17% | 100.00% | 96.35% | 92.05% | 99.44% |
| [speedy 0.8.7][speedy] | 100.00% | 100.00% | 100.00% | 96.35% | 92.05% | 98.75% |
| [wincode 0.2.4][wincode] | 98.38% | 99.80% | 100.00% | 96.35% | 92.05% | 96.42% |
| [wiring 0.2.4][wiring] | 98.57% | 46.05% | 100.00% | 96.34% | 92.05% | 96.59% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.06%\**</span> | <span title="validated on-demand with error">*2.02%\**</span> | ‡ |
| [columnar 0.11.0][columnar] | 6.45% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*2.73%\**</span> | <span title="unvalidated">*49.93%\**</span> <span title="validated upfront with error">*49.94%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.46%\**</span> <span title="validated upfront with error">*79.94%\**</span> | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*99.94%\**</span> | <span title="unvalidated">*97.15%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*23.48%\**</span> | <span title="unvalidated">*99.93%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*890.83 µs\**</span> <span title="prepend">*774.56 µs\**</span> | 3.1644 ms | 1.7138 ms | 489348 | 281173 | 249360 | 2.6405 ms |
| [bin-proto 0.12.3][bin-proto] | 1.8463 ms | 2.7975 ms | † | 566975 | 239350 | 231475 | 2.4343 ms |
| [bincode 2.0.1][bincode] | 322.88 µs | 1.9345 ms | 851.81 µs | 367413 | 221291 | 206242 | 2.0369 ms |
| [bincode 1.3.3][bincode1] | 593.81 µs | 1.8420 ms | 853.62 µs | 569975 | 240525 | 231884 | 2.4510 ms |
| [bitcode 0.6.6][bitcode] | 126.72 µs | 1.2575 ms | 173.09 µs | 327688 | 200947 | 182040 | 745.01 µs |
| [borsh 1.5.7][borsh] | 560.60 µs | 1.8206 ms | † | 446595 | 234236 | 209834 | 2.0635 ms |
| [capnp 0.23.2][capnp] | 440.28 µs | † | † | 803896 | 335606 | 280744 | 3.5865 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 741.93 µs | 4.5566 ms | 3.3414 ms | 1109831 | 344745 | 274333 | 3.4733 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.6148 ms | 10.140 ms | † | 1109821 | 344751 | 274345 | 3.4461 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8650 ms | 4.6148 ms | 3.2907 ms | 1109821 | 344751 | 274345 | 3.4629 ms |
| [columnar 0.11.0][columnar] | 297.97 µs | 1.9172 ms <span title="copy_from">*763.45 µs\**</span> | † | 563728 | 249696 | 217582 | 1.6110 ms |
| [databuf 0.5.0][databuf] | 296.07 µs | 1.7589 ms | 777.03 µs | 356311 | 213062 | 198403 | 1.9519 ms |
| [dlhn 0.1.7][dlhn] | 776.68 µs | 2.5708 ms | † | 366496 | 220600 | 205586 | 2.0213 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2471 ms | † | † | 849472 | 347816 | 294871 | 3.5646 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.8009 ms | 6.7487 ms | 5.3993 ms | 1187688 | 557642 | 553730 | 6.2557 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6423 ms | 6.8517 ms | † | 1623191 | 466527 | 359157 | 5.6770 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2245 ms | 4.7172 ms | † | 1623191 | 466527 | 359157 | 5.6741 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 989.74 µs | 2.8463 ms | † | 391251 | 236877 | 220395 | 2.1691 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.5458 ms | 3.0547 ms | 1.7236 ms | 424533 | 245214 | 226077 | 2.2906 ms |
| [minicbor 1.0.0][minicbor] | 563.90 µs | 3.3666 ms | 1.8654 ms | 428773 | 249857 | 228630 | 2.2835 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0823 ms | 4.2781 ms | 3.0659 ms | 449745 | 252432 | 230965 | 2.2948 ms |
| [nanoserde 0.2.1][nanoserde] | 262.01 µs | 1.8742 ms | † | 567975 | 239930 | 231872 | 2.4485 ms |
| [nibblecode 0.1.0][nibblecode] | 198.67 µs | † | † | 603928 | 432462 | 409595 | 3.5789 ms |
| [postcard 1.1.1][postcard] | 444.72 µs | 1.9938 ms | 801.80 µs | 367489 | 221913 | 207244 | 2.0241 ms |
| [pot 3.0.1][pot] | 2.3666 ms | 6.2793 ms | 5.0611 ms | 599125 | 299158 | 247675 | 2.7034 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.2922 ms\**</span> <span title="populate + encode">*2.9930 ms\**</span> | 3.4581 ms | † | 596811 | 305319 | 268737 | 2.9664 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0732 ms\**</span> <span title="populate + encode">*3.0099 ms\**</span> | 3.8994 ms | † | 596811 | 305319 | 268737 | 3.0346 ms |
| [rkyv 0.8.10][rkyv] | 326.82 µs | <span title="unvalidated">*1.4921 ms\**</span> <span title="validated upfront with error">*1.8425 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3140 ms |
| [ron 0.10.1][ron] | 7.9592 ms | 24.523 ms | 22.723 ms | 1465223 | 434935 | 342907 | 5.5974 ms |
| [savefile 0.18.6][savefile] | 213.62 µs | 1.8317 ms | † | 566991 | 239362 | 231478 | 2.4379 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 631.04 µs | 2.0859 ms | † | 356311 | 212976 | 198423 | 1.9902 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3155 ms | 5.3892 ms | 3.6976 ms | 1276014 | 373898 | 293384 | 3.7419 ms |
| [serde_bare 0.5.0][serde_bare] | 764.50 µs | 2.3369 ms | † | 356311 | 213062 | 198403 | 2.0353 ms |
| [speedy 0.8.7][speedy] | 266.79 µs | 1.6772 ms | 541.12 µs | 449595 | 234970 | 210192 | 2.1424 ms |
| [wincode 0.2.4][wincode] | 201.43 µs | 1.6799 ms | 625.47 µs | 566975 | 239350 | 231475 | 2.4601 ms |
| [wiring 0.2.4][wiring] | 209.55 µs | 1.8457 ms | † | 566975 | 247810 | 225086 | 2.7337 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*85.187 ns\**</span> | <span title="validated on-demand with error">*370.46 ns\**</span> | ‡ |
| [columnar 0.11.0][columnar] | 828.22 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4856 ns\**</span> <span title="validated upfront with error">*2.3988 ms\**</span> | <span title="unvalidated">*1.4209 µs\**</span> <span title="validated upfront with error">*2.4030 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2427 ns\**</span> <span title="validated upfront with error">*256.65 µs\**</span> | <span title="unvalidated">*155.97 ns\**</span> <span title="validated upfront with error">*258.76 µs\**</span> | <span title="unvalidated">*751.96 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*323.51 µs\**</span> | <span title="unvalidated">*156.06 ns\**</span> <span title="validated upfront with error">*321.69 µs\**</span> | <span title="unvalidated">*774.31 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.22%\**</span> <span title="prepend">*16.36%\**</span> | 24.13% | 10.10% | 66.96% | 71.47% | 73.00% | 28.21% |
| [bin-proto 0.12.3][bin-proto] | 6.86% | 27.29% | † | 57.80% | 83.96% | 78.64% | 30.60% |
| [bincode 2.0.1][bincode] | 39.25% | 39.46% | 20.32% | 89.19% | 90.81% | 88.27% | 36.58% |
| [bincode 1.3.3][bincode1] | 21.34% | 41.45% | 20.28% | 57.49% | 83.55% | 78.50% | 30.40% |
| [bitcode 0.6.6][bitcode] | 100.00% | 60.71% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 22.60% | 41.93% | † | 73.37% | 85.79% | 86.75% | 36.10% |
| [capnp 0.23.2][capnp] | 28.78% | † | † | 40.76% | 59.88% | 64.84% | 20.77% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 17.08% | 16.75% | 5.18% | 29.53% | 58.29% | 66.36% | 21.45% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.51% | 7.53% | † | 29.53% | 58.29% | 66.35% | 21.62% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.79% | 16.54% | 5.26% | 29.53% | 58.29% | 66.35% | 21.51% |
| [columnar 0.11.0][columnar] | 42.53% | 39.82% <span title="copy_from">*100.00%\**</span> | † | 58.13% | 80.48% | 83.67% | 46.25% |
| [databuf 0.5.0][databuf] | 42.80% | 43.40% | 22.28% | 91.97% | 94.31% | 91.75% | 38.17% |
| [dlhn 0.1.7][dlhn] | 16.32% | 29.70% | † | 89.41% | 91.09% | 88.55% | 36.86% |
| [flatbuffers 25.12.19][flatbuffers] | 3.90% | † | † | 38.58% | 57.77% | 61.74% | 20.90% |
| [flexbuffers 25.2.10][flexbuffers] | 1.62% | 11.31% | 3.21% | 27.59% | 36.04% | 32.88% | 11.91% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.48% | 11.14% | † | 20.19% | 43.07% | 50.69% | 13.12% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.70% | 16.18% | † | 20.19% | 43.07% | 50.69% | 13.13% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 12.80% | 26.82% | † | 83.75% | 84.83% | 82.60% | 34.35% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.20% | 24.99% | 10.04% | 77.19% | 81.95% | 80.52% | 32.52% |
| [minicbor 1.0.0][minicbor] | 22.47% | 22.68% | 9.28% | 76.42% | 80.42% | 79.62% | 32.63% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.49% | 17.85% | 5.65% | 72.86% | 79.60% | 78.82% | 32.47% |
| [nanoserde 0.2.1][nanoserde] | 48.36% | 40.73% | † | 57.69% | 83.75% | 78.51% | 30.43% |
| [nibblecode 0.1.0][nibblecode] | 63.78% | † | † | 54.26% | 46.47% | 44.44% | 20.82% |
| [postcard 1.1.1][postcard] | 28.49% | 38.29% | 21.59% | 89.17% | 90.55% | 87.84% | 36.81% |
| [pot 3.0.1][pot] | 5.35% | 12.16% | 3.42% | 54.69% | 67.17% | 73.50% | 27.56% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.81%\**</span> <span title="populate + encode">*4.23%\**</span> | 22.08% | † | 54.91% | 65.82% | 67.74% | 25.11% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.81%\**</span> <span title="populate + encode">*4.21%\**</span> | 19.58% | † | 54.91% | 65.82% | 67.74% | 24.55% |
| [rkyv 0.8.10][rkyv] | 38.77% | <span title="unvalidated">*51.17%\**</span> <span title="validated upfront with error">*41.44%\**</span> | † | 54.27% | 78.87% | 82.96% | 32.20% |
| [ron 0.10.1][ron] | 1.59% | 3.11% | 0.76% | 22.36% | 46.20% | 53.09% | 13.31% |
| [savefile 0.18.6][savefile] | 59.32% | 41.68% | † | 57.79% | 83.95% | 78.64% | 30.56% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.08% | 36.60% | † | 91.97% | 94.35% | 91.74% | 37.43% |
| [serde-brief 0.1.1][serde-brief] | 9.63% | 14.17% | 4.68% | 25.68% | 53.74% | 62.05% | 19.91% |
| [serde_bare 0.5.0][serde_bare] | 16.58% | 32.67% | † | 91.97% | 94.31% | 91.75% | 36.60% |
| [speedy 0.8.7][speedy] | 47.50% | 45.52% | 31.99% | 72.89% | 85.52% | 86.61% | 34.77% |
| [wincode 0.2.4][wincode] | 62.91% | 45.45% | 27.67% | 57.80% | 83.96% | 78.64% | 30.28% |
| [wiring 0.2.4][wiring] | 60.47% | 41.36% | † | 57.80% | 81.09% | 80.88% | 27.25% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.46%\**</span> | <span title="validated on-demand with error">*42.10%\**</span> | ‡ |
| [columnar 0.11.0][columnar] | 0.15% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.98%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.94%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*97.11%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.4998 ms\**</span> <span title="prepend">*2.6366 ms\**</span> | 8.4280 ms | 1704643 | 1294259 | 1245668 | 11.738 ms |
| [bin-proto 0.12.3][bin-proto] | 5.5657 ms | 6.8226 ms | 1791489 | 1127998 | 1051146 | 10.351 ms |
| [bincode 2.0.1][bincode] | 1.4125 ms | 3.9669 ms | 1406257 | 1117802 | 1062438 | 9.6970 ms |
| [bincode 1.3.3][bincode1] | 3.9421 ms | 4.1412 ms | 1854234 | 1141994 | 1048745 | 10.561 ms |
| [bitcode 0.6.6][bitcode] | 735.68 µs | 2.3452 ms | 971318 | 878034 | 850340 | 2.9480 ms |
| [borsh 1.5.7][borsh] | 2.9380 ms | 2.8547 ms | 1521989 | 1108471 | 1038528 | 10.050 ms |
| [capnp 0.23.2][capnp] | 2.1641 ms | † | 2724288 | 1546992 | 1239111 | 15.328 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.0371 ms | 18.049 ms | 6012539 | 1695215 | 1464951 | 21.945 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 24.228 ms | 55.229 ms | 6012373 | 1695146 | 1465025 | 21.789 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.004 ms | 21.202 ms | 6012373 | 1695146 | 1465025 | 21.714 ms |
| [columnar 0.11.0][columnar] | 903.62 µs | 3.6607 ms <span title="copy_from">*1.2286 ms\**</span> | 1544752 | 996728 | 897073 | 4.8218 ms |
| [databuf 0.5.0][databuf] | 1.3064 ms | 3.7893 ms | 1319999 | 1062631 | 1008334 | 9.1485 ms |
| [dlhn 0.1.7][dlhn] | 4.9232 ms | 6.6493 ms | 1311281 | 1077520 | 1046095 | 8.6285 ms |
| [flatbuffers 25.12.19][flatbuffers] | 5.3014 ms | † | 2325620 | 1439185 | 1268060 | 13.802 ms |
| [flexbuffers 25.2.10][flexbuffers] | 39.864 ms | 35.262 ms | 5352680 | 2658295 | 2777967 | 36.096 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.990 ms | 31.735 ms | 9390461 | 2391679 | 1842767 | 35.696 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 12.100 ms | 26.755 ms | 9390461 | 2391679 | 1842767 | 35.296 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.2214 ms | 6.1165 ms | 1458773 | 1156055 | 1137788 | 9.7393 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.339 ms | 10.783 ms | 1745322 | 1261627 | 1228923 | 11.930 ms |
| [minicbor 1.0.0][minicbor] | 2.4996 ms | 11.452 ms | 1777386 | 1276218 | 1252558 | 12.972 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.835 ms | 20.642 ms | 1770060 | 1277755 | 1263362 | 12.443 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2876 ms | 2.7974 ms | 1812404 | 1134820 | 1053109 | 10.566 ms |
| [nibblecode 0.1.0][nibblecode] | 504.64 µs | † | 2075936 | 1368001 | 1224891 | 13.609 ms |
| [postcard 1.1.1][postcard] | 1.8575 ms | 4.2005 ms | 1311281 | 1083900 | 1041434 | 8.8015 ms |
| [pot 3.0.1][pot] | 13.956 ms | 30.299 ms | 2604812 | 1482233 | 1298928 | 16.129 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.4714 ms\**</span> <span title="populate + encode">*9.3817 ms\**</span> | 8.7063 ms | 1859886 | 1338076 | 1295351 | 12.481 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.7536 ms\**</span> <span title="populate + encode">*13.081 ms\**</span> | 11.963 ms | 1859886 | 1338076 | 1295351 | 12.398 ms |
| [rkyv 0.8.10][rkyv] | 998.26 µs | <span title="unvalidated">*2.1668 ms\**</span> <span title="validated upfront with error">*2.6365 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.046 ms |
| [ron 0.10.1][ron] | 41.843 ms | 151.46 ms | 8677703 | 2233642 | 1826180 | 34.864 ms |
| [savefile 0.18.6][savefile] | 870.09 µs | 2.7635 ms | 1791505 | 1128012 | 1051153 | 10.534 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1786 ms | 3.3341 ms | 1319999 | 1064380 | 1010708 | 8.8414 ms |
| [serde-brief 0.1.1][serde-brief] | 6.6687 ms | 21.684 ms | 6951772 | 1796265 | 1567819 | 24.760 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8304 ms | 4.8528 ms | 1319999 | 1062645 | 1008349 | 9.0938 ms |
| [speedy 0.8.7][speedy] | 773.86 µs | 2.4983 ms | 1584734 | 1119837 | 1037992 | 10.147 ms |
| [wincode 0.2.4][wincode] | 566.05 µs | 2.3606 ms | 1791489 | 1127998 | 1051146 | 10.361 ms |
| [wiring 0.2.4][wiring] | 641.68 µs | 2.8557 ms | 1791489 | 1156963 | 1082815 | 10.941 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*85.396 ns\**</span> | <span title="validated on-demand with error">*497.47 ns\**</span> | ‡ |
| [columnar 0.11.0][columnar] | 52.846 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4863 ns\**</span> <span title="validated upfront with error">*5.5726 ms\**</span> | <span title="unvalidated">*2.7359 µs\**</span> <span title="validated upfront with error">*5.5654 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2427 ns\**</span> <span title="validated upfront with error">*341.53 µs\**</span> | <span title="unvalidated">*375.79 ns\**</span> <span title="validated upfront with error">*341.61 µs\**</span> | <span title="unvalidated">*237.23 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*447.77 µs\**</span> | <span title="unvalidated">*384.67 ns\**</span> <span title="validated upfront with error">*447.92 µs\**</span> | <span title="unvalidated">*234.78 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.21%\**</span> <span title="prepend">*19.14%\**</span> | 14.58% | 56.98% | 67.84% | 68.26% | 25.12% |
| [bin-proto 0.12.3][bin-proto] | 9.07% | 18.01% | 54.22% | 77.84% | 80.90% | 28.48% |
| [bincode 2.0.1][bincode] | 35.73% | 30.97% | 69.07% | 78.55% | 80.04% | 30.40% |
| [bincode 1.3.3][bincode1] | 12.80% | 29.67% | 52.38% | 76.89% | 81.08% | 27.91% |
| [bitcode 0.6.6][bitcode] | 68.60% | 52.39% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 17.18% | 43.04% | 63.82% | 79.21% | 81.88% | 29.33% |
| [capnp 0.23.2][capnp] | 23.32% | † | 35.65% | 56.76% | 68.63% | 19.23% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.62% | 6.81% | 16.15% | 51.79% | 58.05% | 13.43% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.08% | 2.22% | 16.16% | 51.80% | 58.04% | 13.53% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.04% | 5.79% | 16.16% | 51.80% | 58.04% | 13.58% |
| [columnar 0.11.0][columnar] | 55.85% | 33.56% <span title="copy_from">*100.00%\**</span> | 62.88% | 88.09% | 94.79% | 61.14% |
| [databuf 0.5.0][databuf] | 38.63% | 32.42% | 73.58% | 82.63% | 84.33% | 32.22% |
| [dlhn 0.1.7][dlhn] | 10.25% | 18.48% | 74.07% | 81.49% | 81.29% | 34.17% |
| [flatbuffers 25.12.19][flatbuffers] | 9.52% | † | 41.77% | 61.01% | 67.06% | 21.36% |
| [flexbuffers 25.2.10][flexbuffers] | 1.27% | 3.48% | 18.15% | 33.03% | 30.61% | 8.17% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.40% | 3.87% | 10.34% | 36.71% | 46.14% | 8.26% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.17% | 4.59% | 10.34% | 36.71% | 46.14% | 8.35% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 22.72% | 20.09% | 66.58% | 75.95% | 74.74% | 30.27% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 4.88% | 11.39% | 55.65% | 69.60% | 69.19% | 24.71% |
| [minicbor 1.0.0][minicbor] | 20.19% | 10.73% | 54.65% | 68.80% | 67.89% | 22.73% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.64% | 5.95% | 54.87% | 68.72% | 67.31% | 23.69% |
| [nanoserde 0.2.1][nanoserde] | 39.19% | 43.92% | 53.59% | 77.37% | 80.75% | 27.90% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 64.18% | 69.42% | 21.66% |
| [postcard 1.1.1][postcard] | 27.17% | 29.25% | 74.07% | 81.01% | 81.65% | 33.49% |
| [pot 3.0.1][pot] | 3.62% | 4.05% | 37.29% | 59.24% | 65.46% | 18.28% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.22%\**</span> <span title="populate + encode">*5.38%\**</span> | 14.11% | 52.22% | 65.62% | 65.65% | 23.62% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*8.77%\**</span> <span title="populate + encode">*3.86%\**</span> | 10.27% | 52.22% | 65.62% | 65.65% | 23.78% |
| [rkyv 0.8.10][rkyv] | 50.55% | <span title="unvalidated">*56.70%\**</span> <span title="validated upfront with error">*46.60%\**</span> | 46.79% | 63.45% | 70.25% | 22.60% |
| [ron 0.10.1][ron] | 1.21% | 0.81% | 11.19% | 39.31% | 46.56% | 8.46% |
| [savefile 0.18.6][savefile] | 58.00% | 44.46% | 54.22% | 77.84% | 80.90% | 27.99% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 15.88% | 36.85% | 73.58% | 82.49% | 84.13% | 33.34% |
| [serde-brief 0.1.1][serde-brief] | 7.57% | 5.67% | 13.97% | 48.88% | 54.24% | 11.91% |
| [serde_bare 0.5.0][serde_bare] | 10.45% | 25.32% | 73.58% | 82.63% | 84.33% | 32.42% |
| [speedy 0.8.7][speedy] | 65.21% | 49.18% | 61.29% | 78.41% | 81.92% | 29.05% |
| [wincode 0.2.4][wincode] | 89.15% | 52.05% | 54.22% | 77.84% | 80.90% | 28.45% |
| [wiring 0.2.4][wiring] | 78.64% | 43.02% | 54.22% | 75.89% | 78.53% | 26.95% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.46%\**</span> | <span title="validated on-demand with error">*75.54%\**</span> | ‡ |
| [columnar 0.11.0][columnar] | 2.35% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.74%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*98.97%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.69%\**</span> <span title="validated upfront with error">*0.08%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1013.0
[bin-proto]: https://crates.io/crates/bin-proto/0.12.3
[bincode]: https://crates.io/crates/bincode/2.0.1
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.6
[borsh]: https://crates.io/crates/borsh/1.5.7
[capnp]: https://crates.io/crates/capnp/0.23.2
[cbor4ii]: https://crates.io/crates/cbor4ii/1.0.0
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[columnar]: https://crates.io/crates/columnar/0.11.0
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.12.19
[flexbuffers]: https://crates.io/crates/flexbuffers/25.2.10
[minicbor]: https://crates.io/crates/minicbor/1.0.0
[msgpacker]: https://crates.io/crates/msgpacker/0.4.8
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.2.1
[nibblecode]: https://crates.io/crates/nibblecode/0.1.0
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.7.5
[postcard]: https://crates.io/crates/postcard/1.1.1
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.14.1
[protobuf]: https://crates.io/crates/protobuf/3.7.2
[rkyv]: https://crates.io/crates/rkyv/0.8.10
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.10.1
[savefile]: https://crates.io/crates/savefile/0.18.6
[serde-brief]: https://crates.io/crates/serde-brief/0.1.1
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.140
[simd-json]: https://crates.io/crates/simd-json/0.15.1
[speedy]: https://crates.io/crates/speedy/0.8.7
[wincode]: https://crates.io/crates/wincode/0.2.4
[wiring]: https://crates.io/crates/wiring/0.2.4


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
