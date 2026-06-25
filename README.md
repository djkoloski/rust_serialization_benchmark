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

## Last updated: 2026-06-25 18:04:15

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.98.0-nightly (f28ac764c 2026-06-23)
binary: rustc
commit-hash: f28ac764c36004fa6a6e098d15b4016a838c13c6
commit-date: 2026-06-23
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
| [bilrost 0.1014.2][bilrost] | <span title="encode">*437.29 µs\**</span> <span title="prepend">*420.64 µs\**</span> | 2.5612 ms | 859.27 µs | 804955 | 328941 | 284849 | 4.5576 ms |
| [bin-proto 0.12.8][bin-proto] | 4.6182 ms | 4.7068 ms | † | 1045784 | 373127 | 311553 | 4.6382 ms |
| [bincode 2.0.1][bincode] | 363.17 µs | 2.5042 ms | 690.71 µs | 741295 | 303944 | 256422 | 3.7613 ms |
| [bincode 1.3.3][bincode1] | 549.31 µs | 2.0086 ms | 605.24 µs | 1045784 | 373127 | 311553 | 4.5749 ms |
| [bitcode 0.6.9][bitcode] | 145.33 µs | 1.4697 ms | 63.120 µs | 703710 | 288826 | 227322 | 2.6175 ms |
| [borsh 1.7.0][borsh] | 547.23 µs | 2.1021 ms | † | 885780 | 362204 | 286248 | 4.2107 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 622.85 µs <span title="packed">*1.5526 ms\**</span> | <span title="packed">*1.0015 ms\**</span> | † | 1443216 <span title="packed">*1046865\**</span> | 513986 <span title="packed">*481681\**</span> | 426532 <span title="packed">*458024\**</span> | 6.2406 ms <span title="packed">*5.4992 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 611.66 µs | 5.1756 ms | 3.4079 ms | 1407835 | 403440 | 323561 | 5.0498 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1460 ms | 11.184 ms | † | 1407835 | 403440 | 323561 | 5.1084 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9161 ms | 4.9315 ms | 3.0079 ms | 1407835 | 403440 | 323561 | 5.2768 ms |
| [columnar 0.13.0][columnar] | 260.18 µs | 2.1203 ms <span title="copy_from">*729.13 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.2654 ms |
| [compactly 0.1.6][compactly] | 27.006 ms | 20.193 ms | † | 241251 | 241453 | 241263 | 109.39 µs |
| [databuf 0.5.0][databuf] | 258.18 µs | 2.0524 ms | 671.97 µs | 765778 | 311715 | 263914 | 3.5871 ms |
| [dlhn 0.1.7][dlhn] | 667.75 µs | 2.5214 ms | † | 724953 | 301446 | 253056 | 3.3955 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0384 ms | † | † | 1276368 | 468539 | 388381 | 4.7073 ms |
| [flexbuffers 25.12.19][flexbuffers] | 6.9640 ms | 7.1501 ms | 5.4066 ms | 1829756 | 714318 | 691541 | 8.6206 ms |
| json:<br> [flexon 0.4.6][flexon] | 2.7390 ms | 3.8671 ms | † | 1827461 | 470560 | 360727 | 5.5728 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 3.8179 ms | 5.8929 ms | † | 1827461 | 470560 | 360727 | 5.9330 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 2.1247 ms | 4.7693 ms | † | 1827461 | 470560 | 360727 | 5.5727 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 369.12 µs | 2.5863 ms | 938.06 µs | 764996 | 315291 | 264212 | 3.7416 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 1.5045 ms | 3.2829 ms | 1.4886 ms | 784997 | 325384 | 277608 | 3.9876 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 364.95 µs | 2.2714 ms | 816.22 µs | 784997 | 325384 | 277608 | 3.8455 ms |
| [minicbor 2.2.2][minicbor] | 525.60 µs | 2.9897 ms | 1.3892 ms | 817830 | 332671 | 284034 | 4.0674 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4184 ms | 4.0402 ms | 2.5238 ms | 818669 | 332556 | 284797 | 4.1256 ms |
| [nanoserde 0.2.1][nanoserde] | 267.33 µs | 2.1289 ms | † | 1045784 | 373127 | 311553 | 4.1902 ms |
| [nibblecode 0.1.0][nibblecode] | 192.23 µs | † | † | 1011487 | 492623 | 426886 | 5.6532 ms |
| [postcard 1.1.3][postcard] | 424.73 µs | 2.3002 ms | 719.39 µs | 724953 | 302399 | 252968 | 3.2627 ms |
| [pot 3.0.1][pot] | 2.2698 ms | 6.5249 ms | 4.9526 ms | 971922 | 372513 | 303636 | 4.6151 ms |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*785.14 µs\**</span> <span title="populate + encode">*2.5393 ms\**</span> | 3.9166 ms | 2.5299 ms | 884628 | 363130 | 314959 | 4.4692 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*956.94 µs\**</span> <span title="populate + encode">*2.5008 ms\**</span> | 3.3433 ms | † | 884628 | 363130 | 314959 | 4.4774 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2223 ms\**</span> <span title="populate + encode">*3.0485 ms\**</span> | 4.0153 ms | † | 884628 | 363130 | 314959 | 4.5377 ms |
| [rkyv 0.8.16][rkyv] | 246.46 µs | <span title="unvalidated">*1.5499 ms\**</span> <span title="validated upfront with error">*1.9269 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.7029 ms |
| [ron 0.12.2][ron] | 12.543 ms | 27.648 ms | 24.970 ms | 1607459 | 449158 | 349324 | 5.7073 ms |
| [savefile 0.20.4][savefile] | 193.74 µs | 2.0422 ms | † | 1045800 | 373139 | 311562 | 4.1821 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 648.76 µs | 2.3424 ms | † | 765778 | 311743 | 263822 | 3.7395 ms |
| [serde-brief 0.2.0][serde-brief] | 1.4130 ms | 4.6255 ms | 2.8391 ms | 1584946 | 413733 | 339964 | 4.9098 ms |
| [serde_bare 0.5.0][serde_bare] | 685.82 µs | 2.0829 ms | † | 765778 | 311715 | 263914 | 3.5895 ms |
| [speedy 0.8.7][speedy] | 200.23 µs | 1.7419 ms | 375.18 µs | 885780 | 362204 | 286248 | 4.0273 ms |
| [wincode 0.5.5][wincode] | 168.59 µs | 1.7618 ms | 381.85 µs | 1045784 | 373127 | 311553 | 4.2224 ms |
| [wiring 0.2.4][wiring] | 197.21 µs | 2.0706 ms | † | 1045784 | 337930 | 275808 | 3.6622 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*76.831 ns\**</span> | <span title="validated on-demand with error">*141.38 µs\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 20.871 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4928 ns\**</span> <span title="validated upfront with error">*2.1620 ms\**</span> | <span title="unvalidated">*52.167 µs\**</span> <span title="validated upfront with error">*2.2063 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2451 ns\**</span> <span title="validated upfront with error">*235.10 µs\**</span> | <span title="unvalidated">*10.450 µs\**</span> <span title="validated upfront with error">*246.25 µs\**</span> | <span title="unvalidated">*7.4968 µs\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2450 ns\**</span> <span title="validated upfront with error">*372.99 µs\**</span> | <span title="unvalidated">*10.435 µs\**</span> <span title="validated upfront with error">*383.58 µs\**</span> | <span title="unvalidated">*7.4517 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*33.23%\**</span> <span title="prepend">*34.55%\**</span> | 28.47% | 7.35% | 29.97% | 73.40% | 79.80% | 2.40% |
| [bin-proto 0.12.8][bin-proto] | 3.15% | 15.49% | † | 23.07% | 64.71% | 72.96% | 2.36% |
| [bincode 2.0.1][bincode] | 40.02% | 29.12% | 9.14% | 32.54% | 79.44% | 88.65% | 2.91% |
| [bincode 1.3.3][bincode1] | 26.46% | 36.30% | 10.43% | 23.07% | 64.71% | 72.96% | 2.39% |
| [bitcode 0.6.9][bitcode] | 100.00% | 49.61% | 100.00% | 34.28% | 83.60% | 100.00% | 4.18% |
| [borsh 1.7.0][borsh] | 26.56% | 34.69% | † | 27.24% | 66.66% | 79.41% | 2.60% |
| capnp:<br> [capnp 0.26.0][capnp] | 23.33% <span title="packed">*9.36%\**</span> | <span title="packed">*72.80%\**</span> | † | 16.72% <span title="packed">*23.05%\**</span> | 46.98% <span title="packed">*50.13%\**</span> | 53.30% <span title="packed">*49.63%\**</span> | 1.75% <span title="packed">*1.99%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 23.76% | 14.09% | 1.85% | 17.14% | 59.85% | 70.26% | 2.17% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.62% | 6.52% | † | 17.14% | 59.85% | 70.26% | 2.14% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.58% | 14.79% | 2.10% | 17.14% | 59.85% | 70.26% | 2.07% |
| [columnar 0.13.0][columnar] | 55.86% | 34.39% <span title="copy_from">*100.00%\**</span> | † | 23.07% | 65.22% | 77.34% | 2.56% |
| [compactly 0.1.6][compactly] | 0.54% | 3.61% | † | 100.00% | 100.00% | 94.22% | 100.00% |
| [databuf 0.5.0][databuf] | 56.29% | 35.53% | 9.39% | 31.50% | 77.46% | 86.13% | 3.05% |
| [dlhn 0.1.7][dlhn] | 21.76% | 28.92% | † | 33.28% | 80.10% | 89.83% | 3.22% |
| [flatbuffers 25.12.19][flatbuffers] | 14.00% | † | † | 18.90% | 51.53% | 58.53% | 2.32% |
| [flexbuffers 25.12.19][flexbuffers] | 2.09% | 10.20% | 1.17% | 13.18% | 33.80% | 32.87% | 1.27% |
| json:<br> [flexon 0.4.6][flexon] | 5.31% | 18.85% | † | 13.20% | 51.31% | 63.02% | 1.96% |
| json:<br> [serde_json 1.0.150][serde_json] | 3.81% | 12.37% | † | 13.20% | 51.31% | 63.02% | 1.84% |
| json:<br> [simd-json 0.17.0][simd-json] | 6.84% | 15.29% | † | 13.20% | 51.31% | 63.02% | 1.96% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 39.37% | 28.19% | 6.73% | 31.54% | 76.58% | 86.04% | 2.92% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 9.66% | 22.21% | 4.24% | 30.73% | 74.21% | 81.89% | 2.74% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 39.82% | 32.10% | 7.73% | 30.73% | 74.21% | 81.89% | 2.84% |
| [minicbor 2.2.2][minicbor] | 27.65% | 24.39% | 4.54% | 29.50% | 72.58% | 80.03% | 2.69% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.68% | 18.05% | 2.50% | 29.47% | 72.61% | 79.82% | 2.65% |
| [nanoserde 0.2.1][nanoserde] | 54.36% | 34.25% | † | 23.07% | 64.71% | 72.96% | 2.61% |
| [nibblecode 0.1.0][nibblecode] | 75.60% | † | † | 23.85% | 49.01% | 53.25% | 1.93% |
| [postcard 1.1.3][postcard] | 34.22% | 31.70% | 8.77% | 33.28% | 79.85% | 89.86% | 3.35% |
| [pot 3.0.1][pot] | 6.40% | 11.17% | 1.27% | 24.82% | 64.82% | 74.87% | 2.37% |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*18.51%\**</span> <span title="populate + encode">*5.72%\**</span> | 18.62% | 2.49% | 27.27% | 66.49% | 72.18% | 2.45% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*15.19%\**</span> <span title="populate + encode">*5.81%\**</span> | 21.81% | † | 27.27% | 66.49% | 72.18% | 2.44% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.89%\**</span> <span title="populate + encode">*4.77%\**</span> | 18.16% | † | 27.27% | 66.49% | 72.18% | 2.41% |
| [rkyv 0.8.16][rkyv] | 58.97% | <span title="unvalidated">*47.04%\**</span> <span title="validated upfront with error">*37.84%\**</span> | † | 23.85% | 61.36% | 69.74% | 2.33% |
| [ron 0.12.2][ron] | 1.16% | 2.64% | 0.25% | 15.01% | 53.76% | 65.07% | 1.92% |
| [savefile 0.20.4][savefile] | 75.01% | 35.70% | † | 23.07% | 64.71% | 72.96% | 2.62% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.40% | 31.13% | † | 31.50% | 77.45% | 86.16% | 2.93% |
| [serde-brief 0.2.0][serde-brief] | 10.29% | 15.76% | 2.22% | 15.22% | 58.36% | 66.87% | 2.23% |
| [serde_bare 0.5.0][serde_bare] | 21.19% | 35.01% | † | 31.50% | 77.46% | 86.13% | 3.05% |
| [speedy 0.8.7][speedy] | 72.58% | 41.86% | 16.82% | 27.24% | 66.66% | 79.41% | 2.72% |
| [wincode 0.5.5][wincode] | 86.20% | 41.39% | 16.53% | 23.07% | 64.71% | 72.96% | 2.59% |
| [wiring 0.2.4][wiring] | 73.69% | 35.21% | † | 23.07% | 71.45% | 82.42% | 2.99% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.62%\**</span> | <span title="validated on-demand with error">*7.38%\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 5.97% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.94%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.00%\**</span> <span title="validated upfront with error">*0.47%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.86%\**</span> <span title="validated upfront with error">*4.24%\**</span> | <span title="unvalidated">*99.40%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.72%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*7.1477 ms\**</span> <span title="prepend">*8.8049 ms\**</span> | 7.5967 ms | 8625005 | 6443961 | 6231572 | 76.192 ms |
| [bin-proto 0.12.8][bin-proto] | 8.9670 ms | 10.634 ms | 6000008 | 5378500 | 5346908 | 8.9234 ms |
| [bincode 2.0.1][bincode] | 2.8871 ms | 1.0708 ms | 6000005 | 5378497 | 5346882 | 8.6957 ms |
| [bincode 1.3.3][bincode1] | 5.9160 ms | 5.8343 ms | 6000008 | 5378500 | 5346908 | 8.5767 ms |
| [bitcode 0.6.9][bitcode] | 1.3124 ms | 808.75 µs | 6000006 | 5182295 | 4921841 | 13.742 ms |
| [borsh 1.7.0][borsh] | 6.2169 ms | 4.3118 ms | 6000004 | 5378496 | 5346866 | 9.1305 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 9.8933 ms <span title="packed">*19.944 ms\**</span> | <span title="packed">*13.115 ms\**</span> | 14000088 <span title="packed">*10401737\**</span> | 7130367 <span title="packed">*7308001\**</span> | 6046182 <span title="packed">*7922110\**</span> | 82.552 ms <span title="packed">*71.829 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 8.9715 ms | 44.539 ms | 13125016 | 7524114 | 6757437 | 93.007 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 64.804 ms | 111.71 ms | 13122324 | 7524660 | 6759128 | 93.356 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 35.762 ms | 38.829 ms | 13122324 | 7524660 | 6759128 | 91.567 ms |
| [columnar 0.13.0][columnar] | 1.8052 ms | 1.4438 ms <span title="copy_from">*669.99 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.7440 ms |
| [compactly 0.1.6][compactly] | 354.90 ms | 280.89 ms | 4846786 | 4850065 | 4846903 | 1.8616 ms |
| [databuf 0.5.0][databuf] | 2.4190 ms | 5.4086 ms | 6000003 | 5378495 | 5346897 | 8.7915 ms |
| [dlhn 0.1.7][dlhn] | 5.8619 ms | 7.0329 ms | 6000003 | 5378495 | 5346897 | 8.9625 ms |
| [flatbuffers 25.12.19][flatbuffers] | 515.52 µs | † | 6000024 | 5378434 | 5346878 | 8.6873 ms |
| [flexbuffers 25.12.19][flexbuffers] | 104.78 ms | 74.967 ms | 26609424 | 11901040 | 12486322 | 153.93 ms |
| json:<br> [flexon 0.4.6][flexon] | 76.558 ms | 55.421 ms | 26192883 | 9566084 | 8584671 | 160.65 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 92.922 ms | 100.31 ms | 26192883 | 9566084 | 8584671 | 157.42 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 53.138 ms | 67.155 ms | 26192883 | 9566084 | 8584671 | 157.87 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 652.65 µs | 5.2143 ms | 7500005 | 6058442 | 6014500 | 11.175 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 19.499 ms | 16.614 ms | 8125006 | 6494876 | 6391037 | 70.616 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 754.28 µs | 5.2019 ms | 8125006 | 6494876 | 6391037 | 70.870 ms |
| [minicbor 2.2.2][minicbor] | 5.2007 ms | 11.727 ms | 8125006 | 6494907 | 6390894 | 72.736 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.02 ms | 25.723 ms | 8125037 | 6493484 | 6386940 | 71.700 ms |
| [nanoserde 0.2.1][nanoserde] | 1.7783 ms | 891.09 µs | 6000008 | 5378500 | 5346908 | 8.9053 ms |
| [nibblecode 0.1.0][nibblecode] | 195.92 µs | † | 6000008 | 5378500 | 5346908 | 8.7893 ms |
| [postcard 1.1.3][postcard] | 513.16 µs | 6.0751 ms | 6000003 | 5378495 | 5346897 | 9.2187 ms |
| [pot 3.0.1][pot] | 37.975 ms | 70.330 ms | 10122342 | 6814618 | 6852252 | 84.307 ms |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*8.6546 ms\**</span> <span title="populate + encode">*25.327 ms\**</span> | 26.137 ms | 8750000 | 6665735 | 6421877 | 75.631 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*7.8443 ms\**</span> <span title="populate + encode">*8.6280 ms\**</span> | 14.804 ms | 8750000 | 6665735 | 6421877 | 72.945 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.461 ms\**</span> <span title="populate + encode">*31.406 ms\**</span> | 29.161 ms | 8750000 | 6665735 | 6421877 | 74.459 ms |
| [rkyv 0.8.16][rkyv] | 197.67 µs | <span title="unvalidated">*198.80 µs\**</span> <span title="validated upfront with error">*198.94 µs\**</span> | 6000008 | 5378500 | 5346872 | 9.1950 ms |
| [ron 0.12.2][ron] | 178.41 ms | 610.20 ms | 22192885 | 8970395 | 8137334 | 152.54 ms |
| [savefile 0.20.4][savefile] | 196.93 µs | 198.21 µs | 6000024 | 5378519 | 5346896 | 8.7154 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 6.0220 ms | 4.1747 ms | 6000004 | 5378496 | 5346866 | 8.6500 ms |
| [serde-brief 0.2.0][serde-brief] | 17.233 ms | 37.610 ms | 15750015 | 8024540 | 6813667 | 94.567 ms |
| [serde_bare 0.5.0][serde_bare] | 6.2250 ms | 4.2140 ms | 6000003 | 5378495 | 5346897 | 8.8863 ms |
| [speedy 0.8.7][speedy] | 198.13 µs | 198.61 µs | 6000004 | 5378496 | 5346866 | 8.6523 ms |
| [wincode 0.5.5][wincode] | 197.64 µs | 197.07 µs | 6000008 | 5378500 | 5346908 | 9.0113 ms |
| [wiring 0.2.4][wiring] | 198.97 µs | 338.89 µs | 6000008 | 5378952 | 5346905 | 8.7592 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*105.73 ns\**</span> | <span title="validated on-demand with error">*2.1153 ms\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 20.790 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4894 ns\**</span> <span title="validated upfront with error">*45.603 ns\**</span> | <span title="unvalidated">*77.825 µs\**</span> <span title="validated upfront with error">*77.852 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*1.5567 ns\**</span> | <span title="unvalidated">*38.920 µs\**</span> <span title="validated upfront with error">*38.907 µs\**</span> | <span title="unvalidated">*99.898 µs\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2454 ns\**</span> <span title="validated upfront with error">*5.6193 ns\**</span> | <span title="unvalidated">*38.911 µs\**</span> <span title="validated upfront with error">*38.920 µs\**</span> | <span title="unvalidated">*99.386 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*2.74%\**</span> <span title="prepend">*2.23%\**</span> | 2.59% | 56.19% | 75.27% | 77.78% | 2.44% |
| [bin-proto 0.12.8][bin-proto] | 2.18% | 1.85% | 80.78% | 90.18% | 90.65% | 20.86% |
| [bincode 2.0.1][bincode] | 6.79% | 18.40% | 80.78% | 90.18% | 90.65% | 21.41% |
| [bincode 1.3.3][bincode1] | 3.31% | 3.38% | 80.78% | 90.18% | 90.65% | 21.71% |
| [bitcode 0.6.9][bitcode] | 14.93% | 24.37% | 80.78% | 93.59% | 98.48% | 13.55% |
| [borsh 1.7.0][borsh] | 3.15% | 4.57% | 80.78% | 90.18% | 90.65% | 20.39% |
| capnp:<br> [capnp 0.26.0][capnp] | 1.98% <span title="packed">*0.98%\**</span> | <span title="packed">*1.50%\**</span> | 34.62% <span title="packed">*46.60%\**</span> | 68.02% <span title="packed">*66.37%\**</span> | 80.16% <span title="packed">*61.18%\**</span> | 2.26% <span title="packed">*2.59%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 2.18% | 0.44% | 36.93% | 64.46% | 71.73% | 2.00% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.30% | 0.18% | 36.94% | 64.46% | 71.71% | 1.99% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.55% | 0.51% | 36.94% | 64.46% | 71.71% | 2.03% |
| [columnar 0.13.0][columnar] | 10.85% | 13.65% <span title="copy_from">*29.41%\**</span> | 80.78% | 90.18% | 90.65% | 21.29% |
| [compactly 0.1.6][compactly] | 0.06% | 0.07% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 8.10% | 3.64% | 80.78% | 90.18% | 90.65% | 21.17% |
| [dlhn 0.1.7][dlhn] | 3.34% | 2.80% | 80.78% | 90.18% | 90.65% | 20.77% |
| [flatbuffers 25.12.19][flatbuffers] | 38.00% | † | 80.78% | 90.18% | 90.65% | 21.43% |
| [flexbuffers 25.12.19][flexbuffers] | 0.19% | 0.26% | 18.21% | 40.75% | 38.82% | 1.21% |
| json:<br> [flexon 0.4.6][flexon] | 0.26% | 0.36% | 18.50% | 50.70% | 56.46% | 1.16% |
| json:<br> [serde_json 1.0.150][serde_json] | 0.21% | 0.20% | 18.50% | 50.70% | 56.46% | 1.18% |
| json:<br> [simd-json 0.17.0][simd-json] | 0.37% | 0.29% | 18.50% | 50.70% | 56.46% | 1.18% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 30.02% | 3.78% | 64.62% | 80.05% | 80.59% | 16.66% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 1.00% | 1.19% | 59.65% | 74.68% | 75.84% | 2.64% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 25.97% | 3.79% | 59.65% | 74.68% | 75.84% | 2.63% |
| [minicbor 2.2.2][minicbor] | 3.77% | 1.68% | 59.65% | 74.67% | 75.84% | 2.56% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.77% | 59.65% | 74.69% | 75.89% | 2.60% |
| [nanoserde 0.2.1][nanoserde] | 11.02% | 22.12% | 80.78% | 90.18% | 90.65% | 20.90% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 80.78% | 90.18% | 90.65% | 21.18% |
| [postcard 1.1.3][postcard] | 38.18% | 3.24% | 80.78% | 90.18% | 90.65% | 20.19% |
| [pot 3.0.1][pot] | 0.52% | 0.28% | 47.88% | 71.17% | 70.73% | 2.21% |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*2.26%\**</span> <span title="populate + encode">*0.77%\**</span> | 0.75% | 55.39% | 72.76% | 75.47% | 2.46% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*2.50%\**</span> <span title="populate + encode">*2.27%\**</span> | 1.33% | 55.39% | 72.76% | 75.47% | 2.55% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.35%\**</span> <span title="populate + encode">*0.62%\**</span> | 0.68% | 55.39% | 72.76% | 75.47% | 2.50% |
| [rkyv 0.8.16][rkyv] | 99.11% | <span title="unvalidated">*99.13%\**</span> <span title="validated upfront with error">*99.06%\**</span> | 80.78% | 90.18% | 90.65% | 20.25% |
| [ron 0.12.2][ron] | 0.11% | 0.03% | 21.84% | 54.07% | 59.56% | 1.22% |
| [savefile 0.20.4][savefile] | 99.49% | 99.42% | 80.78% | 90.17% | 90.65% | 21.36% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.25% | 4.72% | 80.78% | 90.18% | 90.65% | 21.52% |
| [serde-brief 0.2.0][serde-brief] | 1.14% | 0.52% | 30.77% | 60.44% | 71.14% | 1.97% |
| [serde_bare 0.5.0][serde_bare] | 3.15% | 4.68% | 80.78% | 90.18% | 90.65% | 20.95% |
| [speedy 0.8.7][speedy] | 98.88% | 99.22% | 80.78% | 90.18% | 90.65% | 21.52% |
| [wincode 0.5.5][wincode] | 99.13% | 100.00% | 80.78% | 90.18% | 90.65% | 20.66% |
| [wiring 0.2.4][wiring] | 98.47% | 58.15% | 80.78% | 90.17% | 90.65% | 21.25% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.84%\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 5.99% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*2.73%\**</span> | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*49.98%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.96%\**</span> | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*99.49%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.94%\**</span> <span title="validated upfront with error">*22.15%\**</span> | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*99.97%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*873.85 µs\**</span> <span title="prepend">*782.63 µs\**</span> | 3.1492 ms | 1.7265 ms | 489348 | 281173 | 249360 | 2.7574 ms |
| [bin-proto 0.12.8][bin-proto] | 2.0110 ms | 2.8522 ms | † | 566975 | 239350 | 231475 | 2.4788 ms |
| [bincode 2.0.1][bincode] | 345.18 µs | 2.0717 ms | 772.42 µs | 367413 | 221291 | 206242 | 2.0347 ms |
| [bincode 1.3.3][bincode1] | 592.77 µs | 1.8404 ms | 870.65 µs | 569975 | 240525 | 231884 | 2.5429 ms |
| [bitcode 0.6.9][bitcode] | 139.52 µs | 1.2762 ms | 171.78 µs | 327688 | 200947 | 182040 | 810.11 µs |
| [borsh 1.7.0][borsh] | 555.22 µs | 1.7896 ms | † | 446595 | 234236 | 209834 | 2.1142 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 492.96 µs <span title="packed">*1.0883 ms\**</span> | <span title="packed">*568.71 µs\**</span> | † | 803896 <span title="packed">*489017\**</span> | 335606 <span title="packed">*293127\**</span> | 280744 <span title="packed">*271528\**</span> | 3.6184 ms <span title="packed">*2.5724 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 723.71 µs | 4.4374 ms | 3.3064 ms | 1109831 | 344745 | 274333 | 3.4826 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.8319 ms | 9.8716 ms | † | 1109821 | 344751 | 274345 | 3.4548 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8718 ms | 4.5488 ms | 3.3471 ms | 1109821 | 344751 | 274345 | 3.7323 ms |
| [columnar 0.13.0][columnar] | 264.32 µs | 1.9400 ms <span title="copy_from">*670.39 µs\**</span> | † | 563392 | 249619 | 217632 | 1.6335 ms |
| [compactly 0.1.6][compactly] | 11.604 ms | 11.413 ms | † | 149292 | 149433 | 149304 | 114.94 µs |
| [databuf 0.5.0][databuf] | 287.15 µs | 1.7306 ms | 777.07 µs | 356311 | 213062 | 198403 | 2.0349 ms |
| [dlhn 0.1.7][dlhn] | 683.55 µs | 2.7239 ms | † | 366496 | 220600 | 205586 | 2.0817 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2524 ms | † | † | 849472 | 347816 | 294871 | 3.5456 ms |
| [flexbuffers 25.12.19][flexbuffers] | 7.7427 ms | 6.8178 ms | 5.5959 ms | 1187688 | 557642 | 553730 | 6.3574 ms |
| json:<br> [flexon 0.4.6][flexon] | 2.7399 ms | 4.5812 ms | † | 1623191 | 466527 | 359157 | 5.9082 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 3.7073 ms | 6.9469 ms | † | 1623191 | 466527 | 359157 | 5.8163 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 2.2301 ms | 4.5631 ms | † | 1623191 | 466527 | 359157 | 5.9705 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 331.21 µs | 2.8323 ms | 1.3573 ms | 391251 | 236877 | 220395 | 2.2563 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 1.4957 ms | 3.0339 ms | 1.7068 ms | 424533 | 245214 | 226077 | 2.3263 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 346.54 µs | 2.0973 ms | 879.70 µs | 416025 | 243812 | 224965 | 2.2935 ms |
| [minicbor 2.2.2][minicbor] | 581.06 µs | 3.3722 ms | 1.8694 ms | 428773 | 249857 | 228630 | 2.4656 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1700 ms | 3.8314 ms | 2.7381 ms | 449745 | 252432 | 230965 | 2.4110 ms |
| [nanoserde 0.2.1][nanoserde] | 269.50 µs | 1.8997 ms | † | 567975 | 239930 | 231872 | 2.6786 ms |
| [nibblecode 0.1.0][nibblecode] | 182.28 µs | † | † | 603928 | 431556 | 408852 | 3.6880 ms |
| [postcard 1.1.3][postcard] | 445.73 µs | 2.1070 ms | 816.43 µs | 367489 | 221913 | 207244 | 2.0536 ms |
| [pot 3.0.1][pot] | 2.4856 ms | 6.2021 ms | 4.8337 ms | 599125 | 299158 | 247675 | 2.8267 ms |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*1.1072 ms\**</span> <span title="populate + encode">*3.1472 ms\**</span> | 3.5324 ms | 2.5728 ms | 596811 | 305319 | 268737 | 3.1654 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*1.3078 ms\**</span> <span title="populate + encode">*3.0307 ms\**</span> | 3.5220 ms | † | 596811 | 305319 | 268737 | 3.0907 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0683 ms\**</span> <span title="populate + encode">*3.0416 ms\**</span> | 3.7916 ms | † | 596811 | 305319 | 268737 | 2.9911 ms |
| [rkyv 0.8.16][rkyv] | 329.00 µs | <span title="unvalidated">*1.5063 ms\**</span> <span title="validated upfront with error">*1.8680 ms\**</span> | † | 603776 | 254776 | 219421 | 2.4246 ms |
| [ron 0.12.2][ron] | 8.2637 ms | 27.930 ms | 26.530 ms | 1465223 | 434935 | 342907 | 5.6887 ms |
| [savefile 0.20.4][savefile] | 214.79 µs | 1.9121 ms | † | 566991 | 239362 | 231478 | 2.5138 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 614.39 µs | 2.0953 ms | † | 356311 | 212976 | 198423 | 2.0006 ms |
| [serde-brief 0.2.0][serde-brief] | 1.2385 ms | 5.1368 ms | 3.4384 ms | 1276014 | 373898 | 293384 | 3.6894 ms |
| [serde_bare 0.5.0][serde_bare] | 731.69 µs | 2.3233 ms | † | 356311 | 213062 | 198403 | 2.2805 ms |
| [speedy 0.8.7][speedy] | 267.04 µs | 1.6633 ms | 543.36 µs | 449595 | 234970 | 210192 | 2.1535 ms |
| [wincode 0.5.5][wincode] | 204.81 µs | 1.6329 ms | 549.14 µs | 566975 | 239350 | 231475 | 2.6214 ms |
| [wiring 0.2.4][wiring] | 206.94 µs | 1.9145 ms | † | 566975 | 247810 | 225086 | 2.5417 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*77.128 ns\**</span> | <span title="validated on-demand with error">*419.48 ns\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 639.16 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4896 ns\**</span> <span title="validated upfront with error">*2.1466 ms\**</span> | <span title="unvalidated">*1.4562 µs\**</span> <span title="validated upfront with error">*2.1508 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2445 ns\**</span> <span title="validated upfront with error">*245.62 µs\**</span> | <span title="unvalidated">*156.28 ns\**</span> <span title="validated upfront with error">*246.50 µs\**</span> | <span title="unvalidated">*726.38 ns\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*343.57 µs\**</span> | <span title="unvalidated">*156.26 ns\**</span> <span title="validated upfront with error">*344.97 µs\**</span> | <span title="unvalidated">*731.20 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*15.97%\**</span> <span title="prepend">*17.83%\**</span> | 18.06% | 9.95% | 30.51% | 53.15% | 59.87% | 4.17% |
| [bin-proto 0.12.8][bin-proto] | 6.94% | 19.94% | † | 26.33% | 62.43% | 64.50% | 4.64% |
| [bincode 2.0.1][bincode] | 40.42% | 27.45% | 22.24% | 40.63% | 67.53% | 72.39% | 5.65% |
| [bincode 1.3.3][bincode1] | 23.54% | 30.90% | 19.73% | 26.19% | 62.13% | 64.39% | 4.52% |
| [bitcode 0.6.9][bitcode] | 100.00% | 44.56% | 100.00% | 45.56% | 74.36% | 82.02% | 14.19% |
| [borsh 1.7.0][borsh] | 25.13% | 31.78% | † | 33.43% | 63.80% | 71.15% | 5.44% |
| capnp:<br> [capnp 0.26.0][capnp] | 28.30% <span title="packed">*12.82%\**</span> | <span title="packed">*100.00%\**</span> | † | 18.57% <span title="packed">*30.53%\**</span> | 44.53% <span title="packed">*50.98%\**</span> | 53.18% <span title="packed">*54.99%\**</span> | 3.18% <span title="packed">*4.47%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 19.28% | 12.82% | 5.20% | 13.45% | 43.35% | 54.42% | 3.30% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.64% | 5.76% | † | 13.45% | 43.35% | 54.42% | 3.33% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.45% | 12.50% | 5.13% | 13.45% | 43.35% | 54.42% | 3.08% |
| [columnar 0.13.0][columnar] | 52.78% | 29.31% <span title="copy_from">*84.83%\**</span> | † | 26.50% | 59.86% | 68.60% | 7.04% |
| [compactly 0.1.6][compactly] | 1.20% | 4.98% | † | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 48.59% | 32.86% | 22.11% | 41.90% | 70.14% | 75.25% | 5.65% |
| [dlhn 0.1.7][dlhn] | 20.41% | 20.88% | † | 40.73% | 67.74% | 72.62% | 5.52% |
| [flatbuffers 25.12.19][flatbuffers] | 4.29% | † | † | 17.57% | 42.96% | 50.63% | 3.24% |
| [flexbuffers 25.12.19][flexbuffers] | 1.80% | 8.34% | 3.07% | 12.57% | 26.80% | 26.96% | 1.81% |
| json:<br> [flexon 0.4.6][flexon] | 5.09% | 12.41% | † | 9.20% | 32.03% | 41.57% | 1.95% |
| json:<br> [serde_json 1.0.150][serde_json] | 3.76% | 8.19% | † | 9.20% | 32.03% | 41.57% | 1.98% |
| json:<br> [simd-json 0.17.0][simd-json] | 6.26% | 12.46% | † | 9.20% | 32.03% | 41.57% | 1.93% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 42.12% | 20.08% | 12.66% | 38.16% | 63.08% | 67.74% | 5.09% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 9.33% | 18.75% | 10.06% | 35.17% | 60.94% | 66.04% | 4.94% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 40.26% | 27.12% | 19.53% | 35.89% | 61.29% | 66.37% | 5.01% |
| [minicbor 2.2.2][minicbor] | 24.01% | 16.86% | 9.19% | 34.82% | 59.81% | 65.30% | 4.66% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.70% | 14.84% | 6.27% | 33.19% | 59.20% | 64.64% | 4.77% |
| [nanoserde 0.2.1][nanoserde] | 51.77% | 29.94% | † | 26.28% | 62.28% | 64.39% | 4.29% |
| [nibblecode 0.1.0][nibblecode] | 76.54% | † | † | 24.72% | 34.63% | 36.52% | 3.12% |
| [postcard 1.1.3][postcard] | 31.30% | 26.99% | 21.04% | 40.62% | 67.34% | 72.04% | 5.60% |
| [pot 3.0.1][pot] | 5.61% | 9.17% | 3.55% | 24.92% | 49.95% | 60.28% | 4.07% |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*12.60%\**</span> <span title="populate + encode">*4.43%\**</span> | 16.10% | 6.68% | 25.01% | 48.94% | 55.56% | 3.63% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*10.67%\**</span> <span title="populate + encode">*4.60%\**</span> | 16.15% | † | 25.01% | 48.94% | 55.56% | 3.72% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*13.06%\**</span> <span title="populate + encode">*4.59%\**</span> | 15.00% | † | 25.01% | 48.94% | 55.56% | 3.84% |
| [rkyv 0.8.16][rkyv] | 42.41% | <span title="unvalidated">*37.76%\**</span> <span title="validated upfront with error">*30.44%\**</span> | † | 24.73% | 58.65% | 68.04% | 4.74% |
| [ron 0.12.2][ron] | 1.69% | 2.04% | 0.65% | 10.19% | 34.36% | 43.54% | 2.02% |
| [savefile 0.20.4][savefile] | 64.96% | 29.74% | † | 26.33% | 62.43% | 64.50% | 4.57% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.71% | 27.14% | † | 41.90% | 70.16% | 75.25% | 5.75% |
| [serde-brief 0.2.0][serde-brief] | 11.27% | 11.07% | 5.00% | 11.70% | 39.97% | 50.89% | 3.12% |
| [serde_bare 0.5.0][serde_bare] | 19.07% | 24.48% | † | 41.90% | 70.14% | 75.25% | 5.04% |
| [speedy 0.8.7][speedy] | 52.25% | 34.19% | 31.61% | 33.21% | 63.60% | 71.03% | 5.34% |
| [wincode 0.5.5][wincode] | 68.12% | 34.83% | 31.28% | 26.33% | 62.43% | 64.50% | 4.38% |
| [wiring 0.2.4][wiring] | 67.42% | 29.71% | † | 26.33% | 60.30% | 66.33% | 4.52% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.61%\**</span> | <span title="validated on-demand with error">*37.25%\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 0.19% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.73%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*99.34%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*4.5973 ms\**</span> <span title="prepend">*2.5331 ms\**</span> | 8.6449 ms | 1704643 | 1294259 | 1245668 | 11.740 ms |
| [bin-proto 0.12.8][bin-proto] | 6.0368 ms | 7.0141 ms | 1791489 | 1127998 | 1051146 | 10.379 ms |
| [bincode 2.0.1][bincode] | 1.4787 ms | 3.8898 ms | 1406257 | 1117802 | 1062438 | 9.5283 ms |
| [bincode 1.3.3][bincode1] | 3.7977 ms | 4.3202 ms | 1854234 | 1141994 | 1048745 | 10.560 ms |
| [bitcode 0.6.9][bitcode] | 685.61 µs | 2.3654 ms | 971318 | 878034 | 850340 | 2.8914 ms |
| [borsh 1.7.0][borsh] | 2.9360 ms | 2.8791 ms | 1521989 | 1108471 | 1038528 | 9.9672 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 2.6905 ms <span title="packed">*4.5935 ms\**</span> | <span title="packed">*1.9183 ms\**</span> | 2724288 <span title="packed">*1616255\**</span> | 1546992 <span title="packed">*1278764\**</span> | 1239111 <span title="packed">*1125654\**</span> | 14.667 ms <span title="packed">*10.136 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 2.9064 ms | 17.094 ms | 6012539 | 1695215 | 1464951 | 21.489 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.519 ms | 53.535 ms | 6012373 | 1695146 | 1465025 | 21.727 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.8230 ms | 20.510 ms | 6012373 | 1695146 | 1465025 | 21.276 ms |
| [columnar 0.13.0][columnar] | 882.63 µs | 3.7787 ms <span title="copy_from">*1.2840 ms\**</span> | 1544720 | 996718 | 896213 | 4.7792 ms |
| [compactly 0.1.6][compactly] | 64.895 ms | 56.019 ms | 802662 | 803238 | 802689 | 309.83 µs |
| [databuf 0.5.0][databuf] | 1.3248 ms | 3.7440 ms | 1319999 | 1062631 | 1008334 | 9.4943 ms |
| [dlhn 0.1.7][dlhn] | 4.4244 ms | 7.4881 ms | 1311281 | 1077520 | 1046095 | 9.3715 ms |
| [flatbuffers 25.12.19][flatbuffers] | 4.9141 ms | † | 2325620 | 1439185 | 1268060 | 13.906 ms |
| [flexbuffers 25.12.19][flexbuffers] | 40.074 ms | 36.767 ms | 5352680 | 2658295 | 2777967 | 34.572 ms |
| json:<br> [flexon 0.4.6][flexon] | 15.515 ms | 24.602 ms | 9390461 | 2391679 | 1842767 | 35.754 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 20.618 ms | 31.199 ms | 9390461 | 2391679 | 1842767 | 34.797 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 12.017 ms | 26.038 ms | 9390461 | 2391679 | 1842767 | 34.826 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 948.85 µs | 5.8358 ms | 1458773 | 1156055 | 1137788 | 10.712 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 10.319 ms | 10.982 ms | 1745322 | 1261627 | 1228923 | 11.668 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 1.1233 ms | 4.2216 ms | 1717696 | 1234725 | 1195988 | 11.357 ms |
| [minicbor 2.2.2][minicbor] | 2.2635 ms | 11.650 ms | 1777386 | 1276218 | 1252558 | 12.678 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.665 ms | 16.066 ms | 1770060 | 1277755 | 1263362 | 14.325 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2895 ms | 2.7616 ms | 1812404 | 1134820 | 1053109 | 10.523 ms |
| [nibblecode 0.1.0][nibblecode] | 510.52 µs | † | 2075936 | 1503676 | 1396923 | 14.179 ms |
| [postcard 1.1.3][postcard] | 1.8158 ms | 4.2400 ms | 1311281 | 1083900 | 1041434 | 9.7639 ms |
| [pot 3.0.1][pot] | 13.896 ms | 31.707 ms | 2604812 | 1482233 | 1298928 | 16.662 ms |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*3.7493 ms\**</span> <span title="populate + encode">*11.220 ms\**</span> | 15.747 ms | 1859886 | 1338076 | 1295351 | 12.511 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*5.4777 ms\**</span> <span title="populate + encode">*9.4125 ms\**</span> | 8.8582 ms | 1859886 | 1338076 | 1295351 | 12.685 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.5416 ms\**</span> <span title="populate + encode">*12.911 ms\**</span> | 12.130 ms | 1859886 | 1338076 | 1295351 | 12.520 ms |
| [rkyv 0.8.16][rkyv] | 988.94 µs | <span title="unvalidated">*2.2079 ms\**</span> <span title="validated upfront with error">*2.6578 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.192 ms |
| [ron 0.12.2][ron] | 45.551 ms | 176.15 ms | 8677703 | 2233642 | 1826180 | 34.666 ms |
| [savefile 0.20.4][savefile] | 870.31 µs | 2.8430 ms | 1791505 | 1128012 | 1051153 | 10.404 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.9234 ms | 3.3098 ms | 1319999 | 1064380 | 1010708 | 9.6973 ms |
| [serde-brief 0.2.0][serde-brief] | 5.4069 ms | 20.609 ms | 6951772 | 1796265 | 1567819 | 23.488 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7577 ms | 5.0384 ms | 1319999 | 1062645 | 1008349 | 9.7542 ms |
| [speedy 0.8.7][speedy] | 766.98 µs | 2.4590 ms | 1584734 | 1119837 | 1037992 | 10.210 ms |
| [wincode 0.5.5][wincode] | 571.07 µs | 2.3248 ms | 1791489 | 1127998 | 1051146 | 10.361 ms |
| [wiring 0.2.4][wiring] | 644.53 µs | 2.7764 ms | 1791489 | 1156963 | 1082815 | 10.948 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*80.687 ns\**</span> | <span title="validated on-demand with error">*721.32 ns\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 51.590 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4905 ns\**</span> <span title="validated upfront with error">*5.4850 ms\**</span> | <span title="unvalidated">*2.7435 µs\**</span> <span title="validated upfront with error">*5.4912 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2451 ns\**</span> <span title="validated upfront with error">*344.12 µs\**</span> | <span title="unvalidated">*377.56 ns\**</span> <span title="validated upfront with error">*344.42 µs\**</span> | <span title="unvalidated">*241.08 ns\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2452 ns\**</span> <span title="validated upfront with error">*441.21 µs\**</span> | <span title="unvalidated">*385.60 ns\**</span> <span title="validated upfront with error">*443.69 µs\**</span> | <span title="unvalidated">*235.55 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1014.2][bilrost] | <span title="encode">*11.10%\**</span> <span title="prepend">*20.15%\**</span> | 14.85% | 47.09% | 62.06% | 64.44% | 2.64% |
| [bin-proto 0.12.8][bin-proto] | 8.46% | 18.31% | 44.80% | 71.21% | 76.36% | 2.99% |
| [bincode 2.0.1][bincode] | 34.52% | 33.01% | 57.08% | 71.86% | 75.55% | 3.25% |
| [bincode 1.3.3][bincode1] | 13.44% | 29.72% | 43.29% | 70.34% | 76.54% | 2.93% |
| [bitcode 0.6.9][bitcode] | 74.46% | 54.28% | 82.64% | 91.48% | 94.40% | 10.72% |
| [borsh 1.7.0][borsh] | 17.39% | 44.60% | 52.74% | 72.46% | 77.29% | 3.11% |
| capnp:<br> [capnp 0.26.0][capnp] | 18.97% <span title="packed">*11.11%\**</span> | <span title="packed">*66.93%\**</span> | 29.46% <span title="packed">*49.66%\**</span> | 51.92% <span title="packed">*62.81%\**</span> | 64.78% <span title="packed">*71.31%\**</span> | 2.11% <span title="packed">*3.06%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 17.57% | 7.51% | 13.35% | 47.38% | 54.79% | 1.44% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.17% | 2.40% | 13.35% | 47.38% | 54.79% | 1.43% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.20% | 6.26% | 13.35% | 47.38% | 54.79% | 1.46% |
| [columnar 0.13.0][columnar] | 57.84% | 33.98% <span title="copy_from">*100.00%\**</span> | 51.96% | 80.59% | 89.56% | 6.48% |
| [compactly 0.1.6][compactly] | 0.79% | 2.29% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 38.54% | 34.29% | 60.81% | 75.59% | 79.61% | 3.26% |
| [dlhn 0.1.7][dlhn] | 11.54% | 17.15% | 61.21% | 74.55% | 76.73% | 3.31% |
| [flatbuffers 25.12.19][flatbuffers] | 10.39% | † | 34.51% | 55.81% | 63.30% | 2.23% |
| [flexbuffers 25.12.19][flexbuffers] | 1.27% | 3.49% | 15.00% | 30.22% | 28.89% | 0.90% |
| json:<br> [flexon 0.4.6][flexon] | 3.29% | 5.22% | 8.55% | 33.58% | 43.56% | 0.87% |
| json:<br> [serde_json 1.0.150][serde_json] | 2.48% | 4.12% | 8.55% | 33.58% | 43.56% | 0.89% |
| json:<br> [simd-json 0.17.0][simd-json] | 4.25% | 4.93% | 8.55% | 33.58% | 43.56% | 0.89% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 53.80% | 22.00% | 55.02% | 69.48% | 70.55% | 2.89% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 4.95% | 11.69% | 45.99% | 63.67% | 65.32% | 2.66% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 45.45% | 30.42% | 46.73% | 65.05% | 67.12% | 2.73% |
| [minicbor 2.2.2][minicbor] | 22.55% | 11.02% | 45.16% | 62.94% | 64.08% | 2.44% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.72% | 7.99% | 45.35% | 62.86% | 63.54% | 2.16% |
| [nanoserde 0.2.1][nanoserde] | 39.59% | 46.49% | 44.29% | 70.78% | 76.22% | 2.94% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 38.67% | 53.42% | 57.46% | 2.19% |
| [postcard 1.1.3][postcard] | 28.12% | 30.28% | 61.21% | 74.11% | 77.08% | 3.17% |
| [pot 3.0.1][pot] | 3.67% | 4.05% | 30.81% | 54.19% | 61.80% | 1.86% |
| protobuf:<br> [buffa 0.7.1][buffa] | <span title="encode">*13.62%\**</span> <span title="populate + encode">*4.55%\**</span> | 8.15% | 43.16% | 60.03% | 61.97% | 2.48% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*9.32%\**</span> <span title="populate + encode">*5.42%\**</span> | 14.50% | 43.16% | 60.03% | 61.97% | 2.44% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*9.21%\**</span> <span title="populate + encode">*3.95%\**</span> | 10.59% | 43.16% | 60.03% | 61.97% | 2.47% |
| [rkyv 0.8.16][rkyv] | 51.62% | <span title="unvalidated">*58.15%\**</span> <span title="validated upfront with error">*48.31%\**</span> | 38.67% | 58.05% | 66.32% | 2.35% |
| [ron 0.12.2][ron] | 1.12% | 0.73% | 9.25% | 35.96% | 43.95% | 0.89% |
| [savefile 0.20.4][savefile] | 58.66% | 45.16% | 44.80% | 71.21% | 76.36% | 2.98% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 17.46% | 38.79% | 60.81% | 75.47% | 79.42% | 3.20% |
| [serde-brief 0.2.0][serde-brief] | 9.44% | 6.23% | 11.55% | 44.72% | 51.20% | 1.32% |
| [serde_bare 0.5.0][serde_bare] | 10.73% | 25.48% | 60.81% | 75.59% | 79.60% | 3.18% |
| [speedy 0.8.7][speedy] | 66.56% | 52.22% | 50.65% | 71.73% | 77.33% | 3.03% |
| [wincode 0.5.5][wincode] | 89.40% | 55.23% | 44.80% | 71.21% | 76.36% | 2.99% |
| [wiring 0.2.4][wiring] | 79.21% | 46.25% | 44.80% | 69.43% | 74.13% | 2.83% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.54%\**</span> | <span title="validated on-demand with error">*52.34%\**</span> | ‡ |
| [columnar 0.13.0][columnar] | 2.41% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.76%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*97.71%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.91%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
