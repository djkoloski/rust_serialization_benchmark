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

## Last updated: 2025-06-26 18:26:04

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.90.0-nightly (0fa4ec6cd 2025-06-25)
binary: rustc
commit-hash: 0fa4ec6cde46fa17ab07acb5531cfe0dc1349ffa
commit-date: 2025-06-25
host: x86_64-unknown-linux-gnu
release: 1.90.0-nightly
LLVM version: 20.1.7
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*449.72 µs\**</span> <span title="prepend">*412.89 µs\**</span> | 2.4965 ms | 963.56 µs | 804955 | 328941 | 284849 | 4.1556 ms |
| [bincode 2.0.1][bincode] | 326.84 µs | 2.2329 ms | 691.67 µs | 741295 | 303944 | 256422 | 3.6962 ms |
| [bincode 1.3.3][bincode1] | 550.44 µs | 2.0271 ms | 588.54 µs | 1045784 | 373127 | 311553 | 4.5136 ms |
| [bitcode 0.6.6][bitcode] | 145.74 µs | 1.4499 ms | 62.482 µs | 703710 | 288826 | 227322 | 2.5153 ms |
| [borsh 1.5.7][borsh] | 551.95 µs | 2.1188 ms | † | 885780 | 362204 | 286248 | 4.1410 ms |
| [capnp 0.21.1][capnp] | 461.82 µs | † | † | 1443216 | 513986 | 426532 | 6.3229 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 653.17 µs | 5.1765 ms | 3.3781 ms | 1407835 | 403440 | 323561 | 5.0584 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.0336 ms | 12.186 ms | † | 1407835 | 403440 | 323561 | 4.9998 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 2.1392 ms | 5.0229 ms | 3.3117 ms | 1407835 | 403440 | 323561 | 4.8870 ms |
| [databuf 0.5.0][databuf] | 257.71 µs | 2.0255 ms | 668.79 µs | 765778 | 311715 | 263914 | 3.5513 ms |
| [dlhn 0.1.7][dlhn] | 756.90 µs | 2.5758 ms | † | 724953 | 301446 | 253056 | 3.2333 ms |
| [flatbuffers 25.2.10][flatbuffers] | 1.0167 ms | † | † | 1276368 | 468539 | 388381 | 4.8437 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.8386 ms | 5.9579 ms | † | 1827461 | 470560 | 360727 | 5.4670 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.0702 ms | 4.6781 ms | † | 1827461 | 470560 | 360727 | 5.9917 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.1852 ms | 2.4852 ms | † | 764996 | 315291 | 264212 | 3.8161 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4599 ms | 3.0276 ms | 1.4067 ms | 784997 | 325384 | 277608 | 3.7846 ms |
| [minicbor 1.0.0][minicbor] | 595.51 µs | 2.9578 ms | 1.4158 ms | 817830 | 332671 | 284034 | 3.9852 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3400 ms | 4.2352 ms | 2.7439 ms | 818669 | 332556 | 284797 | 4.3177 ms |
| [nanoserde 0.2.1][nanoserde] | 261.91 µs | 2.0392 ms | † | 1045784 | 373127 | 311553 | 4.2168 ms |
| [nibblecode 0.1.0][nibblecode] | 186.74 µs | † | † | 1011487 | 474000 | 404668 | 5.6458 ms |
| [postcard 1.1.1][postcard] | 438.07 µs | 2.2758 ms | 838.35 µs | 724953 | 302399 | 252968 | 3.1928 ms |
| [pot 3.0.1][pot] | 2.4044 ms | 6.4718 ms | 5.0054 ms | 971922 | 372513 | 303636 | 4.3531 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*937.86 µs\**</span> <span title="populate + encode">*2.4435 ms\**</span> | 3.3366 ms | † | 884628 | 363130 | 314959 | 4.3803 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.1597 ms\**</span> <span title="populate + encode">*2.9844 ms\**</span> | 3.7821 ms | † | 884628 | 363130 | 314959 | 4.6498 ms |
| [rkyv 0.8.10][rkyv] | 255.51 µs | <span title="unvalidated">*1.5279 ms\**</span> <span title="validated upfront with error">*1.9456 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5751 ms |
| [ron 0.10.1][ron] | 11.485 ms | 24.103 ms | 22.190 ms | 1607459 | 449158 | 349324 | 5.5775 ms |
| [savefile 0.18.6][savefile] | 189.19 µs | 2.1184 ms | † | 1045800 | 373139 | 311562 | 4.1750 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 659.92 µs | 2.3881 ms | † | 765778 | 311743 | 263822 | 3.7999 ms |
| [serde-brief 0.1.1][serde-brief] | 1.6233 ms | 4.8822 ms | 3.1834 ms | 1584946 | 413733 | 339964 | 4.8515 ms |
| [serde_bare 0.5.0][serde_bare] | 697.27 µs | 2.1858 ms | † | 765778 | 311715 | 263914 | 3.4367 ms |
| [speedy 0.8.7][speedy] | 199.54 µs | 1.7389 ms | 366.14 µs | 885780 | 362204 | 286248 | 4.0974 ms |
| [wiring 0.2.4][wiring] | 194.43 µs | 1.9340 ms | † | 1045784 | 337930 | 275808 | 3.6151 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*75.484 ns\**</span> | <span title="validated on-demand with error">*170.09 µs\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4860 ns\**</span> <span title="validated upfront with error">*2.0373 ms\**</span> | <span title="unvalidated">*49.282 µs\**</span> <span title="validated upfront with error">*2.0950 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2445 ns\**</span> <span title="validated upfront with error">*262.05 µs\**</span> | <span title="unvalidated">*10.541 µs\**</span> <span title="validated upfront with error">*273.33 µs\**</span> | <span title="unvalidated">*8.9150 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*383.57 µs\**</span> | <span title="unvalidated">*10.634 µs\**</span> <span title="validated upfront with error">*393.75 µs\**</span> | <span title="unvalidated">*7.5486 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*32.41%\**</span> <span title="prepend">*35.30%\**</span> | 58.08% | 6.48% | 87.42% | 87.80% | 79.80% | 60.53% |
| [bincode 2.0.1][bincode] | 44.59% | 64.93% | 9.03% | 94.93% | 95.03% | 88.65% | 68.05% |
| [bincode 1.3.3][bincode1] | 26.48% | 71.53% | 10.62% | 67.29% | 77.41% | 72.96% | 55.73% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 26.40% | 68.43% | † | 79.45% | 79.74% | 79.41% | 60.74% |
| [capnp 0.21.1][capnp] | 31.56% | † | † | 48.76% | 56.19% | 53.30% | 39.78% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 22.31% | 28.01% | 1.85% | 49.99% | 71.59% | 70.26% | 49.73% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.61% | 11.90% | † | 49.99% | 71.59% | 70.26% | 50.31% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.81% | 28.87% | 1.89% | 49.99% | 71.59% | 70.26% | 51.47% |
| [databuf 0.5.0][databuf] | 56.55% | 71.58% | 9.34% | 91.89% | 92.66% | 86.13% | 70.83% |
| [dlhn 0.1.7][dlhn] | 19.25% | 56.29% | † | 97.07% | 95.81% | 89.83% | 77.79% |
| [flatbuffers 25.2.10][flatbuffers] | 14.33% | † | † | 55.13% | 61.64% | 58.53% | 51.93% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.80% | 24.34% | † | 38.51% | 61.38% | 63.02% | 46.01% |
| json:<br> [simd-json 0.15.1][simd-json] | 7.04% | 30.99% | † | 38.51% | 61.38% | 63.02% | 41.98% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 12.30% | 58.34% | † | 91.99% | 91.61% | 86.04% | 65.91% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.98% | 47.89% | 4.44% | 89.64% | 88.76% | 81.89% | 66.46% |
| [minicbor 1.0.0][minicbor] | 24.47% | 49.02% | 4.41% | 86.05% | 86.82% | 80.03% | 63.12% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.73% | 34.23% | 2.28% | 85.96% | 86.85% | 79.82% | 58.26% |
| [nanoserde 0.2.1][nanoserde] | 55.65% | 71.10% | † | 67.29% | 77.41% | 72.96% | 59.65% |
| [nibblecode 0.1.0][nibblecode] | 78.04% | † | † | 69.57% | 60.93% | 56.17% | 44.55% |
| [postcard 1.1.1][postcard] | 33.27% | 63.71% | 7.45% | 97.07% | 95.51% | 89.86% | 78.78% |
| [pot 3.0.1][pot] | 6.06% | 22.40% | 1.25% | 72.40% | 77.53% | 74.87% | 57.78% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*15.54%\**</span> <span title="populate + encode">*5.96%\**</span> | 43.45% | † | 79.55% | 79.54% | 72.18% | 57.42% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.57%\**</span> <span title="populate + encode">*4.88%\**</span> | 38.34% | † | 79.55% | 79.54% | 72.18% | 54.09% |
| [rkyv 0.8.10][rkyv] | 57.04% | <span title="unvalidated">*94.89%\**</span> <span title="validated upfront with error">*74.52%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.98% |
| [ron 0.10.1][ron] | 1.27% | 6.02% | 0.28% | 43.78% | 64.30% | 65.07% | 45.10% |
| [savefile 0.18.6][savefile] | 77.03% | 68.44% | † | 67.29% | 77.40% | 72.96% | 60.25% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.08% | 60.71% | † | 91.89% | 92.65% | 86.16% | 66.19% |
| [serde-brief 0.1.1][serde-brief] | 8.98% | 29.70% | 1.96% | 44.40% | 69.81% | 66.87% | 51.85% |
| [serde_bare 0.5.0][serde_bare] | 20.90% | 66.33% | † | 91.89% | 92.66% | 86.13% | 73.19% |
| [speedy 0.8.7][speedy] | 73.04% | 83.38% | 17.07% | 79.45% | 79.74% | 79.41% | 61.39% |
| [wiring 0.2.4][wiring] | 74.96% | 74.97% | † | 67.29% | 85.47% | 82.42% | 69.58% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.65%\**</span> | <span title="validated on-demand with error">*6.20%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.39%\**</span> <span title="validated upfront with error">*0.50%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.87%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*3.86%\**</span> | <span title="unvalidated">*84.67%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.13%\**</span> <span title="validated upfront with error">*2.68%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.2472 ms\**</span> <span title="prepend">*8.7679 ms\**</span> | 7.8007 ms | 8625005 | 6443961 | 6231572 | 72.470 ms |
| [bincode 2.0.1][bincode] | 2.4130 ms | 1.0659 ms | 6000005 | 5378497 | 5346882 | 8.5881 ms |
| [bincode 1.3.3][bincode1] | 5.1943 ms | 968.04 µs | 6000008 | 5378500 | 5346908 | 8.5077 ms |
| [bitcode 0.6.6][bitcode] | 1.4144 ms | 799.92 µs | 6000006 | 5182295 | 4921841 | 13.785 ms |
| [borsh 1.5.7][borsh] | 6.3304 ms | 4.1398 ms | 6000004 | 5378496 | 5346866 | 8.7227 ms |
| [capnp 0.21.1][capnp] | 6.1027 ms | † | 14000088 | 7130367 | 6046182 | 90.575 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 9.8838 ms | 46.965 ms | 13125016 | 7524114 | 6757437 | 89.301 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 68.279 ms | 124.89 ms | 13122324 | 7524660 | 6759128 | 89.512 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 36.238 ms | 49.223 ms | 13122324 | 7524660 | 6759128 | 90.913 ms |
| [databuf 0.5.0][databuf] | 2.4124 ms | 5.3303 ms | 6000003 | 5378495 | 5346897 | 8.6305 ms |
| [dlhn 0.1.7][dlhn] | 6.3948 ms | 7.0255 ms | 6000003 | 5378495 | 5346897 | 8.5394 ms |
| [flatbuffers 25.2.10][flatbuffers] | 960.54 µs | † | 6000024 | 5378434 | 5346878 | 8.8819 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 86.687 ms | 89.608 ms | 26192883 | 9566084 | 8584671 | 155.25 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 52.386 ms | 71.091 ms | 26192883 | 9566084 | 8584671 | 155.72 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 21.610 ms | 5.2282 ms | 7500005 | 6058442 | 6014500 | 10.494 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 18.093 ms | 15.967 ms | 8125006 | 6494876 | 6391037 | 76.103 ms |
| [minicbor 1.0.0][minicbor] | 6.0515 ms | 11.884 ms | 8125006 | 6494907 | 6390894 | 71.761 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 119.47 ms | 26.647 ms | 8125037 | 6493484 | 6386940 | 74.559 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2617 ms | 949.79 µs | 6000008 | 5378500 | 5346908 | 8.5252 ms |
| [nibblecode 0.1.0][nibblecode] | 199.95 µs | † | 6000008 | 5378500 | 5346908 | 8.7368 ms |
| [postcard 1.1.1][postcard] | 485.56 µs | 1.2258 ms | 6000003 | 5378495 | 5346897 | 8.6487 ms |
| [pot 3.0.1][pot] | 40.754 ms | 76.674 ms | 10122342 | 6814618 | 6852252 | 81.137 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*7.8161 ms\**</span> <span title="populate + encode">*8.4315 ms\**</span> | 16.375 ms | 8750000 | 6665735 | 6421877 | 75.810 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.964 ms\**</span> <span title="populate + encode">*31.594 ms\**</span> | 29.355 ms | 8750000 | 6665735 | 6421877 | 80.194 ms |
| [rkyv 0.8.10][rkyv] | 204.23 µs | <span title="unvalidated">*150.76 µs\**</span> <span title="validated upfront with error">*201.11 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.6733 ms |
| [ron 0.10.1][ron] | 167.61 ms | 537.46 ms | 22192885 | 8970395 | 8137334 | 151.24 ms |
| [savefile 0.18.6][savefile] | 202.14 µs | 200.82 µs | 6000024 | 5378519 | 5346896 | 8.7890 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1043 ms | 4.3518 ms | 6000004 | 5378496 | 5346866 | 8.4654 ms |
| [serde-brief 0.1.1][serde-brief] | 22.746 ms | 37.827 ms | 15750015 | 8024540 | 6813667 | 93.403 ms |
| [serde_bare 0.5.0][serde_bare] | 5.1954 ms | 4.8247 ms | 6000003 | 5378495 | 5346897 | 8.7138 ms |
| [speedy 0.8.7][speedy] | 198.15 µs | 201.50 µs | 6000004 | 5378496 | 5346866 | 8.5996 ms |
| [wiring 0.2.4][wiring] | 201.39 µs | 322.85 µs | 6000008 | 5378952 | 5346905 | 8.6784 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*109.32 ns\**</span> | <span title="validated on-demand with error">*2.2585 ms\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4861 ns\**</span> <span title="validated upfront with error">*47.059 ns\**</span> | <span title="unvalidated">*52.565 µs\**</span> <span title="validated upfront with error">*77.808 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*1.5545 ns\**</span> | <span title="unvalidated">*49.358 µs\**</span> <span title="validated upfront with error">*77.725 µs\**</span> | <span title="unvalidated">*100.30 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*4.9822 ns\**</span> | <span title="unvalidated">*48.611 µs\**</span> <span title="validated upfront with error">*38.921 µs\**</span> | <span title="unvalidated">*101.60 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.73%\**</span> <span title="prepend">*2.26%\**</span> | 1.93% | 69.57% | 80.42% | 78.98% | 11.68% |
| [bincode 2.0.1][bincode] | 8.21% | 14.14% | 100.00% | 96.35% | 92.05% | 98.57% |
| [bincode 1.3.3][bincode1] | 3.81% | 15.57% | 100.00% | 96.35% | 92.05% | 99.50% |
| [bitcode 0.6.6][bitcode] | 14.01% | 18.85% | 100.00% | 100.00% | 100.00% | 61.41% |
| [borsh 1.5.7][borsh] | 3.13% | 3.64% | 100.00% | 96.35% | 92.05% | 97.05% |
| [capnp 0.21.1][capnp] | 3.25% | † | 42.86% | 72.68% | 81.40% | 9.35% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 2.00% | 0.32% | 45.71% | 68.88% | 72.84% | 9.48% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.29% | 0.12% | 45.72% | 68.87% | 72.82% | 9.46% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.55% | 0.31% | 45.72% | 68.87% | 72.82% | 9.31% |
| [databuf 0.5.0][databuf] | 8.21% | 2.83% | 100.00% | 96.35% | 92.05% | 98.09% |
| [dlhn 0.1.7][dlhn] | 3.10% | 2.15% | 100.00% | 96.35% | 92.05% | 99.13% |
| [flatbuffers 25.2.10][flatbuffers] | 20.63% | † | 100.00% | 96.35% | 92.05% | 95.31% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.23% | 0.17% | 22.91% | 54.17% | 57.33% | 5.45% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.38% | 0.21% | 22.91% | 54.17% | 57.33% | 5.44% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 0.92% | 2.88% | 80.00% | 85.54% | 81.83% | 80.67% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.10% | 0.94% | 73.85% | 79.79% | 77.01% | 11.12% |
| [minicbor 1.0.0][minicbor] | 3.27% | 1.27% | 73.85% | 79.79% | 77.01% | 11.80% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.17% | 0.57% | 73.85% | 79.81% | 77.06% | 11.35% |
| [nanoserde 0.2.1][nanoserde] | 15.71% | 15.87% | 100.00% | 96.35% | 92.05% | 99.30% |
| [nibblecode 0.1.0][nibblecode] | 99.10% | † | 100.00% | 96.35% | 92.05% | 96.89% |
| [postcard 1.1.1][postcard] | 40.81% | 12.30% | 100.00% | 96.35% | 92.05% | 97.88% |
| [pot 3.0.1][pot] | 0.49% | 0.20% | 59.27% | 76.05% | 71.83% | 10.43% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*2.54%\**</span> <span title="populate + encode">*2.35%\**</span> | 0.92% | 68.57% | 77.75% | 76.64% | 11.17% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.32%\**</span> <span title="populate + encode">*0.63%\**</span> | 0.51% | 68.57% | 77.75% | 76.64% | 10.56% |
| [rkyv 0.8.10][rkyv] | 97.02% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*74.96%\**</span> | 100.00% | 96.35% | 92.05% | 97.60% |
| [ron 0.10.1][ron] | 0.12% | 0.03% | 27.04% | 57.77% | 60.48% | 5.60% |
| [savefile 0.18.6][savefile] | 98.03% | 75.07% | 100.00% | 96.35% | 92.05% | 96.32% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.88% | 3.46% | 100.00% | 96.35% | 92.05% | 100.00% |
| [serde-brief 0.1.1][serde-brief] | 0.87% | 0.40% | 38.10% | 64.58% | 72.23% | 9.06% |
| [serde_bare 0.5.0][serde_bare] | 3.81% | 3.12% | 100.00% | 96.35% | 92.05% | 97.15% |
| [speedy 0.8.7][speedy] | 100.00% | 74.82% | 100.00% | 96.35% | 92.05% | 98.44% |
| [wiring 0.2.4][wiring] | 98.39% | 46.70% | 100.00% | 96.34% | 92.05% | 97.55% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.14%\**</span> | <span title="validated on-demand with error">*1.72%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*2.64%\**</span> | <span title="unvalidated">*74.04%\**</span> <span title="validated upfront with error">*50.02%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.95%\**</span> | <span title="unvalidated">*78.85%\**</span> <span title="validated upfront with error">*50.08%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*24.95%\**</span> | <span title="unvalidated">*80.07%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*98.72%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*913.62 µs\**</span> <span title="prepend">*792.26 µs\**</span> | 3.1115 ms | 1.7530 ms | 489348 | 281173 | 249360 | 2.6843 ms |
| [bincode 2.0.1][bincode] | 295.07 µs | 1.8550 ms | 823.51 µs | 367413 | 221291 | 206242 | 2.0465 ms |
| [bincode 1.3.3][bincode1] | 589.19 µs | 1.8196 ms | 853.42 µs | 569975 | 240525 | 231884 | 2.4931 ms |
| [bitcode 0.6.6][bitcode] | 131.10 µs | 1.2560 ms | 171.42 µs | 327688 | 200947 | 182040 | 758.52 µs |
| [borsh 1.5.7][borsh] | 551.23 µs | 1.8267 ms | † | 446595 | 234236 | 209834 | 2.1259 ms |
| [capnp 0.21.1][capnp] | 454.06 µs | † | † | 803896 | 335606 | 280744 | 3.5575 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 799.84 µs | 4.8274 ms | 3.5285 ms | 1109831 | 344745 | 274333 | 3.5801 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.7370 ms | 10.302 ms | † | 1109821 | 344751 | 274345 | 3.4616 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8510 ms | 4.8165 ms | 3.5055 ms | 1109821 | 344751 | 274345 | 3.4738 ms |
| [databuf 0.5.0][databuf] | 313.85 µs | 1.7208 ms | 798.82 µs | 356311 | 213062 | 198403 | 2.0022 ms |
| [dlhn 0.1.7][dlhn] | 772.20 µs | 2.5729 ms | † | 366496 | 220600 | 205586 | 2.0389 ms |
| [flatbuffers 25.2.10][flatbuffers] | 3.2608 ms | † | † | 849472 | 347816 | 294871 | 3.4851 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6309 ms | 6.8439 ms | † | 1623191 | 466527 | 359157 | 5.7596 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2085 ms | 4.6495 ms | † | 1623191 | 466527 | 359157 | 5.7556 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 946.75 µs | 2.8271 ms | † | 391251 | 236877 | 220395 | 2.1811 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4988 ms | 3.0354 ms | 1.7189 ms | 424533 | 245214 | 226077 | 2.2460 ms |
| [minicbor 1.0.0][minicbor] | 559.37 µs | 3.3487 ms | 1.8334 ms | 428773 | 249857 | 228630 | 2.2713 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 4.9905 ms | 4.1062 ms | 2.9401 ms | 449745 | 252432 | 230965 | 2.3143 ms |
| [nanoserde 0.2.1][nanoserde] | 265.34 µs | 1.9213 ms | † | 567975 | 239930 | 231872 | 2.4498 ms |
| [nibblecode 0.1.0][nibblecode] | 181.28 µs | † | † | 603928 | 431471 | 408635 | 3.5742 ms |
| [postcard 1.1.1][postcard] | 452.56 µs | 2.0659 ms | 815.37 µs | 367489 | 221913 | 207244 | 2.0199 ms |
| [pot 3.0.1][pot] | 2.4128 ms | 6.2284 ms | 5.1395 ms | 599125 | 299158 | 247675 | 2.7860 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.2691 ms\**</span> <span title="populate + encode">*2.9840 ms\**</span> | 3.4901 ms | † | 596811 | 305319 | 268737 | 3.0526 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0428 ms\**</span> <span title="populate + encode">*3.0044 ms\**</span> | 3.8382 ms | † | 596811 | 305319 | 268737 | 3.0376 ms |
| [rkyv 0.8.10][rkyv] | 337.85 µs | <span title="unvalidated">*1.5184 ms\**</span> <span title="validated upfront with error">*1.8653 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3891 ms |
| [ron 0.10.1][ron] | 8.2516 ms | 25.169 ms | 23.654 ms | 1465223 | 434935 | 342907 | 5.5551 ms |
| [savefile 0.18.6][savefile] | 208.80 µs | 1.8385 ms | † | 566991 | 239362 | 231478 | 2.4523 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 624.17 µs | 2.0761 ms | † | 356311 | 212976 | 198423 | 1.9351 ms |
| [serde-brief 0.1.1][serde-brief] | 1.4560 ms | 5.3897 ms | 3.8425 ms | 1276014 | 373898 | 293384 | 3.6801 ms |
| [serde_bare 0.5.0][serde_bare] | 758.31 µs | 2.3749 ms | † | 356311 | 213062 | 198403 | 1.9830 ms |
| [speedy 0.8.7][speedy] | 261.27 µs | 1.6710 ms | 562.57 µs | 449595 | 234970 | 210192 | 2.1108 ms |
| [wiring 0.2.4][wiring] | 187.05 µs | 1.8010 ms | † | 566975 | 247810 | 225086 | 2.5598 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*75.477 ns\**</span> | <span title="validated on-demand with error">*416.93 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4862 ns\**</span> <span title="validated upfront with error">*2.4673 ms\**</span> | <span title="unvalidated">*1.3640 µs\**</span> <span title="validated upfront with error">*2.4661 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*268.25 µs\**</span> | <span title="unvalidated">*201.50 ns\**</span> <span title="validated upfront with error">*271.54 µs\**</span> | <span title="unvalidated">*723.69 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2437 ns\**</span> <span title="validated upfront with error">*352.66 µs\**</span> | <span title="unvalidated">*240.19 ns\**</span> <span title="validated upfront with error">*354.71 µs\**</span> | <span title="unvalidated">*764.03 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.35%\**</span> <span title="prepend">*16.55%\**</span> | 40.37% | 9.78% | 66.96% | 71.47% | 73.00% | 28.26% |
| [bincode 2.0.1][bincode] | 44.43% | 67.71% | 20.82% | 89.19% | 90.81% | 88.27% | 37.06% |
| [bincode 1.3.3][bincode1] | 22.25% | 69.03% | 20.09% | 57.49% | 83.55% | 78.50% | 30.42% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 23.78% | 68.76% | † | 73.37% | 85.79% | 86.75% | 35.68% |
| [capnp 0.21.1][capnp] | 28.87% | † | † | 40.76% | 59.88% | 64.84% | 21.32% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.39% | 26.02% | 4.86% | 29.53% | 58.29% | 66.36% | 21.19% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.51% | 12.19% | † | 29.53% | 58.29% | 66.35% | 21.91% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.08% | 26.08% | 4.89% | 29.53% | 58.29% | 66.35% | 21.84% |
| [databuf 0.5.0][databuf] | 41.77% | 72.99% | 21.46% | 91.97% | 94.31% | 91.75% | 37.88% |
| [dlhn 0.1.7][dlhn] | 16.98% | 48.82% | † | 89.41% | 91.09% | 88.55% | 37.20% |
| [flatbuffers 25.2.10][flatbuffers] | 4.02% | † | † | 38.58% | 57.77% | 61.74% | 21.76% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.61% | 18.35% | † | 20.19% | 43.07% | 50.69% | 13.17% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.94% | 27.01% | † | 20.19% | 43.07% | 50.69% | 13.18% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 13.85% | 44.43% | † | 83.75% | 84.83% | 82.60% | 34.78% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.75% | 41.38% | 9.97% | 77.19% | 81.95% | 80.52% | 33.77% |
| [minicbor 1.0.0][minicbor] | 23.44% | 37.51% | 9.35% | 76.42% | 80.42% | 79.62% | 33.40% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.63% | 30.59% | 5.83% | 72.86% | 79.60% | 78.82% | 32.78% |
| [nanoserde 0.2.1][nanoserde] | 49.41% | 65.37% | † | 57.69% | 83.75% | 78.51% | 30.96% |
| [nibblecode 0.1.0][nibblecode] | 72.32% | † | † | 54.26% | 46.57% | 44.55% | 21.22% |
| [postcard 1.1.1][postcard] | 28.97% | 60.80% | 21.02% | 89.17% | 90.55% | 87.84% | 37.55% |
| [pot 3.0.1][pot] | 5.43% | 20.17% | 3.34% | 54.69% | 67.17% | 73.50% | 27.23% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*10.33%\**</span> <span title="populate + encode">*4.39%\**</span> | 35.99% | † | 54.91% | 65.82% | 67.74% | 24.85% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.57%\**</span> <span title="populate + encode">*4.36%\**</span> | 32.72% | † | 54.91% | 65.82% | 67.74% | 24.97% |
| [rkyv 0.8.10][rkyv] | 38.80% | <span title="unvalidated">*82.72%\**</span> <span title="validated upfront with error">*67.34%\**</span> | † | 54.27% | 78.87% | 82.96% | 31.75% |
| [ron 0.10.1][ron] | 1.59% | 4.99% | 0.72% | 22.36% | 46.20% | 53.09% | 13.65% |
| [savefile 0.18.6][savefile] | 62.79% | 68.32% | † | 57.79% | 83.95% | 78.64% | 30.93% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 21.00% | 60.50% | † | 91.97% | 94.35% | 91.74% | 39.20% |
| [serde-brief 0.1.1][serde-brief] | 9.00% | 23.30% | 4.46% | 25.68% | 53.74% | 62.05% | 20.61% |
| [serde_bare 0.5.0][serde_bare] | 17.29% | 52.89% | † | 91.97% | 94.31% | 91.75% | 38.25% |
| [speedy 0.8.7][speedy] | 50.18% | 75.16% | 30.47% | 72.89% | 85.52% | 86.61% | 35.94% |
| [wiring 0.2.4][wiring] | 70.09% | 69.74% | † | 57.80% | 81.09% | 80.88% | 29.63% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.65%\**</span> | <span title="validated on-demand with error">*48.33%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*14.77%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.07%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.94%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*83.89%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*94.72%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.4631 ms\**</span> <span title="prepend">*2.5540 ms\**</span> | 8.5658 ms | 1704643 | 1294259 | 1245668 | 11.774 ms |
| [bincode 2.0.1][bincode] | 1.4168 ms | 3.6456 ms | 1406257 | 1117802 | 1062438 | 9.4834 ms |
| [bincode 1.3.3][bincode1] | 3.6012 ms | 4.1277 ms | 1854234 | 1141994 | 1048745 | 10.433 ms |
| [bitcode 0.6.6][bitcode] | 726.46 µs | 2.3355 ms | 971318 | 878034 | 850340 | 2.8302 ms |
| [borsh 1.5.7][borsh] | 2.9463 ms | 2.9789 ms | 1521989 | 1108471 | 1038528 | 9.9943 ms |
| [capnp 0.21.1][capnp] | 2.2157 ms | † | 2724288 | 1546992 | 1239111 | 14.528 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.1268 ms | 18.344 ms | 6012539 | 1695215 | 1464951 | 21.415 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.134 ms | 56.872 ms | 6012373 | 1695146 | 1465025 | 21.485 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.253 ms | 22.157 ms | 6012373 | 1695146 | 1465025 | 21.384 ms |
| [databuf 0.5.0][databuf] | 1.3221 ms | 4.0003 ms | 1319999 | 1062631 | 1008334 | 8.9322 ms |
| [dlhn 0.1.7][dlhn] | 4.9260 ms | 6.4523 ms | 1311281 | 1077520 | 1046095 | 8.6411 ms |
| [flatbuffers 25.2.10][flatbuffers] | 5.2076 ms | † | 2325620 | 1439185 | 1268060 | 13.643 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 19.860 ms | 30.998 ms | 9390461 | 2391679 | 1842767 | 34.832 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.698 ms | 26.437 ms | 9390461 | 2391679 | 1842767 | 34.967 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.4507 ms | 6.9805 ms | 1458773 | 1156055 | 1137788 | 9.9328 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.450 ms | 11.115 ms | 1745322 | 1261627 | 1228923 | 11.628 ms |
| [minicbor 1.0.0][minicbor] | 2.3818 ms | 11.387 ms | 1777386 | 1276218 | 1252558 | 12.831 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.606 ms | 17.440 ms | 1770060 | 1277755 | 1263362 | 12.899 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2836 ms | 2.8041 ms | 1812404 | 1134820 | 1053109 | 10.464 ms |
| [nibblecode 0.1.0][nibblecode] | 507.09 µs | † | 2075936 | 1558720 | 1452906 | 14.199 ms |
| [postcard 1.1.1][postcard] | 1.9454 ms | 4.2041 ms | 1311281 | 1083900 | 1041434 | 8.9439 ms |
| [pot 3.0.1][pot] | 14.203 ms | 31.448 ms | 2604812 | 1482233 | 1298928 | 15.752 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*5.4353 ms\**</span> <span title="populate + encode">*9.3472 ms\**</span> | 9.2712 ms | 1859886 | 1338076 | 1295351 | 12.288 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.4035 ms\**</span> <span title="populate + encode">*12.621 ms\**</span> | 11.942 ms | 1859886 | 1338076 | 1295351 | 12.481 ms |
| [rkyv 0.8.10][rkyv] | 994.63 µs | <span title="unvalidated">*2.1893 ms\**</span> <span title="validated upfront with error">*2.6338 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.055 ms |
| [ron 0.10.1][ron] | 42.737 ms | 156.78 ms | 8677703 | 2233642 | 1826180 | 34.825 ms |
| [savefile 0.18.6][savefile] | 864.67 µs | 2.7408 ms | 1791505 | 1128012 | 1051153 | 10.482 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.0765 ms | 3.3870 ms | 1319999 | 1064380 | 1010708 | 8.9000 ms |
| [serde-brief 0.1.1][serde-brief] | 6.8192 ms | 22.302 ms | 6951772 | 1796265 | 1567819 | 23.663 ms |
| [serde_bare 0.5.0][serde_bare] | 5.2412 ms | 4.8084 ms | 1319999 | 1062645 | 1008349 | 8.8748 ms |
| [speedy 0.8.7][speedy] | 770.81 µs | 2.4677 ms | 1584734 | 1119837 | 1037992 | 10.077 ms |
| [wiring 0.2.4][wiring] | 662.26 µs | 2.7724 ms | 1791489 | 1156963 | 1082815 | 10.570 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*72.008 ns\**</span> | <span title="validated on-demand with error">*712.29 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4865 ns\**</span> <span title="validated upfront with error">*5.4274 ms\**</span> | <span title="unvalidated">*2.6003 µs\**</span> <span title="validated upfront with error">*5.4336 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*337.01 µs\**</span> | <span title="unvalidated">*439.80 ns\**</span> <span title="validated upfront with error">*336.11 µs\**</span> | <span title="unvalidated">*344.28 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*425.73 µs\**</span> | <span title="unvalidated">*417.93 ns\**</span> <span title="validated upfront with error">*425.50 µs\**</span> | <span title="unvalidated">*235.35 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.36%\**</span> <span title="prepend">*19.85%\**</span> | 25.56% | 56.98% | 67.84% | 68.26% | 24.04% |
| [bincode 2.0.1][bincode] | 35.79% | 60.05% | 69.07% | 78.55% | 80.04% | 29.84% |
| [bincode 1.3.3][bincode1] | 14.08% | 53.04% | 52.38% | 76.89% | 81.08% | 27.13% |
| [bitcode 0.6.6][bitcode] | 69.80% | 93.74% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 17.21% | 73.49% | 63.82% | 79.21% | 81.88% | 28.32% |
| [capnp 0.21.1][capnp] | 22.89% | † | 35.65% | 56.76% | 68.63% | 19.48% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.22% | 11.93% | 16.15% | 51.79% | 58.05% | 13.22% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.19% | 3.85% | 16.16% | 51.80% | 58.04% | 13.17% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 4.95% | 9.88% | 16.16% | 51.80% | 58.04% | 13.23% |
| [databuf 0.5.0][databuf] | 38.35% | 54.73% | 73.58% | 82.63% | 84.33% | 31.69% |
| [dlhn 0.1.7][dlhn] | 10.29% | 33.93% | 74.07% | 81.49% | 81.29% | 32.75% |
| [flatbuffers 25.2.10][flatbuffers] | 9.74% | † | 41.77% | 61.01% | 67.06% | 20.74% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.55% | 7.06% | 10.34% | 36.71% | 46.14% | 8.13% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.33% | 8.28% | 10.34% | 36.71% | 46.14% | 8.09% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 20.69% | 31.36% | 66.58% | 75.95% | 74.74% | 28.49% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 4.85% | 19.70% | 55.65% | 69.60% | 69.19% | 24.34% |
| [minicbor 1.0.0][minicbor] | 21.29% | 19.23% | 54.65% | 68.80% | 67.89% | 22.06% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.71% | 12.55% | 54.87% | 68.72% | 67.31% | 21.94% |
| [nanoserde 0.2.1][nanoserde] | 39.51% | 78.07% | 53.59% | 77.37% | 80.75% | 27.05% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 56.33% | 58.53% | 19.93% |
| [postcard 1.1.1][postcard] | 26.07% | 52.08% | 74.07% | 81.01% | 81.65% | 31.64% |
| [pot 3.0.1][pot] | 3.57% | 6.96% | 37.29% | 59.24% | 65.46% | 17.97% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*9.33%\**</span> <span title="populate + encode">*5.43%\**</span> | 23.61% | 52.22% | 65.62% | 65.65% | 23.03% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*9.38%\**</span> <span title="populate + encode">*4.02%\**</span> | 18.33% | 52.22% | 65.62% | 65.65% | 22.68% |
| [rkyv 0.8.10][rkyv] | 50.98% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.12%\**</span> | 46.79% | 63.45% | 70.25% | 21.68% |
| [ron 0.10.1][ron] | 1.19% | 1.40% | 11.19% | 39.31% | 46.56% | 8.13% |
| [savefile 0.18.6][savefile] | 58.65% | 79.88% | 54.22% | 77.84% | 80.90% | 27.00% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.48% | 64.64% | 73.58% | 82.49% | 84.13% | 31.80% |
| [serde-brief 0.1.1][serde-brief] | 7.44% | 9.82% | 13.97% | 48.88% | 54.24% | 11.96% |
| [serde_bare 0.5.0][serde_bare] | 9.68% | 45.53% | 73.58% | 82.63% | 84.33% | 31.89% |
| [speedy 0.8.7][speedy] | 65.79% | 88.72% | 61.29% | 78.41% | 81.92% | 28.08% |
| [wiring 0.2.4][wiring] | 76.57% | 78.97% | 54.22% | 75.89% | 78.53% | 26.78% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.73%\**</span> | <span title="validated on-demand with error">*58.67%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.07%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*95.03%\**</span> <span title="validated upfront with error">*0.12%\**</span> | <span title="unvalidated">*68.36%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
[nibblecode]: https://crates.io/crates/nibblecode/0.1.0
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
