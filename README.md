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

## Last updated: 2024-11-4 23:36:44

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.84.0-nightly (b8c8287a2 2024-11-03)
binary: rustc
commit-hash: b8c8287a229cd79604aa84c25e1235fc78cd5f2e
commit-date: 2024-11-03
host: x86_64-unknown-linux-gnu
release: 1.84.0-nightly
LLVM version: 19.1.3
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
Vulnerability Spectre v2:           Mitigation; Retpolines; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
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
| [bilrost 0.1010.0][bilrost] | <span title="encode">*698.50 µs\**</span> <span title="prepend">*626.21 µs\**</span> | 3.2356 ms | 874632 | 355446 | 311723 | 5.8913 ms |
| [bincode 2.0.0-rc][bincode] | 359.43 µs | 2.4917 ms | 741295 | 303944 | 257153 | 3.9721 ms |
| [bincode 1.3.3][bincode1] | 522.18 µs | 2.4001 ms | 1045784 | 373127 | 311761 | 4.8907 ms |
| [bitcode 0.6.3][bitcode] | 138.63 µs | 1.4804 ms | 703710 | 288826 | 229755 | 2.6015 ms |
| [borsh 1.5.1][borsh] | 549.09 µs | 2.4444 ms | 885780 | 362204 | 286514 | 4.5678 ms |
| [capnp 0.19.7][capnp] | 456.22 µs | † | 1443216 | 513986 | 428649 | 6.8451 ms |
| [cbor4ii 0.3.3][cbor4ii] | 589.35 µs | 4.8469 ms | 1407835 | 403440 | 324081 | 5.1350 ms |
| [ciborium 0.2.2][ciborium] | 4.1384 ms | 12.222 ms | 1407835 | 403440 | 324081 | 5.3692 ms |
| [databuf 0.5.0][databuf] | 272.64 µs | 2.0181 ms | 765778 | 311715 | 264630 | 4.1506 ms |
| [dlhn 0.1.7][dlhn] | 737.93 µs | 2.5983 ms | 724953 | 301446 | 253629 | 3.8284 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0311 ms | † | 1276368 | 468539 | 388832 | 5.5982 ms |
| [msgpacker 0.4.3][msgpacker] | 1.1738 ms | 2.5810 ms | 764996 | 315291 | 264898 | 4.1837 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.5531 ms | 4.2894 ms | 818669 | 332556 | 285514 | 4.6684 ms |
| [nanoserde 0.1.37][nanoserde] | 266.41 µs | 2.1014 ms | 1045784 | 373127 | 311761 | 4.5693 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 649.92 µs | 2.2541 ms | 765778 | 311743 | 264518 | 4.2027 ms |
| [postcard 1.0.10][postcard] | 420.33 µs | 2.2383 ms | 724953 | 302399 | 253747 | 3.8453 ms |
| [pot 3.0.1][pot] | 2.4472 ms | 6.7452 ms | 971922 | 372513 | 304122 | 5.0420 ms |
| [prost 0.13.2][prost] | <span title="encode">*929.05 µs\**</span> <span title="populate + encode">*2.5967 ms\**</span> | 3.5845 ms | 884628 | 363130 | 315494 | 5.2597 ms |
| [rkyv 0.8.5][rkyv] | 264.55 µs | <span title="unvalidated">*1.6030 ms\**</span> <span title="validated upfront with error">*2.1723 ms\**</span> | 1011488 | 393526 | 326517 | 5.3003 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.3123 ms | 3.3108 ms | 784997 | 325384 | 278219 | 4.4553 ms |
| [ron 0.8.1][ron] | 12.127 ms | 15.527 ms | 1607459 | 449158 | 349713 | 6.3280 ms |
| [savefile 0.17.7][savefile] | 187.21 µs | 2.1982 ms | 1045800 | 373140 | 311777 | 4.8218 ms |
| [serde-brief 0.1.0][serde-brief] | 1.5051 ms | 5.7323 ms | 1584946 | 413733 | 341439 | 5.3606 ms |
| [serde_bare 0.5.0][serde_bare] | 682.42 µs | 2.2723 ms | 765778 | 311715 | 264630 | 3.8662 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.9802 ms | 4.9212 ms | 1407835 | 403440 | 324081 | 4.8718 ms |
| [serde_json 1.0.128][serde_json] | 4.1119 ms | 5.4994 ms | 1827461 | 470560 | 361090 | 5.6963 ms |
| [simd-json 0.13.10][simd-json] | 2.1759 ms | 4.6872 ms | 1827461 | 470560 | 361090 | 5.7182 ms |
| [speedy 0.8.7][speedy] | 199.28 µs | 1.7994 ms | 885780 | 362204 | 286514 | 4.2354 ms |
| [wiring 0.2.2][wiring] | 194.26 µs | 2.0111 ms | 1045784 | 337930 | 276188 | 3.9686 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*74.653 ns\**</span> | <span title="validated on-demand with error">*169.54 µs\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4798 ns\**</span> <span title="validated upfront with error">*2.0175 ms\**</span> | <span title="unvalidated">*50.286 µs\**</span> <span title="validated upfront with error">*2.0440 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2505 ns\**</span> <span title="validated upfront with error">*570.73 µs\**</span> | <span title="unvalidated">*10.581 µs\**</span> <span title="validated upfront with error">*587.01 µs\**</span> | <span title="unvalidated">*7.9317 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*19.85%\**</span> <span title="prepend">*22.14%\**</span> | 45.75% | 80.46% | 81.26% | 73.70% | 44.16% |
| [bincode 2.0.0-rc][bincode] | 38.57% | 59.41% | 94.93% | 95.03% | 89.35% | 65.49% |
| [bincode 1.3.3][bincode1] | 26.55% | 61.68% | 67.29% | 77.41% | 73.70% | 53.19% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 25.25% | 60.56% | 79.45% | 79.74% | 80.19% | 56.95% |
| [capnp 0.19.7][capnp] | 30.39% | † | 48.76% | 56.19% | 53.60% | 38.01% |
| [cbor4ii 0.3.3][cbor4ii] | 23.52% | 30.54% | 49.99% | 71.59% | 70.89% | 50.66% |
| [ciborium 0.2.2][ciborium] | 3.35% | 12.11% | 49.99% | 71.59% | 70.89% | 48.45% |
| [databuf 0.5.0][databuf] | 50.85% | 73.36% | 91.89% | 92.66% | 86.82% | 62.68% |
| [dlhn 0.1.7][dlhn] | 18.79% | 56.98% | 97.07% | 95.81% | 90.59% | 67.95% |
| [flatbuffers 24.3.25][flatbuffers] | 13.44% | † | 55.13% | 61.64% | 59.09% | 46.47% |
| [msgpacker 0.4.3][msgpacker] | 11.81% | 57.36% | 91.99% | 91.61% | 86.73% | 62.18% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.50% | 34.51% | 85.96% | 86.85% | 80.47% | 55.73% |
| [nanoserde 0.1.37][nanoserde] | 52.04% | 70.45% | 67.29% | 77.41% | 73.70% | 56.93% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 21.33% | 65.68% | 91.89% | 92.65% | 86.86% | 61.90% |
| [postcard 1.0.10][postcard] | 32.98% | 66.14% | 97.07% | 95.51% | 90.54% | 67.65% |
| [pot 3.0.1][pot] | 5.66% | 21.95% | 72.40% | 77.53% | 75.55% | 51.60% |
| [prost 0.13.2][prost] | <span title="encode">*14.92%\**</span> <span title="populate + encode">*5.34%\**</span> | 41.30% | 79.55% | 79.54% | 72.82% | 49.46% |
| [rkyv 0.8.5][rkyv] | 52.40% | <span title="unvalidated">*92.35%\**</span> <span title="validated upfront with error">*68.15%\**</span> | 69.57% | 73.39% | 70.37% | 49.08% |
| [rmp-serde 1.3.0][rmp-serde] | 10.56% | 44.71% | 89.64% | 88.76% | 82.58% | 58.39% |
| [ron 0.8.1][ron] | 1.14% | 9.53% | 43.78% | 64.30% | 65.70% | 41.11% |
| [savefile 0.17.7][savefile] | 74.05% | 67.35% | 67.29% | 77.40% | 73.69% | 53.95% |
| [serde-brief 0.1.0][serde-brief] | 9.21% | 25.83% | 44.40% | 69.81% | 67.29% | 48.53% |
| [serde_bare 0.5.0][serde_bare] | 20.31% | 65.15% | 91.89% | 92.66% | 86.82% | 67.29% |
| [serde_cbor 0.11.2][serde_cbor] | 7.00% | 30.08% | 49.99% | 71.59% | 70.89% | 53.40% |
| [serde_json 1.0.128][serde_json] | 3.37% | 26.92% | 38.51% | 61.38% | 63.63% | 45.67% |
| [simd-json 0.13.10][simd-json] | 6.37% | 31.58% | 38.51% | 61.38% | 63.63% | 45.50% |
| [speedy 0.8.7][speedy] | 69.57% | 82.27% | 79.45% | 79.74% | 80.19% | 61.42% |
| [wiring 0.2.2][wiring] | 71.36% | 73.61% | 67.29% | 85.47% | 83.19% | 65.55% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.68%\**</span> | <span title="validated on-demand with error">*6.24%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.43%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.04%\**</span> <span title="validated upfront with error">*0.52%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*1.80%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*6.6636 ms\**</span> <span title="prepend">*8.5503 ms\**</span> | 9.1340 ms | 8625005 | 6443961 | 6231572 | 72.344 ms |
| [bincode 2.0.0-rc][bincode] | 2.8621 ms | 1.0202 ms | 6000005 | 5378497 | 5345897 | 7.7182 ms |
| [bincode 1.3.3][bincode1] | 4.9006 ms | 5.8036 ms | 6000008 | 5378500 | 5345890 | 7.4831 ms |
| [bitcode 0.6.3][bitcode] | 1.4421 ms | 806.03 µs | 6000006 | 5182295 | 4923880 | 12.587 ms |
| [borsh 1.5.1][borsh] | 6.1241 ms | 4.1953 ms | 6000004 | 5378496 | 5345889 | 7.4386 ms |
| [capnp 0.19.7][capnp] | 5.3219 ms | † | 14000088 | 7130367 | 6051062 | 78.977 ms |
| [cbor4ii 0.3.3][cbor4ii] | 10.014 ms | 48.591 ms | 13125016 | 7524114 | 6757967 | 94.723 ms |
| [ciborium 0.2.2][ciborium] | 68.765 ms | 118.45 ms | 13122324 | 7524660 | 6759658 | 91.963 ms |
| [databuf 0.5.0][databuf] | 2.3983 ms | 5.2792 ms | 6000003 | 5378495 | 5345900 | 7.7154 ms |
| [dlhn 0.1.7][dlhn] | 6.4214 ms | 6.7573 ms | 6000003 | 5378495 | 5345900 | 7.7613 ms |
| [flatbuffers 24.3.25][flatbuffers] | 855.42 µs | † | 6000024 | 5378434 | 5345910 | 8.5761 ms |
| [msgpacker 0.4.3][msgpacker] | 18.687 ms | 5.2360 ms | 7500005 | 6058442 | 6014337 | 9.9539 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 127.28 ms | 32.854 ms | 8125037 | 6493484 | 6386940 | 71.836 ms |
| [nanoserde 0.1.37][nanoserde] | 1.3541 ms | 1.0685 ms | 6000008 | 5378500 | 5345890 | 7.7353 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 5.0760 ms | 4.5511 ms | 6000004 | 5378496 | 5345889 | 7.6358 ms |
| [postcard 1.0.10][postcard] | 507.76 µs | 1.2025 ms | 6000003 | 5378495 | 5345900 | 7.5803 ms |
| [pot 3.0.1][pot] | 41.837 ms | 75.850 ms | 10122342 | 6814618 | 6852251 | 91.376 ms |
| [prost 0.13.2][prost] | <span title="encode">*7.7785 ms\**</span> <span title="populate + encode">*8.9195 ms\**</span> | 13.175 ms | 8750000 | 6665735 | 6421871 | 73.689 ms |
| [rkyv 0.8.5][rkyv] | 238.69 µs | <span title="unvalidated">*150.11 µs\**</span> <span title="validated upfront with error">*149.32 µs\**</span> | 6000008 | 5378500 | 5345892 | 7.5156 ms |
| [rmp-serde 1.3.0][rmp-serde] | 15.853 ms | 18.250 ms | 8125006 | 6494876 | 6391037 | 71.465 ms |
| [ron 0.8.1][ron] | 174.81 ms | 237.38 ms | 22192885 | 8970395 | 8138755 | 146.89 ms |
| [savefile 0.17.7][savefile] | 244.11 µs | 245.04 µs | 6000024 | 5378513 | 5345893 | 8.1522 ms |
| [serde-brief 0.1.0][serde-brief] | 22.878 ms | 39.662 ms | 15750015 | 8024540 | 6816643 | 96.049 ms |
| [serde_bare 0.5.0][serde_bare] | 6.1713 ms | 4.7317 ms | 6000003 | 5378495 | 5345900 | 8.0037 ms |
| [serde_cbor 0.11.2][serde_cbor] | 33.977 ms | 47.845 ms | 13122324 | 7524660 | 6759658 | 98.134 ms |
| [serde_json 1.0.128][serde_json] | 88.331 ms | 86.233 ms | 26192883 | 9566084 | 8586741 | 151.81 ms |
| [simd-json 0.13.10][simd-json] | 53.246 ms | 73.087 ms | 26192883 | 9566084 | 8586741 | 151.90 ms |
| [speedy 0.8.7][speedy] | 238.67 µs | 585.49 µs | 6000004 | 5378496 | 5345889 | 8.1618 ms |
| [wiring 0.2.2][wiring] | 149.70 µs | 319.78 µs | 6000008 | 5378952 | 5345894 | 8.5669 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*105.84 ns\**</span> | <span title="validated on-demand with error">*2.1349 ms\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4744 ns\**</span> <span title="validated upfront with error">*40.100 ns\**</span> | <span title="unvalidated">*53.964 µs\**</span> <span title="validated upfront with error">*77.362 µs\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2371 ns\**</span> <span title="validated upfront with error">*5.2754 ns\**</span> | <span title="unvalidated">*48.319 µs\**</span> <span title="validated upfront with error">*38.770 µs\**</span> | <span title="unvalidated">*79.354 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*2.25%\**</span> <span title="prepend">*1.75%\**</span> | 1.63% | 69.57% | 80.42% | 79.02% | 10.28% |
| [bincode 2.0.0-rc][bincode] | 5.23% | 14.64% | 100.00% | 96.35% | 92.11% | 96.38% |
| [bincode 1.3.3][bincode1] | 3.05% | 2.57% | 100.00% | 96.35% | 92.11% | 99.41% |
| [bitcode 0.6.3][bitcode] | 10.38% | 18.53% | 100.00% | 100.00% | 100.00% | 59.10% |
| [borsh 1.5.1][borsh] | 2.44% | 3.56% | 100.00% | 96.35% | 92.11% | 100.00% |
| [capnp 0.19.7][capnp] | 2.81% | † | 42.86% | 72.68% | 81.37% | 9.42% |
| [cbor4ii 0.3.3][cbor4ii] | 1.49% | 0.31% | 45.71% | 68.88% | 72.86% | 7.85% |
| [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 45.72% | 68.87% | 72.84% | 8.09% |
| [databuf 0.5.0][databuf] | 6.24% | 2.83% | 100.00% | 96.35% | 92.11% | 96.41% |
| [dlhn 0.1.7][dlhn] | 2.33% | 2.21% | 100.00% | 96.35% | 92.11% | 95.84% |
| [flatbuffers 24.3.25][flatbuffers] | 17.50% | † | 100.00% | 96.35% | 92.11% | 86.74% |
| [msgpacker 0.4.3][msgpacker] | 0.80% | 2.85% | 80.00% | 85.54% | 81.87% | 74.73% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.45% | 73.85% | 79.81% | 77.09% | 10.35% |
| [nanoserde 0.1.37][nanoserde] | 11.06% | 13.97% | 100.00% | 96.35% | 92.11% | 96.16% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.95% | 3.28% | 100.00% | 96.35% | 92.11% | 97.42% |
| [postcard 1.0.10][postcard] | 29.48% | 12.42% | 100.00% | 96.35% | 92.11% | 98.13% |
| [pot 3.0.1][pot] | 0.36% | 0.20% | 59.27% | 76.05% | 71.86% | 8.14% |
| [prost 0.13.2][prost] | <span title="encode">*1.92%\**</span> <span title="populate + encode">*1.68%\**</span> | 1.13% | 68.57% | 77.75% | 76.67% | 10.09% |
| [rkyv 0.8.5][rkyv] | 62.72% | <span title="unvalidated">*99.47%\**</span> <span title="validated upfront with error">*100.00%\**</span> | 100.00% | 96.35% | 92.11% | 98.98% |
| [rmp-serde 1.3.0][rmp-serde] | 0.94% | 0.82% | 73.85% | 79.79% | 77.04% | 10.41% |
| [ron 0.8.1][ron] | 0.09% | 0.06% | 27.04% | 57.77% | 60.50% | 5.06% |
| [savefile 0.17.7][savefile] | 61.32% | 60.94% | 100.00% | 96.35% | 92.11% | 91.25% |
| [serde-brief 0.1.0][serde-brief] | 0.65% | 0.38% | 38.10% | 64.58% | 72.23% | 7.74% |
| [serde_bare 0.5.0][serde_bare] | 2.43% | 3.16% | 100.00% | 96.35% | 92.11% | 92.94% |
| [serde_cbor 0.11.2][serde_cbor] | 0.44% | 0.31% | 45.72% | 68.87% | 72.84% | 7.58% |
| [serde_json 1.0.128][serde_json] | 0.17% | 0.17% | 22.91% | 54.17% | 57.34% | 4.90% |
| [simd-json 0.13.10][simd-json] | 0.28% | 0.20% | 22.91% | 54.17% | 57.34% | 4.90% |
| [speedy 0.8.7][speedy] | 62.72% | 25.50% | 100.00% | 96.35% | 92.11% | 91.14% |
| [wiring 0.2.2][wiring] | 100.00% | 46.69% | 100.00% | 96.34% | 92.11% | 86.83% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.17%\**</span> | <span title="validated on-demand with error">*1.82%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*3.09%\**</span> | <span title="unvalidated">*71.84%\**</span> <span title="validated upfront with error">*50.12%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*23.45%\**</span> | <span title="unvalidated">*80.24%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*956.15 µs\**</span> <span title="prepend">*846.30 µs\**</span> | 3.2173 ms | 489348 | 281173 | 249546 | 3.0684 ms |
| [bincode 2.0.0-rc][bincode] | 332.51 µs | 2.1307 ms | 367413 | 221291 | 206273 | 2.5057 ms |
| [bincode 1.3.3][bincode1] | 590.20 µs | 1.8495 ms | 569975 | 240525 | 232423 | 2.8948 ms |
| [bitcode 0.6.3][bitcode] | 132.64 µs | 1.2660 ms | 327688 | 200947 | 182736 | 762.32 µs |
| [borsh 1.5.1][borsh] | 549.87 µs | 1.8444 ms | 446595 | 234236 | 210008 | 2.4694 ms |
| [capnp 0.19.7][capnp] | 462.49 µs | † | 803896 | 335606 | 280851 | 3.9281 ms |
| [cbor4ii 0.3.3][cbor4ii] | 784.17 µs | 4.6461 ms | 1109831 | 344745 | 274514 | 3.8443 ms |
| [ciborium 0.2.2][ciborium] | 3.7028 ms | 10.397 ms | 1109821 | 344751 | 274526 | 3.8450 ms |
| [databuf 0.5.0][databuf] | 326.68 µs | 1.7623 ms | 356311 | 213062 | 198488 | 2.3799 ms |
| [dlhn 0.1.7][dlhn] | 805.07 µs | 2.6167 ms | 366496 | 220600 | 205683 | 2.4958 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.3027 ms | † | 844168 | 345696 | 294015 | 3.9107 ms |
| [msgpacker 0.4.3][msgpacker] | 917.07 µs | 2.8234 ms | 391251 | 236877 | 220476 | 2.6238 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3496 ms | 3.9511 ms | 449745 | 252432 | 231110 | 2.8342 ms |
| [nanoserde 0.1.37][nanoserde] | 308.54 µs | 1.8994 ms | 567975 | 239930 | 232419 | 2.8804 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 600.16 µs | 1.9765 ms | 356311 | 212976 | 198524 | 2.3963 ms |
| [postcard 1.0.10][postcard] | 443.62 µs | 1.9896 ms | 367489 | 221913 | 207344 | 2.4758 ms |
| [pot 3.0.1][pot] | 2.4611 ms | 6.0389 ms | 599125 | 299158 | 247693 | 3.1764 ms |
| [prost 0.13.2][prost] | <span title="encode">*1.2450 ms\**</span> <span title="populate + encode">*2.9574 ms\**</span> | 3.5215 ms | 596811 | 305319 | 269310 | 3.4703 ms |
| [rkyv 0.8.5][rkyv] | 337.18 µs | <span title="unvalidated">*1.5118 ms\**</span> <span title="validated upfront with error">*2.0260 ms\**</span> | 603776 | 254776 | 220087 | 2.7852 ms |
| [rmp-serde 1.3.0][rmp-serde] | 1.4308 ms | 3.0412 ms | 424533 | 245214 | 226188 | 2.6972 ms |
| [ron 0.8.1][ron] | 7.1032 ms | 16.341 ms | 1465223 | 434935 | 343338 | 5.9049 ms |
| [savefile 0.17.7][savefile] | 211.67 µs | 1.8514 ms | 566991 | 239361 | 232013 | 2.8855 ms |
| [serde-brief 0.1.0][serde-brief] | 1.3659 ms | 5.2834 ms | 1276014 | 373898 | 293679 | 4.0709 ms |
| [serde_bare 0.5.0][serde_bare] | 747.89 µs | 2.3256 ms | 356311 | 213062 | 198488 | 2.3908 ms |
| [serde_cbor 0.11.2][serde_cbor] | 1.8585 ms | 4.6814 ms | 1109821 | 344751 | 274526 | 3.8568 ms |
| [serde_json 1.0.128][serde_json] | 3.9913 ms | 6.4631 ms | 1623191 | 466527 | 359623 | 6.0955 ms |
| [simd-json 0.13.10][simd-json] | 2.2427 ms | 4.5292 ms | 1623191 | 466527 | 359623 | 6.0373 ms |
| [speedy 0.8.7][speedy] | 281.61 µs | 1.6044 ms | 449595 | 234970 | 210361 | 2.4787 ms |
| [wiring 0.2.2][wiring] | 221.12 µs | 1.8093 ms | 566975 | 247810 | 225259 | 2.8966 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*74.556 ns\**</span> | <span title="validated on-demand with error">*420.03 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4741 ns\**</span> <span title="validated upfront with error">*2.2245 ms\**</span> | <span title="unvalidated">*1.3562 µs\**</span> <span title="validated upfront with error">*2.4888 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2374 ns\**</span> <span title="validated upfront with error">*511.42 µs\**</span> | <span title="unvalidated">*239.62 ns\**</span> <span title="validated upfront with error">*500.70 µs\**</span> | <span title="unvalidated">*716.06 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*13.87%\**</span> <span title="prepend">*15.67%\**</span> | 39.35% | 66.96% | 71.47% | 73.23% | 24.84% |
| [bincode 2.0.0-rc][bincode] | 39.89% | 59.42% | 89.19% | 90.81% | 88.59% | 30.42% |
| [bincode 1.3.3][bincode1] | 22.47% | 68.45% | 57.49% | 83.55% | 78.62% | 26.33% |
| [bitcode 0.6.3][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 24.12% | 68.64% | 73.37% | 85.79% | 87.01% | 30.87% |
| [capnp 0.19.7][capnp] | 28.68% | † | 40.76% | 59.88% | 65.07% | 19.41% |
| [cbor4ii 0.3.3][cbor4ii] | 16.91% | 27.25% | 29.53% | 58.29% | 66.57% | 19.83% |
| [ciborium 0.2.2][ciborium] | 3.58% | 12.18% | 29.53% | 58.29% | 66.56% | 19.83% |
| [databuf 0.5.0][databuf] | 40.60% | 71.84% | 91.97% | 94.31% | 92.06% | 32.03% |
| [dlhn 0.1.7][dlhn] | 16.48% | 48.38% | 89.41% | 91.09% | 88.84% | 30.54% |
| [flatbuffers 24.3.25][flatbuffers] | 4.02% | † | 38.82% | 58.13% | 62.15% | 19.49% |
| [msgpacker 0.4.3][msgpacker] | 14.46% | 44.84% | 83.75% | 84.83% | 82.88% | 29.05% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.48% | 32.04% | 72.86% | 79.60% | 79.07% | 26.90% |
| [nanoserde 0.1.37][nanoserde] | 42.99% | 66.65% | 57.69% | 83.75% | 78.62% | 26.47% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 22.10% | 64.05% | 91.97% | 94.35% | 92.05% | 31.81% |
| [postcard 1.0.10][postcard] | 29.90% | 63.63% | 89.17% | 90.55% | 88.13% | 30.79% |
| [pot 3.0.1][pot] | 5.39% | 20.96% | 54.69% | 67.17% | 73.78% | 24.00% |
| [prost 0.13.2][prost] | <span title="encode">*10.65%\**</span> <span title="populate + encode">*4.49%\**</span> | 35.95% | 54.91% | 65.82% | 67.85% | 21.97% |
| [rkyv 0.8.5][rkyv] | 39.34% | <span title="unvalidated">*83.74%\**</span> <span title="validated upfront with error">*62.49%\**</span> | 54.27% | 78.87% | 83.03% | 27.37% |
| [rmp-serde 1.3.0][rmp-serde] | 9.27% | 41.63% | 77.19% | 81.95% | 80.79% | 28.26% |
| [ron 0.8.1][ron] | 1.87% | 7.75% | 22.36% | 46.20% | 53.22% | 12.91% |
| [savefile 0.17.7][savefile] | 62.66% | 68.38% | 57.79% | 83.95% | 78.76% | 26.42% |
| [serde-brief 0.1.0][serde-brief] | 9.71% | 23.96% | 25.68% | 53.74% | 62.22% | 18.73% |
| [serde_bare 0.5.0][serde_bare] | 17.74% | 54.44% | 91.97% | 94.31% | 92.06% | 31.89% |
| [serde_cbor 0.11.2][serde_cbor] | 7.14% | 27.04% | 29.53% | 58.29% | 66.56% | 19.77% |
| [serde_json 1.0.128][serde_json] | 3.32% | 19.59% | 20.19% | 43.07% | 50.81% | 12.51% |
| [simd-json 0.13.10][simd-json] | 5.91% | 27.95% | 20.19% | 43.07% | 50.81% | 12.63% |
| [speedy 0.8.7][speedy] | 47.10% | 78.91% | 72.89% | 85.52% | 86.87% | 30.75% |
| [wiring 0.2.2][wiring] | 59.99% | 69.97% | 57.80% | 81.09% | 81.12% | 26.32% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*57.05%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.67%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*4.4452 ms\**</span> <span title="prepend">*2.4551 ms\**</span> | 8.5307 ms | 1664428 | 1264167 | 1216472 | 10.994 ms |
| [bincode 2.0.0-rc][bincode] | 1.4224 ms | 3.6393 ms | 1372381 | 1091486 | 1037296 | 8.8427 ms |
| [bincode 1.3.3][bincode1] | 3.7645 ms | 4.2802 ms | 1811011 | 1115281 | 1025627 | 9.7781 ms |
| [bitcode 0.6.3][bitcode] | 703.53 µs | 2.3197 ms | 948499 | 857321 | 837658 | 3.1612 ms |
| [borsh 1.5.1][borsh] | 2.8129 ms | 2.8549 ms | 1486162 | 1082357 | 1013550 | 9.5717 ms |
| [capnp 0.19.7][capnp] | 2.1674 ms | † | 2664040 | 1511895 | 1212087 | 13.806 ms |
| [cbor4ii 0.3.3][cbor4ii] | 3.2351 ms | 17.439 ms | 5878791 | 1655835 | 1431390 | 20.591 ms |
| [ciborium 0.2.2][ciborium] | 23.372 ms | 54.463 ms | 5878653 | 1655791 | 1431560 | 20.724 ms |
| [databuf 0.5.0][databuf] | 1.2707 ms | 3.7162 ms | 1288257 | 1037579 | 984337 | 8.4515 ms |
| [dlhn 0.1.7][dlhn] | 4.8860 ms | 7.5668 ms | 1279599 | 1052061 | 1021161 | 8.1009 ms |
| [flatbuffers 24.3.25][flatbuffers] | 5.2771 ms | † | 2273740 | 1408408 | 1235566 | 12.658 ms |
| [msgpacker 0.4.3][msgpacker] | 2.7729 ms | 6.6024 ms | 1424043 | 1128758 | 1110156 | 9.4264 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.813 ms | 17.514 ms | 1728519 | 1247642 | 1233323 | 11.561 ms |
| [nanoserde 0.1.37][nanoserde] | 1.2295 ms | 2.9181 ms | 1770477 | 1108304 | 1029947 | 10.014 ms |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 2.8216 ms | 3.1301 ms | 1288257 | 1039269 | 986510 | 8.5630 ms |
| [postcard 1.0.10][postcard] | 1.9181 ms | 4.2494 ms | 1279599 | 1058243 | 1016738 | 8.2941 ms |
| [pot 3.0.1][pot] | 13.900 ms | 29.841 ms | 2544810 | 1447453 | 1268390 | 15.080 ms |
| [prost 0.13.2][prost] | <span title="encode">*5.2813 ms\**</span> <span title="populate + encode">*9.1632 ms\**</span> | 8.4194 ms | 1818378 | 1307777 | 1266311 | 11.648 ms |
| [rkyv 0.8.5][rkyv] | 1.0203 ms | <span title="unvalidated">*2.1646 ms\**</span> <span title="validated upfront with error">*2.5860 ms\**</span> | 2029080 | 1351984 | 1183990 | 12.265 ms |
| [rmp-serde 1.3.0][rmp-serde] | 10.064 ms | 10.940 ms | 1703813 | 1231892 | 1200208 | 10.874 ms |
| [ron 0.8.1][ron] | 36.014 ms | 89.821 ms | 8476284 | 2181196 | 1783971 | 33.458 ms |
| [savefile 0.17.7][savefile] | 809.12 µs | 2.7223 ms | 1750226 | 1101682 | 1027828 | 10.027 ms |
| [serde-brief 0.1.0][serde-brief] | 6.6090 ms | 21.259 ms | 6796949 | 1754624 | 1533223 | 22.717 ms |
| [serde_bare 0.5.0][serde_bare] | 4.7388 ms | 4.7526 ms | 1288257 | 1037597 | 984356 | 8.5428 ms |
| [serde_cbor 0.11.2][serde_cbor] | 9.6980 ms | 20.936 ms | 5878653 | 1655791 | 1431560 | 20.501 ms |
| [serde_json 1.0.128][serde_json] | 21.736 ms | 29.690 ms | 9175594 | 2334253 | 1800713 | 33.383 ms |
| [simd-json 0.13.10][simd-json] | 11.644 ms | 26.770 ms | 9175594 | 2334253 | 1800713 | 33.486 ms |
| [speedy 0.8.7][speedy] | 710.74 µs | 2.4039 ms | 1546963 | 1093532 | 1013443 | 9.7296 ms |
| [wiring 0.2.2][wiring] | 695.66 µs | 2.7367 ms | 1750210 | 1129857 | 1058906 | 10.393 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*74.606 ns\**</span> | <span title="validated on-demand with error">*712.85 ns\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4743 ns\**</span> <span title="validated upfront with error">*4.6570 ms\**</span> | <span title="unvalidated">*2.6279 µs\**</span> <span title="validated upfront with error">*4.8984 ms\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*1.2369 ns\**</span> <span title="validated upfront with error">*434.66 µs\**</span> | <span title="unvalidated">*433.46 ns\**</span> <span title="validated upfront with error">*437.38 µs\**</span> | <span title="unvalidated">*234.17 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | <span title="encode">*15.65%\**</span> <span title="prepend">*28.34%\**</span> | 25.37% | 56.99% | 67.82% | 68.86% | 28.75% |
| [bincode 2.0.0-rc][bincode] | 48.91% | 59.48% | 69.11% | 78.55% | 80.75% | 35.75% |
| [bincode 1.3.3][bincode1] | 18.48% | 50.57% | 52.37% | 76.87% | 81.67% | 32.33% |
| [bitcode 0.6.3][bitcode] | 98.88% | 93.31% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.1][borsh] | 24.73% | 75.82% | 63.82% | 79.21% | 82.65% | 33.03% |
| [capnp 0.19.7][capnp] | 32.10% | † | 35.60% | 56.71% | 69.11% | 22.90% |
| [cbor4ii 0.3.3][cbor4ii] | 21.50% | 12.41% | 16.13% | 51.78% | 58.52% | 15.35% |
| [ciborium 0.2.2][ciborium] | 2.98% | 3.97% | 16.13% | 51.78% | 58.51% | 15.25% |
| [databuf 0.5.0][databuf] | 54.75% | 58.25% | 73.63% | 82.63% | 85.10% | 37.40% |
| [dlhn 0.1.7][dlhn] | 14.24% | 28.61% | 74.12% | 81.49% | 82.03% | 39.02% |
| [flatbuffers 24.3.25][flatbuffers] | 13.18% | † | 41.72% | 60.87% | 67.80% | 24.97% |
| [msgpacker 0.4.3][msgpacker] | 25.09% | 32.79% | 66.61% | 75.95% | 75.45% | 33.54% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.33% | 12.36% | 54.87% | 68.72% | 67.92% | 27.34% |
| [nanoserde 0.1.37][nanoserde] | 56.58% | 74.18% | 53.57% | 77.35% | 81.33% | 31.57% |
| [parity-scale-codec 3.6.12][parity-scale-codec] | 24.65% | 69.15% | 73.63% | 82.49% | 84.91% | 36.92% |
| [postcard 1.0.10][postcard] | 36.27% | 50.94% | 74.12% | 81.01% | 82.39% | 38.11% |
| [pot 3.0.1][pot] | 5.00% | 7.25% | 37.27% | 59.23% | 66.04% | 20.96% |
| [prost 0.13.2][prost] | <span title="encode">*13.17%\**</span> <span title="populate + encode">*7.59%\**</span> | 25.71% | 52.16% | 65.56% | 66.15% | 27.14% |
| [rkyv 0.8.5][rkyv] | 68.18% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.70%\**</span> | 46.75% | 63.41% | 70.75% | 25.77% |
| [rmp-serde 1.3.0][rmp-serde] | 6.91% | 19.79% | 55.67% | 69.59% | 69.79% | 29.07% |
| [ron 0.8.1][ron] | 1.93% | 2.41% | 11.19% | 39.31% | 46.95% | 9.45% |
| [savefile 0.17.7][savefile] | 85.98% | 79.51% | 54.19% | 77.82% | 81.50% | 31.53% |
| [serde-brief 0.1.0][serde-brief] | 10.53% | 10.18% | 13.95% | 48.86% | 54.63% | 13.92% |
| [serde_bare 0.5.0][serde_bare] | 14.68% | 45.55% | 73.63% | 82.63% | 85.10% | 37.00% |
| [serde_cbor 0.11.2][serde_cbor] | 7.17% | 10.34% | 16.13% | 51.78% | 58.51% | 15.42% |
| [serde_json 1.0.128][serde_json] | 3.20% | 7.29% | 10.34% | 36.73% | 46.52% | 9.47% |
| [simd-json 0.13.10][simd-json] | 5.97% | 8.09% | 10.34% | 36.73% | 46.52% | 9.44% |
| [speedy 0.8.7][speedy] | 97.88% | 90.05% | 61.31% | 78.40% | 82.65% | 32.49% |
| [wiring 0.2.2][wiring] | 100.00% | 79.10% | 54.19% | 75.88% | 79.11% | 30.42% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.66%\**</span> | <span title="validated on-demand with error">*60.81%\**</span> | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.49%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [rkyv 0.8.5][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1010.0
[bincode]: https://crates.io/crates/bincode/2.0.0-rc
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.3
[borsh]: https://crates.io/crates/borsh/1.5.1
[capnp]: https://crates.io/crates/capnp/0.19.7
[cbor4ii]: https://crates.io/crates/cbor4ii/0.3.3
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/24.3.25
[msgpacker]: https://crates.io/crates/msgpacker/0.4.3
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.1.37
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.6.12
[postcard]: https://crates.io/crates/postcard/1.0.10
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.13.2
[rkyv]: https://crates.io/crates/rkyv/0.8.5
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.8.1
[savefile]: https://crates.io/crates/savefile/0.17.7
[serde-brief]: https://crates.io/crates/serde-brief/0.1.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.128
[simd-json]: https://crates.io/crates/simd-json/0.13.10
[speedy]: https://crates.io/crates/speedy/0.8.7
[wiring]: https://crates.io/crates/wiring/0.2.2


## Footnotes:

\* *mouse over for situational details*

† *do not provide deserialization capabilities, but the user can write their own*

‡ *do not support buffer mutation (`capnp` and `flatbuffers` may but not for rust)*
