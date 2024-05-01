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

## Last updated: 2024-5-1 22:43:2

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
| [abomonation 0.7.3][abomonation] | 396.89 µs | <span title="unvalidated">*1.4259 ms\**</span> | 1705800 | 520083 | 413333 | 6.8709 ms |
| [alkahest 0.1.5][alkahest] | 182.66 µs | † | 1045784 | 454157 | 389424 | 5.9628 ms |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*748.85 µs\**</span> <span title="prepend">*721.06 µs\**</span> | 3.1539 ms | 874632 | 355446 | 311723 | 5.0874 ms |
| [bincode 2.0.0-rc][bincode] | 199.03 µs | 2.3731 ms | 741295 | 303944 | 257153 | 4.0257 ms |
| [bincode 1.3.3][bincode1] | 508.76 µs | 1.9790 ms | 1045784 | 373127 | 311761 | 4.8550 ms |
| [bitcode 0.6.0][bitcode] | 140.85 µs | 1.4572 ms | 703710 | 288826 | 229755 | 2.3215 ms |
| [borsh 1.3.1][borsh] | 530.04 µs | 2.2460 ms | 885780 | 362204 | 286514 | 4.4061 ms |
| [bson 2.9.0][bson] | 2.1733 ms | 6.7748 ms | 1924682 | 532821 | 376270 | 5.6684 ms |
| [capnp 0.18.13][capnp] | 465.60 µs | † | 1443216 | 513986 | 428649 | 6.8480 ms |
| [cbor4ii 0.3.2][cbor4ii] | 881.63 µs | 4.9518 ms | 1407835 | 403440 | 324081 | 4.5699 ms |
| [ciborium 0.2.2][ciborium] | 3.8349 ms | 9.9464 ms | 1407835 | 403440 | 324081 | 4.8172 ms |
| [databuf 0.5.0][databuf] | 254.38 µs | 1.9857 ms | 765778 | 311715 | 264630 | 4.0821 ms |
| [dlhn 0.1.6][dlhn] | 793.59 µs | 2.3602 ms | 724953 | 301446 | 253629 | 3.7064 ms |
| [flatbuffers 23.5.26][flatbuffers] | 1.3499 ms | † | 1276368 | 468539 | 388832 | 5.0917 ms |
| [msgpacker 0.4.3][msgpacker] | 1.0503 ms | 2.4610 ms | 764996 | 315291 | 264898 | 3.7032 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3082 ms | 3.9807 ms | 818669 | 332556 | 285514 | 4.3201 ms |
| [nanoserde 0.1.37][nanoserde] | 264.86 µs | 2.0043 ms | 1045784 | 373127 | 311761 | 4.5312 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 653.86 µs | 2.1605 ms | 765778 | 311743 | 264518 | 4.1404 ms |
| [postcard 1.0.8][postcard] | 385.94 µs | 2.1636 ms | 724953 | 302399 | 253747 | 3.8451 ms |
| [pot 3.0.0][pot] | 2.3080 ms | 6.2856 ms | 971922 | 372513 | 304122 | 4.4918 ms |
| [prost 0.12.4][prost] | <span title="encode">*962.91 µs\**</span> <span title="populate + encode">*2.4720 ms\**</span> | 3.3213 ms | 884628 | 363130 | 315494 | 5.1065 ms |
| [rkyv 0.7.44][rkyv] | 212.40 µs | <span title="unvalidated">*1.4220 ms\**</span> <span title="validated upfront with error">*1.9111 ms\**</span> | 1011488 | 383862 | 333545 | 4.7282 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.2085 ms | 3.3843 ms | 784997 | 325384 | 278219 | 4.1816 ms |
| [ron 0.8.1][ron] | 13.989 ms | 15.386 ms | 1607459 | 449158 | 349713 | 5.9405 ms |
| [savefile 0.16.5][savefile] | 200.22 µs | 2.0962 ms | 1045800 | 373139 | 311755 | 4.7269 ms |
| [serde_bare 0.5.0][serde_bare] | 660.82 µs | 2.0401 ms | 765778 | 311715 | 264630 | 3.9018 ms |
| [serde_cbor 0.11.2][serde_cbor] | 2.0091 ms | 4.7730 ms | 1407835 | 403440 | 324081 | 4.4611 ms |
| [serde_json 1.0.115][serde_json] | 3.7995 ms | 5.4105 ms | 1827461 | 470560 | 361090 | 5.5185 ms |
| [simd-json 0.13.9][simd-json] | 2.0464 ms | 4.5892 ms | 1827461 | 470560 | 361090 | 5.6100 ms |
| [speedy 0.8.7][speedy] | 191.38 µs | 1.7059 ms | 885780 | 362204 | 286514 | 4.2098 ms |
| [wiring 0.2.1][wiring] | 194.99 µs | 1.8949 ms | 2091568 | 674994 | 276295 | 3.9933 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*22.235 µs\**</span> | <span title="unvalidated">*39.758 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8136 ns\**</span> | <span title="validated on-demand with panic">*24.192 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*74.502 ns\**</span> | <span title="validated on-demand with error">*177.58 µs\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.3928 ns\**</span> <span title="validated upfront with error">*1.8310 ms\**</span> | <span title="unvalidated">*50.552 µs\**</span> <span title="validated upfront with error">*1.8534 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2186 ns\**</span> <span title="validated upfront with error">*483.30 µs\**</span> | <span title="unvalidated">*10.483 µs\**</span> <span title="validated upfront with error">*496.74 µs\**</span> | 9.5373 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 35.49% | <span title="unvalidated">*99.73%\**</span> | 41.25% | 55.53% | 55.59% | 33.79% |
| [alkahest 0.1.5][alkahest] | 77.11% | † | 67.29% | 63.60% | 59.00% | 38.93% |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*18.81%\**</span> <span title="prepend">*19.53%\**</span> | 45.09% | 80.46% | 81.26% | 73.70% | 45.63% |
| [bincode 2.0.0-rc][bincode] | 70.77% | 59.92% | 94.93% | 95.03% | 89.35% | 57.67% |
| [bincode 1.3.3][bincode1] | 27.68% | 71.85% | 67.29% | 77.41% | 73.70% | 47.82% |
| [bitcode 0.6.0][bitcode] | 100.00% | 97.58% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 26.57% | 63.31% | 79.45% | 79.74% | 80.19% | 52.69% |
| [bson 2.9.0][bson] | 6.48% | 20.99% | 36.56% | 54.21% | 61.06% | 40.96% |
| [capnp 0.18.13][capnp] | 30.25% | † | 48.76% | 56.19% | 53.60% | 33.90% |
| [cbor4ii 0.3.2][cbor4ii] | 15.98% | 28.72% | 49.99% | 71.59% | 70.89% | 50.80% |
| [ciborium 0.2.2][ciborium] | 3.67% | 14.30% | 49.99% | 71.59% | 70.89% | 48.19% |
| [databuf 0.5.0][databuf] | 55.37% | 71.61% | 91.89% | 92.66% | 86.82% | 56.87% |
| [dlhn 0.1.6][dlhn] | 17.75% | 60.25% | 97.07% | 95.81% | 90.59% | 62.63% |
| [flatbuffers 23.5.26][flatbuffers] | 10.43% | † | 55.13% | 61.64% | 59.09% | 45.59% |
| [msgpacker 0.4.3][msgpacker] | 13.41% | 57.78% | 91.99% | 91.61% | 86.73% | 62.69% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.65% | 35.72% | 85.96% | 86.85% | 80.47% | 53.74% |
| [nanoserde 0.1.37][nanoserde] | 53.18% | 70.95% | 67.29% | 77.41% | 73.70% | 51.23% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 21.54% | 65.82% | 91.89% | 92.65% | 86.86% | 56.07% |
| [postcard 1.0.8][postcard] | 36.50% | 65.72% | 97.07% | 95.51% | 90.54% | 60.38% |
| [pot 3.0.0][pot] | 6.10% | 22.62% | 72.40% | 77.53% | 75.55% | 51.68% |
| [prost 0.12.4][prost] | <span title="encode">*14.63%\**</span> <span title="populate + encode">*5.70%\**</span> | 42.81% | 79.55% | 79.54% | 72.82% | 45.46% |
| [rkyv 0.7.44][rkyv] | 66.31% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*74.41%\**</span> | 69.57% | 75.24% | 68.88% | 49.10% |
| [rmp-serde 1.1.2][rmp-serde] | 11.65% | 42.02% | 89.64% | 88.76% | 82.58% | 55.52% |
| [ron 0.8.1][ron] | 1.01% | 9.24% | 43.78% | 64.30% | 65.70% | 39.08% |
| [savefile 0.16.5][savefile] | 70.35% | 67.84% | 67.29% | 77.40% | 73.70% | 49.11% |
| [serde_bare 0.5.0][serde_bare] | 21.31% | 69.70% | 91.89% | 92.66% | 86.82% | 59.50% |
| [serde_cbor 0.11.2][serde_cbor] | 7.01% | 29.79% | 49.99% | 71.59% | 70.89% | 52.04% |
| [serde_json 1.0.115][serde_json] | 3.71% | 26.28% | 38.51% | 61.38% | 63.63% | 42.07% |
| [simd-json 0.13.9][simd-json] | 6.88% | 30.99% | 38.51% | 61.38% | 63.63% | 41.38% |
| [speedy 0.8.7][speedy] | 73.60% | 83.36% | 79.45% | 79.74% | 80.19% | 55.15% |
| [wiring 0.2.1][wiring] | 72.23% | 75.04% | 33.65% | 42.79% | 83.16% | 58.13% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.01%\**</span> | <span title="unvalidated">*26.37%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*67.19%\**</span> | <span title="validated on-demand with panic">*43.33%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.64%\**</span> | <span title="validated on-demand with error">*5.90%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*50.93%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.74%\**</span> <span title="validated upfront with error">*0.57%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.11%\**</span> | 100.00% |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 259.20 µs | <span title="unvalidated">*259.20 µs\**</span> | 6000024 | 5378513 | 5345890 | 7.4580 ms |
| [alkahest 0.1.5][alkahest] | 144.49 µs | † | 6000008 | 5378500 | 5345890 | 7.3264 ms |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*6.4614 ms\**</span> <span title="prepend">*8.3875 ms\**</span> | 10.210 ms | 8625005 | 6443961 | 6231572 | 67.633 ms |
| [bincode 2.0.0-rc][bincode] | 413.78 µs | 807.08 µs | 6000005 | 5378497 | 5345897 | 7.2359 ms |
| [bincode 1.3.3][bincode1] | 4.9667 ms | 4.4266 ms | 6000008 | 5378500 | 5345890 | 7.2610 ms |
| [bitcode 0.6.0][bitcode] | 1.3814 ms | 594.88 µs | 6000006 | 5182295 | 4923880 | 12.408 ms |
| [borsh 1.3.1][borsh] | 5.7457 ms | 4.4974 ms | 6000004 | 5378496 | 5345889 | 7.3261 ms |
| [bson 2.9.0][bson] | 44.274 ms | 76.062 ms | 23013911 | 9212089 | 7497811 | 106.16 ms |
| [capnp 0.18.13][capnp] | 5.4032 ms | † | 14000088 | 7130367 | 6051062 | 76.929 ms |
| [cbor4ii 0.3.2][cbor4ii] | 10.305 ms | 46.706 ms | 13125016 | 7524114 | 6757967 | 91.329 ms |
| [ciborium 0.2.2][ciborium] | 62.728 ms | 104.25 ms | 13122324 | 7524660 | 6759658 | 91.213 ms |
| [databuf 0.5.0][databuf] | 2.3402 ms | 5.1656 ms | 6000003 | 5378495 | 5345900 | 7.8404 ms |
| [dlhn 0.1.6][dlhn] | 6.1567 ms | 6.1084 ms | 6000003 | 5378495 | 5345900 | 7.8352 ms |
| [flatbuffers 23.5.26][flatbuffers] | 635.57 µs | † | 6000024 | 5378434 | 5345910 | 8.2614 ms |
| [msgpacker 0.4.3][msgpacker] | 18.726 ms | 8.3419 ms | 7500005 | 6058442 | 6014337 | 9.1896 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 116.81 ms | 25.787 ms | 8125037 | 6493484 | 6386940 | 69.983 ms |
| [nanoserde 0.1.37][nanoserde] | 1.1410 ms | 880.91 µs | 6000008 | 5378500 | 5345890 | 7.5005 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 4.7612 ms | 4.4793 ms | 6000004 | 5378496 | 5345889 | 7.4643 ms |
| [postcard 1.0.8][postcard] | 464.95 µs | 1.0257 ms | 6000003 | 5378495 | 5345900 | 7.5195 ms |
| [pot 3.0.0][pot] | 38.400 ms | 68.473 ms | 10122342 | 6814618 | 6852251 | 79.142 ms |
| [prost 0.12.4][prost] | <span title="encode">*7.7099 ms\**</span> <span title="populate + encode">*8.9102 ms\**</span> | 12.980 ms | 8750000 | 6665735 | 6421871 | 71.895 ms |
| [rkyv 0.7.44][rkyv] | 197.58 µs | <span title="unvalidated">*143.63 µs\**</span> <span title="validated upfront with error">*191.60 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.3876 ms |
| [rmp-serde 1.1.2][rmp-serde] | 13.284 ms | 18.399 ms | 8125006 | 6494876 | 6391037 | 65.646 ms |
| [ron 0.8.1][ron] | 170.91 ms | 244.27 ms | 22192885 | 8970395 | 8138755 | 145.22 ms |
| [savefile 0.16.5][savefile] | 258.56 µs | 264.30 µs | 6000024 | 5378518 | 5345893 | 7.6299 ms |
| [serde_bare 0.5.0][serde_bare] | 6.0346 ms | 4.1405 ms | 6000003 | 5378495 | 5345900 | 7.0724 ms |
| [serde_cbor 0.11.2][serde_cbor] | 34.998 ms | 42.701 ms | 13122324 | 7524660 | 6759658 | 91.972 ms |
| [serde_json 1.0.115][serde_json] | 86.865 ms | 86.672 ms | 26192883 | 9566084 | 8586741 | 146.87 ms |
| [simd-json 0.13.9][simd-json] | 53.404 ms | 72.123 ms | 26192883 | 9566084 | 8586741 | 151.53 ms |
| [speedy 0.8.7][speedy] | 258.75 µs | 259.61 µs | 6000004 | 5378496 | 5345889 | 7.4608 ms |
| [wiring 0.2.1][wiring] | 192.65 µs | 345.97 µs | 12000016 | 10757697 | 10691743 | 14.279 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*2.1130 ns\**</span> | <span title="unvalidated">*139.94 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8149 ns\**</span> | <span title="validated on-demand with panic">*76.002 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*103.87 ns\**</span> | <span title="validated on-demand with error">*2.1315 ms\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4167 ns\**</span> <span title="validated upfront with error">*37.269 ns\**</span> | <span title="unvalidated">*52.917 µs\**</span> <span title="validated upfront with error">*75.589 µs\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.1963 ns\**</span> <span title="validated upfront with error">*10.501 ns\**</span> | <span title="unvalidated">*47.513 µs\**</span> <span title="validated upfront with error">*37.947 µs\**</span> | 103.74 µs |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 55.74% | <span title="unvalidated">*55.41%\**</span> | 100.00% | 96.35% | 92.11% | 94.83% |
| [alkahest 0.1.5][alkahest] | 100.00% | † | 100.00% | 96.35% | 92.11% | 96.53% |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*2.24%\**</span> <span title="prepend">*1.72%\**</span> | 1.41% | 69.57% | 80.42% | 79.02% | 10.46% |
| [bincode 2.0.0-rc][bincode] | 34.92% | 17.80% | 100.00% | 96.35% | 92.11% | 97.74% |
| [bincode 1.3.3][bincode1] | 2.91% | 3.24% | 100.00% | 96.35% | 92.11% | 97.40% |
| [bitcode 0.6.0][bitcode] | 10.46% | 24.14% | 100.00% | 100.00% | 100.00% | 57.00% |
| [borsh 1.3.1][borsh] | 2.51% | 3.19% | 100.00% | 96.35% | 92.11% | 96.54% |
| [bson 2.9.0][bson] | 0.33% | 0.19% | 26.07% | 56.26% | 65.67% | 6.66% |
| [capnp 0.18.13][capnp] | 2.67% | † | 42.86% | 72.68% | 81.37% | 9.19% |
| [cbor4ii 0.3.2][cbor4ii] | 1.40% | 0.31% | 45.71% | 68.88% | 72.86% | 7.74% |
| [ciborium 0.2.2][ciborium] | 0.23% | 0.14% | 45.72% | 68.87% | 72.84% | 7.75% |
| [databuf 0.5.0][databuf] | 6.17% | 2.78% | 100.00% | 96.35% | 92.11% | 90.20% |
| [dlhn 0.1.6][dlhn] | 2.35% | 2.35% | 100.00% | 96.35% | 92.11% | 90.26% |
| [flatbuffers 23.5.26][flatbuffers] | 22.73% | † | 100.00% | 96.35% | 92.11% | 85.61% |
| [msgpacker 0.4.3][msgpacker] | 0.77% | 1.72% | 80.00% | 85.54% | 81.87% | 76.96% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.56% | 73.85% | 79.81% | 77.09% | 10.11% |
| [nanoserde 0.1.37][nanoserde] | 12.66% | 16.30% | 100.00% | 96.35% | 92.11% | 94.29% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.03% | 3.21% | 100.00% | 96.35% | 92.11% | 94.75% |
| [postcard 1.0.8][postcard] | 31.08% | 14.00% | 100.00% | 96.35% | 92.11% | 94.05% |
| [pot 3.0.0][pot] | 0.38% | 0.21% | 59.27% | 76.05% | 71.86% | 8.94% |
| [prost 0.12.4][prost] | <span title="encode">*1.87%\**</span> <span title="populate + encode">*1.62%\**</span> | 1.11% | 68.57% | 77.75% | 76.67% | 9.84% |
| [rkyv 0.7.44][rkyv] | 73.13% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*74.96%\**</span> | 100.00% | 96.35% | 92.11% | 95.73% |
| [rmp-serde 1.1.2][rmp-serde] | 1.09% | 0.78% | 73.85% | 79.79% | 77.04% | 10.77% |
| [ron 0.8.1][ron] | 0.08% | 0.06% | 27.04% | 57.77% | 60.50% | 4.87% |
| [savefile 0.16.5][savefile] | 55.88% | 54.34% | 100.00% | 96.35% | 92.11% | 92.69% |
| [serde_bare 0.5.0][serde_bare] | 2.39% | 3.47% | 100.00% | 96.35% | 92.11% | 100.00% |
| [serde_cbor 0.11.2][serde_cbor] | 0.41% | 0.34% | 45.72% | 68.87% | 72.84% | 7.69% |
| [serde_json 1.0.115][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.82% |
| [simd-json 0.13.9][simd-json] | 0.27% | 0.20% | 22.91% | 54.17% | 57.34% | 4.67% |
| [speedy 0.8.7][speedy] | 55.84% | 55.33% | 100.00% | 96.35% | 92.11% | 94.79% |
| [wiring 0.2.1][wiring] | 75.00% | 41.52% | 50.00% | 48.17% | 46.05% | 49.53% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*56.62%\**</span> | <span title="unvalidated">*27.12%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*65.92%\**</span> | <span title="validated on-demand with panic">*49.93%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.15%\**</span> | <span title="validated on-demand with error">*1.78%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.50%\**</span> <span title="validated upfront with error">*3.21%\**</span> | <span title="unvalidated">*71.71%\**</span> <span title="validated upfront with error">*50.20%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*11.39%\**</span> | <span title="unvalidated">*79.87%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 189.13 µs | <span title="unvalidated">*1.2864 ms\**</span> | 1290592 | 396581 | 340428 | 4.9405 ms |
| [alkahest 0.1.5][alkahest] | 215.46 µs | † | 667570 | 325484 | 320452 | 3.9212 ms |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*913.57 µs\**</span> <span title="prepend">*907.73 µs\**</span> | 3.0918 ms | 489348 | 281173 | 249546 | 2.9467 ms |
| [bincode 2.0.0-rc][bincode] | 265.28 µs | 2.0011 ms | 367413 | 221291 | 206273 | 2.4700 ms |
| [bincode 1.3.3][bincode1] | 556.06 µs | 1.7457 ms | 569975 | 240525 | 232423 | 2.8979 ms |
| [bitcode 0.6.0][bitcode] | 128.29 µs | 1.2352 ms | 327688 | 200947 | 182736 | 733.70 µs |
| [borsh 1.3.1][borsh] | 507.27 µs | 1.7852 ms | 446595 | 234236 | 210008 | 2.4124 ms |
| [bson 2.9.0][bson] | 2.8695 ms | 8.0250 ms | 1619653 | 502185 | 328399 | 4.8497 ms |
| [capnp 0.18.13][capnp] | 445.95 µs | † | 803896 | 335606 | 280851 | 3.7580 ms |
| [cbor4ii 0.3.2][cbor4ii] | 775.26 µs | 4.5674 ms | 1109831 | 344745 | 274514 | 3.8469 ms |
| [ciborium 0.2.2][ciborium] | 3.5371 ms | 9.1415 ms | 1109821 | 344751 | 274526 | 3.8313 ms |
| [databuf 0.5.0][databuf] | 317.76 µs | 1.6947 ms | 356311 | 213062 | 198488 | 2.3392 ms |
| [dlhn 0.1.6][dlhn] | 778.80 µs | 2.4976 ms | 366496 | 220600 | 205683 | 2.2955 ms |
| [flatbuffers 23.5.26][flatbuffers] | 3.2451 ms | † | 844168 | 345696 | 294015 | 3.8138 ms |
| [msgpacker 0.4.3][msgpacker] | 892.69 µs | 2.7641 ms | 391251 | 236877 | 220476 | 2.6233 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1451 ms | 3.8363 ms | 449745 | 252432 | 231110 | 2.7865 ms |
| [nanoserde 0.1.37][nanoserde] | 279.06 µs | 1.8608 ms | 567975 | 239930 | 232419 | 2.8690 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 635.31 µs | 1.9359 ms | 356311 | 212976 | 198524 | 2.3892 ms |
| [postcard 1.0.8][postcard] | 416.40 µs | 1.9089 ms | 367489 | 221913 | 207344 | 2.4792 ms |
| [pot 3.0.0][pot] | 2.2244 ms | 5.7910 ms | 599125 | 299158 | 247693 | 3.0222 ms |
| [prost 0.12.4][prost] | <span title="encode">*1.0920 ms\**</span> <span title="populate + encode">*2.7315 ms\**</span> | 3.5031 ms | 596811 | 305319 | 269310 | 3.4591 ms |
| [rkyv 0.7.44][rkyv] | 292.37 µs | <span title="unvalidated">*1.2343 ms\**</span> <span title="validated upfront with error">*1.7169 ms\**</span> | 596952 | 253967 | 220706 | 2.7055 ms |
| [rmp-serde 1.1.2][rmp-serde] | 1.3293 ms | 2.9953 ms | 424533 | 245214 | 226188 | 2.6833 ms |
| [ron 0.8.1][ron] | 8.2034 ms | 16.762 ms | 1465223 | 434935 | 343338 | 5.8482 ms |
| [savefile 0.16.5][savefile] | 218.31 µs | 1.7571 ms | 566991 | 239361 | 232010 | 2.7217 ms |
| [serde_bare 0.5.0][serde_bare] | 705.86 µs | 2.1442 ms | 356311 | 213062 | 198488 | 2.3895 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.7600 ms | 4.5726 ms | 1109821 | 344751 | 274526 | 3.6903 ms |
| [serde_json 1.0.115][serde_json] | 3.7057 ms | 6.6326 ms | 1623191 | 466527 | 359623 | 5.8205 ms |
| [simd-json 0.13.9][simd-json] | 2.1494 ms | 4.6101 ms | 1623191 | 466527 | 359623 | 5.9702 ms |
| [speedy 0.8.7][speedy] | 265.07 µs | 1.5919 ms | 449595 | 234970 | 210361 | 2.7660 ms |
| [wiring 0.2.1][wiring] | 198.34 µs | 1.7723 ms | 1133950 | 495111 | 225317 | 2.8952 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*35.922 µs\**</span> | <span title="unvalidated">*36.565 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8124 ns\**</span> | <span title="validated on-demand with panic">*4.5167 µs\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*75.105 ns\**</span> | <span title="validated on-demand with error">*422.70 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4081 ns\**</span> <span title="validated upfront with error">*2.1202 ms\**</span> | <span title="unvalidated">*1.3316 µs\**</span> <span title="validated upfront with error">*2.1162 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2024 ns\**</span> <span title="validated upfront with error">*473.33 µs\**</span> | <span title="unvalidated">*159.90 ns\**</span> <span title="validated upfront with error">*483.66 µs\**</span> | 966.66 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 67.83% | <span title="unvalidated">*95.95%\**</span> | 25.39% | 50.67% | 53.68% | 14.85% |
| [alkahest 0.1.5][alkahest] | 59.54% | † | 49.09% | 61.74% | 57.02% | 18.71% |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*14.04%\**</span> <span title="prepend">*14.13%\**</span> | 39.92% | 66.96% | 71.47% | 73.23% | 24.90% |
| [bincode 2.0.0-rc][bincode] | 48.36% | 61.68% | 89.19% | 90.81% | 88.59% | 29.70% |
| [bincode 1.3.3][bincode1] | 23.07% | 70.71% | 57.49% | 83.55% | 78.62% | 25.32% |
| [bitcode 0.6.0][bitcode] | 100.00% | 99.93% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 25.29% | 69.14% | 73.37% | 85.79% | 87.01% | 30.41% |
| [bson 2.9.0][bson] | 4.47% | 15.38% | 20.23% | 40.01% | 55.64% | 15.13% |
| [capnp 0.18.13][capnp] | 28.77% | † | 40.76% | 59.88% | 65.07% | 19.52% |
| [cbor4ii 0.3.2][cbor4ii] | 16.55% | 27.02% | 29.53% | 58.29% | 66.57% | 19.07% |
| [ciborium 0.2.2][ciborium] | 3.63% | 13.50% | 29.53% | 58.29% | 66.56% | 19.15% |
| [databuf 0.5.0][databuf] | 40.37% | 72.83% | 91.97% | 94.31% | 92.06% | 31.37% |
| [dlhn 0.1.6][dlhn] | 16.47% | 49.42% | 89.41% | 91.09% | 88.84% | 31.96% |
| [flatbuffers 23.5.26][flatbuffers] | 3.95% | † | 38.82% | 58.13% | 62.15% | 19.24% |
| [msgpacker 0.4.3][msgpacker] | 14.37% | 44.65% | 83.75% | 84.83% | 82.88% | 27.97% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.49% | 32.17% | 72.86% | 79.60% | 79.07% | 26.33% |
| [nanoserde 0.1.37][nanoserde] | 45.97% | 66.33% | 57.69% | 83.75% | 78.62% | 25.57% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 20.19% | 63.76% | 91.97% | 94.35% | 92.05% | 30.71% |
| [postcard 1.0.8][postcard] | 30.81% | 64.66% | 89.17% | 90.55% | 88.13% | 29.59% |
| [pot 3.0.0][pot] | 5.77% | 21.31% | 54.69% | 67.17% | 73.78% | 24.28% |
| [prost 0.12.4][prost] | <span title="encode">*11.75%\**</span> <span title="populate + encode">*4.70%\**</span> | 35.23% | 54.91% | 65.82% | 67.85% | 21.21% |
| [rkyv 0.7.44][rkyv] | 43.88% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*71.89%\**</span> | 54.89% | 79.12% | 82.80% | 27.12% |
| [rmp-serde 1.1.2][rmp-serde] | 9.65% | 41.21% | 77.19% | 81.95% | 80.79% | 27.34% |
| [ron 0.8.1][ron] | 1.56% | 7.36% | 22.36% | 46.20% | 53.22% | 12.55% |
| [savefile 0.16.5][savefile] | 58.77% | 70.25% | 57.79% | 83.95% | 78.76% | 26.96% |
| [serde_bare 0.5.0][serde_bare] | 18.17% | 57.56% | 91.97% | 94.31% | 92.06% | 30.71% |
| [serde_cbor 0.11.2][serde_cbor] | 7.29% | 26.99% | 29.53% | 58.29% | 66.56% | 19.88% |
| [serde_json 1.0.115][serde_json] | 3.46% | 18.61% | 20.19% | 43.07% | 50.81% | 12.61% |
| [simd-json 0.13.9][simd-json] | 5.97% | 26.77% | 20.19% | 43.07% | 50.81% | 12.29% |
| [speedy 0.8.7][speedy] | 48.40% | 77.54% | 72.89% | 85.52% | 86.87% | 26.53% |
| [wiring 0.2.1][wiring] | 64.68% | 69.64% | 28.90% | 40.59% | 81.10% | 25.34% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.44%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*66.34%\**</span> | <span title="validated on-demand with panic">*3.54%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.60%\**</span> | <span title="validated on-demand with error">*37.83%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*49.93%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.01%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | 100.00% |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 476.69 µs | <span title="unvalidated">*2.2754 ms\**</span> | 2984682 | 1406994 | 1270095 | 14.306 ms |
| [alkahest 0.1.5][alkahest] | 710.11 µs | † | 1863391 | 1234113 | 1202345 | 11.207 ms |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*4.8896 ms\**</span> <span title="prepend">*2.7144 ms\**</span> | 8.2818 ms | 1664428 | 1264167 | 1216472 | 11.072 ms |
| [bincode 2.0.0-rc][bincode] | 681.68 µs | 3.5490 ms | 1372381 | 1091486 | 1037296 | 8.8835 ms |
| [bincode 1.3.3][bincode1] | 3.5677 ms | 3.8544 ms | 1811011 | 1115281 | 1025627 | 9.7876 ms |
| [bitcode 0.6.0][bitcode] | 703.05 µs | 2.2420 ms | 948499 | 857321 | 837658 | 2.9688 ms |
| [borsh 1.3.1][borsh] | 2.7585 ms | 2.8210 ms | 1486162 | 1082357 | 1013550 | 9.3159 ms |
| [bson 2.9.0][bson] | 21.035 ms | 42.601 ms | 10030880 | 2833079 | 1600859 | 27.245 ms |
| [capnp 0.18.13][capnp] | 2.0925 ms | † | 2664040 | 1511895 | 1212087 | 13.961 ms |
| [cbor4ii 0.3.2][cbor4ii] | 4.1831 ms | 17.037 ms | 5878791 | 1655835 | 1431390 | 20.481 ms |
| [ciborium 0.2.2][ciborium] | 22.131 ms | 45.890 ms | 5878653 | 1655791 | 1431560 | 20.335 ms |
| [databuf 0.5.0][databuf] | 1.7704 ms | 3.4662 ms | 1288257 | 1037579 | 984337 | 8.4125 ms |
| [dlhn 0.1.6][dlhn] | 4.9382 ms | 6.1385 ms | 1279599 | 1052061 | 1021161 | 8.2148 ms |
| [flatbuffers 23.5.26][flatbuffers] | 5.0941 ms | † | 2273740 | 1408408 | 1235566 | 12.660 ms |
| [msgpacker 0.4.3][msgpacker] | 1.8799 ms | 4.5293 ms | 1424043 | 1128758 | 1110156 | 9.1850 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.378 ms | 15.453 ms | 1728519 | 1247642 | 1233323 | 11.579 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2840 ms | 2.8091 ms | 1770477 | 1108304 | 1029947 | 9.8661 ms |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 3.0757 ms | 3.0292 ms | 1288257 | 1039269 | 986510 | 8.3603 ms |
| [postcard 1.0.8][postcard] | 1.7510 ms | 3.8126 ms | 1279599 | 1058243 | 1016738 | 7.9162 ms |
| [pot 3.0.0][pot] | 13.359 ms | 29.812 ms | 2544810 | 1447453 | 1268390 | 15.099 ms |
| [prost 0.12.4][prost] | <span title="encode">*4.9844 ms\**</span> <span title="populate + encode">*8.8263 ms\**</span> | 9.4357 ms | 1818378 | 1307777 | 1266311 | 11.050 ms |
| [rkyv 0.7.44][rkyv] | 1.2757 ms | <span title="unvalidated">*2.1470 ms\**</span> <span title="validated upfront with error">*2.7243 ms\**</span> | 2029080 | 1335117 | 1158855 | 11.511 ms |
| [rmp-serde 1.1.2][rmp-serde] | 8.7720 ms | 11.869 ms | 1703813 | 1231892 | 1200208 | 10.836 ms |
| [ron 0.8.1][ron] | 37.053 ms | 91.638 ms | 8476284 | 2181196 | 1783971 | 32.218 ms |
| [savefile 0.16.5][savefile] | 985.87 µs | 2.5742 ms | 1750226 | 1101682 | 1027827 | 9.2557 ms |
| [serde_bare 0.5.0][serde_bare] | 4.5927 ms | 4.3738 ms | 1288257 | 1037597 | 984356 | 8.5813 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.1042 ms | 21.163 ms | 5878653 | 1655791 | 1431560 | 20.724 ms |
| [serde_json 1.0.115][serde_json] | 20.669 ms | 29.534 ms | 9175594 | 2334253 | 1800713 | 33.173 ms |
| [simd-json 0.13.9][simd-json] | 11.183 ms | 25.368 ms | 9175594 | 2334253 | 1800713 | 31.875 ms |
| [speedy 0.8.7][speedy] | 719.09 µs | 2.4865 ms | 1546963 | 1093532 | 1013443 | 9.7680 ms |
| [wiring 0.2.1][wiring] | 723.30 µs | 2.6283 ms | 3500420 | 2259435 | 1061377 | 10.288 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*63.493 µs\**</span> | <span title="unvalidated">*64.651 µs\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*1.8117 ns\**</span> | <span title="validated on-demand with panic">*612.40 ns\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*75.400 ns\**</span> | <span title="validated on-demand with error">*699.01 ns\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*2.4096 ns\**</span> <span title="validated upfront with error">*5.1318 ms\**</span> | <span title="unvalidated">*2.5657 µs\**</span> <span title="validated upfront with error">*5.0959 ms\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*1.2364 ns\**</span> <span title="validated upfront with error">*598.37 µs\**</span> | <span title="unvalidated">*362.48 ns\**</span> <span title="validated upfront with error">*596.95 µs\**</span> | 624.87 ns |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | 100.00% | <span title="unvalidated">*94.36%\**</span> | 31.78% | 60.93% | 65.95% | 20.75% |
| [alkahest 0.1.5][alkahest] | 67.13% | † | 50.90% | 69.47% | 69.67% | 26.49% |
| [bilrost 0.1007.0][bilrost] | <span title="encode">*9.75%\**</span> <span title="prepend">*17.56%\**</span> | 25.92% | 56.99% | 67.82% | 68.86% | 26.81% |
| [bincode 2.0.0-rc][bincode] | 69.93% | 60.50% | 69.11% | 78.55% | 80.75% | 33.42% |
| [bincode 1.3.3][bincode1] | 13.36% | 55.70% | 52.37% | 76.87% | 81.67% | 30.33% |
| [bitcode 0.6.0][bitcode] | 67.80% | 95.76% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.3.1][borsh] | 17.28% | 76.11% | 63.82% | 79.21% | 82.65% | 31.87% |
| [bson 2.9.0][bson] | 2.27% | 5.04% | 9.46% | 30.26% | 52.33% | 10.90% |
| [capnp 0.18.13][capnp] | 22.78% | † | 35.60% | 56.71% | 69.11% | 21.27% |
| [cbor4ii 0.3.2][cbor4ii] | 11.40% | 12.60% | 16.13% | 51.78% | 58.52% | 14.50% |
| [ciborium 0.2.2][ciborium] | 2.15% | 4.68% | 16.13% | 51.78% | 58.51% | 14.60% |
| [databuf 0.5.0][databuf] | 26.93% | 61.94% | 73.63% | 82.63% | 85.10% | 35.29% |
| [dlhn 0.1.6][dlhn] | 9.65% | 34.98% | 74.12% | 81.49% | 82.03% | 36.14% |
| [flatbuffers 23.5.26][flatbuffers] | 9.36% | † | 41.72% | 60.87% | 67.80% | 23.45% |
| [msgpacker 0.4.3][msgpacker] | 25.36% | 47.40% | 66.61% | 75.95% | 75.45% | 32.32% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.57% | 13.89% | 54.87% | 68.72% | 67.92% | 25.64% |
| [nanoserde 0.1.37][nanoserde] | 37.13% | 76.43% | 53.57% | 77.35% | 81.33% | 30.09% |
| [parity-scale-codec 3.6.9][parity-scale-codec] | 15.50% | 70.88% | 73.63% | 82.49% | 84.91% | 35.51% |
| [postcard 1.0.8][postcard] | 27.22% | 56.31% | 74.12% | 81.01% | 82.39% | 37.50% |
| [pot 3.0.0][pot] | 3.57% | 7.20% | 37.27% | 59.23% | 66.04% | 19.66% |
| [prost 0.12.4][prost] | <span title="encode">*9.56%\**</span> <span title="populate + encode">*5.40%\**</span> | 22.75% | 52.16% | 65.56% | 66.15% | 26.87% |
| [rkyv 0.7.44][rkyv] | 37.37% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*78.81%\**</span> | 46.75% | 64.21% | 72.28% | 25.79% |
| [rmp-serde 1.1.2][rmp-serde] | 5.43% | 18.09% | 55.67% | 69.59% | 69.79% | 27.40% |
| [ron 0.8.1][ron] | 1.29% | 2.34% | 11.19% | 39.31% | 46.95% | 9.21% |
| [savefile 0.16.5][savefile] | 48.35% | 83.40% | 54.19% | 77.82% | 81.50% | 32.08% |
| [serde_bare 0.5.0][serde_bare] | 10.38% | 49.09% | 73.63% | 82.63% | 85.10% | 34.60% |
| [serde_cbor 0.11.2][serde_cbor] | 5.24% | 10.15% | 16.13% | 51.78% | 58.51% | 14.33% |
| [serde_json 1.0.115][serde_json] | 2.31% | 7.27% | 10.34% | 36.73% | 46.52% | 8.95% |
| [simd-json 0.13.9][simd-json] | 4.26% | 8.46% | 10.34% | 36.73% | 46.52% | 9.31% |
| [speedy 0.8.7][speedy] | 66.29% | 86.35% | 61.31% | 78.40% | 82.65% | 30.39% |
| [wiring 0.2.1][wiring] | 65.90% | 81.69% | 27.10% | 37.94% | 78.92% | 28.86% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [abomonation 0.7.3][abomonation] | <span title="unvalidated">*0.00%\**</span> | <span title="unvalidated">*0.56%\**</span> | ‡ |
| [alkahest 0.1.5][alkahest] | <span title="validated on-demand with panic">*68.25%\**</span> | <span title="validated on-demand with panic">*59.19%\**</span> | ‡ |
| [capnp 0.18.13][capnp] | <span title="validated on-demand with error">*1.64%\**</span> | <span title="validated on-demand with error">*51.86%\**</span> | ‡ |
| [flatbuffers 23.5.26][flatbuffers] | <span title="unvalidated">*51.31%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*14.13%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.7.44][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | 100.00% |

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
