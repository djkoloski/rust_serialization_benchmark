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

## Last updated: 2025-06-06 20:18:54

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.89.0-nightly (ccf3198de 2025-06-05)
binary: rustc
commit-hash: ccf3198de316b488ee17441935182e9d5292b4d3
commit-date: 2025-06-05
host: x86_64-unknown-linux-gnu
release: 1.89.0-nightly
LLVM version: 20.1.5
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
BogoMIPS:                             4890.85
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*431.22 µs\**</span> <span title="prepend">*414.06 µs\**</span> | 2.5587 ms | 971.13 µs | 804955 | 328941 | 284849 | 4.1027 ms |
| [bincode 2.0.1][bincode] | 326.93 µs | 2.2742 ms | 689.23 µs | 741295 | 303944 | 256422 | 3.6286 ms |
| [bincode 1.3.3][bincode1] | 555.30 µs | 2.1204 ms | 617.69 µs | 1045784 | 373127 | 311553 | 4.4780 ms |
| [bitcode 0.6.6][bitcode] | 145.83 µs | 1.4516 ms | 60.324 µs | 703710 | 288826 | 227322 | 2.5016 ms |
| [borsh 1.5.7][borsh] | 547.65 µs | 2.1284 ms | † | 885780 | 362204 | 286248 | 4.1400 ms |
| [capnp 0.21.1][capnp] | 450.22 µs | † | † | 1443216 | 513986 | 426532 | 6.3168 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 661.24 µs | 5.2868 ms | 3.3594 ms | 1407835 | 403440 | 323561 | 4.7689 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.1269 ms | 12.481 ms | † | 1407835 | 403440 | 323561 | 4.7216 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 2.0916 ms | 5.1676 ms | 3.2237 ms | 1407835 | 403440 | 323561 | 4.6665 ms |
| [databuf 0.5.0][databuf] | 260.26 µs | 2.0888 ms | 670.12 µs | 765778 | 311715 | 263914 | 3.8393 ms |
| [dlhn 0.1.7][dlhn] | 728.06 µs | 2.6297 ms | † | 724953 | 301446 | 253056 | 3.1810 ms |
| [flatbuffers 25.2.10][flatbuffers] | 1.0434 ms | † | † | 1276368 | 468539 | 388381 | 4.7943 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6536 ms | 6.0675 ms | † | 1827461 | 470560 | 360727 | 5.3934 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1094 ms | 4.6290 ms | † | 1827461 | 470560 | 360727 | 5.3933 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.1762 ms | 2.5645 ms | † | 764996 | 315291 | 264212 | 3.9015 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4595 ms | 3.1520 ms | 1.4460 ms | 784997 | 325384 | 277608 | 3.7349 ms |
| [minicbor 1.0.0][minicbor] | 432.01 µs | 2.9456 ms | 1.3724 ms | 817830 | 332671 | 284034 | 3.9896 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2982 ms | 4.3517 ms | 2.8940 ms | 818669 | 332556 | 284797 | 4.2935 ms |
| [nanoserde 0.2.1][nanoserde] | 251.87 µs | 2.0821 ms | † | 1045784 | 373127 | 311553 | 4.1003 ms |
| [postcard 1.1.1][postcard] | 418.16 µs | 2.3103 ms | 855.99 µs | 724953 | 302399 | 252968 | 3.1721 ms |
| [pot 3.0.1][pot] | 2.3920 ms | 6.7651 ms | 5.1501 ms | 971922 | 372513 | 303636 | 4.4242 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*940.51 µs\**</span> <span title="populate + encode">*2.4838 ms\**</span> | 3.2624 ms | † | 884628 | 363130 | 314959 | 4.7386 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.1597 ms\**</span> <span title="populate + encode">*2.9754 ms\**</span> | 3.8189 ms | † | 884628 | 363130 | 314959 | 4.7763 ms |
| [rkyv 0.8.10][rkyv] | 242.21 µs | <span title="unvalidated">*1.5405 ms\**</span> <span title="validated upfront with error">*1.9354 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6209 ms |
| [ron 0.10.1][ron] | 11.008 ms | 25.747 ms | 22.746 ms | 1607459 | 449158 | 349324 | 5.4927 ms |
| [savefile 0.18.6][savefile] | 190.49 µs | 2.1200 ms | † | 1045800 | 373139 | 311562 | 4.2643 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 670.95 µs | 2.4054 ms | † | 765778 | 311743 | 263822 | 3.7759 ms |
| [serde-brief 0.1.1][serde-brief] | 1.5600 ms | 4.8930 ms | 3.0220 ms | 1584946 | 413733 | 339964 | 4.8107 ms |
| [serde_bare 0.5.0][serde_bare] | 697.25 µs | 2.3031 ms | † | 765778 | 311715 | 263914 | 3.4209 ms |
| [speedy 0.8.7][speedy] | 195.63 µs | 1.7214 ms | 377.75 µs | 885780 | 362204 | 286248 | 3.9539 ms |
| [wiring 0.2.4][wiring] | 194.53 µs | 2.0073 ms | † | 1045784 | 337930 | 275808 | 3.5951 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*72.738 ns\**</span> | <span title="validated on-demand with error">*182.13 µs\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4869 ns\**</span> <span title="validated upfront with error">*2.0524 ms\**</span> | <span title="unvalidated">*52.106 µs\**</span> <span title="validated upfront with error">*2.1204 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2517 ns\**</span> <span title="validated upfront with error">*385.56 µs\**</span> | <span title="unvalidated">*10.522 µs\**</span> <span title="validated upfront with error">*408.97 µs\**</span> | <span title="unvalidated">*7.7578 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*33.82%\**</span> <span title="prepend">*35.22%\**</span> | 56.73% | 6.21% | 87.42% | 87.80% | 79.80% | 60.97% |
| [bincode 2.0.1][bincode] | 44.61% | 63.83% | 8.75% | 94.93% | 95.03% | 88.65% | 68.94% |
| [bincode 1.3.3][bincode1] | 26.26% | 68.46% | 9.77% | 67.29% | 77.41% | 72.96% | 55.86% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 26.63% | 68.20% | † | 79.45% | 79.74% | 79.41% | 60.43% |
| [capnp 0.21.1][capnp] | 32.39% | † | † | 48.76% | 56.19% | 53.30% | 39.60% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 22.05% | 27.46% | 1.80% | 49.99% | 71.59% | 70.26% | 52.46% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.53% | 11.63% | † | 49.99% | 71.59% | 70.26% | 52.98% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.97% | 28.09% | 1.87% | 49.99% | 71.59% | 70.26% | 53.61% |
| [databuf 0.5.0][databuf] | 56.03% | 69.49% | 9.00% | 91.89% | 92.66% | 86.13% | 65.16% |
| [dlhn 0.1.7][dlhn] | 20.03% | 55.20% | † | 97.07% | 95.81% | 89.83% | 78.64% |
| [flatbuffers 25.2.10][flatbuffers] | 13.98% | † | † | 55.13% | 61.64% | 58.53% | 52.18% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.99% | 23.92% | † | 38.51% | 61.38% | 63.02% | 46.38% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.91% | 31.36% | † | 38.51% | 61.38% | 63.02% | 46.38% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 12.40% | 56.60% | † | 91.99% | 91.61% | 86.04% | 64.12% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.99% | 46.05% | 4.17% | 89.64% | 88.76% | 81.89% | 66.98% |
| [minicbor 1.0.0][minicbor] | 33.76% | 49.28% | 4.40% | 86.05% | 86.82% | 80.03% | 62.70% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.75% | 33.36% | 2.08% | 85.96% | 86.85% | 79.82% | 58.26% |
| [nanoserde 0.2.1][nanoserde] | 57.90% | 69.72% | † | 67.29% | 77.41% | 72.96% | 61.01% |
| [postcard 1.1.1][postcard] | 34.87% | 62.83% | 7.05% | 97.07% | 95.51% | 89.86% | 78.86% |
| [pot 3.0.1][pot] | 6.10% | 21.46% | 1.17% | 72.40% | 77.53% | 74.87% | 56.54% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*15.51%\**</span> <span title="populate + encode">*5.87%\**</span> | 44.49% | † | 79.55% | 79.54% | 72.18% | 52.79% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.57%\**</span> <span title="populate + encode">*4.90%\**</span> | 38.01% | † | 79.55% | 79.54% | 72.18% | 52.38% |
| [rkyv 0.8.10][rkyv] | 60.21% | <span title="unvalidated">*94.23%\**</span> <span title="validated upfront with error">*75.00%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.14% |
| [ron 0.10.1][ron] | 1.32% | 5.64% | 0.27% | 43.78% | 64.30% | 65.07% | 45.54% |
| [savefile 0.18.6][savefile] | 76.56% | 68.47% | † | 67.29% | 77.40% | 72.96% | 58.66% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 21.73% | 60.35% | † | 91.89% | 92.65% | 86.16% | 66.25% |
| [serde-brief 0.1.1][serde-brief] | 9.35% | 29.67% | 2.00% | 44.40% | 69.81% | 66.87% | 52.00% |
| [serde_bare 0.5.0][serde_bare] | 20.92% | 63.03% | † | 91.89% | 92.66% | 86.13% | 73.13% |
| [speedy 0.8.7][speedy] | 74.54% | 84.33% | 15.97% | 79.45% | 79.74% | 79.41% | 63.27% |
| [wiring 0.2.4][wiring] | 74.97% | 72.32% | † | 67.29% | 85.47% | 82.42% | 69.58% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*5.78%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.33%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.19%\**</span> <span title="validated upfront with error">*0.50%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.57%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.2370 ms\**</span> <span title="prepend">*8.8748 ms\**</span> | 7.7701 ms | 8625005 | 6443961 | 6231572 | 71.513 ms |
| [bincode 2.0.1][bincode] | 2.4118 ms | 1.0256 ms | 6000005 | 5378497 | 5346882 | 8.4095 ms |
| [bincode 1.3.3][bincode1] | 5.2330 ms | 927.19 µs | 6000008 | 5378500 | 5346908 | 8.4303 ms |
| [bitcode 0.6.6][bitcode] | 1.4146 ms | 799.27 µs | 6000006 | 5182295 | 4921841 | 13.731 ms |
| [borsh 1.5.7][borsh] | 6.2189 ms | 4.3086 ms | 6000004 | 5378496 | 5346866 | 8.4443 ms |
| [capnp 0.21.1][capnp] | 5.3691 ms | † | 14000088 | 7130367 | 6046182 | 85.809 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 9.9014 ms | 48.960 ms | 13125016 | 7524114 | 6757437 | 90.781 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 71.604 ms | 119.83 ms | 13122324 | 7524660 | 6759128 | 89.698 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 37.259 ms | 47.098 ms | 13122324 | 7524660 | 6759128 | 91.956 ms |
| [databuf 0.5.0][databuf] | 2.4141 ms | 5.3595 ms | 6000003 | 5378495 | 5346897 | 8.4732 ms |
| [dlhn 0.1.7][dlhn] | 6.1650 ms | 7.9990 ms | 6000003 | 5378495 | 5346897 | 8.4375 ms |
| [flatbuffers 25.2.10][flatbuffers] | 950.00 µs | † | 6000024 | 5378434 | 5346878 | 8.5192 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 87.467 ms | 92.376 ms | 26192883 | 9566084 | 8584671 | 159.06 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 51.881 ms | 73.062 ms | 26192883 | 9566084 | 8584671 | 156.11 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 21.401 ms | 5.2389 ms | 7500005 | 6058442 | 6014500 | 10.341 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 18.538 ms | 15.940 ms | 8125006 | 6494876 | 6391037 | 70.591 ms |
| [minicbor 1.0.0][minicbor] | 6.0520 ms | 11.863 ms | 8125006 | 6494907 | 6390894 | 69.202 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 118.31 ms | 26.035 ms | 8125037 | 6493484 | 6386940 | 70.046 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2488 ms | 949.86 µs | 6000008 | 5378500 | 5346908 | 8.7511 ms |
| [postcard 1.1.1][postcard] | 481.16 µs | 1.4113 ms | 6000003 | 5378495 | 5346897 | 8.4521 ms |
| [pot 3.0.1][pot] | 40.769 ms | 75.769 ms | 10122342 | 6814618 | 6852252 | 82.646 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*7.8777 ms\**</span> <span title="populate + encode">*8.4283 ms\**</span> | 16.128 ms | 8750000 | 6665735 | 6421877 | 72.451 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*15.372 ms\**</span> <span title="populate + encode">*31.708 ms\**</span> | 29.249 ms | 8750000 | 6665735 | 6421877 | 78.035 ms |
| [rkyv 0.8.10][rkyv] | 148.55 µs | <span title="unvalidated">*201.83 µs\**</span> <span title="validated upfront with error">*202.19 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.3325 ms |
| [ron 0.10.1][ron] | 168.83 ms | 540.40 ms | 22192885 | 8970395 | 8137334 | 150.59 ms |
| [savefile 0.18.6][savefile] | 148.75 µs | 209.99 µs | 6000024 | 5378519 | 5346896 | 8.7616 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1153 ms | 4.1446 ms | 6000004 | 5378496 | 5346866 | 8.4105 ms |
| [serde-brief 0.1.1][serde-brief] | 23.495 ms | 36.033 ms | 15750015 | 8024540 | 6813667 | 93.865 ms |
| [serde_bare 0.5.0][serde_bare] | 5.1273 ms | 4.7209 ms | 6000003 | 5378495 | 5346897 | 8.8205 ms |
| [speedy 0.8.7][speedy] | 148.67 µs | 148.91 µs | 6000004 | 5378496 | 5346866 | 8.7643 ms |
| [wiring 0.2.4][wiring] | 149.59 µs | 355.45 µs | 6000008 | 5378952 | 5346905 | 8.7337 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*101.93 ns\**</span> | <span title="validated on-demand with error">*2.2738 ms\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4859 ns\**</span> <span title="validated upfront with error">*45.112 ns\**</span> | <span title="unvalidated">*52.564 µs\**</span> <span title="validated upfront with error">*77.917 µs\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*5.2909 ns\**</span> | <span title="unvalidated">*38.893 µs\**</span> <span title="validated upfront with error">*38.898 µs\**</span> | <span title="unvalidated">*76.998 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.05%\**</span> <span title="prepend">*1.67%\**</span> | 1.92% | 69.57% | 80.42% | 78.98% | 11.65% |
| [bincode 2.0.1][bincode] | 6.16% | 14.52% | 100.00% | 96.35% | 92.05% | 99.08% |
| [bincode 1.3.3][bincode1] | 2.84% | 16.06% | 100.00% | 96.35% | 92.05% | 98.84% |
| [bitcode 0.6.6][bitcode] | 10.50% | 18.63% | 100.00% | 100.00% | 100.00% | 60.69% |
| [borsh 1.5.7][borsh] | 2.39% | 3.46% | 100.00% | 96.35% | 92.05% | 98.68% |
| [capnp 0.21.1][capnp] | 2.77% | † | 42.86% | 72.68% | 81.40% | 9.71% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.50% | 0.30% | 45.71% | 68.88% | 72.84% | 9.18% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.21% | 0.12% | 45.72% | 68.87% | 72.82% | 9.29% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.40% | 0.32% | 45.72% | 68.87% | 72.82% | 9.06% |
| [databuf 0.5.0][databuf] | 6.15% | 2.78% | 100.00% | 96.35% | 92.05% | 98.34% |
| [dlhn 0.1.7][dlhn] | 2.41% | 1.86% | 100.00% | 96.35% | 92.05% | 98.76% |
| [flatbuffers 25.2.10][flatbuffers] | 15.64% | † | 100.00% | 96.35% | 92.05% | 97.81% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.16% | 22.91% | 54.17% | 57.33% | 5.24% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.29% | 0.20% | 22.91% | 54.17% | 57.33% | 5.34% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 0.69% | 2.84% | 80.00% | 85.54% | 81.83% | 80.58% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.80% | 0.93% | 73.85% | 79.79% | 77.01% | 11.80% |
| [minicbor 1.0.0][minicbor] | 2.45% | 1.26% | 73.85% | 79.79% | 77.01% | 12.04% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.13% | 0.57% | 73.85% | 79.81% | 77.06% | 11.90% |
| [nanoserde 0.2.1][nanoserde] | 11.90% | 15.68% | 100.00% | 96.35% | 92.05% | 95.22% |
| [postcard 1.1.1][postcard] | 30.87% | 10.55% | 100.00% | 96.35% | 92.05% | 98.58% |
| [pot 3.0.1][pot] | 0.36% | 0.20% | 59.27% | 76.05% | 71.83% | 10.08% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.89%\**</span> <span title="populate + encode">*1.76%\**</span> | 0.92% | 68.57% | 77.75% | 76.64% | 11.50% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*0.97%\**</span> <span title="populate + encode">*0.47%\**</span> | 0.51% | 68.57% | 77.75% | 76.64% | 10.68% |
| [rkyv 0.8.10][rkyv] | 100.00% | <span title="unvalidated">*73.78%\**</span> <span title="validated upfront with error">*73.65%\**</span> | 100.00% | 96.35% | 92.05% | 100.00% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 27.04% | 57.77% | 60.48% | 5.53% |
| [savefile 0.18.6][savefile] | 99.87% | 70.91% | 100.00% | 96.35% | 92.05% | 95.10% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.90% | 3.59% | 100.00% | 96.35% | 92.05% | 99.07% |
| [serde-brief 0.1.1][serde-brief] | 0.63% | 0.41% | 38.10% | 64.58% | 72.23% | 8.88% |
| [serde_bare 0.5.0][serde_bare] | 2.90% | 3.15% | 100.00% | 96.35% | 92.05% | 94.47% |
| [speedy 0.8.7][speedy] | 99.92% | 100.00% | 100.00% | 96.35% | 92.05% | 95.07% |
| [wiring 0.2.4][wiring] | 99.30% | 41.89% | 100.00% | 96.34% | 92.05% | 95.41% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.22%\**</span> | <span title="validated on-demand with error">*1.71%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*2.76%\**</span> | <span title="unvalidated">*73.99%\**</span> <span title="validated upfront with error">*49.92%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*23.50%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.99%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*928.47 µs\**</span> <span title="prepend">*792.09 µs\**</span> | 3.1085 ms | 1.7436 ms | 489348 | 281173 | 249360 | 2.6964 ms |
| [bincode 2.0.1][bincode] | 314.37 µs | 1.8580 ms | 803.43 µs | 367413 | 221291 | 206242 | 2.0418 ms |
| [bincode 1.3.3][bincode1] | 598.68 µs | 1.8615 ms | 887.93 µs | 569975 | 240525 | 231884 | 2.4883 ms |
| [bitcode 0.6.6][bitcode] | 129.68 µs | 1.2648 ms | 171.88 µs | 327688 | 200947 | 182040 | 789.00 µs |
| [borsh 1.5.7][borsh] | 550.28 µs | 1.8484 ms | † | 446595 | 234236 | 209834 | 2.0699 ms |
| [capnp 0.21.1][capnp] | 430.78 µs | † | † | 803896 | 335606 | 280744 | 3.5047 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 805.71 µs | 4.7613 ms | 3.5860 ms | 1109831 | 344745 | 274333 | 3.4228 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.7386 ms | 10.377 ms | † | 1109821 | 344751 | 274345 | 3.4115 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8746 ms | 4.8242 ms | 3.5185 ms | 1109821 | 344751 | 274345 | 3.3941 ms |
| [databuf 0.5.0][databuf] | 311.39 µs | 1.7618 ms | 807.84 µs | 356311 | 213062 | 198403 | 1.9470 ms |
| [dlhn 0.1.7][dlhn] | 778.39 µs | 2.6980 ms | † | 366496 | 220600 | 205586 | 2.0022 ms |
| [flatbuffers 25.2.10][flatbuffers] | 3.2589 ms | † | † | 849472 | 347816 | 294871 | 3.4949 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.7189 ms | 6.8133 ms | † | 1623191 | 466527 | 359157 | 5.6616 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2313 ms | 4.6966 ms | † | 1623191 | 466527 | 359157 | 5.5900 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 943.67 µs | 2.8851 ms | † | 391251 | 236877 | 220395 | 2.1699 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.5462 ms | 3.0275 ms | 1.6941 ms | 424533 | 245214 | 226077 | 2.2340 ms |
| [minicbor 1.0.0][minicbor] | 555.56 µs | 3.3932 ms | 1.8928 ms | 428773 | 249857 | 228630 | 2.2876 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0034 ms | 4.0823 ms | 2.9480 ms | 449745 | 252432 | 230965 | 2.2928 ms |
| [nanoserde 0.2.1][nanoserde] | 269.40 µs | 1.9188 ms | † | 567975 | 239930 | 231872 | 2.4382 ms |
| [postcard 1.1.1][postcard] | 447.43 µs | 2.1013 ms | 811.91 µs | 367489 | 221913 | 207244 | 2.0469 ms |
| [pot 3.0.1][pot] | 2.4054 ms | 6.5295 ms | 5.2533 ms | 599125 | 299158 | 247675 | 2.7014 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.2744 ms\**</span> <span title="populate + encode">*3.0091 ms\**</span> | 3.5914 ms | † | 596811 | 305319 | 268737 | 2.9236 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0607 ms\**</span> <span title="populate + encode">*3.0287 ms\**</span> | 3.8712 ms | † | 596811 | 305319 | 268737 | 2.9926 ms |
| [rkyv 0.8.10][rkyv] | 344.34 µs | <span title="unvalidated">*1.4987 ms\**</span> <span title="validated upfront with error">*1.8677 ms\**</span> | † | 603776 | 254776 | 219421 | 2.2770 ms |
| [ron 0.10.1][ron] | 7.7634 ms | 25.921 ms | 25.020 ms | 1465223 | 434935 | 342907 | 5.4644 ms |
| [savefile 0.18.6][savefile] | 213.67 µs | 1.8277 ms | † | 566991 | 239362 | 231478 | 2.3995 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 632.23 µs | 2.1144 ms | † | 356311 | 212976 | 198423 | 1.9939 ms |
| [serde-brief 0.1.1][serde-brief] | 1.4037 ms | 5.2633 ms | 3.6909 ms | 1276014 | 373898 | 293384 | 3.6500 ms |
| [serde_bare 0.5.0][serde_bare] | 754.74 µs | 2.3688 ms | † | 356311 | 213062 | 198403 | 1.9769 ms |
| [speedy 0.8.7][speedy] | 260.67 µs | 1.6816 ms | 560.66 µs | 449595 | 234970 | 210192 | 2.0734 ms |
| [wiring 0.2.4][wiring] | 186.38 µs | 1.8622 ms | † | 566975 | 247810 | 225086 | 2.4909 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*72.344 ns\**</span> | <span title="validated on-demand with error">*416.88 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4863 ns\**</span> <span title="validated upfront with error">*2.4213 ms\**</span> | <span title="unvalidated">*1.3592 µs\**</span> <span title="validated upfront with error">*2.4273 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2437 ns\**</span> <span title="validated upfront with error">*353.32 µs\**</span> | <span title="unvalidated">*164.11 ns\**</span> <span title="validated upfront with error">*352.94 µs\**</span> | <span title="unvalidated">*804.38 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*13.97%\**</span> <span title="prepend">*16.37%\**</span> | 40.69% | 9.86% | 66.96% | 71.47% | 73.00% | 29.26% |
| [bincode 2.0.1][bincode] | 41.25% | 68.07% | 21.39% | 89.19% | 90.81% | 88.27% | 38.64% |
| [bincode 1.3.3][bincode1] | 21.66% | 67.95% | 19.36% | 57.49% | 83.55% | 78.50% | 31.71% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 23.57% | 68.43% | † | 73.37% | 85.79% | 86.75% | 38.12% |
| [capnp 0.21.1][capnp] | 30.10% | † | † | 40.76% | 59.88% | 64.84% | 22.51% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.10% | 26.56% | 4.79% | 29.53% | 58.29% | 66.36% | 23.05% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.47% | 12.19% | † | 29.53% | 58.29% | 66.35% | 23.13% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.92% | 26.22% | 4.89% | 29.53% | 58.29% | 66.35% | 23.25% |
| [databuf 0.5.0][databuf] | 41.65% | 71.79% | 21.28% | 91.97% | 94.31% | 91.75% | 40.52% |
| [dlhn 0.1.7][dlhn] | 16.66% | 46.88% | † | 89.41% | 91.09% | 88.55% | 39.41% |
| [flatbuffers 25.2.10][flatbuffers] | 3.98% | † | † | 38.58% | 57.77% | 61.74% | 22.58% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.49% | 18.56% | † | 20.19% | 43.07% | 50.69% | 13.94% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.81% | 26.93% | † | 20.19% | 43.07% | 50.69% | 14.11% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 13.74% | 43.84% | † | 83.75% | 84.83% | 82.60% | 36.36% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.39% | 41.78% | 10.15% | 77.19% | 81.95% | 80.52% | 35.32% |
| [minicbor 1.0.0][minicbor] | 23.34% | 37.27% | 9.08% | 76.42% | 80.42% | 79.62% | 34.49% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.59% | 30.98% | 5.83% | 72.86% | 79.60% | 78.82% | 34.41% |
| [nanoserde 0.2.1][nanoserde] | 48.14% | 65.92% | † | 57.69% | 83.75% | 78.51% | 32.36% |
| [postcard 1.1.1][postcard] | 28.98% | 60.19% | 21.17% | 89.17% | 90.55% | 87.84% | 38.55% |
| [pot 3.0.1][pot] | 5.39% | 19.37% | 3.27% | 54.69% | 67.17% | 73.50% | 29.21% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*10.18%\**</span> <span title="populate + encode">*4.31%\**</span> | 35.22% | † | 54.91% | 65.82% | 67.74% | 26.99% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.23%\**</span> <span title="populate + encode">*4.28%\**</span> | 32.67% | † | 54.91% | 65.82% | 67.74% | 26.37% |
| [rkyv 0.8.10][rkyv] | 37.66% | <span title="unvalidated">*84.39%\**</span> <span title="validated upfront with error">*67.72%\**</span> | † | 54.27% | 78.87% | 82.96% | 34.65% |
| [ron 0.10.1][ron] | 1.67% | 4.88% | 0.69% | 22.36% | 46.20% | 53.09% | 14.44% |
| [savefile 0.18.6][savefile] | 60.69% | 69.20% | † | 57.79% | 83.95% | 78.64% | 32.88% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.51% | 59.82% | † | 91.97% | 94.35% | 91.74% | 39.57% |
| [serde-brief 0.1.1][serde-brief] | 9.24% | 24.03% | 4.66% | 25.68% | 53.74% | 62.05% | 21.62% |
| [serde_bare 0.5.0][serde_bare] | 17.18% | 53.39% | † | 91.97% | 94.31% | 91.75% | 39.91% |
| [speedy 0.8.7][speedy] | 49.75% | 75.21% | 30.66% | 72.89% | 85.52% | 86.61% | 38.05% |
| [wiring 0.2.4][wiring] | 69.58% | 67.92% | † | 57.80% | 81.09% | 80.88% | 31.68% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*39.37%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.07%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.4797 ms\**</span> <span title="prepend">*2.5423 ms\**</span> | 8.4708 ms | 1704643 | 1294259 | 1245668 | 11.800 ms |
| [bincode 2.0.1][bincode] | 1.1885 ms | 3.7869 ms | 1406257 | 1117802 | 1062438 | 9.4868 ms |
| [bincode 1.3.3][bincode1] | 3.6829 ms | 4.2364 ms | 1854234 | 1141994 | 1048745 | 10.224 ms |
| [bitcode 0.6.6][bitcode] | 726.54 µs | 2.3762 ms | 971318 | 878034 | 850340 | 2.8587 ms |
| [borsh 1.5.7][borsh] | 2.9272 ms | 2.9398 ms | 1521989 | 1108471 | 1038528 | 9.9862 ms |
| [capnp 0.21.1][capnp] | 2.2103 ms | † | 2724288 | 1546992 | 1239111 | 14.634 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.0765 ms | 18.514 ms | 6012539 | 1695215 | 1464951 | 21.340 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 22.941 ms | 57.007 ms | 6012373 | 1695146 | 1465025 | 21.557 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.214 ms | 22.008 ms | 6012373 | 1695146 | 1465025 | 21.267 ms |
| [databuf 0.5.0][databuf] | 1.3150 ms | 3.7291 ms | 1319999 | 1062631 | 1008334 | 8.9206 ms |
| [dlhn 0.1.7][dlhn] | 4.9987 ms | 6.4224 ms | 1311281 | 1077520 | 1046095 | 8.7446 ms |
| [flatbuffers 25.2.10][flatbuffers] | 5.2144 ms | † | 2325620 | 1439185 | 1268060 | 13.813 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.469 ms | 31.701 ms | 9390461 | 2391679 | 1842767 | 34.901 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.557 ms | 26.742 ms | 9390461 | 2391679 | 1842767 | 34.742 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.4219 ms | 7.0035 ms | 1458773 | 1156055 | 1137788 | 10.108 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.404 ms | 11.097 ms | 1745322 | 1261627 | 1228923 | 11.571 ms |
| [minicbor 1.0.0][minicbor] | 2.4183 ms | 11.489 ms | 1777386 | 1276218 | 1252558 | 12.723 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.712 ms | 17.738 ms | 1770060 | 1277755 | 1263362 | 12.654 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2755 ms | 2.8102 ms | 1812404 | 1134820 | 1053109 | 10.229 ms |
| [postcard 1.1.1][postcard] | 1.9925 ms | 4.2476 ms | 1311281 | 1083900 | 1041434 | 8.7965 ms |
| [pot 3.0.1][pot] | 14.090 ms | 31.406 ms | 2604812 | 1482233 | 1298928 | 15.899 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*5.4077 ms\**</span> <span title="populate + encode">*9.2712 ms\**</span> | 8.5418 ms | 1859886 | 1338076 | 1295351 | 12.146 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.5142 ms\**</span> <span title="populate + encode">*12.706 ms\**</span> | 11.813 ms | 1859886 | 1338076 | 1295351 | 12.413 ms |
| [rkyv 0.8.10][rkyv] | 933.34 µs | <span title="unvalidated">*2.1576 ms\**</span> <span title="validated upfront with error">*2.6284 ms\**</span> | 2075936 | 1383779 | 1210377 | 12.887 ms |
| [ron 0.10.1][ron] | 41.662 ms | 159.56 ms | 8677703 | 2233642 | 1826180 | 34.310 ms |
| [savefile 0.18.6][savefile] | 859.54 µs | 2.7436 ms | 1791505 | 1128012 | 1051153 | 10.281 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1159 ms | 3.4334 ms | 1319999 | 1064380 | 1010708 | 8.8949 ms |
| [serde-brief 0.1.1][serde-brief] | 6.5814 ms | 20.685 ms | 6951772 | 1796265 | 1567819 | 23.984 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9008 ms | 4.8025 ms | 1319999 | 1062645 | 1008349 | 8.9764 ms |
| [speedy 0.8.7][speedy] | 785.21 µs | 2.4705 ms | 1584734 | 1119837 | 1037992 | 9.9907 ms |
| [wiring 0.2.4][wiring] | 665.92 µs | 2.7673 ms | 1791489 | 1156963 | 1082815 | 10.893 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*72.358 ns\**</span> | <span title="validated on-demand with error">*710.88 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4863 ns\**</span> <span title="validated upfront with error">*5.5389 ms\**</span> | <span title="unvalidated">*2.6089 µs\**</span> <span title="validated upfront with error">*5.5468 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2435 ns\**</span> <span title="validated upfront with error">*448.70 µs\**</span> | <span title="unvalidated">*415.10 ns\**</span> <span title="validated upfront with error">*450.24 µs\**</span> | <span title="unvalidated">*236.62 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.87%\**</span> <span title="prepend">*26.19%\**</span> | 25.47% | 56.98% | 67.84% | 68.26% | 24.23% |
| [bincode 2.0.1][bincode] | 56.03% | 56.98% | 69.07% | 78.55% | 80.04% | 30.13% |
| [bincode 1.3.3][bincode1] | 18.08% | 50.93% | 52.38% | 76.89% | 81.08% | 27.96% |
| [bitcode 0.6.6][bitcode] | 91.66% | 90.80% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 22.75% | 73.39% | 63.82% | 79.21% | 81.88% | 28.63% |
| [capnp 0.21.1][capnp] | 30.13% | † | 35.65% | 56.76% | 68.63% | 19.54% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 21.65% | 11.65% | 16.15% | 51.79% | 58.05% | 13.40% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.90% | 3.78% | 16.16% | 51.80% | 58.04% | 13.26% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.52% | 9.80% | 16.16% | 51.80% | 58.04% | 13.44% |
| [databuf 0.5.0][databuf] | 50.64% | 57.86% | 73.58% | 82.63% | 84.33% | 32.05% |
| [dlhn 0.1.7][dlhn] | 13.32% | 33.59% | 74.07% | 81.49% | 81.29% | 32.69% |
| [flatbuffers 25.2.10][flatbuffers] | 12.77% | † | 41.77% | 61.01% | 67.06% | 20.70% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.25% | 6.81% | 10.34% | 36.71% | 46.14% | 8.19% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.76% | 8.07% | 10.34% | 36.71% | 46.14% | 8.23% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 27.50% | 30.81% | 66.58% | 75.95% | 74.74% | 28.28% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 6.40% | 19.44% | 55.65% | 69.60% | 69.19% | 24.71% |
| [minicbor 1.0.0][minicbor] | 27.54% | 18.78% | 54.65% | 68.80% | 67.89% | 22.47% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.24% | 12.16% | 54.87% | 68.72% | 67.31% | 22.59% |
| [nanoserde 0.2.1][nanoserde] | 52.21% | 76.78% | 53.59% | 77.37% | 80.75% | 27.95% |
| [postcard 1.1.1][postcard] | 33.42% | 50.80% | 74.07% | 81.01% | 81.65% | 32.50% |
| [pot 3.0.1][pot] | 4.73% | 6.87% | 37.29% | 59.24% | 65.46% | 17.98% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*12.31%\**</span> <span title="populate + encode">*7.18%\**</span> | 25.26% | 52.22% | 65.62% | 65.65% | 23.54% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.08%\**</span> <span title="populate + encode">*5.24%\**</span> | 18.26% | 52.22% | 65.62% | 65.65% | 23.03% |
| [rkyv 0.8.10][rkyv] | 71.35% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*82.09%\**</span> | 46.79% | 63.45% | 70.25% | 22.18% |
| [ron 0.10.1][ron] | 1.60% | 1.35% | 11.19% | 39.31% | 46.56% | 8.33% |
| [savefile 0.18.6][savefile] | 77.47% | 78.64% | 54.22% | 77.84% | 80.90% | 27.81% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 21.37% | 62.84% | 73.58% | 82.49% | 84.13% | 32.14% |
| [serde-brief 0.1.1][serde-brief] | 10.12% | 10.43% | 13.97% | 48.88% | 54.24% | 11.92% |
| [serde_bare 0.5.0][serde_bare] | 13.59% | 44.93% | 73.58% | 82.63% | 84.33% | 31.85% |
| [speedy 0.8.7][speedy] | 84.81% | 87.33% | 61.29% | 78.41% | 81.92% | 28.61% |
| [wiring 0.2.4][wiring] | 100.00% | 77.97% | 54.22% | 75.89% | 78.53% | 26.24% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.72%\**</span> | <span title="validated on-demand with error">*58.39%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*15.91%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1013.0
[bincode]: https://crates.io/crates/bincode/2.0.1
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.6
[borsh]: https://crates.io/crates/borsh/1.5.7
[capnp]: https://crates.io/crates/capnp/0.21.1
[cbor4ii]: https://crates.io/crates/cbor4ii/1.0.0
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.2.10
[minicbor]: https://crates.io/crates/minicbor/1.0.0
[msgpacker]: https://crates.io/crates/msgpacker/0.4.8
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.2.1
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.7.5
[postcard]: https://crates.io/crates/postcard/1.1.1
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.13.5
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
[wiring]: https://crates.io/crates/wiring/0.2.4


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
