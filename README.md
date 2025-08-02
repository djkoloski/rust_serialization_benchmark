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

## Last updated: 2025-08-02 22:15:44

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.90.0-nightly (4b55fe199 2025-08-01)
binary: rustc
commit-hash: 4b55fe199cfe9c710555a5af7f2a49491ad38254
commit-date: 2025-08-01
host: x86_64-unknown-linux-gnu
release: 1.90.0-nightly
LLVM version: 20.1.8
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*460.91 µs\**</span> <span title="prepend">*410.93 µs\**</span> | 2.5921 ms | 874.37 µs | 804955 | 328941 | 284849 | 4.0916 ms |
| [bincode 2.0.1][bincode] | 281.90 µs | 2.2386 ms | 649.50 µs | 741295 | 303944 | 256422 | 3.5637 ms |
| [bincode 1.3.3][bincode1] | 547.00 µs | 2.0049 ms | 612.19 µs | 1045784 | 373127 | 311553 | 4.7088 ms |
| [bitcode 0.6.6][bitcode] | 144.76 µs | 1.4502 ms | 62.714 µs | 703710 | 288826 | 227322 | 2.4840 ms |
| [borsh 1.5.7][borsh] | 552.34 µs | 2.1185 ms | † | 885780 | 362204 | 286248 | 4.0627 ms |
| [capnp 0.21.1][capnp] | 515.72 µs | † | † | 1443216 | 513986 | 426532 | 6.4731 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 620.84 µs | 4.8621 ms | 3.3605 ms | 1407835 | 403440 | 323561 | 4.9501 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.9769 ms | 12.152 ms | † | 1407835 | 403440 | 323561 | 4.9779 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 2.0619 ms | 4.8007 ms | 3.2927 ms | 1407835 | 403440 | 323561 | 4.7177 ms |
| [databuf 0.5.0][databuf] | 275.53 µs | 1.9849 ms | 663.79 µs | 765778 | 311715 | 263914 | 3.6932 ms |
| [dlhn 0.1.7][dlhn] | 702.60 µs | 2.5195 ms | † | 724953 | 301446 | 253056 | 3.1629 ms |
| [flatbuffers 25.2.10][flatbuffers] | 1.1163 ms | † | † | 1276368 | 468539 | 388381 | 4.7091 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.8356 ms | 7.2347 ms | 5.7297 ms | 1829756 | 714318 | 691541 | 8.4779 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.7433 ms | 5.9752 ms | † | 1827461 | 470560 | 360727 | 5.4251 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1218 ms | 4.7518 ms | † | 1827461 | 470560 | 360727 | 5.5851 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.2108 ms | 2.4861 ms | † | 764996 | 315291 | 264212 | 3.7844 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4868 ms | 3.1681 ms | 1.4102 ms | 784997 | 325384 | 277608 | 3.7164 ms |
| [minicbor 1.0.0][minicbor] | 764.37 µs | 2.9349 ms | 1.4503 ms | 817830 | 332671 | 284034 | 3.9282 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.4335 ms | 4.2350 ms | 2.7105 ms | 818669 | 332556 | 284797 | 4.3059 ms |
| [nanoserde 0.2.1][nanoserde] | 262.73 µs | 2.0372 ms | † | 1045784 | 373127 | 311553 | 4.1314 ms |
| [nibblecode 0.1.0][nibblecode] | 181.27 µs | † | † | 1011487 | 493681 | 427587 | 5.8934 ms |
| [postcard 1.1.1][postcard] | 420.46 µs | 2.1855 ms | 603.50 µs | 724953 | 302399 | 252968 | 3.4888 ms |
| [pot 3.0.1][pot] | 2.2598 ms | 6.5153 ms | 5.0014 ms | 971922 | 372513 | 303636 | 4.3558 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*947.63 µs\**</span> <span title="populate + encode">*2.4246 ms\**</span> | 3.3499 ms | † | 884628 | 363130 | 314959 | 4.6203 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.3164 ms\**</span> <span title="populate + encode">*3.1420 ms\**</span> | 3.8387 ms | † | 884628 | 363130 | 314959 | 4.6916 ms |
| [rkyv 0.8.10][rkyv] | 243.98 µs | <span title="unvalidated">*1.5306 ms\**</span> <span title="validated upfront with error">*1.9229 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5776 ms |
| [ron 0.10.1][ron] | 11.593 ms | 24.607 ms | 22.337 ms | 1607459 | 449158 | 349324 | 5.5046 ms |
| [savefile 0.18.6][savefile] | 193.17 µs | 2.1440 ms | † | 1045800 | 373139 | 311562 | 4.1103 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 651.38 µs | 2.3832 ms | † | 765778 | 311743 | 263822 | 3.7113 ms |
| [serde-brief 0.1.1][serde-brief] | 1.5854 ms | 4.7466 ms | 3.1934 ms | 1584946 | 413733 | 339964 | 4.8091 ms |
| [serde_bare 0.5.0][serde_bare] | 695.29 µs | 2.0891 ms | † | 765778 | 311715 | 263914 | 3.4079 ms |
| [speedy 0.8.7][speedy] | 201.08 µs | 1.7186 ms | 370.09 µs | 885780 | 362204 | 286248 | 3.8478 ms |
| [wiring 0.2.4][wiring] | 195.17 µs | 1.9392 ms | † | 1045784 | 337930 | 275808 | 3.6174 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*71.701 ns\**</span> | <span title="validated on-demand with error">*174.20 µs\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4861 ns\**</span> <span title="validated upfront with error">*2.0781 ms\**</span> | <span title="unvalidated">*49.643 µs\**</span> <span title="validated upfront with error">*2.0891 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2434 ns\**</span> <span title="validated upfront with error">*260.09 µs\**</span> | <span title="unvalidated">*10.545 µs\**</span> <span title="validated upfront with error">*271.24 µs\**</span> | <span title="unvalidated">*8.8706 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2432 ns\**</span> <span title="validated upfront with error">*384.87 µs\**</span> | <span title="unvalidated">*10.470 µs\**</span> <span title="validated upfront with error">*396.22 µs\**</span> | <span title="unvalidated">*7.3895 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*31.41%\**</span> <span title="prepend">*35.23%\**</span> | 55.95% | 7.17% | 87.42% | 87.80% | 79.80% | 60.71% |
| [bincode 2.0.1][bincode] | 51.35% | 64.78% | 9.66% | 94.93% | 95.03% | 88.65% | 69.70% |
| [bincode 1.3.3][bincode1] | 26.46% | 72.33% | 10.24% | 67.29% | 77.41% | 72.96% | 52.75% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 26.21% | 68.45% | † | 79.45% | 79.74% | 79.41% | 61.14% |
| [capnp 0.21.1][capnp] | 28.07% | † | † | 48.76% | 56.19% | 53.30% | 38.37% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 23.32% | 29.83% | 1.87% | 49.99% | 71.59% | 70.26% | 50.18% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.64% | 11.93% | † | 49.99% | 71.59% | 70.26% | 49.90% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.02% | 30.21% | 1.90% | 49.99% | 71.59% | 70.26% | 52.65% |
| [databuf 0.5.0][databuf] | 52.54% | 73.06% | 9.45% | 91.89% | 92.66% | 86.13% | 67.26% |
| [dlhn 0.1.7][dlhn] | 20.60% | 57.56% | † | 97.07% | 95.81% | 89.83% | 78.54% |
| [flatbuffers 25.2.10][flatbuffers] | 12.97% | † | † | 55.13% | 61.64% | 58.53% | 52.75% |
| [flexbuffers 25.2.10][flexbuffers] | 2.12% | 20.05% | 1.09% | 38.46% | 40.43% | 32.87% | 29.30% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.87% | 24.27% | † | 38.51% | 61.38% | 63.02% | 45.79% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.82% | 30.52% | † | 38.51% | 61.38% | 63.02% | 44.48% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 11.96% | 58.33% | † | 91.99% | 91.61% | 86.04% | 65.64% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.74% | 45.78% | 4.45% | 89.64% | 88.76% | 81.89% | 66.84% |
| [minicbor 1.0.0][minicbor] | 18.94% | 49.41% | 4.32% | 86.05% | 86.82% | 80.03% | 63.24% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.66% | 34.24% | 2.31% | 85.96% | 86.85% | 79.82% | 57.69% |
| [nanoserde 0.2.1][nanoserde] | 55.10% | 71.19% | † | 67.29% | 77.41% | 72.96% | 60.12% |
| [nibblecode 0.1.0][nibblecode] | 79.86% | † | † | 69.57% | 58.50% | 53.16% | 42.15% |
| [postcard 1.1.1][postcard] | 34.43% | 66.36% | 10.39% | 97.07% | 95.51% | 89.86% | 71.20% |
| [pot 3.0.1][pot] | 6.41% | 22.26% | 1.25% | 72.40% | 77.53% | 74.87% | 57.03% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*15.28%\**</span> <span title="populate + encode">*5.97%\**</span> | 43.29% | † | 79.55% | 79.54% | 72.18% | 53.76% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.00%\**</span> <span title="populate + encode">*4.61%\**</span> | 37.78% | † | 79.55% | 79.54% | 72.18% | 52.95% |
| [rkyv 0.8.10][rkyv] | 59.33% | <span title="unvalidated">*94.75%\**</span> <span title="validated upfront with error">*75.42%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.26% |
| [ron 0.10.1][ron] | 1.25% | 5.89% | 0.28% | 43.78% | 64.30% | 65.07% | 45.13% |
| [savefile 0.18.6][savefile] | 74.94% | 67.64% | † | 67.29% | 77.40% | 72.96% | 60.43% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.22% | 60.85% | † | 91.89% | 92.65% | 86.16% | 66.93% |
| [serde-brief 0.1.1][serde-brief] | 9.13% | 30.55% | 1.96% | 44.40% | 69.81% | 66.87% | 51.65% |
| [serde_bare 0.5.0][serde_bare] | 20.82% | 69.42% | † | 91.89% | 92.66% | 86.13% | 72.89% |
| [speedy 0.8.7][speedy] | 71.99% | 84.38% | 16.95% | 79.45% | 79.74% | 79.41% | 64.56% |
| [wiring 0.2.4][wiring] | 74.17% | 74.78% | † | 67.29% | 85.47% | 82.42% | 68.67% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.73%\**</span> | <span title="validated on-demand with error">*6.01%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.09%\**</span> <span title="validated upfront with error">*0.50%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.29%\**</span> <span title="validated upfront with error">*3.86%\**</span> | <span title="unvalidated">*83.30%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.64%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.3190 ms\**</span> <span title="prepend">*9.0579 ms\**</span> | 7.7878 ms | 8625005 | 6443961 | 6231572 | 75.073 ms |
| [bincode 2.0.1][bincode] | 2.8844 ms | 1.0617 ms | 6000005 | 5378497 | 5346882 | 8.5539 ms |
| [bincode 1.3.3][bincode1] | 5.8160 ms | 4.6774 ms | 6000008 | 5378500 | 5346908 | 8.3499 ms |
| [bitcode 0.6.6][bitcode] | 1.4137 ms | 802.91 µs | 6000006 | 5182295 | 4921841 | 13.678 ms |
| [borsh 1.5.7][borsh] | 6.2076 ms | 4.1351 ms | 6000004 | 5378496 | 5346866 | 8.3535 ms |
| [capnp 0.21.1][capnp] | 5.2573 ms | † | 14000088 | 7130367 | 6046182 | 80.513 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 9.1931 ms | 51.513 ms | 13125016 | 7524114 | 6757437 | 89.228 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 70.512 ms | 123.99 ms | 13122324 | 7524660 | 6759128 | 89.462 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 34.836 ms | 46.688 ms | 13122324 | 7524660 | 6759128 | 89.491 ms |
| [databuf 0.5.0][databuf] | 2.4091 ms | 5.3372 ms | 6000003 | 5378495 | 5346897 | 8.4688 ms |
| [dlhn 0.1.7][dlhn] | 6.0899 ms | 7.0454 ms | 6000003 | 5378495 | 5346897 | 8.4452 ms |
| [flatbuffers 25.2.10][flatbuffers] | 937.30 µs | † | 6000024 | 5378434 | 5346878 | 8.8588 ms |
| [flexbuffers 25.2.10][flexbuffers] | 107.95 ms | 79.713 ms | 26609424 | 11901040 | 12486322 | 148.49 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 87.192 ms | 90.609 ms | 26192883 | 9566084 | 8584671 | 154.69 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 51.863 ms | 71.972 ms | 26192883 | 9566084 | 8584671 | 155.73 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 21.492 ms | 5.0846 ms | 7500005 | 6058442 | 6014500 | 10.705 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 18.640 ms | 15.983 ms | 8125006 | 6494876 | 6391037 | 69.660 ms |
| [minicbor 1.0.0][minicbor] | 5.1863 ms | 12.052 ms | 8125006 | 6494907 | 6390894 | 68.636 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 119.61 ms | 26.600 ms | 8125037 | 6493484 | 6386940 | 70.915 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2630 ms | 947.08 µs | 6000008 | 5378500 | 5346908 | 8.4232 ms |
| [nibblecode 0.1.0][nibblecode] | 149.54 µs | † | 6000008 | 5378500 | 5346908 | 8.6262 ms |
| [postcard 1.1.1][postcard] | 479.87 µs | 1.1653 ms | 6000003 | 5378495 | 5346897 | 8.5982 ms |
| [pot 3.0.1][pot] | 37.769 ms | 75.325 ms | 10122342 | 6814618 | 6852252 | 82.896 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*7.7913 ms\**</span> <span title="populate + encode">*8.4390 ms\**</span> | 13.425 ms | 8750000 | 6665735 | 6421877 | 71.228 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.818 ms\**</span> <span title="populate + encode">*31.144 ms\**</span> | 29.384 ms | 8750000 | 6665735 | 6421877 | 79.703 ms |
| [rkyv 0.8.10][rkyv] | 153.43 µs | <span title="unvalidated">*155.67 µs\**</span> <span title="validated upfront with error">*154.28 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.6030 ms |
| [ron 0.10.1][ron] | 166.99 ms | 545.81 ms | 22192885 | 8970395 | 8137334 | 150.25 ms |
| [savefile 0.18.6][savefile] | 205.98 µs | 201.84 µs | 6000024 | 5378519 | 5346896 | 8.5794 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 4.9115 ms | 4.3724 ms | 6000004 | 5378496 | 5346866 | 8.7350 ms |
| [serde-brief 0.1.1][serde-brief] | 23.155 ms | 39.843 ms | 15750015 | 8024540 | 6813667 | 92.198 ms |
| [serde_bare 0.5.0][serde_bare] | 6.2478 ms | 3.7275 ms | 6000003 | 5378495 | 5346897 | 8.4442 ms |
| [speedy 0.8.7][speedy] | 199.24 µs | 199.50 µs | 6000004 | 5378496 | 5346866 | 8.3225 ms |
| [wiring 0.2.4][wiring] | 198.76 µs | 355.15 µs | 6000008 | 5378952 | 5346905 | 8.3201 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*100.55 ns\**</span> | <span title="validated on-demand with error">*2.2065 ms\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4878 ns\**</span> <span title="validated upfront with error">*44.814 ns\**</span> | <span title="unvalidated">*52.582 µs\**</span> <span title="validated upfront with error">*77.727 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2428 ns\**</span> <span title="validated upfront with error">*1.5546 ns\**</span> | <span title="unvalidated">*48.659 µs\**</span> <span title="validated upfront with error">*77.773 µs\**</span> | <span title="unvalidated">*101.80 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2434 ns\**</span> <span title="validated upfront with error">*5.3116 ns\**</span> | <span title="unvalidated">*58.383 µs\**</span> <span title="validated upfront with error">*38.861 µs\**</span> | <span title="unvalidated">*77.131 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.04%\**</span> <span title="prepend">*1.65%\**</span> | 1.98% | 69.57% | 80.42% | 78.98% | 11.08% |
| [bincode 2.0.1][bincode] | 5.18% | 14.53% | 100.00% | 96.35% | 92.05% | 97.27% |
| [bincode 1.3.3][bincode1] | 2.57% | 3.30% | 100.00% | 96.35% | 92.05% | 99.64% |
| [bitcode 0.6.6][bitcode] | 10.58% | 19.22% | 100.00% | 100.00% | 100.00% | 60.83% |
| [borsh 1.5.7][borsh] | 2.41% | 3.73% | 100.00% | 96.35% | 92.05% | 99.60% |
| [capnp 0.21.1][capnp] | 2.84% | † | 42.86% | 72.68% | 81.40% | 10.33% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.63% | 0.30% | 45.71% | 68.88% | 72.84% | 9.32% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.21% | 0.12% | 45.72% | 68.87% | 72.82% | 9.30% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.43% | 0.33% | 45.72% | 68.87% | 72.82% | 9.30% |
| [databuf 0.5.0][databuf] | 6.21% | 2.89% | 100.00% | 96.35% | 92.05% | 98.24% |
| [dlhn 0.1.7][dlhn] | 2.46% | 2.19% | 100.00% | 96.35% | 92.05% | 98.52% |
| [flatbuffers 25.2.10][flatbuffers] | 15.95% | † | 100.00% | 96.35% | 92.05% | 93.92% |
| [flexbuffers 25.2.10][flexbuffers] | 0.14% | 0.19% | 22.55% | 43.54% | 39.42% | 5.60% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.33% | 5.38% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.29% | 0.21% | 22.91% | 54.17% | 57.33% | 5.34% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 0.70% | 3.03% | 80.00% | 85.54% | 81.83% | 77.72% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.80% | 0.97% | 73.85% | 79.79% | 77.01% | 11.94% |
| [minicbor 1.0.0][minicbor] | 2.88% | 1.28% | 73.85% | 79.79% | 77.01% | 12.12% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.13% | 0.58% | 73.85% | 79.81% | 77.06% | 11.73% |
| [nanoserde 0.2.1][nanoserde] | 11.84% | 16.29% | 100.00% | 96.35% | 92.05% | 98.78% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 100.00% | 96.35% | 92.05% | 96.45% |
| [postcard 1.1.1][postcard] | 31.16% | 13.24% | 100.00% | 96.35% | 92.05% | 96.77% |
| [pot 3.0.1][pot] | 0.40% | 0.20% | 59.27% | 76.05% | 71.83% | 10.04% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.92%\**</span> <span title="populate + encode">*1.77%\**</span> | 1.15% | 68.57% | 77.75% | 76.64% | 11.68% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.01%\**</span> <span title="populate + encode">*0.48%\**</span> | 0.53% | 68.57% | 77.75% | 76.64% | 10.44% |
| [rkyv 0.8.10][rkyv] | 97.46% | <span title="unvalidated">*99.11%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.05% | 96.71% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 27.04% | 57.77% | 60.48% | 5.54% |
| [savefile 0.18.6][savefile] | 72.60% | 76.44% | 100.00% | 96.35% | 92.05% | 96.98% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.04% | 3.53% | 100.00% | 96.35% | 92.05% | 95.25% |
| [serde-brief 0.1.1][serde-brief] | 0.65% | 0.39% | 38.10% | 64.58% | 72.23% | 9.02% |
| [serde_bare 0.5.0][serde_bare] | 2.39% | 4.14% | 100.00% | 96.35% | 92.05% | 98.53% |
| [speedy 0.8.7][speedy] | 75.06% | 77.33% | 100.00% | 96.35% | 92.05% | 99.97% |
| [wiring 0.2.4][wiring] | 75.24% | 43.44% | 100.00% | 96.34% | 92.05% | 100.00% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.24%\**</span> | <span title="validated on-demand with error">*1.76%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.96%\**</span> <span title="validated upfront with error">*2.77%\**</span> | <span title="unvalidated">*73.91%\**</span> <span title="validated upfront with error">*50.00%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.94%\**</span> | <span title="unvalidated">*79.86%\**</span> <span title="validated upfront with error">*49.97%\**</span> | <span title="unvalidated">*75.77%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.95%\**</span> <span title="validated upfront with error">*23.40%\**</span> | <span title="unvalidated">*66.56%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*925.71 µs\**</span> <span title="prepend">*788.41 µs\**</span> | 3.1477 ms | 1.7425 ms | 489348 | 281173 | 249360 | 2.6142 ms |
| [bincode 2.0.1][bincode] | 269.21 µs | 1.8569 ms | 818.82 µs | 367413 | 221291 | 206242 | 2.0305 ms |
| [bincode 1.3.3][bincode1] | 621.09 µs | 1.8062 ms | 862.63 µs | 569975 | 240525 | 231884 | 2.4546 ms |
| [bitcode 0.6.6][bitcode] | 134.81 µs | 1.2480 ms | 169.85 µs | 327688 | 200947 | 182040 | 733.38 µs |
| [borsh 1.5.7][borsh] | 557.61 µs | 1.8129 ms | † | 446595 | 234236 | 209834 | 2.0238 ms |
| [capnp 0.21.1][capnp] | 439.70 µs | † | † | 803896 | 335606 | 280744 | 3.5595 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 813.08 µs | 4.6494 ms | 3.4779 ms | 1109831 | 344745 | 274333 | 3.3968 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.7506 ms | 10.298 ms | † | 1109821 | 344751 | 274345 | 3.4188 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8853 ms | 4.7731 ms | 3.4357 ms | 1109821 | 344751 | 274345 | 3.4682 ms |
| [databuf 0.5.0][databuf] | 317.43 µs | 1.7308 ms | 812.56 µs | 356311 | 213062 | 198403 | 1.9467 ms |
| [dlhn 0.1.7][dlhn] | 745.59 µs | 2.6039 ms | † | 366496 | 220600 | 205586 | 2.0194 ms |
| [flatbuffers 25.2.10][flatbuffers] | 3.2618 ms | † | † | 849472 | 347816 | 294871 | 3.5153 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.9961 ms | 6.8283 ms | 5.6110 ms | 1187688 | 557642 | 553730 | 6.1770 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6709 ms | 6.8194 ms | † | 1623191 | 466527 | 359157 | 5.6686 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2182 ms | 4.6131 ms | † | 1623191 | 466527 | 359157 | 5.6063 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 959.76 µs | 2.8489 ms | † | 391251 | 236877 | 220395 | 2.1534 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4844 ms | 3.0636 ms | 1.6823 ms | 424533 | 245214 | 226077 | 2.2072 ms |
| [minicbor 1.0.0][minicbor] | 560.99 µs | 3.3520 ms | 1.8603 ms | 428773 | 249857 | 228630 | 2.2206 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 4.9662 ms | 3.9792 ms | 2.9357 ms | 449745 | 252432 | 230965 | 2.3013 ms |
| [nanoserde 0.2.1][nanoserde] | 279.56 µs | 1.9261 ms | † | 567975 | 239930 | 231872 | 2.4384 ms |
| [nibblecode 0.1.0][nibblecode] | 184.75 µs | † | † | 603928 | 429367 | 408015 | 3.6152 ms |
| [postcard 1.1.1][postcard] | 448.34 µs | 2.0826 ms | 816.08 µs | 367489 | 221913 | 207244 | 2.0313 ms |
| [pot 3.0.1][pot] | 2.3612 ms | 6.1662 ms | 5.0781 ms | 599125 | 299158 | 247675 | 2.9052 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.2797 ms\**</span> <span title="populate + encode">*2.9763 ms\**</span> | 3.6460 ms | † | 596811 | 305319 | 268737 | 2.9386 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0357 ms\**</span> <span title="populate + encode">*2.9813 ms\**</span> | 3.8307 ms | † | 596811 | 305319 | 268737 | 2.9599 ms |
| [rkyv 0.8.10][rkyv] | 338.43 µs | <span title="unvalidated">*1.4940 ms\**</span> <span title="validated upfront with error">*1.8637 ms\**</span> | † | 603776 | 254776 | 219421 | 2.2883 ms |
| [ron 0.10.1][ron] | 7.9461 ms | 25.530 ms | 23.756 ms | 1465223 | 434935 | 342907 | 5.5263 ms |
| [savefile 0.18.6][savefile] | 208.98 µs | 1.8136 ms | † | 566991 | 239362 | 231478 | 2.4090 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 610.16 µs | 2.1333 ms | † | 356311 | 212976 | 198423 | 1.9734 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3458 ms | 5.2963 ms | 3.7557 ms | 1276014 | 373898 | 293384 | 3.6379 ms |
| [serde_bare 0.5.0][serde_bare] | 756.78 µs | 2.3080 ms | † | 356311 | 213062 | 198403 | 1.9532 ms |
| [speedy 0.8.7][speedy] | 258.72 µs | 1.6838 ms | 568.17 µs | 449595 | 234970 | 210192 | 2.0294 ms |
| [wiring 0.2.4][wiring] | 189.26 µs | 1.8412 ms | † | 566975 | 247810 | 225086 | 2.5088 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*70.011 ns\**</span> | <span title="validated on-demand with error">*418.40 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4861 ns\**</span> <span title="validated upfront with error">*2.4529 ms\**</span> | <span title="unvalidated">*1.3611 µs\**</span> <span title="validated upfront with error">*2.4565 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*252.01 µs\**</span> | <span title="unvalidated">*201.47 ns\**</span> <span title="validated upfront with error">*254.79 µs\**</span> | <span title="unvalidated">*722.94 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*354.06 µs\**</span> | <span title="unvalidated">*240.73 ns\**</span> <span title="validated upfront with error">*356.77 µs\**</span> | <span title="unvalidated">*756.50 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.56%\**</span> <span title="prepend">*17.10%\**</span> | 39.65% | 9.75% | 66.96% | 71.47% | 73.00% | 28.05% |
| [bincode 2.0.1][bincode] | 50.08% | 67.21% | 20.74% | 89.19% | 90.81% | 88.27% | 36.12% |
| [bincode 1.3.3][bincode1] | 21.71% | 69.10% | 19.69% | 57.49% | 83.55% | 78.50% | 29.88% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 24.18% | 68.84% | † | 73.37% | 85.79% | 86.75% | 36.24% |
| [capnp 0.21.1][capnp] | 30.66% | † | † | 40.76% | 59.88% | 64.84% | 20.60% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.58% | 26.84% | 4.88% | 29.53% | 58.29% | 66.36% | 21.59% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.59% | 12.12% | † | 29.53% | 58.29% | 66.35% | 21.45% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.15% | 26.15% | 4.94% | 29.53% | 58.29% | 66.35% | 21.15% |
| [databuf 0.5.0][databuf] | 42.47% | 72.11% | 20.90% | 91.97% | 94.31% | 91.75% | 37.67% |
| [dlhn 0.1.7][dlhn] | 18.08% | 47.93% | † | 89.41% | 91.09% | 88.55% | 36.32% |
| [flatbuffers 25.2.10][flatbuffers] | 4.13% | † | † | 38.58% | 57.77% | 61.74% | 20.86% |
| [flexbuffers 25.2.10][flexbuffers] | 1.69% | 18.28% | 3.03% | 27.59% | 36.04% | 32.88% | 11.87% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.67% | 18.30% | † | 20.19% | 43.07% | 50.69% | 12.94% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.08% | 27.05% | † | 20.19% | 43.07% | 50.69% | 13.08% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 14.05% | 43.81% | † | 83.75% | 84.83% | 82.60% | 34.06% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.08% | 40.74% | 10.10% | 77.19% | 81.95% | 80.52% | 33.23% |
| [minicbor 1.0.0][minicbor] | 24.03% | 37.23% | 9.13% | 76.42% | 80.42% | 79.62% | 33.03% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.71% | 31.36% | 5.79% | 72.86% | 79.60% | 78.82% | 31.87% |
| [nanoserde 0.2.1][nanoserde] | 48.22% | 64.79% | † | 57.69% | 83.75% | 78.51% | 30.08% |
| [nibblecode 0.1.0][nibblecode] | 72.97% | † | † | 54.26% | 46.80% | 44.62% | 20.29% |
| [postcard 1.1.1][postcard] | 30.07% | 59.93% | 20.81% | 89.17% | 90.55% | 87.84% | 36.10% |
| [pot 3.0.1][pot] | 5.71% | 20.24% | 3.34% | 54.69% | 67.17% | 73.50% | 25.24% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*10.53%\**</span> <span title="populate + encode">*4.53%\**</span> | 34.23% | † | 54.91% | 65.82% | 67.74% | 24.96% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*13.02%\**</span> <span title="populate + encode">*4.52%\**</span> | 32.58% | † | 54.91% | 65.82% | 67.74% | 24.78% |
| [rkyv 0.8.10][rkyv] | 39.83% | <span title="unvalidated">*83.53%\**</span> <span title="validated upfront with error">*66.96%\**</span> | † | 54.27% | 78.87% | 82.96% | 32.05% |
| [ron 0.10.1][ron] | 1.70% | 4.89% | 0.71% | 22.36% | 46.20% | 53.09% | 13.27% |
| [savefile 0.18.6][savefile] | 64.51% | 68.81% | † | 57.79% | 83.95% | 78.64% | 30.44% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.09% | 58.50% | † | 91.97% | 94.35% | 91.74% | 37.16% |
| [serde-brief 0.1.1][serde-brief] | 10.02% | 23.56% | 4.52% | 25.68% | 53.74% | 62.05% | 20.16% |
| [serde_bare 0.5.0][serde_bare] | 17.81% | 54.07% | † | 91.97% | 94.31% | 91.75% | 37.55% |
| [speedy 0.8.7][speedy] | 52.11% | 74.12% | 29.89% | 72.89% | 85.52% | 86.61% | 36.14% |
| [wiring 0.2.4][wiring] | 71.23% | 67.78% | † | 57.80% | 81.09% | 80.88% | 29.23% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.78%\**</span> | <span title="validated on-demand with error">*48.15%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*14.80%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.08%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*83.69%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*95.56%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.4937 ms\**</span> <span title="prepend">*2.5233 ms\**</span> | 8.6120 ms | 1704643 | 1294259 | 1245668 | 11.579 ms |
| [bincode 2.0.1][bincode] | 1.1889 ms | 4.1527 ms | 1406257 | 1117802 | 1062438 | 9.5810 ms |
| [bincode 1.3.3][bincode1] | 3.9644 ms | 4.1659 ms | 1854234 | 1141994 | 1048745 | 10.279 ms |
| [bitcode 0.6.6][bitcode] | 735.23 µs | 2.3333 ms | 971318 | 878034 | 850340 | 2.8900 ms |
| [borsh 1.5.7][borsh] | 2.8588 ms | 2.9203 ms | 1521989 | 1108471 | 1038528 | 10.177 ms |
| [capnp 0.21.1][capnp] | 2.1596 ms | † | 2724288 | 1546992 | 1239111 | 14.608 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.0371 ms | 17.960 ms | 6012539 | 1695215 | 1464951 | 21.370 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.090 ms | 55.788 ms | 6012373 | 1695146 | 1465025 | 21.083 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.8623 ms | 21.392 ms | 6012373 | 1695146 | 1465025 | 21.578 ms |
| [databuf 0.5.0][databuf] | 1.3140 ms | 3.7052 ms | 1319999 | 1062631 | 1008334 | 8.9328 ms |
| [dlhn 0.1.7][dlhn] | 4.7771 ms | 6.3491 ms | 1311281 | 1077520 | 1046095 | 8.6548 ms |
| [flatbuffers 25.2.10][flatbuffers] | 4.9345 ms | † | 2325620 | 1439185 | 1268060 | 14.837 ms |
| [flexbuffers 25.2.10][flexbuffers] | 41.217 ms | 34.485 ms | 5352680 | 2658295 | 2777967 | 35.878 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.168 ms | 31.699 ms | 9390461 | 2391679 | 1842767 | 34.827 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.390 ms | 26.420 ms | 9390461 | 2391679 | 1842767 | 34.872 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.4107 ms | 6.2350 ms | 1458773 | 1156055 | 1137788 | 9.7898 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.9247 ms | 11.213 ms | 1745322 | 1261627 | 1228923 | 11.511 ms |
| [minicbor 1.0.0][minicbor] | 2.3945 ms | 11.399 ms | 1777386 | 1276218 | 1252558 | 12.458 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.725 ms | 17.569 ms | 1770060 | 1277755 | 1263362 | 12.535 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2824 ms | 2.8312 ms | 1812404 | 1134820 | 1053109 | 10.305 ms |
| [nibblecode 0.1.0][nibblecode] | 514.03 µs | † | 2075936 | 1558720 | 1452905 | 14.058 ms |
| [postcard 1.1.1][postcard] | 1.9900 ms | 4.1879 ms | 1311281 | 1083900 | 1041434 | 8.6741 ms |
| [pot 3.0.1][pot] | 13.427 ms | 30.454 ms | 2604812 | 1482233 | 1298928 | 15.974 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*5.4075 ms\**</span> <span title="populate + encode">*9.3120 ms\**</span> | 8.9485 ms | 1859886 | 1338076 | 1295351 | 12.260 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*4.8081 ms\**</span> <span title="populate + encode">*11.947 ms\**</span> | 11.913 ms | 1859886 | 1338076 | 1295351 | 12.314 ms |
| [rkyv 0.8.10][rkyv] | 991.58 µs | <span title="unvalidated">*2.1852 ms\**</span> <span title="validated upfront with error">*2.6292 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.176 ms |
| [ron 0.10.1][ron] | 43.127 ms | 150.82 ms | 8677703 | 2233642 | 1826180 | 34.843 ms |
| [savefile 0.18.6][savefile] | 851.38 µs | 2.7390 ms | 1791505 | 1128012 | 1051153 | 10.239 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.0601 ms | 3.4283 ms | 1319999 | 1064380 | 1010708 | 8.7977 ms |
| [serde-brief 0.1.1][serde-brief] | 6.5938 ms | 20.947 ms | 6951772 | 1796265 | 1567819 | 25.428 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7868 ms | 4.7958 ms | 1319999 | 1062645 | 1008349 | 8.7717 ms |
| [speedy 0.8.7][speedy] | 778.61 µs | 2.4417 ms | 1584734 | 1119837 | 1037992 | 10.148 ms |
| [wiring 0.2.4][wiring] | 670.53 µs | 2.7959 ms | 1791489 | 1156963 | 1082815 | 10.654 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*70.359 ns\**</span> | <span title="validated on-demand with error">*709.53 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.5021 ns\**</span> <span title="validated upfront with error">*5.4576 ms\**</span> | <span title="unvalidated">*2.6042 µs\**</span> <span title="validated upfront with error">*5.4632 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2436 ns\**</span> <span title="validated upfront with error">*340.40 µs\**</span> | <span title="unvalidated">*439.49 ns\**</span> <span title="validated upfront with error">*341.61 µs\**</span> | <span title="unvalidated">*346.10 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2432 ns\**</span> <span title="validated upfront with error">*425.82 µs\**</span> | <span title="unvalidated">*419.77 ns\**</span> <span title="validated upfront with error">*426.87 µs\**</span> | <span title="unvalidated">*235.76 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.44%\**</span> <span title="prepend">*20.37%\**</span> | 25.37% | 56.98% | 67.84% | 68.26% | 24.96% |
| [bincode 2.0.1][bincode] | 43.24% | 52.62% | 69.07% | 78.55% | 80.04% | 30.16% |
| [bincode 1.3.3][bincode1] | 12.97% | 52.45% | 52.38% | 76.89% | 81.08% | 28.12% |
| [bitcode 0.6.6][bitcode] | 69.91% | 93.65% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 17.98% | 74.83% | 63.82% | 79.21% | 81.88% | 28.40% |
| [capnp 0.21.1][capnp] | 23.80% | † | 35.65% | 56.76% | 68.63% | 19.78% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.93% | 12.17% | 16.15% | 51.79% | 58.05% | 13.52% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.23% | 3.92% | 16.16% | 51.80% | 58.04% | 13.71% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.21% | 10.22% | 16.16% | 51.80% | 58.04% | 13.39% |
| [databuf 0.5.0][databuf] | 39.12% | 58.98% | 73.58% | 82.63% | 84.33% | 32.35% |
| [dlhn 0.1.7][dlhn] | 10.76% | 34.42% | 74.07% | 81.49% | 81.29% | 33.39% |
| [flatbuffers 25.2.10][flatbuffers] | 10.42% | † | 41.77% | 61.01% | 67.06% | 19.48% |
| [flexbuffers 25.2.10][flexbuffers] | 1.25% | 6.34% | 18.15% | 33.03% | 30.61% | 8.06% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.55% | 6.89% | 10.34% | 36.71% | 46.14% | 8.30% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.51% | 8.27% | 10.34% | 36.71% | 46.14% | 8.29% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 21.32% | 35.05% | 66.58% | 75.95% | 74.74% | 29.52% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 5.18% | 19.49% | 55.65% | 69.60% | 69.19% | 25.11% |
| [minicbor 1.0.0][minicbor] | 21.47% | 19.17% | 54.65% | 68.80% | 67.89% | 23.20% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.73% | 12.44% | 54.87% | 68.72% | 67.31% | 23.06% |
| [nanoserde 0.2.1][nanoserde] | 40.08% | 77.18% | 53.59% | 77.37% | 80.75% | 28.05% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 56.33% | 58.53% | 20.56% |
| [postcard 1.1.1][postcard] | 25.83% | 52.18% | 74.07% | 81.01% | 81.65% | 33.32% |
| [pot 3.0.1][pot] | 3.83% | 7.18% | 37.29% | 59.24% | 65.46% | 18.09% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*9.51%\**</span> <span title="populate + encode">*5.52%\**</span> | 24.42% | 52.22% | 65.62% | 65.65% | 23.57% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*10.69%\**</span> <span title="populate + encode">*4.30%\**</span> | 18.34% | 52.22% | 65.62% | 65.65% | 23.47% |
| [rkyv 0.8.10][rkyv] | 51.84% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.11%\**</span> | 46.79% | 63.45% | 70.25% | 21.93% |
| [ron 0.10.1][ron] | 1.19% | 1.45% | 11.19% | 39.31% | 46.56% | 8.29% |
| [savefile 0.18.6][savefile] | 60.38% | 79.78% | 54.22% | 77.84% | 80.90% | 28.23% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.80% | 63.74% | 73.58% | 82.49% | 84.13% | 32.85% |
| [serde-brief 0.1.1][serde-brief] | 7.80% | 10.43% | 13.97% | 48.88% | 54.24% | 11.37% |
| [serde_bare 0.5.0][serde_bare] | 10.74% | 45.56% | 73.58% | 82.63% | 84.33% | 32.95% |
| [speedy 0.8.7][speedy] | 66.02% | 89.50% | 61.29% | 78.41% | 81.92% | 28.48% |
| [wiring 0.2.4][wiring] | 76.66% | 78.16% | 54.22% | 75.89% | 78.53% | 27.12% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.77%\**</span> | <span title="validated on-demand with error">*59.16%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.69%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.12%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*95.51%\**</span> <span title="validated upfront with error">*0.12%\**</span> | <span title="unvalidated">*68.12%\**</span> |
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
[flexbuffers]: https://crates.io/crates/flexbuffers/25.2.10
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
