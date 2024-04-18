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
* **Size**: the size of the buffer when serialized
* **Zlib**: the size of the buffer after zlib compression
* **Zstd**: the size of the buffer after zstd compression
* **Zstd Time**: the time taken to compress the serialized buffer with zstd

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2024-4-18 3:54:59

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.77.0-nightly (11f32b73e 2024-01-31)
binary: rustc
commit-hash: 11f32b73e0dc9287e305b5b9980d24aecdc8c17f
commit-date: 2024-01-31
host: x86_64-unknown-linux-gnu
release: 1.77.0-nightly
LLVM version: 17.0.6
```

### CPU info

```
Architecture:                       x86_64
CPU op-mode(s):                     32-bit, 64-bit
Address sizes:                      48 bits physical, 48 bits virtual
Byte Order:                         Little Endian
CPU(s):                             4
On-line CPU(s) list:                0-3
Vendor ID:                          AuthenticAMD
Model name:                         AMD EPYC 7763 64-Core Processor
CPU family:                         25
Model:                              1
Thread(s) per core:                 2
Core(s) per socket:                 2
Socket(s):                          1
Stepping:                           1
BogoMIPS:                           4890.86
Flags:                              fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext invpcid_single vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
Virtualization:                     AMD-V
Hypervisor vendor:                  Microsoft
Virtualization type:                full
L1d cache:                          64 KiB (2 instances)
L1i cache:                          64 KiB (2 instances)
L2 cache:                           1 MiB (2 instances)
L3 cache:                           32 MiB (1 instance)
NUMA node(s):                       1
NUMA node0 CPU(s):                  0-3
Vulnerability Gather data sampling: Not affected
Vulnerability Itlb multihit:        Not affected
Vulnerability L1tf:                 Not affected
Vulnerability Mds:                  Not affected
Vulnerability Meltdown:             Not affected
Vulnerability Mmio stale data:      Not affected
Vulnerability Retbleed:             Not affected
Vulnerability Spec rstack overflow: Vulnerable: Safe RET, no microcode
Vulnerability Spec store bypass:    Vulnerable
Vulnerability Spectre v1:           Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:           Mitigation; Retpolines, STIBP disabled, RSB filling, PBRSB-eIBRS Not affected
Vulnerability Srbds:                Not affected
Vulnerability Tsx async abort:      Not affected
```

</details>

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 407.61 µs | <span title="unvalidated">*1.4903 ms\**</span> | 1705800 | 520081 | 413351 | 6.9419 ms |
| [alkahest 0.1.5][alkahest] | 191.94 µs | † | 1045784 | 454157 | 389424 | 6.3108 ms |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*787.73 µs\**</span> <span title="prepend">*752.29 µs\**</span> | 3.1951 ms | 874632 | 355446 | 311723 | 5.1147 ms |
| [bincode 2.0.0-rc][bincode] | 214.90 µs | 2.4428 ms | 741295 | 303944 | 257153 | 4.0383 ms |
| [bincode 1.3.3][bincode1] | 525.09 µs | 1.9796 ms | 1045784 | 373127 | 311761 | 4.9463 ms |
| [bitcode 0.6.0][bitcode] | 144.56 µs | 1.4989 ms | 703710 | 288826 | 229755 | 2.4572 ms |
| [borsh 1.3.1][borsh] | 546.94 µs | 2.2298 ms | 885780 | 362204 | 286514 | 4.5714 ms |
| [bson 2.9.0][bson] | 2.2724 ms | 7.2171 ms | 1924682 | 532821 | 376270 | 5.7411 ms |
| [capnp 0.18.13][capnp] | 478.17 µs | † | 1443216 | 513986 | 428649 | 6.9141 ms |
| [cbor4ii 0.3.2][cbor4ii] | 908.16 µs | 4.7731 ms | 1407835 | 403440 | 324081 | 4.9673 ms |
| [ciborium 0.2.2][ciborium] | 3.8619 ms | 10.233 ms | 1407835 | 403440 | 324081 | 4.8462 ms |
| [databuf 0.5.0][databuf] | 294.44 µs | 2.0361 ms | 765778 | 311715 | 264630 | 3.9041 ms |
| [dlhn 0.1.6][dlhn] | 804.23 µs | 2.4246 ms | 724953 | 301446 | 253629 | 3.5964 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.5601 ms | † | 1276368 | 468539 | 388832 | 5.2393 ms |
| [msgpacker 0.4.3][msgpacker] | 1.2641 ms | 2.5263 ms | 764996 | 315291 | 264898 | 3.9925 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5637 ms | 3.9883 ms | 818669 | 332556 | 285514 | 4.3784 ms |
| [nanoserde 0.1.37][nanoserde] | 258.19 µs | 2.0650 ms | 1045784 | 373127 | 311761 | 4.7504 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 653.48 µs | 2.2268 ms | 765778 | 311743 | 264518 | 4.1748 ms |
| [postcard 1.0.8][postcard] | 421.38 µs | 2.1433 ms | 724953 | 302399 | 253747 | 3.8590 ms |
| [pot 3.0.0][pot] | 2.2866 ms | 6.4107 ms | 971922 | 372513 | 304122 | 4.6771 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.0323 ms\**</span> <span title="populate + encode">*2.5788 ms\**</span> | 3.4239 ms | 884628 | 363130 | 315494 | 5.2404 ms |
| [rkyv 0.7.44][rkyv] | 222.07 µs | <span title="unvalidated">*1.4560 ms\**</span> <span title="validated upfront with error">*1.9873 ms\**</span> | 1011488 | 383862 | 333545 | 5.1321 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3115 ms | 3.4827 ms | 784997 | 325384 | 278219 | 4.1631 ms |
| [ron 0.8.1][ron] | 14.459 ms | 17.194 ms | 1607459 | 449158 | 349713 | 5.7784 ms |
| [savefile 0.16.5][savefile] | 200.77 µs | 2.0941 ms | 1045800 | 373139 | 311755 | 4.7033 ms |
| [serde_bare 0.5.0][serde_bare] | 669.63 µs | 2.0799 ms | 765778 | 311715 | 264630 | 3.9824 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9229 ms | 5.3767 ms | 1407835 | 403440 | 324081 | 4.9194 ms |
| [serde_json 1.0.115][serde_json] | 4.1078 ms | 5.8184 ms | 1827461 | 470560 | 361090 | 5.6802 ms |
| [simd-json 0.13.9][simd-json] | 2.0581 ms | 4.7017 ms | 1827461 | 470560 | 361090 | 5.6607 ms |
| [speedy 0.8.7][speedy] | 199.15 µs | 1.7563 ms | 885780 | 362204 | 286514 | 4.6220 ms |
| [wiring 0.1.6][wiring] | 912.89 µs | 5.6293 ms | 1045784 | 337930 | 276188 | 4.1433 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.840 µs\**</span> | <span title="unvalidated">*40.871 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8554 ns\**</span> | <span title="validated on-demand with panic">*24.845 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.572 ns\**</span> | <span title="validated on-demand with error">*163.27 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4750 ns\**</span> <span title="validated upfront with error">*1.8224 ms\**</span> | <span title="unvalidated">*55.587 µs\**</span> <span title="validated upfront with error">*1.8745 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2370 ns\**</span> <span title="validated upfront with error">*525.10 µs\**</span> | <span title="unvalidated">*10.677 µs\**</span> <span title="validated upfront with error">*535.32 µs\**</span> | 9.9453 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 35.47% | <span title="unvalidated">*97.70%\**</span> | 41.25% | 55.53% | 55.58% | 35.40% |
| [alkahest 0.1.5][alkahest] | 75.32% | † | 67.29% | 63.60% | 59.00% | 38.94% |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*18.35%\**</span> <span title="prepend">*19.22%\**</span> | 45.57% | 80.46% | 81.26% | 73.70% | 48.04% |
| [bincode 2.0.0-rc][bincode] | 67.27% | 59.60% | 94.93% | 95.03% | 89.35% | 60.85% |
| [bincode 1.3.3][bincode1] | 27.53% | 73.55% | 67.29% | 77.41% | 73.70% | 49.68% |
| [bitcode 0.6.0][bitcode] | 100.00% | 97.14% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 26.43% | 65.30% | 79.45% | 79.74% | 80.19% | 53.75% |
| [bson 2.9.0][bson] | 6.36% | 20.17% | 36.56% | 54.21% | 61.06% | 42.80% |
| [capnp 0.18.13][capnp] | 30.23% | † | 48.76% | 56.19% | 53.60% | 35.54% |
| [cbor4ii 0.3.2][cbor4ii] | 15.92% | 30.50% | 49.99% | 71.59% | 70.89% | 49.47% |
| [ciborium 0.2.2][ciborium] | 3.74% | 14.23% | 49.99% | 71.59% | 70.89% | 50.70% |
| [databuf 0.5.0][databuf] | 49.10% | 71.51% | 91.89% | 92.66% | 86.82% | 62.94% |
| [dlhn 0.1.6][dlhn] | 17.97% | 60.05% | 97.07% | 95.81% | 90.59% | 68.32% |
| [flatbuffers 23.5.26][flatbuffers] | 9.27% | † | 55.13% | 61.64% | 59.09% | 46.90% |
| [msgpacker 0.4.3][msgpacker] | 11.44% | 57.63% | 91.99% | 91.61% | 86.73% | 61.55% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.60% | 36.51% | 85.96% | 86.85% | 80.47% | 56.12% |
| [nanoserde 0.1.37][nanoserde] | 55.99% | 70.51% | 67.29% | 77.41% | 73.70% | 51.73% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 22.12% | 65.39% | 91.89% | 92.65% | 86.86% | 58.86% |
| [postcard 1.0.8][postcard] | 34.31% | 67.93% | 97.07% | 95.51% | 90.54% | 63.67% |
| [pot 3.0.0][pot] | 6.32% | 22.71% | 72.40% | 77.53% | 75.55% | 52.54% |
| [prost 0.12.4][prost] | <span title="encode">*14.00%\**</span> <span title="populate + encode">*5.61%\**</span> | 42.52% | 79.55% | 79.54% | 72.82% | 46.89% |
| [rkyv 0.7.44][rkyv] | 65.10% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*73.27%\**</span> | 69.57% | 75.24% | 68.88% | 47.88% |
| [rmp-serde 1.1.2][rmp-serde] | 11.02% | 41.81% | 89.64% | 88.76% | 82.58% | 59.02% |
| [ron 0.8.1][ron] | 1.00% | 8.47% | 43.78% | 64.30% | 65.70% | 42.52% |
| [savefile 0.16.5][savefile] | 72.00% | 69.53% | 67.29% | 77.40% | 73.70% | 52.24% |
| [serde_bare 0.5.0][serde_bare] | 21.59% | 70.00% | 91.89% | 92.66% | 86.82% | 61.70% |
| [serde_cbor 0.11.2][serde_cbor] | 7.52% | 27.08% | 49.99% | 71.59% | 70.89% | 49.95% |
| [serde_json 1.0.115][serde_json] | 3.52% | 25.02% | 38.51% | 61.38% | 63.63% | 43.26% |
| [simd-json 0.13.9][simd-json] | 7.02% | 30.97% | 38.51% | 61.38% | 63.63% | 43.41% |
| [speedy 0.8.7][speedy] | 72.59% | 82.90% | 79.45% | 79.74% | 80.19% | 53.16% |
| [wiring 0.1.6][wiring] | 15.84% | 25.86% | 67.29% | 85.47% | 83.19% | 59.31% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*26.12%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*42.97%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*6.54%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*19.21%\**</span> <span title="validated upfront with error">*0.57%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.99%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 311.67 µs | <span title="unvalidated">*277.09 µs\**</span> | 6000024 | 5378513 | 5345890 | 8.6462 ms |
| [alkahest 0.1.5][alkahest] | 202.58 µs | † | 6000008 | 5378500 | 5345890 | 7.6623 ms |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*6.6278 ms\**</span> <span title="prepend">*8.3466 ms\**</span> | 10.238 ms | 8625005 | 6443961 | 6231572 | 71.096 ms |
| [bincode 2.0.0-rc][bincode] | 430.04 µs | 825.16 µs | 6000005 | 5378497 | 5345897 | 7.5997 ms |
| [bincode 1.3.3][bincode1] | 5.0420 ms | 4.1229 ms | 6000008 | 5378500 | 5345890 | 7.5986 ms |
| [bitcode 0.6.0][bitcode] | 1.4151 ms | 618.61 µs | 6000006 | 5182295 | 4923880 | 12.919 ms |
| [borsh 1.3.1][borsh] | 6.0485 ms | 4.1728 ms | 6000004 | 5378496 | 5345889 | 7.5450 ms |
| [bson 2.9.0][bson] | 46.654 ms | 82.260 ms | 23013911 | 9212089 | 7497811 | 112.90 ms |
| [capnp 0.18.13][capnp] | 6.6955 ms | † | 14000088 | 7130367 | 6051062 | 81.804 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.402 ms | 47.407 ms | 13125016 | 7524114 | 6757967 | 90.832 ms |
| [ciborium 0.2.2][ciborium] | 65.709 ms | 100.62 ms | 13122324 | 7524660 | 6759658 | 90.755 ms |
| [databuf 0.5.0][databuf] | 2.3960 ms | 5.2972 ms | 6000003 | 5378495 | 5345900 | 7.5910 ms |
| [dlhn 0.1.6][dlhn] | 7.5806 ms | 5.7256 ms | 6000003 | 5378495 | 5345900 | 7.5276 ms |
| [flatbuffers 23.5.26][flatbuffers] | 676.40 µs | † | 6000024 | 5378434 | 5345910 | 7.6807 ms |
| [msgpacker 0.4.3][msgpacker] | 19.480 ms | 8.5883 ms | 7500005 | 6058442 | 6014337 | 10.274 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 128.35 ms | 27.172 ms | 8125037 | 6493484 | 6386940 | 75.895 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2088 ms | 901.99 µs | 6000008 | 5378500 | 5345890 | 7.5397 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 5.0899 ms | 4.0317 ms | 6000004 | 5378496 | 5345889 | 7.9453 ms |
| [postcard 1.0.8][postcard] | 491.64 µs | 1.0926 ms | 6000003 | 5378495 | 5345900 | 7.9533 ms |
| [pot 3.0.0][pot] | 38.460 ms | 74.672 ms | 10122342 | 6814618 | 6852251 | 80.013 ms |
| [prost 0.12.4][prost] | <span title="encode">*8.0583 ms\**</span> <span title="populate + encode">*9.0972 ms\**</span> | 13.345 ms | 8750000 | 6665735 | 6421871 | 72.483 ms |
| [rkyv 0.7.44][rkyv] | 206.48 µs | <span title="unvalidated">*203.51 µs\**</span> <span title="validated upfront with error">*198.57 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.8564 ms |
| [rmp-serde 1.1.2][rmp-serde] | 13.466 ms | 18.776 ms | 8125006 | 6494876 | 6391037 | 70.831 ms |
| [ron 0.8.1][ron] | 174.10 ms | 260.00 ms | 22192885 | 8970395 | 8138755 | 150.86 ms |
| [savefile 0.16.5][savefile] | 259.85 µs | 259.41 µs | 6000024 | 5378518 | 5345893 | 7.6081 ms |
| [serde_bare 0.5.0][serde_bare] | 6.2079 ms | 4.1075 ms | 6000003 | 5378495 | 5345900 | 7.6274 ms |
| [serde_cbor 0.11.2][serde_cbor] | 36.000 ms | 43.425 ms | 13122324 | 7524660 | 6759658 | 95.016 ms |
| [serde_json 1.0.115][serde_json] | 87.601 ms | 88.575 ms | 26192883 | 9566084 | 8586741 | 156.79 ms |
| [simd-json 0.13.9][simd-json] | 53.849 ms | 74.393 ms | 26192883 | 9566084 | 8586741 | 166.77 ms |
| [speedy 0.8.7][speedy] | 258.38 µs | 258.24 µs | 6000004 | 5378496 | 5345889 | 7.6430 ms |
| [wiring 0.1.6][wiring] | 9.0387 ms | 17.865 ms | 6000008 | 5378952 | 5345894 | 7.6130 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1654 ns\**</span> | <span title="unvalidated">*142.00 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8553 ns\**</span> | <span title="validated on-demand with panic">*77.341 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*116.89 ns\**</span> | <span title="validated on-demand with error">*2.1407 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4725 ns\**</span> <span title="validated upfront with error">*37.909 ns\**</span> | <span title="unvalidated">*54.144 µs\**</span> <span title="validated upfront with error">*77.391 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2380 ns\**</span> <span title="validated upfront with error">*13.623 ns\**</span> | <span title="unvalidated">*48.392 µs\**</span> <span title="validated upfront with error">*38.727 µs\**</span> | 109.02 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 65.00% | <span title="unvalidated">*71.66%\**</span> | 100.00% | 96.35% | 92.11% | 87.06% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 98.24% |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*3.06%\**</span> <span title="prepend">*2.43%\**</span> | 1.94% | 69.57% | 80.42% | 79.02% | 10.59% |
| [bincode 2.0.0-rc][bincode] | 47.11% | 24.06% | 100.00% | 96.35% | 92.11% | 99.05% |
| [bincode 1.3.3][bincode1] | 4.02% | 4.82% | 100.00% | 96.35% | 92.11% | 99.07% |
| [bitcode 0.6.0][bitcode] | 14.32% | 32.10% | 100.00% | 100.00% | 100.00% | 58.27% |
| [borsh 1.3.1][borsh] | 3.35% | 4.76% | 100.00% | 96.35% | 92.11% | 99.77% |
| [bson 2.9.0][bson] | 0.43% | 0.24% | 26.07% | 56.26% | 65.67% | 6.67% |
| [capnp 0.18.13][capnp] | 3.03% | † | 42.86% | 72.68% | 81.37% | 9.20% |
| [cbor4ii 0.3.2][cbor4ii] | 1.95% | 0.42% | 45.71% | 68.88% | 72.86% | 8.29% |
| [ciborium 0.2.2][ciborium] | 0.31% | 0.20% | 45.72% | 68.87% | 72.84% | 8.29% |
| [databuf 0.5.0][databuf] | 8.45% | 3.75% | 100.00% | 96.35% | 92.11% | 99.16% |
| [dlhn 0.1.6][dlhn] | 2.67% | 3.47% | 100.00% | 96.35% | 92.11% | 100.00% |
| [flatbuffers 23.5.26][flatbuffers] | 29.95% | † | 100.00% | 96.35% | 92.11% | 98.01% |
| [msgpacker 0.4.3][msgpacker] | 1.04% | 2.31% | 80.00% | 85.54% | 81.87% | 73.27% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.16% | 0.73% | 73.85% | 79.81% | 77.09% | 9.92% |
| [nanoserde 0.1.37][nanoserde] | 16.76% | 22.01% | 100.00% | 96.35% | 92.11% | 99.84% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.98% | 4.93% | 100.00% | 96.35% | 92.11% | 94.74% |
| [postcard 1.0.8][postcard] | 41.20% | 18.17% | 100.00% | 96.35% | 92.11% | 94.65% |
| [pot 3.0.0][pot] | 0.53% | 0.27% | 59.27% | 76.05% | 71.86% | 9.41% |
| [prost 0.12.4][prost] | <span title="encode">*2.51%\**</span> <span title="populate + encode">*2.23%\**</span> | 1.49% | 68.57% | 77.75% | 76.67% | 10.39% |
| [rkyv 0.7.44][rkyv] | 98.11% | <span title="unvalidated">*97.57%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 95.81% |
| [rmp-serde 1.1.2][rmp-serde] | 1.50% | 1.06% | 73.85% | 79.79% | 77.04% | 10.63% |
| [ron 0.8.1][ron] | 0.12% | 0.08% | 27.04% | 57.77% | 60.50% | 4.99% |
| [savefile 0.16.5][savefile] | 77.96% | 76.55% | 100.00% | 96.35% | 92.11% | 98.94% |
| [serde_bare 0.5.0][serde_bare] | 3.26% | 4.83% | 100.00% | 96.35% | 92.11% | 98.69% |
| [serde_cbor 0.11.2][serde_cbor] | 0.56% | 0.46% | 45.72% | 68.87% | 72.84% | 7.92% |
| [serde_json 1.0.115][serde_json] | 0.23% | 0.22% | 22.91% | 54.17% | 57.34% | 4.80% |
| [simd-json 0.13.9][simd-json] | 0.38% | 0.27% | 22.91% | 54.17% | 57.34% | 4.51% |
| [speedy 0.8.7][speedy] | 78.40% | 76.89% | 100.00% | 96.35% | 92.11% | 98.49% |
| [wiring 0.1.6][wiring] | 2.24% | 1.11% | 100.00% | 96.34% | 92.11% | 98.88% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.17%\**</span> | <span title="unvalidated">*27.27%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.73%\**</span> | <span title="validated on-demand with panic">*50.07%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.06%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.07%\**</span> <span title="validated upfront with error">*3.27%\**</span> | <span title="unvalidated">*71.53%\**</span> <span title="validated upfront with error">*50.04%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*9.09%\**</span> | <span title="unvalidated">*80.03%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 192.40 µs | <span title="unvalidated">*1.3244 ms\**</span> | 1290592 | 396689 | 340193 | 5.2959 ms |
| [alkahest 0.1.5][alkahest] | 220.63 µs | † | 667570 | 325484 | 320452 | 4.1851 ms |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*937.93 µs\**</span> <span title="prepend">*924.23 µs\**</span> | 3.1427 ms | 489348 | 281173 | 249546 | 3.2054 ms |
| [bincode 2.0.0-rc][bincode] | 281.46 µs | 2.0787 ms | 367413 | 221291 | 206273 | 2.6644 ms |
| [bincode 1.3.3][bincode1] | 575.41 µs | 1.8064 ms | 569975 | 240525 | 232423 | 3.0593 ms |
| [bitcode 0.6.0][bitcode] | 132.96 µs | 1.2690 ms | 327688 | 200947 | 182736 | 764.69 µs |
| [borsh 1.3.1][borsh] | 557.22 µs | 1.8522 ms | 446595 | 234236 | 210008 | 2.6464 ms |
| [bson 2.9.0][bson] | 3.0000 ms | 8.3537 ms | 1619653 | 502185 | 328399 | 5.0063 ms |
| [capnp 0.18.13][capnp] | 463.34 µs | † | 803896 | 335606 | 280851 | 4.0860 ms |
| [cbor4ii 0.3.2][cbor4ii] | 795.60 µs | 4.7781 ms | 1109831 | 344745 | 274514 | 4.0458 ms |
| [ciborium 0.2.2][ciborium] | 3.6672 ms | 9.7676 ms | 1109821 | 344751 | 274526 | 4.0494 ms |
| [databuf 0.5.0][databuf] | 291.22 µs | 1.7569 ms | 356311 | 213062 | 198488 | 2.5366 ms |
| [dlhn 0.1.6][dlhn] | 813.96 µs | 2.5663 ms | 366496 | 220600 | 205683 | 2.6419 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.3299 ms | † | 844168 | 345696 | 294015 | 4.0353 ms |
| [msgpacker 0.4.3][msgpacker] | 1.0040 ms | 2.8094 ms | 391251 | 236877 | 220476 | 2.8182 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2562 ms | 3.8626 ms | 449745 | 252432 | 231110 | 2.9674 ms |
| [nanoserde 0.1.37][nanoserde] | 268.64 µs | 1.8788 ms | 567975 | 239930 | 232419 | 2.8828 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 651.32 µs | 1.9975 ms | 356311 | 212976 | 198524 | 2.4485 ms |
| [postcard 1.0.8][postcard] | 430.26 µs | 1.9891 ms | 367489 | 221913 | 207344 | 2.5014 ms |
| [pot 3.0.0][pot] | 2.3233 ms | 6.0386 ms | 599125 | 299158 | 247693 | 3.3984 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.3238 ms\**</span> <span title="populate + encode">*3.0119 ms\**</span> | 3.6022 ms | 596811 | 305319 | 269310 | 3.5129 ms |
| [rkyv 0.7.44][rkyv] | 296.49 µs | <span title="unvalidated">*1.2625 ms\**</span> <span title="validated upfront with error">*1.7789 ms\**</span> | 596952 | 253967 | 220706 | 2.7408 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3022 ms | 3.0022 ms | 424533 | 245214 | 226188 | 2.7203 ms |
| [ron 0.8.1][ron] | 8.6576 ms | 17.944 ms | 1465223 | 434935 | 343338 | 6.2973 ms |
| [savefile 0.16.5][savefile] | 219.54 µs | 1.8577 ms | 566991 | 239361 | 232010 | 2.8797 ms |
| [serde_bare 0.5.0][serde_bare] | 735.84 µs | 2.2123 ms | 356311 | 213062 | 198488 | 2.5740 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7619 ms | 4.8047 ms | 1109821 | 344751 | 274526 | 3.8605 ms |
| [serde_json 1.0.115][serde_json] | 4.0073 ms | 6.7827 ms | 1623191 | 466527 | 359623 | 6.0768 ms |
| [simd-json 0.13.9][simd-json] | 2.1981 ms | 5.0991 ms | 1623191 | 466527 | 359623 | 6.1348 ms |
| [speedy 0.8.7][speedy] | 278.99 µs | 1.6503 ms | 449595 | 234970 | 210361 | 2.4972 ms |
| [wiring 0.1.6][wiring] | 970.17 µs | 4.8778 ms | 566975 | 247810 | 225259 | 2.9626 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*39.435 µs\**</span> | <span title="unvalidated">*37.684 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8582 ns\**</span> | <span title="validated on-demand with panic">*4.6131 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.643 ns\**</span> | <span title="validated on-demand with error">*431.53 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4731 ns\**</span> <span title="validated upfront with error">*1.8248 ms\**</span> | <span title="unvalidated">*1.4909 µs\**</span> <span title="validated upfront with error">*1.8306 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2372 ns\**</span> <span title="validated upfront with error">*521.15 µs\**</span> | <span title="unvalidated">*239.38 ns\**</span> <span title="validated upfront with error">*521.18 µs\**</span> | 851.25 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 69.11% | <span title="unvalidated">*95.33%\**</span> | 25.39% | 50.66% | 53.72% | 14.44% |
| [alkahest 0.1.5][alkahest] | 60.26% | † | 49.09% | 61.74% | 57.02% | 18.27% |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*14.18%\**</span> <span title="prepend">*14.39%\**</span> | 40.17% | 66.96% | 71.47% | 73.23% | 23.86% |
| [bincode 2.0.0-rc][bincode] | 47.24% | 60.74% | 89.19% | 90.81% | 88.59% | 28.70% |
| [bincode 1.3.3][bincode1] | 23.11% | 69.89% | 57.49% | 83.55% | 78.62% | 25.00% |
| [bitcode 0.6.0][bitcode] | 100.00% | 99.49% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 23.86% | 68.16% | 73.37% | 85.79% | 87.01% | 28.90% |
| [bson 2.9.0][bson] | 4.43% | 15.11% | 20.23% | 40.01% | 55.64% | 15.27% |
| [capnp 0.18.13][capnp] | 28.70% | † | 40.76% | 59.88% | 65.07% | 18.71% |
| [cbor4ii 0.3.2][cbor4ii] | 16.71% | 26.42% | 29.53% | 58.29% | 66.57% | 18.90% |
| [ciborium 0.2.2][ciborium] | 3.63% | 12.93% | 29.53% | 58.29% | 66.56% | 18.88% |
| [databuf 0.5.0][databuf] | 45.66% | 71.86% | 91.97% | 94.31% | 92.06% | 30.15% |
| [dlhn 0.1.6][dlhn] | 16.33% | 49.20% | 89.41% | 91.09% | 88.84% | 28.94% |
| [flatbuffers 23.5.26][flatbuffers] | 3.99% | † | 38.82% | 58.13% | 62.15% | 18.95% |
| [msgpacker 0.4.3][msgpacker] | 13.24% | 44.94% | 83.75% | 84.83% | 82.88% | 27.13% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.53% | 32.69% | 72.86% | 79.60% | 79.07% | 25.77% |
| [nanoserde 0.1.37][nanoserde] | 49.49% | 67.20% | 57.69% | 83.75% | 78.62% | 26.53% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.41% | 63.20% | 91.97% | 94.35% | 92.05% | 31.23% |
| [postcard 1.0.8][postcard] | 30.90% | 63.47% | 89.17% | 90.55% | 88.13% | 30.57% |
| [pot 3.0.0][pot] | 5.72% | 20.91% | 54.69% | 67.17% | 73.78% | 22.50% |
| [prost 0.12.4][prost] | <span title="encode">*10.04%\**</span> <span title="populate + encode">*4.41%\**</span> | 35.05% | 54.91% | 65.82% | 67.85% | 21.77% |
| [rkyv 0.7.44][rkyv] | 44.84% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*70.97%\**</span> | 54.89% | 79.12% | 82.80% | 27.90% |
| [rmp-serde 1.1.2][rmp-serde] | 10.21% | 42.05% | 77.19% | 81.95% | 80.79% | 28.11% |
| [ron 0.8.1][ron] | 1.54% | 7.04% | 22.36% | 46.20% | 53.22% | 12.14% |
| [savefile 0.16.5][savefile] | 60.56% | 67.96% | 57.79% | 83.95% | 78.76% | 26.55% |
| [serde_bare 0.5.0][serde_bare] | 18.07% | 57.07% | 91.97% | 94.31% | 92.06% | 29.71% |
| [serde_cbor 0.11.2][serde_cbor] | 7.55% | 26.28% | 29.53% | 58.29% | 66.56% | 19.81% |
| [serde_json 1.0.115][serde_json] | 3.32% | 18.61% | 20.19% | 43.07% | 50.81% | 12.58% |
| [simd-json 0.13.9][simd-json] | 6.05% | 24.76% | 20.19% | 43.07% | 50.81% | 12.46% |
| [speedy 0.8.7][speedy] | 47.66% | 76.50% | 72.89% | 85.52% | 86.87% | 30.62% |
| [wiring 0.1.6][wiring] | 13.70% | 25.88% | 57.80% | 81.09% | 81.12% | 25.81% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.64%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.58%\**</span> | <span title="validated on-demand with panic">*5.19%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*55.47%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.03%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.06%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 491.62 µs | <span title="unvalidated">*2.3307 ms\**</span> | 2984682 | 1408301 | 1273840 | 14.367 ms |
| [alkahest 0.1.5][alkahest] | 721.18 µs | † | 1863391 | 1234113 | 1202345 | 11.478 ms |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*4.9662 ms\**</span> <span title="prepend">*2.7386 ms\**</span> | 8.6021 ms | 1664428 | 1264167 | 1216472 | 11.213 ms |
| [bincode 2.0.0-rc][bincode] | 702.79 µs | 3.7034 ms | 1372381 | 1091486 | 1037296 | 9.7738 ms |
| [bincode 1.3.3][bincode1] | 3.8167 ms | 4.0619 ms | 1811011 | 1115281 | 1025627 | 9.9218 ms |
| [bitcode 0.6.0][bitcode] | 708.43 µs | 2.3160 ms | 948499 | 857321 | 837658 | 3.0749 ms |
| [borsh 1.3.1][borsh] | 2.8767 ms | 2.8910 ms | 1486162 | 1082357 | 1013550 | 9.7133 ms |
| [bson 2.9.0][bson] | 22.032 ms | 46.149 ms | 10030880 | 2833079 | 1600859 | 27.435 ms |
| [capnp 0.18.13][capnp] | 2.3302 ms | † | 2664040 | 1511895 | 1212087 | 14.123 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.2980 ms | 17.474 ms | 5878791 | 1655835 | 1431390 | 20.896 ms |
| [ciborium 0.2.2][ciborium] | 22.924 ms | 47.633 ms | 5878653 | 1655791 | 1431560 | 20.710 ms |
| [databuf 0.5.0][databuf] | 1.3469 ms | 3.5778 ms | 1288257 | 1037579 | 984337 | 8.5170 ms |
| [dlhn 0.1.6][dlhn] | 5.1515 ms | 7.0511 ms | 1279599 | 1052061 | 1021161 | 8.2672 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.2514 ms | † | 2273740 | 1408408 | 1235566 | 12.801 ms |
| [msgpacker 0.4.3][msgpacker] | 1.9246 ms | 4.5513 ms | 1424043 | 1128758 | 1110156 | 9.4028 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 31.031 ms | 15.504 ms | 1728519 | 1247642 | 1233323 | 11.810 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2919 ms | 2.8857 ms | 1770477 | 1108304 | 1029947 | 9.8679 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.0944 ms | 2.9941 ms | 1288257 | 1039269 | 986510 | 8.4514 ms |
| [postcard 1.0.8][postcard] | 1.8067 ms | 3.9379 ms | 1279599 | 1058243 | 1016738 | 8.3460 ms |
| [pot 3.0.0][pot] | 13.385 ms | 31.091 ms | 2544810 | 1447453 | 1268390 | 15.250 ms |
| [prost 0.12.4][prost] | <span title="encode">*5.8440 ms\**</span> <span title="populate + encode">*9.9167 ms\**</span> | 9.6306 ms | 1818378 | 1307777 | 1266311 | 11.482 ms |
| [rkyv 0.7.44][rkyv] | 1.0988 ms | <span title="unvalidated">*2.1672 ms\**</span> <span title="validated upfront with error">*2.8020 ms\**</span> | 2029080 | 1335117 | 1158855 | 12.100 ms |
| [rmp-serde 1.1.2][rmp-serde] | 8.4525 ms | 12.126 ms | 1703813 | 1231892 | 1200208 | 11.058 ms |
| [ron 0.8.1][ron] | 37.976 ms | 102.23 ms | 8476284 | 2181196 | 1783971 | 33.326 ms |
| [savefile 0.16.5][savefile] | 1.0211 ms | 2.6523 ms | 1750226 | 1101682 | 1027827 | 9.8367 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8979 ms | 4.4312 ms | 1288257 | 1037597 | 984356 | 8.6210 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.4296 ms | 21.499 ms | 5878653 | 1655791 | 1431560 | 20.865 ms |
| [serde_json 1.0.115][serde_json] | 22.502 ms | 29.866 ms | 9175594 | 2334253 | 1800713 | 33.570 ms |
| [simd-json 0.13.9][simd-json] | 11.592 ms | 26.138 ms | 9175594 | 2334253 | 1800713 | 34.963 ms |
| [speedy 0.8.7][speedy] | 713.08 µs | 2.5048 ms | 1546963 | 1093532 | 1013443 | 9.7084 ms |
| [wiring 0.1.6][wiring] | 4.4631 ms | 7.8874 ms | 1750210 | 1129857 | 1058906 | 10.172 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*65.731 µs\**</span> | <span title="unvalidated">*66.741 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8556 ns\**</span> | <span title="validated on-demand with panic">*626.58 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.736 ns\**</span> | <span title="validated on-demand with error">*1.0142 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4738 ns\**</span> <span title="validated upfront with error">*3.8456 ms\**</span> | <span title="unvalidated">*2.7109 µs\**</span> <span title="validated upfront with error">*3.8456 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2371 ns\**</span> <span title="validated upfront with error">*619.50 µs\**</span> | <span title="unvalidated">*356.69 ns\**</span> <span title="validated upfront with error">*621.92 µs\**</span> | 502.04 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*92.98%\**</span> | 31.78% | 60.88% | 65.76% | 21.40% |
| [alkahest 0.1.5][alkahest] | 68.17% | † | 50.90% | 69.47% | 69.67% | 26.79% |
| [bilrost 0.1006.0][bilrost] | <span title="encode">*9.90%\**</span> <span title="prepend">*17.95%\**</span> | 25.19% | 56.99% | 67.82% | 68.86% | 27.42% |
| [bincode 2.0.0-rc][bincode] | 69.95% | 58.52% | 69.11% | 78.55% | 80.75% | 31.46% |
| [bincode 1.3.3][bincode1] | 12.88% | 53.35% | 52.37% | 76.87% | 81.67% | 30.99% |
| [bitcode 0.6.0][bitcode] | 69.40% | 93.58% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 17.09% | 74.96% | 63.82% | 79.21% | 82.65% | 31.66% |
| [bson 2.9.0][bson] | 2.23% | 4.70% | 9.46% | 30.26% | 52.33% | 11.21% |
| [capnp 0.18.13][capnp] | 21.10% | † | 35.60% | 56.71% | 69.11% | 21.77% |
| [cbor4ii 0.3.2][cbor4ii] | 11.44% | 12.40% | 16.13% | 51.78% | 58.52% | 14.72% |
| [ciborium 0.2.2][ciborium] | 2.14% | 4.55% | 16.13% | 51.78% | 58.51% | 14.85% |
| [databuf 0.5.0][databuf] | 36.50% | 60.57% | 73.63% | 82.63% | 85.10% | 36.10% |
| [dlhn 0.1.6][dlhn] | 9.54% | 30.74% | 74.12% | 81.49% | 82.03% | 37.19% |
| [flatbuffers 23.5.26][flatbuffers] | 9.36% | † | 41.72% | 60.87% | 67.80% | 24.02% |
| [msgpacker 0.4.3][msgpacker] | 25.54% | 47.62% | 66.61% | 75.95% | 75.45% | 32.70% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.58% | 13.98% | 54.87% | 68.72% | 67.92% | 26.04% |
| [nanoserde 0.1.37][nanoserde] | 38.05% | 75.10% | 53.57% | 77.35% | 81.33% | 31.16% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 15.89% | 72.38% | 73.63% | 82.49% | 84.91% | 36.38% |
| [postcard 1.0.8][postcard] | 27.21% | 55.03% | 74.12% | 81.01% | 82.39% | 36.84% |
| [pot 3.0.0][pot] | 3.67% | 6.97% | 37.27% | 59.23% | 66.04% | 20.16% |
| [prost 0.12.4][prost] | <span title="encode">*8.41%\**</span> <span title="populate + encode">*4.96%\**</span> | 22.50% | 52.16% | 65.56% | 66.15% | 26.78% |
| [rkyv 0.7.44][rkyv] | 44.74% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.34%\**</span> | 46.75% | 64.21% | 72.28% | 25.41% |
| [rmp-serde 1.1.2][rmp-serde] | 5.82% | 17.87% | 55.67% | 69.59% | 69.79% | 27.81% |
| [ron 0.8.1][ron] | 1.29% | 2.12% | 11.19% | 39.31% | 46.95% | 9.23% |
| [savefile 0.16.5][savefile] | 48.15% | 81.71% | 54.19% | 77.82% | 81.50% | 31.26% |
| [serde_bare 0.5.0][serde_bare] | 10.04% | 48.91% | 73.63% | 82.63% | 85.10% | 35.67% |
| [serde_cbor 0.11.2][serde_cbor] | 5.21% | 10.08% | 16.13% | 51.78% | 58.51% | 14.74% |
| [serde_json 1.0.115][serde_json] | 2.18% | 7.26% | 10.34% | 36.73% | 46.52% | 9.16% |
| [simd-json 0.13.9][simd-json] | 4.24% | 8.29% | 10.34% | 36.73% | 46.52% | 8.79% |
| [speedy 0.8.7][speedy] | 68.94% | 86.52% | 61.31% | 78.40% | 82.65% | 31.67% |
| [wiring 0.1.6][wiring] | 11.02% | 27.48% | 54.19% | 75.88% | 79.11% | 30.23% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.53%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.67%\**</span> | <span title="validated on-demand with panic">*56.93%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*35.17%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.16%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bilrost]: https://crates.io/crates/bilrost/0.1006.0
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.0
[borsh]: https://crates.io/crates/borsh/1.3.1
[bson]: https://crates.io/crates/bson/2.9.0
[capnp]: https://crates.io/crates/capnp/0.18.13
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.2
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.6
[flatbuffers]: https://crates.io/crates/flatbuffers/23.5.26
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.9
[postcard]: https://crates.io/crates/postcard/1.0.8
[pot]: https://crates.io/crates/pot/3.0.0
[prost]: https://crates.io/crates/prost/0.12.4
[rkyv]: https://crates.io/crates/rkyv/0.7.44
[rmp-serde]: https://crates.io/crates/rmp-serde/1.1.2
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.16.5
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.115
[simd-json]: https://crates.io/crates/simd-json/0.13.9
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.1.6


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
