<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## Maintainers

These benchmarks are maintained by a small group of volunteers. Special thanks to:

- [djkoloski](https://github.com/djkoloski)
- [mumbleskates](https://github.com/mumbleskates)
- [finnbear](https://github.com/finnbear)

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

## Last updated: 2026-04-21 01:07:27

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.97.0-nightly (e22c616e4 2026-04-19)
binary: rustc
commit-hash: e22c616e4e87914135c1db261a03e0437255335e
commit-date: 2026-04-19
host: x86_64-unknown-linux-gnu
release: 1.97.0-nightly
LLVM version: 22.1.2
```

### CPU info

```
Architecture:                            x86_64
CPU op-mode(s):                          32-bit, 64-bit
Address sizes:                           48 bits physical, 48 bits virtual
Byte Order:                              Little Endian
CPU(s):                                  4
On-line CPU(s) list:                     0-3
Vendor ID:                               AuthenticAMD
Model name:                              AMD EPYC 7763 64-Core Processor
CPU family:                              25
Model:                                   1
Thread(s) per core:                      2
Core(s) per socket:                      2
Socket(s):                               1
Stepping:                                1
BogoMIPS:                                4890.86
Flags:                                   fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves user_shstk clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
Virtualization:                          AMD-V
Hypervisor vendor:                       Microsoft
Virtualization type:                     full
L1d cache:                               64 KiB (2 instances)
L1i cache:                               64 KiB (2 instances)
L2 cache:                                1 MiB (2 instances)
L3 cache:                                32 MiB (1 instance)
NUMA node(s):                            1
NUMA node0 CPU(s):                       0-3
Vulnerability Gather data sampling:      Not affected
Vulnerability Ghostwrite:                Not affected
Vulnerability Indirect target selection: Not affected
Vulnerability Itlb multihit:             Not affected
Vulnerability L1tf:                      Not affected
Vulnerability Mds:                       Not affected
Vulnerability Meltdown:                  Not affected
Vulnerability Mmio stale data:           Not affected
Vulnerability Old microcode:             Not affected
Vulnerability Reg file data sampling:    Not affected
Vulnerability Retbleed:                  Not affected
Vulnerability Spec rstack overflow:      Vulnerable: Safe RET, no microcode
Vulnerability Spec store bypass:         Vulnerable
Vulnerability Spectre v1:                Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:                Mitigation; Retpolines; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
Vulnerability Srbds:                     Not affected
Vulnerability Tsa:                       Vulnerable: No microcode
Vulnerability Tsx async abort:           Not affected
Vulnerability Vmscape:                   Not affected
```

</details>

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*447.04 µs\**</span> <span title="prepend">*421.66 µs\**</span> | 2.6185 ms | 867.59 µs | 804955 | 328941 | 284849 | 4.1616 ms |
| [bin-proto 0.12.7][bin-proto] | 4.1860 ms | 4.6820 ms | † | 1045784 | 373127 | 311553 | 4.4236 ms |
| [bincode 2.0.1][bincode] | 330.20 µs | 2.1660 ms | 669.93 µs | 741295 | 303944 | 256422 | 3.5935 ms |
| [bincode 1.3.3][bincode1] | 525.98 µs | 2.0582 ms | 589.67 µs | 1045784 | 373127 | 311553 | 4.4658 ms |
| [bitcode 0.6.6][bitcode] | 146.61 µs | 1.4670 ms | 62.925 µs | 703710 | 288826 | 227322 | 2.5262 ms |
| [borsh 1.5.7][borsh] | 558.53 µs | 2.1333 ms | † | 885780 | 362204 | 286248 | 4.1048 ms |
| [capnp 0.23.2][capnp] | 468.29 µs | † | † | 1443216 | 513986 | 426532 | 6.0933 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 613.65 µs | 5.1274 ms | 3.3821 ms | 1407835 | 403440 | 323561 | 5.0362 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1326 ms | 11.721 ms | † | 1407835 | 403440 | 323561 | 4.9801 ms |
| [columnar 0.11.1][columnar] | 252.08 µs | 2.1487 ms <span title="copy_from">*804.93 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.0922 ms |
| [compactly 0.1.6][compactly] | 27.162 ms | 20.486 ms | † | 241251 | 241453 | 241263 | 102.27 µs |
| [databuf 0.5.0][databuf] | 268.79 µs | 2.0569 ms | 671.85 µs | 765778 | 311715 | 263914 | 3.5020 ms |
| [dlhn 0.1.7][dlhn] | 626.23 µs | 2.5523 ms | † | 724953 | 301446 | 253056 | 3.2396 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0324 ms | † | † | 1276368 | 468539 | 388381 | 4.7882 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.6466 ms | 7.3312 ms | 5.5951 ms | 1829756 | 714318 | 691541 | 8.6546 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.8262 ms | 3.8048 ms | † | 1827461 | 470560 | 360727 | 5.4501 ms |
| [minicbor 1.0.0][minicbor] | 544.67 µs | 3.0017 ms | 1.3225 ms | 817830 | 332671 | 284034 | 4.1366 ms |
| [serde-brief 0.1.1][serde-brief] | 1.4235 ms | 4.7077 ms | 2.8694 ms | 1584946 | 413733 | 339964 | 4.8664 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*82.955 ns\**</span> | <span title="validated on-demand with error">*132.14 µs\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 23.033 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4891 ns\**</span> <span title="validated upfront with error">*2.1109 ms\**</span> | <span title="unvalidated">*52.247 µs\**</span> <span title="validated upfront with error">*2.2054 ms\**</span> | ‡ |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*32.80%\**</span> <span title="prepend">*34.77%\**</span> | 30.74% | 7.25% | 29.97% | 73.40% | 79.80% | 2.46% |
| [bin-proto 0.12.7][bin-proto] | 3.50% | 17.19% | † | 23.07% | 64.71% | 72.96% | 2.31% |
| [bincode 2.0.1][bincode] | 44.40% | 37.16% | 9.39% | 32.54% | 79.44% | 88.65% | 2.85% |
| [bincode 1.3.3][bincode1] | 27.87% | 39.11% | 10.67% | 23.07% | 64.71% | 72.96% | 2.29% |
| [bitcode 0.6.6][bitcode] | 100.00% | 54.87% | 100.00% | 34.28% | 83.60% | 100.00% | 4.05% |
| [borsh 1.5.7][borsh] | 26.25% | 37.73% | † | 27.24% | 66.66% | 79.41% | 2.49% |
| [capnp 0.23.2][capnp] | 31.31% | † | † | 16.72% | 46.98% | 53.30% | 1.68% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 23.89% | 15.70% | 1.86% | 17.14% | 59.85% | 70.26% | 2.03% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.68% | 6.87% | † | 17.14% | 59.85% | 70.26% | 2.05% |
| [columnar 0.11.1][columnar] | 58.16% | 37.46% <span title="copy_from">*100.00%\**</span> | † | 23.07% | 65.22% | 77.34% | 2.50% |
| [compactly 0.1.6][compactly] | 0.54% | 3.93% | † | 100.00% | 100.00% | 94.22% | 100.00% |
| [databuf 0.5.0][databuf] | 54.54% | 39.13% | 9.37% | 31.50% | 77.46% | 86.13% | 2.92% |
| [dlhn 0.1.7][dlhn] | 23.41% | 31.54% | † | 33.28% | 80.10% | 89.83% | 3.16% |
| [flatbuffers 25.12.19][flatbuffers] | 14.20% | † | † | 18.90% | 51.53% | 58.53% | 2.14% |
| [flexbuffers 25.2.10][flexbuffers] | 1.92% | 10.98% | 1.12% | 13.18% | 33.80% | 32.87% | 1.18% |
| json:<br> [flexon 0.4.5][flexon] | 5.19% | 21.16% | † | 13.20% | 51.31% | 63.02% | 1.88% |
| [minicbor 1.0.0][minicbor] | 26.92% | 26.82% | 4.76% | 29.50% | 72.58% | 80.03% | 2.47% |
| [serde-brief 0.1.1][serde-brief] | 10.30% | 17.10% | 2.19% | 15.22% | 58.36% | 66.87% | 2.10% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*3.00%\**</span> | <span title="validated on-demand with error">*39.54%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 10.81% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.37%\**</span> | ‡ |

[bilrost]: https://crates.io/crates/bilrost/0.1013.0
[bin-proto]: https://crates.io/crates/bin-proto/0.12.7
[bincode]: https://crates.io/crates/bincode/2.0.1
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.6
[borsh]: https://crates.io/crates/borsh/1.5.7
[capnp]: https://crates.io/crates/capnp/0.23.2
[cbor4ii]: https://crates.io/crates/cbor4ii/1.0.0
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[columnar]: https://crates.io/crates/columnar/0.11.1
[compactly]: https://crates.io/crates/compactly/0.1.6
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.12.19
[flexbuffers]: https://crates.io/crates/flexbuffers/25.2.10
[flexon]: https://crates.io/crates/flexon/0.4.5
[minicbor]: https://crates.io/crates/minicbor/1.0.0
[serde-brief]: https://crates.io/crates/serde-brief/0.1.1


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
