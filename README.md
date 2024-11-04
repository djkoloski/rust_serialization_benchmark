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

## Last updated: 2024-11-4 21:8:36

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
BogoMIPS:                           4890.84
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
| [capnp 0.19.7][capnp] | 463.26 µs | † | 1443216 | 513986 | 428649 | 6.3557 ms |
| [flatbuffers 24.3.25][flatbuffers] | 1.0169 ms | † | 1276368 | 468539 | 388832 | 5.1700 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | ‡ | ‡ | ‡ |
| [bincode 2.0.0-rc][bincode] | ‡ | ‡ | ‡ |
| [bincode 1.3.3][bincode1] | ‡ | ‡ | ‡ |
| [bitcode 0.6.3][bitcode] | ‡ | ‡ | ‡ |
| [borsh 1.5.1][borsh] | ‡ | ‡ | ‡ |
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.766 ns\**</span> | <span title="validated on-demand with error">*165.37 µs\**</span> | ‡ |
| [cbor4ii 0.3.3][cbor4ii] | ‡ | ‡ | ‡ |
| [ciborium 0.2.2][ciborium] | ‡ | ‡ | ‡ |
| [databuf 0.5.0][databuf] | ‡ | ‡ | ‡ |
| [dlhn 0.1.7][dlhn] | ‡ | ‡ | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4752 ns\**</span> <span title="validated upfront with error">*1.9291 ms\**</span> | <span title="unvalidated">*50.530 µs\**</span> <span title="validated upfront with error">*2.0735 ms\**</span> | ‡ |
| [msgpacker 0.4.3][msgpacker] | ‡ | ‡ | ‡ |
| [nachricht-serde 0.4.0][nachricht-serde] | ‡ | ‡ | ‡ |
| [nanoserde 0.1.37][nanoserde] | ‡ | ‡ | ‡ |
| [parity-scale-codec 3.6.12][parity-scale-codec] | ‡ | ‡ | ‡ |
| [postcard 1.0.10][postcard] | ‡ | ‡ | ‡ |
| [pot 3.0.1][pot] | ‡ | ‡ | ‡ |
| [prost 0.13.2][prost] | ‡ | ‡ | ‡ |
| [rmp-serde 1.3.0][rmp-serde] | ‡ | ‡ | ‡ |
| [ron 0.8.1][ron] | ‡ | ‡ | ‡ |
| [savefile 0.17.7][savefile] | ‡ | ‡ | ‡ |
| [serde-brief 0.1.0][serde-brief] | ‡ | ‡ | ‡ |
| [serde_bare 0.5.0][serde_bare] | ‡ | ‡ | ‡ |
| [serde_cbor 0.11.2][serde_cbor] | ‡ | ‡ | ‡ |
| [serde_json 1.0.128][serde_json] | ‡ | ‡ | ‡ |
| [simd-json 0.13.10][simd-json] | ‡ | ‡ | ‡ |
| [speedy 0.8.7][speedy] | ‡ | ‡ | ‡ |
| [wiring 0.2.2][wiring] | ‡ | ‡ | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [capnp 0.19.7][capnp] | 31.30% | † | 48.76% | 56.19% | 53.60% | 40.63% |
| [flatbuffers 24.3.25][flatbuffers] | 14.26% | † | 55.13% | 61.64% | 59.09% | 49.95% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | ‡ | ‡ | ‡ |
| [bincode 2.0.0-rc][bincode] | ‡ | ‡ | ‡ |
| [bincode 1.3.3][bincode1] | ‡ | ‡ | ‡ |
| [bitcode 0.6.3][bitcode] | ‡ | ‡ | ‡ |
| [borsh 1.5.1][borsh] | ‡ | ‡ | ‡ |
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*6.30%\**</span> | ‡ |
| [cbor4ii 0.3.3][cbor4ii] | ‡ | ‡ | ‡ |
| [ciborium 0.2.2][ciborium] | ‡ | ‡ | ‡ |
| [databuf 0.5.0][databuf] | ‡ | ‡ | ‡ |
| [dlhn 0.1.7][dlhn] | ‡ | ‡ | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.63%\**</span> <span title="validated upfront with error">*0.50%\**</span> | ‡ |
| [msgpacker 0.4.3][msgpacker] | ‡ | ‡ | ‡ |
| [nachricht-serde 0.4.0][nachricht-serde] | ‡ | ‡ | ‡ |
| [nanoserde 0.1.37][nanoserde] | ‡ | ‡ | ‡ |
| [parity-scale-codec 3.6.12][parity-scale-codec] | ‡ | ‡ | ‡ |
| [postcard 1.0.10][postcard] | ‡ | ‡ | ‡ |
| [pot 3.0.1][pot] | ‡ | ‡ | ‡ |
| [prost 0.13.2][prost] | ‡ | ‡ | ‡ |
| [rmp-serde 1.3.0][rmp-serde] | ‡ | ‡ | ‡ |
| [ron 0.8.1][ron] | ‡ | ‡ | ‡ |
| [savefile 0.17.7][savefile] | ‡ | ‡ | ‡ |
| [serde-brief 0.1.0][serde-brief] | ‡ | ‡ | ‡ |
| [serde_bare 0.5.0][serde_bare] | ‡ | ‡ | ‡ |
| [serde_cbor 0.11.2][serde_cbor] | ‡ | ‡ | ‡ |
| [serde_json 1.0.128][serde_json] | ‡ | ‡ | ‡ |
| [simd-json 0.13.10][simd-json] | ‡ | ‡ | ‡ |
| [speedy 0.8.7][speedy] | ‡ | ‡ | ‡ |
| [wiring 0.2.2][wiring] | ‡ | ‡ | ‡ |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [capnp 0.19.7][capnp] | 5.4618 ms | † | 14000088 | 7130367 | 6051062 | 78.383 ms |
| [flatbuffers 24.3.25][flatbuffers] | 881.83 µs | † | 6000024 | 5378434 | 5345910 | 7.8045 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | ‡ | ‡ | ‡ |
| [bincode 2.0.0-rc][bincode] | ‡ | ‡ | ‡ |
| [bincode 1.3.3][bincode1] | ‡ | ‡ | ‡ |
| [bitcode 0.6.3][bitcode] | ‡ | ‡ | ‡ |
| [borsh 1.5.1][borsh] | ‡ | ‡ | ‡ |
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*104.65 ns\**</span> | <span title="validated on-demand with error">*2.1345 ms\**</span> | ‡ |
| [cbor4ii 0.3.3][cbor4ii] | ‡ | ‡ | ‡ |
| [ciborium 0.2.2][ciborium] | ‡ | ‡ | ‡ |
| [databuf 0.5.0][databuf] | ‡ | ‡ | ‡ |
| [dlhn 0.1.7][dlhn] | ‡ | ‡ | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4759 ns\**</span> <span title="validated upfront with error">*39.458 ns\**</span> | <span title="unvalidated">*53.998 µs\**</span> <span title="validated upfront with error">*77.427 µs\**</span> | ‡ |
| [msgpacker 0.4.3][msgpacker] | ‡ | ‡ | ‡ |
| [nachricht-serde 0.4.0][nachricht-serde] | ‡ | ‡ | ‡ |
| [nanoserde 0.1.37][nanoserde] | ‡ | ‡ | ‡ |
| [parity-scale-codec 3.6.12][parity-scale-codec] | ‡ | ‡ | ‡ |
| [postcard 1.0.10][postcard] | ‡ | ‡ | ‡ |
| [pot 3.0.1][pot] | ‡ | ‡ | ‡ |
| [prost 0.13.2][prost] | ‡ | ‡ | ‡ |
| [rmp-serde 1.3.0][rmp-serde] | ‡ | ‡ | ‡ |
| [ron 0.8.1][ron] | ‡ | ‡ | ‡ |
| [savefile 0.17.7][savefile] | ‡ | ‡ | ‡ |
| [serde-brief 0.1.0][serde-brief] | ‡ | ‡ | ‡ |
| [serde_bare 0.5.0][serde_bare] | ‡ | ‡ | ‡ |
| [serde_cbor 0.11.2][serde_cbor] | ‡ | ‡ | ‡ |
| [serde_json 1.0.128][serde_json] | ‡ | ‡ | ‡ |
| [simd-json 0.13.10][simd-json] | ‡ | ‡ | ‡ |
| [speedy 0.8.7][speedy] | ‡ | ‡ | ‡ |
| [wiring 0.2.2][wiring] | ‡ | ‡ | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [capnp 0.19.7][capnp] | 3.61% | † | 42.86% | 72.68% | 81.37% | 9.43% |
| [flatbuffers 24.3.25][flatbuffers] | 22.36% | † | 100.00% | 96.35% | 92.11% | 94.66% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | ‡ | ‡ | ‡ |
| [bincode 2.0.0-rc][bincode] | ‡ | ‡ | ‡ |
| [bincode 1.3.3][bincode1] | ‡ | ‡ | ‡ |
| [bitcode 0.6.3][bitcode] | ‡ | ‡ | ‡ |
| [borsh 1.5.1][borsh] | ‡ | ‡ | ‡ |
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.18%\**</span> | <span title="validated on-demand with error">*1.81%\**</span> | ‡ |
| [cbor4ii 0.3.3][cbor4ii] | ‡ | ‡ | ‡ |
| [ciborium 0.2.2][ciborium] | ‡ | ‡ | ‡ |
| [databuf 0.5.0][databuf] | ‡ | ‡ | ‡ |
| [dlhn 0.1.7][dlhn] | ‡ | ‡ | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.95%\**</span> <span title="validated upfront with error">*3.13%\**</span> | <span title="unvalidated">*71.66%\**</span> <span title="validated upfront with error">*49.98%\**</span> | ‡ |
| [msgpacker 0.4.3][msgpacker] | ‡ | ‡ | ‡ |
| [nachricht-serde 0.4.0][nachricht-serde] | ‡ | ‡ | ‡ |
| [nanoserde 0.1.37][nanoserde] | ‡ | ‡ | ‡ |
| [parity-scale-codec 3.6.12][parity-scale-codec] | ‡ | ‡ | ‡ |
| [postcard 1.0.10][postcard] | ‡ | ‡ | ‡ |
| [pot 3.0.1][pot] | ‡ | ‡ | ‡ |
| [prost 0.13.2][prost] | ‡ | ‡ | ‡ |
| [rmp-serde 1.3.0][rmp-serde] | ‡ | ‡ | ‡ |
| [ron 0.8.1][ron] | ‡ | ‡ | ‡ |
| [savefile 0.17.7][savefile] | ‡ | ‡ | ‡ |
| [serde-brief 0.1.0][serde-brief] | ‡ | ‡ | ‡ |
| [serde_bare 0.5.0][serde_bare] | ‡ | ‡ | ‡ |
| [serde_cbor 0.11.2][serde_cbor] | ‡ | ‡ | ‡ |
| [serde_json 1.0.128][serde_json] | ‡ | ‡ | ‡ |
| [simd-json 0.13.10][simd-json] | ‡ | ‡ | ‡ |
| [speedy 0.8.7][speedy] | ‡ | ‡ | ‡ |
| [wiring 0.2.2][wiring] | ‡ | ‡ | ‡ |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [capnp 0.19.7][capnp] | 480.55 µs | † | 803896 | 335606 | 280851 | 3.9095 ms |
| [flatbuffers 24.3.25][flatbuffers] | 3.2835 ms | † | 844168 | 345696 | 294015 | 3.8592 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | ‡ | ‡ | ‡ |
| [bincode 2.0.0-rc][bincode] | ‡ | ‡ | ‡ |
| [bincode 1.3.3][bincode1] | ‡ | ‡ | ‡ |
| [bitcode 0.6.3][bitcode] | ‡ | ‡ | ‡ |
| [borsh 1.5.1][borsh] | ‡ | ‡ | ‡ |
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.868 ns\**</span> | <span title="validated on-demand with error">*454.75 ns\**</span> | ‡ |
| [cbor4ii 0.3.3][cbor4ii] | ‡ | ‡ | ‡ |
| [ciborium 0.2.2][ciborium] | ‡ | ‡ | ‡ |
| [databuf 0.5.0][databuf] | ‡ | ‡ | ‡ |
| [dlhn 0.1.7][dlhn] | ‡ | ‡ | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4785 ns\**</span> <span title="validated upfront with error">*2.1914 ms\**</span> | <span title="unvalidated">*1.3549 µs\**</span> <span title="validated upfront with error">*2.2023 ms\**</span> | ‡ |
| [msgpacker 0.4.3][msgpacker] | ‡ | ‡ | ‡ |
| [nachricht-serde 0.4.0][nachricht-serde] | ‡ | ‡ | ‡ |
| [nanoserde 0.1.37][nanoserde] | ‡ | ‡ | ‡ |
| [parity-scale-codec 3.6.12][parity-scale-codec] | ‡ | ‡ | ‡ |
| [postcard 1.0.10][postcard] | ‡ | ‡ | ‡ |
| [pot 3.0.1][pot] | ‡ | ‡ | ‡ |
| [prost 0.13.2][prost] | ‡ | ‡ | ‡ |
| [rmp-serde 1.3.0][rmp-serde] | ‡ | ‡ | ‡ |
| [ron 0.8.1][ron] | ‡ | ‡ | ‡ |
| [savefile 0.17.7][savefile] | ‡ | ‡ | ‡ |
| [serde-brief 0.1.0][serde-brief] | ‡ | ‡ | ‡ |
| [serde_bare 0.5.0][serde_bare] | ‡ | ‡ | ‡ |
| [serde_cbor 0.11.2][serde_cbor] | ‡ | ‡ | ‡ |
| [serde_json 1.0.128][serde_json] | ‡ | ‡ | ‡ |
| [simd-json 0.13.10][simd-json] | ‡ | ‡ | ‡ |
| [speedy 0.8.7][speedy] | ‡ | ‡ | ‡ |
| [wiring 0.2.2][wiring] | ‡ | ‡ | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [capnp 0.19.7][capnp] | 27.31% | † | 40.76% | 59.88% | 65.07% | 19.14% |
| [flatbuffers 24.3.25][flatbuffers] | 4.00% | † | 38.82% | 58.13% | 62.15% | 19.39% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | ‡ | ‡ | ‡ |
| [bincode 2.0.0-rc][bincode] | ‡ | ‡ | ‡ |
| [bincode 1.3.3][bincode1] | ‡ | ‡ | ‡ |
| [bitcode 0.6.3][bitcode] | ‡ | ‡ | ‡ |
| [borsh 1.5.1][borsh] | ‡ | ‡ | ‡ |
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*52.53%\**</span> | ‡ |
| [cbor4ii 0.3.3][cbor4ii] | ‡ | ‡ | ‡ |
| [ciborium 0.2.2][ciborium] | ‡ | ‡ | ‡ |
| [databuf 0.5.0][databuf] | ‡ | ‡ | ‡ |
| [dlhn 0.1.7][dlhn] | ‡ | ‡ | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*49.93%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*17.63%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [msgpacker 0.4.3][msgpacker] | ‡ | ‡ | ‡ |
| [nachricht-serde 0.4.0][nachricht-serde] | ‡ | ‡ | ‡ |
| [nanoserde 0.1.37][nanoserde] | ‡ | ‡ | ‡ |
| [parity-scale-codec 3.6.12][parity-scale-codec] | ‡ | ‡ | ‡ |
| [postcard 1.0.10][postcard] | ‡ | ‡ | ‡ |
| [pot 3.0.1][pot] | ‡ | ‡ | ‡ |
| [prost 0.13.2][prost] | ‡ | ‡ | ‡ |
| [rmp-serde 1.3.0][rmp-serde] | ‡ | ‡ | ‡ |
| [ron 0.8.1][ron] | ‡ | ‡ | ‡ |
| [savefile 0.17.7][savefile] | ‡ | ‡ | ‡ |
| [serde-brief 0.1.0][serde-brief] | ‡ | ‡ | ‡ |
| [serde_bare 0.5.0][serde_bare] | ‡ | ‡ | ‡ |
| [serde_cbor 0.11.2][serde_cbor] | ‡ | ‡ | ‡ |
| [serde_json 1.0.128][serde_json] | ‡ | ‡ | ‡ |
| [simd-json 0.13.10][simd-json] | ‡ | ‡ | ‡ |
| [speedy 0.8.7][speedy] | ‡ | ‡ | ‡ |
| [wiring 0.2.2][wiring] | ‡ | ‡ | ‡ |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [capnp 0.19.7][capnp] | 2.1950 ms | † | 2664040 | 1511895 | 1212087 | 14.076 ms |
| [flatbuffers 24.3.25][flatbuffers] | 5.2933 ms | † | 2273740 | 1408408 | 1235566 | 12.796 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | ‡ | ‡ | ‡ |
| [bincode 2.0.0-rc][bincode] | ‡ | ‡ | ‡ |
| [bincode 1.3.3][bincode1] | ‡ | ‡ | ‡ |
| [bitcode 0.6.3][bitcode] | ‡ | ‡ | ‡ |
| [borsh 1.5.1][borsh] | ‡ | ‡ | ‡ |
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*72.874 ns\**</span> | <span title="validated on-demand with error">*1.0135 µs\**</span> | ‡ |
| [cbor4ii 0.3.3][cbor4ii] | ‡ | ‡ | ‡ |
| [ciborium 0.2.2][ciborium] | ‡ | ‡ | ‡ |
| [databuf 0.5.0][databuf] | ‡ | ‡ | ‡ |
| [dlhn 0.1.7][dlhn] | ‡ | ‡ | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*2.4736 ns\**</span> <span title="validated upfront with error">*4.8916 ms\**</span> | <span title="unvalidated">*2.6523 µs\**</span> <span title="validated upfront with error">*4.7869 ms\**</span> | ‡ |
| [msgpacker 0.4.3][msgpacker] | ‡ | ‡ | ‡ |
| [nachricht-serde 0.4.0][nachricht-serde] | ‡ | ‡ | ‡ |
| [nanoserde 0.1.37][nanoserde] | ‡ | ‡ | ‡ |
| [parity-scale-codec 3.6.12][parity-scale-codec] | ‡ | ‡ | ‡ |
| [postcard 1.0.10][postcard] | ‡ | ‡ | ‡ |
| [pot 3.0.1][pot] | ‡ | ‡ | ‡ |
| [prost 0.13.2][prost] | ‡ | ‡ | ‡ |
| [rmp-serde 1.3.0][rmp-serde] | ‡ | ‡ | ‡ |
| [ron 0.8.1][ron] | ‡ | ‡ | ‡ |
| [savefile 0.17.7][savefile] | ‡ | ‡ | ‡ |
| [serde-brief 0.1.0][serde-brief] | ‡ | ‡ | ‡ |
| [serde_bare 0.5.0][serde_bare] | ‡ | ‡ | ‡ |
| [serde_cbor 0.11.2][serde_cbor] | ‡ | ‡ | ‡ |
| [serde_json 1.0.128][serde_json] | ‡ | ‡ | ‡ |
| [simd-json 0.13.10][simd-json] | ‡ | ‡ | ‡ |
| [speedy 0.8.7][speedy] | ‡ | ‡ | ‡ |
| [wiring 0.2.2][wiring] | ‡ | ‡ | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [capnp 0.19.7][capnp] | 31.87% | † | 35.60% | 56.71% | 69.11% | 21.35% |
| [flatbuffers 24.3.25][flatbuffers] | 13.21% | † | 41.72% | 60.87% | 67.80% | 23.49% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [bilrost 0.1010.0][bilrost] | ‡ | ‡ | ‡ |
| [bincode 2.0.0-rc][bincode] | ‡ | ‡ | ‡ |
| [bincode 1.3.3][bincode1] | ‡ | ‡ | ‡ |
| [bitcode 0.6.3][bitcode] | ‡ | ‡ | ‡ |
| [borsh 1.5.1][borsh] | ‡ | ‡ | ‡ |
| [capnp 0.19.7][capnp] | <span title="validated on-demand with error">*1.70%\**</span> | <span title="validated on-demand with error">*43.63%\**</span> | ‡ |
| [cbor4ii 0.3.3][cbor4ii] | ‡ | ‡ | ‡ |
| [ciborium 0.2.2][ciborium] | ‡ | ‡ | ‡ |
| [databuf 0.5.0][databuf] | ‡ | ‡ | ‡ |
| [dlhn 0.1.7][dlhn] | ‡ | ‡ | ‡ |
| [flatbuffers 24.3.25][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*16.67%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [msgpacker 0.4.3][msgpacker] | ‡ | ‡ | ‡ |
| [nachricht-serde 0.4.0][nachricht-serde] | ‡ | ‡ | ‡ |
| [nanoserde 0.1.37][nanoserde] | ‡ | ‡ | ‡ |
| [parity-scale-codec 3.6.12][parity-scale-codec] | ‡ | ‡ | ‡ |
| [postcard 1.0.10][postcard] | ‡ | ‡ | ‡ |
| [pot 3.0.1][pot] | ‡ | ‡ | ‡ |
| [prost 0.13.2][prost] | ‡ | ‡ | ‡ |
| [rmp-serde 1.3.0][rmp-serde] | ‡ | ‡ | ‡ |
| [ron 0.8.1][ron] | ‡ | ‡ | ‡ |
| [savefile 0.17.7][savefile] | ‡ | ‡ | ‡ |
| [serde-brief 0.1.0][serde-brief] | ‡ | ‡ | ‡ |
| [serde_bare 0.5.0][serde_bare] | ‡ | ‡ | ‡ |
| [serde_cbor 0.11.2][serde_cbor] | ‡ | ‡ | ‡ |
| [serde_json 1.0.128][serde_json] | ‡ | ‡ | ‡ |
| [simd-json 0.13.10][simd-json] | ‡ | ‡ | ‡ |
| [speedy 0.8.7][speedy] | ‡ | ‡ | ‡ |
| [wiring 0.2.2][wiring] | ‡ | ‡ | ‡ |

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
