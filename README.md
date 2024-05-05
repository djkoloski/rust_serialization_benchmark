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

## Last updated: 2024-5-5 14:16:23

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
| [abomonation 0.7.3][abomonation] | 388.30 µs | <span title="unvalidated">*1.4553 ms\**</span> | 1705800 | 520085 | 413215 | 6.7601 ms |
| [alkahest 0.1.5][alkahest] | 194.88 µs | † | 1045784 | 454157 | 389424 | 6.0234 ms |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*781.46 µs\**</span> <span title="prepend">*729.54 µs\**</span> | 3.1793 ms | 874632 | 355446 | 311723 | 5.0678 ms |
| [bincode 2.0.0-rc][bincode] | 213.35 µs | 2.5771 ms | 741295 | 303944 | 257153 | 3.9887 ms |
| [bincode 1.3.3][bincode1] | 526.02 µs | 2.0283 ms | 1045784 | 373127 | 311761 | 4.8509 ms |
| [bitcode 0.6.0][bitcode] | 143.54 µs | 1.4848 ms | 703710 | 288826 | 229755 | 2.4313 ms |
| [borsh 1.3.1][borsh] | 545.68 µs | 2.3023 ms | 885780 | 362204 | 286514 | 4.5376 ms |
| [bson 2.9.0][bson] | 2.2942 ms | 7.1524 ms | 1924682 | 532821 | 376270 | 5.6447 ms |
| [capnp 0.18.13][capnp] | 560.36 µs | † | 1443216 | 513986 | 428649 | 6.7664 ms |
| [cbor4ii 0.3.2][cbor4ii] | 901.08 µs | 5.0874 ms | 1407835 | 403440 | 324081 | 4.8194 ms |
| [ciborium 0.2.2][ciborium] | 3.8650 ms | 10.127 ms | 1407835 | 403440 | 324081 | 4.8079 ms |
| [databuf 0.5.0][databuf] | 276.38 µs | 2.0724 ms | 765778 | 311715 | 264630 | 4.1529 ms |
| [dlhn 0.1.6][dlhn] | 801.55 µs | 2.5024 ms | 724953 | 301446 | 253629 | 3.8139 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.3596 ms | † | 1276368 | 468539 | 388832 | 5.3179 ms |
| [msgpacker 0.4.3][msgpacker] | 1.1249 ms | 2.5537 ms | 764996 | 315291 | 264898 | 3.9499 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.6261 ms | 4.1247 ms | 818669 | 332556 | 285514 | 4.4076 ms |
| [nanoserde 0.1.37][nanoserde] | 258.42 µs | 2.0675 ms | 1045784 | 373127 | 311761 | 4.5508 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 701.41 µs | 2.1929 ms | 765778 | 311743 | 264518 | 4.1486 ms |
| [postcard 1.0.8][postcard] | 418.01 µs | 2.2391 ms | 724953 | 302399 | 253747 | 3.8084 ms |
| [pot 3.0.0][pot] | 2.3888 ms | 6.5241 ms | 971922 | 372513 | 304122 | 4.6931 ms |
| [prost 0.12.4][prost] | <span title="encode">*997.10 µs\**</span> <span title="populate + encode">*2.5247 ms\**</span> | 3.3365 ms | 884628 | 363130 | 315494 | 5.1536 ms |
| [rkyv 0.7.44][rkyv] | 218.24 µs | <span title="unvalidated">*1.4689 ms\**</span> <span title="validated upfront with error">*2.0033 ms\**</span> | 1011488 | 383862 | 333545 | 5.2316 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3028 ms | 3.4644 ms | 784997 | 325384 | 278219 | 4.2048 ms |
| [ron 0.8.1][ron] | 14.280 ms | 17.159 ms | 1607459 | 449158 | 349713 | 6.0514 ms |
| [savefile 0.16.5][savefile] | 205.04 µs | 2.1047 ms | 1045800 | 373139 | 311755 | 4.7661 ms |
| [serde_bare 0.5.0][serde_bare] | 674.00 µs | 2.1207 ms | 765778 | 311715 | 264630 | 3.9158 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0219 ms | 5.1234 ms | 1407835 | 403440 | 324081 | 4.8201 ms |
| [serde_json 1.0.115][serde_json] | 3.7372 ms | 5.8615 ms | 1827461 | 470560 | 361090 | 5.6102 ms |
| [simd-json 0.13.9][simd-json] | 2.1384 ms | 4.6856 ms | 1827461 | 470560 | 361090 | 5.6261 ms |
| [speedy 0.8.7][speedy] | 197.07 µs | 1.7735 ms | 885780 | 362204 | 286514 | 4.2123 ms |
| [wiring 0.2.1][wiring] | 194.15 µs | 1.9633 ms | 1045784 | 337930 | 276188 | 3.9432 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.552 µs\**</span> | <span title="unvalidated">*38.974 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8548 ns\**</span> | <span title="validated on-demand with panic">*24.860 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*78.774 ns\**</span> | <span title="validated on-demand with error">*162.51 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4730 ns\**</span> <span title="validated upfront with error">*1.8537 ms\**</span> | <span title="unvalidated">*52.047 µs\**</span> <span title="validated upfront with error">*1.9622 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2364 ns\**</span> <span title="validated upfront with error">*525.61 µs\**</span> | <span title="unvalidated">*10.642 µs\**</span> <span title="validated upfront with error">*537.43 µs\**</span> | 9.6032 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 36.97% | <span title="unvalidated">*100.00%\**</span> | 41.25% | 55.53% | 55.60% | 35.97% |
| [alkahest 0.1.5][alkahest] | 73.66% | † | 67.29% | 63.60% | 59.00% | 40.36% |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*18.37%\**</span> <span title="prepend">*19.68%\**</span> | 45.77% | 80.46% | 81.26% | 73.70% | 47.98% |
| [bincode 2.0.0-rc][bincode] | 67.28% | 56.47% | 94.93% | 95.03% | 89.35% | 60.95% |
| [bincode 1.3.3][bincode1] | 27.29% | 71.75% | 67.29% | 77.41% | 73.70% | 50.12% |
| [bitcode 0.6.0][bitcode] | 100.00% | 98.01% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 26.30% | 63.21% | 79.45% | 79.74% | 80.19% | 53.58% |
| [bson 2.9.0][bson] | 6.26% | 20.35% | 36.56% | 54.21% | 61.06% | 43.07% |
| [capnp 0.18.13][capnp] | 25.62% | † | 48.76% | 56.19% | 53.60% | 35.93% |
| [cbor4ii 0.3.2][cbor4ii] | 15.93% | 28.61% | 49.99% | 71.59% | 70.89% | 50.45% |
| [ciborium 0.2.2][ciborium] | 3.71% | 14.37% | 49.99% | 71.59% | 70.89% | 50.57% |
| [databuf 0.5.0][databuf] | 51.94% | 70.22% | 91.89% | 92.66% | 86.82% | 58.54% |
| [dlhn 0.1.6][dlhn] | 17.91% | 58.16% | 97.07% | 95.81% | 90.59% | 63.75% |
| [flatbuffers 23.5.26][flatbuffers] | 10.56% | † | 55.13% | 61.64% | 59.09% | 45.72% |
| [msgpacker 0.4.3][msgpacker] | 12.76% | 56.99% | 91.99% | 91.61% | 86.73% | 61.55% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.55% | 35.28% | 85.96% | 86.85% | 80.47% | 55.16% |
| [nanoserde 0.1.37][nanoserde] | 55.55% | 70.39% | 67.29% | 77.41% | 73.70% | 53.43% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.46% | 66.36% | 91.89% | 92.65% | 86.86% | 58.61% |
| [postcard 1.0.8][postcard] | 34.34% | 64.99% | 97.07% | 95.51% | 90.54% | 63.84% |
| [pot 3.0.0][pot] | 6.01% | 22.31% | 72.40% | 77.53% | 75.55% | 51.81% |
| [prost 0.12.4][prost] | <span title="encode">*14.40%\**</span> <span title="populate + encode">*5.69%\**</span> | 43.62% | 79.55% | 79.54% | 72.82% | 47.18% |
| [rkyv 0.7.44][rkyv] | 65.77% | <span title="unvalidated">*99.07%\**</span> <span title="validated upfront with error">*72.65%\**</span> | 69.57% | 75.24% | 68.88% | 46.47% |
| [rmp-serde 1.1.2][rmp-serde] | 11.02% | 42.01% | 89.64% | 88.76% | 82.58% | 57.82% |
| [ron 0.8.1][ron] | 1.01% | 8.48% | 43.78% | 64.30% | 65.70% | 40.18% |
| [savefile 0.16.5][savefile] | 70.01% | 69.15% | 67.29% | 77.40% | 73.70% | 51.01% |
| [serde_bare 0.5.0][serde_bare] | 21.30% | 68.62% | 91.89% | 92.66% | 86.82% | 62.09% |
| [serde_cbor 0.11.2][serde_cbor] | 7.10% | 28.40% | 49.99% | 71.59% | 70.89% | 50.44% |
| [serde_json 1.0.115][serde_json] | 3.84% | 24.83% | 38.51% | 61.38% | 63.63% | 43.34% |
| [simd-json 0.13.9][simd-json] | 6.71% | 31.06% | 38.51% | 61.38% | 63.63% | 43.21% |
| [speedy 0.8.7][speedy] | 72.84% | 82.06% | 79.45% | 79.74% | 80.19% | 57.72% |
| [wiring 0.2.1][wiring] | 73.93% | 74.13% | 67.29% | 85.47% | 83.19% | 61.66% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*27.31%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.66%\**</span> | <span title="validated on-demand with panic">*42.81%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.57%\**</span> | <span title="validated on-demand with error">*6.55%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.45%\**</span> <span title="validated upfront with error">*0.54%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.98%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 237.31 µs | <span title="unvalidated">*238.69 µs\**</span> | 6000024 | 5378514 | 5345891 | 7.5669 ms |
| [alkahest 0.1.5][alkahest] | 148.76 µs | † | 6000008 | 5378500 | 5345890 | 7.5519 ms |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*6.6664 ms\**</span> <span title="prepend">*8.4840 ms\**</span> | 10.811 ms | 8625005 | 6443961 | 6231572 | 70.842 ms |
| [bincode 2.0.0-rc][bincode] | 423.57 µs | 826.88 µs | 6000005 | 5378497 | 5345897 | 7.5272 ms |
| [bincode 1.3.3][bincode1] | 5.1204 ms | 4.9154 ms | 6000008 | 5378500 | 5345890 | 7.4247 ms |
| [bitcode 0.6.0][bitcode] | 1.4281 ms | 595.24 µs | 6000006 | 5182295 | 4923880 | 12.597 ms |
| [borsh 1.3.1][borsh] | 6.1719 ms | 4.2969 ms | 6000004 | 5378496 | 5345889 | 8.1370 ms |
| [bson 2.9.0][bson] | 44.747 ms | 79.430 ms | 23013911 | 9212089 | 7497811 | 106.32 ms |
| [capnp 0.18.13][capnp] | 6.1265 ms | † | 14000088 | 7130367 | 6051062 | 80.604 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.358 ms | 48.051 ms | 13125016 | 7524114 | 6757967 | 89.390 ms |
| [ciborium 0.2.2][ciborium] | 68.655 ms | 106.46 ms | 13122324 | 7524660 | 6759658 | 90.317 ms |
| [databuf 0.5.0][databuf] | 2.4039 ms | 5.2776 ms | 6000003 | 5378495 | 5345900 | 8.2054 ms |
| [dlhn 0.1.6][dlhn] | 6.3619 ms | 5.9113 ms | 6000003 | 5378495 | 5345900 | 8.1883 ms |
| [flatbuffers 23.5.26][flatbuffers] | 675.16 µs | † | 6000024 | 5378434 | 5345910 | 7.9826 ms |
| [msgpacker 0.4.3][msgpacker] | 21.295 ms | 8.7475 ms | 7500005 | 6058442 | 6014337 | 10.214 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.08 ms | 27.191 ms | 8125037 | 6493484 | 6386940 | 67.833 ms |
| [nanoserde 0.1.37][nanoserde] | 1.6680 ms | 898.59 µs | 6000008 | 5378500 | 5345890 | 7.9059 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.8730 ms | 3.9882 ms | 6000004 | 5378496 | 5345889 | 7.7785 ms |
| [postcard 1.0.8][postcard] | 509.57 µs | 1.5993 ms | 6000003 | 5378495 | 5345900 | 8.0403 ms |
| [pot 3.0.0][pot] | 39.371 ms | 72.888 ms | 10122342 | 6814618 | 6852251 | 81.124 ms |
| [prost 0.12.4][prost] | <span title="encode">*8.0556 ms\**</span> <span title="populate + encode">*9.4856 ms\**</span> | 13.894 ms | 8750000 | 6665735 | 6421871 | 70.896 ms |
| [rkyv 0.7.44][rkyv] | 188.53 µs | <span title="unvalidated">*199.81 µs\**</span> <span title="validated upfront with error">*198.23 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.8378 ms |
| [rmp-serde 1.1.2][rmp-serde] | 13.957 ms | 19.252 ms | 8125006 | 6494876 | 6391037 | 106.80 ms |
| [ron 0.8.1][ron] | 172.32 ms | 260.47 ms | 22192885 | 8970395 | 8138755 | 148.14 ms |
| [savefile 0.16.5][savefile] | 238.76 µs | 237.40 µs | 6000024 | 5378518 | 5345893 | 8.8119 ms |
| [serde_bare 0.5.0][serde_bare] | 6.3359 ms | 4.1928 ms | 6000003 | 5378495 | 5345900 | 7.9721 ms |
| [serde_cbor 0.11.2][serde_cbor] | 35.183 ms | 44.959 ms | 13122324 | 7524660 | 6759658 | 90.122 ms |
| [serde_json 1.0.115][serde_json] | 89.807 ms | 89.601 ms | 26192883 | 9566084 | 8586741 | 155.22 ms |
| [simd-json 0.13.9][simd-json] | 54.047 ms | 74.398 ms | 26192883 | 9566084 | 8586741 | 153.13 ms |
| [speedy 0.8.7][speedy] | 239.54 µs | 237.56 µs | 6000004 | 5378496 | 5345889 | 7.7605 ms |
| [wiring 0.2.1][wiring] | 198.49 µs | 318.18 µs | 6000008 | 5378952 | 5345894 | 7.9539 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1640 ns\**</span> | <span title="unvalidated">*141.69 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8622 ns\**</span> | <span title="validated on-demand with panic">*77.321 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*111.15 ns\**</span> | <span title="validated on-demand with error">*2.1426 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4738 ns\**</span> <span title="validated upfront with error">*37.432 ns\**</span> | <span title="unvalidated">*54.120 µs\**</span> <span title="validated upfront with error">*77.381 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2367 ns\**</span> <span title="validated upfront with error">*10.533 ns\**</span> | <span title="unvalidated">*46.024 µs\**</span> <span title="validated upfront with error">*77.398 µs\**</span> | 105.56 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 62.69% | <span title="unvalidated">*83.05%\**</span> | 100.00% | 96.35% | 92.11% | 98.12% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 98.32% |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*2.23%\**</span> <span title="prepend">*1.75%\**</span> | 1.83% | 69.57% | 80.42% | 79.02% | 10.48% |
| [bincode 2.0.0-rc][bincode] | 35.12% | 23.97% | 100.00% | 96.35% | 92.11% | 98.64% |
| [bincode 1.3.3][bincode1] | 2.91% | 4.03% | 100.00% | 96.35% | 92.11% | 100.00% |
| [bitcode 0.6.0][bitcode] | 10.42% | 33.30% | 100.00% | 100.00% | 100.00% | 58.94% |
| [borsh 1.3.1][borsh] | 2.41% | 4.61% | 100.00% | 96.35% | 92.11% | 91.25% |
| [bson 2.9.0][bson] | 0.33% | 0.25% | 26.07% | 56.26% | 65.67% | 6.98% |
| [capnp 0.18.13][capnp] | 2.43% | † | 42.86% | 72.68% | 81.37% | 9.21% |
| [cbor4ii 0.3.2][cbor4ii] | 1.44% | 0.41% | 45.71% | 68.88% | 72.86% | 8.31% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.19% | 45.72% | 68.87% | 72.84% | 8.22% |
| [databuf 0.5.0][databuf] | 6.19% | 3.76% | 100.00% | 96.35% | 92.11% | 90.49% |
| [dlhn 0.1.6][dlhn] | 2.34% | 3.35% | 100.00% | 96.35% | 92.11% | 90.67% |
| [flatbuffers 23.5.26][flatbuffers] | 22.03% | † | 100.00% | 96.35% | 92.11% | 93.01% |
| [msgpacker 0.4.3][msgpacker] | 0.70% | 2.27% | 80.00% | 85.54% | 81.87% | 72.69% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.73% | 73.85% | 79.81% | 77.09% | 10.95% |
| [nanoserde 0.1.37][nanoserde] | 8.92% | 22.06% | 100.00% | 96.35% | 92.11% | 93.91% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.05% | 4.97% | 100.00% | 96.35% | 92.11% | 95.45% |
| [postcard 1.0.8][postcard] | 29.19% | 12.39% | 100.00% | 96.35% | 92.11% | 92.34% |
| [pot 3.0.0][pot] | 0.38% | 0.27% | 59.27% | 76.05% | 71.86% | 9.15% |
| [prost 0.12.4][prost] | <span title="encode">*1.85%\**</span> <span title="populate + encode">*1.57%\**</span> | 1.43% | 68.57% | 77.75% | 76.67% | 10.47% |
| [rkyv 0.7.44][rkyv] | 78.91% | <span title="unvalidated">*99.21%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 94.73% |
| [rmp-serde 1.1.2][rmp-serde] | 1.07% | 1.03% | 73.85% | 79.79% | 77.04% | 6.95% |
| [ron 0.8.1][ron] | 0.09% | 0.08% | 27.04% | 57.77% | 60.50% | 5.01% |
| [savefile 0.16.5][savefile] | 62.31% | 83.50% | 100.00% | 96.35% | 92.11% | 84.26% |
| [serde_bare 0.5.0][serde_bare] | 2.35% | 4.73% | 100.00% | 96.35% | 92.11% | 93.13% |
| [serde_cbor 0.11.2][serde_cbor] | 0.42% | 0.44% | 45.72% | 68.87% | 72.84% | 8.24% |
| [serde_json 1.0.115][serde_json] | 0.17% | 0.22% | 22.91% | 54.17% | 57.34% | 4.78% |
| [simd-json 0.13.9][simd-json] | 0.28% | 0.27% | 22.91% | 54.17% | 57.34% | 4.85% |
| [speedy 0.8.7][speedy] | 62.10% | 83.44% | 100.00% | 96.35% | 92.11% | 95.67% |
| [wiring 0.2.1][wiring] | 74.95% | 62.30% | 100.00% | 96.34% | 92.11% | 93.35% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*57.15%\**</span> | <span title="unvalidated">*32.48%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.41%\**</span> | <span title="validated on-demand with panic">*59.52%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.11%\**</span> | <span title="validated on-demand with error">*2.15%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*3.30%\**</span> | <span title="unvalidated">*85.04%\**</span> <span title="validated upfront with error">*59.48%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*11.74%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*59.46%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 190.30 µs | <span title="unvalidated">*1.3067 ms\**</span> | 1290592 | 396676 | 340554 | 5.2242 ms |
| [alkahest 0.1.5][alkahest] | 217.00 µs | † | 667570 | 325484 | 320452 | 3.9407 ms |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*919.84 µs\**</span> <span title="prepend">*933.43 µs\**</span> | 3.1748 ms | 489348 | 281173 | 249546 | 3.0936 ms |
| [bincode 2.0.0-rc][bincode] | 280.25 µs | 2.0781 ms | 367413 | 221291 | 206273 | 2.5093 ms |
| [bincode 1.3.3][bincode1] | 568.89 µs | 1.8111 ms | 569975 | 240525 | 232423 | 2.9136 ms |
| [bitcode 0.6.0][bitcode] | 127.67 µs | 1.2653 ms | 327688 | 200947 | 182736 | 749.09 µs |
| [borsh 1.3.1][borsh] | 554.66 µs | 1.8282 ms | 446595 | 234236 | 210008 | 2.5189 ms |
| [bson 2.9.0][bson] | 2.8671 ms | 8.2837 ms | 1619653 | 502185 | 328399 | 4.8488 ms |
| [capnp 0.18.13][capnp] | 455.97 µs | † | 803896 | 335606 | 280851 | 3.9503 ms |
| [cbor4ii 0.3.2][cbor4ii] | 806.83 µs | 4.7936 ms | 1109831 | 344745 | 274514 | 3.8123 ms |
| [ciborium 0.2.2][ciborium] | 3.6626 ms | 9.4787 ms | 1109821 | 344751 | 274526 | 3.7917 ms |
| [databuf 0.5.0][databuf] | 320.49 µs | 1.7279 ms | 356311 | 213062 | 198488 | 2.4021 ms |
| [dlhn 0.1.6][dlhn] | 782.46 µs | 2.6971 ms | 366496 | 220600 | 205683 | 2.4993 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.3549 ms | † | 844168 | 345696 | 294015 | 3.8784 ms |
| [msgpacker 0.4.3][msgpacker] | 859.11 µs | 2.8043 ms | 391251 | 236877 | 220476 | 2.6373 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2047 ms | 3.8967 ms | 449745 | 252432 | 231110 | 2.7897 ms |
| [nanoserde 0.1.37][nanoserde] | 280.65 µs | 1.8807 ms | 567975 | 239930 | 232419 | 2.9029 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 674.76 µs | 2.0027 ms | 356311 | 212976 | 198524 | 2.4033 ms |
| [postcard 1.0.8][postcard] | 432.82 µs | 1.9547 ms | 367489 | 221913 | 207344 | 2.4987 ms |
| [pot 3.0.0][pot] | 2.3221 ms | 6.0368 ms | 599125 | 299158 | 247693 | 3.1763 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.1301 ms\**</span> <span title="populate + encode">*2.8041 ms\**</span> | 3.5518 ms | 596811 | 305319 | 269310 | 3.5273 ms |
| [rkyv 0.7.44][rkyv] | 300.29 µs | <span title="unvalidated">*1.2559 ms\**</span> <span title="validated upfront with error">*1.7662 ms\**</span> | 596952 | 253967 | 220706 | 2.6958 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3556 ms | 3.0010 ms | 424533 | 245214 | 226188 | 2.7036 ms |
| [ron 0.8.1][ron] | 8.1539 ms | 17.714 ms | 1465223 | 434935 | 343338 | 5.8713 ms |
| [savefile 0.16.5][savefile] | 223.87 µs | 1.8300 ms | 566991 | 239361 | 232010 | 2.9037 ms |
| [serde_bare 0.5.0][serde_bare] | 701.43 µs | 2.2270 ms | 356311 | 213062 | 198488 | 2.4095 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8118 ms | 4.7233 ms | 1109821 | 344751 | 274526 | 3.8548 ms |
| [serde_json 1.0.115][serde_json] | 3.7645 ms | 6.8189 ms | 1623191 | 466527 | 359623 | 6.0339 ms |
| [simd-json 0.13.9][simd-json] | 2.2371 ms | 4.5517 ms | 1623191 | 466527 | 359623 | 6.0265 ms |
| [speedy 0.8.7][speedy] | 273.04 µs | 1.6501 ms | 449595 | 234970 | 210361 | 2.5257 ms |
| [wiring 0.2.1][wiring] | 203.15 µs | 1.8283 ms | 566975 | 247810 | 225259 | 2.9431 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*37.083 µs\**</span> | <span title="unvalidated">*37.931 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8618 ns\**</span> | <span title="validated on-demand with panic">*4.6303 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*78.812 ns\**</span> | <span title="validated on-demand with error">*437.77 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4739 ns\**</span> <span title="validated upfront with error">*2.1562 ms\**</span> | <span title="unvalidated">*1.3736 µs\**</span> <span title="validated upfront with error">*2.1561 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2368 ns\**</span> <span title="validated upfront with error">*498.64 µs\**</span> | <span title="unvalidated">*163.24 ns\**</span> <span title="validated upfront with error">*499.85 µs\**</span> | 953.85 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 67.09% | <span title="unvalidated">*96.11%\**</span> | 25.39% | 50.66% | 53.66% | 14.34% |
| [alkahest 0.1.5][alkahest] | 58.83% | † | 49.09% | 61.74% | 57.02% | 19.01% |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*13.88%\**</span> <span title="prepend">*13.68%\**</span> | 39.56% | 66.96% | 71.47% | 73.23% | 24.21% |
| [bincode 2.0.0-rc][bincode] | 45.56% | 60.44% | 89.19% | 90.81% | 88.59% | 29.85% |
| [bincode 1.3.3][bincode1] | 22.44% | 69.34% | 57.49% | 83.55% | 78.62% | 25.71% |
| [bitcode 0.6.0][bitcode] | 100.00% | 99.26% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 23.02% | 68.70% | 73.37% | 85.79% | 87.01% | 29.74% |
| [bson 2.9.0][bson] | 4.45% | 15.16% | 20.23% | 40.01% | 55.64% | 15.45% |
| [capnp 0.18.13][capnp] | 28.00% | † | 40.76% | 59.88% | 65.07% | 18.96% |
| [cbor4ii 0.3.2][cbor4ii] | 15.82% | 26.20% | 29.53% | 58.29% | 66.57% | 19.65% |
| [ciborium 0.2.2][ciborium] | 3.49% | 13.25% | 29.53% | 58.29% | 66.56% | 19.76% |
| [databuf 0.5.0][databuf] | 39.84% | 72.68% | 91.97% | 94.31% | 92.06% | 31.18% |
| [dlhn 0.1.6][dlhn] | 16.32% | 46.56% | 89.41% | 91.09% | 88.84% | 29.97% |
| [flatbuffers 23.5.26][flatbuffers] | 3.81% | † | 38.82% | 58.13% | 62.15% | 19.31% |
| [msgpacker 0.4.3][msgpacker] | 14.86% | 44.78% | 83.75% | 84.83% | 82.88% | 28.40% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.45% | 32.23% | 72.86% | 79.60% | 79.07% | 26.85% |
| [nanoserde 0.1.37][nanoserde] | 45.49% | 66.78% | 57.69% | 83.75% | 78.62% | 25.81% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 18.92% | 62.71% | 91.97% | 94.35% | 92.05% | 31.17% |
| [postcard 1.0.8][postcard] | 29.50% | 64.25% | 89.17% | 90.55% | 88.13% | 29.98% |
| [pot 3.0.0][pot] | 5.50% | 20.80% | 54.69% | 67.17% | 73.78% | 23.58% |
| [prost 0.12.4][prost] | <span title="encode">*11.30%\**</span> <span title="populate + encode">*4.55%\**</span> | 35.36% | 54.91% | 65.82% | 67.85% | 21.24% |
| [rkyv 0.7.44][rkyv] | 42.52% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.11%\**</span> | 54.89% | 79.12% | 82.80% | 27.79% |
| [rmp-serde 1.1.2][rmp-serde] | 9.42% | 41.85% | 77.19% | 81.95% | 80.79% | 27.71% |
| [ron 0.8.1][ron] | 1.57% | 7.09% | 22.36% | 46.20% | 53.22% | 12.76% |
| [savefile 0.16.5][savefile] | 57.03% | 68.63% | 57.79% | 83.95% | 78.76% | 25.80% |
| [serde_bare 0.5.0][serde_bare] | 18.20% | 56.39% | 91.97% | 94.31% | 92.06% | 31.09% |
| [serde_cbor 0.11.2][serde_cbor] | 7.05% | 26.59% | 29.53% | 58.29% | 66.56% | 19.43% |
| [serde_json 1.0.115][serde_json] | 3.39% | 18.42% | 20.19% | 43.07% | 50.81% | 12.41% |
| [simd-json 0.13.9][simd-json] | 5.71% | 27.59% | 20.19% | 43.07% | 50.81% | 12.43% |
| [speedy 0.8.7][speedy] | 46.76% | 76.11% | 72.89% | 85.52% | 86.87% | 29.66% |
| [wiring 0.2.1][wiring] | 62.85% | 68.69% | 57.80% | 81.09% | 81.12% | 25.45% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.43%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.43%\**</span> | <span title="validated on-demand with panic">*3.53%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.57%\**</span> | <span title="validated on-demand with error">*37.29%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*11.88%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 494.74 µs | <span title="unvalidated">*2.3138 ms\**</span> | 2984682 | 1406960 | 1270204 | 14.375 ms |
| [alkahest 0.1.5][alkahest] | 736.82 µs | † | 1863391 | 1234113 | 1202345 | 11.492 ms |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*5.0616 ms\**</span> <span title="prepend">*2.9598 ms\**</span> | 8.3336 ms | 1664428 | 1264167 | 1216472 | 11.787 ms |
| [bincode 2.0.0-rc][bincode] | 704.51 µs | 3.6795 ms | 1372381 | 1091486 | 1037296 | 9.1873 ms |
| [bincode 1.3.3][bincode1] | 3.7819 ms | 4.0190 ms | 1811011 | 1115281 | 1025627 | 9.9232 ms |
| [bitcode 0.6.0][bitcode] | 720.61 µs | 2.3067 ms | 948499 | 857321 | 837658 | 3.1392 ms |
| [borsh 1.3.1][borsh] | 2.8414 ms | 2.9373 ms | 1486162 | 1082357 | 1013550 | 9.4999 ms |
| [bson 2.9.0][bson] | 20.208 ms | 43.719 ms | 10030880 | 2833079 | 1600859 | 27.279 ms |
| [capnp 0.18.13][capnp] | 2.1595 ms | † | 2664040 | 1511895 | 1212087 | 14.081 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.2969 ms | 17.734 ms | 5878791 | 1655835 | 1431390 | 20.954 ms |
| [ciborium 0.2.2][ciborium] | 23.167 ms | 46.753 ms | 5878653 | 1655791 | 1431560 | 21.097 ms |
| [databuf 0.5.0][databuf] | 1.5985 ms | 3.5864 ms | 1288257 | 1037579 | 984337 | 8.5242 ms |
| [dlhn 0.1.6][dlhn] | 5.2027 ms | 6.2461 ms | 1279599 | 1052061 | 1021161 | 8.2980 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.1821 ms | † | 2273740 | 1408408 | 1235566 | 12.952 ms |
| [msgpacker 0.4.3][msgpacker] | 1.9257 ms | 4.5374 ms | 1424043 | 1128758 | 1110156 | 9.3866 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.786 ms | 15.580 ms | 1728519 | 1247642 | 1233323 | 11.711 ms |
| [nanoserde 0.1.37][nanoserde] | 1.5142 ms | 2.8971 ms | 1770477 | 1108304 | 1029947 | 10.032 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.1573 ms | 2.9936 ms | 1288257 | 1039269 | 986510 | 8.5351 ms |
| [postcard 1.0.8][postcard] | 1.8447 ms | 3.9237 ms | 1279599 | 1058243 | 1016738 | 8.3967 ms |
| [pot 3.0.0][pot] | 13.440 ms | 30.000 ms | 2544810 | 1447453 | 1268390 | 15.334 ms |
| [prost 0.12.4][prost] | <span title="encode">*5.0090 ms\**</span> <span title="populate + encode">*9.0471 ms\**</span> | 9.6005 ms | 1818378 | 1307777 | 1266311 | 11.576 ms |
| [rkyv 0.7.44][rkyv] | 1.2908 ms | <span title="unvalidated">*2.1606 ms\**</span> <span title="validated upfront with error">*2.7791 ms\**</span> | 2029080 | 1335117 | 1158855 | 12.329 ms |
| [rmp-serde 1.1.2][rmp-serde] | 8.5612 ms | 12.153 ms | 1703813 | 1231892 | 1200208 | 11.026 ms |
| [ron 0.8.1][ron] | 36.780 ms | 99.490 ms | 8476284 | 2181196 | 1783971 | 34.268 ms |
| [savefile 0.16.5][savefile] | 1.0226 ms | 2.6249 ms | 1750226 | 1101682 | 1027827 | 9.9529 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9549 ms | 4.4516 ms | 1288257 | 1037597 | 984356 | 8.6224 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.6169 ms | 21.475 ms | 5878653 | 1655791 | 1431560 | 21.123 ms |
| [serde_json 1.0.115][serde_json] | 20.185 ms | 31.425 ms | 9175594 | 2334253 | 1800713 | 34.284 ms |
| [simd-json 0.13.9][simd-json] | 11.518 ms | 26.311 ms | 9175594 | 2334253 | 1800713 | 34.031 ms |
| [speedy 0.8.7][speedy] | 738.68 µs | 2.5228 ms | 1546963 | 1093532 | 1013443 | 9.9670 ms |
| [wiring 0.2.1][wiring] | 744.35 µs | 2.6951 ms | 1750210 | 1129857 | 1058906 | 10.503 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*66.140 µs\**</span> | <span title="unvalidated">*67.275 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8551 ns\**</span> | <span title="validated on-demand with panic">*627.85 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*79.095 ns\**</span> | <span title="validated on-demand with error">*712.45 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4735 ns\**</span> <span title="validated upfront with error">*4.9928 ms\**</span> | <span title="unvalidated">*2.6522 µs\**</span> <span title="validated upfront with error">*4.9798 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2449 ns\**</span> <span title="validated upfront with error">*618.32 µs\**</span> | <span title="unvalidated">*489.37 ns\**</span> <span title="validated upfront with error">*619.70 µs\**</span> | 505.40 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*93.38%\**</span> | 31.78% | 60.93% | 65.95% | 21.84% |
| [alkahest 0.1.5][alkahest] | 67.15% | † | 50.90% | 69.47% | 69.67% | 27.32% |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*9.77%\**</span> <span title="prepend">*16.72%\**</span> | 25.93% | 56.99% | 67.82% | 68.86% | 26.63% |
| [bincode 2.0.0-rc][bincode] | 70.22% | 58.72% | 69.11% | 78.55% | 80.75% | 34.17% |
| [bincode 1.3.3][bincode1] | 13.08% | 53.76% | 52.37% | 76.87% | 81.67% | 31.63% |
| [bitcode 0.6.0][bitcode] | 68.66% | 93.67% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 17.41% | 73.56% | 63.82% | 79.21% | 82.65% | 33.04% |
| [bson 2.9.0][bson] | 2.45% | 4.94% | 9.46% | 30.26% | 52.33% | 11.51% |
| [capnp 0.18.13][capnp] | 22.91% | † | 35.60% | 56.71% | 69.11% | 22.29% |
| [cbor4ii 0.3.2][cbor4ii] | 11.51% | 12.18% | 16.13% | 51.78% | 58.52% | 14.98% |
| [ciborium 0.2.2][ciborium] | 2.14% | 4.62% | 16.13% | 51.78% | 58.51% | 14.88% |
| [databuf 0.5.0][databuf] | 30.95% | 60.24% | 73.63% | 82.63% | 85.10% | 36.83% |
| [dlhn 0.1.6][dlhn] | 9.51% | 34.59% | 74.12% | 81.49% | 82.03% | 37.83% |
| [flatbuffers 23.5.26][flatbuffers] | 9.55% | † | 41.72% | 60.87% | 67.80% | 24.24% |
| [msgpacker 0.4.3][msgpacker] | 25.69% | 47.62% | 66.61% | 75.95% | 75.45% | 33.44% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.61% | 13.87% | 54.87% | 68.72% | 67.92% | 26.81% |
| [nanoserde 0.1.37][nanoserde] | 32.67% | 74.58% | 53.57% | 77.35% | 81.33% | 31.29% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 15.67% | 72.17% | 73.63% | 82.49% | 84.91% | 36.78% |
| [postcard 1.0.8][postcard] | 26.82% | 55.07% | 74.12% | 81.01% | 82.39% | 37.39% |
| [pot 3.0.0][pot] | 3.68% | 7.20% | 37.27% | 59.23% | 66.04% | 20.47% |
| [prost 0.12.4][prost] | <span title="encode">*9.88%\**</span> <span title="populate + encode">*5.47%\**</span> | 22.51% | 52.16% | 65.56% | 66.15% | 27.12% |
| [rkyv 0.7.44][rkyv] | 38.33% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*77.74%\**</span> | 46.75% | 64.21% | 72.28% | 25.46% |
| [rmp-serde 1.1.2][rmp-serde] | 5.78% | 17.78% | 55.67% | 69.59% | 69.79% | 28.47% |
| [ron 0.8.1][ron] | 1.35% | 2.17% | 11.19% | 39.31% | 46.95% | 9.16% |
| [savefile 0.16.5][savefile] | 48.38% | 82.31% | 54.19% | 77.82% | 81.50% | 31.54% |
| [serde_bare 0.5.0][serde_bare] | 9.98% | 48.54% | 73.63% | 82.63% | 85.10% | 36.41% |
| [serde_cbor 0.11.2][serde_cbor] | 5.14% | 10.06% | 16.13% | 51.78% | 58.51% | 14.86% |
| [serde_json 1.0.115][serde_json] | 2.45% | 6.88% | 10.34% | 36.73% | 46.52% | 9.16% |
| [simd-json 0.13.9][simd-json] | 4.30% | 8.21% | 10.34% | 36.73% | 46.52% | 9.22% |
| [speedy 0.8.7][speedy] | 66.98% | 85.64% | 61.31% | 78.40% | 82.65% | 31.50% |
| [wiring 0.2.1][wiring] | 66.47% | 80.17% | 54.19% | 75.88% | 79.11% | 29.89% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.73%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*67.11%\**</span> | <span title="validated on-demand with panic">*77.94%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.57%\**</span> | <span title="validated on-demand with error">*68.69%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.33%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*18.45%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.08%\**</span> | 100.00% |

[abomonation]: https://crates.io/crates/abomonation/0.7.3
[alkahest]: https://crates.io/crates/alkahest/0.1.5
[bilrost]: https://crates.io/crates/bilrost/0.1007.0
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
[wiring]: https://crates.io/crates/wiring/0.2.1


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
