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

## Last updated: 2026-04-18 09:18:31

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.97.0-nightly (e9e32aca5 2026-04-17)
binary: rustc
commit-hash: e9e32aca5a4ffd08cbc29547b039d64b92a2c03b
commit-date: 2026-04-17
host: x86_64-unknown-linux-gnu
release: 1.97.0-nightly
LLVM version: 22.1.2
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*435.32 µs\**</span> <span title="prepend">*421.52 µs\**</span> | 2.5522 ms | 904.90 µs | 804955 | 328941 | 284849 | 4.1650 ms |
| [bin-proto 0.12.7][bin-proto] | 4.1940 ms | 4.8893 ms | † | 1045784 | 373127 | 311553 | 4.4968 ms |
| [bincode 2.0.1][bincode] | 343.51 µs | 2.1754 ms | 693.15 µs | 741295 | 303944 | 256422 | 3.6378 ms |
| [bincode 1.3.3][bincode1] | 550.68 µs | 2.1001 ms | 630.00 µs | 1045784 | 373127 | 311553 | 4.4626 ms |
| [bitcode 0.6.6][bitcode] | 146.08 µs | 1.4524 ms | 62.411 µs | 703710 | 288826 | 227322 | 2.4978 ms |
| [borsh 1.5.7][borsh] | 558.86 µs | 2.1540 ms | † | 885780 | 362204 | 286248 | 4.0988 ms |
| [capnp 0.23.2][capnp] | 463.10 µs | † | † | 1443216 | 513986 | 426532 | 6.1641 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 614.59 µs | 5.6078 ms | 4.7997 ms | 1407835 | 403440 | 323561 | 4.9985 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1534 ms | 11.243 ms | † | 1407835 | 403440 | 323561 | 4.9913 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9906 ms | 4.9084 ms | 3.2268 ms | 1407835 | 403440 | 323561 | 4.8914 ms |
| [columnar 0.11.1][columnar] | 260.30 µs | 2.1893 ms <span title="copy_from">*810.86 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.3587 ms |
| [compactly 0.1.6][compactly] | 27.049 ms | 20.354 ms | † | 241251 | 241453 | 241263 | 101.14 µs |
| [databuf 0.5.0][databuf] | 260.92 µs | 2.0606 ms | 657.69 µs | 765778 | 311715 | 263914 | 3.4549 ms |
| [dlhn 0.1.7][dlhn] | 618.36 µs | 2.6119 ms | † | 724953 | 301446 | 253056 | 3.2073 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0302 ms | † | † | 1276368 | 468539 | 388381 | 4.7650 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.6324 ms | 7.5871 ms | 5.8769 ms | 1829756 | 714318 | 691541 | 8.5156 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.7425 ms | 3.7705 ms | † | 1827461 | 470560 | 360727 | 5.4381 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.8696 ms | 6.0149 ms | † | 1827461 | 470560 | 360727 | 5.8551 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1294 ms | 4.7766 ms | † | 1827461 | 470560 | 360727 | 5.5553 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.1588 ms | 2.6909 ms | † | 764996 | 315291 | 264212 | 3.7538 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4530 ms | 3.0916 ms | 1.3891 ms | 784997 | 325384 | 277608 | 3.7483 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 366.97 µs | 2.4668 ms | 811.75 µs | 784997 | 325384 | 277608 | 3.7507 ms |
| [minicbor 1.0.0][minicbor] | 537.47 µs | 3.0027 ms | 1.4108 ms | 817830 | 332671 | 284034 | 3.8944 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4209 ms | 4.1478 ms | 2.5735 ms | 818669 | 332556 | 284797 | 4.2618 ms |
| [nanoserde 0.2.1][nanoserde] | 260.97 µs | 2.3183 ms | † | 1045784 | 373127 | 311553 | 4.1680 ms |
| [nibblecode 0.1.0][nibblecode] | 196.47 µs | † | † | 1011487 | 492588 | 426911 | 5.4104 ms |
| [postcard 1.1.1][postcard] | 420.88 µs | 2.2774 ms | 712.51 µs | 724953 | 302399 | 252968 | 3.1722 ms |
| [pot 3.0.1][pot] | 2.1430 ms | 6.3213 ms | 4.5715 ms | 971922 | 372513 | 303636 | 4.4114 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*938.96 µs\**</span> <span title="populate + encode">*2.4882 ms\**</span> | 3.4082 ms | † | 884628 | 363130 | 314959 | 4.4045 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.1916 ms\**</span> <span title="populate + encode">*3.0464 ms\**</span> | 3.8400 ms | † | 884628 | 363130 | 314959 | 4.3933 ms |
| [rkyv 0.8.10][rkyv] | 245.29 µs | <span title="unvalidated">*1.5468 ms\**</span> <span title="validated upfront with error">*1.8986 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5524 ms |
| [ron 0.10.1][ron] | 11.539 ms | 24.410 ms | 24.211 ms | 1607459 | 449158 | 349324 | 5.5336 ms |
| [savefile 0.18.6][savefile] | 196.86 µs | 2.1198 ms | † | 1045800 | 373139 | 311562 | 4.2211 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 662.66 µs | 2.5274 ms | † | 765778 | 311743 | 263822 | 3.4684 ms |
| [serde-brief 0.1.1][serde-brief] | 1.4030 ms | 4.7299 ms | 3.0439 ms | 1584946 | 413733 | 339964 | 4.8578 ms |
| [serde_bare 0.5.0][serde_bare] | 683.31 µs | 2.0723 ms | † | 765778 | 311715 | 263914 | 3.4694 ms |
| [speedy 0.8.7][speedy] | 200.24 µs | 1.7384 ms | 367.41 µs | 885780 | 362204 | 286248 | 3.9285 ms |
| [wincode 0.5.3][wincode] | 174.54 µs | 1.7521 ms | 373.52 µs | 1045784 | 373127 | 311553 | 4.1478 ms |
| [wiring 0.2.4][wiring] | 193.57 µs | 2.0676 ms | † | 1045784 | 337930 | 275808 | 3.6362 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*78.537 ns\**</span> | <span title="validated on-demand with error">*132.43 µs\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 23.354 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4891 ns\**</span> <span title="validated upfront with error">*2.0816 ms\**</span> | <span title="unvalidated">*55.305 µs\**</span> <span title="validated upfront with error">*2.1226 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2450 ns\**</span> <span title="validated upfront with error">*235.35 µs\**</span> | <span title="unvalidated">*10.535 µs\**</span> <span title="validated upfront with error">*246.38 µs\**</span> | <span title="unvalidated">*7.5008 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2455 ns\**</span> <span title="validated upfront with error">*340.32 µs\**</span> | <span title="unvalidated">*10.419 µs\**</span> <span title="validated upfront with error">*349.26 µs\**</span> | <span title="unvalidated">*7.4972 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*33.56%\**</span> <span title="prepend">*34.66%\**</span> | 31.77% | 6.90% | 29.97% | 73.40% | 79.80% | 2.43% |
| [bin-proto 0.12.7][bin-proto] | 3.48% | 16.58% | † | 23.07% | 64.71% | 72.96% | 2.25% |
| [bincode 2.0.1][bincode] | 42.53% | 37.27% | 9.00% | 32.54% | 79.44% | 88.65% | 2.78% |
| [bincode 1.3.3][bincode1] | 26.53% | 38.61% | 9.91% | 23.07% | 64.71% | 72.96% | 2.27% |
| [bitcode 0.6.6][bitcode] | 100.00% | 55.83% | 100.00% | 34.28% | 83.60% | 100.00% | 4.05% |
| [borsh 1.5.7][borsh] | 26.14% | 37.64% | † | 27.24% | 66.66% | 79.41% | 2.47% |
| [capnp 0.23.2][capnp] | 31.54% | † | † | 16.72% | 46.98% | 53.30% | 1.64% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 23.77% | 14.46% | 1.30% | 17.14% | 59.85% | 70.26% | 2.02% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.63% | 7.21% | † | 17.14% | 59.85% | 70.26% | 2.03% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.34% | 16.52% | 1.93% | 17.14% | 59.85% | 70.26% | 2.07% |
| [columnar 0.11.1][columnar] | 56.12% | 37.04% <span title="copy_from">*100.00%\**</span> | † | 23.07% | 65.22% | 77.34% | 2.32% |
| [compactly 0.1.6][compactly] | 0.54% | 3.98% | † | 100.00% | 100.00% | 94.22% | 100.00% |
| [databuf 0.5.0][databuf] | 55.99% | 39.35% | 9.49% | 31.50% | 77.46% | 86.13% | 2.93% |
| [dlhn 0.1.7][dlhn] | 23.62% | 31.04% | † | 33.28% | 80.10% | 89.83% | 3.15% |
| [flatbuffers 25.12.19][flatbuffers] | 14.18% | † | † | 18.90% | 51.53% | 58.53% | 2.12% |
| [flexbuffers 25.2.10][flexbuffers] | 2.20% | 10.69% | 1.06% | 13.18% | 33.80% | 32.87% | 1.19% |
| json:<br> [flexon 0.4.5][flexon] | 5.33% | 21.51% | † | 13.20% | 51.31% | 63.02% | 1.86% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.78% | 13.48% | † | 13.20% | 51.31% | 63.02% | 1.73% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.86% | 16.98% | † | 13.20% | 51.31% | 63.02% | 1.82% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 12.61% | 30.13% | † | 31.54% | 76.58% | 86.04% | 2.69% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.05% | 26.23% | 4.49% | 30.73% | 74.21% | 81.89% | 2.70% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 39.81% | 32.87% | 7.69% | 30.73% | 74.21% | 81.89% | 2.70% |
| [minicbor 1.0.0][minicbor] | 27.18% | 27.00% | 4.42% | 29.50% | 72.58% | 80.03% | 2.60% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.69% | 19.55% | 2.43% | 29.47% | 72.61% | 79.82% | 2.37% |
| [nanoserde 0.2.1][nanoserde] | 55.98% | 34.98% | † | 23.07% | 64.71% | 72.96% | 2.43% |
| [nibblecode 0.1.0][nibblecode] | 74.35% | † | † | 23.85% | 49.02% | 53.25% | 1.87% |
| [postcard 1.1.1][postcard] | 34.71% | 35.60% | 8.76% | 33.28% | 79.85% | 89.86% | 3.19% |
| [pot 3.0.1][pot] | 6.82% | 12.83% | 1.37% | 24.82% | 64.82% | 74.87% | 2.29% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*15.56%\**</span> <span title="populate + encode">*5.87%\**</span> | 23.79% | † | 27.27% | 66.49% | 72.18% | 2.30% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.26%\**</span> <span title="populate + encode">*4.80%\**</span> | 21.12% | † | 27.27% | 66.49% | 72.18% | 2.30% |
| [rkyv 0.8.10][rkyv] | 59.55% | <span title="unvalidated">*52.42%\**</span> <span title="validated upfront with error">*42.71%\**</span> | † | 23.85% | 61.36% | 69.74% | 2.22% |
| [ron 0.10.1][ron] | 1.27% | 3.32% | 0.26% | 15.01% | 53.76% | 65.07% | 1.83% |
| [savefile 0.18.6][savefile] | 74.21% | 38.25% | † | 23.07% | 64.71% | 72.96% | 2.40% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.04% | 32.08% | † | 31.50% | 77.45% | 86.16% | 2.92% |
| [serde-brief 0.1.1][serde-brief] | 10.41% | 17.14% | 2.05% | 15.22% | 58.36% | 66.87% | 2.08% |
| [serde_bare 0.5.0][serde_bare] | 21.38% | 39.13% | † | 31.50% | 77.46% | 86.13% | 2.92% |
| [speedy 0.8.7][speedy] | 72.95% | 46.64% | 16.99% | 27.24% | 66.66% | 79.41% | 2.57% |
| [wincode 0.5.3][wincode] | 83.69% | 46.28% | 16.71% | 23.07% | 64.71% | 72.96% | 2.44% |
| [wiring 0.2.4][wiring] | 75.47% | 39.22% | † | 23.07% | 71.45% | 82.42% | 2.78% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.59%\**</span> | <span title="validated on-demand with error">*7.87%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.33% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.84%\**</span> <span title="validated upfront with error">*0.49%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*98.90%\**</span> <span title="validated upfront with error">*4.23%\**</span> | <span title="unvalidated">*99.95%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.98%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.2063 ms\**</span> <span title="prepend">*8.6419 ms\**</span> | 7.8567 ms | 8625005 | 6443961 | 6231572 | 74.234 ms |
| [bin-proto 0.12.7][bin-proto] | 8.5317 ms | 10.037 ms | 6000008 | 5378500 | 5346908 | 8.4645 ms |
| [bincode 2.0.1][bincode] | 2.8859 ms | 1.0758 ms | 6000005 | 5378497 | 5346882 | 8.6194 ms |
| [bincode 1.3.3][bincode1] | 4.8252 ms | 4.7513 ms | 6000008 | 5378500 | 5346908 | 8.5848 ms |
| [bitcode 0.6.6][bitcode] | 1.3124 ms | 817.11 µs | 6000006 | 5182295 | 4921841 | 13.239 ms |
| [borsh 1.5.7][borsh] | 5.9822 ms | 4.1386 ms | 6000004 | 5378496 | 5346866 | 8.4278 ms |
| [capnp 0.23.2][capnp] | 6.0864 ms | † | 14000088 | 7130367 | 6046182 | 80.449 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 8.9863 ms | 50.055 ms | 13125016 | 7524114 | 6757437 | 91.698 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 63.621 ms | 110.92 ms | 13122324 | 7524660 | 6759128 | 92.010 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 36.356 ms | 43.142 ms | 13122324 | 7524660 | 6759128 | 91.930 ms |
| [columnar 0.11.1][columnar] | 1.7852 ms | 1.4504 ms <span title="copy_from">*685.52 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.6656 ms |
| [compactly 0.1.6][compactly] | 355.34 ms | 282.69 ms | 4846786 | 4850065 | 4846903 | 1.6082 ms |
| [databuf 0.5.0][databuf] | 2.4170 ms | 5.4122 ms | 6000003 | 5378495 | 5346897 | 8.5376 ms |
| [dlhn 0.1.7][dlhn] | 5.5708 ms | 6.9466 ms | 6000003 | 5378495 | 5346897 | 8.6166 ms |
| [flatbuffers 25.12.19][flatbuffers] | 477.48 µs | † | 6000024 | 5378434 | 5346878 | 8.8666 ms |
| [flexbuffers 25.2.10][flexbuffers] | 102.84 ms | 79.215 ms | 26609424 | 11901040 | 12486322 | 151.32 ms |
| json:<br> [flexon 0.4.5][flexon] | 76.999 ms | 55.260 ms | 26192883 | 9566084 | 8584671 | 155.93 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 88.700 ms | 98.499 ms | 26192883 | 9566084 | 8584671 | 156.27 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 52.988 ms | 72.504 ms | 26192883 | 9566084 | 8584671 | 155.82 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 19.427 ms | 5.0529 ms | 7500005 | 6058442 | 6014500 | 10.856 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 16.239 ms | 17.154 ms | 8125006 | 6494876 | 6391037 | 74.640 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 735.40 µs | 5.2335 ms | 8125006 | 6494876 | 6391037 | 76.403 ms |
| [minicbor 1.0.0][minicbor] | 5.1927 ms | 11.521 ms | 8125006 | 6494907 | 6390894 | 70.087 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.33 ms | 28.306 ms | 8125037 | 6493484 | 6386940 | 74.648 ms |
| [nanoserde 0.2.1][nanoserde] | 1.6688 ms | 892.29 µs | 6000008 | 5378500 | 5346908 | 8.3980 ms |
| [nibblecode 0.1.0][nibblecode] | 201.32 µs | † | 6000008 | 5378500 | 5346908 | 8.6482 ms |
| [postcard 1.1.1][postcard] | 513.71 µs | 1.0186 ms | 6000003 | 5378495 | 5346897 | 8.6963 ms |
| [pot 3.0.1][pot] | 36.443 ms | 61.718 ms | 10122342 | 6814618 | 6852252 | 81.932 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*7.8714 ms\**</span> <span title="populate + encode">*8.4851 ms\**</span> | 13.538 ms | 8750000 | 6665735 | 6421877 | 72.723 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.379 ms\**</span> <span title="populate + encode">*30.767 ms\**</span> | 29.490 ms | 8750000 | 6665735 | 6421877 | 79.902 ms |
| [rkyv 0.8.10][rkyv] | 201.69 µs | <span title="unvalidated">*203.85 µs\**</span> <span title="validated upfront with error">*203.26 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.5255 ms |
| [ron 0.10.1][ron] | 169.00 ms | 514.75 ms | 22192885 | 8970395 | 8137334 | 151.33 ms |
| [savefile 0.18.6][savefile] | 202.97 µs | 203.25 µs | 6000024 | 5378519 | 5346896 | 8.6415 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 4.8293 ms | 2.8815 ms | 6000004 | 5378496 | 5346866 | 8.6858 ms |
| [serde-brief 0.1.1][serde-brief] | 17.340 ms | 37.107 ms | 15750015 | 8024540 | 6813667 | 93.961 ms |
| [serde_bare 0.5.0][serde_bare] | 6.0303 ms | 4.8477 ms | 6000003 | 5378495 | 5346897 | 8.7026 ms |
| [speedy 0.8.7][speedy] | 202.05 µs | 201.91 µs | 6000004 | 5378496 | 5346866 | 8.5107 ms |
| [wincode 0.5.3][wincode] | 203.16 µs | 202.05 µs | 6000008 | 5378500 | 5346908 | 8.7097 ms |
| [wiring 0.2.4][wiring] | 201.49 µs | 340.34 µs | 6000008 | 5378952 | 5346905 | 8.6496 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*111.71 ns\**</span> | <span title="validated on-demand with error">*2.0309 ms\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 21.953 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4888 ns\**</span> <span title="validated upfront with error">*44.536 ns\**</span> | <span title="unvalidated">*77.829 µs\**</span> <span title="validated upfront with error">*77.946 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*1.5566 ns\**</span> | <span title="unvalidated">*38.892 µs\**</span> <span title="validated upfront with error">*38.915 µs\**</span> | <span title="unvalidated">*101.33 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2445 ns\**</span> <span title="validated upfront with error">*4.9915 ns\**</span> | <span title="unvalidated">*38.895 µs\**</span> <span title="validated upfront with error">*38.903 µs\**</span> | <span title="unvalidated">*101.19 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.79%\**</span> <span title="prepend">*2.33%\**</span> | 2.57% | 56.19% | 75.27% | 77.78% | 2.17% |
| [bin-proto 0.12.7][bin-proto] | 2.36% | 2.01% | 80.78% | 90.18% | 90.65% | 19.00% |
| [bincode 2.0.1][bincode] | 6.98% | 18.77% | 80.78% | 90.18% | 90.65% | 18.66% |
| [bincode 1.3.3][bincode1] | 4.17% | 4.25% | 80.78% | 90.18% | 90.65% | 18.73% |
| [bitcode 0.6.6][bitcode] | 15.34% | 24.71% | 80.78% | 93.59% | 98.48% | 12.15% |
| [borsh 1.5.7][borsh] | 3.37% | 4.88% | 80.78% | 90.18% | 90.65% | 19.08% |
| [capnp 0.23.2][capnp] | 3.31% | † | 34.62% | 68.02% | 80.16% | 2.00% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 2.24% | 0.40% | 36.93% | 64.46% | 71.73% | 1.75% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.32% | 0.18% | 36.94% | 64.46% | 71.71% | 1.75% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.55% | 0.47% | 36.94% | 64.46% | 71.71% | 1.75% |
| [columnar 0.11.1][columnar] | 11.28% | 13.92% <span title="copy_from">*29.45%\**</span> | 80.78% | 90.18% | 90.65% | 18.56% |
| [compactly 0.1.6][compactly] | 0.06% | 0.07% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 8.33% | 3.73% | 80.78% | 90.18% | 90.65% | 18.84% |
| [dlhn 0.1.7][dlhn] | 3.61% | 2.91% | 80.78% | 90.18% | 90.65% | 18.66% |
| [flatbuffers 25.12.19][flatbuffers] | 42.16% | † | 80.78% | 90.18% | 90.65% | 18.14% |
| [flexbuffers 25.2.10][flexbuffers] | 0.20% | 0.25% | 18.21% | 40.75% | 38.82% | 1.06% |
| json:<br> [flexon 0.4.5][flexon] | 0.26% | 0.37% | 18.50% | 50.70% | 56.46% | 1.03% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.23% | 0.20% | 18.50% | 50.70% | 56.46% | 1.03% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.38% | 0.28% | 18.50% | 50.70% | 56.46% | 1.03% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.04% | 4.00% | 64.62% | 80.05% | 80.59% | 14.81% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.24% | 1.18% | 59.65% | 74.68% | 75.84% | 2.15% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 27.38% | 3.86% | 59.65% | 74.68% | 75.84% | 2.10% |
| [minicbor 1.0.0][minicbor] | 3.88% | 1.75% | 59.65% | 74.67% | 75.84% | 2.29% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.71% | 59.65% | 74.69% | 75.89% | 2.15% |
| [nanoserde 0.2.1][nanoserde] | 12.06% | 22.63% | 80.78% | 90.18% | 90.65% | 19.15% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 80.78% | 90.18% | 90.65% | 18.60% |
| [postcard 1.1.1][postcard] | 39.19% | 19.82% | 80.78% | 90.18% | 90.65% | 18.49% |
| [pot 3.0.1][pot] | 0.55% | 0.33% | 47.88% | 71.17% | 70.73% | 1.96% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*2.56%\**</span> <span title="populate + encode">*2.37%\**</span> | 1.49% | 55.39% | 72.76% | 75.47% | 2.21% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.40%\**</span> <span title="populate + encode">*0.65%\**</span> | 0.68% | 55.39% | 72.76% | 75.47% | 2.01% |
| [rkyv 0.8.10][rkyv] | 99.82% | <span title="unvalidated">*99.05%\**</span> <span title="validated upfront with error">*99.34%\**</span> | 80.78% | 90.18% | 90.65% | 18.86% |
| [ron 0.10.1][ron] | 0.12% | 0.04% | 21.84% | 54.07% | 59.56% | 1.06% |
| [savefile 0.18.6][savefile] | 99.19% | 99.34% | 80.78% | 90.17% | 90.65% | 18.61% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 4.17% | 7.01% | 80.78% | 90.18% | 90.65% | 18.52% |
| [serde-brief 0.1.1][serde-brief] | 1.16% | 0.54% | 30.77% | 60.44% | 71.14% | 1.71% |
| [serde_bare 0.5.0][serde_bare] | 3.34% | 4.17% | 80.78% | 90.18% | 90.65% | 18.48% |
| [speedy 0.8.7][speedy] | 99.64% | 100.00% | 80.78% | 90.18% | 90.65% | 18.90% |
| [wincode 0.5.3][wincode] | 99.09% | 99.93% | 80.78% | 90.18% | 90.65% | 18.46% |
| [wiring 0.2.4][wiring] | 99.92% | 59.33% | 80.78% | 90.17% | 90.65% | 18.59% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.11%\**</span> | <span title="validated on-demand with error">*1.92%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.67% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*2.79%\**</span> | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*49.90%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*79.95%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.94%\**</span> | <span title="unvalidated">*99.86%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.93%\**</span> | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*99.97%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*873.07 µs\**</span> <span title="prepend">*787.61 µs\**</span> | 3.1538 ms | 1.7028 ms | 489348 | 281173 | 249360 | 2.6454 ms |
| [bin-proto 0.12.7][bin-proto] | 1.7535 ms | 2.9070 ms | † | 566975 | 239350 | 231475 | 2.4645 ms |
| [bincode 2.0.1][bincode] | 317.83 µs | 1.8043 ms | 795.54 µs | 367413 | 221291 | 206242 | 2.0440 ms |
| [bincode 1.3.3][bincode1] | 623.37 µs | 1.8756 ms | 871.38 µs | 569975 | 240525 | 231884 | 2.4861 ms |
| [bitcode 0.6.6][bitcode] | 131.16 µs | 1.2651 ms | 169.29 µs | 327688 | 200947 | 182040 | 732.99 µs |
| [borsh 1.5.7][borsh] | 564.19 µs | 1.8036 ms | † | 446595 | 234236 | 209834 | 2.0903 ms |
| [capnp 0.23.2][capnp] | 440.85 µs | † | † | 803896 | 335606 | 280744 | 3.5749 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 730.27 µs | 4.6499 ms | 3.3912 ms | 1109831 | 344745 | 274333 | 3.4646 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.7124 ms | 9.9726 ms | † | 1109821 | 344751 | 274345 | 3.4470 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9191 ms | 4.7292 ms | 3.4963 ms | 1109821 | 344751 | 274345 | 3.4193 ms |
| [columnar 0.11.1][columnar] | 288.16 µs | 1.9063 ms <span title="copy_from">*777.24 µs\**</span> | † | 563728 | 249696 | 217582 | 1.5852 ms |
| [compactly 0.1.6][compactly] | 11.639 ms | 11.325 ms | † | 149292 | 149433 | 149304 | 88.685 µs |
| [databuf 0.5.0][databuf] | 282.47 µs | 1.7368 ms | 791.16 µs | 356311 | 213062 | 198403 | 1.9387 ms |
| [dlhn 0.1.7][dlhn] | 651.64 µs | 2.6040 ms | † | 366496 | 220600 | 205586 | 2.0175 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2703 ms | † | † | 849472 | 347816 | 294871 | 3.4481 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.6486 ms | 7.0751 ms | 5.8076 ms | 1187688 | 557642 | 553730 | 6.2092 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.7179 ms | 4.6486 ms | † | 1623191 | 466527 | 359157 | 5.6841 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.7084 ms | 6.9413 ms | † | 1623191 | 466527 | 359157 | 5.6745 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2232 ms | 4.5739 ms | † | 1623191 | 466527 | 359157 | 5.6896 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 893.78 µs | 2.7547 ms | † | 391251 | 236877 | 220395 | 2.1985 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4557 ms | 2.9905 ms | 1.6821 ms | 424533 | 245214 | 226077 | 2.2407 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 386.31 µs | 2.2157 ms | 930.70 µs | 416025 | 243812 | 224965 | 2.2604 ms |
| [minicbor 1.0.0][minicbor] | 548.58 µs | 3.3630 ms | 1.8560 ms | 428773 | 249857 | 228630 | 2.3254 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1162 ms | 3.9384 ms | 2.8027 ms | 449745 | 252432 | 230965 | 2.3324 ms |
| [nanoserde 0.2.1][nanoserde] | 274.72 µs | 1.9210 ms | † | 567975 | 239930 | 231872 | 2.5095 ms |
| [nibblecode 0.1.0][nibblecode] | 179.63 µs | † | † | 603928 | 432715 | 410090 | 3.6461 ms |
| [postcard 1.1.1][postcard] | 445.50 µs | 2.1027 ms | 822.63 µs | 367489 | 221913 | 207244 | 2.0388 ms |
| [pot 3.0.1][pot] | 2.3727 ms | 5.7274 ms | 4.3746 ms | 599125 | 299158 | 247675 | 2.7160 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.2725 ms\**</span> <span title="populate + encode">*3.0054 ms\**</span> | 3.5692 ms | † | 596811 | 305319 | 268737 | 3.0583 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0391 ms\**</span> <span title="populate + encode">*2.9954 ms\**</span> | 3.8334 ms | † | 596811 | 305319 | 268737 | 3.0428 ms |
| [rkyv 0.8.10][rkyv] | 329.44 µs | <span title="unvalidated">*1.5125 ms\**</span> <span title="validated upfront with error">*1.8739 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3125 ms |
| [ron 0.10.1][ron] | 8.1442 ms | 26.373 ms | 24.656 ms | 1465223 | 434935 | 342907 | 5.5550 ms |
| [savefile 0.18.6][savefile] | 214.43 µs | 1.8221 ms | † | 566991 | 239362 | 231478 | 2.4549 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 615.00 µs | 2.1165 ms | † | 356311 | 212976 | 198423 | 1.9672 ms |
| [serde-brief 0.1.1][serde-brief] | 1.2364 ms | 5.2735 ms | 3.7244 ms | 1276014 | 373898 | 293384 | 3.6147 ms |
| [serde_bare 0.5.0][serde_bare] | 718.63 µs | 2.4074 ms | † | 356311 | 213062 | 198403 | 1.9622 ms |
| [speedy 0.8.7][speedy] | 265.57 µs | 1.6966 ms | 543.24 µs | 449595 | 234970 | 210192 | 2.0576 ms |
| [wincode 0.5.3][wincode] | 204.88 µs | 1.6478 ms | 564.48 µs | 566975 | 239350 | 231475 | 2.4214 ms |
| [wiring 0.2.4][wiring] | 208.59 µs | 1.9286 ms | † | 566975 | 247810 | 225086 | 2.5035 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*89.407 ns\**</span> | <span title="validated on-demand with error">*419.07 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 880.38 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4898 ns\**</span> <span title="validated upfront with error">*2.3743 ms\**</span> | <span title="unvalidated">*1.4176 µs\**</span> <span title="validated upfront with error">*2.3762 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2445 ns\**</span> <span title="validated upfront with error">*253.92 µs\**</span> | <span title="unvalidated">*156.18 ns\**</span> <span title="validated upfront with error">*252.36 µs\**</span> | <span title="unvalidated">*760.55 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2450 ns\**</span> <span title="validated upfront with error">*340.54 µs\**</span> | <span title="unvalidated">*156.22 ns\**</span> <span title="validated upfront with error">*342.21 µs\**</span> | <span title="unvalidated">*772.80 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*15.02%\**</span> <span title="prepend">*16.65%\**</span> | 24.64% | 9.94% | 30.51% | 53.15% | 59.87% | 3.35% |
| [bin-proto 0.12.7][bin-proto] | 7.48% | 26.74% | † | 26.33% | 62.43% | 64.50% | 3.60% |
| [bincode 2.0.1][bincode] | 41.27% | 43.08% | 21.28% | 40.63% | 67.53% | 72.39% | 4.34% |
| [bincode 1.3.3][bincode1] | 21.04% | 41.44% | 19.43% | 26.19% | 62.13% | 64.39% | 3.57% |
| [bitcode 0.6.6][bitcode] | 100.00% | 61.44% | 100.00% | 45.56% | 74.36% | 82.02% | 12.10% |
| [borsh 1.5.7][borsh] | 23.25% | 43.09% | † | 33.43% | 63.80% | 71.15% | 4.24% |
| [capnp 0.23.2][capnp] | 29.75% | † | † | 18.57% | 44.53% | 53.18% | 2.48% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 17.96% | 16.72% | 4.99% | 13.45% | 43.35% | 54.42% | 2.56% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.53% | 7.79% | † | 13.45% | 43.35% | 54.42% | 2.57% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.83% | 16.43% | 4.84% | 13.45% | 43.35% | 54.42% | 2.59% |
| [columnar 0.11.1][columnar] | 45.52% | 40.77% <span title="copy_from">*100.00%\**</span> | † | 26.48% | 59.85% | 68.62% | 5.59% |
| [compactly 0.1.6][compactly] | 1.13% | 6.86% | † | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 46.43% | 44.75% | 21.40% | 41.90% | 70.14% | 75.25% | 4.57% |
| [dlhn 0.1.7][dlhn] | 20.13% | 29.85% | † | 40.73% | 67.74% | 72.62% | 4.40% |
| [flatbuffers 25.12.19][flatbuffers] | 4.01% | † | † | 17.57% | 42.96% | 50.63% | 2.57% |
| [flexbuffers 25.2.10][flexbuffers] | 1.71% | 10.99% | 2.91% | 12.57% | 26.80% | 26.96% | 1.43% |
| json:<br> [flexon 0.4.5][flexon] | 4.83% | 16.72% | † | 9.20% | 32.03% | 41.57% | 1.56% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.54% | 11.20% | † | 9.20% | 32.03% | 41.57% | 1.56% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.90% | 16.99% | † | 9.20% | 32.03% | 41.57% | 1.56% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 14.67% | 28.22% | † | 38.16% | 63.08% | 67.74% | 4.03% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.01% | 25.99% | 10.06% | 35.17% | 60.94% | 66.04% | 3.96% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 33.95% | 35.08% | 18.19% | 35.89% | 61.29% | 66.37% | 3.92% |
| [minicbor 1.0.0][minicbor] | 23.91% | 23.11% | 9.12% | 34.82% | 59.81% | 65.30% | 3.81% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.56% | 19.73% | 6.04% | 33.19% | 59.20% | 64.64% | 3.80% |
| [nanoserde 0.2.1][nanoserde] | 47.74% | 40.46% | † | 26.28% | 62.28% | 64.39% | 3.53% |
| [nibblecode 0.1.0][nibblecode] | 73.02% | † | † | 24.72% | 34.53% | 36.41% | 2.43% |
| [postcard 1.1.1][postcard] | 29.44% | 36.96% | 20.58% | 40.62% | 67.34% | 72.04% | 4.35% |
| [pot 3.0.1][pot] | 5.53% | 13.57% | 3.87% | 24.92% | 49.95% | 60.28% | 3.27% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*10.31%\**</span> <span title="populate + encode">*4.36%\**</span> | 21.78% | † | 25.01% | 48.94% | 55.56% | 2.90% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.62%\**</span> <span title="populate + encode">*4.38%\**</span> | 20.28% | † | 25.01% | 48.94% | 55.56% | 2.91% |
| [rkyv 0.8.10][rkyv] | 39.81% | <span title="unvalidated">*51.39%\**</span> <span title="validated upfront with error">*41.48%\**</span> | † | 24.73% | 58.65% | 68.04% | 3.84% |
| [ron 0.10.1][ron] | 1.61% | 2.95% | 0.69% | 10.19% | 34.36% | 43.54% | 1.60% |
| [savefile 0.18.6][savefile] | 61.17% | 42.66% | † | 26.33% | 62.43% | 64.50% | 3.61% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 21.33% | 36.72% | † | 41.90% | 70.16% | 75.25% | 4.51% |
| [serde-brief 0.1.1][serde-brief] | 10.61% | 14.74% | 4.55% | 11.70% | 39.97% | 50.89% | 2.45% |
| [serde_bare 0.5.0][serde_bare] | 18.25% | 32.29% | † | 41.90% | 70.14% | 75.25% | 4.52% |
| [speedy 0.8.7][speedy] | 49.39% | 45.81% | 31.16% | 33.21% | 63.60% | 71.03% | 4.31% |
| [wincode 0.5.3][wincode] | 64.02% | 47.17% | 29.99% | 26.33% | 62.43% | 64.50% | 3.66% |
| [wiring 0.2.4][wiring] | 62.88% | 40.30% | † | 26.33% | 60.30% | 66.33% | 3.54% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.39%\**</span> | <span title="validated on-demand with error">*37.27%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 0.14% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.02%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*98.41%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5413 ms\**</span> <span title="prepend">*2.5039 ms\**</span> | 8.2239 ms | 1704643 | 1294259 | 1245668 | 11.702 ms |
| [bin-proto 0.12.7][bin-proto] | 4.8797 ms | 6.5073 ms | 1791489 | 1127998 | 1051146 | 10.289 ms |
| [bincode 2.0.1][bincode] | 1.2344 ms | 3.6275 ms | 1406257 | 1117802 | 1062438 | 9.7040 ms |
| [bincode 1.3.3][bincode1] | 3.7692 ms | 4.3090 ms | 1854234 | 1141994 | 1048745 | 10.380 ms |
| [bitcode 0.6.6][bitcode] | 699.45 µs | 2.3733 ms | 971318 | 878034 | 850340 | 2.9494 ms |
| [borsh 1.5.7][borsh] | 2.8806 ms | 2.8938 ms | 1521989 | 1108471 | 1038528 | 10.003 ms |
| [capnp 0.23.2][capnp] | 2.1765 ms | † | 2724288 | 1546992 | 1239111 | 14.955 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.2011 ms | 18.420 ms | 6012539 | 1695215 | 1464951 | 21.428 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.913 ms | 54.000 ms | 6012373 | 1695146 | 1465025 | 21.226 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.8668 ms | 21.507 ms | 6012373 | 1695146 | 1465025 | 22.233 ms |
| [columnar 0.11.1][columnar] | 950.20 µs | 3.7450 ms <span title="copy_from">*1.2698 ms\**</span> | 1544752 | 996728 | 897073 | 4.7054 ms |
| [compactly 0.1.6][compactly] | 64.474 ms | 56.593 ms | 802662 | 803238 | 802689 | 303.56 µs |
| [databuf 0.5.0][databuf] | 1.3005 ms | 3.7603 ms | 1319999 | 1062631 | 1008334 | 8.9847 ms |
| [dlhn 0.1.7][dlhn] | 4.5048 ms | 7.4955 ms | 1311281 | 1077520 | 1046095 | 8.7012 ms |
| [flatbuffers 25.12.19][flatbuffers] | 4.9389 ms | † | 2325620 | 1439185 | 1268060 | 13.534 ms |
| [flexbuffers 25.2.10][flexbuffers] | 39.689 ms | 36.802 ms | 5352680 | 2658295 | 2777967 | 35.204 ms |
| json:<br> [flexon 0.4.5][flexon] | 15.091 ms | 24.722 ms | 9390461 | 2391679 | 1842767 | 35.458 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 21.254 ms | 31.571 ms | 9390461 | 2391679 | 1842767 | 35.026 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.878 ms | 25.660 ms | 9390461 | 2391679 | 1842767 | 35.121 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.3079 ms | 5.9113 ms | 1458773 | 1156055 | 1137788 | 9.9318 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.5379 ms | 10.762 ms | 1745322 | 1261627 | 1228923 | 11.804 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 1.8365 ms | 5.8966 ms | 1794467 | 1273669 | 1242301 | 11.670 ms |
| [minicbor 1.0.0][minicbor] | 2.4795 ms | 11.437 ms | 1777386 | 1276218 | 1252558 | 12.579 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.930 ms | 16.509 ms | 1770060 | 1277755 | 1263362 | 12.797 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2943 ms | 2.7711 ms | 1812404 | 1134820 | 1053109 | 10.364 ms |
| [nibblecode 0.1.0][nibblecode] | 506.22 µs | † | 2075936 | 1541008 | 1432747 | 13.899 ms |
| [postcard 1.1.1][postcard] | 1.7817 ms | 4.1993 ms | 1311281 | 1083900 | 1041434 | 8.8756 ms |
| [pot 3.0.1][pot] | 13.346 ms | 26.938 ms | 2604812 | 1482233 | 1298928 | 16.161 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.4131 ms\**</span> <span title="populate + encode">*9.2995 ms\**</span> | 8.7911 ms | 1859886 | 1338076 | 1295351 | 12.347 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.8031 ms\**</span> <span title="populate + encode">*12.966 ms\**</span> | 12.061 ms | 1859886 | 1338076 | 1295351 | 12.423 ms |
| [rkyv 0.8.10][rkyv] | 983.28 µs | <span title="unvalidated">*2.1945 ms\**</span> <span title="validated upfront with error">*2.6302 ms\**</span> | 2075936 | 1383779 | 1210377 | 12.966 ms |
| [ron 0.10.1][ron] | 43.298 ms | 158.20 ms | 8677703 | 2233642 | 1826180 | 34.773 ms |
| [savefile 0.18.6][savefile] | 861.45 µs | 2.5679 ms | 1791505 | 1128012 | 1051153 | 10.219 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.0322 ms | 3.2771 ms | 1319999 | 1064380 | 1010708 | 8.8122 ms |
| [serde-brief 0.1.1][serde-brief] | 5.4596 ms | 21.996 ms | 6951772 | 1796265 | 1567819 | 24.584 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8040 ms | 5.0192 ms | 1319999 | 1062645 | 1008349 | 8.9181 ms |
| [speedy 0.8.7][speedy] | 742.75 µs | 2.4611 ms | 1584734 | 1119837 | 1037992 | 10.035 ms |
| [wincode 0.5.3][wincode] | 597.45 µs | 2.3282 ms | 1791489 | 1127998 | 1051146 | 10.201 ms |
| [wiring 0.2.4][wiring] | 669.30 µs | 2.7588 ms | 1791489 | 1156963 | 1082815 | 10.662 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*83.395 ns\**</span> | <span title="validated on-demand with error">*724.42 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 58.228 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4896 ns\**</span> <span title="validated upfront with error">*5.6718 ms\**</span> | <span title="unvalidated">*2.7658 µs\**</span> <span title="validated upfront with error">*5.6759 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2448 ns\**</span> <span title="validated upfront with error">*372.63 µs\**</span> | <span title="unvalidated">*377.07 ns\**</span> <span title="validated upfront with error">*370.22 µs\**</span> | <span title="unvalidated">*237.23 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*418.35 µs\**</span> | <span title="unvalidated">*385.43 ns\**</span> <span title="validated upfront with error">*419.24 µs\**</span> | <span title="unvalidated">*234.09 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.15%\**</span> <span title="prepend">*20.22%\**</span> | 15.44% | 47.09% | 62.06% | 64.44% | 2.59% |
| [bin-proto 0.12.7][bin-proto] | 10.37% | 19.51% | 44.80% | 71.21% | 76.36% | 2.95% |
| [bincode 2.0.1][bincode] | 41.01% | 35.00% | 57.08% | 71.86% | 75.55% | 3.13% |
| [bincode 1.3.3][bincode1] | 13.43% | 29.47% | 43.29% | 70.34% | 76.54% | 2.92% |
| [bitcode 0.6.6][bitcode] | 72.37% | 53.50% | 82.64% | 91.48% | 94.40% | 10.29% |
| [borsh 1.5.7][borsh] | 17.57% | 43.88% | 52.74% | 72.46% | 77.29% | 3.03% |
| [capnp 0.23.2][capnp] | 23.26% | † | 29.46% | 51.92% | 64.78% | 2.03% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 15.81% | 6.89% | 13.35% | 47.38% | 54.79% | 1.42% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.12% | 2.35% | 13.35% | 47.38% | 54.79% | 1.43% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.13% | 5.90% | 13.35% | 47.38% | 54.79% | 1.37% |
| [columnar 0.11.1][columnar] | 53.28% | 33.91% <span title="copy_from">*100.00%\**</span> | 51.96% | 80.59% | 89.48% | 6.45% |
| [compactly 0.1.6][compactly] | 0.79% | 2.24% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 38.93% | 33.77% | 60.81% | 75.59% | 79.61% | 3.38% |
| [dlhn 0.1.7][dlhn] | 11.24% | 16.94% | 61.21% | 74.55% | 76.73% | 3.49% |
| [flatbuffers 25.12.19][flatbuffers] | 10.25% | † | 34.51% | 55.81% | 63.30% | 2.24% |
| [flexbuffers 25.2.10][flexbuffers] | 1.28% | 3.45% | 15.00% | 30.22% | 28.89% | 0.86% |
| json:<br> [flexon 0.4.5][flexon] | 3.35% | 5.14% | 8.55% | 33.58% | 43.56% | 0.86% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.38% | 4.02% | 8.55% | 33.58% | 43.56% | 0.87% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.26% | 4.95% | 8.55% | 33.58% | 43.56% | 0.86% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 21.93% | 21.48% | 55.02% | 69.48% | 70.55% | 3.06% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 5.31% | 11.80% | 45.99% | 63.67% | 65.32% | 2.57% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 27.56% | 21.53% | 44.73% | 63.06% | 64.61% | 2.60% |
| [minicbor 1.0.0][minicbor] | 20.42% | 11.10% | 45.16% | 62.94% | 64.08% | 2.41% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.69% | 7.69% | 45.35% | 62.86% | 63.54% | 2.37% |
| [nanoserde 0.2.1][nanoserde] | 39.11% | 45.82% | 44.29% | 70.78% | 76.22% | 2.93% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 38.67% | 52.12% | 56.02% | 2.18% |
| [postcard 1.1.1][postcard] | 28.41% | 30.24% | 61.21% | 74.11% | 77.08% | 3.42% |
| [pot 3.0.1][pot] | 3.79% | 4.71% | 30.81% | 54.19% | 61.80% | 1.88% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.35%\**</span> <span title="populate + encode">*5.44%\**</span> | 14.44% | 43.16% | 60.03% | 61.97% | 2.46% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*8.72%\**</span> <span title="populate + encode">*3.90%\**</span> | 10.53% | 43.16% | 60.03% | 61.97% | 2.44% |
| [rkyv 0.8.10][rkyv] | 51.48% | <span title="unvalidated">*57.86%\**</span> <span title="validated upfront with error">*48.28%\**</span> | 38.67% | 58.05% | 66.32% | 2.34% |
| [ron 0.10.1][ron] | 1.17% | 0.80% | 9.25% | 35.96% | 43.95% | 0.87% |
| [savefile 0.18.6][savefile] | 58.76% | 49.45% | 44.80% | 71.21% | 76.36% | 2.97% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.69% | 38.75% | 60.81% | 75.47% | 79.42% | 3.44% |
| [serde-brief 0.1.1][serde-brief] | 9.27% | 5.77% | 11.55% | 44.72% | 51.20% | 1.23% |
| [serde_bare 0.5.0][serde_bare] | 10.54% | 25.30% | 60.81% | 75.59% | 79.60% | 3.40% |
| [speedy 0.8.7][speedy] | 68.15% | 51.59% | 50.65% | 71.73% | 77.33% | 3.02% |
| [wincode 0.5.3][wincode] | 84.73% | 54.54% | 44.80% | 71.21% | 76.36% | 2.98% |
| [wiring 0.2.4][wiring] | 75.63% | 46.03% | 44.80% | 69.43% | 74.13% | 2.85% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.49%\**</span> | <span title="validated on-demand with error">*52.05%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 2.14% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.63%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*98.68%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.83%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1013.0
[bin-proto]: https://crates.io/crates/bin-proto/0.12.7
[bincode]: https://crates.io/crates/bincode/2.0.1
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.6
[borsh]: https://crates.io/crates/borsh/1.5.7
[capnp]: https://crates.io/crates/capnp/0.23.2
[cbor4ii]: https://crates.io/crates/cbor4ii/1.0.0
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[columnar]: https://crates.io/crates/columnar/0.11.1
[compactly]: https://crates.io/crates/compactly/0.1.6
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.12.19
[flexbuffers]: https://crates.io/crates/flexbuffers/25.2.10
[flexon]: https://crates.io/crates/flexon/0.4.5
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
[wincode]: https://crates.io/crates/wincode/0.5.3
[wiring]: https://crates.io/crates/wiring/0.2.4
[zerompk]: https://crates.io/crates/zerompk/0.3.2


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
