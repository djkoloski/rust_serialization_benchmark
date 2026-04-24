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

## Last updated: 2026-04-24 06:15:02

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.97.0-nightly (36ba2c771 2026-04-23)
binary: rustc
commit-hash: 36ba2c7712052d731a7082d0eba5ed3d9d56c133
commit-date: 2026-04-23
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
BogoMIPS:                                4890.84
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*463.27 µs\**</span> <span title="prepend">*421.45 µs\**</span> | 2.5464 ms | 868.75 µs | 804955 | 328941 | 284849 | 4.1190 ms |
| [bin-proto 0.12.7][bin-proto] | 4.3115 ms | 4.6458 ms | † | 1045784 | 373127 | 311553 | 4.4838 ms |
| [bincode 2.0.1][bincode] | 336.98 µs | 2.1362 ms | 702.50 µs | 741295 | 303944 | 256422 | 3.4239 ms |
| [bincode 1.3.3][bincode1] | 552.43 µs | 2.0893 ms | 634.28 µs | 1045784 | 373127 | 311553 | 4.4718 ms |
| [bitcode 0.6.6][bitcode] | 138.84 µs | 1.4670 ms | 60.887 µs | 703710 | 288826 | 227322 | 2.5535 ms |
| [borsh 1.5.7][borsh] | 554.36 µs | 2.1627 ms | † | 885780 | 362204 | 286248 | 4.0821 ms |
| [capnp 0.23.2][capnp] | 489.09 µs | † | † | 1443216 | 513986 | 426532 | 6.1265 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 615.78 µs | 5.2106 ms | 3.3812 ms | 1407835 | 403440 | 323561 | 5.0241 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.2815 ms | 11.190 ms | † | 1407835 | 403440 | 323561 | 5.0583 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8804 ms | 5.0705 ms | 3.2425 ms | 1407835 | 403440 | 323561 | 4.7074 ms |
| [columnar 0.11.1][columnar] | 253.08 µs | 2.1896 ms <span title="copy_from">*823.75 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.0919 ms |
| [compactly 0.1.6][compactly] | 27.013 ms | 20.273 ms | † | 241251 | 241453 | 241263 | 106.06 µs |
| [databuf 0.5.0][databuf] | 261.34 µs | 2.0443 ms | 677.51 µs | 765778 | 311715 | 263914 | 3.4782 ms |
| [dlhn 0.1.7][dlhn] | 671.06 µs | 2.5803 ms | † | 724953 | 301446 | 253056 | 3.1694 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0315 ms | † | † | 1276368 | 468539 | 388381 | 4.7773 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.1672 ms | 7.0418 ms | 5.7929 ms | 1829756 | 714318 | 691541 | 8.5374 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.7541 ms | 3.7759 ms | † | 1827461 | 470560 | 360727 | 5.4796 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.8240 ms | 5.9020 ms | † | 1827461 | 470560 | 360727 | 5.4606 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1577 ms | 4.7827 ms | † | 1827461 | 470560 | 360727 | 5.4415 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 359.25 µs | 2.5971 ms | 915.95 µs | 764996 | 315291 | 264212 | 3.5858 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4960 ms | 3.1607 ms | 1.4255 ms | 784997 | 325384 | 277608 | 3.7620 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 367.94 µs | 2.5236 ms | 811.98 µs | 784997 | 325384 | 277608 | 3.8162 ms |
| [minicbor 1.0.0][minicbor] | 540.87 µs | 3.0434 ms | 1.4527 ms | 817830 | 332671 | 284034 | 4.0156 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5551 ms | 4.0181 ms | 2.4616 ms | 818669 | 332556 | 284797 | 4.0411 ms |
| [nanoserde 0.2.1][nanoserde] | 235.43 µs | 2.1075 ms | † | 1045784 | 373127 | 311553 | 4.1062 ms |
| [nibblecode 0.1.0][nibblecode] | 185.21 µs | † | † | 1011487 | 482464 | 412798 | 5.3821 ms |
| [postcard 1.1.1][postcard] | 428.61 µs | 2.2860 ms | 722.58 µs | 724953 | 302399 | 252968 | 3.2395 ms |
| [pot 3.0.1][pot] | 2.3099 ms | 6.5076 ms | 4.7785 ms | 971922 | 372513 | 303636 | 4.3426 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*965.84 µs\**</span> <span title="populate + encode">*2.4952 ms\**</span> | 3.4402 ms | † | 884628 | 363130 | 314959 | 4.3744 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2006 ms\**</span> <span title="populate + encode">*3.0299 ms\**</span> | 3.8241 ms | † | 884628 | 363130 | 314959 | 4.3629 ms |
| [rkyv 0.8.10][rkyv] | 247.23 µs | <span title="unvalidated">*1.5516 ms\**</span> <span title="validated upfront with error">*1.9196 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6029 ms |
| [ron 0.10.1][ron] | 12.653 ms | 23.325 ms | 21.018 ms | 1607459 | 449158 | 349324 | 5.5392 ms |
| [savefile 0.18.6][savefile] | 196.45 µs | 2.1852 ms | † | 1045800 | 373139 | 311562 | 4.2110 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 663.24 µs | 2.4206 ms | † | 765778 | 311743 | 263822 | 3.4457 ms |
| [serde-brief 0.1.1][serde-brief] | 1.4299 ms | 4.8016 ms | 3.0037 ms | 1584946 | 413733 | 339964 | 4.8675 ms |
| [serde_bare 0.5.0][serde_bare] | 685.89 µs | 2.1062 ms | † | 765778 | 311715 | 263914 | 3.5596 ms |
| [speedy 0.8.7][speedy] | 200.26 µs | 1.7527 ms | 373.00 µs | 885780 | 362204 | 286248 | 3.8130 ms |
| [wincode 0.5.3][wincode] | 174.76 µs | 1.7708 ms | 384.10 µs | 1045784 | 373127 | 311553 | 4.1447 ms |
| [wiring 0.2.4][wiring] | 193.02 µs | 2.0742 ms | † | 1045784 | 337930 | 275808 | 3.6329 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*78.816 ns\**</span> | <span title="validated on-demand with error">*135.62 µs\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 23.520 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4898 ns\**</span> <span title="validated upfront with error">*2.0752 ms\**</span> | <span title="unvalidated">*51.872 µs\**</span> <span title="validated upfront with error">*2.1850 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2445 ns\**</span> <span title="validated upfront with error">*237.00 µs\**</span> | <span title="unvalidated">*10.391 µs\**</span> <span title="validated upfront with error">*247.60 µs\**</span> | <span title="unvalidated">*7.6650 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2462 ns\**</span> <span title="validated upfront with error">*354.08 µs\**</span> | <span title="unvalidated">*10.432 µs\**</span> <span title="validated upfront with error">*366.21 µs\**</span> | <span title="unvalidated">*7.6281 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*29.97%\**</span> <span title="prepend">*32.94%\**</span> | 32.35% | 7.01% | 29.97% | 73.40% | 79.80% | 2.57% |
| [bin-proto 0.12.7][bin-proto] | 3.22% | 17.73% | † | 23.07% | 64.71% | 72.96% | 2.37% |
| [bincode 2.0.1][bincode] | 41.20% | 38.56% | 8.67% | 32.54% | 79.44% | 88.65% | 3.10% |
| [bincode 1.3.3][bincode1] | 25.13% | 39.43% | 9.60% | 23.07% | 64.71% | 72.96% | 2.37% |
| [bitcode 0.6.6][bitcode] | 100.00% | 56.15% | 100.00% | 34.28% | 83.60% | 100.00% | 4.15% |
| [borsh 1.5.7][borsh] | 25.05% | 38.09% | † | 27.24% | 66.66% | 79.41% | 2.60% |
| [capnp 0.23.2][capnp] | 28.39% | † | † | 16.72% | 46.98% | 53.30% | 1.73% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 22.55% | 15.81% | 1.80% | 17.14% | 59.85% | 70.26% | 2.11% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.23% | 7.36% | † | 17.14% | 59.85% | 70.26% | 2.10% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.38% | 16.25% | 1.88% | 17.14% | 59.85% | 70.26% | 2.25% |
| [columnar 0.11.1][columnar] | 54.86% | 37.62% <span title="copy_from">*100.00%\**</span> | † | 23.07% | 65.22% | 77.34% | 2.59% |
| [compactly 0.1.6][compactly] | 0.51% | 4.06% | † | 100.00% | 100.00% | 94.22% | 100.00% |
| [databuf 0.5.0][databuf] | 53.13% | 40.29% | 8.99% | 31.50% | 77.46% | 86.13% | 3.05% |
| [dlhn 0.1.7][dlhn] | 20.69% | 31.92% | † | 33.28% | 80.10% | 89.83% | 3.35% |
| [flatbuffers 25.12.19][flatbuffers] | 13.46% | † | † | 18.90% | 51.53% | 58.53% | 2.22% |
| [flexbuffers 25.2.10][flexbuffers] | 1.94% | 11.70% | 1.05% | 13.18% | 33.80% | 32.87% | 1.24% |
| json:<br> [flexon 0.4.5][flexon] | 5.04% | 21.82% | † | 13.20% | 51.31% | 63.02% | 1.94% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.63% | 13.96% | † | 13.20% | 51.31% | 63.02% | 1.94% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.43% | 17.22% | † | 13.20% | 51.31% | 63.02% | 1.95% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 38.65% | 31.72% | 6.65% | 31.54% | 76.58% | 86.04% | 2.96% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.28% | 26.06% | 4.27% | 30.73% | 74.21% | 81.89% | 2.82% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 37.73% | 32.64% | 7.50% | 30.73% | 74.21% | 81.89% | 2.78% |
| [minicbor 1.0.0][minicbor] | 25.67% | 27.07% | 4.19% | 29.50% | 72.58% | 80.03% | 2.64% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.50% | 20.50% | 2.47% | 29.47% | 72.61% | 79.82% | 2.62% |
| [nanoserde 0.2.1][nanoserde] | 58.97% | 39.09% | † | 23.07% | 64.71% | 72.96% | 2.58% |
| [nibblecode 0.1.0][nibblecode] | 74.96% | † | † | 23.85% | 50.05% | 55.07% | 1.97% |
| [postcard 1.1.1][postcard] | 32.39% | 36.03% | 8.43% | 33.28% | 79.85% | 89.86% | 3.27% |
| [pot 3.0.1][pot] | 6.01% | 12.66% | 1.27% | 24.82% | 64.82% | 74.87% | 2.44% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*14.38%\**</span> <span title="populate + encode">*5.56%\**</span> | 23.94% | † | 27.27% | 66.49% | 72.18% | 2.42% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.56%\**</span> <span title="populate + encode">*4.58%\**</span> | 21.54% | † | 27.27% | 66.49% | 72.18% | 2.43% |
| [rkyv 0.8.10][rkyv] | 56.16% | <span title="unvalidated">*53.09%\**</span> <span title="validated upfront with error">*42.91%\**</span> | † | 23.85% | 61.36% | 69.74% | 2.30% |
| [ron 0.10.1][ron] | 1.10% | 3.53% | 0.29% | 15.01% | 53.76% | 65.07% | 1.91% |
| [savefile 0.18.6][savefile] | 70.67% | 37.70% | † | 23.07% | 64.71% | 72.96% | 2.52% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.93% | 34.03% | † | 31.50% | 77.45% | 86.16% | 3.08% |
| [serde-brief 0.1.1][serde-brief] | 9.71% | 17.16% | 2.03% | 15.22% | 58.36% | 66.87% | 2.18% |
| [serde_bare 0.5.0][serde_bare] | 20.24% | 39.11% | † | 31.50% | 77.46% | 86.13% | 2.98% |
| [speedy 0.8.7][speedy] | 69.33% | 47.00% | 16.32% | 27.24% | 66.66% | 79.41% | 2.78% |
| [wincode 0.5.3][wincode] | 79.45% | 46.52% | 15.85% | 23.07% | 64.71% | 72.96% | 2.56% |
| [wiring 0.2.4][wiring] | 71.93% | 39.71% | † | 23.07% | 71.45% | 82.42% | 2.92% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.58%\**</span> | <span title="validated on-demand with error">*7.66%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.29% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.03%\**</span> <span title="validated upfront with error">*0.48%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*4.20%\**</span> | <span title="unvalidated">*99.52%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.86%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.61%\**</span> <span title="validated upfront with error">*2.84%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.2020 ms\**</span> <span title="prepend">*8.7458 ms\**</span> | 7.8226 ms | 8625005 | 6443961 | 6231572 | 72.058 ms |
| [bin-proto 0.12.7][bin-proto] | 8.3393 ms | 10.689 ms | 6000008 | 5378500 | 5346908 | 8.4314 ms |
| [bincode 2.0.1][bincode] | 2.4162 ms | 1.0456 ms | 6000005 | 5378497 | 5346882 | 8.6628 ms |
| [bincode 1.3.3][bincode1] | 5.8836 ms | 5.8333 ms | 6000008 | 5378500 | 5346908 | 8.7681 ms |
| [bitcode 0.6.6][bitcode] | 1.2996 ms | 806.01 µs | 6000006 | 5182295 | 4921841 | 13.249 ms |
| [borsh 1.5.7][borsh] | 6.2591 ms | 4.1592 ms | 6000004 | 5378496 | 5346866 | 8.6217 ms |
| [capnp 0.23.2][capnp] | 5.7059 ms | † | 14000088 | 7130367 | 6046182 | 80.451 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 9.1304 ms | 50.453 ms | 13125016 | 7524114 | 6757437 | 89.448 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 68.830 ms | 126.55 ms | 13122324 | 7524660 | 6759128 | 89.850 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 35.006 ms | 41.163 ms | 13122324 | 7524660 | 6759128 | 90.454 ms |
| [columnar 0.11.1][columnar] | 1.7579 ms | 1.4554 ms <span title="copy_from">*682.70 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.4854 ms |
| [compactly 0.1.6][compactly] | 355.96 ms | 282.18 ms | 4846786 | 4850065 | 4846903 | 1.6494 ms |
| [databuf 0.5.0][databuf] | 2.4208 ms | 5.3140 ms | 6000003 | 5378495 | 5346897 | 8.7054 ms |
| [dlhn 0.1.7][dlhn] | 6.1790 ms | 7.0399 ms | 6000003 | 5378495 | 5346897 | 8.7000 ms |
| [flatbuffers 25.12.19][flatbuffers] | 447.14 µs | † | 6000024 | 5378434 | 5346878 | 8.8618 ms |
| [flexbuffers 25.2.10][flexbuffers] | 107.66 ms | 87.528 ms | 26609424 | 11901040 | 12486322 | 152.07 ms |
| json:<br> [flexon 0.4.5][flexon] | 76.064 ms | 56.165 ms | 26192883 | 9566084 | 8584671 | 155.76 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 89.311 ms | 100.74 ms | 26192883 | 9566084 | 8584671 | 156.63 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 53.433 ms | 67.458 ms | 26192883 | 9566084 | 8584671 | 157.56 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 658.24 µs | 5.1937 ms | 7500005 | 6058442 | 6014500 | 10.454 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 18.796 ms | 16.611 ms | 8125006 | 6494876 | 6391037 | 74.688 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 730.09 µs | 5.4958 ms | 8125006 | 6494876 | 6391037 | 76.263 ms |
| [minicbor 1.0.0][minicbor] | 6.0937 ms | 11.604 ms | 8125006 | 6494907 | 6390894 | 70.772 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 119.11 ms | 24.712 ms | 8125037 | 6493484 | 6386940 | 69.905 ms |
| [nanoserde 0.2.1][nanoserde] | 1.6047 ms | 894.73 µs | 6000008 | 5378500 | 5346908 | 8.9363 ms |
| [nibblecode 0.1.0][nibblecode] | 148.53 µs | † | 6000008 | 5378500 | 5346908 | 8.4435 ms |
| [postcard 1.1.1][postcard] | 480.23 µs | 1.0021 ms | 6000003 | 5378495 | 5346897 | 8.6135 ms |
| [pot 3.0.1][pot] | 38.693 ms | 69.991 ms | 10122342 | 6814618 | 6852252 | 80.836 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*7.8160 ms\**</span> <span title="populate + encode">*8.3242 ms\**</span> | 13.192 ms | 8750000 | 6665735 | 6421877 | 71.685 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.397 ms\**</span> <span title="populate + encode">*30.902 ms\**</span> | 29.980 ms | 8750000 | 6665735 | 6421877 | 72.313 ms |
| [rkyv 0.8.10][rkyv] | 150.61 µs | <span title="unvalidated">*150.45 µs\**</span> <span title="validated upfront with error">*151.76 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.6878 ms |
| [ron 0.10.1][ron] | 163.42 ms | 517.22 ms | 22192885 | 8970395 | 8137334 | 151.53 ms |
| [savefile 0.18.6][savefile] | 149.17 µs | 150.54 µs | 6000024 | 5378519 | 5346896 | 8.7785 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1614 ms | 3.9662 ms | 6000004 | 5378496 | 5346866 | 8.6146 ms |
| [serde-brief 0.1.1][serde-brief] | 17.539 ms | 38.859 ms | 15750015 | 8024540 | 6813667 | 92.710 ms |
| [serde_bare 0.5.0][serde_bare] | 6.0223 ms | 4.8479 ms | 6000003 | 5378495 | 5346897 | 8.9697 ms |
| [speedy 0.8.7][speedy] | 183.09 µs | 150.54 µs | 6000004 | 5378496 | 5346866 | 8.4238 ms |
| [wincode 0.5.3][wincode] | 149.40 µs | 149.19 µs | 6000008 | 5378500 | 5346908 | 8.6476 ms |
| [wiring 0.2.4][wiring] | 199.41 µs | 337.12 µs | 6000008 | 5378952 | 5346905 | 8.6090 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*111.06 ns\**</span> | <span title="validated on-demand with error">*1.9654 ms\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 21.789 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4907 ns\**</span> <span title="validated upfront with error">*45.146 ns\**</span> | <span title="unvalidated">*77.841 µs\**</span> <span title="validated upfront with error">*77.999 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2453 ns\**</span> <span title="validated upfront with error">*1.8688 ns\**</span> | <span title="unvalidated">*38.906 µs\**</span> <span title="validated upfront with error">*38.921 µs\**</span> | <span title="unvalidated">*79.383 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2463 ns\**</span> <span title="validated upfront with error">*5.2983 ns\**</span> | <span title="unvalidated">*38.901 µs\**</span> <span title="validated upfront with error">*38.911 µs\**</span> | <span title="unvalidated">*75.717 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.06%\**</span> <span title="prepend">*1.70%\**</span> | 1.91% | 56.19% | 75.27% | 77.78% | 2.29% |
| [bin-proto 0.12.7][bin-proto] | 1.78% | 1.40% | 80.78% | 90.18% | 90.65% | 19.56% |
| [bincode 2.0.1][bincode] | 6.15% | 14.27% | 80.78% | 90.18% | 90.65% | 19.04% |
| [bincode 1.3.3][bincode1] | 2.52% | 2.56% | 80.78% | 90.18% | 90.65% | 18.81% |
| [bitcode 0.6.6][bitcode] | 11.43% | 18.51% | 80.78% | 93.59% | 98.48% | 12.45% |
| [borsh 1.5.7][borsh] | 2.37% | 3.59% | 80.78% | 90.18% | 90.65% | 19.13% |
| [capnp 0.23.2][capnp] | 2.60% | † | 34.62% | 68.02% | 80.16% | 2.05% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.63% | 0.30% | 36.93% | 64.46% | 71.73% | 1.84% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.22% | 0.12% | 36.94% | 64.46% | 71.71% | 1.84% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.42% | 0.36% | 36.94% | 64.46% | 71.71% | 1.82% |
| [columnar 0.11.1][columnar] | 8.45% | 10.25% <span title="copy_from">*21.85%\**</span> | 80.78% | 90.18% | 90.65% | 19.44% |
| [compactly 0.1.6][compactly] | 0.04% | 0.05% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 6.14% | 2.81% | 80.78% | 90.18% | 90.65% | 18.95% |
| [dlhn 0.1.7][dlhn] | 2.40% | 2.12% | 80.78% | 90.18% | 90.65% | 18.96% |
| [flatbuffers 25.12.19][flatbuffers] | 33.22% | † | 80.78% | 90.18% | 90.65% | 18.61% |
| [flexbuffers 25.2.10][flexbuffers] | 0.14% | 0.17% | 18.21% | 40.75% | 38.82% | 1.08% |
| json:<br> [flexon 0.4.5][flexon] | 0.20% | 0.27% | 18.50% | 50.70% | 56.46% | 1.06% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.15% | 18.50% | 50.70% | 56.46% | 1.05% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.28% | 0.22% | 18.50% | 50.70% | 56.46% | 1.05% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 22.56% | 2.87% | 64.62% | 80.05% | 80.59% | 15.78% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.79% | 0.90% | 59.65% | 74.68% | 75.84% | 2.21% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 20.34% | 2.71% | 59.65% | 74.68% | 75.84% | 2.16% |
| [minicbor 1.0.0][minicbor] | 2.44% | 1.29% | 59.65% | 74.67% | 75.84% | 2.33% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.60% | 59.65% | 74.69% | 75.89% | 2.36% |
| [nanoserde 0.2.1][nanoserde] | 9.26% | 16.67% | 80.78% | 90.18% | 90.65% | 18.46% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 80.78% | 90.18% | 90.65% | 19.53% |
| [postcard 1.1.1][postcard] | 30.93% | 14.89% | 80.78% | 90.18% | 90.65% | 19.15% |
| [pot 3.0.1][pot] | 0.38% | 0.21% | 47.88% | 71.17% | 70.73% | 2.04% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.90%\**</span> <span title="populate + encode">*1.78%\**</span> | 1.13% | 55.39% | 72.76% | 75.47% | 2.30% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.03%\**</span> <span title="populate + encode">*0.48%\**</span> | 0.50% | 55.39% | 72.76% | 75.47% | 2.28% |
| [rkyv 0.8.10][rkyv] | 98.62% | <span title="unvalidated">*99.16%\**</span> <span title="validated upfront with error">*98.31%\**</span> | 80.78% | 90.18% | 90.65% | 18.99% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 21.84% | 54.07% | 59.56% | 1.09% |
| [savefile 0.18.6][savefile] | 99.57% | 99.10% | 80.78% | 90.17% | 90.65% | 18.79% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.88% | 3.76% | 80.78% | 90.18% | 90.65% | 19.15% |
| [serde-brief 0.1.1][serde-brief] | 0.85% | 0.38% | 30.77% | 60.44% | 71.14% | 1.78% |
| [serde_bare 0.5.0][serde_bare] | 2.47% | 3.08% | 80.78% | 90.18% | 90.65% | 18.39% |
| [speedy 0.8.7][speedy] | 81.12% | 99.10% | 80.78% | 90.18% | 90.65% | 19.58% |
| [wincode 0.5.3][wincode] | 99.42% | 100.00% | 80.78% | 90.18% | 90.65% | 19.07% |
| [wiring 0.2.4][wiring] | 74.48% | 44.25% | 80.78% | 90.17% | 90.65% | 19.16% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.12%\**</span> | <span title="validated on-demand with error">*1.98%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.72% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*2.76%\**</span> | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*49.87%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*66.64%\**</span> | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*99.95%\**</span> | <span title="unvalidated">*95.38%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.92%\**</span> <span title="validated upfront with error">*23.50%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.97%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*871.40 µs\**</span> <span title="prepend">*788.06 µs\**</span> | 3.1260 ms | 1.6942 ms | 489348 | 281173 | 249360 | 2.5972 ms |
| [bin-proto 0.12.7][bin-proto] | 1.8392 ms | 2.7888 ms | † | 566975 | 239350 | 231475 | 2.4831 ms |
| [bincode 2.0.1][bincode] | 310.03 µs | 1.8123 ms | 790.67 µs | 367413 | 221291 | 206242 | 2.0557 ms |
| [bincode 1.3.3][bincode1] | 586.57 µs | 1.8542 ms | 878.48 µs | 569975 | 240525 | 231884 | 2.6039 ms |
| [bitcode 0.6.6][bitcode] | 125.47 µs | 1.2641 ms | 172.07 µs | 327688 | 200947 | 182040 | 764.22 µs |
| [borsh 1.5.7][borsh] | 554.91 µs | 1.8126 ms | † | 446595 | 234236 | 209834 | 2.0730 ms |
| [capnp 0.23.2][capnp] | 462.10 µs | † | † | 803896 | 335606 | 280744 | 3.5540 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 746.63 µs | 4.8106 ms | 3.4464 ms | 1109831 | 344745 | 274333 | 3.4497 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.8780 ms | 10.434 ms | † | 1109821 | 344751 | 274345 | 3.4407 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8484 ms | 4.6962 ms | 3.4226 ms | 1109821 | 344751 | 274345 | 3.4592 ms |
| [columnar 0.11.1][columnar] | 283.96 µs | 1.9011 ms <span title="copy_from">*757.24 µs\**</span> | † | 563728 | 249696 | 217582 | 1.5903 ms |
| [compactly 0.1.6][compactly] | 11.695 ms | 11.326 ms | † | 149292 | 149433 | 149304 | 89.286 µs |
| [databuf 0.5.0][databuf] | 290.10 µs | 1.7442 ms | 813.00 µs | 356311 | 213062 | 198403 | 2.0706 ms |
| [dlhn 0.1.7][dlhn] | 700.89 µs | 2.6920 ms | † | 366496 | 220600 | 205586 | 2.0428 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2534 ms | † | † | 849472 | 347816 | 294871 | 3.5275 ms |
| [flexbuffers 25.2.10][flexbuffers] | 8.0904 ms | 6.7970 ms | 5.5496 ms | 1187688 | 557642 | 553730 | 6.3734 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.7258 ms | 4.6322 ms | † | 1623191 | 466527 | 359157 | 5.8108 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.8230 ms | 7.1742 ms | † | 1623191 | 466527 | 359157 | 6.0083 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2216 ms | 4.5452 ms | † | 1623191 | 466527 | 359157 | 5.8067 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 330.97 µs | 2.9087 ms | 1.3075 ms | 391251 | 236877 | 220395 | 2.2390 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4644 ms | 3.0465 ms | 1.6893 ms | 424533 | 245214 | 226077 | 2.2841 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 387.49 µs | 2.2223 ms | 934.85 µs | 416025 | 243812 | 224965 | 2.2336 ms |
| [minicbor 1.0.0][minicbor] | 538.38 µs | 3.3885 ms | 1.9224 ms | 428773 | 249857 | 228630 | 2.3341 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 4.9905 ms | 3.7821 ms | 2.6819 ms | 449745 | 252432 | 230965 | 2.4316 ms |
| [nanoserde 0.2.1][nanoserde] | 265.12 µs | 1.9016 ms | † | 567975 | 239930 | 231872 | 2.4954 ms |
| [nibblecode 0.1.0][nibblecode] | 180.97 µs | † | † | 603928 | 430458 | 406401 | 3.6625 ms |
| [postcard 1.1.1][postcard] | 452.70 µs | 2.0750 ms | 808.11 µs | 367489 | 221913 | 207244 | 2.0347 ms |
| [pot 3.0.1][pot] | 2.4078 ms | 6.1816 ms | 5.0284 ms | 599125 | 299158 | 247675 | 2.7391 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.3000 ms\**</span> <span title="populate + encode">*3.0423 ms\**</span> | 3.5767 ms | † | 596811 | 305319 | 268737 | 2.9979 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0446 ms\**</span> <span title="populate + encode">*3.0167 ms\**</span> | 3.8998 ms | † | 596811 | 305319 | 268737 | 3.0016 ms |
| [rkyv 0.8.10][rkyv] | 331.98 µs | <span title="unvalidated">*1.5065 ms\**</span> <span title="validated upfront with error">*1.8713 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3575 ms |
| [ron 0.10.1][ron] | 8.3636 ms | 24.075 ms | 22.954 ms | 1465223 | 434935 | 342907 | 5.6865 ms |
| [savefile 0.18.6][savefile] | 215.65 µs | 1.8087 ms | † | 566991 | 239362 | 231478 | 2.4783 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 628.17 µs | 2.1147 ms | † | 356311 | 212976 | 198423 | 1.9434 ms |
| [serde-brief 0.1.1][serde-brief] | 1.1753 ms | 5.2765 ms | 3.6409 ms | 1276014 | 373898 | 293384 | 3.7126 ms |
| [serde_bare 0.5.0][serde_bare] | 744.85 µs | 2.3721 ms | † | 356311 | 213062 | 198403 | 1.9970 ms |
| [speedy 0.8.7][speedy] | 268.76 µs | 1.6917 ms | 550.91 µs | 449595 | 234970 | 210192 | 2.0667 ms |
| [wincode 0.5.3][wincode] | 204.48 µs | 1.6446 ms | 564.28 µs | 566975 | 239350 | 231475 | 2.4986 ms |
| [wiring 0.2.4][wiring] | 207.20 µs | 1.9688 ms | † | 566975 | 247810 | 225086 | 2.5622 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*85.434 ns\**</span> | <span title="validated on-demand with error">*424.45 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 885.21 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.5064 ns\**</span> <span title="validated upfront with error">*2.1701 ms\**</span> | <span title="unvalidated">*1.4201 µs\**</span> <span title="validated upfront with error">*2.1721 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2453 ns\**</span> <span title="validated upfront with error">*256.05 µs\**</span> | <span title="unvalidated">*156.26 ns\**</span> <span title="validated upfront with error">*258.70 µs\**</span> | <span title="unvalidated">*726.15 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2446 ns\**</span> <span title="validated upfront with error">*336.57 µs\**</span> | <span title="unvalidated">*156.21 ns\**</span> <span title="validated upfront with error">*335.79 µs\**</span> | <span title="unvalidated">*730.21 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.40%\**</span> <span title="prepend">*15.92%\**</span> | 24.22% | 10.16% | 30.51% | 53.15% | 59.87% | 3.44% |
| [bin-proto 0.12.7][bin-proto] | 6.82% | 27.15% | † | 26.33% | 62.43% | 64.50% | 3.60% |
| [bincode 2.0.1][bincode] | 40.47% | 41.78% | 21.76% | 40.63% | 67.53% | 72.39% | 4.34% |
| [bincode 1.3.3][bincode1] | 21.39% | 40.84% | 19.59% | 26.19% | 62.13% | 64.39% | 3.43% |
| [bitcode 0.6.6][bitcode] | 100.00% | 59.90% | 100.00% | 45.56% | 74.36% | 82.02% | 11.68% |
| [borsh 1.5.7][borsh] | 22.61% | 41.78% | † | 33.43% | 63.80% | 71.15% | 4.31% |
| [capnp 0.23.2][capnp] | 27.15% | † | † | 18.57% | 44.53% | 53.18% | 2.51% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.80% | 15.74% | 4.99% | 13.45% | 43.35% | 54.42% | 2.59% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.24% | 7.26% | † | 13.45% | 43.35% | 54.42% | 2.59% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.79% | 16.12% | 5.03% | 13.45% | 43.35% | 54.42% | 2.58% |
| [columnar 0.11.1][columnar] | 44.19% | 39.83% <span title="copy_from">*100.00%\**</span> | † | 26.48% | 59.85% | 68.62% | 5.61% |
| [compactly 0.1.6][compactly] | 1.07% | 6.69% | † | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 43.25% | 43.41% | 21.16% | 41.90% | 70.14% | 75.25% | 4.31% |
| [dlhn 0.1.7][dlhn] | 17.90% | 28.13% | † | 40.73% | 67.74% | 72.62% | 4.37% |
| [flatbuffers 25.12.19][flatbuffers] | 3.86% | † | † | 17.57% | 42.96% | 50.63% | 2.53% |
| [flexbuffers 25.2.10][flexbuffers] | 1.55% | 11.14% | 3.10% | 12.57% | 26.80% | 26.96% | 1.40% |
| json:<br> [flexon 0.4.5][flexon] | 4.60% | 16.35% | † | 9.20% | 32.03% | 41.57% | 1.54% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.28% | 10.56% | † | 9.20% | 32.03% | 41.57% | 1.49% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.65% | 16.66% | † | 9.20% | 32.03% | 41.57% | 1.54% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 37.91% | 26.03% | 13.16% | 38.16% | 63.08% | 67.74% | 3.99% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.57% | 24.86% | 10.19% | 35.17% | 60.94% | 66.04% | 3.91% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 32.38% | 34.07% | 18.41% | 35.89% | 61.29% | 66.37% | 4.00% |
| [minicbor 1.0.0][minicbor] | 23.31% | 22.35% | 8.95% | 34.82% | 59.81% | 65.30% | 3.83% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.51% | 20.02% | 6.42% | 33.19% | 59.20% | 64.64% | 3.67% |
| [nanoserde 0.2.1][nanoserde] | 47.33% | 39.82% | † | 26.28% | 62.28% | 64.39% | 3.58% |
| [nibblecode 0.1.0][nibblecode] | 69.33% | † | † | 24.72% | 34.71% | 36.74% | 2.44% |
| [postcard 1.1.1][postcard] | 27.72% | 36.49% | 21.29% | 40.62% | 67.34% | 72.04% | 4.39% |
| [pot 3.0.1][pot] | 5.21% | 12.25% | 3.42% | 24.92% | 49.95% | 60.28% | 3.26% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.65%\**</span> <span title="populate + encode">*4.12%\**</span> | 21.17% | † | 25.01% | 48.94% | 55.56% | 2.98% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.01%\**</span> <span title="populate + encode">*4.16%\**</span> | 19.42% | † | 25.01% | 48.94% | 55.56% | 2.97% |
| [rkyv 0.8.10][rkyv] | 37.79% | <span title="unvalidated">*50.26%\**</span> <span title="validated upfront with error">*40.47%\**</span> | † | 24.73% | 58.65% | 68.04% | 3.79% |
| [ron 0.10.1][ron] | 1.50% | 3.15% | 0.75% | 10.19% | 34.36% | 43.54% | 1.57% |
| [savefile 0.18.6][savefile] | 58.18% | 41.87% | † | 26.33% | 62.43% | 64.50% | 3.60% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 19.97% | 35.81% | † | 41.90% | 70.16% | 75.25% | 4.59% |
| [serde-brief 0.1.1][serde-brief] | 10.68% | 14.35% | 4.73% | 11.70% | 39.97% | 50.89% | 2.40% |
| [serde_bare 0.5.0][serde_bare] | 16.85% | 31.92% | † | 41.90% | 70.14% | 75.25% | 4.47% |
| [speedy 0.8.7][speedy] | 46.68% | 44.76% | 31.23% | 33.21% | 63.60% | 71.03% | 4.32% |
| [wincode 0.5.3][wincode] | 61.36% | 46.04% | 30.49% | 26.33% | 62.43% | 64.50% | 3.57% |
| [wiring 0.2.4][wiring] | 60.56% | 38.46% | † | 26.33% | 60.30% | 66.33% | 3.48% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.46%\**</span> | <span title="validated on-demand with error">*36.80%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 0.14% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.66%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.00%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.94%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*99.44%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5741 ms\**</span> <span title="prepend">*2.5549 ms\**</span> | 8.2770 ms | 1704643 | 1294259 | 1245668 | 11.796 ms |
| [bin-proto 0.12.7][bin-proto] | 5.1211 ms | 6.4536 ms | 1791489 | 1127998 | 1051146 | 10.417 ms |
| [bincode 2.0.1][bincode] | 1.2346 ms | 3.6320 ms | 1406257 | 1117802 | 1062438 | 9.7152 ms |
| [bincode 1.3.3][bincode1] | 3.7322 ms | 4.5054 ms | 1854234 | 1141994 | 1048745 | 10.592 ms |
| [bitcode 0.6.6][bitcode] | 694.57 µs | 2.3644 ms | 971318 | 878034 | 850340 | 3.1217 ms |
| [borsh 1.5.7][borsh] | 2.9326 ms | 2.9198 ms | 1521989 | 1108471 | 1038528 | 10.283 ms |
| [capnp 0.23.2][capnp] | 2.2717 ms | † | 2724288 | 1546992 | 1239111 | 14.842 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.2788 ms | 18.829 ms | 6012539 | 1695215 | 1464951 | 21.828 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.745 ms | 52.408 ms | 6012373 | 1695146 | 1465025 | 22.169 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.9053 ms | 21.738 ms | 6012373 | 1695146 | 1465025 | 21.552 ms |
| [columnar 0.11.1][columnar] | 910.33 µs | 3.7581 ms <span title="copy_from">*1.2611 ms\**</span> | 1544752 | 996728 | 897073 | 4.8672 ms |
| [compactly 0.1.6][compactly] | 64.210 ms | 56.420 ms | 802662 | 803238 | 802689 | 355.42 µs |
| [databuf 0.5.0][databuf] | 1.3323 ms | 3.7732 ms | 1319999 | 1062631 | 1008334 | 9.2551 ms |
| [dlhn 0.1.7][dlhn] | 4.4771 ms | 7.3137 ms | 1311281 | 1077520 | 1046095 | 9.4147 ms |
| [flatbuffers 25.12.19][flatbuffers] | 4.9509 ms | † | 2325620 | 1439185 | 1268060 | 13.674 ms |
| [flexbuffers 25.2.10][flexbuffers] | 41.490 ms | 35.851 ms | 5352680 | 2658295 | 2777967 | 34.913 ms |
| json:<br> [flexon 0.4.5][flexon] | 15.315 ms | 24.422 ms | 9390461 | 2391679 | 1842767 | 35.157 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 22.268 ms | 31.760 ms | 9390461 | 2391679 | 1842767 | 35.122 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.897 ms | 25.714 ms | 9390461 | 2391679 | 1842767 | 35.583 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 947.27 µs | 5.7379 ms | 1458773 | 1156055 | 1137788 | 9.7691 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.425 ms | 10.894 ms | 1745322 | 1261627 | 1228923 | 12.350 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 1.8366 ms | 5.5016 ms | 1794467 | 1273669 | 1242301 | 12.222 ms |
| [minicbor 1.0.0][minicbor] | 2.4107 ms | 11.569 ms | 1777386 | 1276218 | 1252558 | 13.069 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.976 ms | 16.522 ms | 1770060 | 1277755 | 1263362 | 12.661 ms |
| [nanoserde 0.2.1][nanoserde] | 1.3066 ms | 2.7828 ms | 1812404 | 1134820 | 1053109 | 10.578 ms |
| [nibblecode 0.1.0][nibblecode] | 507.76 µs | † | 2075936 | 1548252 | 1439671 | 14.256 ms |
| [postcard 1.1.1][postcard] | 1.8670 ms | 4.1950 ms | 1311281 | 1083900 | 1041434 | 8.7948 ms |
| [pot 3.0.1][pot] | 14.075 ms | 30.142 ms | 2604812 | 1482233 | 1298928 | 16.644 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.5057 ms\**</span> <span title="populate + encode">*9.4135 ms\**</span> | 8.7297 ms | 1859886 | 1338076 | 1295351 | 12.862 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.8977 ms\**</span> <span title="populate + encode">*13.199 ms\**</span> | 12.284 ms | 1859886 | 1338076 | 1295351 | 12.977 ms |
| [rkyv 0.8.10][rkyv] | 986.03 µs | <span title="unvalidated">*2.2204 ms\**</span> <span title="validated upfront with error">*2.6625 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.746 ms |
| [ron 0.10.1][ron] | 44.081 ms | 150.12 ms | 8677703 | 2233642 | 1826180 | 35.838 ms |
| [savefile 0.18.6][savefile] | 873.62 µs | 2.5506 ms | 1791505 | 1128012 | 1051153 | 10.681 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1568 ms | 3.3145 ms | 1319999 | 1064380 | 1010708 | 8.9453 ms |
| [serde-brief 0.1.1][serde-brief] | 5.6084 ms | 22.199 ms | 6951772 | 1796265 | 1567819 | 23.945 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8359 ms | 5.0083 ms | 1319999 | 1062645 | 1008349 | 9.0946 ms |
| [speedy 0.8.7][speedy] | 747.00 µs | 2.4601 ms | 1584734 | 1119837 | 1037992 | 10.113 ms |
| [wincode 0.5.3][wincode] | 596.56 µs | 2.3296 ms | 1791489 | 1127998 | 1051146 | 10.318 ms |
| [wiring 0.2.4][wiring] | 649.92 µs | 2.7587 ms | 1791489 | 1156963 | 1082815 | 11.088 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*85.420 ns\**</span> | <span title="validated on-demand with error">*725.20 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 58.045 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4926 ns\**</span> <span title="validated upfront with error">*5.7462 ms\**</span> | <span title="unvalidated">*2.7558 µs\**</span> <span title="validated upfront with error">*5.7536 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2458 ns\**</span> <span title="validated upfront with error">*342.92 µs\**</span> | <span title="unvalidated">*378.73 ns\**</span> <span title="validated upfront with error">*342.28 µs\**</span> | <span title="unvalidated">*237.58 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2451 ns\**</span> <span title="validated upfront with error">*417.91 µs\**</span> | <span title="unvalidated">*387.42 ns\**</span> <span title="validated upfront with error">*418.58 µs\**</span> | <span title="unvalidated">*235.48 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.10%\**</span> <span title="prepend">*19.87%\**</span> | 15.24% | 47.09% | 62.06% | 64.44% | 3.01% |
| [bin-proto 0.12.7][bin-proto] | 9.92% | 19.54% | 44.80% | 71.21% | 76.36% | 3.41% |
| [bincode 2.0.1][bincode] | 41.13% | 34.72% | 57.08% | 71.86% | 75.55% | 3.66% |
| [bincode 1.3.3][bincode1] | 13.60% | 27.99% | 43.29% | 70.34% | 76.54% | 3.36% |
| [bitcode 0.6.6][bitcode] | 73.10% | 53.34% | 82.64% | 91.48% | 94.40% | 11.39% |
| [borsh 1.5.7][borsh] | 17.31% | 43.19% | 52.74% | 72.46% | 77.29% | 3.46% |
| [capnp 0.23.2][capnp] | 22.35% | † | 29.46% | 51.92% | 64.78% | 2.39% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 15.49% | 6.70% | 13.35% | 47.38% | 54.79% | 1.63% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.14% | 2.41% | 13.35% | 47.38% | 54.79% | 1.60% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.13% | 5.80% | 13.35% | 47.38% | 54.79% | 1.65% |
| [columnar 0.11.1][columnar] | 55.78% | 33.56% <span title="copy_from">*100.00%\**</span> | 51.96% | 80.59% | 89.48% | 7.30% |
| [compactly 0.1.6][compactly] | 0.79% | 2.24% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 38.11% | 33.42% | 60.81% | 75.59% | 79.61% | 3.84% |
| [dlhn 0.1.7][dlhn] | 11.34% | 17.24% | 61.21% | 74.55% | 76.73% | 3.78% |
| [flatbuffers 25.12.19][flatbuffers] | 10.26% | † | 34.51% | 55.81% | 63.30% | 2.60% |
| [flexbuffers 25.2.10][flexbuffers] | 1.22% | 3.52% | 15.00% | 30.22% | 28.89% | 1.02% |
| json:<br> [flexon 0.4.5][flexon] | 3.32% | 5.16% | 8.55% | 33.58% | 43.56% | 1.01% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.28% | 3.97% | 8.55% | 33.58% | 43.56% | 1.01% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.27% | 4.90% | 8.55% | 33.58% | 43.56% | 1.00% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 53.60% | 21.98% | 55.02% | 69.48% | 70.55% | 3.64% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 4.87% | 11.58% | 45.99% | 63.67% | 65.32% | 2.88% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 27.65% | 22.92% | 44.73% | 63.06% | 64.61% | 2.91% |
| [minicbor 1.0.0][minicbor] | 21.06% | 10.90% | 45.16% | 62.94% | 64.08% | 2.72% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.69% | 7.63% | 45.35% | 62.86% | 63.54% | 2.81% |
| [nanoserde 0.2.1][nanoserde] | 38.86% | 45.32% | 44.29% | 70.78% | 76.22% | 3.36% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 38.67% | 51.88% | 55.76% | 2.49% |
| [postcard 1.1.1][postcard] | 27.20% | 30.06% | 61.21% | 74.11% | 77.08% | 4.04% |
| [pot 3.0.1][pot] | 3.61% | 4.18% | 30.81% | 54.19% | 61.80% | 2.14% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.22%\**</span> <span title="populate + encode">*5.39%\**</span> | 14.45% | 43.16% | 60.03% | 61.97% | 2.76% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*8.61%\**</span> <span title="populate + encode">*3.85%\**</span> | 10.27% | 43.16% | 60.03% | 61.97% | 2.74% |
| [rkyv 0.8.10][rkyv] | 51.50% | <span title="unvalidated">*56.80%\**</span> <span title="validated upfront with error">*47.37%\**</span> | 38.67% | 58.05% | 66.32% | 2.59% |
| [ron 0.10.1][ron] | 1.15% | 0.84% | 9.25% | 35.96% | 43.95% | 0.99% |
| [savefile 0.18.6][savefile] | 58.12% | 49.44% | 44.80% | 71.21% | 76.36% | 3.33% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.08% | 38.05% | 60.81% | 75.47% | 79.42% | 3.97% |
| [serde-brief 0.1.1][serde-brief] | 9.05% | 5.68% | 11.55% | 44.72% | 51.20% | 1.48% |
| [serde_bare 0.5.0][serde_bare] | 10.50% | 25.18% | 60.81% | 75.59% | 79.60% | 3.91% |
| [speedy 0.8.7][speedy] | 67.97% | 51.26% | 50.65% | 71.73% | 77.33% | 3.51% |
| [wincode 0.5.3][wincode] | 85.11% | 54.13% | 44.80% | 71.21% | 76.36% | 3.44% |
| [wiring 0.2.4][wiring] | 78.13% | 45.71% | 44.80% | 69.43% | 74.13% | 3.21% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.46%\**</span> | <span title="validated on-demand with error">*52.22%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 2.15% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.74%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.94%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*99.12%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.76%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
[msgpacker]: https://crates.io/crates/msgpacker/0.7.1
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
