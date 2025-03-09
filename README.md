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

## Last updated: 2025-03-09 04:24:16

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.87.0-nightly (efea9896f 2025-03-08)
binary: rustc
commit-hash: efea9896f506baa08f40444e07774e827646d57a
commit-date: 2025-03-08
host: x86_64-unknown-linux-gnu
release: 1.87.0-nightly
LLVM version: 20.1.0
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
Flags:                                fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves user_shstk clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
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
| [bilrost 0.1012.3][bilrost] | <span title="encode">*475.11 µs\**</span> <span title="prepend">*432.51 µs\**</span> | 2.6336 ms | 915.50 µs | 804955 | 328941 | 284849 | 4.1958 ms |
| [bincode 2.0.0][bincode] | 315.49 µs | 2.2391 ms | 679.85 µs | 741295 | 303944 | 256422 | 3.6475 ms |
| [bincode 1.3.3][bincode1] | 546.20 µs | 2.0683 ms | 606.03 µs | 1045784 | 373127 | 311553 | 4.4566 ms |
| [bitcode 0.6.5][bitcode] | 138.81 µs | 1.4485 ms | 60.162 µs | 703710 | 288826 | 227322 | 2.5525 ms |
| [borsh 1.5.5][borsh] | 547.74 µs | 2.1951 ms | † | 885780 | 362204 | 286248 | 4.2150 ms |
| [capnp 0.20.6][capnp] | 495.65 µs | † | † | 1443216 | 513986 | 426532 | 6.2371 ms |
| [cbor4ii 1.0.0][cbor4ii] | 654.58 µs | 5.1754 ms | 3.3341 ms | 1407835 | 403440 | 323561 | 5.0940 ms |
| [ciborium 0.2.2][ciborium] | 4.1312 ms | 11.977 ms | † | 1407835 | 403440 | 323561 | 5.0136 ms |
| [databuf 0.5.0][databuf] | 258.66 µs | 1.9987 ms | 665.68 µs | 765778 | 311715 | 263914 | 3.8476 ms |
| [dlhn 0.1.7][dlhn] | 730.01 µs | 2.5256 ms | † | 724953 | 301446 | 253056 | 3.4434 ms |
| [flatbuffers 25.2.10][flatbuffers] | 1.0297 ms | † | † | 1276368 | 468539 | 388381 | 4.8421 ms |
| [minicbor 0.26.1][minicbor] | 488.11 µs | 2.9976 ms | 1.3457 ms | 817830 | 332671 | 284034 | 4.1532 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3038 ms | 4.2985 ms | 2.7488 ms | 818669 | 332556 | 284797 | 4.0350 ms |
| [nanoserde 0.1.37][nanoserde] | 257.05 µs | 2.0637 ms | † | 1045784 | 373127 | 311553 | 4.1603 ms |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 661.43 µs | 2.4183 ms | † | 765778 | 311743 | 263822 | 3.5065 ms |
| [postcard 1.1.1][postcard] | 426.18 µs | 2.2558 ms | 819.28 µs | 724953 | 302399 | 252968 | 3.2747 ms |
| [pot 3.0.1][pot] | 2.3899 ms | 6.9199 ms | 5.4104 ms | 971922 | 372513 | 303636 | 4.3959 ms |
| [prost 0.13.5][prost] | <span title="encode">*936.42 µs\**</span> <span title="populate + encode">*2.4790 ms\**</span> | 3.3301 ms | † | 884628 | 363130 | 314959 | 4.4483 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.1788 ms\**</span> <span title="populate + encode">*2.9977 ms\**</span> | 3.7286 ms | † | 884628 | 363130 | 314959 | 4.4270 ms |
| [rkyv 0.8.10][rkyv] | 248.03 µs | <span title="unvalidated">*1.5456 ms\**</span> <span title="validated upfront with error">*1.9478 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6430 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3618 ms | 3.0580 ms | 1.4100 ms | 784997 | 325384 | 277608 | 3.8072 ms |
| [ron 0.8.1][ron] | 12.154 ms | 15.664 ms | 13.977 ms | 1607459 | 449158 | 349324 | 5.5579 ms |
| [savefile 0.18.5][savefile] | 191.05 µs | 2.2252 ms | † | 1045800 | 373139 | 311562 | 4.2063 ms |
| [serde-brief 0.1.1][serde-brief] | 1.6245 ms | 5.1397 ms | 3.1100 ms | 1584946 | 413733 | 339964 | 4.8956 ms |
| [serde_bare 0.5.0][serde_bare] | 712.10 µs | 2.0522 ms | † | 765778 | 311715 | 263914 | 3.4966 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0901 ms | 4.9326 ms | 3.5093 ms | 1407835 | 403440 | 323561 | 4.7941 ms |
| [serde_json 1.0.140][serde_json] | 3.6927 ms | 6.1232 ms | † | 1827461 | 470560 | 360727 | 5.5718 ms |
| [simd-json 0.14.3][simd-json] | 2.0674 ms | 4.7367 ms | † | 1827461 | 470560 | 360727 | 5.5353 ms |
| [speedy 0.8.7][speedy] | 199.86 µs | 1.7355 ms | 383.41 µs | 885780 | 362204 | 286248 | 3.9197 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*73.320 ns\**</span> | <span title="validated on-demand with error">*176.29 µs\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4873 ns\**</span> <span title="validated upfront with error">*2.0905 ms\**</span> | <span title="unvalidated">*49.548 µs\**</span> <span title="validated upfront with error">*2.1126 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2434 ns\**</span> <span title="validated upfront with error">*391.89 µs\**</span> | <span title="unvalidated">*10.471 µs\**</span> <span title="validated upfront with error">*403.05 µs\**</span> | <span title="unvalidated">*7.5484 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*29.22%\**</span> <span title="prepend">*32.09%\**</span> | 55.00% | 6.57% | 87.42% | 87.80% | 79.80% | 60.83% |
| [bincode 2.0.0][bincode] | 44.00% | 64.69% | 8.85% | 94.93% | 95.03% | 88.65% | 69.98% |
| [bincode 1.3.3][bincode1] | 25.41% | 70.03% | 9.93% | 67.29% | 77.41% | 72.96% | 57.27% |
| [bitcode 0.6.5][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 25.34% | 65.99% | † | 79.45% | 79.74% | 79.41% | 60.56% |
| [capnp 0.20.6][capnp] | 28.01% | † | † | 48.76% | 56.19% | 53.30% | 40.92% |
| [cbor4ii 1.0.0][cbor4ii] | 21.21% | 27.99% | 1.80% | 49.99% | 71.59% | 70.26% | 50.11% |
| [ciborium 0.2.2][ciborium] | 3.36% | 12.09% | † | 49.99% | 71.59% | 70.26% | 50.91% |
| [databuf 0.5.0][databuf] | 53.67% | 72.47% | 9.04% | 91.89% | 92.66% | 86.13% | 66.34% |
| [dlhn 0.1.7][dlhn] | 19.01% | 57.35% | † | 97.07% | 95.81% | 89.83% | 74.13% |
| [flatbuffers 25.2.10][flatbuffers] | 13.48% | † | † | 55.13% | 61.64% | 58.53% | 52.71% |
| [minicbor 0.26.1][minicbor] | 28.44% | 48.32% | 4.47% | 86.05% | 86.82% | 80.03% | 61.46% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.62% | 33.70% | 2.19% | 85.96% | 86.85% | 79.82% | 63.26% |
| [nanoserde 0.1.37][nanoserde] | 54.00% | 70.19% | † | 67.29% | 77.41% | 72.96% | 61.35% |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 20.99% | 59.90% | † | 91.89% | 92.65% | 86.16% | 72.79% |
| [postcard 1.1.1][postcard] | 32.57% | 64.21% | 7.34% | 97.07% | 95.51% | 89.86% | 77.95% |
| [pot 3.0.1][pot] | 5.81% | 20.93% | 1.11% | 72.40% | 77.53% | 74.87% | 58.07% |
| [prost 0.13.5][prost] | <span title="encode">*14.82%\**</span> <span title="populate + encode">*5.60%\**</span> | 43.50% | † | 79.55% | 79.54% | 72.18% | 57.38% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*11.78%\**</span> <span title="populate + encode">*4.63%\**</span> | 38.85% | † | 79.55% | 79.54% | 72.18% | 57.66% |
| [rkyv 0.8.10][rkyv] | 55.97% | <span title="unvalidated">*93.72%\**</span> <span title="validated upfront with error">*74.37%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.98% |
| [rmp-serde 1.3.0][rmp-serde] | 10.19% | 47.37% | 4.27% | 89.64% | 88.76% | 81.89% | 67.04% |
| [ron 0.8.1][ron] | 1.14% | 9.25% | 0.43% | 43.78% | 64.30% | 65.07% | 45.93% |
| [savefile 0.18.5][savefile] | 72.66% | 65.10% | † | 67.29% | 77.40% | 72.96% | 60.68% |
| [serde-brief 0.1.1][serde-brief] | 8.54% | 28.18% | 1.93% | 44.40% | 69.81% | 66.87% | 52.14% |
| [serde_bare 0.5.0][serde_bare] | 19.49% | 70.58% | † | 91.89% | 92.66% | 86.13% | 73.00% |
| [serde_cbor 0.11.2][serde_cbor] | 6.64% | 29.37% | 1.71% | 49.99% | 71.59% | 70.26% | 53.24% |
| [serde_json 1.0.140][serde_json] | 3.76% | 23.66% | † | 38.51% | 61.38% | 63.02% | 45.81% |
| [simd-json 0.14.3][simd-json] | 6.71% | 30.58% | † | 38.51% | 61.38% | 63.02% | 46.11% |
| [speedy 0.8.7][speedy] | 69.45% | 83.46% | 15.69% | 79.45% | 79.74% | 79.41% | 65.12% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*5.94%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.13%\**</span> <span title="validated upfront with error">*0.50%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.60%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*6.7345 ms\**</span> <span title="prepend">*8.8481 ms\**</span> | 7.8496 ms | 8625005 | 6443961 | 6231572 | 74.105 ms |
| [bincode 2.0.0][bincode] | 2.8879 ms | 1.0266 ms | 6000005 | 5378497 | 5346882 | 8.8509 ms |
| [bincode 1.3.3][bincode1] | 5.1591 ms | 4.4768 ms | 6000008 | 5378500 | 5346908 | 8.8827 ms |
| [bitcode 0.6.5][bitcode] | 1.5692 ms | 810.79 µs | 6000006 | 5182295 | 4921841 | 14.180 ms |
| [borsh 1.5.5][borsh] | 6.2311 ms | 4.2193 ms | 6000004 | 5378496 | 5346866 | 8.9066 ms |
| [capnp 0.20.6][capnp] | 6.6791 ms | † | 14000088 | 7130367 | 6046182 | 82.387 ms |
| [cbor4ii 1.0.0][cbor4ii] | 10.002 ms | 48.803 ms | 13125016 | 7524114 | 6757437 | 90.728 ms |
| [ciborium 0.2.2][ciborium] | 73.004 ms | 119.20 ms | 13122324 | 7524660 | 6759128 | 91.183 ms |
| [databuf 0.5.0][databuf] | 2.4155 ms | 5.3477 ms | 6000003 | 5378495 | 5346897 | 8.9130 ms |
| [dlhn 0.1.7][dlhn] | 6.3797 ms | 8.1428 ms | 6000003 | 5378495 | 5346897 | 8.9436 ms |
| [flatbuffers 25.2.10][flatbuffers] | 874.50 µs | † | 6000024 | 5378434 | 5346878 | 9.0242 ms |
| [minicbor 0.26.1][minicbor] | 5.1934 ms | 12.155 ms | 8125006 | 6494907 | 6390894 | 71.415 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 117.51 ms | 26.304 ms | 8125037 | 6493484 | 6386940 | 71.772 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2492 ms | 1.1084 ms | 6000008 | 5378500 | 5346908 | 8.6579 ms |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 5.0696 ms | 4.9713 ms | 6000004 | 5378496 | 5346866 | 8.6280 ms |
| [postcard 1.1.1][postcard] | 480.78 µs | 1.1664 ms | 6000003 | 5378495 | 5346897 | 8.7844 ms |
| [pot 3.0.1][pot] | 39.251 ms | 76.264 ms | 10122342 | 6814618 | 6852252 | 82.966 ms |
| [prost 0.13.5][prost] | <span title="encode">*7.7971 ms\**</span> <span title="populate + encode">*8.5349 ms\**</span> | 12.458 ms | 8750000 | 6665735 | 6421877 | 72.549 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*15.335 ms\**</span> <span title="populate + encode">*31.759 ms\**</span> | 30.214 ms | 8750000 | 6665735 | 6421877 | 80.486 ms |
| [rkyv 0.8.10][rkyv] | 153.59 µs | <span title="unvalidated">*163.73 µs\**</span> <span title="validated upfront with error">*150.43 µs\**</span> | 6000008 | 5378500 | 5346872 | 9.0109 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.705 ms | 16.055 ms | 8125006 | 6494876 | 6391037 | 70.229 ms |
| [ron 0.8.1][ron] | 168.56 ms | 241.74 ms | 22192885 | 8970395 | 8137334 | 153.54 ms |
| [savefile 0.18.5][savefile] | 152.57 µs | 160.86 µs | 6000024 | 5378519 | 5346896 | 8.6993 ms |
| [serde-brief 0.1.1][serde-brief] | 22.908 ms | 38.974 ms | 15750015 | 8024540 | 6813667 | 94.845 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9171 ms | 4.8522 ms | 6000003 | 5378495 | 5346897 | 8.8373 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.873 ms | 46.724 ms | 13122324 | 7524660 | 6759128 | 91.112 ms |
| [serde_json 1.0.140][serde_json] | 86.189 ms | 89.044 ms | 26192883 | 9566084 | 8584671 | 162.19 ms |
| [simd-json 0.14.3][simd-json] | 51.528 ms | 76.187 ms | 26192883 | 9566084 | 8584671 | 156.41 ms |
| [speedy 0.8.7][speedy] | 148.87 µs | 186.75 µs | 6000004 | 5378496 | 5346866 | 8.5236 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*104.69 ns\**</span> | <span title="validated on-demand with error">*2.2738 ms\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4874 ns\**</span> <span title="validated upfront with error">*46.894 ns\**</span> | <span title="unvalidated">*52.341 µs\**</span> <span title="validated upfront with error">*77.753 µs\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2432 ns\**</span> <span title="validated upfront with error">*4.9851 ns\**</span> | <span title="unvalidated">*48.653 µs\**</span> <span title="validated upfront with error">*38.886 µs\**</span> | <span title="unvalidated">*77.025 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*2.21%\**</span> <span title="prepend">*1.68%\**</span> | 1.92% | 69.57% | 80.42% | 78.98% | 11.50% |
| [bincode 2.0.0][bincode] | 5.15% | 14.65% | 100.00% | 96.35% | 92.05% | 96.30% |
| [bincode 1.3.3][bincode1] | 2.89% | 3.36% | 100.00% | 96.35% | 92.05% | 95.96% |
| [bitcode 0.6.5][bitcode] | 9.49% | 18.55% | 100.00% | 100.00% | 100.00% | 60.11% |
| [borsh 1.5.5][borsh] | 2.39% | 3.57% | 100.00% | 96.35% | 92.05% | 95.70% |
| [capnp 0.20.6][capnp] | 2.23% | † | 42.86% | 72.68% | 81.40% | 10.35% |
| [cbor4ii 1.0.0][cbor4ii] | 1.49% | 0.31% | 45.71% | 68.88% | 72.84% | 9.39% |
| [ciborium 0.2.2][ciborium] | 0.20% | 0.13% | 45.72% | 68.87% | 72.82% | 9.35% |
| [databuf 0.5.0][databuf] | 6.16% | 2.81% | 100.00% | 96.35% | 92.05% | 95.63% |
| [dlhn 0.1.7][dlhn] | 2.33% | 1.85% | 100.00% | 96.35% | 92.05% | 95.30% |
| [flatbuffers 25.2.10][flatbuffers] | 17.02% | † | 100.00% | 96.35% | 92.05% | 94.45% |
| [minicbor 0.26.1][minicbor] | 2.87% | 1.24% | 73.85% | 79.79% | 77.01% | 11.94% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.13% | 0.57% | 73.85% | 79.81% | 77.06% | 11.88% |
| [nanoserde 0.1.37][nanoserde] | 11.92% | 13.57% | 100.00% | 96.35% | 92.05% | 98.45% |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 2.94% | 3.03% | 100.00% | 96.35% | 92.05% | 98.79% |
| [postcard 1.1.1][postcard] | 30.96% | 12.90% | 100.00% | 96.35% | 92.05% | 97.03% |
| [pot 3.0.1][pot] | 0.38% | 0.20% | 59.27% | 76.05% | 71.83% | 10.27% |
| [prost 0.13.5][prost] | <span title="encode">*1.91%\**</span> <span title="populate + encode">*1.74%\**</span> | 1.21% | 68.57% | 77.75% | 76.64% | 11.75% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*0.97%\**</span> <span title="populate + encode">*0.47%\**</span> | 0.50% | 68.57% | 77.75% | 76.64% | 10.59% |
| [rkyv 0.8.10][rkyv] | 96.93% | <span title="unvalidated">*91.88%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.05% | 94.59% |
| [rmp-serde 1.3.0][rmp-serde] | 0.95% | 0.94% | 73.85% | 79.79% | 77.01% | 12.14% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.48% | 5.55% |
| [savefile 0.18.5][savefile] | 97.57% | 93.52% | 100.00% | 96.35% | 92.05% | 97.98% |
| [serde-brief 0.1.1][serde-brief] | 0.65% | 0.39% | 38.10% | 64.58% | 72.23% | 8.99% |
| [serde_bare 0.5.0][serde_bare] | 3.03% | 3.10% | 100.00% | 96.35% | 92.05% | 96.45% |
| [serde_cbor 0.11.2][serde_cbor] | 0.41% | 0.32% | 45.72% | 68.87% | 72.82% | 9.36% |
| [serde_json 1.0.140][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.33% | 5.26% |
| [simd-json 0.14.3][simd-json] | 0.29% | 0.20% | 22.91% | 54.17% | 57.33% | 5.45% |
| [speedy 0.8.7][speedy] | 100.00% | 80.55% | 100.00% | 96.35% | 92.05% | 100.00% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.19%\**</span> | <span title="validated on-demand with error">*1.71%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*2.65%\**</span> | <span title="unvalidated">*74.29%\**</span> <span title="validated upfront with error">*50.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.94%\**</span> | <span title="unvalidated">*79.93%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*921.43 µs\**</span> <span title="prepend">*839.35 µs\**</span> | 3.1131 ms | 1.7460 ms | 489348 | 281173 | 249360 | 2.6361 ms |
| [bincode 2.0.0][bincode] | 311.00 µs | 1.8566 ms | 814.53 µs | 367413 | 221291 | 206242 | 2.0592 ms |
| [bincode 1.3.3][bincode1] | 598.67 µs | 1.8280 ms | 862.79 µs | 569975 | 240525 | 231884 | 2.4890 ms |
| [bitcode 0.6.5][bitcode] | 134.91 µs | 1.2563 ms | 169.74 µs | 327688 | 200947 | 182040 | 728.96 µs |
| [borsh 1.5.5][borsh] | 552.81 µs | 1.8135 ms | † | 446595 | 234236 | 209834 | 2.0458 ms |
| [capnp 0.20.6][capnp] | 439.03 µs | † | † | 803896 | 335606 | 280744 | 3.4962 ms |
| [cbor4ii 1.0.0][cbor4ii] | 805.72 µs | 4.8259 ms | 3.5070 ms | 1109831 | 344745 | 274333 | 3.4498 ms |
| [ciborium 0.2.2][ciborium] | 3.7415 ms | 10.229 ms | † | 1109821 | 344751 | 274345 | 3.3816 ms |
| [databuf 0.5.0][databuf] | 317.24 µs | 1.7448 ms | 777.38 µs | 356311 | 213062 | 198403 | 1.9411 ms |
| [dlhn 0.1.7][dlhn] | 741.05 µs | 2.6172 ms | † | 366496 | 220600 | 205586 | 2.1235 ms |
| [flatbuffers 25.2.10][flatbuffers] | 3.2230 ms | † | † | 844168 | 345696 | 293916 | 3.4365 ms |
| [minicbor 0.26.1][minicbor] | 573.20 µs | 3.3786 ms | 1.8701 ms | 428773 | 249857 | 228630 | 2.2730 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0214 ms | 4.1162 ms | 2.9837 ms | 449745 | 252432 | 230965 | 2.3780 ms |
| [nanoserde 0.1.37][nanoserde] | 272.55 µs | 1.9604 ms | † | 567975 | 239930 | 231872 | 2.4730 ms |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 621.79 µs | 2.1071 ms | † | 356311 | 212976 | 198423 | 1.9282 ms |
| [postcard 1.1.1][postcard] | 452.44 µs | 2.0674 ms | 830.39 µs | 367489 | 221913 | 207244 | 2.0194 ms |
| [pot 3.0.1][pot] | 2.4099 ms | 6.2172 ms | 5.1049 ms | 599125 | 299158 | 247675 | 2.7355 ms |
| [prost 0.13.5][prost] | <span title="encode">*1.2853 ms\**</span> <span title="populate + encode">*3.0051 ms\**</span> | 3.5207 ms | † | 596811 | 305319 | 268737 | 2.9749 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*1.0476 ms\**</span> <span title="populate + encode">*3.0003 ms\**</span> | 3.7304 ms | † | 596811 | 305319 | 268737 | 3.0089 ms |
| [rkyv 0.8.10][rkyv] | 345.94 µs | <span title="unvalidated">*1.5087 ms\**</span> <span title="validated upfront with error">*1.8857 ms\**</span> | † | 603776 | 254776 | 219421 | 2.4036 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.5026 ms | 3.0503 ms | 1.7134 ms | 424533 | 245214 | 226077 | 2.2786 ms |
| [ron 0.8.1][ron] | 7.2860 ms | 17.954 ms | 16.302 ms | 1465223 | 434935 | 342907 | 5.5037 ms |
| [savefile 0.18.5][savefile] | 212.21 µs | 1.8603 ms | † | 566991 | 239362 | 231478 | 2.4387 ms |
| [serde-brief 0.1.1][serde-brief] | 1.4004 ms | 5.3899 ms | 3.8548 ms | 1276014 | 373898 | 293384 | 3.6386 ms |
| [serde_bare 0.5.0][serde_bare] | 760.79 µs | 2.3548 ms | † | 356311 | 213062 | 198403 | 1.9502 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8299 ms | 4.8945 ms | 3.5697 ms | 1109821 | 344751 | 274345 | 3.4508 ms |
| [serde_json 1.0.140][serde_json] | 3.6697 ms | 6.7330 ms | † | 1623191 | 466527 | 359157 | 5.5941 ms |
| [simd-json 0.14.3][simd-json] | 2.2045 ms | 4.6354 ms | † | 1623191 | 466527 | 359157 | 5.6356 ms |
| [speedy 0.8.7][speedy] | 261.42 µs | 1.6247 ms | 568.87 µs | 449595 | 234970 | 210192 | 2.0725 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*73.926 ns\**</span> | <span title="validated on-demand with error">*411.00 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4855 ns\**</span> <span title="validated upfront with error">*2.4479 ms\**</span> | <span title="unvalidated">*1.3580 µs\**</span> <span title="validated upfront with error">*2.4548 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2437 ns\**</span> <span title="validated upfront with error">*355.24 µs\**</span> | <span title="unvalidated">*240.20 ns\**</span> <span title="validated upfront with error">*359.18 µs\**</span> | <span title="unvalidated">*810.03 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*14.64%\**</span> <span title="prepend">*16.07%\**</span> | 40.36% | 9.72% | 66.96% | 71.47% | 73.00% | 27.65% |
| [bincode 2.0.0][bincode] | 43.38% | 67.67% | 20.84% | 89.19% | 90.81% | 88.27% | 35.40% |
| [bincode 1.3.3][bincode1] | 22.53% | 68.73% | 19.67% | 57.49% | 83.55% | 78.50% | 29.29% |
| [bitcode 0.6.5][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 24.40% | 69.27% | † | 73.37% | 85.79% | 86.75% | 35.63% |
| [capnp 0.20.6][capnp] | 30.73% | † | † | 40.76% | 59.88% | 64.84% | 20.85% |
| [cbor4ii 1.0.0][cbor4ii] | 16.74% | 26.03% | 4.84% | 29.53% | 58.29% | 66.36% | 21.13% |
| [ciborium 0.2.2][ciborium] | 3.61% | 12.28% | † | 29.53% | 58.29% | 66.35% | 21.56% |
| [databuf 0.5.0][databuf] | 42.53% | 72.00% | 21.83% | 91.97% | 94.31% | 91.75% | 37.55% |
| [dlhn 0.1.7][dlhn] | 18.21% | 48.00% | † | 89.41% | 91.09% | 88.55% | 34.33% |
| [flatbuffers 25.2.10][flatbuffers] | 4.19% | † | † | 38.82% | 58.13% | 61.94% | 21.21% |
| [minicbor 0.26.1][minicbor] | 23.54% | 37.18% | 9.08% | 76.42% | 80.42% | 79.62% | 32.07% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.69% | 30.52% | 5.69% | 72.86% | 79.60% | 78.82% | 30.65% |
| [nanoserde 0.1.37][nanoserde] | 49.50% | 64.08% | † | 57.69% | 83.75% | 78.51% | 29.48% |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 21.70% | 59.62% | † | 91.97% | 94.35% | 91.74% | 37.81% |
| [postcard 1.1.1][postcard] | 29.82% | 60.77% | 20.44% | 89.17% | 90.55% | 87.84% | 36.10% |
| [pot 3.0.1][pot] | 5.60% | 20.21% | 3.33% | 54.69% | 67.17% | 73.50% | 26.65% |
| [prost 0.13.5][prost] | <span title="encode">*10.50%\**</span> <span title="populate + encode">*4.49%\**</span> | 35.68% | † | 54.91% | 65.82% | 67.74% | 24.50% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*12.88%\**</span> <span title="populate + encode">*4.50%\**</span> | 33.68% | † | 54.91% | 65.82% | 67.74% | 24.23% |
| [rkyv 0.8.10][rkyv] | 39.00% | <span title="unvalidated">*83.27%\**</span> <span title="validated upfront with error">*66.62%\**</span> | † | 54.27% | 78.87% | 82.96% | 30.33% |
| [rmp-serde 1.3.0][rmp-serde] | 8.98% | 41.19% | 9.91% | 77.19% | 81.95% | 80.52% | 31.99% |
| [ron 0.8.1][ron] | 1.85% | 7.00% | 1.04% | 22.36% | 46.20% | 53.09% | 13.24% |
| [savefile 0.18.5][savefile] | 63.57% | 67.53% | † | 57.79% | 83.95% | 78.64% | 29.89% |
| [serde-brief 0.1.1][serde-brief] | 9.63% | 23.31% | 4.40% | 25.68% | 53.74% | 62.05% | 20.03% |
| [serde_bare 0.5.0][serde_bare] | 17.73% | 53.35% | † | 91.97% | 94.31% | 91.75% | 37.38% |
| [serde_cbor 0.11.2][serde_cbor] | 7.37% | 25.67% | 4.76% | 29.53% | 58.29% | 66.35% | 21.12% |
| [serde_json 1.0.140][serde_json] | 3.68% | 18.66% | † | 20.19% | 43.07% | 50.69% | 13.03% |
| [simd-json 0.14.3][simd-json] | 6.12% | 27.10% | † | 20.19% | 43.07% | 50.69% | 12.93% |
| [speedy 0.8.7][speedy] | 51.61% | 77.33% | 29.84% | 72.89% | 85.52% | 86.61% | 35.17% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*58.44%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.04%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.69%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.07%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*4.5808 ms\**</span> <span title="prepend">*2.5599 ms\**</span> | 8.4035 ms | 1704643 | 1294259 | 1245668 | 11.914 ms |
| [bincode 2.0.0][bincode] | 1.4593 ms | 3.6933 ms | 1406257 | 1117802 | 1062438 | 9.5873 ms |
| [bincode 1.3.3][bincode1] | 3.9391 ms | 4.1783 ms | 1854234 | 1141994 | 1048745 | 10.979 ms |
| [bitcode 0.6.5][bitcode] | 762.71 µs | 2.3284 ms | 971318 | 878034 | 850340 | 2.9004 ms |
| [borsh 1.5.5][borsh] | 2.9320 ms | 2.8607 ms | 1521989 | 1108471 | 1038528 | 9.9027 ms |
| [capnp 0.20.6][capnp] | 2.1323 ms | † | 2724288 | 1546992 | 1239111 | 14.401 ms |
| [cbor4ii 1.0.0][cbor4ii] | 3.0539 ms | 17.977 ms | 6012539 | 1695215 | 1464951 | 21.089 ms |
| [ciborium 0.2.2][ciborium] | 23.729 ms | 56.399 ms | 6012373 | 1695146 | 1465025 | 21.759 ms |
| [databuf 0.5.0][databuf] | 1.3149 ms | 3.7887 ms | 1319999 | 1062631 | 1008334 | 8.9395 ms |
| [dlhn 0.1.7][dlhn] | 4.8338 ms | 6.3859 ms | 1311281 | 1077520 | 1046095 | 8.5842 ms |
| [flatbuffers 25.2.10][flatbuffers] | 5.1680 ms | † | 2325620 | 1440289 | 1264800 | 14.026 ms |
| [minicbor 0.26.1][minicbor] | 2.5191 ms | 11.649 ms | 1777386 | 1276218 | 1252558 | 12.388 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.370 ms | 17.342 ms | 1770060 | 1277755 | 1263362 | 12.538 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3050 ms | 2.9524 ms | 1812404 | 1134820 | 1053109 | 10.380 ms |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 2.9551 ms | 3.3909 ms | 1319999 | 1064380 | 1010708 | 9.0060 ms |
| [postcard 1.1.1][postcard] | 1.9865 ms | 4.2391 ms | 1311281 | 1083900 | 1041434 | 8.7110 ms |
| [pot 3.0.1][pot] | 13.932 ms | 32.555 ms | 2604812 | 1482233 | 1298928 | 15.898 ms |
| [prost 0.13.5][prost] | <span title="encode">*5.4487 ms\**</span> <span title="populate + encode">*9.3563 ms\**</span> | 8.5889 ms | 1859886 | 1338076 | 1295351 | 12.063 ms |
| [protobuf 3.7.1][protobuf] | <span title="encode">*5.5339 ms\**</span> <span title="populate + encode">*12.717 ms\**</span> | 12.045 ms | 1859886 | 1338076 | 1295351 | 12.103 ms |
| [rkyv 0.8.10][rkyv] | 939.26 µs | <span title="unvalidated">*2.1681 ms\**</span> <span title="validated upfront with error">*2.5887 ms\**</span> | 2075936 | 1383779 | 1210377 | 12.937 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.573 ms | 11.039 ms | 1745322 | 1261627 | 1228923 | 11.551 ms |
| [ron 0.8.1][ron] | 36.874 ms | 90.378 ms | 8677703 | 2233642 | 1826180 | 34.325 ms |
| [savefile 0.18.5][savefile] | 854.28 µs | 2.7374 ms | 1791505 | 1128012 | 1051153 | 10.107 ms |
| [serde-brief 0.1.1][serde-brief] | 6.8749 ms | 21.647 ms | 6951772 | 1796265 | 1567819 | 23.719 ms |
| [serde_bare 0.5.0][serde_bare] | 5.0736 ms | 4.7813 ms | 1319999 | 1062645 | 1008349 | 9.0705 ms |
| [serde_cbor 0.11.2][serde_cbor] | 10.473 ms | 22.062 ms | 6012373 | 1695146 | 1465025 | 21.148 ms |
| [serde_json 1.0.140][serde_json] | 20.468 ms | 30.970 ms | 9390461 | 2391679 | 1842767 | 35.007 ms |
| [simd-json 0.14.3][simd-json] | 11.662 ms | 25.816 ms | 9390461 | 2391679 | 1842767 | 34.753 ms |
| [speedy 0.8.7][speedy] | 766.27 µs | 2.4250 ms | 1584734 | 1119837 | 1037992 | 10.023 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*74.659 ns\**</span> | <span title="validated on-demand with error">*711.39 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4870 ns\**</span> <span title="validated upfront with error">*5.8721 ms\**</span> | <span title="unvalidated">*2.6022 µs\**</span> <span title="validated upfront with error">*5.8734 ms\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*421.91 µs\**</span> | <span title="unvalidated">*419.82 ns\**</span> <span title="validated upfront with error">*423.56 µs\**</span> | <span title="unvalidated">*235.20 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1012.3][bilrost] | <span title="encode">*16.65%\**</span> <span title="prepend">*29.79%\**</span> | 25.80% | 56.98% | 67.84% | 68.26% | 24.34% |
| [bincode 2.0.0][bincode] | 52.27% | 58.70% | 69.07% | 78.55% | 80.04% | 30.25% |
| [bincode 1.3.3][bincode1] | 19.36% | 51.89% | 52.38% | 76.89% | 81.08% | 26.42% |
| [bitcode 0.6.5][bitcode] | 100.00% | 93.12% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.5][borsh] | 26.01% | 75.79% | 63.82% | 79.21% | 81.88% | 29.29% |
| [capnp 0.20.6][capnp] | 35.77% | † | 35.65% | 56.76% | 68.63% | 20.14% |
| [cbor4ii 1.0.0][cbor4ii] | 24.97% | 12.06% | 16.15% | 51.79% | 58.05% | 13.75% |
| [ciborium 0.2.2][ciborium] | 3.21% | 3.84% | 16.16% | 51.80% | 58.04% | 13.33% |
| [databuf 0.5.0][databuf] | 58.01% | 57.23% | 73.58% | 82.63% | 84.33% | 32.44% |
| [dlhn 0.1.7][dlhn] | 15.78% | 33.95% | 74.07% | 81.49% | 81.29% | 33.79% |
| [flatbuffers 25.2.10][flatbuffers] | 14.76% | † | 41.77% | 60.96% | 67.23% | 20.68% |
| [minicbor 0.26.1][minicbor] | 30.28% | 18.61% | 54.65% | 68.80% | 67.89% | 23.41% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.60% | 12.50% | 54.87% | 68.72% | 67.31% | 23.13% |
| [nanoserde 0.1.37][nanoserde] | 58.45% | 73.44% | 53.59% | 77.37% | 80.75% | 27.94% |
| [parity-scale-codec 3.7.4][parity-scale-codec] | 25.81% | 63.94% | 73.58% | 82.49% | 84.13% | 32.21% |
| [postcard 1.1.1][postcard] | 38.39% | 51.15% | 74.07% | 81.01% | 81.65% | 33.30% |
| [pot 3.0.1][pot] | 5.47% | 6.66% | 37.29% | 59.24% | 65.46% | 18.24% |
| [prost 0.13.5][prost] | <span title="encode">*14.00%\**</span> <span title="populate + encode">*8.15%\**</span> | 25.24% | 52.22% | 65.62% | 65.65% | 24.04% |
| [protobuf 3.7.1][protobuf] | <span title="encode">*13.78%\**</span> <span title="populate + encode">*6.00%\**</span> | 18.00% | 52.22% | 65.62% | 65.65% | 23.96% |
| [rkyv 0.8.10][rkyv] | 81.20% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.75%\**</span> | 46.79% | 63.45% | 70.25% | 22.42% |
| [rmp-serde 1.3.0][rmp-serde] | 7.21% | 19.64% | 55.65% | 69.60% | 69.19% | 25.11% |
| [ron 0.8.1][ron] | 2.07% | 2.40% | 11.19% | 39.31% | 46.56% | 8.45% |
| [savefile 0.18.5][savefile] | 89.28% | 79.20% | 54.22% | 77.84% | 80.90% | 28.70% |
| [serde-brief 0.1.1][serde-brief] | 11.09% | 10.02% | 13.97% | 48.88% | 54.24% | 12.23% |
| [serde_bare 0.5.0][serde_bare] | 15.03% | 45.35% | 73.58% | 82.63% | 84.33% | 31.98% |
| [serde_cbor 0.11.2][serde_cbor] | 7.28% | 9.83% | 16.16% | 51.80% | 58.04% | 13.71% |
| [serde_json 1.0.140][serde_json] | 3.73% | 7.00% | 10.34% | 36.71% | 46.14% | 8.29% |
| [simd-json 0.14.3][simd-json] | 6.54% | 8.40% | 10.34% | 36.71% | 46.14% | 8.35% |
| [speedy 0.8.7][speedy] | 99.54% | 89.41% | 61.29% | 78.41% | 81.92% | 28.94% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.20.6][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*59.01%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.13%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1012.3
[bincode]: https://crates.io/crates/bincode/2.0.0
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.5
[borsh]: https://crates.io/crates/borsh/1.5.5
[capnp]: https://crates.io/crates/capnp/0.20.6
[cbor4ii]: https://crates.io/crates/cbor4ii/1.0.0
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.2.10
[minicbor]: https://crates.io/crates/minicbor/0.26.1
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.7.4
[postcard]: https://crates.io/crates/postcard/1.1.1
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.13.5
[protobuf]: https://crates.io/crates/protobuf/3.7.1
[rkyv]: https://crates.io/crates/rkyv/0.8.10
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.18.5
[serde-brief]: https://crates.io/crates/serde-brief/0.1.1
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.140
[simd-json]: https://crates.io/crates/simd-json/0.14.3
[speedy]: https://crates.io/crates/speedy/0.8.7


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
