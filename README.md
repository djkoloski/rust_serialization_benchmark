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

## Last updated: 2026-06-23 23:21:31

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.98.0-nightly (4429659e4 2026-06-22)
binary: rustc
commit-hash: 4429659e4745016bd3f26a4a421843edc7fbc422
commit-date: 2026-06-22
host: x86_64-unknown-linux-gnu
release: 1.98.0-nightly
LLVM version: 22.1.7
```

### CPU info

```
Architecture:                            x86_64
CPU op-mode(s):                          32-bit, 64-bit
Address sizes:                           48 bits physical, 48 bits virtual
Byte Order:                              Little Endian
CPU(s):                                  4
On-line CPU(s) list:                     0-3
Vendor ID:                               AuthenticAMD
Model name:                              AMD EPYC 7763 64-Core Processor
CPU family:                              25
Model:                                   1
Thread(s) per core:                      2
Core(s) per socket:                      2
Socket(s):                               1
Stepping:                                1
BogoMIPS:                                4890.85
Flags:                                   fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves user_shstk clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
Virtualization:                          AMD-V
Hypervisor vendor:                       Microsoft
Virtualization type:                     full
L1d cache:                               64 KiB (2 instances)
L1i cache:                               64 KiB (2 instances)
L2 cache:                                1 MiB (2 instances)
L3 cache:                                32 MiB (1 instance)
NUMA node(s):                            1
NUMA node0 CPU(s):                       0-3
Vulnerability Gather data sampling:      Not affected
Vulnerability Ghostwrite:                Not affected
Vulnerability Indirect target selection: Not affected
Vulnerability Itlb multihit:             Not affected
Vulnerability L1tf:                      Not affected
Vulnerability Mds:                       Not affected
Vulnerability Meltdown:                  Not affected
Vulnerability Mmio stale data:           Not affected
Vulnerability Old microcode:             Not affected
Vulnerability Reg file data sampling:    Not affected
Vulnerability Retbleed:                  Not affected
Vulnerability Spec rstack overflow:      Vulnerable: Safe RET, no microcode
Vulnerability Spec store bypass:         Vulnerable
Vulnerability Spectre v1:                Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:                Mitigation; Retpolines; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
Vulnerability Srbds:                     Not affected
Vulnerability Tsa:                       Vulnerable: No microcode
Vulnerability Tsx async abort:           Not affected
Vulnerability Vmscape:                   Not affected
```

</details>

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*432.24 µs\**</span> <span title="prepend">*418.10 µs\**</span> | 2.6497 ms | 870.86 µs | 804955 | 328941 | 284849 | 4.2155 ms |
| [bin-proto 0.12.8][bin-proto] | 4.4728 ms | 4.6875 ms | † | 1045784 | 373127 | 311553 | 4.6627 ms |
| [bincode 2.0.1][bincode] | 366.97 µs | 2.1850 ms | 701.96 µs | 741295 | 303944 | 256422 | 3.6435 ms |
| [bincode 1.3.3][bincode1] | 545.90 µs | 2.1148 ms | 599.94 µs | 1045784 | 373127 | 311553 | 4.5716 ms |
| [bitcode 0.6.9][bitcode] | 144.69 µs | 1.4604 ms | 63.377 µs | 703710 | 288826 | 227322 | 2.5130 ms |
| [borsh 1.7.0][borsh] | 550.92 µs | 2.0962 ms | † | 885780 | 362204 | 286248 | 4.3217 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 632.12 µs <span title="packed">*1.5162 ms\**</span> | <span title="packed">*1.0178 ms\**</span> | † | 1443216 <span title="packed">*1046865\**</span> | 513986 <span title="packed">*481681\**</span> | 426532 <span title="packed">*458024\**</span> | 6.1046 ms <span title="packed">*5.3536 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 617.17 µs | 5.1460 ms | 3.4235 ms | 1407835 | 403440 | 323561 | 5.0379 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1047 ms | 11.278 ms | † | 1407835 | 403440 | 323561 | 5.0278 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9120 ms | 4.6175 ms | 2.9117 ms | 1407835 | 403440 | 323561 | 4.9430 ms |
| [columnar 0.13.0][columnar] | 261.94 µs | 2.0994 ms <span title="copy_from">*738.59 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.2058 ms |
| [compactly 0.1.6][compactly] | 26.988 ms | 20.242 ms | † | 241251 | 241453 | 241263 | 88.907 µs |
| [databuf 0.5.0][databuf] | 263.86 µs | 2.0830 ms | 666.97 µs | 765778 | 311715 | 263914 | 3.5031 ms |
| [dlhn 0.1.7][dlhn] | 670.60 µs | 2.6072 ms | † | 724953 | 301446 | 253056 | 3.2227 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0498 ms | † | † | 1276368 | 468539 | 388381 | 4.7761 ms |
| [flexbuffers 25.12.19][flexbuffers] | 6.6691 ms | 7.0772 ms | 5.5147 ms | 1829756 | 714318 | 691541 | 8.5656 ms |
| json:<br> [flexon 0.4.6][flexon] | 2.7969 ms | 3.8763 ms | † | 1827461 | 470560 | 360727 | 5.5140 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 3.8062 ms | 5.8068 ms | † | 1827461 | 470560 | 360727 | 5.7398 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 2.1723 ms | 4.8239 ms | † | 1827461 | 470560 | 360727 | 5.5589 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 349.71 µs | 2.5757 ms | 912.04 µs | 764996 | 315291 | 264212 | 3.5547 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 1.5145 ms | 3.2267 ms | 1.4633 ms | 784997 | 325384 | 277608 | 3.8309 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 355.37 µs | 2.3224 ms | 817.97 µs | 784997 | 325384 | 277608 | 3.7533 ms |
| [minicbor 2.2.2][minicbor] | 687.31 µs | 3.0142 ms | 1.3542 ms | 817830 | 332671 | 284034 | 3.9945 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.7158 ms | 4.3919 ms | 2.5678 ms | 818669 | 332556 | 284797 | 4.1383 ms |
| [nanoserde 0.2.1][nanoserde] | 261.70 µs | 2.1210 ms | † | 1045784 | 373127 | 311553 | 4.2887 ms |
| [nibblecode 0.1.0][nibblecode] | 209.41 µs | † | † | 1011487 | 492697 | 426693 | 5.4494 ms |
| [postcard 1.1.3][postcard] | 420.09 µs | 2.3138 ms | 721.91 µs | 724953 | 302399 | 252968 | 3.2191 ms |
| [pot 3.0.1][pot] | 2.3999 ms | 6.4540 ms | 4.8546 ms | 971922 | 372513 | 303636 | 4.3699 ms |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*780.14 µs\**</span> <span title="populate + encode">*2.5437 ms\**</span> | 3.9855 ms | 2.5525 ms | 884628 | 363130 | 314959 | 4.4518 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*956.63 µs\**</span> <span title="populate + encode">*2.4677 ms\**</span> | 3.3696 ms | † | 884628 | 363130 | 314959 | 4.3842 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0064 ms\**</span> <span title="populate + encode">*2.8184 ms\**</span> | 4.0155 ms | † | 884628 | 363130 | 314959 | 4.3488 ms |
| [rkyv 0.8.16][rkyv] | 245.57 µs | <span title="unvalidated">*1.5548 ms\**</span> <span title="validated upfront with error">*1.9365 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5678 ms |
| [ron 0.12.2][ron] | 11.008 ms | 26.444 ms | 24.827 ms | 1607459 | 449158 | 349324 | 5.6489 ms |
| [savefile 0.20.4][savefile] | 195.71 µs | 2.0937 ms | † | 1045800 | 373139 | 311562 | 4.1783 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 648.41 µs | 2.3916 ms | † | 765778 | 311743 | 263822 | 3.5354 ms |
| [serde-brief 0.2.0][serde-brief] | 1.3996 ms | 4.5831 ms | 2.8629 ms | 1584946 | 413733 | 339964 | 4.8980 ms |
| [serde_bare 0.5.0][serde_bare] | 702.17 µs | 2.1876 ms | † | 765778 | 311715 | 263914 | 3.4989 ms |
| [speedy 0.8.7][speedy] | 199.65 µs | 1.7755 ms | 367.82 µs | 885780 | 362204 | 286248 | 3.8437 ms |
| [wincode 0.5.5][wincode] | 170.56 µs | 1.7740 ms | 367.77 µs | 1045784 | 373127 | 311553 | 4.1643 ms |
| [wiring 0.2.4][wiring] | 195.54 µs | 2.1751 ms | † | 1045784 | 337930 | 275808 | 3.6468 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*78.683 ns\**</span> | <span title="validated on-demand with error">*143.54 µs\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 20.862 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4939 ns\**</span> <span title="validated upfront with error">*2.0609 ms\**</span> | <span title="unvalidated">*51.679 µs\**</span> <span title="validated upfront with error">*2.1532 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2442 ns\**</span> <span title="validated upfront with error">*234.76 µs\**</span> | <span title="unvalidated">*10.426 µs\**</span> <span title="validated upfront with error">*245.93 µs\**</span> | <span title="unvalidated">*8.4366 µs\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2461 ns\**</span> <span title="validated upfront with error">*369.74 µs\**</span> | <span title="unvalidated">*10.361 µs\**</span> <span title="validated upfront with error">*374.65 µs\**</span> | <span title="unvalidated">*7.4057 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*33.47%\**</span> <span title="prepend">*34.61%\**</span> | 27.87% | 7.28% | 29.97% | 73.40% | 79.80% | 2.11% |
| [bin-proto 0.12.8][bin-proto] | 3.23% | 15.76% | † | 23.07% | 64.71% | 72.96% | 1.91% |
| [bincode 2.0.1][bincode] | 39.43% | 33.80% | 9.03% | 32.54% | 79.44% | 88.65% | 2.44% |
| [bincode 1.3.3][bincode1] | 26.50% | 34.92% | 10.56% | 23.07% | 64.71% | 72.96% | 1.94% |
| [bitcode 0.6.9][bitcode] | 100.00% | 50.57% | 100.00% | 34.28% | 83.60% | 100.00% | 3.54% |
| [borsh 1.7.0][borsh] | 26.26% | 35.23% | † | 27.24% | 66.66% | 79.41% | 2.06% |
| capnp:<br> [capnp 0.26.0][capnp] | 22.89% <span title="packed">*9.54%\**</span> | <span title="packed">*72.57%\**</span> | † | 16.72% <span title="packed">*23.05%\**</span> | 46.98% <span title="packed">*50.13%\**</span> | 53.30% <span title="packed">*49.63%\**</span> | 1.46% <span title="packed">*1.66%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 23.44% | 14.35% | 1.85% | 17.14% | 59.85% | 70.26% | 1.76% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.66% | 6.55% | † | 17.14% | 59.85% | 70.26% | 1.77% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.57% | 16.00% | 2.18% | 17.14% | 59.85% | 70.26% | 1.80% |
| [columnar 0.13.0][columnar] | 55.24% | 35.18% <span title="copy_from">*100.00%\**</span> | † | 23.07% | 65.22% | 77.34% | 2.11% |
| [compactly 0.1.6][compactly] | 0.54% | 3.65% | † | 100.00% | 100.00% | 94.22% | 100.00% |
| [databuf 0.5.0][databuf] | 54.84% | 35.46% | 9.50% | 31.50% | 77.46% | 86.13% | 2.54% |
| [dlhn 0.1.7][dlhn] | 21.58% | 28.33% | † | 33.28% | 80.10% | 89.83% | 2.76% |
| [flatbuffers 25.12.19][flatbuffers] | 13.78% | † | † | 18.90% | 51.53% | 58.53% | 1.86% |
| [flexbuffers 25.12.19][flexbuffers] | 2.17% | 10.44% | 1.15% | 13.18% | 33.80% | 32.87% | 1.04% |
| json:<br> [flexon 0.4.6][flexon] | 5.17% | 19.05% | † | 13.20% | 51.31% | 63.02% | 1.61% |
| json:<br> [serde_json 1.0.150][serde_json] | 3.80% | 12.72% | † | 13.20% | 51.31% | 63.02% | 1.55% |
| json:<br> [simd-json 0.17.0][simd-json] | 6.66% | 15.31% | † | 13.20% | 51.31% | 63.02% | 1.60% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 41.37% | 28.68% | 6.95% | 31.54% | 76.58% | 86.04% | 2.50% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 9.55% | 22.89% | 4.33% | 30.73% | 74.21% | 81.89% | 2.32% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 40.72% | 31.80% | 7.75% | 30.73% | 74.21% | 81.89% | 2.37% |
| [minicbor 2.2.2][minicbor] | 21.05% | 24.50% | 4.68% | 29.50% | 72.58% | 80.03% | 2.23% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.53% | 16.82% | 2.47% | 29.47% | 72.61% | 79.82% | 2.15% |
| [nanoserde 0.2.1][nanoserde] | 55.29% | 34.82% | † | 23.07% | 64.71% | 72.96% | 2.07% |
| [nibblecode 0.1.0][nibblecode] | 69.09% | † | † | 23.85% | 49.01% | 53.28% | 1.63% |
| [postcard 1.1.3][postcard] | 34.44% | 31.92% | 8.78% | 33.28% | 79.85% | 89.86% | 2.76% |
| [pot 3.0.1][pot] | 6.03% | 11.44% | 1.31% | 24.82% | 64.82% | 74.87% | 2.03% |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*18.55%\**</span> <span title="populate + encode">*5.69%\**</span> | 18.53% | 2.48% | 27.27% | 66.49% | 72.18% | 2.00% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*15.12%\**</span> <span title="populate + encode">*5.86%\**</span> | 21.92% | † | 27.27% | 66.49% | 72.18% | 2.03% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.38%\**</span> <span title="populate + encode">*5.13%\**</span> | 18.39% | † | 27.27% | 66.49% | 72.18% | 2.04% |
| [rkyv 0.8.16][rkyv] | 58.92% | <span title="unvalidated">*47.50%\**</span> <span title="validated upfront with error">*38.14%\**</span> | † | 23.85% | 61.36% | 69.74% | 1.95% |
| [ron 0.12.2][ron] | 1.31% | 2.79% | 0.26% | 15.01% | 53.76% | 65.07% | 1.57% |
| [savefile 0.20.4][savefile] | 73.93% | 35.28% | † | 23.07% | 64.71% | 72.96% | 2.13% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.31% | 30.88% | † | 31.50% | 77.45% | 86.16% | 2.51% |
| [serde-brief 0.2.0][serde-brief] | 10.34% | 16.12% | 2.21% | 15.22% | 58.36% | 66.87% | 1.82% |
| [serde_bare 0.5.0][serde_bare] | 20.61% | 33.76% | † | 31.50% | 77.46% | 86.13% | 2.54% |
| [speedy 0.8.7][speedy] | 72.47% | 41.60% | 17.23% | 27.24% | 66.66% | 79.41% | 2.31% |
| [wincode 0.5.5][wincode] | 84.83% | 41.63% | 17.23% | 23.07% | 64.71% | 72.96% | 2.13% |
| [wiring 0.2.4][wiring] | 74.00% | 33.96% | † | 23.07% | 71.45% | 82.42% | 2.44% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.58%\**</span> | <span title="validated on-demand with error">*7.22%\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 5.96% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.89%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.05%\**</span> <span title="validated upfront with error">*0.48%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.38%\**</span> <span title="validated upfront with error">*4.21%\**</span> | <span title="unvalidated">*87.78%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.85%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.77%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*7.2616 ms\**</span> <span title="prepend">*8.6585 ms\**</span> | 7.7282 ms | 8625005 | 6443961 | 6231572 | 72.656 ms |
| [bin-proto 0.12.8][bin-proto] | 8.3131 ms | 10.542 ms | 6000008 | 5378500 | 5346908 | 8.4022 ms |
| [bincode 2.0.1][bincode] | 2.4192 ms | 1.0467 ms | 6000005 | 5378497 | 5346882 | 8.5104 ms |
| [bincode 1.3.3][bincode1] | 5.8821 ms | 4.6733 ms | 6000008 | 5378500 | 5346908 | 8.4524 ms |
| [bitcode 0.6.9][bitcode] | 1.3140 ms | 811.67 µs | 6000006 | 5182295 | 4921841 | 13.741 ms |
| [borsh 1.7.0][borsh] | 6.1369 ms | 4.2911 ms | 6000004 | 5378496 | 5346866 | 8.3703 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 9.0022 ms <span title="packed">*20.174 ms\**</span> | <span title="packed">*13.431 ms\**</span> | 14000088 <span title="packed">*10401737\**</span> | 7130367 <span title="packed">*7308001\**</span> | 6046182 <span title="packed">*7922110\**</span> | 81.855 ms <span title="packed">*67.175 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 8.7641 ms | 44.669 ms | 13125016 | 7524114 | 6757437 | 91.947 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 66.557 ms | 115.21 ms | 13122324 | 7524660 | 6759128 | 92.747 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 34.038 ms | 38.515 ms | 13122324 | 7524660 | 6759128 | 91.640 ms |
| [columnar 0.13.0][columnar] | 1.7914 ms | 1.4252 ms <span title="copy_from">*669.93 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.3852 ms |
| [compactly 0.1.6][compactly] | 355.10 ms | 280.92 ms | 4846786 | 4850065 | 4846903 | 1.6248 ms |
| [databuf 0.5.0][databuf] | 2.4196 ms | 5.3213 ms | 6000003 | 5378495 | 5346897 | 8.5129 ms |
| [dlhn 0.1.7][dlhn] | 5.9212 ms | 6.8664 ms | 6000003 | 5378495 | 5346897 | 8.4857 ms |
| [flatbuffers 25.12.19][flatbuffers] | 468.11 µs | † | 6000024 | 5378434 | 5346878 | 8.4727 ms |
| [flexbuffers 25.12.19][flexbuffers] | 104.44 ms | 74.116 ms | 26609424 | 11901040 | 12486322 | 151.66 ms |
| json:<br> [flexon 0.4.6][flexon] | 77.077 ms | 55.800 ms | 26192883 | 9566084 | 8584671 | 155.27 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 93.472 ms | 100.17 ms | 26192883 | 9566084 | 8584671 | 156.28 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 53.028 ms | 67.579 ms | 26192883 | 9566084 | 8584671 | 155.68 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 665.51 µs | 5.0285 ms | 7500005 | 6058442 | 6014500 | 10.342 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 19.555 ms | 16.872 ms | 8125006 | 6494876 | 6391037 | 70.242 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 734.00 µs | 5.2088 ms | 8125006 | 6494876 | 6391037 | 69.968 ms |
| [minicbor 2.2.2][minicbor] | 6.0636 ms | 11.383 ms | 8125006 | 6494907 | 6390894 | 70.655 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 120.29 ms | 26.033 ms | 8125037 | 6493484 | 6386940 | 71.122 ms |
| [nanoserde 0.2.1][nanoserde] | 1.7671 ms | 898.82 µs | 6000008 | 5378500 | 5346908 | 8.6134 ms |
| [nibblecode 0.1.0][nibblecode] | 203.29 µs | † | 6000008 | 5378500 | 5346908 | 8.7111 ms |
| [postcard 1.1.3][postcard] | 481.27 µs | 6.4915 ms | 6000003 | 5378495 | 5346897 | 8.4933 ms |
| [pot 3.0.1][pot] | 37.553 ms | 71.083 ms | 10122342 | 6814618 | 6852252 | 82.934 ms |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*8.9260 ms\**</span> <span title="populate + encode">*26.026 ms\**</span> | 26.193 ms | 8750000 | 6665735 | 6421877 | 74.596 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*7.8197 ms\**</span> <span title="populate + encode">*8.6373 ms\**</span> | 14.700 ms | 8750000 | 6665735 | 6421877 | 73.716 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*16.731 ms\**</span> <span title="populate + encode">*33.339 ms\**</span> | 31.753 ms | 8750000 | 6665735 | 6421877 | 76.596 ms |
| [rkyv 0.8.16][rkyv] | 202.56 µs | <span title="unvalidated">*200.61 µs\**</span> <span title="validated upfront with error">*200.81 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.4964 ms |
| [ron 0.12.2][ron] | 172.91 ms | 586.88 ms | 22192885 | 8970395 | 8137334 | 150.92 ms |
| [savefile 0.20.4][savefile] | 201.27 µs | 202.69 µs | 6000024 | 5378519 | 5346896 | 8.6293 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 6.0229 ms | 4.0136 ms | 6000004 | 5378496 | 5346866 | 8.7953 ms |
| [serde-brief 0.2.0][serde-brief] | 17.345 ms | 35.710 ms | 15750015 | 8024540 | 6813667 | 94.558 ms |
| [serde_bare 0.5.0][serde_bare] | 6.0013 ms | 4.1980 ms | 6000003 | 5378495 | 5346897 | 8.7016 ms |
| [speedy 0.8.7][speedy] | 199.69 µs | 200.58 µs | 6000004 | 5378496 | 5346866 | 8.5121 ms |
| [wincode 0.5.5][wincode] | 202.08 µs | 200.35 µs | 6000008 | 5378500 | 5346908 | 8.6340 ms |
| [wiring 0.2.4][wiring] | 202.61 µs | 338.04 µs | 6000008 | 5378952 | 5346905 | 8.6051 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*111.13 ns\**</span> | <span title="validated on-demand with error">*2.2198 ms\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 20.768 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4973 ns\**</span> <span title="validated upfront with error">*44.469 ns\**</span> | <span title="unvalidated">*77.837 µs\**</span> <span title="validated upfront with error">*77.885 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2479 ns\**</span> <span title="validated upfront with error">*1.5580 ns\**</span> | <span title="unvalidated">*38.924 µs\**</span> <span title="validated upfront with error">*38.928 µs\**</span> | <span title="unvalidated">*100.02 µs\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2593 ns\**</span> <span title="validated upfront with error">*5.6072 ns\**</span> | <span title="unvalidated">*38.897 µs\**</span> <span title="validated upfront with error">*38.948 µs\**</span> | <span title="unvalidated">*100.02 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*2.75%\**</span> <span title="prepend">*2.31%\**</span> | 2.59% | 56.19% | 75.27% | 77.78% | 2.24% |
| [bin-proto 0.12.8][bin-proto] | 2.40% | 1.90% | 80.78% | 90.18% | 90.65% | 19.34% |
| [bincode 2.0.1][bincode] | 8.25% | 19.14% | 80.78% | 90.18% | 90.65% | 19.09% |
| [bincode 1.3.3][bincode1] | 3.39% | 4.29% | 80.78% | 90.18% | 90.65% | 19.22% |
| [bitcode 0.6.9][bitcode] | 15.20% | 24.68% | 80.78% | 93.59% | 98.48% | 11.82% |
| [borsh 1.7.0][borsh] | 3.25% | 4.67% | 80.78% | 90.18% | 90.65% | 19.41% |
| capnp:<br> [capnp 0.26.0][capnp] | 2.22% <span title="packed">*0.99%\**</span> | <span title="packed">*1.49%\**</span> | 34.62% <span title="packed">*46.60%\**</span> | 68.02% <span title="packed">*66.37%\**</span> | 80.16% <span title="packed">*61.18%\**</span> | 1.98% <span title="packed">*2.42%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 2.28% | 0.45% | 36.93% | 64.46% | 71.73% | 1.77% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.30% | 0.17% | 36.94% | 64.46% | 71.71% | 1.75% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.59% | 0.52% | 36.94% | 64.46% | 71.71% | 1.77% |
| [columnar 0.13.0][columnar] | 11.15% | 14.06% <span title="copy_from">*29.91%\**</span> | 80.78% | 90.18% | 90.65% | 19.38% |
| [compactly 0.1.6][compactly] | 0.06% | 0.07% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 8.25% | 3.77% | 80.78% | 90.18% | 90.65% | 19.09% |
| [dlhn 0.1.7][dlhn] | 3.37% | 2.92% | 80.78% | 90.18% | 90.65% | 19.15% |
| [flatbuffers 25.12.19][flatbuffers] | 42.66% | † | 80.78% | 90.18% | 90.65% | 19.18% |
| [flexbuffers 25.12.19][flexbuffers] | 0.19% | 0.27% | 18.21% | 40.75% | 38.82% | 1.07% |
| json:<br> [flexon 0.4.6][flexon] | 0.26% | 0.36% | 18.50% | 50.70% | 56.46% | 1.05% |
| json:<br> [serde_json 1.0.150][serde_json] | 0.21% | 0.20% | 18.50% | 50.70% | 56.46% | 1.04% |
| json:<br> [simd-json 0.17.0][simd-json] | 0.38% | 0.30% | 18.50% | 50.70% | 56.46% | 1.04% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 30.01% | 3.98% | 64.62% | 80.05% | 80.59% | 15.71% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 1.02% | 1.19% | 59.65% | 74.68% | 75.84% | 2.31% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 27.21% | 3.85% | 59.65% | 74.68% | 75.84% | 2.32% |
| [minicbor 2.2.2][minicbor] | 3.29% | 1.76% | 59.65% | 74.67% | 75.84% | 2.30% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.17% | 0.77% | 59.65% | 74.69% | 75.89% | 2.28% |
| [nanoserde 0.2.1][nanoserde] | 11.30% | 22.29% | 80.78% | 90.18% | 90.65% | 18.86% |
| [nibblecode 0.1.0][nibblecode] | 98.23% | † | 80.78% | 90.18% | 90.65% | 18.65% |
| [postcard 1.1.3][postcard] | 41.49% | 3.09% | 80.78% | 90.18% | 90.65% | 19.13% |
| [pot 3.0.1][pot] | 0.53% | 0.28% | 47.88% | 71.17% | 70.73% | 1.96% |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*2.24%\**</span> <span title="populate + encode">*0.77%\**</span> | 0.76% | 55.39% | 72.76% | 75.47% | 2.18% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*2.55%\**</span> <span title="populate + encode">*2.31%\**</span> | 1.36% | 55.39% | 72.76% | 75.47% | 2.20% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.19%\**</span> <span title="populate + encode">*0.60%\**</span> | 0.63% | 55.39% | 72.76% | 75.47% | 2.12% |
| [rkyv 0.8.16][rkyv] | 98.58% | <span title="unvalidated">*99.87%\**</span> <span title="validated upfront with error">*99.77%\**</span> | 80.78% | 90.18% | 90.65% | 19.12% |
| [ron 0.12.2][ron] | 0.12% | 0.03% | 21.84% | 54.07% | 59.56% | 1.08% |
| [savefile 0.20.4][savefile] | 99.21% | 98.85% | 80.78% | 90.17% | 90.65% | 18.83% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.32% | 4.99% | 80.78% | 90.18% | 90.65% | 18.47% |
| [serde-brief 0.2.0][serde-brief] | 1.15% | 0.56% | 30.77% | 60.44% | 71.14% | 1.72% |
| [serde_bare 0.5.0][serde_bare] | 3.33% | 4.77% | 80.78% | 90.18% | 90.65% | 18.67% |
| [speedy 0.8.7][speedy] | 100.00% | 99.89% | 80.78% | 90.18% | 90.65% | 19.09% |
| [wincode 0.5.5][wincode] | 98.82% | 100.00% | 80.78% | 90.18% | 90.65% | 18.82% |
| [wiring 0.2.4][wiring] | 98.56% | 59.27% | 80.78% | 90.17% | 90.65% | 18.88% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.12%\**</span> | <span title="validated on-demand with error">*1.75%\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 6.01% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*2.81%\**</span> | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*49.94%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*80.10%\**</span> | <span title="unvalidated">*99.93%\**</span> <span title="validated upfront with error">*99.92%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.09%\**</span> <span title="validated upfront with error">*22.26%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.87%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*886.29 µs\**</span> <span title="prepend">*792.54 µs\**</span> | 3.1283 ms | 1.6842 ms | 489348 | 281173 | 249360 | 2.6385 ms |
| [bin-proto 0.12.8][bin-proto] | 1.8645 ms | 2.8880 ms | † | 566975 | 239350 | 231475 | 2.4704 ms |
| [bincode 2.0.1][bincode] | 343.85 µs | 1.8191 ms | 796.93 µs | 367413 | 221291 | 206242 | 2.0425 ms |
| [bincode 1.3.3][bincode1] | 591.09 µs | 1.8565 ms | 859.81 µs | 569975 | 240525 | 231884 | 2.6575 ms |
| [bitcode 0.6.9][bitcode] | 124.16 µs | 1.2722 ms | 171.43 µs | 327688 | 200947 | 182040 | 753.87 µs |
| [borsh 1.7.0][borsh] | 553.12 µs | 1.7929 ms | † | 446595 | 234236 | 209834 | 2.1207 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 511.40 µs <span title="packed">*1.0501 ms\**</span> | <span title="packed">*574.01 µs\**</span> | † | 803896 <span title="packed">*489017\**</span> | 335606 <span title="packed">*293127\**</span> | 280744 <span title="packed">*271528\**</span> | 3.5776 ms <span title="packed">*2.5830 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 724.42 µs | 4.4789 ms | 3.2165 ms | 1109831 | 344745 | 274333 | 3.4609 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.8343 ms | 10.151 ms | † | 1109821 | 344751 | 274345 | 3.5257 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8874 ms | 4.6236 ms | 3.1803 ms | 1109821 | 344751 | 274345 | 3.4367 ms |
| [columnar 0.13.0][columnar] | 258.93 µs | 1.8942 ms <span title="copy_from">*670.55 µs\**</span> | † | 563392 | 249619 | 217632 | 1.6062 ms |
| [compactly 0.1.6][compactly] | 11.623 ms | 11.327 ms | † | 149292 | 149433 | 149304 | 70.883 µs |
| [databuf 0.5.0][databuf] | 286.77 µs | 1.7209 ms | 806.12 µs | 356311 | 213062 | 198403 | 1.9532 ms |
| [dlhn 0.1.7][dlhn] | 697.08 µs | 2.6783 ms | † | 366496 | 220600 | 205586 | 2.0514 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2893 ms | † | † | 849472 | 347816 | 294871 | 3.4778 ms |
| [flexbuffers 25.12.19][flexbuffers] | 7.8213 ms | 6.7474 ms | 5.4038 ms | 1187688 | 557642 | 553730 | 6.1945 ms |
| json:<br> [flexon 0.4.6][flexon] | 2.7867 ms | 4.5392 ms | † | 1623191 | 466527 | 359157 | 5.7503 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 3.7005 ms | 6.8670 ms | † | 1623191 | 466527 | 359157 | 5.7125 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 2.2127 ms | 4.5456 ms | † | 1623191 | 466527 | 359157 | 5.7530 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 330.11 µs | 2.8321 ms | 1.3065 ms | 391251 | 236877 | 220395 | 2.1740 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 1.4690 ms | 3.0050 ms | 1.7062 ms | 424533 | 245214 | 226077 | 2.3069 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 349.92 µs | 2.1119 ms | 874.21 µs | 416025 | 243812 | 224965 | 2.2610 ms |
| [minicbor 2.2.2][minicbor] | 565.63 µs | 3.4240 ms | 1.8323 ms | 428773 | 249857 | 228630 | 2.2607 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0522 ms | 3.8489 ms | 2.7165 ms | 449745 | 252432 | 230965 | 2.3112 ms |
| [nanoserde 0.2.1][nanoserde] | 270.56 µs | 1.9151 ms | † | 567975 | 239930 | 231872 | 2.4831 ms |
| [nibblecode 0.1.0][nibblecode] | 181.70 µs | † | † | 603928 | 427286 | 401852 | 3.5680 ms |
| [postcard 1.1.3][postcard] | 446.23 µs | 2.1014 ms | 812.54 µs | 367489 | 221913 | 207244 | 2.0223 ms |
| [pot 3.0.1][pot] | 2.3911 ms | 6.1875 ms | 4.8800 ms | 599125 | 299158 | 247675 | 2.7659 ms |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*1.1067 ms\**</span> <span title="populate + encode">*3.1633 ms\**</span> | 3.6092 ms | 2.5783 ms | 596811 | 305319 | 268737 | 3.0157 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*1.2945 ms\**</span> <span title="populate + encode">*3.0481 ms\**</span> | 3.6190 ms | † | 596811 | 305319 | 268737 | 3.0178 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.1051 ms\**</span> <span title="populate + encode">*3.0835 ms\**</span> | 3.9167 ms | † | 596811 | 305319 | 268737 | 3.0107 ms |
| [rkyv 0.8.16][rkyv] | 329.06 µs | <span title="unvalidated">*1.5094 ms\**</span> <span title="validated upfront with error">*1.8658 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3389 ms |
| [ron 0.12.2][ron] | 8.1181 ms | 27.493 ms | 25.959 ms | 1465223 | 434935 | 342907 | 5.5282 ms |
| [savefile 0.20.4][savefile] | 212.89 µs | 1.8605 ms | † | 566991 | 239362 | 231478 | 2.4446 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 620.17 µs | 2.1850 ms | † | 356311 | 212976 | 198423 | 1.9710 ms |
| [serde-brief 0.2.0][serde-brief] | 1.1851 ms | 5.0590 ms | 3.4351 ms | 1276014 | 373898 | 293384 | 3.6177 ms |
| [serde_bare 0.5.0][serde_bare] | 762.25 µs | 2.3569 ms | † | 356311 | 213062 | 198403 | 1.9551 ms |
| [speedy 0.8.7][speedy] | 271.17 µs | 1.6815 ms | 548.92 µs | 449595 | 234970 | 210192 | 2.0820 ms |
| [wincode 0.5.5][wincode] | 205.54 µs | 1.6518 ms | 552.80 µs | 566975 | 239350 | 231475 | 2.4611 ms |
| [wiring 0.2.4][wiring] | 204.83 µs | 1.9342 ms | † | 566975 | 247810 | 225086 | 2.5362 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*76.884 ns\**</span> | <span title="validated on-demand with error">*413.62 ns\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 673.89 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4890 ns\**</span> <span title="validated upfront with error">*2.1326 ms\**</span> | <span title="unvalidated">*1.4300 µs\**</span> <span title="validated upfront with error">*2.1329 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2442 ns\**</span> <span title="validated upfront with error">*249.18 µs\**</span> | <span title="unvalidated">*156.25 ns\**</span> <span title="validated upfront with error">*248.54 µs\**</span> | <span title="unvalidated">*741.58 ns\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2455 ns\**</span> <span title="validated upfront with error">*331.39 µs\**</span> | <span title="unvalidated">*156.29 ns\**</span> <span title="validated upfront with error">*334.76 µs\**</span> | <span title="unvalidated">*750.16 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*14.01%\**</span> <span title="prepend">*15.67%\**</span> | 18.35% | 10.18% | 30.51% | 53.15% | 59.87% | 2.69% |
| [bin-proto 0.12.8][bin-proto] | 6.66% | 19.88% | † | 26.33% | 62.43% | 64.50% | 2.87% |
| [bincode 2.0.1][bincode] | 36.11% | 31.55% | 21.51% | 40.63% | 67.53% | 72.39% | 3.47% |
| [bincode 1.3.3][bincode1] | 21.01% | 30.92% | 19.94% | 26.19% | 62.13% | 64.39% | 2.67% |
| [bitcode 0.6.9][bitcode] | 100.00% | 45.12% | 100.00% | 45.56% | 74.36% | 82.02% | 9.40% |
| [borsh 1.7.0][borsh] | 22.45% | 32.02% | † | 33.43% | 63.80% | 71.15% | 3.34% |
| capnp:<br> [capnp 0.26.0][capnp] | 24.28% <span title="packed">*11.82%\**</span> | <span title="packed">*100.00%\**</span> | † | 18.57% <span title="packed">*30.53%\**</span> | 44.53% <span title="packed">*50.98%\**</span> | 53.18% <span title="packed">*54.99%\**</span> | 1.98% <span title="packed">*2.74%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 17.14% | 12.82% | 5.33% | 13.45% | 43.35% | 54.42% | 2.05% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.24% | 5.65% | † | 13.45% | 43.35% | 54.42% | 2.01% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.58% | 12.41% | 5.39% | 13.45% | 43.35% | 54.42% | 2.06% |
| [columnar 0.13.0][columnar] | 47.95% | 30.30% <span title="copy_from">*85.60%\**</span> | † | 26.50% | 59.86% | 68.60% | 4.41% |
| [compactly 0.1.6][compactly] | 1.07% | 5.07% | † | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 43.30% | 33.36% | 21.27% | 41.90% | 70.14% | 75.25% | 3.63% |
| [dlhn 0.1.7][dlhn] | 17.81% | 21.43% | † | 40.73% | 67.74% | 72.62% | 3.46% |
| [flatbuffers 25.12.19][flatbuffers] | 3.77% | † | † | 17.57% | 42.96% | 50.63% | 2.04% |
| [flexbuffers 25.12.19][flexbuffers] | 1.59% | 8.51% | 3.17% | 12.57% | 26.80% | 26.96% | 1.14% |
| json:<br> [flexon 0.4.6][flexon] | 4.46% | 12.65% | † | 9.20% | 32.03% | 41.57% | 1.23% |
| json:<br> [serde_json 1.0.150][serde_json] | 3.36% | 8.36% | † | 9.20% | 32.03% | 41.57% | 1.24% |
| json:<br> [simd-json 0.17.0][simd-json] | 5.61% | 12.63% | † | 9.20% | 32.03% | 41.57% | 1.23% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 37.61% | 20.27% | 13.12% | 38.16% | 63.08% | 67.74% | 3.26% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 8.45% | 19.10% | 10.05% | 35.17% | 60.94% | 66.04% | 3.07% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 35.48% | 27.18% | 19.61% | 35.89% | 61.29% | 66.37% | 3.14% |
| [minicbor 2.2.2][minicbor] | 21.95% | 16.76% | 9.36% | 34.82% | 59.81% | 65.30% | 3.14% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.46% | 14.91% | 6.31% | 33.19% | 59.20% | 64.64% | 3.07% |
| [nanoserde 0.2.1][nanoserde] | 45.89% | 29.97% | † | 26.28% | 62.28% | 64.39% | 2.85% |
| [nibblecode 0.1.0][nibblecode] | 68.33% | † | † | 24.72% | 34.97% | 37.15% | 1.99% |
| [postcard 1.1.3][postcard] | 27.82% | 27.32% | 21.10% | 40.62% | 67.34% | 72.04% | 3.51% |
| [pot 3.0.1][pot] | 5.19% | 9.28% | 3.51% | 24.92% | 49.95% | 60.28% | 2.56% |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*11.22%\**</span> <span title="populate + encode">*3.93%\**</span> | 15.90% | 6.65% | 25.01% | 48.94% | 55.56% | 2.35% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*9.59%\**</span> <span title="populate + encode">*4.07%\**</span> | 15.86% | † | 25.01% | 48.94% | 55.56% | 2.35% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.24%\**</span> <span title="populate + encode">*4.03%\**</span> | 14.66% | † | 25.01% | 48.94% | 55.56% | 2.35% |
| [rkyv 0.8.16][rkyv] | 37.73% | <span title="unvalidated">*38.03%\**</span> <span title="validated upfront with error">*30.76%\**</span> | † | 24.73% | 58.65% | 68.04% | 3.03% |
| [ron 0.12.2][ron] | 1.53% | 2.09% | 0.66% | 10.19% | 34.36% | 43.54% | 1.28% |
| [savefile 0.20.4][savefile] | 58.32% | 30.85% | † | 26.33% | 62.43% | 64.50% | 2.90% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.02% | 26.27% | † | 41.90% | 70.16% | 75.25% | 3.60% |
| [serde-brief 0.2.0][serde-brief] | 10.48% | 11.35% | 4.99% | 11.70% | 39.97% | 50.89% | 1.96% |
| [serde_bare 0.5.0][serde_bare] | 16.29% | 24.35% | † | 41.90% | 70.14% | 75.25% | 3.63% |
| [speedy 0.8.7][speedy] | 45.79% | 34.14% | 31.23% | 33.21% | 63.60% | 71.03% | 3.40% |
| [wincode 0.5.5][wincode] | 60.41% | 34.75% | 31.01% | 26.33% | 62.43% | 64.50% | 2.88% |
| [wiring 0.2.4][wiring] | 60.62% | 29.68% | † | 26.33% | 60.30% | 66.33% | 2.79% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.62%\**</span> | <span title="validated on-demand with error">*37.78%\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 0.18% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.93%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.90%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*98.86%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*4.5478 ms\**</span> <span title="prepend">*2.5075 ms\**</span> | 8.4890 ms | 1704643 | 1294259 | 1245668 | 11.766 ms |
| [bin-proto 0.12.8][bin-proto] | 5.4802 ms | 6.6195 ms | 1791489 | 1127998 | 1051146 | 10.304 ms |
| [bincode 2.0.1][bincode] | 1.2837 ms | 3.9800 ms | 1406257 | 1117802 | 1062438 | 9.9487 ms |
| [bincode 1.3.3][bincode1] | 3.9313 ms | 4.4396 ms | 1854234 | 1141994 | 1048745 | 10.343 ms |
| [bitcode 0.6.9][bitcode] | 696.97 µs | 2.3623 ms | 971318 | 878034 | 850340 | 3.1593 ms |
| [borsh 1.7.0][borsh] | 2.9518 ms | 2.8682 ms | 1521989 | 1108471 | 1038528 | 10.011 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 2.6177 ms <span title="packed">*4.5615 ms\**</span> | <span title="packed">*1.9224 ms\**</span> | 2724288 <span title="packed">*1616255\**</span> | 1546992 <span title="packed">*1278764\**</span> | 1239111 <span title="packed">*1125654\**</span> | 14.726 ms <span title="packed">*9.2122 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 2.9147 ms | 16.622 ms | 6012539 | 1695215 | 1464951 | 22.053 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 24.352 ms | 54.163 ms | 6012373 | 1695146 | 1465025 | 21.230 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.095 ms | 20.268 ms | 6012373 | 1695146 | 1465025 | 22.956 ms |
| [columnar 0.13.0][columnar] | 873.31 µs | 3.8196 ms <span title="copy_from">*1.2481 ms\**</span> | 1544720 | 996718 | 896213 | 4.7925 ms |
| [compactly 0.1.6][compactly] | 64.451 ms | 56.588 ms | 802662 | 803238 | 802689 | 389.93 µs |
| [databuf 0.5.0][databuf] | 1.3218 ms | 3.8139 ms | 1319999 | 1062631 | 1008334 | 9.1080 ms |
| [dlhn 0.1.7][dlhn] | 4.4691 ms | 7.4255 ms | 1311281 | 1077520 | 1046095 | 8.9681 ms |
| [flatbuffers 25.12.19][flatbuffers] | 4.9273 ms | † | 2325620 | 1439185 | 1268060 | 13.762 ms |
| [flexbuffers 25.12.19][flexbuffers] | 39.991 ms | 35.422 ms | 5352680 | 2658295 | 2777967 | 35.696 ms |
| json:<br> [flexon 0.4.6][flexon] | 15.889 ms | 24.715 ms | 9390461 | 2391679 | 1842767 | 35.356 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 20.829 ms | 31.101 ms | 9390461 | 2391679 | 1842767 | 35.709 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 12.072 ms | 25.469 ms | 9390461 | 2391679 | 1842767 | 34.926 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 954.42 µs | 5.8468 ms | 1458773 | 1156055 | 1137788 | 10.139 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 10.381 ms | 11.424 ms | 1745322 | 1261627 | 1228923 | 12.004 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 1.1281 ms | 4.2142 ms | 1717696 | 1234725 | 1195988 | 11.238 ms |
| [minicbor 2.2.2][minicbor] | 2.2479 ms | 11.564 ms | 1777386 | 1276218 | 1252558 | 12.954 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.104 ms | 15.923 ms | 1770060 | 1277755 | 1263362 | 12.903 ms |
| [nanoserde 0.2.1][nanoserde] | 1.3021 ms | 2.8035 ms | 1812404 | 1134820 | 1053109 | 10.299 ms |
| [nibblecode 0.1.0][nibblecode] | 504.81 µs | † | 2075936 | 1541007 | 1432745 | 14.111 ms |
| [postcard 1.1.3][postcard] | 1.8204 ms | 4.1769 ms | 1311281 | 1083900 | 1041434 | 8.7406 ms |
| [pot 3.0.1][pot] | 13.684 ms | 29.638 ms | 2604812 | 1482233 | 1298928 | 16.042 ms |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*3.7245 ms\**</span> <span title="populate + encode">*11.155 ms\**</span> | 15.756 ms | 1859886 | 1338076 | 1295351 | 12.347 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*5.4536 ms\**</span> <span title="populate + encode">*9.3765 ms\**</span> | 8.6372 ms | 1859886 | 1338076 | 1295351 | 12.730 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.4553 ms\**</span> <span title="populate + encode">*12.773 ms\**</span> | 12.101 ms | 1859886 | 1338076 | 1295351 | 12.483 ms |
| [rkyv 0.8.16][rkyv] | 981.19 µs | <span title="unvalidated">*2.1821 ms\**</span> <span title="validated upfront with error">*2.6543 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.716 ms |
| [ron 0.12.2][ron] | 42.169 ms | 171.86 ms | 8677703 | 2233642 | 1826180 | 35.469 ms |
| [savefile 0.20.4][savefile] | 863.11 µs | 2.8831 ms | 1791505 | 1128012 | 1051153 | 10.840 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1430 ms | 3.3655 ms | 1319999 | 1064380 | 1010708 | 8.9447 ms |
| [serde-brief 0.2.0][serde-brief] | 6.1924 ms | 19.890 ms | 6951772 | 1796265 | 1567819 | 24.073 ms |
| [serde_bare 0.5.0][serde_bare] | 5.0002 ms | 5.0179 ms | 1319999 | 1062645 | 1008349 | 9.0426 ms |
| [speedy 0.8.7][speedy] | 771.45 µs | 2.4585 ms | 1584734 | 1119837 | 1037992 | 10.388 ms |
| [wincode 0.5.5][wincode] | 573.93 µs | 2.3079 ms | 1791489 | 1127998 | 1051146 | 10.395 ms |
| [wiring 0.2.4][wiring] | 640.37 µs | 2.7853 ms | 1791489 | 1156963 | 1082815 | 10.670 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*81.282 ns\**</span> | <span title="validated on-demand with error">*724.16 ns\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 51.648 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4891 ns\**</span> <span title="validated upfront with error">*5.4863 ms\**</span> | <span title="unvalidated">*2.7694 µs\**</span> <span title="validated upfront with error">*5.4937 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2443 ns\**</span> <span title="validated upfront with error">*343.99 µs\**</span> | <span title="unvalidated">*377.83 ns\**</span> <span title="validated upfront with error">*344.51 µs\**</span> | <span title="unvalidated">*238.63 ns\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2442 ns\**</span> <span title="validated upfront with error">*456.64 µs\**</span> | <span title="unvalidated">*385.34 ns\**</span> <span title="validated upfront with error">*459.96 µs\**</span> | <span title="unvalidated">*236.19 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*11.10%\**</span> <span title="prepend">*20.13%\**</span> | 14.70% | 47.09% | 62.06% | 64.44% | 3.31% |
| [bin-proto 0.12.8][bin-proto] | 9.21% | 18.85% | 44.80% | 71.21% | 76.36% | 3.78% |
| [bincode 2.0.1][bincode] | 39.32% | 31.36% | 57.08% | 71.86% | 75.55% | 3.92% |
| [bincode 1.3.3][bincode1] | 12.84% | 28.11% | 43.29% | 70.34% | 76.54% | 3.77% |
| [bitcode 0.6.9][bitcode] | 72.43% | 52.83% | 82.64% | 91.48% | 94.40% | 12.34% |
| [borsh 1.7.0][borsh] | 17.10% | 43.52% | 52.74% | 72.46% | 77.29% | 3.89% |
| capnp:<br> [capnp 0.26.0][capnp] | 19.28% <span title="packed">*11.07%\**</span> | <span title="packed">*64.92%\**</span> | 29.46% <span title="packed">*49.66%\**</span> | 51.92% <span title="packed">*62.81%\**</span> | 64.78% <span title="packed">*71.31%\**</span> | 2.65% <span title="packed">*4.23%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 17.32% | 7.51% | 13.35% | 47.38% | 54.79% | 1.77% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.07% | 2.30% | 13.35% | 47.38% | 54.79% | 1.84% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.00% | 6.16% | 13.35% | 47.38% | 54.79% | 1.70% |
| [columnar 0.13.0][columnar] | 57.80% | 32.68% <span title="copy_from">*100.00%\**</span> | 51.96% | 80.59% | 89.56% | 8.14% |
| [compactly 0.1.6][compactly] | 0.78% | 2.21% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 38.19% | 32.73% | 60.81% | 75.59% | 79.61% | 4.28% |
| [dlhn 0.1.7][dlhn] | 11.30% | 16.81% | 61.21% | 74.55% | 76.73% | 4.35% |
| [flatbuffers 25.12.19][flatbuffers] | 10.25% | † | 34.51% | 55.81% | 63.30% | 2.83% |
| [flexbuffers 25.12.19][flexbuffers] | 1.26% | 3.52% | 15.00% | 30.22% | 28.89% | 1.09% |
| json:<br> [flexon 0.4.6][flexon] | 3.18% | 5.05% | 8.55% | 33.58% | 43.56% | 1.10% |
| json:<br> [serde_json 1.0.150][serde_json] | 2.42% | 4.01% | 8.55% | 33.58% | 43.56% | 1.09% |
| json:<br> [simd-json 0.17.0][simd-json] | 4.18% | 4.90% | 8.55% | 33.58% | 43.56% | 1.12% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 52.89% | 21.35% | 55.02% | 69.48% | 70.55% | 3.85% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 4.86% | 10.93% | 45.99% | 63.67% | 65.32% | 3.25% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 44.75% | 29.62% | 46.73% | 65.05% | 67.12% | 3.47% |
| [minicbor 2.2.2][minicbor] | 22.46% | 10.79% | 45.16% | 62.94% | 64.08% | 3.01% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.68% | 7.84% | 45.35% | 62.86% | 63.54% | 3.02% |
| [nanoserde 0.2.1][nanoserde] | 38.77% | 44.52% | 44.29% | 70.78% | 76.22% | 3.79% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 38.67% | 52.12% | 56.02% | 2.76% |
| [postcard 1.1.3][postcard] | 27.73% | 29.88% | 61.21% | 74.11% | 77.08% | 4.46% |
| [pot 3.0.1][pot] | 3.69% | 4.21% | 30.81% | 54.19% | 61.80% | 2.43% |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*13.55%\**</span> <span title="populate + encode">*4.53%\**</span> | 7.92% | 43.16% | 60.03% | 61.97% | 3.16% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*9.26%\**</span> <span title="populate + encode">*5.38%\**</span> | 14.45% | 43.16% | 60.03% | 61.97% | 3.06% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*9.25%\**</span> <span title="populate + encode">*3.95%\**</span> | 10.31% | 43.16% | 60.03% | 61.97% | 3.12% |
| [rkyv 0.8.16][rkyv] | 51.45% | <span title="unvalidated">*57.20%\**</span> <span title="validated upfront with error">*47.02%\**</span> | 38.67% | 58.05% | 66.32% | 2.84% |
| [ron 0.12.2][ron] | 1.20% | 0.73% | 9.25% | 35.96% | 43.95% | 1.10% |
| [savefile 0.20.4][savefile] | 58.49% | 43.29% | 44.80% | 71.21% | 76.36% | 3.60% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.06% | 37.09% | 60.81% | 75.47% | 79.42% | 4.36% |
| [serde-brief 0.2.0][serde-brief] | 8.15% | 6.28% | 11.55% | 44.72% | 51.20% | 1.62% |
| [serde_bare 0.5.0][serde_bare] | 10.10% | 24.87% | 60.81% | 75.59% | 79.60% | 4.31% |
| [speedy 0.8.7][speedy] | 65.44% | 50.77% | 50.65% | 71.73% | 77.33% | 3.75% |
| [wincode 0.5.5][wincode] | 87.96% | 54.08% | 44.80% | 71.21% | 76.36% | 3.75% |
| [wiring 0.2.4][wiring] | 78.83% | 44.81% | 44.80% | 69.43% | 74.13% | 3.65% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.53%\**</span> | <span title="validated on-demand with error">*52.17%\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 2.41% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.64%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*98.98%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*98.05%\**</span> <span title="validated upfront with error">*0.08%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1014.2
[bin-proto]: https://crates.io/crates/bin-proto/0.12.8
[bincode]: https://crates.io/crates/bincode/2.0.1
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.9
[borsh]: https://crates.io/crates/borsh/1.7.0
[buffa]: https://crates.io/crates/buffa/0.7.1
[capnp]: https://crates.io/crates/capnp/0.26.0
[cbor4ii]: https://crates.io/crates/cbor4ii/1.2.2
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[columnar]: https://crates.io/crates/columnar/0.13.0
[compactly]: https://crates.io/crates/compactly/0.1.6
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.12.19
[flexbuffers]: https://crates.io/crates/flexbuffers/25.12.19
[flexon]: https://crates.io/crates/flexon/0.4.6
[minicbor]: https://crates.io/crates/minicbor/2.2.2
[msgpacker]: https://crates.io/crates/msgpacker/0.7.1
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.2.1
[nibblecode]: https://crates.io/crates/nibblecode/0.1.0
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.7.5
[postcard]: https://crates.io/crates/postcard/1.1.3
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.14.4
[protobuf]: https://crates.io/crates/protobuf/3.7.2
[rkyv]: https://crates.io/crates/rkyv/0.8.16
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.1
[ron]: https://crates.io/crates/ron/0.12.2
[savefile]: https://crates.io/crates/savefile/0.20.4
[serde-brief]: https://crates.io/crates/serde-brief/0.2.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.150
[simd-json]: https://crates.io/crates/simd-json/0.17.0
[speedy]: https://crates.io/crates/speedy/0.8.7
[wincode]: https://crates.io/crates/wincode/0.5.5
[wiring]: https://crates.io/crates/wiring/0.2.4
[zerompk]: https://crates.io/crates/zerompk/0.6.0


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
