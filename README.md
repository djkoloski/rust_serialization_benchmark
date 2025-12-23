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

## Last updated: 2025-12-23 16:52:58

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.94.0-nightly (4f14395c3 2025-12-22)
binary: rustc
commit-hash: 4f14395c37db4c1be874e6b0ace6721674223c22
commit-date: 2025-12-22
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*431.79 µs\**</span> <span title="prepend">*424.70 µs\**</span> | 2.5271 ms | 873.06 µs | 804955 | 328941 | 284849 | 4.0881 ms |
| [bin-proto 0.12.2][bin-proto] | 4.5067 ms | 4.8684 ms | † | 1045784 | 373127 | 311553 | 4.4945 ms |
| [bincode 2.0.1][bincode] | 343.78 µs | 2.2324 ms | 680.74 µs | 741295 | 303944 | 256422 | 3.4343 ms |
| [bincode 1.3.3][bincode1] | 549.28 µs | 2.0673 ms | 607.08 µs | 1045784 | 373127 | 311553 | 4.4724 ms |
| [bitcode 0.6.6][bitcode] | 144.73 µs | 1.4312 ms | 62.659 µs | 703710 | 288826 | 227322 | 2.5126 ms |
| [borsh 1.5.7][borsh] | 540.06 µs | 2.1423 ms | † | 885780 | 362204 | 286248 | 4.1545 ms |
| [capnp 0.23.2][capnp] | 445.13 µs | † | † | 1443216 | 513986 | 426532 | 6.1308 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 643.23 µs | 5.2383 ms | 3.3801 ms | 1407835 | 403440 | 323561 | 4.6984 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.2065 ms | 11.332 ms | † | 1407835 | 403440 | 323561 | 4.6807 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9391 ms | 4.8350 ms | 3.2541 ms | 1407835 | 403440 | 323561 | 5.0047 ms |
| [databuf 0.5.0][databuf] | 276.05 µs | 2.0083 ms | 648.67 µs | 765778 | 311715 | 263914 | 3.4211 ms |
| [dlhn 0.1.7][dlhn] | 743.31 µs | 2.5356 ms | † | 724953 | 301446 | 253056 | 3.2324 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0262 ms | † | † | 1276368 | 468539 | 388381 | 4.7476 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.6403 ms | 7.6087 ms | 5.8316 ms | 1829756 | 714318 | 691541 | 8.5377 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.9412 ms | 6.0356 ms | † | 1827461 | 470560 | 360727 | 5.4816 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1606 ms | 4.8136 ms | † | 1827461 | 470560 | 360727 | 5.4828 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.0367 ms | 2.5291 ms | † | 764996 | 315291 | 264212 | 3.6833 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.3879 ms | 2.9854 ms | 1.3824 ms | 784997 | 325384 | 277608 | 3.7554 ms |
| [minicbor 1.0.0][minicbor] | 560.43 µs | 2.9847 ms | 1.3435 ms | 817830 | 332671 | 284034 | 3.9792 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3039 ms | 4.6671 ms | 3.1742 ms | 818669 | 332556 | 284797 | 3.9524 ms |
| [nanoserde 0.2.1][nanoserde] | 246.36 µs | 2.0572 ms | † | 1045784 | 373127 | 311553 | 4.1922 ms |
| [nibblecode 0.1.0][nibblecode] | 193.42 µs | † | † | 1011487 | 489170 | 421830 | 5.4020 ms |
| [postcard 1.1.1][postcard] | 427.29 µs | 2.2813 ms | 831.78 µs | 724953 | 302399 | 252968 | 3.4208 ms |
| [pot 3.0.1][pot] | 2.2595 ms | 6.4052 ms | 4.9745 ms | 971922 | 372513 | 303636 | 4.3698 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*956.62 µs\**</span> <span title="populate + encode">*2.4243 ms\**</span> | 3.3467 ms | † | 884628 | 363130 | 314959 | 4.3650 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2217 ms\**</span> <span title="populate + encode">*2.9990 ms\**</span> | 3.7925 ms | † | 884628 | 363130 | 314959 | 4.3850 ms |
| [rkyv 0.8.10][rkyv] | 249.47 µs | <span title="unvalidated">*1.5415 ms\**</span> <span title="validated upfront with error">*1.9413 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6077 ms |
| [ron 0.10.1][ron] | 11.456 ms | 23.190 ms | 21.115 ms | 1607459 | 449158 | 349324 | 5.5076 ms |
| [savefile 0.18.6][savefile] | 196.15 µs | 2.1443 ms | † | 1045800 | 373139 | 311562 | 4.2819 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 670.88 µs | 2.3768 ms | † | 765778 | 311743 | 263822 | 3.4851 ms |
| [serde-brief 0.1.1][serde-brief] | 1.5420 ms | 4.9602 ms | 2.9400 ms | 1584946 | 413733 | 339964 | 4.9917 ms |
| [serde_bare 0.5.0][serde_bare] | 692.88 µs | 2.0802 ms | † | 765778 | 311715 | 263914 | 3.4784 ms |
| [speedy 0.8.7][speedy] | 200.66 µs | 1.7436 ms | 372.08 µs | 885780 | 362204 | 286248 | 4.0176 ms |
| [wincode 0.2.4][wincode] | 174.65 µs | 1.9454 ms | 464.01 µs | 1045784 | 373127 | 311553 | 4.1652 ms |
| [wiring 0.2.4][wiring] | 200.99 µs | 1.9310 ms | † | 1045784 | 337930 | 275808 | 3.6612 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*82.800 ns\**</span> | <span title="validated on-demand with error">*173.21 µs\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4870 ns\**</span> <span title="validated upfront with error">*2.0321 ms\**</span> | <span title="unvalidated">*51.940 µs\**</span> <span title="validated upfront with error">*2.1364 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*259.10 µs\**</span> | <span title="unvalidated">*10.449 µs\**</span> <span title="validated upfront with error">*269.99 µs\**</span> | <span title="unvalidated">*8.1357 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*396.06 µs\**</span> | <span title="unvalidated">*10.615 µs\**</span> <span title="validated upfront with error">*408.41 µs\**</span> | <span title="unvalidated">*7.5423 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*33.52%\**</span> <span title="prepend">*34.08%\**</span> | 56.63% | 7.18% | 87.42% | 87.80% | 79.80% | 61.46% |
| [bin-proto 0.12.2][bin-proto] | 3.21% | 29.40% | † | 67.29% | 77.41% | 72.96% | 55.90% |
| [bincode 2.0.1][bincode] | 42.10% | 64.11% | 9.20% | 94.93% | 95.03% | 88.65% | 73.16% |
| [bincode 1.3.3][bincode1] | 26.35% | 69.23% | 10.32% | 67.29% | 77.41% | 72.96% | 56.18% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 26.80% | 66.81% | † | 79.45% | 79.74% | 79.41% | 60.48% |
| [capnp 0.23.2][capnp] | 32.51% | † | † | 48.76% | 56.19% | 53.30% | 40.98% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 22.50% | 27.32% | 1.85% | 49.99% | 71.59% | 70.26% | 53.48% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.51% | 12.63% | † | 49.99% | 71.59% | 70.26% | 53.68% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.46% | 29.60% | 1.93% | 49.99% | 71.59% | 70.26% | 50.20% |
| [databuf 0.5.0][databuf] | 52.43% | 71.26% | 9.66% | 91.89% | 92.66% | 86.13% | 73.44% |
| [dlhn 0.1.7][dlhn] | 19.47% | 56.44% | † | 97.07% | 95.81% | 89.83% | 77.73% |
| [flatbuffers 25.12.19][flatbuffers] | 14.10% | † | † | 55.13% | 61.64% | 58.53% | 52.92% |
| [flexbuffers 25.2.10][flexbuffers] | 2.18% | 18.81% | 1.07% | 38.46% | 40.43% | 32.87% | 29.43% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.67% | 23.71% | † | 38.51% | 61.38% | 63.02% | 45.84% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.70% | 29.73% | † | 38.51% | 61.38% | 63.02% | 45.83% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 13.96% | 56.59% | † | 91.99% | 91.61% | 86.04% | 68.22% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.43% | 47.94% | 4.53% | 89.64% | 88.76% | 81.89% | 66.91% |
| [minicbor 1.0.0][minicbor] | 25.82% | 47.95% | 4.66% | 86.05% | 86.82% | 80.03% | 63.14% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.73% | 30.67% | 1.97% | 85.96% | 86.85% | 79.82% | 63.57% |
| [nanoserde 0.2.1][nanoserde] | 58.75% | 69.57% | † | 67.29% | 77.41% | 72.96% | 59.94% |
| [nibblecode 0.1.0][nibblecode] | 74.83% | † | † | 69.57% | 59.04% | 53.89% | 46.51% |
| [postcard 1.1.1][postcard] | 33.87% | 62.74% | 7.53% | 97.07% | 95.51% | 89.86% | 73.45% |
| [pot 3.0.1][pot] | 6.41% | 22.34% | 1.26% | 72.40% | 77.53% | 74.87% | 57.50% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*15.13%\**</span> <span title="populate + encode">*5.97%\**</span> | 42.76% | † | 79.55% | 79.54% | 72.18% | 57.56% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.85%\**</span> <span title="populate + encode">*4.83%\**</span> | 37.74% | † | 79.55% | 79.54% | 72.18% | 57.30% |
| [rkyv 0.8.10][rkyv] | 58.01% | <span title="unvalidated">*92.84%\**</span> <span title="validated upfront with error">*73.72%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.53% |
| [ron 0.10.1][ron] | 1.26% | 6.17% | 0.30% | 43.78% | 64.30% | 65.07% | 45.62% |
| [savefile 0.18.6][savefile] | 73.79% | 66.74% | † | 67.29% | 77.40% | 72.96% | 58.68% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 21.57% | 60.22% | † | 91.89% | 92.65% | 86.16% | 72.10% |
| [serde-brief 0.1.1][serde-brief] | 9.39% | 28.85% | 2.13% | 44.40% | 69.81% | 66.87% | 50.34% |
| [serde_bare 0.5.0][serde_bare] | 20.89% | 68.80% | † | 91.89% | 92.66% | 86.13% | 72.23% |
| [speedy 0.8.7][speedy] | 72.13% | 82.08% | 16.84% | 79.45% | 79.74% | 79.41% | 62.54% |
| [wincode 0.2.4][wincode] | 82.87% | 73.57% | 13.50% | 67.29% | 77.41% | 72.96% | 60.32% |
| [wiring 0.2.4][wiring] | 72.01% | 74.12% | † | 67.29% | 85.47% | 82.42% | 68.63% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.50%\**</span> | <span title="validated on-demand with error">*6.03%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.12%\**</span> <span title="validated upfront with error">*0.49%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*3.87%\**</span> | <span title="unvalidated">*92.71%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*98.44%\**</span> <span title="validated upfront with error">*2.56%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.2803 ms\**</span> <span title="prepend">*8.9027 ms\**</span> | 8.0118 ms | 8625005 | 6443961 | 6231572 | 72.697 ms |
| [bin-proto 0.12.2][bin-proto] | 8.9332 ms | 10.092 ms | 6000008 | 5378500 | 5346908 | 8.5074 ms |
| [bincode 2.0.1][bincode] | 2.8882 ms | 987.72 µs | 6000005 | 5378497 | 5346882 | 8.6111 ms |
| [bincode 1.3.3][bincode1] | 5.1555 ms | 4.7062 ms | 6000008 | 5378500 | 5346908 | 8.5979 ms |
| [bitcode 0.6.6][bitcode] | 1.2691 ms | 803.99 µs | 6000006 | 5182295 | 4921841 | 13.146 ms |
| [borsh 1.5.7][borsh] | 6.2098 ms | 4.1351 ms | 6000004 | 5378496 | 5346866 | 8.5200 ms |
| [capnp 0.23.2][capnp] | 6.4684 ms | † | 14000088 | 7130367 | 6046182 | 80.702 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 9.0721 ms | 49.261 ms | 13125016 | 7524114 | 6757437 | 89.504 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 66.394 ms | 114.46 ms | 13122324 | 7524660 | 6759128 | 90.502 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 36.429 ms | 40.704 ms | 13122324 | 7524660 | 6759128 | 90.758 ms |
| [databuf 0.5.0][databuf] | 2.4144 ms | 5.3305 ms | 6000003 | 5378495 | 5346897 | 8.7028 ms |
| [dlhn 0.1.7][dlhn] | 6.2145 ms | 6.9084 ms | 6000003 | 5378495 | 5346897 | 8.6327 ms |
| [flatbuffers 25.12.19][flatbuffers] | 953.36 µs | † | 6000024 | 5378434 | 5346878 | 8.8519 ms |
| [flexbuffers 25.2.10][flexbuffers] | 101.83 ms | 99.764 ms | 26609424 | 11901040 | 12486322 | 151.44 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 87.304 ms | 99.959 ms | 26192883 | 9566084 | 8584671 | 154.21 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 52.404 ms | 69.904 ms | 26192883 | 9566084 | 8584671 | 153.98 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 3.5218 ms | 5.0822 ms | 7500005 | 6058442 | 6014500 | 10.511 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 15.381 ms | 17.680 ms | 8125006 | 6494876 | 6391037 | 75.823 ms |
| [minicbor 1.0.0][minicbor] | 5.1891 ms | 11.799 ms | 8125006 | 6494907 | 6390894 | 70.141 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 124.19 ms | 30.870 ms | 8125037 | 6493484 | 6386940 | 73.491 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2539 ms | 893.51 µs | 6000008 | 5378500 | 5346908 | 8.5755 ms |
| [nibblecode 0.1.0][nibblecode] | 199.19 µs | † | 6000008 | 5378500 | 5346908 | 8.5871 ms |
| [postcard 1.1.1][postcard] | 480.80 µs | 1.1548 ms | 6000003 | 5378495 | 5346897 | 8.5883 ms |
| [pot 3.0.1][pot] | 38.001 ms | 69.349 ms | 10122342 | 6814618 | 6852252 | 83.272 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*7.8190 ms\**</span> <span title="populate + encode">*8.3703 ms\**</span> | 11.334 ms | 8750000 | 6665735 | 6421877 | 71.862 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.905 ms\**</span> <span title="populate + encode">*31.284 ms\**</span> | 29.076 ms | 8750000 | 6665735 | 6421877 | 79.187 ms |
| [rkyv 0.8.10][rkyv] | 153.79 µs | <span title="unvalidated">*200.00 µs\**</span> <span title="validated upfront with error">*179.28 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.5850 ms |
| [ron 0.10.1][ron] | 174.67 ms | 500.82 ms | 22192885 | 8970395 | 8137334 | 149.84 ms |
| [savefile 0.18.6][savefile] | 148.92 µs | 149.07 µs | 6000024 | 5378519 | 5346896 | 8.7874 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1144 ms | 2.8526 ms | 6000004 | 5378496 | 5346866 | 8.4118 ms |
| [serde-brief 0.1.1][serde-brief] | 22.807 ms | 33.749 ms | 15750015 | 8024540 | 6813667 | 93.157 ms |
| [serde_bare 0.5.0][serde_bare] | 6.2497 ms | 4.1286 ms | 6000003 | 5378495 | 5346897 | 8.7938 ms |
| [speedy 0.8.7][speedy] | 198.96 µs | 148.96 µs | 6000004 | 5378496 | 5346866 | 8.5349 ms |
| [wincode 0.2.4][wincode] | 152.26 µs | 149.12 µs | 6000008 | 5378500 | 5346908 | 8.9538 ms |
| [wiring 0.2.4][wiring] | 150.90 µs | 320.83 µs | 6000008 | 5378952 | 5346905 | 8.5073 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*111.22 ns\**</span> | <span title="validated on-demand with error">*2.2163 ms\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4870 ns\**</span> <span title="validated upfront with error">*45.315 ns\**</span> | <span title="unvalidated">*77.746 µs\**</span> <span title="validated upfront with error">*77.794 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2434 ns\**</span> <span title="validated upfront with error">*1.5551 ns\**</span> | <span title="unvalidated">*38.850 µs\**</span> <span title="validated upfront with error">*38.854 µs\**</span> | <span title="unvalidated">*79.050 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2474 ns\**</span> <span title="validated upfront with error">*4.9840 ns\**</span> | <span title="unvalidated">*38.870 µs\**</span> <span title="validated upfront with error">*38.861 µs\**</span> | <span title="unvalidated">*77.185 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.05%\**</span> <span title="prepend">*1.67%\**</span> | 1.86% | 69.57% | 80.42% | 78.98% | 11.57% |
| [bin-proto 0.12.2][bin-proto] | 1.67% | 1.48% | 100.00% | 96.35% | 92.05% | 98.88% |
| [bincode 2.0.1][bincode] | 5.16% | 15.08% | 100.00% | 96.35% | 92.05% | 97.69% |
| [bincode 1.3.3][bincode1] | 2.89% | 3.17% | 100.00% | 96.35% | 92.05% | 97.84% |
| [bitcode 0.6.6][bitcode] | 11.73% | 18.53% | 100.00% | 100.00% | 100.00% | 63.99% |
| [borsh 1.5.7][borsh] | 2.40% | 3.60% | 100.00% | 96.35% | 92.05% | 98.73% |
| [capnp 0.23.2][capnp] | 2.30% | † | 42.86% | 72.68% | 81.40% | 10.42% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.64% | 0.30% | 45.71% | 68.88% | 72.84% | 9.40% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.82% | 9.29% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.41% | 0.37% | 45.72% | 68.87% | 72.82% | 9.27% |
| [databuf 0.5.0][databuf] | 6.17% | 2.79% | 100.00% | 96.35% | 92.05% | 96.66% |
| [dlhn 0.1.7][dlhn] | 2.40% | 2.16% | 100.00% | 96.35% | 92.05% | 97.44% |
| [flatbuffers 25.12.19][flatbuffers] | 15.62% | † | 100.00% | 96.35% | 92.05% | 95.03% |
| [flexbuffers 25.2.10][flexbuffers] | 0.15% | 0.15% | 22.55% | 43.54% | 39.42% | 5.55% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.15% | 22.91% | 54.17% | 57.33% | 5.45% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.28% | 0.21% | 22.91% | 54.17% | 57.33% | 5.46% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 4.23% | 2.93% | 80.00% | 85.54% | 81.83% | 80.03% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.97% | 0.84% | 73.85% | 79.79% | 77.01% | 11.09% |
| [minicbor 1.0.0][minicbor] | 2.87% | 1.26% | 73.85% | 79.79% | 77.01% | 11.99% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.48% | 73.85% | 79.81% | 77.06% | 11.45% |
| [nanoserde 0.2.1][nanoserde] | 11.88% | 16.67% | 100.00% | 96.35% | 92.05% | 98.09% |
| [nibblecode 0.1.0][nibblecode] | 74.76% | † | 100.00% | 96.35% | 92.05% | 97.96% |
| [postcard 1.1.1][postcard] | 30.97% | 12.90% | 100.00% | 96.35% | 92.05% | 97.94% |
| [pot 3.0.1][pot] | 0.39% | 0.21% | 59.27% | 76.05% | 71.83% | 10.10% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.90%\**</span> <span title="populate + encode">*1.78%\**</span> | 1.31% | 68.57% | 77.75% | 76.64% | 11.71% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.00%\**</span> <span title="populate + encode">*0.48%\**</span> | 0.51% | 68.57% | 77.75% | 76.64% | 10.62% |
| [rkyv 0.8.10][rkyv] | 96.83% | <span title="unvalidated">*74.48%\**</span> <span title="validated upfront with error">*83.09%\**</span> | 100.00% | 96.35% | 92.05% | 97.98% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 27.04% | 57.77% | 60.48% | 5.61% |
| [savefile 0.18.6][savefile] | 100.00% | 99.93% | 100.00% | 96.35% | 92.05% | 95.73% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.91% | 5.22% | 100.00% | 96.35% | 92.05% | 100.00% |
| [serde-brief 0.1.1][serde-brief] | 0.65% | 0.44% | 38.10% | 64.58% | 72.23% | 9.03% |
| [serde_bare 0.5.0][serde_bare] | 2.38% | 3.61% | 100.00% | 96.35% | 92.05% | 95.66% |
| [speedy 0.8.7][speedy] | 74.85% | 100.00% | 100.00% | 96.35% | 92.05% | 98.56% |
| [wincode 0.2.4][wincode] | 97.81% | 99.89% | 100.00% | 96.35% | 92.05% | 93.95% |
| [wiring 0.2.4][wiring] | 98.69% | 46.43% | 100.00% | 96.34% | 92.05% | 98.88% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.12%\**</span> | <span title="validated on-demand with error">*1.75%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*2.74%\**</span> | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*49.94%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.96%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.99%\**</span> | <span title="unvalidated">*97.64%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.68%\**</span> <span title="validated upfront with error">*24.95%\**</span> | <span title="unvalidated">*99.95%\**</span> <span title="validated upfront with error">*99.97%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*872.86 µs\**</span> <span title="prepend">*782.24 µs\**</span> | 3.1578 ms | 1.7214 ms | 489348 | 281173 | 249360 | 2.6095 ms |
| [bincode 2.0.1][bincode] | 319.02 µs | 1.8746 ms | 859.47 µs | 367413 | 221291 | 206242 | 2.0580 ms |
| [bincode 1.3.3][bincode1] | 629.21 µs | 1.8259 ms | 863.98 µs | 569975 | 240525 | 231884 | 2.4814 ms |
| [bitcode 0.6.6][bitcode] | 122.08 µs | 1.2520 ms | 171.15 µs | 327688 | 200947 | 182040 | 812.95 µs |
| [borsh 1.5.7][borsh] | 553.92 µs | 1.8211 ms | † | 446595 | 234236 | 209834 | 2.0613 ms |
| [capnp 0.23.2][capnp] | 450.73 µs | † | † | 803896 | 335606 | 280744 | 3.5576 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 706.50 µs | 4.7448 ms | 3.3485 ms | 1109831 | 344745 | 274333 | 3.4219 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.7664 ms | 10.650 ms | † | 1109821 | 344751 | 274345 | 3.4354 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8721 ms | 4.4961 ms | 3.3271 ms | 1109821 | 344751 | 274345 | 3.4592 ms |
| [databuf 0.5.0][databuf] | 297.09 µs | 1.7081 ms | 774.31 µs | 356311 | 213062 | 198403 | 1.9254 ms |
| [dlhn 0.1.7][dlhn] | 799.86 µs | 2.5718 ms | † | 366496 | 220600 | 205586 | 2.0248 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2781 ms | † | † | 849472 | 347816 | 294871 | 3.4950 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.7889 ms | 7.2068 ms | 5.8433 ms | 1187688 | 557642 | 553730 | 6.1178 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6708 ms | 6.9136 ms | † | 1623191 | 466527 | 359157 | 5.7022 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2066 ms | 4.5981 ms | † | 1623191 | 466527 | 359157 | 5.7351 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 769.31 µs | 2.8770 ms | † | 391251 | 236877 | 220395 | 2.2131 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4760 ms | 2.9551 ms | 1.6796 ms | 424533 | 245214 | 226077 | 2.2618 ms |
| [minicbor 1.0.0][minicbor] | 564.65 µs | 3.3169 ms | 1.8636 ms | 428773 | 249857 | 228630 | 2.2497 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1720 ms | 4.2639 ms | 3.1134 ms | 449745 | 252432 | 230965 | 2.3935 ms |
| [nanoserde 0.2.1][nanoserde] | 262.14 µs | 1.8680 ms | † | 567975 | 239930 | 231872 | 2.4796 ms |
| [nibblecode 0.1.0][nibblecode] | 176.72 µs | † | † | 603928 | 428867 | 404377 | 3.5469 ms |
| [postcard 1.1.1][postcard] | 445.17 µs | 2.0747 ms | 803.06 µs | 367489 | 221913 | 207244 | 2.0283 ms |
| [pot 3.0.1][pot] | 2.3639 ms | 6.0031 ms | 4.8013 ms | 599125 | 299158 | 247675 | 2.7300 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.2768 ms\**</span> <span title="populate + encode">*2.9833 ms\**</span> | 3.6015 ms | † | 596811 | 305319 | 268737 | 2.9976 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0703 ms\**</span> <span title="populate + encode">*3.0143 ms\**</span> | 3.7545 ms | † | 596811 | 305319 | 268737 | 3.2452 ms |
| [rkyv 0.8.10][rkyv] | 329.75 µs | <span title="unvalidated">*1.4898 ms\**</span> <span title="validated upfront with error">*1.8549 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3561 ms |
| [ron 0.10.1][ron] | 7.9459 ms | 24.039 ms | 22.498 ms | 1465223 | 434935 | 342907 | 5.6617 ms |
| [savefile 0.18.6][savefile] | 207.86 µs | 1.8442 ms | † | 566991 | 239362 | 231478 | 2.4731 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 634.03 µs | 2.0929 ms | † | 356311 | 212976 | 198423 | 1.9230 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3431 ms | 5.3271 ms | 3.6778 ms | 1276014 | 373898 | 293384 | 3.6323 ms |
| [serde_bare 0.5.0][serde_bare] | 763.41 µs | 2.2993 ms | † | 356311 | 213062 | 198403 | 1.9475 ms |
| [speedy 0.8.7][speedy] | 265.27 µs | 1.6644 ms | 563.03 µs | 449595 | 234970 | 210192 | 2.0591 ms |
| [wincode 0.2.4][wincode] | 200.13 µs | 1.6603 ms | 634.90 µs | 566975 | 239350 | 231475 | 2.4832 ms |
| [wiring 0.2.4][wiring] | 204.26 µs | 1.8398 ms | † | 566975 | 247810 | 225086 | 2.4980 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*84.373 ns\**</span> | <span title="validated on-demand with error">*422.15 ns\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4871 ns\**</span> <span title="validated upfront with error">*2.4010 ms\**</span> | <span title="unvalidated">*1.4283 µs\**</span> <span title="validated upfront with error">*2.4000 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*259.43 µs\**</span> | <span title="unvalidated">*156.03 ns\**</span> <span title="validated upfront with error">*266.11 µs\**</span> | <span title="unvalidated">*792.25 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*344.98 µs\**</span> | <span title="unvalidated">*156.15 ns\**</span> <span title="validated upfront with error">*343.47 µs\**</span> | <span title="unvalidated">*739.87 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*13.99%\**</span> <span title="prepend">*15.61%\**</span> | 39.65% | 9.94% | 66.96% | 71.47% | 73.00% | 31.15% |
| [bincode 2.0.1][bincode] | 38.27% | 66.79% | 19.91% | 89.19% | 90.81% | 88.27% | 39.50% |
| [bincode 1.3.3][bincode1] | 19.40% | 68.57% | 19.81% | 57.49% | 83.55% | 78.50% | 32.76% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 22.04% | 68.75% | † | 73.37% | 85.79% | 86.75% | 39.44% |
| [capnp 0.23.2][capnp] | 27.08% | † | † | 40.76% | 59.88% | 64.84% | 22.85% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 17.28% | 26.39% | 5.11% | 29.53% | 58.29% | 66.36% | 23.76% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.24% | 11.76% | † | 29.53% | 58.29% | 66.35% | 23.66% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.52% | 27.85% | 5.14% | 29.53% | 58.29% | 66.35% | 23.50% |
| [databuf 0.5.0][databuf] | 41.09% | 73.30% | 22.10% | 91.97% | 94.31% | 91.75% | 42.22% |
| [dlhn 0.1.7][dlhn] | 15.26% | 48.68% | † | 89.41% | 91.09% | 88.55% | 40.15% |
| [flatbuffers 25.12.19][flatbuffers] | 3.72% | † | † | 38.58% | 57.77% | 61.74% | 23.26% |
| [flexbuffers 25.2.10][flexbuffers] | 1.57% | 17.37% | 2.93% | 27.59% | 36.04% | 32.88% | 13.29% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.33% | 18.11% | † | 20.19% | 43.07% | 50.69% | 14.26% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.53% | 27.23% | † | 20.19% | 43.07% | 50.69% | 14.18% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 15.87% | 43.52% | † | 83.75% | 84.83% | 82.60% | 36.73% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.27% | 42.37% | 10.19% | 77.19% | 81.95% | 80.52% | 35.94% |
| [minicbor 1.0.0][minicbor] | 21.62% | 37.75% | 9.18% | 76.42% | 80.42% | 79.62% | 36.14% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.36% | 29.36% | 5.50% | 72.86% | 79.60% | 78.82% | 33.96% |
| [nanoserde 0.2.1][nanoserde] | 46.57% | 67.02% | † | 57.69% | 83.75% | 78.51% | 32.79% |
| [nibblecode 0.1.0][nibblecode] | 69.08% | † | † | 54.26% | 46.86% | 45.02% | 22.92% |
| [postcard 1.1.1][postcard] | 27.42% | 60.35% | 21.31% | 89.17% | 90.55% | 87.84% | 40.08% |
| [pot 3.0.1][pot] | 5.16% | 20.86% | 3.56% | 54.69% | 67.17% | 73.50% | 29.78% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.56%\**</span> <span title="populate + encode">*4.09%\**</span> | 34.76% | † | 54.91% | 65.82% | 67.74% | 27.12% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.41%\**</span> <span title="populate + encode">*4.05%\**</span> | 33.35% | † | 54.91% | 65.82% | 67.74% | 25.05% |
| [rkyv 0.8.10][rkyv] | 37.02% | <span title="unvalidated">*84.04%\**</span> <span title="validated upfront with error">*67.50%\**</span> | † | 54.27% | 78.87% | 82.96% | 34.50% |
| [ron 0.10.1][ron] | 1.54% | 5.21% | 0.76% | 22.36% | 46.20% | 53.09% | 14.36% |
| [savefile 0.18.6][savefile] | 58.73% | 67.89% | † | 57.79% | 83.95% | 78.64% | 32.87% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 19.25% | 59.82% | † | 91.97% | 94.35% | 91.74% | 42.28% |
| [serde-brief 0.1.1][serde-brief] | 9.09% | 23.50% | 4.65% | 25.68% | 53.74% | 62.05% | 22.38% |
| [serde_bare 0.5.0][serde_bare] | 15.99% | 54.45% | † | 91.97% | 94.31% | 91.75% | 41.74% |
| [speedy 0.8.7][speedy] | 46.02% | 75.22% | 30.40% | 72.89% | 85.52% | 86.61% | 39.48% |
| [wincode 0.2.4][wincode] | 61.00% | 75.41% | 26.96% | 57.80% | 83.96% | 78.64% | 32.74% |
| [wiring 0.2.4][wiring] | 59.77% | 68.05% | † | 57.80% | 81.09% | 80.88% | 32.54% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.47%\**</span> | <span title="validated on-demand with error">*36.96%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.92%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*93.39%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.92%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5180 ms\**</span> <span title="prepend">*2.6420 ms\**</span> | 8.3449 ms | 1704643 | 1294259 | 1245668 | 11.668 ms |
| [bin-proto 0.12.2][bin-proto] | 5.9362 ms | 6.9378 ms | 1791489 | 1127998 | 1051146 | 10.326 ms |
| [bincode 2.0.1][bincode] | 1.4126 ms | 3.8589 ms | 1406257 | 1117802 | 1062438 | 9.5478 ms |
| [bincode 1.3.3][bincode1] | 3.9708 ms | 4.0935 ms | 1854234 | 1141994 | 1048745 | 10.396 ms |
| [bitcode 0.6.6][bitcode] | 729.83 µs | 2.3427 ms | 971318 | 878034 | 850340 | 2.9367 ms |
| [borsh 1.5.7][borsh] | 2.9451 ms | 2.8265 ms | 1521989 | 1108471 | 1038528 | 9.8074 ms |
| [capnp 0.23.2][capnp] | 2.2360 ms | † | 2724288 | 1546992 | 1239111 | 14.405 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.0151 ms | 18.017 ms | 6012539 | 1695215 | 1464951 | 21.274 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.329 ms | 55.147 ms | 6012373 | 1695146 | 1465025 | 21.041 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.9544 ms | 20.836 ms | 6012373 | 1695146 | 1465025 | 21.191 ms |
| [databuf 0.5.0][databuf] | 1.3000 ms | 3.9057 ms | 1319999 | 1062631 | 1008334 | 9.1248 ms |
| [dlhn 0.1.7][dlhn] | 4.9159 ms | 6.4142 ms | 1311281 | 1077520 | 1046095 | 8.8184 ms |
| [flatbuffers 25.12.19][flatbuffers] | 5.1312 ms | † | 2325620 | 1439185 | 1268060 | 13.409 ms |
| [flexbuffers 25.2.10][flexbuffers] | 39.871 ms | 36.932 ms | 5352680 | 2658295 | 2777967 | 34.181 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.880 ms | 31.964 ms | 9390461 | 2391679 | 1842767 | 34.878 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 12.133 ms | 26.267 ms | 9390461 | 2391679 | 1842767 | 34.822 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.6000 ms | 6.2325 ms | 1458773 | 1156055 | 1137788 | 9.7069 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.791 ms | 10.830 ms | 1745322 | 1261627 | 1228923 | 11.632 ms |
| [minicbor 1.0.0][minicbor] | 2.5105 ms | 11.411 ms | 1777386 | 1276218 | 1252558 | 12.570 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.732 ms | 20.249 ms | 1770060 | 1277755 | 1263362 | 12.547 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2831 ms | 2.8118 ms | 1812404 | 1134820 | 1053109 | 10.403 ms |
| [nibblecode 0.1.0][nibblecode] | 497.07 µs | † | 2075936 | 1558721 | 1452906 | 13.984 ms |
| [postcard 1.1.1][postcard] | 1.8957 ms | 4.1999 ms | 1311281 | 1083900 | 1041434 | 8.6654 ms |
| [pot 3.0.1][pot] | 13.623 ms | 30.481 ms | 2604812 | 1482233 | 1298928 | 15.932 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.4248 ms\**</span> <span title="populate + encode">*9.2999 ms\**</span> | 8.6778 ms | 1859886 | 1338076 | 1295351 | 12.448 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.6510 ms\**</span> <span title="populate + encode">*12.861 ms\**</span> | 11.775 ms | 1859886 | 1338076 | 1295351 | 12.372 ms |
| [rkyv 0.8.10][rkyv] | 978.73 µs | <span title="unvalidated">*2.1727 ms\**</span> <span title="validated upfront with error">*2.5940 ms\**</span> | 2075936 | 1383779 | 1210377 | 12.965 ms |
| [ron 0.10.1][ron] | 42.925 ms | 151.81 ms | 8677703 | 2233642 | 1826180 | 34.152 ms |
| [savefile 0.18.6][savefile] | 882.97 µs | 2.7504 ms | 1791505 | 1128012 | 1051153 | 10.196 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1005 ms | 3.3151 ms | 1319999 | 1064380 | 1010708 | 8.6905 ms |
| [serde-brief 0.1.1][serde-brief] | 6.8710 ms | 21.662 ms | 6951772 | 1796265 | 1567819 | 23.321 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8323 ms | 4.8726 ms | 1319999 | 1062645 | 1008349 | 8.7614 ms |
| [speedy 0.8.7][speedy] | 743.81 µs | 2.4772 ms | 1584734 | 1119837 | 1037992 | 10.005 ms |
| [wincode 0.2.4][wincode] | 563.38 µs | 2.3891 ms | 1791489 | 1127998 | 1051146 | 10.391 ms |
| [wiring 0.2.4][wiring] | 636.74 µs | 2.7839 ms | 1791489 | 1156963 | 1082815 | 10.822 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*84.539 ns\**</span> | <span title="validated on-demand with error">*725.92 ns\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4885 ns\**</span> <span title="validated upfront with error">*5.5015 ms\**</span> | <span title="unvalidated">*2.7709 µs\**</span> <span title="validated upfront with error">*5.5047 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2438 ns\**</span> <span title="validated upfront with error">*344.10 µs\**</span> | <span title="unvalidated">*375.82 ns\**</span> <span title="validated upfront with error">*345.00 µs\**</span> | <span title="unvalidated">*237.07 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2434 ns\**</span> <span title="validated upfront with error">*415.67 µs\**</span> | <span title="unvalidated">*385.25 ns\**</span> <span title="validated upfront with error">*415.77 µs\**</span> | <span title="unvalidated">*234.86 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.00%\**</span> <span title="prepend">*18.81%\**</span> | 26.04% | 56.98% | 67.84% | 68.26% | 25.17% |
| [bin-proto 0.12.2][bin-proto] | 8.37% | 31.32% | 54.22% | 77.84% | 80.90% | 28.44% |
| [bincode 2.0.1][bincode] | 35.19% | 56.30% | 69.07% | 78.55% | 80.04% | 30.76% |
| [bincode 1.3.3][bincode1] | 12.52% | 53.08% | 52.38% | 76.89% | 81.08% | 28.25% |
| [bitcode 0.6.6][bitcode] | 68.11% | 92.74% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 16.88% | 76.87% | 63.82% | 79.21% | 81.88% | 29.94% |
| [capnp 0.23.2][capnp] | 22.23% | † | 35.65% | 56.76% | 68.63% | 20.39% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.49% | 12.06% | 16.15% | 51.79% | 58.05% | 13.80% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.13% | 3.94% | 16.16% | 51.80% | 58.04% | 13.96% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 4.99% | 10.43% | 16.16% | 51.80% | 58.04% | 13.86% |
| [databuf 0.5.0][databuf] | 38.24% | 55.63% | 73.58% | 82.63% | 84.33% | 32.18% |
| [dlhn 0.1.7][dlhn] | 10.11% | 33.87% | 74.07% | 81.49% | 81.29% | 33.30% |
| [flatbuffers 25.12.19][flatbuffers] | 9.69% | † | 41.77% | 61.01% | 67.06% | 21.90% |
| [flexbuffers 25.2.10][flexbuffers] | 1.25% | 5.88% | 18.15% | 33.03% | 30.61% | 8.59% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.38% | 6.80% | 10.34% | 36.71% | 46.14% | 8.42% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.10% | 8.27% | 10.34% | 36.71% | 46.14% | 8.43% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 31.07% | 34.86% | 66.58% | 75.95% | 74.74% | 30.25% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 4.61% | 20.06% | 55.65% | 69.60% | 69.19% | 25.25% |
| [minicbor 1.0.0][minicbor] | 19.80% | 19.04% | 54.65% | 68.80% | 67.89% | 23.36% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.67% | 10.73% | 54.87% | 68.72% | 67.31% | 23.41% |
| [nanoserde 0.2.1][nanoserde] | 38.74% | 77.27% | 53.59% | 77.37% | 80.75% | 28.23% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 56.33% | 58.53% | 21.00% |
| [postcard 1.1.1][postcard] | 26.22% | 51.73% | 74.07% | 81.01% | 81.65% | 33.89% |
| [pot 3.0.1][pot] | 3.65% | 7.13% | 37.29% | 59.24% | 65.46% | 18.43% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.16%\**</span> <span title="populate + encode">*5.34%\**</span> | 25.04% | 52.22% | 65.62% | 65.65% | 23.59% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*8.80%\**</span> <span title="populate + encode">*3.86%\**</span> | 18.45% | 52.22% | 65.62% | 65.65% | 23.74% |
| [rkyv 0.8.10][rkyv] | 50.79% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.76%\**</span> | 46.79% | 63.45% | 70.25% | 22.65% |
| [ron 0.10.1][ron] | 1.16% | 1.43% | 11.19% | 39.31% | 46.56% | 8.60% |
| [savefile 0.18.6][savefile] | 56.30% | 79.00% | 54.22% | 77.84% | 80.90% | 28.80% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.03% | 65.54% | 73.58% | 82.49% | 84.13% | 33.79% |
| [serde-brief 0.1.1][serde-brief] | 7.23% | 10.03% | 13.97% | 48.88% | 54.24% | 12.59% |
| [serde_bare 0.5.0][serde_bare] | 10.29% | 44.59% | 73.58% | 82.63% | 84.33% | 33.52% |
| [speedy 0.8.7][speedy] | 66.83% | 87.71% | 61.29% | 78.41% | 81.92% | 29.35% |
| [wincode 0.2.4][wincode] | 88.23% | 90.94% | 54.22% | 77.84% | 80.90% | 28.26% |
| [wiring 0.2.4][wiring] | 78.06% | 78.05% | 54.22% | 75.89% | 78.53% | 27.14% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.47%\**</span> | <span title="validated on-demand with error">*51.77%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.56%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*99.07%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.55%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1013.0
[bin-proto]: https://crates.io/crates/bin-proto/0.12.2
[bincode]: https://crates.io/crates/bincode/2.0.1
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.6
[borsh]: https://crates.io/crates/borsh/1.5.7
[capnp]: https://crates.io/crates/capnp/0.23.2
[cbor4ii]: https://crates.io/crates/cbor4ii/1.0.0
[ciborium]: https://crates.io/crates/ciborium/0.2.2
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
