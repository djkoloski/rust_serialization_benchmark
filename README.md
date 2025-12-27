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

## Last updated: 2025-12-27 14:04:13

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.94.0-nightly (1107bbac4 2025-12-26)
binary: rustc
commit-hash: 1107bbac4b303d49c3e67a2ec62710902bf4b341
commit-date: 2025-12-26
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*449.62 µs\**</span> <span title="prepend">*412.95 µs\**</span> | 2.5923 ms | 892.03 µs | 804955 | 328941 | 284849 | 4.3836 ms |
| [bin-proto 0.12.3][bin-proto] | 4.1925 ms | 4.8472 ms | † | 1045784 | 373127 | 311553 | 4.5868 ms |
| [bincode 2.0.1][bincode] | 346.10 µs | 2.2154 ms | 682.65 µs | 741295 | 303944 | 256422 | 3.4487 ms |
| [bincode 1.3.3][bincode1] | 554.22 µs | 2.1021 ms | 628.59 µs | 1045784 | 373127 | 311553 | 4.6086 ms |
| [bitcode 0.6.6][bitcode] | 140.20 µs | 1.4412 ms | 62.693 µs | 703710 | 288826 | 227322 | 2.5530 ms |
| [borsh 1.5.7][borsh] | 549.27 µs | 2.1565 ms | † | 885780 | 362204 | 286248 | 4.1325 ms |
| [capnp 0.23.2][capnp] | 452.74 µs | † | † | 1443216 | 513986 | 426532 | 6.1202 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 634.97 µs | 5.2768 ms | 3.5296 ms | 1407835 | 403440 | 323561 | 5.1563 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1114 ms | 11.493 ms | † | 1407835 | 403440 | 323561 | 5.0672 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9432 ms | 4.6544 ms | 3.1237 ms | 1407835 | 403440 | 323561 | 5.1883 ms |
| [databuf 0.5.0][databuf] | 259.82 µs | 2.0385 ms | 670.26 µs | 765778 | 311715 | 263914 | 3.6801 ms |
| [dlhn 0.1.7][dlhn] | 722.92 µs | 2.5804 ms | † | 724953 | 301446 | 253056 | 3.3807 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0415 ms | † | † | 1276368 | 468539 | 388381 | 4.8272 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.7903 ms | 7.4890 ms | 5.7604 ms | 1829756 | 714318 | 691541 | 8.6922 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 4.1018 ms | 5.8549 ms | † | 1827461 | 470560 | 360727 | 5.6078 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1511 ms | 4.7218 ms | † | 1827461 | 470560 | 360727 | 6.2826 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.2863 ms | 2.5773 ms | † | 764996 | 315291 | 264212 | 3.6113 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4467 ms | 3.0690 ms | 1.4156 ms | 784997 | 325384 | 277608 | 3.8044 ms |
| [minicbor 1.0.0][minicbor] | 535.73 µs | 3.0030 ms | 1.3621 ms | 817830 | 332671 | 284034 | 4.1113 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3820 ms | 4.6276 ms | 3.1472 ms | 818669 | 332556 | 284797 | 4.0634 ms |
| [nanoserde 0.2.1][nanoserde] | 243.62 µs | 2.0846 ms | † | 1045784 | 373127 | 311553 | 4.2121 ms |
| [nibblecode 0.1.0][nibblecode] | 185.93 µs | † | † | 1011487 | 472518 | 403434 | 5.7194 ms |
| [postcard 1.1.1][postcard] | 423.02 µs | 2.2531 ms | 850.48 µs | 724953 | 302399 | 252968 | 3.6314 ms |
| [pot 3.0.1][pot] | 2.3669 ms | 6.4851 ms | 4.9010 ms | 971922 | 372513 | 303636 | 4.4881 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*929.34 µs\**</span> <span title="populate + encode">*2.4183 ms\**</span> | 3.2882 ms | † | 884628 | 363130 | 314959 | 4.4277 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2006 ms\**</span> <span title="populate + encode">*2.9887 ms\**</span> | 3.7945 ms | † | 884628 | 363130 | 314959 | 4.4645 ms |
| [rkyv 0.8.10][rkyv] | 248.90 µs | <span title="unvalidated">*1.5320 ms\**</span> <span title="validated upfront with error">*1.9219 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6283 ms |
| [ron 0.10.1][ron] | 11.784 ms | 23.881 ms | 21.749 ms | 1607459 | 449158 | 349324 | 5.8252 ms |
| [savefile 0.18.6][savefile] | 189.80 µs | 2.1692 ms | † | 1045800 | 373139 | 311562 | 4.2158 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 672.65 µs | 2.3705 ms | † | 765778 | 311743 | 263822 | 3.5158 ms |
| [serde-brief 0.1.1][serde-brief] | 1.5164 ms | 4.9269 ms | 2.9102 ms | 1584946 | 413733 | 339964 | 5.2785 ms |
| [serde_bare 0.5.0][serde_bare] | 691.97 µs | 2.1054 ms | † | 765778 | 311715 | 263914 | 3.5069 ms |
| [speedy 0.8.7][speedy] | 200.31 µs | 1.7404 ms | 384.59 µs | 885780 | 362204 | 286248 | 3.9235 ms |
| [wincode 0.2.4][wincode] | 177.42 µs | 1.9623 ms | 487.25 µs | 1045784 | 373127 | 311553 | 4.2796 ms |
| [wiring 0.2.4][wiring] | 198.62 µs | 1.9338 ms | † | 1045784 | 337930 | 275808 | 3.7414 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*81.910 ns\**</span> | <span title="validated on-demand with error">*173.00 µs\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4862 ns\**</span> <span title="validated upfront with error">*2.0809 ms\**</span> | <span title="unvalidated">*51.636 µs\**</span> <span title="validated upfront with error">*2.1520 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2613 ns\**</span> <span title="validated upfront with error">*259.15 µs\**</span> | <span title="unvalidated">*10.345 µs\**</span> <span title="validated upfront with error">*271.26 µs\**</span> | <span title="unvalidated">*7.5688 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2440 ns\**</span> <span title="validated upfront with error">*377.74 µs\**</span> | <span title="unvalidated">*10.372 µs\**</span> <span title="validated upfront with error">*388.92 µs\**</span> | <span title="unvalidated">*7.6709 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*31.18%\**</span> <span title="prepend">*33.95%\**</span> | 55.60% | 7.03% | 87.42% | 87.80% | 79.80% | 58.24% |
| [bin-proto 0.12.3][bin-proto] | 3.34% | 29.73% | † | 67.29% | 77.41% | 72.96% | 55.66% |
| [bincode 2.0.1][bincode] | 40.51% | 65.05% | 9.18% | 94.93% | 95.03% | 88.65% | 74.03% |
| [bincode 1.3.3][bincode1] | 25.30% | 68.56% | 9.97% | 67.29% | 77.41% | 72.96% | 55.40% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 25.52% | 66.83% | † | 79.45% | 79.74% | 79.41% | 61.78% |
| [capnp 0.23.2][capnp] | 30.97% | † | † | 48.76% | 56.19% | 53.30% | 41.71% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 22.08% | 27.31% | 1.78% | 49.99% | 71.59% | 70.26% | 49.51% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.51% | 12.54% | † | 49.99% | 71.59% | 70.26% | 50.38% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.21% | 30.96% | 2.01% | 49.99% | 71.59% | 70.26% | 49.21% |
| [databuf 0.5.0][databuf] | 53.96% | 70.70% | 9.35% | 91.89% | 92.66% | 86.13% | 69.37% |
| [dlhn 0.1.7][dlhn] | 19.39% | 55.85% | † | 97.07% | 95.81% | 89.83% | 75.52% |
| [flatbuffers 25.12.19][flatbuffers] | 13.46% | † | † | 55.13% | 61.64% | 58.53% | 52.89% |
| [flexbuffers 25.2.10][flexbuffers] | 2.06% | 19.24% | 1.09% | 38.46% | 40.43% | 32.87% | 29.37% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.42% | 24.62% | † | 38.51% | 61.38% | 63.02% | 45.53% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.52% | 30.52% | † | 38.51% | 61.38% | 63.02% | 40.64% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 10.90% | 55.92% | † | 91.99% | 91.61% | 86.04% | 70.69% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.69% | 46.96% | 4.43% | 89.64% | 88.76% | 81.89% | 67.11% |
| [minicbor 1.0.0][minicbor] | 26.17% | 47.99% | 4.60% | 86.05% | 86.82% | 80.03% | 62.10% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.60% | 31.14% | 1.99% | 85.96% | 86.85% | 79.82% | 62.83% |
| [nanoserde 0.2.1][nanoserde] | 57.55% | 69.14% | † | 67.29% | 77.41% | 72.96% | 60.61% |
| [nibblecode 0.1.0][nibblecode] | 75.40% | † | † | 69.57% | 61.12% | 56.35% | 44.64% |
| [postcard 1.1.1][postcard] | 33.14% | 63.97% | 7.37% | 97.07% | 95.51% | 89.86% | 70.30% |
| [pot 3.0.1][pot] | 5.92% | 22.22% | 1.28% | 72.40% | 77.53% | 74.87% | 56.88% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*15.09%\**</span> <span title="populate + encode">*5.80%\**</span> | 43.83% | † | 79.55% | 79.54% | 72.18% | 57.66% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.68%\**</span> <span title="populate + encode">*4.69%\**</span> | 37.98% | † | 79.55% | 79.54% | 72.18% | 57.18% |
| [rkyv 0.8.10][rkyv] | 56.33% | <span title="unvalidated">*94.07%\**</span> <span title="validated upfront with error">*74.99%\**</span> | † | 69.57% | 73.39% | 69.74% | 55.16% |
| [ron 0.10.1][ron] | 1.19% | 6.03% | 0.29% | 43.78% | 64.30% | 65.07% | 43.83% |
| [savefile 0.18.6][savefile] | 73.87% | 66.44% | † | 67.29% | 77.40% | 72.96% | 60.56% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.84% | 60.80% | † | 91.89% | 92.65% | 86.16% | 72.62% |
| [serde-brief 0.1.1][serde-brief] | 9.25% | 29.25% | 2.15% | 44.40% | 69.81% | 66.87% | 48.37% |
| [serde_bare 0.5.0][serde_bare] | 20.26% | 68.45% | † | 91.89% | 92.66% | 86.13% | 72.80% |
| [speedy 0.8.7][speedy] | 69.99% | 82.81% | 16.30% | 79.45% | 79.74% | 79.41% | 65.07% |
| [wincode 0.2.4][wincode] | 79.02% | 73.44% | 12.87% | 67.29% | 77.41% | 72.96% | 59.66% |
| [wiring 0.2.4][wiring] | 70.59% | 74.53% | † | 67.29% | 85.47% | 82.42% | 68.24% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.52%\**</span> | <span title="validated on-demand with error">*5.98%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.04%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.03%\**</span> <span title="validated upfront with error">*0.48%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*98.63%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*3.81%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.74%\**</span> <span title="validated upfront with error">*2.66%\**</span> | <span title="unvalidated">*98.67%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.3959 ms\**</span> <span title="prepend">*8.8316 ms\**</span> | 7.9195 ms | 8625005 | 6443961 | 6231572 | 74.342 ms |
| [bin-proto 0.12.3][bin-proto] | 8.4479 ms | 10.045 ms | 6000008 | 5378500 | 5346908 | 8.7730 ms |
| [bincode 2.0.1][bincode] | 2.8907 ms | 983.58 µs | 6000005 | 5378497 | 5346882 | 8.7555 ms |
| [bincode 1.3.3][bincode1] | 4.7900 ms | 4.7354 ms | 6000008 | 5378500 | 5346908 | 8.8062 ms |
| [bitcode 0.6.6][bitcode] | 1.3136 ms | 802.21 µs | 6000006 | 5182295 | 4921841 | 14.882 ms |
| [borsh 1.5.7][borsh] | 6.3719 ms | 4.4692 ms | 6000004 | 5378496 | 5346866 | 8.9068 ms |
| [capnp 0.23.2][capnp] | 5.8182 ms | † | 14000088 | 7130367 | 6046182 | 83.130 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 9.0820 ms | 49.789 ms | 13125016 | 7524114 | 6757437 | 93.320 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 66.175 ms | 113.31 ms | 13122324 | 7524660 | 6759128 | 91.585 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 33.015 ms | 40.101 ms | 13122324 | 7524660 | 6759128 | 93.573 ms |
| [databuf 0.5.0][databuf] | 2.4130 ms | 5.4129 ms | 6000003 | 5378495 | 5346897 | 8.7753 ms |
| [dlhn 0.1.7][dlhn] | 6.0212 ms | 7.1497 ms | 6000003 | 5378495 | 5346897 | 8.8090 ms |
| [flatbuffers 25.12.19][flatbuffers] | 947.11 µs | † | 6000024 | 5378434 | 5346878 | 8.7449 ms |
| [flexbuffers 25.2.10][flexbuffers] | 102.99 ms | 89.898 ms | 26609424 | 11901040 | 12486322 | 155.00 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 88.329 ms | 101.02 ms | 26192883 | 9566084 | 8584671 | 160.28 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 52.165 ms | 71.331 ms | 26192883 | 9566084 | 8584671 | 160.87 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 19.743 ms | 5.1278 ms | 7500005 | 6058442 | 6014500 | 10.595 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 18.008 ms | 18.075 ms | 8125006 | 6494876 | 6391037 | 70.837 ms |
| [minicbor 1.0.0][minicbor] | 5.1860 ms | 11.732 ms | 8125006 | 6494907 | 6390894 | 71.950 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 119.58 ms | 30.826 ms | 8125037 | 6493484 | 6386940 | 72.071 ms |
| [nanoserde 0.2.1][nanoserde] | 1.4269 ms | 827.10 µs | 6000008 | 5378500 | 5346908 | 8.8722 ms |
| [nibblecode 0.1.0][nibblecode] | 150.43 µs | † | 6000008 | 5378500 | 5346908 | 9.1121 ms |
| [postcard 1.1.1][postcard] | 511.26 µs | 1.2889 ms | 6000003 | 5378495 | 5346897 | 8.7739 ms |
| [pot 3.0.1][pot] | 39.483 ms | 70.156 ms | 10122342 | 6814618 | 6852252 | 83.650 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*7.7636 ms\**</span> <span title="populate + encode">*8.7100 ms\**</span> | 10.922 ms | 8750000 | 6665735 | 6421877 | 74.640 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*15.772 ms\**</span> <span title="populate + encode">*32.465 ms\**</span> | 30.024 ms | 8750000 | 6665735 | 6421877 | 80.395 ms |
| [rkyv 0.8.10][rkyv] | 148.75 µs | <span title="unvalidated">*151.97 µs\**</span> <span title="validated upfront with error">*151.74 µs\**</span> | 6000008 | 5378500 | 5346872 | 9.0585 ms |
| [ron 0.10.1][ron] | 170.33 ms | 515.92 ms | 22192885 | 8970395 | 8137334 | 151.77 ms |
| [savefile 0.18.6][savefile] | 149.58 µs | 151.05 µs | 6000024 | 5378519 | 5346896 | 8.7819 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.0992 ms | 4.2052 ms | 6000004 | 5378496 | 5346866 | 8.9484 ms |
| [serde-brief 0.1.1][serde-brief] | 22.813 ms | 35.458 ms | 15750015 | 8024540 | 6813667 | 99.814 ms |
| [serde_bare 0.5.0][serde_bare] | 6.2397 ms | 4.9256 ms | 6000003 | 5378495 | 5346897 | 9.1271 ms |
| [speedy 0.8.7][speedy] | 201.31 µs | 203.75 µs | 6000004 | 5378496 | 5346866 | 8.9600 ms |
| [wincode 0.2.4][wincode] | 203.12 µs | 202.34 µs | 6000008 | 5378500 | 5346908 | 8.8823 ms |
| [wiring 0.2.4][wiring] | 203.02 µs | 325.68 µs | 6000008 | 5378952 | 5346905 | 8.8994 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*112.68 ns\**</span> | <span title="validated on-demand with error">*2.2632 ms\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4851 ns\**</span> <span title="validated upfront with error">*45.165 ns\**</span> | <span title="unvalidated">*77.747 µs\**</span> <span title="validated upfront with error">*77.730 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2433 ns\**</span> <span title="validated upfront with error">*1.5551 ns\**</span> | <span title="unvalidated">*38.852 µs\**</span> <span title="validated upfront with error">*38.972 µs\**</span> | <span title="unvalidated">*79.118 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*4.9836 ns\**</span> | <span title="unvalidated">*38.860 µs\**</span> <span title="validated upfront with error">*38.850 µs\**</span> | <span title="unvalidated">*76.818 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.01%\**</span> <span title="prepend">*1.68%\**</span> | 1.91% | 69.57% | 80.42% | 78.98% | 11.76% |
| [bin-proto 0.12.3][bin-proto] | 1.76% | 1.50% | 100.00% | 96.35% | 92.05% | 99.68% |
| [bincode 2.0.1][bincode] | 5.15% | 15.36% | 100.00% | 96.35% | 92.05% | 99.88% |
| [bincode 1.3.3][bincode1] | 3.11% | 3.19% | 100.00% | 96.35% | 92.05% | 99.30% |
| [bitcode 0.6.6][bitcode] | 11.32% | 18.83% | 100.00% | 100.00% | 100.00% | 58.76% |
| [borsh 1.5.7][borsh] | 2.33% | 3.38% | 100.00% | 96.35% | 92.05% | 98.18% |
| [capnp 0.23.2][capnp] | 2.56% | † | 42.86% | 72.68% | 81.40% | 10.52% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.64% | 0.30% | 45.71% | 68.88% | 72.84% | 9.37% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.82% | 9.55% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.45% | 0.38% | 45.72% | 68.87% | 72.82% | 9.35% |
| [databuf 0.5.0][databuf] | 6.16% | 2.79% | 100.00% | 96.35% | 92.05% | 99.65% |
| [dlhn 0.1.7][dlhn] | 2.47% | 2.11% | 100.00% | 96.35% | 92.05% | 99.27% |
| [flatbuffers 25.12.19][flatbuffers] | 15.71% | † | 100.00% | 96.35% | 92.05% | 100.00% |
| [flexbuffers 25.2.10][flexbuffers] | 0.14% | 0.17% | 22.55% | 43.54% | 39.42% | 5.64% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.15% | 22.91% | 54.17% | 57.33% | 5.46% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.29% | 0.21% | 22.91% | 54.17% | 57.33% | 5.44% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 0.75% | 2.95% | 80.00% | 85.54% | 81.83% | 82.54% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.83% | 0.84% | 73.85% | 79.79% | 77.01% | 12.35% |
| [minicbor 1.0.0][minicbor] | 2.87% | 1.29% | 73.85% | 79.79% | 77.01% | 12.15% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.49% | 73.85% | 79.81% | 77.06% | 12.13% |
| [nanoserde 0.2.1][nanoserde] | 10.42% | 18.26% | 100.00% | 96.35% | 92.05% | 98.57% |
| [nibblecode 0.1.0][nibblecode] | 98.88% | † | 100.00% | 96.35% | 92.05% | 95.97% |
| [postcard 1.1.1][postcard] | 29.09% | 11.72% | 100.00% | 96.35% | 92.05% | 99.67% |
| [pot 3.0.1][pot] | 0.38% | 0.22% | 59.27% | 76.05% | 71.83% | 10.45% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.92%\**</span> <span title="populate + encode">*1.71%\**</span> | 1.38% | 68.57% | 77.75% | 76.64% | 11.72% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*0.94%\**</span> <span title="populate + encode">*0.46%\**</span> | 0.50% | 68.57% | 77.75% | 76.64% | 10.88% |
| [rkyv 0.8.10][rkyv] | 100.00% | <span title="unvalidated">*99.39%\**</span> <span title="validated upfront with error">*99.55%\**</span> | 100.00% | 96.35% | 92.05% | 96.54% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 27.04% | 57.77% | 60.48% | 5.76% |
| [savefile 0.18.6][savefile] | 99.45% | 100.00% | 100.00% | 96.35% | 92.05% | 99.58% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.92% | 3.59% | 100.00% | 96.35% | 92.05% | 97.73% |
| [serde-brief 0.1.1][serde-brief] | 0.65% | 0.43% | 38.10% | 64.58% | 72.23% | 8.76% |
| [serde_bare 0.5.0][serde_bare] | 2.38% | 3.07% | 100.00% | 96.35% | 92.05% | 95.81% |
| [speedy 0.8.7][speedy] | 73.89% | 74.13% | 100.00% | 96.35% | 92.05% | 97.60% |
| [wincode 0.2.4][wincode] | 73.23% | 74.65% | 100.00% | 96.35% | 92.05% | 98.45% |
| [wiring 0.2.4][wiring] | 73.27% | 46.38% | 100.00% | 96.34% | 92.05% | 98.26% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.10%\**</span> | <span title="validated on-demand with error">*1.72%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*2.75%\**</span> | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*49.98%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*79.94%\**</span> | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*99.69%\**</span> | <span title="unvalidated">*97.09%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.94%\**</span> | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*890.14 µs\**</span> <span title="prepend">*778.53 µs\**</span> | 3.1457 ms | 1.7215 ms | 489348 | 281173 | 249360 | 2.6384 ms |
| [bin-proto 0.12.3][bin-proto] | 1.8187 ms | 2.8225 ms | † | 566975 | 239350 | 231475 | 2.5059 ms |
| [bincode 2.0.1][bincode] | 317.90 µs | 1.8502 ms | 829.03 µs | 367413 | 221291 | 206242 | 2.0509 ms |
| [bincode 1.3.3][bincode1] | 599.98 µs | 1.8357 ms | 862.51 µs | 569975 | 240525 | 231884 | 2.4845 ms |
| [bitcode 0.6.6][bitcode] | 128.04 µs | 1.2575 ms | 174.08 µs | 327688 | 200947 | 182040 | 773.22 µs |
| [borsh 1.5.7][borsh] | 555.87 µs | 1.8199 ms | † | 446595 | 234236 | 209834 | 2.1179 ms |
| [capnp 0.23.2][capnp] | 456.96 µs | † | † | 803896 | 335606 | 280744 | 3.5340 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 716.85 µs | 4.7121 ms | 3.3918 ms | 1109831 | 344745 | 274333 | 3.6105 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.7355 ms | 10.200 ms | † | 1109821 | 344751 | 274345 | 3.5111 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8746 ms | 4.5979 ms | 3.1997 ms | 1109821 | 344751 | 274345 | 3.4559 ms |
| [databuf 0.5.0][databuf] | 289.65 µs | 1.7141 ms | 777.90 µs | 356311 | 213062 | 198403 | 1.9755 ms |
| [dlhn 0.1.7][dlhn] | 787.28 µs | 2.5731 ms | † | 366496 | 220600 | 205586 | 2.1335 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.3384 ms | † | † | 849472 | 347816 | 294871 | 3.6458 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.7741 ms | 7.0038 ms | 5.6816 ms | 1187688 | 557642 | 553730 | 6.2931 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6809 ms | 6.9136 ms | † | 1623191 | 466527 | 359157 | 5.7733 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2145 ms | 4.6113 ms | † | 1623191 | 466527 | 359157 | 5.7230 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 962.88 µs | 2.8661 ms | † | 391251 | 236877 | 220395 | 2.2245 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.5147 ms | 2.9967 ms | 1.7302 ms | 424533 | 245214 | 226077 | 2.3284 ms |
| [minicbor 1.0.0][minicbor] | 552.33 µs | 3.3658 ms | 1.8505 ms | 428773 | 249857 | 228630 | 2.3472 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0901 ms | 4.2941 ms | 3.0899 ms | 449745 | 252432 | 230965 | 2.3880 ms |
| [nanoserde 0.2.1][nanoserde] | 263.82 µs | 1.8478 ms | † | 567975 | 239930 | 231872 | 2.4804 ms |
| [nibblecode 0.1.0][nibblecode] | 180.61 µs | † | † | 603928 | 394134 | 362175 | 3.6648 ms |
| [postcard 1.1.1][postcard] | 448.82 µs | 2.0589 ms | 825.38 µs | 367489 | 221913 | 207244 | 2.0992 ms |
| [pot 3.0.1][pot] | 2.4347 ms | 6.1374 ms | 5.0225 ms | 599125 | 299158 | 247675 | 2.7380 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.2852 ms\**</span> <span title="populate + encode">*2.9744 ms\**</span> | 3.4190 ms | † | 596811 | 305319 | 268737 | 3.0517 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0713 ms\**</span> <span title="populate + encode">*3.0509 ms\**</span> | 3.8789 ms | † | 596811 | 305319 | 268737 | 3.0020 ms |
| [rkyv 0.8.10][rkyv] | 331.10 µs | <span title="unvalidated">*1.4994 ms\**</span> <span title="validated upfront with error">*1.8398 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3297 ms |
| [ron 0.10.1][ron] | 7.9748 ms | 24.418 ms | 22.945 ms | 1465223 | 434935 | 342907 | 5.5930 ms |
| [savefile 0.18.6][savefile] | 212.46 µs | 1.8397 ms | † | 566991 | 239362 | 231478 | 2.4776 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 614.30 µs | 2.0793 ms | † | 356311 | 212976 | 198423 | 1.9465 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3018 ms | 5.3133 ms | 3.6105 ms | 1276014 | 373898 | 293384 | 3.7207 ms |
| [serde_bare 0.5.0][serde_bare] | 762.15 µs | 2.3441 ms | † | 356311 | 213062 | 198403 | 1.9713 ms |
| [speedy 0.8.7][speedy] | 267.86 µs | 1.6681 ms | 553.82 µs | 449595 | 234970 | 210192 | 2.0772 ms |
| [wincode 0.2.4][wincode] | 200.50 µs | 1.6561 ms | 631.49 µs | 566975 | 239350 | 231475 | 2.4561 ms |
| [wiring 0.2.4][wiring] | 206.48 µs | 1.8474 ms | † | 566975 | 247810 | 225086 | 2.5535 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*81.082 ns\**</span> | <span title="validated on-demand with error">*427.21 ns\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4877 ns\**</span> <span title="validated upfront with error">*2.4126 ms\**</span> | <span title="unvalidated">*1.4182 µs\**</span> <span title="validated upfront with error">*2.3847 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2432 ns\**</span> <span title="validated upfront with error">*259.23 µs\**</span> | <span title="unvalidated">*156.11 ns\**</span> <span title="validated upfront with error">*258.92 µs\**</span> | <span title="unvalidated">*728.44 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2427 ns\**</span> <span title="validated upfront with error">*328.50 µs\**</span> | <span title="unvalidated">*155.99 ns\**</span> <span title="validated upfront with error">*328.42 µs\**</span> | <span title="unvalidated">*720.68 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.38%\**</span> <span title="prepend">*16.45%\**</span> | 39.98% | 10.11% | 66.96% | 71.47% | 73.00% | 29.31% |
| [bin-proto 0.12.3][bin-proto] | 7.04% | 44.55% | † | 57.80% | 83.96% | 78.64% | 30.86% |
| [bincode 2.0.1][bincode] | 40.28% | 67.97% | 21.00% | 89.19% | 90.81% | 88.27% | 37.70% |
| [bincode 1.3.3][bincode1] | 21.34% | 68.50% | 20.18% | 57.49% | 83.55% | 78.50% | 31.12% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 23.03% | 69.10% | † | 73.37% | 85.79% | 86.75% | 36.51% |
| [capnp 0.23.2][capnp] | 28.02% | † | † | 40.76% | 59.88% | 64.84% | 21.88% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 17.86% | 26.69% | 5.13% | 29.53% | 58.29% | 66.36% | 21.42% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.43% | 12.33% | † | 29.53% | 58.29% | 66.35% | 22.02% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.83% | 27.35% | 5.44% | 29.53% | 58.29% | 66.35% | 22.37% |
| [databuf 0.5.0][databuf] | 44.21% | 73.36% | 22.38% | 91.97% | 94.31% | 91.75% | 39.14% |
| [dlhn 0.1.7][dlhn] | 16.26% | 48.87% | † | 89.41% | 91.09% | 88.55% | 36.24% |
| [flatbuffers 25.12.19][flatbuffers] | 3.84% | † | † | 38.58% | 57.77% | 61.74% | 21.21% |
| [flexbuffers 25.2.10][flexbuffers] | 1.65% | 17.95% | 3.06% | 27.59% | 36.04% | 32.88% | 12.29% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.48% | 18.19% | † | 20.19% | 43.07% | 50.69% | 13.39% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.78% | 27.27% | † | 20.19% | 43.07% | 50.69% | 13.51% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 13.30% | 43.87% | † | 83.75% | 84.83% | 82.60% | 34.76% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.45% | 41.96% | 10.06% | 77.19% | 81.95% | 80.52% | 33.21% |
| [minicbor 1.0.0][minicbor] | 23.18% | 37.36% | 9.41% | 76.42% | 80.42% | 79.62% | 32.94% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.52% | 29.28% | 5.63% | 72.86% | 79.60% | 78.82% | 32.38% |
| [nanoserde 0.2.1][nanoserde] | 48.53% | 68.05% | † | 57.69% | 83.75% | 78.51% | 31.17% |
| [nibblecode 0.1.0][nibblecode] | 70.89% | † | † | 54.26% | 50.98% | 50.26% | 21.10% |
| [postcard 1.1.1][postcard] | 28.53% | 61.08% | 21.09% | 89.17% | 90.55% | 87.84% | 36.83% |
| [pot 3.0.1][pot] | 5.26% | 20.49% | 3.47% | 54.69% | 67.17% | 73.50% | 28.24% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.96%\**</span> <span title="populate + encode">*4.30%\**</span> | 36.78% | † | 54.91% | 65.82% | 67.74% | 25.34% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.95%\**</span> <span title="populate + encode">*4.20%\**</span> | 32.42% | † | 54.91% | 65.82% | 67.74% | 25.76% |
| [rkyv 0.8.10][rkyv] | 38.67% | <span title="unvalidated">*83.87%\**</span> <span title="validated upfront with error">*68.35%\**</span> | † | 54.27% | 78.87% | 82.96% | 33.19% |
| [ron 0.10.1][ron] | 1.61% | 5.15% | 0.76% | 22.36% | 46.20% | 53.09% | 13.82% |
| [savefile 0.18.6][savefile] | 60.27% | 68.35% | † | 57.79% | 83.95% | 78.64% | 31.21% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.84% | 60.48% | † | 91.97% | 94.35% | 91.74% | 39.72% |
| [serde-brief 0.1.1][serde-brief] | 9.84% | 23.67% | 4.82% | 25.68% | 53.74% | 62.05% | 20.78% |
| [serde_bare 0.5.0][serde_bare] | 16.80% | 53.65% | † | 91.97% | 94.31% | 91.75% | 39.22% |
| [speedy 0.8.7][speedy] | 47.80% | 75.39% | 31.43% | 72.89% | 85.52% | 86.61% | 37.22% |
| [wincode 0.2.4][wincode] | 63.86% | 75.93% | 27.57% | 57.80% | 83.96% | 78.64% | 31.48% |
| [wiring 0.2.4][wiring] | 62.01% | 68.07% | † | 57.80% | 81.09% | 80.88% | 30.28% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.53%\**</span> | <span title="validated on-demand with error">*36.51%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.00%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.92%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*98.93%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5190 ms\**</span> <span title="prepend">*2.6402 ms\**</span> | 8.3449 ms | 1704643 | 1294259 | 1245668 | 12.130 ms |
| [bin-proto 0.12.3][bin-proto] | 5.1033 ms | 6.8957 ms | 1791489 | 1127998 | 1051146 | 10.474 ms |
| [bincode 2.0.1][bincode] | 1.4126 ms | 3.9485 ms | 1406257 | 1117802 | 1062438 | 9.8019 ms |
| [bincode 1.3.3][bincode1] | 3.9628 ms | 4.1798 ms | 1854234 | 1141994 | 1048745 | 10.658 ms |
| [bitcode 0.6.6][bitcode] | 731.82 µs | 2.3440 ms | 971318 | 878034 | 850340 | 2.9850 ms |
| [borsh 1.5.7][borsh] | 2.8984 ms | 2.8724 ms | 1521989 | 1108471 | 1038528 | 10.058 ms |
| [capnp 0.23.2][capnp] | 2.1869 ms | † | 2724288 | 1546992 | 1239111 | 14.756 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.0182 ms | 18.071 ms | 6012539 | 1695215 | 1464951 | 21.767 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.686 ms | 55.010 ms | 6012373 | 1695146 | 1465025 | 21.607 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.9145 ms | 21.201 ms | 6012373 | 1695146 | 1465025 | 21.888 ms |
| [databuf 0.5.0][databuf] | 1.2991 ms | 3.8063 ms | 1319999 | 1062631 | 1008334 | 8.9296 ms |
| [dlhn 0.1.7][dlhn] | 4.9799 ms | 6.3596 ms | 1311281 | 1077520 | 1046095 | 8.7904 ms |
| [flatbuffers 25.12.19][flatbuffers] | 5.3805 ms | † | 2325620 | 1439185 | 1268060 | 13.962 ms |
| [flexbuffers 25.2.10][flexbuffers] | 40.137 ms | 36.488 ms | 5352680 | 2658295 | 2777967 | 35.621 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.739 ms | 31.581 ms | 9390461 | 2391679 | 1842767 | 35.742 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.882 ms | 26.845 ms | 9390461 | 2391679 | 1842767 | 35.336 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.2434 ms | 6.2455 ms | 1458773 | 1156055 | 1137788 | 10.198 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 11.035 ms | 11.058 ms | 1745322 | 1261627 | 1228923 | 11.818 ms |
| [minicbor 1.0.0][minicbor] | 2.2080 ms | 11.110 ms | 1777386 | 1276218 | 1252558 | 12.988 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.740 ms | 20.465 ms | 1770060 | 1277755 | 1263362 | 12.896 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2848 ms | 2.7839 ms | 1812404 | 1134820 | 1053109 | 10.690 ms |
| [nibblecode 0.1.0][nibblecode] | 506.83 µs | † | 2075936 | 1518506 | 1413251 | 14.282 ms |
| [postcard 1.1.1][postcard] | 1.8630 ms | 4.2434 ms | 1311281 | 1083900 | 1041434 | 8.9348 ms |
| [pot 3.0.1][pot] | 14.107 ms | 30.906 ms | 2604812 | 1482233 | 1298928 | 16.314 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.4231 ms\**</span> <span title="populate + encode">*9.3070 ms\**</span> | 8.7846 ms | 1859886 | 1338076 | 1295351 | 12.709 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.5945 ms\**</span> <span title="populate + encode">*13.077 ms\**</span> | 12.073 ms | 1859886 | 1338076 | 1295351 | 12.567 ms |
| [rkyv 0.8.10][rkyv] | 990.00 µs | <span title="unvalidated">*2.1711 ms\**</span> <span title="validated upfront with error">*2.5695 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.283 ms |
| [ron 0.10.1][ron] | 43.822 ms | 155.11 ms | 8677703 | 2233642 | 1826180 | 35.871 ms |
| [savefile 0.18.6][savefile] | 845.04 µs | 2.7552 ms | 1791505 | 1128012 | 1051153 | 10.432 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.9741 ms | 3.3338 ms | 1319999 | 1064380 | 1010708 | 8.9718 ms |
| [serde-brief 0.1.1][serde-brief] | 6.7217 ms | 21.267 ms | 6951772 | 1796265 | 1567819 | 23.705 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8332 ms | 4.8172 ms | 1319999 | 1062645 | 1008349 | 9.2954 ms |
| [speedy 0.8.7][speedy] | 773.99 µs | 2.4766 ms | 1584734 | 1119837 | 1037992 | 10.354 ms |
| [wincode 0.2.4][wincode] | 543.60 µs | 2.3767 ms | 1791489 | 1127998 | 1051146 | 10.398 ms |
| [wiring 0.2.4][wiring] | 634.23 µs | 2.8035 ms | 1791489 | 1156963 | 1082815 | 10.886 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*81.233 ns\**</span> | <span title="validated on-demand with error">*1.0266 µs\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4858 ns\**</span> <span title="validated upfront with error">*5.6588 ms\**</span> | <span title="unvalidated">*2.7619 µs\**</span> <span title="validated upfront with error">*5.6973 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*343.59 µs\**</span> | <span title="unvalidated">*379.09 ns\**</span> <span title="validated upfront with error">*342.49 µs\**</span> | <span title="unvalidated">*237.15 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2434 ns\**</span> <span title="validated upfront with error">*399.55 µs\**</span> | <span title="unvalidated">*384.30 ns\**</span> <span title="validated upfront with error">*400.61 µs\**</span> | <span title="unvalidated">*235.24 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.22%\**</span> <span title="prepend">*19.20%\**</span> | 26.02% | 56.98% | 67.84% | 68.26% | 24.61% |
| [bin-proto 0.12.3][bin-proto] | 9.93% | 31.48% | 54.22% | 77.84% | 80.90% | 28.50% |
| [bincode 2.0.1][bincode] | 35.88% | 54.99% | 69.07% | 78.55% | 80.04% | 30.45% |
| [bincode 1.3.3][bincode1] | 12.79% | 51.94% | 52.38% | 76.89% | 81.08% | 28.01% |
| [bitcode 0.6.6][bitcode] | 69.26% | 92.62% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 17.49% | 75.58% | 63.82% | 79.21% | 81.88% | 29.68% |
| [capnp 0.23.2][capnp] | 23.18% | † | 35.65% | 56.76% | 68.63% | 20.23% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.79% | 12.01% | 16.15% | 51.79% | 58.05% | 13.71% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.14% | 3.95% | 16.16% | 51.80% | 58.04% | 13.81% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.11% | 10.24% | 16.16% | 51.80% | 58.04% | 13.64% |
| [databuf 0.5.0][databuf] | 39.01% | 57.04% | 73.58% | 82.63% | 84.33% | 33.43% |
| [dlhn 0.1.7][dlhn] | 10.18% | 34.14% | 74.07% | 81.49% | 81.29% | 33.96% |
| [flatbuffers 25.12.19][flatbuffers] | 9.42% | † | 41.77% | 61.01% | 67.06% | 21.38% |
| [flexbuffers 25.2.10][flexbuffers] | 1.26% | 5.95% | 18.15% | 33.03% | 30.61% | 8.38% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.44% | 6.87% | 10.34% | 36.71% | 46.14% | 8.35% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.27% | 8.09% | 10.34% | 36.71% | 46.14% | 8.45% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 22.59% | 34.76% | 66.58% | 75.95% | 74.74% | 29.27% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 4.59% | 19.63% | 55.65% | 69.60% | 69.19% | 25.26% |
| [minicbor 1.0.0][minicbor] | 22.95% | 19.54% | 54.65% | 68.80% | 67.89% | 22.98% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.70% | 10.61% | 54.87% | 68.72% | 67.31% | 23.15% |
| [nanoserde 0.2.1][nanoserde] | 39.45% | 77.99% | 53.59% | 77.37% | 80.75% | 27.92% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 57.82% | 60.17% | 20.90% |
| [postcard 1.1.1][postcard] | 27.21% | 51.16% | 74.07% | 81.01% | 81.65% | 33.41% |
| [pot 3.0.1][pot] | 3.59% | 7.02% | 37.29% | 59.24% | 65.46% | 18.30% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.35%\**</span> <span title="populate + encode">*5.45%\**</span> | 24.71% | 52.22% | 65.62% | 65.65% | 23.49% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*9.06%\**</span> <span title="populate + encode">*3.88%\**</span> | 17.98% | 52.22% | 65.62% | 65.65% | 23.75% |
| [rkyv 0.8.10][rkyv] | 51.19% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*84.50%\**</span> | 46.79% | 63.45% | 70.25% | 22.47% |
| [ron 0.10.1][ron] | 1.16% | 1.40% | 11.19% | 39.31% | 46.56% | 8.32% |
| [savefile 0.18.6][savefile] | 59.98% | 78.80% | 54.22% | 77.84% | 80.90% | 28.61% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 17.04% | 65.12% | 73.58% | 82.49% | 84.13% | 33.27% |
| [serde-brief 0.1.1][serde-brief] | 7.54% | 10.21% | 13.97% | 48.88% | 54.24% | 12.59% |
| [serde_bare 0.5.0][serde_bare] | 10.49% | 45.07% | 73.58% | 82.63% | 84.33% | 32.11% |
| [speedy 0.8.7][speedy] | 65.48% | 87.66% | 61.29% | 78.41% | 81.92% | 28.83% |
| [wincode 0.2.4][wincode] | 93.24% | 91.35% | 54.22% | 77.84% | 80.90% | 28.71% |
| [wiring 0.2.4][wiring] | 79.91% | 77.44% | 54.22% | 75.89% | 78.53% | 27.42% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.53%\**</span> | <span title="validated on-demand with error">*36.93%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.73%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*99.19%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*98.64%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1013.0
[bin-proto]: https://crates.io/crates/bin-proto/0.12.3
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
