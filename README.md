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

## Last updated: 2026-04-21 04:21:57

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.97.0-nightly (66da6cae1 2026-04-20)
binary: rustc
commit-hash: 66da6cae1a6f12e9585493ab8f8f19cf753091fd
commit-date: 2026-04-20
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
BogoMIPS:                                4890.86
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*445.68 µs\**</span> <span title="prepend">*419.01 µs\**</span> | 2.5651 ms | 930.09 µs | 804955 | 328941 | 284849 | 4.0704 ms |
| [bin-proto 0.12.7][bin-proto] | 4.3148 ms | 4.6743 ms | † | 1045784 | 373127 | 311553 | 4.4746 ms |
| [bincode 2.0.1][bincode] | 328.90 µs | 2.1713 ms | 668.77 µs | 741295 | 303944 | 256422 | 3.6437 ms |
| [bincode 1.3.3][bincode1] | 522.69 µs | 2.0268 ms | 607.65 µs | 1045784 | 373127 | 311553 | 4.3859 ms |
| [bitcode 0.6.6][bitcode] | 139.79 µs | 1.4636 ms | 60.760 µs | 703710 | 288826 | 227322 | 2.5382 ms |
| [borsh 1.5.7][borsh] | 553.02 µs | 2.1386 ms | † | 885780 | 362204 | 286248 | 4.1948 ms |
| [capnp 0.23.2][capnp] | 446.31 µs | † | † | 1443216 | 513986 | 426532 | 6.1086 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 618.62 µs | 5.2244 ms | 3.3666 ms | 1407835 | 403440 | 323561 | 5.0243 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1949 ms | 11.289 ms | † | 1407835 | 403440 | 323561 | 4.9324 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9329 ms | 4.7536 ms | 3.1219 ms | 1407835 | 403440 | 323561 | 4.7053 ms |
| [columnar 0.11.1][columnar] | 251.62 µs | 2.1547 ms <span title="copy_from">*832.50 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.2132 ms |
| [compactly 0.1.6][compactly] | 27.020 ms | 20.295 ms | † | 241251 | 241453 | 241263 | 92.955 µs |
| [databuf 0.5.0][databuf] | 261.59 µs | 2.0414 ms | 647.83 µs | 765778 | 311715 | 263914 | 3.4207 ms |
| [dlhn 0.1.7][dlhn] | 684.53 µs | 2.5471 ms | † | 724953 | 301446 | 253056 | 3.1822 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0279 ms | † | † | 1276368 | 468539 | 388381 | 4.7696 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.9667 ms | 7.2157 ms | 5.4444 ms | 1829756 | 714318 | 691541 | 8.5595 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.8235 ms | 3.9212 ms | † | 1827461 | 470560 | 360727 | 5.4899 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 4.0483 ms | 5.9870 ms | † | 1827461 | 470560 | 360727 | 5.9168 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1112 ms | 4.7427 ms | † | 1827461 | 470560 | 360727 | 5.5676 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 927.14 µs | 2.5936 ms | † | 764996 | 315291 | 264212 | 3.6039 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4364 ms | 3.0683 ms | 1.3774 ms | 784997 | 325384 | 277608 | 3.7722 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 355.52 µs | 2.5369 ms | 787.98 µs | 784997 | 325384 | 277608 | 3.8710 ms |
| [minicbor 1.0.0][minicbor] | 415.23 µs | 3.2102 ms | 1.5396 ms | 817830 | 332671 | 284034 | 3.9884 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3416 ms | 4.0039 ms | 2.4591 ms | 818669 | 332556 | 284797 | 4.2582 ms |
| [nanoserde 0.2.1][nanoserde] | 242.39 µs | 2.1074 ms | † | 1045784 | 373127 | 311553 | 4.1240 ms |
| [nibblecode 0.1.0][nibblecode] | 187.26 µs | † | † | 1011487 | 473180 | 403990 | 5.2661 ms |
| [postcard 1.1.1][postcard] | 424.63 µs | 2.2821 ms | 746.75 µs | 724953 | 302399 | 252968 | 3.1927 ms |
| [pot 3.0.1][pot] | 2.2524 ms | 6.5162 ms | 4.8635 ms | 971922 | 372513 | 303636 | 4.3780 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*948.92 µs\**</span> <span title="populate + encode">*2.4684 ms\**</span> | 3.4420 ms | † | 884628 | 363130 | 314959 | 4.4022 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2320 ms\**</span> <span title="populate + encode">*3.0701 ms\**</span> | 4.0591 ms | † | 884628 | 363130 | 314959 | 4.4965 ms |
| [rkyv 0.8.10][rkyv] | 252.45 µs | <span title="unvalidated">*1.5514 ms\**</span> <span title="validated upfront with error">*1.9074 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5057 ms |
| [ron 0.10.1][ron] | 12.490 ms | 24.171 ms | 22.002 ms | 1607459 | 449158 | 349324 | 5.6064 ms |
| [savefile 0.18.6][savefile] | 194.49 µs | 2.1046 ms | † | 1045800 | 373139 | 311562 | 4.2085 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 674.56 µs | 2.5797 ms | † | 765778 | 311743 | 263822 | 3.5867 ms |
| [serde-brief 0.1.1][serde-brief] | 1.4055 ms | 4.6163 ms | 2.8505 ms | 1584946 | 413733 | 339964 | 4.7950 ms |
| [serde_bare 0.5.0][serde_bare] | 704.10 µs | 2.0813 ms | † | 765778 | 311715 | 263914 | 3.4318 ms |
| [speedy 0.8.7][speedy] | 191.05 µs | 1.7405 ms | 375.08 µs | 885780 | 362204 | 286248 | 3.8204 ms |
| [wincode 0.5.3][wincode] | 165.70 µs | 1.7668 ms | 365.60 µs | 1045784 | 373127 | 311553 | 4.1871 ms |
| [wiring 0.2.4][wiring] | 196.68 µs | 2.0631 ms | † | 1045784 | 337930 | 275808 | 3.6364 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*81.729 ns\**</span> | <span title="validated on-demand with error">*132.15 µs\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 23.057 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4901 ns\**</span> <span title="validated upfront with error">*2.1514 ms\**</span> | <span title="unvalidated">*52.077 µs\**</span> <span title="validated upfront with error">*2.4888 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2449 ns\**</span> <span title="validated upfront with error">*242.17 µs\**</span> | <span title="unvalidated">*10.577 µs\**</span> <span title="validated upfront with error">*255.11 µs\**</span> | <span title="unvalidated">*7.6540 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2452 ns\**</span> <span title="validated upfront with error">*362.68 µs\**</span> | <span title="unvalidated">*10.505 µs\**</span> <span title="validated upfront with error">*378.36 µs\**</span> | <span title="unvalidated">*7.6100 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*31.37%\**</span> <span title="prepend">*33.36%\**</span> | 32.45% | 6.53% | 29.97% | 73.40% | 79.80% | 2.28% |
| [bin-proto 0.12.7][bin-proto] | 3.24% | 17.81% | † | 23.07% | 64.71% | 72.96% | 2.08% |
| [bincode 2.0.1][bincode] | 42.50% | 38.34% | 9.09% | 32.54% | 79.44% | 88.65% | 2.55% |
| [bincode 1.3.3][bincode1] | 26.74% | 41.07% | 10.00% | 23.07% | 64.71% | 72.96% | 2.12% |
| [bitcode 0.6.6][bitcode] | 100.00% | 56.88% | 100.00% | 34.28% | 83.60% | 100.00% | 3.66% |
| [borsh 1.5.7][borsh] | 25.28% | 38.93% | † | 27.24% | 66.66% | 79.41% | 2.22% |
| [capnp 0.23.2][capnp] | 31.32% | † | † | 16.72% | 46.98% | 53.30% | 1.52% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 22.60% | 15.93% | 1.80% | 17.14% | 59.85% | 70.26% | 1.85% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.38% | 7.37% | † | 17.14% | 59.85% | 70.26% | 1.88% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.23% | 17.51% | 1.95% | 17.14% | 59.85% | 70.26% | 1.98% |
| [columnar 0.11.1][columnar] | 55.56% | 38.64% <span title="copy_from">*100.00%\**</span> | † | 23.07% | 65.22% | 77.34% | 2.21% |
| [compactly 0.1.6][compactly] | 0.52% | 4.10% | † | 100.00% | 100.00% | 94.22% | 100.00% |
| [databuf 0.5.0][databuf] | 53.44% | 40.78% | 9.38% | 31.50% | 77.46% | 86.13% | 2.72% |
| [dlhn 0.1.7][dlhn] | 20.42% | 32.68% | † | 33.28% | 80.10% | 89.83% | 2.92% |
| [flatbuffers 25.12.19][flatbuffers] | 13.60% | † | † | 18.90% | 51.53% | 58.53% | 1.95% |
| [flexbuffers 25.2.10][flexbuffers] | 2.01% | 11.54% | 1.12% | 13.18% | 33.80% | 32.87% | 1.09% |
| json:<br> [flexon 0.4.5][flexon] | 4.95% | 21.23% | † | 13.20% | 51.31% | 63.02% | 1.69% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.45% | 13.91% | † | 13.20% | 51.31% | 63.02% | 1.57% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.62% | 17.55% | † | 13.20% | 51.31% | 63.02% | 1.67% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 15.08% | 32.10% | † | 31.54% | 76.58% | 86.04% | 2.58% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.73% | 27.13% | 4.41% | 30.73% | 74.21% | 81.89% | 2.46% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 39.32% | 32.82% | 7.71% | 30.73% | 74.21% | 81.89% | 2.40% |
| [minicbor 1.0.0][minicbor] | 33.67% | 25.93% | 3.95% | 29.50% | 72.58% | 80.03% | 2.33% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.62% | 20.79% | 2.47% | 29.47% | 72.61% | 79.82% | 2.18% |
| [nanoserde 0.2.1][nanoserde] | 57.67% | 39.50% | † | 23.07% | 64.71% | 72.96% | 2.25% |
| [nibblecode 0.1.0][nibblecode] | 74.65% | † | † | 23.85% | 51.03% | 56.27% | 1.77% |
| [postcard 1.1.1][postcard] | 32.92% | 36.48% | 8.14% | 33.28% | 79.85% | 89.86% | 2.91% |
| [pot 3.0.1][pot] | 6.21% | 12.78% | 1.25% | 24.82% | 64.82% | 74.87% | 2.12% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*14.73%\**</span> <span title="populate + encode">*5.66%\**</span> | 24.19% | † | 27.27% | 66.49% | 72.18% | 2.11% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.35%\**</span> <span title="populate + encode">*4.55%\**</span> | 20.51% | † | 27.27% | 66.49% | 72.18% | 2.07% |
| [rkyv 0.8.10][rkyv] | 55.37% | <span title="unvalidated">*53.66%\**</span> <span title="validated upfront with error">*43.65%\**</span> | † | 23.85% | 61.36% | 69.74% | 2.06% |
| [ron 0.10.1][ron] | 1.12% | 3.44% | 0.28% | 15.01% | 53.76% | 65.07% | 1.66% |
| [savefile 0.18.6][savefile] | 71.88% | 39.56% | † | 23.07% | 64.71% | 72.96% | 2.21% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.72% | 32.27% | † | 31.50% | 77.45% | 86.16% | 2.59% |
| [serde-brief 0.1.1][serde-brief] | 9.95% | 18.03% | 2.13% | 15.22% | 58.36% | 66.87% | 1.94% |
| [serde_bare 0.5.0][serde_bare] | 19.85% | 40.00% | † | 31.50% | 77.46% | 86.13% | 2.71% |
| [speedy 0.8.7][speedy] | 73.17% | 47.83% | 16.20% | 27.24% | 66.66% | 79.41% | 2.43% |
| [wincode 0.5.3][wincode] | 84.36% | 47.12% | 16.62% | 23.07% | 64.71% | 72.96% | 2.22% |
| [wiring 0.2.4][wiring] | 71.07% | 40.35% | † | 23.07% | 71.45% | 82.42% | 2.56% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.52%\**</span> | <span title="validated on-demand with error">*7.95%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.40% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.17%\**</span> <span title="validated upfront with error">*0.42%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.32%\**</span> <span title="validated upfront with error">*4.12%\**</span> | <span title="unvalidated">*99.43%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.78%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.3466 ms\**</span> <span title="prepend">*8.6886 ms\**</span> | 7.8224 ms | 8625005 | 6443961 | 6231572 | 71.672 ms |
| [bin-proto 0.12.7][bin-proto] | 8.1487 ms | 10.367 ms | 6000008 | 5378500 | 5346908 | 8.5528 ms |
| [bincode 2.0.1][bincode] | 2.4205 ms | 1.0756 ms | 6000005 | 5378497 | 5346882 | 8.7665 ms |
| [bincode 1.3.3][bincode1] | 5.8746 ms | 5.8240 ms | 6000008 | 5378500 | 5346908 | 8.6033 ms |
| [bitcode 0.6.6][bitcode] | 1.3311 ms | 807.90 µs | 6000006 | 5182295 | 4921841 | 13.786 ms |
| [borsh 1.5.7][borsh] | 6.0885 ms | 4.3409 ms | 6000004 | 5378496 | 5346866 | 8.5017 ms |
| [capnp 0.23.2][capnp] | 6.0395 ms | † | 14000088 | 7130367 | 6046182 | 83.551 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 9.0252 ms | 49.699 ms | 13125016 | 7524114 | 6757437 | 98.654 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 59.865 ms | 124.49 ms | 13122324 | 7524660 | 6759128 | 91.040 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 33.993 ms | 42.094 ms | 13122324 | 7524660 | 6759128 | 91.971 ms |
| [columnar 0.11.1][columnar] | 1.7513 ms | 1.4854 ms <span title="copy_from">*708.58 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.6078 ms |
| [compactly 0.1.6][compactly] | 357.45 ms | 288.84 ms | 4846786 | 4850065 | 4846903 | 1.9728 ms |
| [databuf 0.5.0][databuf] | 2.4160 ms | 5.3734 ms | 6000003 | 5378495 | 5346897 | 8.5227 ms |
| [dlhn 0.1.7][dlhn] | 6.2147 ms | 6.8129 ms | 6000003 | 5378495 | 5346897 | 8.5613 ms |
| [flatbuffers 25.12.19][flatbuffers] | 445.12 µs | † | 6000024 | 5378434 | 5346878 | 8.5256 ms |
| [flexbuffers 25.2.10][flexbuffers] | 111.14 ms | 81.298 ms | 26609424 | 11901040 | 12486322 | 152.16 ms |
| json:<br> [flexon 0.4.5][flexon] | 76.580 ms | 55.787 ms | 26192883 | 9566084 | 8584671 | 158.33 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 88.547 ms | 100.41 ms | 26192883 | 9566084 | 8584671 | 157.39 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 53.178 ms | 67.364 ms | 26192883 | 9566084 | 8584671 | 156.54 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.0968 ms | 5.0116 ms | 7500005 | 6058442 | 6014500 | 10.226 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 14.475 ms | 16.739 ms | 8125006 | 6494876 | 6391037 | 73.108 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 736.99 µs | 5.2749 ms | 8125006 | 6494876 | 6391037 | 71.341 ms |
| [minicbor 1.0.0][minicbor] | 6.0625 ms | 11.467 ms | 8125006 | 6494907 | 6390894 | 69.712 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 119.06 ms | 26.688 ms | 8125037 | 6493484 | 6386940 | 69.727 ms |
| [nanoserde 0.2.1][nanoserde] | 1.7450 ms | 827.34 µs | 6000008 | 5378500 | 5346908 | 8.7945 ms |
| [nibblecode 0.1.0][nibblecode] | 149.04 µs | † | 6000008 | 5378500 | 5346908 | 8.6416 ms |
| [postcard 1.1.1][postcard] | 479.99 µs | 1.1006 ms | 6000003 | 5378495 | 5346897 | 8.7826 ms |
| [pot 3.0.1][pot] | 39.988 ms | 72.312 ms | 10122342 | 6814618 | 6852252 | 82.904 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*7.8474 ms\**</span> <span title="populate + encode">*8.7257 ms\**</span> | 11.257 ms | 8750000 | 6665735 | 6421877 | 73.740 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*15.948 ms\**</span> <span title="populate + encode">*32.724 ms\**</span> | 29.458 ms | 8750000 | 6665735 | 6421877 | 79.974 ms |
| [rkyv 0.8.10][rkyv] | 148.86 µs | <span title="unvalidated">*150.10 µs\**</span> <span title="validated upfront with error">*150.15 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.7907 ms |
| [ron 0.10.1][ron] | 170.66 ms | 507.15 ms | 22192885 | 8970395 | 8137334 | 152.12 ms |
| [savefile 0.18.6][savefile] | 149.06 µs | 148.87 µs | 6000024 | 5378519 | 5346896 | 8.4397 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1209 ms | 4.0353 ms | 6000004 | 5378496 | 5346866 | 9.0248 ms |
| [serde-brief 0.1.1][serde-brief] | 17.802 ms | 36.334 ms | 15750015 | 8024540 | 6813667 | 96.117 ms |
| [serde_bare 0.5.0][serde_bare] | 6.1345 ms | 4.2345 ms | 6000003 | 5378495 | 5346897 | 8.9880 ms |
| [speedy 0.8.7][speedy] | 148.74 µs | 149.64 µs | 6000004 | 5378496 | 5346866 | 8.7515 ms |
| [wincode 0.5.3][wincode] | 149.31 µs | 148.75 µs | 6000008 | 5378500 | 5346908 | 8.3376 ms |
| [wiring 0.2.4][wiring] | 150.20 µs | 342.86 µs | 6000008 | 5378952 | 5346905 | 8.8059 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*112.49 ns\**</span> | <span title="validated on-demand with error">*2.0238 ms\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 21.788 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4890 ns\**</span> <span title="validated upfront with error">*44.712 ns\**</span> | <span title="unvalidated">*77.905 µs\**</span> <span title="validated upfront with error">*77.929 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2451 ns\**</span> <span title="validated upfront with error">*1.5580 ns\**</span> | <span title="unvalidated">*38.910 µs\**</span> <span title="validated upfront with error">*38.987 µs\**</span> | <span title="unvalidated">*78.903 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2446 ns\**</span> <span title="validated upfront with error">*2.4899 ns\**</span> | <span title="unvalidated">*38.901 µs\**</span> <span title="validated upfront with error">*39.032 µs\**</span> | <span title="unvalidated">*77.210 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.02%\**</span> <span title="prepend">*1.71%\**</span> | 1.90% | 56.19% | 75.27% | 77.78% | 2.75% |
| [bin-proto 0.12.7][bin-proto] | 1.83% | 1.43% | 80.78% | 90.18% | 90.65% | 23.07% |
| [bincode 2.0.1][bincode] | 6.15% | 13.83% | 80.78% | 90.18% | 90.65% | 22.50% |
| [bincode 1.3.3][bincode1] | 2.53% | 2.55% | 80.78% | 90.18% | 90.65% | 22.93% |
| [bitcode 0.6.6][bitcode] | 11.17% | 18.41% | 80.78% | 93.59% | 98.48% | 14.31% |
| [borsh 1.5.7][borsh] | 2.44% | 3.43% | 80.78% | 90.18% | 90.65% | 23.20% |
| [capnp 0.23.2][capnp] | 2.46% | † | 34.62% | 68.02% | 80.16% | 2.36% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.65% | 0.30% | 36.93% | 64.46% | 71.73% | 2.00% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.25% | 0.12% | 36.94% | 64.46% | 71.71% | 2.17% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.44% | 0.35% | 36.94% | 64.46% | 71.71% | 2.15% |
| [columnar 0.11.1][columnar] | 8.49% | 10.01% <span title="copy_from">*20.99%\**</span> | 80.78% | 90.18% | 90.65% | 22.92% |
| [compactly 0.1.6][compactly] | 0.04% | 0.05% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 6.16% | 2.77% | 80.78% | 90.18% | 90.65% | 23.15% |
| [dlhn 0.1.7][dlhn] | 2.39% | 2.18% | 80.78% | 90.18% | 90.65% | 23.04% |
| [flatbuffers 25.12.19][flatbuffers] | 33.42% | † | 80.78% | 90.18% | 90.65% | 23.14% |
| [flexbuffers 25.2.10][flexbuffers] | 0.13% | 0.18% | 18.21% | 40.75% | 38.82% | 1.30% |
| json:<br> [flexon 0.4.5][flexon] | 0.19% | 0.27% | 18.50% | 50.70% | 56.46% | 1.25% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.15% | 18.50% | 50.70% | 56.46% | 1.25% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.28% | 0.22% | 18.50% | 50.70% | 56.46% | 1.26% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 7.09% | 2.97% | 64.62% | 80.05% | 80.59% | 19.29% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.03% | 0.89% | 59.65% | 74.68% | 75.84% | 2.70% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 20.18% | 2.82% | 59.65% | 74.68% | 75.84% | 2.77% |
| [minicbor 1.0.0][minicbor] | 2.45% | 1.30% | 59.65% | 74.67% | 75.84% | 2.83% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.56% | 59.65% | 74.69% | 75.89% | 2.83% |
| [nanoserde 0.2.1][nanoserde] | 8.52% | 17.98% | 80.78% | 90.18% | 90.65% | 22.43% |
| [nibblecode 0.1.0][nibblecode] | 99.80% | † | 80.78% | 90.18% | 90.65% | 22.83% |
| [postcard 1.1.1][postcard] | 30.99% | 13.52% | 80.78% | 90.18% | 90.65% | 22.46% |
| [pot 3.0.1][pot] | 0.37% | 0.21% | 47.88% | 71.17% | 70.73% | 2.38% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.90%\**</span> <span title="populate + encode">*1.70%\**</span> | 1.32% | 55.39% | 72.76% | 75.47% | 2.68% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*0.93%\**</span> <span title="populate + encode">*0.45%\**</span> | 0.50% | 55.39% | 72.76% | 75.47% | 2.47% |
| [rkyv 0.8.10][rkyv] | 99.92% | <span title="unvalidated">*99.10%\**</span> <span title="validated upfront with error">*99.07%\**</span> | 80.78% | 90.18% | 90.65% | 22.44% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 21.84% | 54.07% | 59.56% | 1.30% |
| [savefile 0.18.6][savefile] | 99.79% | 99.92% | 80.78% | 90.17% | 90.65% | 23.38% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.90% | 3.69% | 80.78% | 90.18% | 90.65% | 21.86% |
| [serde-brief 0.1.1][serde-brief] | 0.84% | 0.41% | 30.77% | 60.44% | 71.14% | 2.05% |
| [serde_bare 0.5.0][serde_bare] | 2.42% | 3.51% | 80.78% | 90.18% | 90.65% | 21.95% |
| [speedy 0.8.7][speedy] | 100.00% | 99.41% | 80.78% | 90.18% | 90.65% | 22.54% |
| [wincode 0.5.3][wincode] | 99.62% | 100.00% | 80.78% | 90.18% | 90.65% | 23.66% |
| [wiring 0.2.4][wiring] | 99.03% | 43.39% | 80.78% | 90.17% | 90.65% | 22.40% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.11%\**</span> | <span title="validated on-demand with error">*1.92%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.71% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*2.78%\**</span> | <span title="unvalidated">*49.93%\**</span> <span title="validated upfront with error">*49.92%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*79.88%\**</span> | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*99.78%\**</span> | <span title="unvalidated">*97.85%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*49.99%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.66%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*879.92 µs\**</span> <span title="prepend">*792.93 µs\**</span> | 3.1225 ms | 1.7397 ms | 489348 | 281173 | 249360 | 2.6000 ms |
| [bin-proto 0.12.7][bin-proto] | 1.7822 ms | 2.7631 ms | † | 566975 | 239350 | 231475 | 2.4451 ms |
| [bincode 2.0.1][bincode] | 306.87 µs | 1.7912 ms | 783.59 µs | 367413 | 221291 | 206242 | 2.0239 ms |
| [bincode 1.3.3][bincode1] | 569.51 µs | 1.8401 ms | 862.79 µs | 569975 | 240525 | 231884 | 2.5026 ms |
| [bitcode 0.6.6][bitcode] | 127.85 µs | 1.2686 ms | 171.53 µs | 327688 | 200947 | 182040 | 764.26 µs |
| [borsh 1.5.7][borsh] | 539.75 µs | 1.8431 ms | † | 446595 | 234236 | 209834 | 2.0396 ms |
| [capnp 0.23.2][capnp] | 433.66 µs | † | † | 803896 | 335606 | 280744 | 3.5091 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 741.59 µs | 4.8583 ms | 3.5032 ms | 1109831 | 344745 | 274333 | 3.3940 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.6717 ms | 10.531 ms | † | 1109821 | 344751 | 274345 | 3.5060 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8431 ms | 4.5394 ms | 3.3214 ms | 1109821 | 344751 | 274345 | 3.4056 ms |
| [columnar 0.11.1][columnar] | 283.51 µs | 1.9063 ms <span title="copy_from">*770.40 µs\**</span> | † | 563728 | 249696 | 217582 | 1.5634 ms |
| [compactly 0.1.6][compactly] | 11.653 ms | 11.229 ms | † | 149292 | 149433 | 149304 | 82.995 µs |
| [databuf 0.5.0][databuf] | 290.30 µs | 1.7325 ms | 820.07 µs | 356311 | 213062 | 198403 | 1.9003 ms |
| [dlhn 0.1.7][dlhn] | 678.13 µs | 2.6966 ms | † | 366496 | 220600 | 205586 | 2.1166 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.3143 ms | † | † | 849472 | 347816 | 294871 | 3.5903 ms |
| [flexbuffers 25.2.10][flexbuffers] | 8.1546 ms | 6.9021 ms | 5.5961 ms | 1187688 | 557642 | 553730 | 6.2405 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.6828 ms | 4.6348 ms | † | 1623191 | 466527 | 359157 | 5.7807 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.7846 ms | 6.9181 ms | † | 1623191 | 466527 | 359157 | 5.9342 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1957 ms | 4.5998 ms | † | 1623191 | 466527 | 359157 | 5.7093 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 720.62 µs | 2.7016 ms | † | 391251 | 236877 | 220395 | 2.2013 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4222 ms | 2.9944 ms | 1.6826 ms | 424533 | 245214 | 226077 | 2.2587 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 392.75 µs | 2.1953 ms | 925.09 µs | 416025 | 243812 | 224965 | 2.3341 ms |
| [minicbor 1.0.0][minicbor] | 554.76 µs | 3.3699 ms | 1.9049 ms | 428773 | 249857 | 228630 | 2.2718 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0807 ms | 3.8086 ms | 2.7638 ms | 449745 | 252432 | 230965 | 2.2779 ms |
| [nanoserde 0.2.1][nanoserde] | 266.85 µs | 1.9110 ms | † | 567975 | 239930 | 231872 | 2.5352 ms |
| [nibblecode 0.1.0][nibblecode] | 180.33 µs | † | † | 603928 | 429924 | 405288 | 3.5207 ms |
| [postcard 1.1.1][postcard] | 447.35 µs | 2.0954 ms | 819.35 µs | 367489 | 221913 | 207244 | 2.0063 ms |
| [pot 3.0.1][pot] | 2.3903 ms | 6.1543 ms | 5.0208 ms | 599125 | 299158 | 247675 | 2.9261 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.2683 ms\**</span> <span title="populate + encode">*3.0031 ms\**</span> | 3.5124 ms | † | 596811 | 305319 | 268737 | 3.0053 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0915 ms\**</span> <span title="populate + encode">*3.0579 ms\**</span> | 3.8248 ms | † | 596811 | 305319 | 268737 | 3.0402 ms |
| [rkyv 0.8.10][rkyv] | 331.17 µs | <span title="unvalidated">*1.5095 ms\**</span> <span title="validated upfront with error">*1.8467 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3205 ms |
| [ron 0.10.1][ron] | 8.1625 ms | 24.330 ms | 22.919 ms | 1465223 | 434935 | 342907 | 5.5819 ms |
| [savefile 0.18.6][savefile] | 216.22 µs | 1.7981 ms | † | 566991 | 239362 | 231478 | 2.4586 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 631.69 µs | 2.1096 ms | † | 356311 | 212976 | 198423 | 1.9450 ms |
| [serde-brief 0.1.1][serde-brief] | 1.1680 ms | 5.2623 ms | 3.5484 ms | 1276014 | 373898 | 293384 | 3.6386 ms |
| [serde_bare 0.5.0][serde_bare] | 753.63 µs | 2.3047 ms | † | 356311 | 213062 | 198403 | 1.9841 ms |
| [speedy 0.8.7][speedy] | 268.50 µs | 1.6872 ms | 537.99 µs | 449595 | 234970 | 210192 | 2.0827 ms |
| [wincode 0.5.3][wincode] | 209.28 µs | 1.6456 ms | 549.15 µs | 566975 | 239350 | 231475 | 2.6051 ms |
| [wiring 0.2.4][wiring] | 210.50 µs | 1.9128 ms | † | 566975 | 247810 | 225086 | 2.5899 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*80.429 ns\**</span> | <span title="validated on-demand with error">*416.76 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 870.65 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4895 ns\**</span> <span title="validated upfront with error">*2.1825 ms\**</span> | <span title="unvalidated">*1.4194 µs\**</span> <span title="validated upfront with error">*2.1829 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*253.59 µs\**</span> | <span title="unvalidated">*156.35 ns\**</span> <span title="validated upfront with error">*252.10 µs\**</span> | <span title="unvalidated">*724.80 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2445 ns\**</span> <span title="validated upfront with error">*320.47 µs\**</span> | <span title="unvalidated">*157.19 ns\**</span> <span title="validated upfront with error">*323.01 µs\**</span> | <span title="unvalidated">*747.54 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.53%\**</span> <span title="prepend">*16.12%\**</span> | 24.67% | 9.86% | 30.51% | 53.15% | 59.87% | 3.19% |
| [bin-proto 0.12.7][bin-proto] | 7.17% | 27.88% | † | 26.33% | 62.43% | 64.50% | 3.39% |
| [bincode 2.0.1][bincode] | 41.66% | 43.01% | 21.89% | 40.63% | 67.53% | 72.39% | 4.10% |
| [bincode 1.3.3][bincode1] | 22.45% | 41.87% | 19.88% | 26.19% | 62.13% | 64.39% | 3.32% |
| [bitcode 0.6.6][bitcode] | 100.00% | 60.73% | 100.00% | 45.56% | 74.36% | 82.02% | 10.86% |
| [borsh 1.5.7][borsh] | 23.69% | 41.80% | † | 33.43% | 63.80% | 71.15% | 4.07% |
| [capnp 0.23.2][capnp] | 29.48% | † | † | 18.57% | 44.53% | 53.18% | 2.37% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 17.24% | 15.86% | 4.90% | 13.45% | 43.35% | 54.42% | 2.45% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.48% | 7.32% | † | 13.45% | 43.35% | 54.42% | 2.37% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.94% | 16.97% | 5.16% | 13.45% | 43.35% | 54.42% | 2.44% |
| [columnar 0.11.1][columnar] | 45.10% | 40.41% <span title="copy_from">*100.00%\**</span> | † | 26.48% | 59.85% | 68.62% | 5.31% |
| [compactly 0.1.6][compactly] | 1.10% | 6.86% | † | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 44.04% | 44.47% | 20.92% | 41.90% | 70.14% | 75.25% | 4.37% |
| [dlhn 0.1.7][dlhn] | 18.85% | 28.57% | † | 40.73% | 67.74% | 72.62% | 3.92% |
| [flatbuffers 25.12.19][flatbuffers] | 3.86% | † | † | 17.57% | 42.96% | 50.63% | 2.31% |
| [flexbuffers 25.2.10][flexbuffers] | 1.57% | 11.16% | 3.07% | 12.57% | 26.80% | 26.96% | 1.33% |
| json:<br> [flexon 0.4.5][flexon] | 4.77% | 16.62% | † | 9.20% | 32.03% | 41.57% | 1.44% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.38% | 11.14% | † | 9.20% | 32.03% | 41.57% | 1.40% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.82% | 16.75% | † | 9.20% | 32.03% | 41.57% | 1.45% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 17.74% | 28.52% | † | 38.16% | 63.08% | 67.74% | 3.77% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.99% | 25.73% | 10.19% | 35.17% | 60.94% | 66.04% | 3.67% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 32.55% | 35.09% | 18.54% | 35.89% | 61.29% | 66.37% | 3.56% |
| [minicbor 1.0.0][minicbor] | 23.05% | 22.86% | 9.00% | 34.82% | 59.81% | 65.30% | 3.65% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.52% | 20.23% | 6.21% | 33.19% | 59.20% | 64.64% | 3.64% |
| [nanoserde 0.2.1][nanoserde] | 47.91% | 40.31% | † | 26.28% | 62.28% | 64.39% | 3.27% |
| [nibblecode 0.1.0][nibblecode] | 70.90% | † | † | 24.72% | 34.76% | 36.84% | 2.36% |
| [postcard 1.1.1][postcard] | 28.58% | 36.77% | 20.93% | 40.62% | 67.34% | 72.04% | 4.14% |
| [pot 3.0.1][pot] | 5.35% | 12.52% | 3.42% | 24.92% | 49.95% | 60.28% | 2.84% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*10.08%\**</span> <span title="populate + encode">*4.26%\**</span> | 21.93% | † | 25.01% | 48.94% | 55.56% | 2.76% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.71%\**</span> <span title="populate + encode">*4.18%\**</span> | 20.14% | † | 25.01% | 48.94% | 55.56% | 2.73% |
| [rkyv 0.8.10][rkyv] | 38.61% | <span title="unvalidated">*51.04%\**</span> <span title="validated upfront with error">*41.72%\**</span> | † | 24.73% | 58.65% | 68.04% | 3.58% |
| [ron 0.10.1][ron] | 1.57% | 3.17% | 0.75% | 10.19% | 34.36% | 43.54% | 1.49% |
| [savefile 0.18.6][savefile] | 59.13% | 42.85% | † | 26.33% | 62.43% | 64.50% | 3.38% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.24% | 36.52% | † | 41.90% | 70.16% | 75.25% | 4.27% |
| [serde-brief 0.1.1][serde-brief] | 10.95% | 14.64% | 4.83% | 11.70% | 39.97% | 50.89% | 2.28% |
| [serde_bare 0.5.0][serde_bare] | 16.96% | 33.43% | † | 41.90% | 70.14% | 75.25% | 4.18% |
| [speedy 0.8.7][speedy] | 47.62% | 45.66% | 31.88% | 33.21% | 63.60% | 71.03% | 3.98% |
| [wincode 0.5.3][wincode] | 61.09% | 46.82% | 31.24% | 26.33% | 62.43% | 64.50% | 3.19% |
| [wiring 0.2.4][wiring] | 60.74% | 40.28% | † | 26.33% | 60.30% | 66.33% | 3.20% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.55%\**</span> | <span title="validated on-demand with error">*37.52%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 0.14% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.02%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.47%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*96.96%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5439 ms\**</span> <span title="prepend">*2.5474 ms\**</span> | 8.5404 ms | 1704643 | 1294259 | 1245668 | 12.243 ms |
| [bin-proto 0.12.7][bin-proto] | 4.9726 ms | 6.3709 ms | 1791489 | 1127998 | 1051146 | 10.817 ms |
| [bincode 2.0.1][bincode] | 1.2321 ms | 3.8106 ms | 1406257 | 1117802 | 1062438 | 9.7686 ms |
| [bincode 1.3.3][bincode1] | 3.8492 ms | 4.5376 ms | 1854234 | 1141994 | 1048745 | 10.796 ms |
| [bitcode 0.6.6][bitcode] | 687.92 µs | 2.3687 ms | 971318 | 878034 | 850340 | 2.8788 ms |
| [borsh 1.5.7][borsh] | 2.8282 ms | 2.8787 ms | 1521989 | 1108471 | 1038528 | 10.134 ms |
| [capnp 0.23.2][capnp] | 2.2231 ms | † | 2724288 | 1546992 | 1239111 | 14.909 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.2333 ms | 18.406 ms | 6012539 | 1695215 | 1464951 | 21.500 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.045 ms | 52.223 ms | 6012373 | 1695146 | 1465025 | 21.772 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.9302 ms | 21.037 ms | 6012373 | 1695146 | 1465025 | 21.230 ms |
| [columnar 0.11.1][columnar] | 884.15 µs | 3.7657 ms <span title="copy_from">*1.2952 ms\**</span> | 1544752 | 996728 | 897073 | 4.7843 ms |
| [compactly 0.1.6][compactly] | 64.765 ms | 56.472 ms | 802662 | 803238 | 802689 | 276.82 µs |
| [databuf 0.5.0][databuf] | 1.3165 ms | 3.7588 ms | 1319999 | 1062631 | 1008334 | 8.9312 ms |
| [dlhn 0.1.7][dlhn] | 4.2034 ms | 7.3852 ms | 1311281 | 1077520 | 1046095 | 8.8481 ms |
| [flatbuffers 25.12.19][flatbuffers] | 5.0113 ms | † | 2325620 | 1439185 | 1268060 | 13.617 ms |
| [flexbuffers 25.2.10][flexbuffers] | 41.553 ms | 34.432 ms | 5352680 | 2658295 | 2777967 | 34.842 ms |
| json:<br> [flexon 0.4.5][flexon] | 14.818 ms | 24.863 ms | 9390461 | 2391679 | 1842767 | 34.674 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 21.339 ms | 31.594 ms | 9390461 | 2391679 | 1842767 | 35.732 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 12.109 ms | 25.332 ms | 9390461 | 2391679 | 1842767 | 35.297 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.7538 ms | 5.9051 ms | 1458773 | 1156055 | 1137788 | 10.014 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.9777 ms | 10.862 ms | 1745322 | 1261627 | 1228923 | 11.649 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 1.8315 ms | 5.8472 ms | 1794467 | 1273669 | 1242301 | 11.631 ms |
| [minicbor 1.0.0][minicbor] | 2.4366 ms | 11.793 ms | 1777386 | 1276218 | 1252558 | 12.612 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.596 ms | 15.749 ms | 1770060 | 1277755 | 1263362 | 12.592 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2968 ms | 2.7658 ms | 1812404 | 1134820 | 1053109 | 10.307 ms |
| [nibblecode 0.1.0][nibblecode] | 502.31 µs | † | 2075936 | 1520951 | 1415182 | 14.137 ms |
| [postcard 1.1.1][postcard] | 1.7905 ms | 4.2445 ms | 1311281 | 1083900 | 1041434 | 8.6630 ms |
| [pot 3.0.1][pot] | 14.125 ms | 31.079 ms | 2604812 | 1482233 | 1298928 | 16.799 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.4114 ms\**</span> <span title="populate + encode">*9.4103 ms\**</span> | 8.7832 ms | 1859886 | 1338076 | 1295351 | 12.340 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.7553 ms\**</span> <span title="populate + encode">*13.189 ms\**</span> | 12.290 ms | 1859886 | 1338076 | 1295351 | 12.480 ms |
| [rkyv 0.8.10][rkyv] | 977.77 µs | <span title="unvalidated">*2.1997 ms\**</span> <span title="validated upfront with error">*2.6286 ms\**</span> | 2075936 | 1383779 | 1210377 | 12.960 ms |
| [ron 0.10.1][ron] | 43.928 ms | 159.23 ms | 8677703 | 2233642 | 1826180 | 34.357 ms |
| [savefile 0.18.6][savefile] | 846.32 µs | 2.5582 ms | 1791505 | 1128012 | 1051153 | 10.531 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1778 ms | 3.2449 ms | 1319999 | 1064380 | 1010708 | 9.3382 ms |
| [serde-brief 0.1.1][serde-brief] | 5.5783 ms | 22.030 ms | 6951772 | 1796265 | 1567819 | 23.913 ms |
| [serde_bare 0.5.0][serde_bare] | 4.6358 ms | 4.9635 ms | 1319999 | 1062645 | 1008349 | 8.7499 ms |
| [speedy 0.8.7][speedy] | 771.04 µs | 2.4662 ms | 1584734 | 1119837 | 1037992 | 10.574 ms |
| [wincode 0.5.3][wincode] | 586.37 µs | 2.3032 ms | 1791489 | 1127998 | 1051146 | 10.256 ms |
| [wiring 0.2.4][wiring] | 646.89 µs | 2.8024 ms | 1791489 | 1156963 | 1082815 | 10.665 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*80.482 ns\**</span> | <span title="validated on-demand with error">*721.52 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 58.248 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4889 ns\**</span> <span title="validated upfront with error">*5.7179 ms\**</span> | <span title="unvalidated">*2.7714 µs\**</span> <span title="validated upfront with error">*5.7149 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2442 ns\**</span> <span title="validated upfront with error">*349.49 µs\**</span> | <span title="unvalidated">*394.29 ns\**</span> <span title="validated upfront with error">*350.24 µs\**</span> | <span title="unvalidated">*239.40 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2448 ns\**</span> <span title="validated upfront with error">*411.51 µs\**</span> | <span title="unvalidated">*385.51 ns\**</span> <span title="validated upfront with error">*413.04 µs\**</span> | <span title="unvalidated">*235.93 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.05%\**</span> <span title="prepend">*19.72%\**</span> | 15.17% | 47.09% | 62.06% | 64.44% | 2.26% |
| [bin-proto 0.12.7][bin-proto] | 10.10% | 20.33% | 44.80% | 71.21% | 76.36% | 2.56% |
| [bincode 2.0.1][bincode] | 40.77% | 33.99% | 57.08% | 71.86% | 75.55% | 2.83% |
| [bincode 1.3.3][bincode1] | 13.05% | 28.54% | 43.29% | 70.34% | 76.54% | 2.56% |
| [bitcode 0.6.6][bitcode] | 73.02% | 54.68% | 82.64% | 91.48% | 94.40% | 9.62% |
| [borsh 1.5.7][borsh] | 17.76% | 44.99% | 52.74% | 72.46% | 77.29% | 2.73% |
| [capnp 0.23.2][capnp] | 22.60% | † | 29.46% | 51.92% | 64.78% | 1.86% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 15.54% | 7.04% | 13.35% | 47.38% | 54.79% | 1.29% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.18% | 2.48% | 13.35% | 47.38% | 54.79% | 1.27% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.06% | 6.16% | 13.35% | 47.38% | 54.79% | 1.30% |
| [columnar 0.11.1][columnar] | 56.81% | 34.39% <span title="copy_from">*100.00%\**</span> | 51.96% | 80.59% | 89.48% | 5.79% |
| [compactly 0.1.6][compactly] | 0.78% | 2.29% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 38.15% | 34.46% | 60.81% | 75.59% | 79.61% | 3.10% |
| [dlhn 0.1.7][dlhn] | 11.95% | 17.54% | 61.21% | 74.55% | 76.73% | 3.13% |
| [flatbuffers 25.12.19][flatbuffers] | 10.02% | † | 34.51% | 55.81% | 63.30% | 2.03% |
| [flexbuffers 25.2.10][flexbuffers] | 1.21% | 3.76% | 15.00% | 30.22% | 28.89% | 0.79% |
| json:<br> [flexon 0.4.5][flexon] | 3.39% | 5.21% | 8.55% | 33.58% | 43.56% | 0.80% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.35% | 4.10% | 8.55% | 33.58% | 43.56% | 0.77% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.15% | 5.11% | 8.55% | 33.58% | 43.56% | 0.78% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 28.64% | 21.93% | 55.02% | 69.48% | 70.55% | 2.76% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 5.03% | 11.92% | 45.99% | 63.67% | 65.32% | 2.38% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 27.43% | 22.15% | 44.73% | 63.06% | 64.61% | 2.38% |
| [minicbor 1.0.0][minicbor] | 20.62% | 10.98% | 45.16% | 62.94% | 64.08% | 2.19% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.70% | 8.22% | 45.35% | 62.86% | 63.54% | 2.20% |
| [nanoserde 0.2.1][nanoserde] | 38.73% | 46.83% | 44.29% | 70.78% | 76.22% | 2.69% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 38.67% | 52.81% | 56.72% | 1.96% |
| [postcard 1.1.1][postcard] | 28.05% | 30.51% | 61.21% | 74.11% | 77.08% | 3.20% |
| [pot 3.0.1][pot] | 3.56% | 4.17% | 30.81% | 54.19% | 61.80% | 1.65% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.28%\**</span> <span title="populate + encode">*5.34%\**</span> | 14.75% | 43.16% | 60.03% | 61.97% | 2.24% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*8.73%\**</span> <span title="populate + encode">*3.81%\**</span> | 10.54% | 43.16% | 60.03% | 61.97% | 2.22% |
| [rkyv 0.8.10][rkyv] | 51.37% | <span title="unvalidated">*58.88%\**</span> <span title="validated upfront with error">*49.27%\**</span> | 38.67% | 58.05% | 66.32% | 2.14% |
| [ron 0.10.1][ron] | 1.14% | 0.81% | 9.25% | 35.96% | 43.95% | 0.81% |
| [savefile 0.18.6][savefile] | 59.35% | 50.63% | 44.80% | 71.21% | 76.36% | 2.63% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 15.81% | 39.91% | 60.81% | 75.47% | 79.42% | 2.96% |
| [serde-brief 0.1.1][serde-brief] | 9.00% | 5.88% | 11.55% | 44.72% | 51.20% | 1.16% |
| [serde_bare 0.5.0][serde_bare] | 10.84% | 26.09% | 60.81% | 75.59% | 79.60% | 3.16% |
| [speedy 0.8.7][speedy] | 65.15% | 52.52% | 50.65% | 71.73% | 77.33% | 2.62% |
| [wincode 0.5.3][wincode] | 85.66% | 56.23% | 44.80% | 71.21% | 76.36% | 2.70% |
| [wiring 0.2.4][wiring] | 77.65% | 46.22% | 44.80% | 69.43% | 74.13% | 2.60% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.55%\**</span> | <span title="validated on-demand with error">*53.43%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 2.14% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.91%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.77%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*98.55%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
