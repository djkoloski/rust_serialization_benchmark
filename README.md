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

## Last updated: 2026-04-17 11:11:59

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.97.0-nightly (7af3402cd 2026-04-16)
binary: rustc
commit-hash: 7af3402cda75aaead39f72516fd6cbb2f3ee0dbd
commit-date: 2026-04-16
host: x86_64-unknown-linux-gnu
release: 1.97.0-nightly
LLVM version: 22.1.2
```

### CPU info

```
Architecture:                            x86_64
CPU op-mode(s):                          32-bit, 64-bit
Address sizes:                           46 bits physical, 57 bits virtual
Byte Order:                              Little Endian
CPU(s):                                  4
On-line CPU(s) list:                     0-3
Vendor ID:                               GenuineIntel
Model name:                              Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz
CPU family:                              6
Model:                                   106
Thread(s) per core:                      2
Core(s) per socket:                      2
Socket(s):                               1
Stepping:                                6
CPU(s) scaling MHz:                      124%
CPU max MHz:                             2800.0000
CPU min MHz:                             800.0000
BogoMIPS:                                5586.87
Flags:                                   fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology tsc_reliable nonstop_tsc cpuid aperfmperf tsc_known_freq pni pclmulqdq vmx ssse3 fma cx16 pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand hypervisor lahf_lm abm 3dnowprefetch tpr_shadow ept vpid ept_ad fsgsbase tsc_adjust bmi1 hle avx2 smep bmi2 erms invpcid rtm avx512f avx512dq rdseed adx smap avx512ifma clflushopt clwb avx512cd sha_ni avx512bw avx512vl xsaveopt xsavec xgetbv1 xsaves vnmi avx512vbmi umip avx512_vbmi2 gfni vaes vpclmulqdq avx512_vnni avx512_bitalg avx512_vpopcntdq la57 rdpid fsrm arch_capabilities
Virtualization:                          VT-x
Hypervisor vendor:                       Microsoft
Virtualization type:                     full
L1d cache:                               96 KiB (2 instances)
L1i cache:                               64 KiB (2 instances)
L2 cache:                                2.5 MiB (2 instances)
L3 cache:                                48 MiB (1 instance)
NUMA node(s):                            1
NUMA node0 CPU(s):                       0-3
Vulnerability Gather data sampling:      Not affected
Vulnerability Ghostwrite:                Not affected
Vulnerability Indirect target selection: Mitigation; Aligned branch/return thunks
Vulnerability Itlb multihit:             Not affected
Vulnerability L1tf:                      Not affected
Vulnerability Mds:                       Not affected
Vulnerability Meltdown:                  Not affected
Vulnerability Mmio stale data:           Vulnerable: Clear CPU buffers attempted, no microcode; SMT Host state unknown
Vulnerability Old microcode:             Not affected
Vulnerability Reg file data sampling:    Not affected
Vulnerability Retbleed:                  Vulnerable
Vulnerability Spec rstack overflow:      Not affected
Vulnerability Spec store bypass:         Vulnerable
Vulnerability Spectre v1:                Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:                Mitigation; Retpolines; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Retpoline
Vulnerability Srbds:                     Not affected
Vulnerability Tsa:                       Not affected
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*409.54 µs\**</span> <span title="prepend">*371.26 µs\**</span> | 2.6760 ms | 886.80 µs | 804955 | 328941 | 284849 | 3.4846 ms |
| [bin-proto 0.12.7][bin-proto] | 3.4014 ms | 4.3698 ms | † | 1045784 | 373127 | 311553 | 3.8995 ms |
| [bincode 2.0.1][bincode] | 329.50 µs | 2.3641 ms | 707.61 µs | 741295 | 303944 | 256422 | 3.1388 ms |
| [bincode 1.3.3][bincode1] | 416.61 µs | 2.2311 ms | 640.68 µs | 1045784 | 373127 | 311553 | 3.9820 ms |
| [bitcode 0.6.6][bitcode] | 222.87 µs | 2.0023 ms | 78.498 µs | 703710 | 288826 | 227322 | 2.2589 ms |
| [borsh 1.5.7][borsh] | 407.35 µs | 2.3846 ms | † | 885780 | 362204 | 286248 | 3.3752 ms |
| [capnp 0.23.2][capnp] | 474.60 µs | † | † | 1443216 | 513986 | 426532 | 5.7914 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 519.48 µs | 4.8821 ms | 2.9481 ms | 1407835 | 403440 | 323561 | 4.5676 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.8401 ms | 10.136 ms | † | 1407835 | 403440 | 323561 | 4.5717 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.4499 ms | 4.7215 ms | 2.8446 ms | 1407835 | 403440 | 323561 | 4.3305 ms |
| [columnar 0.11.1][columnar] | 273.90 µs | 2.3883 ms <span title="copy_from">*848.99 µs\**</span> | † | 1045928 | 370212 | 293907 | 3.5695 ms |
| [compactly 0.1.6][compactly] | 29.567 ms | 20.504 ms | † | 241251 | 241453 | 241263 | 120.78 µs |
| [databuf 0.5.0][databuf] | 262.41 µs | 2.2848 ms | 663.79 µs | 765778 | 311715 | 263914 | 3.0197 ms |
| [dlhn 0.1.7][dlhn] | 482.86 µs | 2.6024 ms | † | 724953 | 301446 | 253056 | 2.7552 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0493 ms | † | † | 1276368 | 468539 | 388381 | 4.1943 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.0302 ms | 6.7543 ms | 4.8857 ms | 1829756 | 714318 | 691541 | 7.3064 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.0910 ms | 3.9853 ms | † | 1827461 | 470560 | 360727 | 4.8986 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.2817 ms | 6.1019 ms | † | 1827461 | 470560 | 360727 | 5.0246 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 1.8587 ms | 4.8378 ms | † | 1827461 | 470560 | 360727 | 4.9325 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.1010 ms | 2.6425 ms | † | 764996 | 315291 | 264212 | 2.9940 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.1761 ms | 3.1604 ms | 1.3124 ms | 784997 | 325384 | 277608 | 3.3498 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 331.61 µs | 2.5962 ms | 872.48 µs | 784997 | 325384 | 277608 | 3.3030 ms |
| [minicbor 1.0.0][minicbor] | 359.90 µs | 3.0790 ms | 1.3596 ms | 817830 | 332671 | 284034 | 3.3625 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 4.9064 ms | 4.0479 ms | 2.2742 ms | 818669 | 332556 | 284797 | 3.3586 ms |
| [nanoserde 0.2.1][nanoserde] | 245.61 µs | 2.3094 ms | † | 1045784 | 373127 | 311553 | 3.7232 ms |
| [nibblecode 0.1.0][nibblecode] | 160.43 µs | † | † | 1011487 | 473998 | 404668 | 4.5926 ms |
| [postcard 1.1.1][postcard] | 319.80 µs | 2.3957 ms | 655.03 µs | 724953 | 302399 | 252968 | 2.7294 ms |
| [pot 3.0.1][pot] | 1.8325 ms | 5.8029 ms | 3.9429 ms | 971922 | 372513 | 303636 | 3.8673 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.0335 ms\**</span> <span title="populate + encode">*3.0090 ms\**</span> | 3.4566 ms | † | 884628 | 363130 | 314959 | 3.7470 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*906.70 µs\**</span> <span title="populate + encode">*3.3611 ms\**</span> | 3.8648 ms | † | 884628 | 363130 | 314959 | 4.0258 ms |
| [rkyv 0.8.10][rkyv] | 243.54 µs | <span title="unvalidated">*1.8480 ms\**</span> <span title="validated upfront with error">*2.2648 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.0202 ms |
| [ron 0.10.1][ron] | 11.100 ms | 21.228 ms | 18.911 ms | 1607459 | 449158 | 349324 | 5.1825 ms |
| [savefile 0.18.6][savefile] | 172.56 µs | 2.3361 ms | † | 1045800 | 373139 | 311562 | 3.6351 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 480.50 µs | 2.5200 ms | † | 765778 | 311743 | 263822 | 2.9079 ms |
| [serde-brief 0.1.1][serde-brief] | 1.1679 ms | 4.3671 ms | 2.5172 ms | 1584946 | 413733 | 339964 | 4.5760 ms |
| [serde_bare 0.5.0][serde_bare] | 519.29 µs | 2.2561 ms | † | 765778 | 311715 | 263914 | 2.9745 ms |
| [speedy 0.8.7][speedy] | 190.40 µs | 2.1275 ms | 427.40 µs | 885780 | 362204 | 286248 | 3.4099 ms |
| [wincode 0.2.4][wincode] | 153.13 µs | 2.2308 ms | 521.71 µs | 1045784 | 373127 | 311553 | 3.6433 ms |
| [wiring 0.2.4][wiring] | 174.06 µs | 2.3621 ms | † | 1045784 | 337930 | 275808 | 3.2915 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*67.583 ns\**</span> | <span title="validated on-demand with error">*114.26 µs\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 19.340 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*1.9271 ns\**</span> <span title="validated upfront with error">*1.7278 ms\**</span> | <span title="unvalidated">*50.918 µs\**</span> <span title="validated upfront with error">*1.7870 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="validated upfront with error">*269.67 µs\**</span> | <span title="unvalidated">*12.665 µs\**</span> <span title="validated upfront with error">*282.45 µs\**</span> | <span title="unvalidated">*7.2539 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="validated upfront with error">*401.65 µs\**</span> | <span title="unvalidated">*12.661 µs\**</span> <span title="validated upfront with error">*414.98 µs\**</span> | <span title="unvalidated">*7.2366 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*37.39%\**</span> <span title="prepend">*41.25%\**</span> | 31.73% | 8.85% | 29.97% | 73.40% | 79.80% | 3.47% |
| [bin-proto 0.12.7][bin-proto] | 4.50% | 19.43% | † | 23.07% | 64.71% | 72.96% | 3.10% |
| [bincode 2.0.1][bincode] | 46.47% | 35.91% | 11.09% | 32.54% | 79.44% | 88.65% | 3.85% |
| [bincode 1.3.3][bincode1] | 36.76% | 38.05% | 12.25% | 23.07% | 64.71% | 72.96% | 3.03% |
| [bitcode 0.6.6][bitcode] | 68.71% | 42.40% | 100.00% | 34.28% | 83.60% | 100.00% | 5.35% |
| [borsh 1.5.7][borsh] | 37.59% | 35.60% | † | 27.24% | 66.66% | 79.41% | 3.58% |
| [capnp 0.23.2][capnp] | 32.27% | † | † | 16.72% | 46.98% | 53.30% | 2.09% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 29.48% | 17.39% | 2.66% | 17.14% | 59.85% | 70.26% | 2.64% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 5.39% | 8.38% | † | 17.14% | 59.85% | 70.26% | 2.64% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.56% | 17.98% | 2.76% | 17.14% | 59.85% | 70.26% | 2.79% |
| [columnar 0.11.1][columnar] | 55.91% | 35.55% <span title="copy_from">*100.00%\**</span> | † | 23.07% | 65.22% | 77.34% | 3.38% |
| [compactly 0.1.6][compactly] | 0.52% | 4.14% | † | 100.00% | 100.00% | 94.22% | 100.00% |
| [databuf 0.5.0][databuf] | 58.36% | 37.16% | 11.83% | 31.50% | 77.46% | 86.13% | 4.00% |
| [dlhn 0.1.7][dlhn] | 31.71% | 32.62% | † | 33.28% | 80.10% | 89.83% | 4.38% |
| [flatbuffers 25.12.19][flatbuffers] | 14.59% | † | † | 18.90% | 51.53% | 58.53% | 2.88% |
| [flexbuffers 25.2.10][flexbuffers] | 2.54% | 12.57% | 1.61% | 13.18% | 33.80% | 32.87% | 1.65% |
| json:<br> [flexon 0.4.5][flexon] | 7.32% | 21.30% | † | 13.20% | 51.31% | 63.02% | 2.47% |
| json:<br> [serde_json 1.0.140][serde_json] | 4.67% | 13.91% | † | 13.20% | 51.31% | 63.02% | 2.40% |
| json:<br> [simd-json 0.15.1][simd-json] | 8.24% | 17.55% | † | 13.20% | 51.31% | 63.02% | 2.45% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 13.91% | 32.13% | † | 31.54% | 76.58% | 86.04% | 4.03% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 13.02% | 26.86% | 5.98% | 30.73% | 74.21% | 81.89% | 3.61% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 46.18% | 32.70% | 9.00% | 30.73% | 74.21% | 81.89% | 3.66% |
| [minicbor 1.0.0][minicbor] | 42.55% | 27.57% | 5.77% | 29.50% | 72.58% | 80.03% | 3.59% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.12% | 20.97% | 3.45% | 29.47% | 72.61% | 79.82% | 3.60% |
| [nanoserde 0.2.1][nanoserde] | 62.35% | 36.76% | † | 23.07% | 64.71% | 72.96% | 3.24% |
| [nibblecode 0.1.0][nibblecode] | 95.45% | † | † | 23.85% | 50.94% | 56.17% | 2.63% |
| [postcard 1.1.1][postcard] | 47.88% | 35.44% | 11.98% | 33.28% | 79.85% | 89.86% | 4.42% |
| [pot 3.0.1][pot] | 8.36% | 14.63% | 1.99% | 24.82% | 64.82% | 74.87% | 3.12% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*14.82%\**</span> <span title="populate + encode">*5.09%\**</span> | 24.56% | † | 27.27% | 66.49% | 72.18% | 3.22% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*16.89%\**</span> <span title="populate + encode">*4.56%\**</span> | 21.97% | † | 27.27% | 66.49% | 72.18% | 3.00% |
| [rkyv 0.8.10][rkyv] | 62.88% | <span title="unvalidated">*45.94%\**</span> <span title="validated upfront with error">*37.49%\**</span> | † | 23.85% | 61.36% | 69.74% | 3.00% |
| [ron 0.10.1][ron] | 1.38% | 4.00% | 0.42% | 15.01% | 53.76% | 65.07% | 2.33% |
| [savefile 0.18.6][savefile] | 88.74% | 36.34% | † | 23.07% | 64.71% | 72.96% | 3.32% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 31.87% | 33.69% | † | 31.50% | 77.45% | 86.16% | 4.15% |
| [serde-brief 0.1.1][serde-brief] | 13.11% | 19.44% | 3.12% | 15.22% | 58.36% | 66.87% | 2.64% |
| [serde_bare 0.5.0][serde_bare] | 29.49% | 37.63% | † | 31.50% | 77.46% | 86.13% | 4.06% |
| [speedy 0.8.7][speedy] | 80.43% | 39.91% | 18.37% | 27.24% | 66.66% | 79.41% | 3.54% |
| [wincode 0.2.4][wincode] | 100.00% | 38.06% | 15.05% | 23.07% | 64.71% | 72.96% | 3.31% |
| [wiring 0.2.4][wiring] | 87.98% | 35.94% | † | 23.07% | 71.45% | 82.42% | 3.67% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*2.85%\**</span> | <span title="validated on-demand with error">*11.08%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 9.96% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*24.87%\**</span> <span title="validated upfront with error">*0.71%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*4.48%\**</span> | <span title="unvalidated">*99.76%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*3.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*13.554 ms\**</span> <span title="prepend">*4.9652 ms\**</span> | 8.7218 ms | 8625005 | 6443961 | 6231572 | 60.010 ms |
| [bin-proto 0.12.7][bin-proto] | 7.0554 ms | 8.6935 ms | 6000008 | 5378500 | 5346908 | 8.9266 ms |
| [bincode 2.0.1][bincode] | 1.3082 ms | 915.34 µs | 6000005 | 5378497 | 5346882 | 8.7103 ms |
| [bincode 1.3.3][bincode1] | 4.4251 ms | 5.3131 ms | 6000008 | 5378500 | 5346908 | 8.6840 ms |
| [bitcode 0.6.6][bitcode] | 2.2507 ms | 1.9035 ms | 6000006 | 5182295 | 4921841 | 13.818 ms |
| [borsh 1.5.7][borsh] | 4.4351 ms | 4.6024 ms | 6000004 | 5378496 | 5346866 | 9.0098 ms |
| [capnp 0.23.2][capnp] | 6.0300 ms | † | 14000088 | 7130367 | 6046182 | 69.664 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 6.1226 ms | 48.742 ms | 13125016 | 7524114 | 6757437 | 81.365 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 49.601 ms | 94.380 ms | 13122324 | 7524660 | 6759128 | 78.637 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 28.687 ms | 35.535 ms | 13122324 | 7524660 | 6759128 | 81.387 ms |
| [columnar 0.11.1][columnar] | 1.7149 ms | 1.4057 ms <span title="copy_from">*589.89 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.6480 ms |
| [compactly 0.1.6][compactly] | 393.59 ms | 293.82 ms | 4846786 | 4850065 | 4846903 | 2.3803 ms |
| [databuf 0.5.0][databuf] | 1.2332 ms | 5.0589 ms | 6000003 | 5378495 | 5346897 | 9.4766 ms |
| [dlhn 0.1.7][dlhn] | 4.5898 ms | 8.1642 ms | 6000003 | 5378495 | 5346897 | 9.2801 ms |
| [flatbuffers 25.12.19][flatbuffers] | 709.39 µs | † | 6000024 | 5378434 | 5346878 | 9.0585 ms |
| [flexbuffers 25.2.10][flexbuffers] | 96.602 ms | 67.318 ms | 26609424 | 11901040 | 12486322 | 133.71 ms |
| json:<br> [flexon 0.4.5][flexon] | 71.443 ms | 57.044 ms | 26192883 | 9566084 | 8584671 | 139.23 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 80.709 ms | 92.811 ms | 26192883 | 9566084 | 8584671 | 138.82 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 51.823 ms | 76.718 ms | 26192883 | 9566084 | 8584671 | 144.27 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 17.351 ms | 5.6667 ms | 7500005 | 6058442 | 6014500 | 10.797 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 14.026 ms | 15.963 ms | 8125006 | 6494876 | 6391037 | 55.943 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 748.45 µs | 4.6652 ms | 8125006 | 6494876 | 6391037 | 55.291 ms |
| [minicbor 1.0.0][minicbor] | 3.2220 ms | 11.027 ms | 8125006 | 6494907 | 6390894 | 54.745 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 113.55 ms | 26.552 ms | 8125037 | 6493484 | 6386940 | 55.005 ms |
| [nanoserde 0.2.1][nanoserde] | 2.3254 ms | 825.95 µs | 6000008 | 5378500 | 5346908 | 9.0436 ms |
| [nibblecode 0.1.0][nibblecode] | 378.60 µs | † | 6000008 | 5378500 | 5346908 | 8.5749 ms |
| [postcard 1.1.1][postcard] | 453.99 µs | 1.8949 ms | 6000003 | 5378495 | 5346897 | 8.8231 ms |
| [pot 3.0.1][pot] | 29.937 ms | 57.557 ms | 10122342 | 6814618 | 6852252 | 66.188 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.1570 ms\**</span> <span title="populate + encode">*10.388 ms\**</span> | 10.506 ms | 8750000 | 6665735 | 6421877 | 57.748 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.606 ms\**</span> <span title="populate + encode">*34.895 ms\**</span> | 29.198 ms | 8750000 | 6665735 | 6421877 | 64.308 ms |
| [rkyv 0.8.10][rkyv] | 377.83 µs | <span title="unvalidated">*378.44 µs\**</span> <span title="validated upfront with error">*436.40 µs\**</span> | 6000008 | 5378500 | 5346872 | 9.0467 ms |
| [ron 0.10.1][ron] | 168.02 ms | 434.69 ms | 22192885 | 8970395 | 8137334 | 135.24 ms |
| [savefile 0.18.6][savefile] | 447.36 µs | 378.38 µs | 6000024 | 5378519 | 5346896 | 8.7875 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.5616 ms | 4.4121 ms | 6000004 | 5378496 | 5346866 | 8.5236 ms |
| [serde-brief 0.1.1][serde-brief] | 13.721 ms | 29.256 ms | 15750015 | 8024540 | 6813667 | 82.350 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8559 ms | 5.0555 ms | 6000003 | 5378495 | 5346897 | 9.0169 ms |
| [speedy 0.8.7][speedy] | 443.19 µs | 446.96 µs | 6000004 | 5378496 | 5346866 | 9.0429 ms |
| [wincode 0.2.4][wincode] | 435.19 µs | 435.59 µs | 6000008 | 5378500 | 5346908 | 8.7216 ms |
| [wiring 0.2.4][wiring] | 1.6952 ms | 398.81 µs | 6000008 | 5378952 | 5346905 | 8.9058 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*117.07 ns\**</span> | <span title="validated on-demand with error">*1.7204 ms\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 17.965 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*1.9271 ns\**</span> <span title="validated upfront with error">*40.608 ns\**</span> | <span title="unvalidated">*62.652 µs\**</span> <span title="validated upfront with error">*72.464 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="validated upfront with error">*1.4419 ns\**</span> | <span title="unvalidated">*26.039 µs\**</span> <span title="validated upfront with error">*26.120 µs\**</span> | <span title="unvalidated">*225.34 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="validated upfront with error">*4.7807 ns\**</span> | <span title="unvalidated">*27.076 µs\**</span> <span title="validated upfront with error">*25.235 µs\**</span> | <span title="unvalidated">*227.82 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.79%\**</span> <span title="prepend">*7.61%\**</span> | 4.34% | 56.19% | 75.27% | 77.78% | 3.97% |
| [bin-proto 0.12.7][bin-proto] | 5.36% | 4.35% | 80.78% | 90.18% | 90.65% | 26.67% |
| [bincode 2.0.1][bincode] | 28.88% | 41.34% | 80.78% | 90.18% | 90.65% | 27.33% |
| [bincode 1.3.3][bincode1] | 8.54% | 7.12% | 80.78% | 90.18% | 90.65% | 27.41% |
| [bitcode 0.6.6][bitcode] | 16.79% | 19.88% | 80.78% | 93.59% | 98.48% | 17.23% |
| [borsh 1.5.7][borsh] | 8.52% | 8.22% | 80.78% | 90.18% | 90.65% | 26.42% |
| [capnp 0.23.2][capnp] | 6.27% | † | 34.62% | 68.02% | 80.16% | 3.42% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 6.17% | 0.78% | 36.93% | 64.46% | 71.73% | 2.93% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.76% | 0.40% | 36.94% | 64.46% | 71.71% | 3.03% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.32% | 1.06% | 36.94% | 64.46% | 71.71% | 2.92% |
| [columnar 0.11.1][columnar] | 22.03% | 26.92% <span title="copy_from">*64.14%\**</span> | 80.78% | 90.18% | 90.65% | 27.52% |
| [compactly 0.1.6][compactly] | 0.10% | 0.13% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 30.64% | 7.48% | 80.78% | 90.18% | 90.65% | 25.12% |
| [dlhn 0.1.7][dlhn] | 8.23% | 4.63% | 80.78% | 90.18% | 90.65% | 25.65% |
| [flatbuffers 25.12.19][flatbuffers] | 53.26% | † | 80.78% | 90.18% | 90.65% | 26.28% |
| [flexbuffers 25.2.10][flexbuffers] | 0.39% | 0.56% | 18.21% | 40.75% | 38.82% | 1.78% |
| json:<br> [flexon 0.4.5][flexon] | 0.53% | 0.66% | 18.50% | 50.70% | 56.46% | 1.71% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.47% | 0.41% | 18.50% | 50.70% | 56.46% | 1.71% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.73% | 0.49% | 18.50% | 50.70% | 56.46% | 1.65% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.18% | 6.68% | 64.62% | 80.05% | 80.59% | 22.05% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 2.69% | 2.37% | 59.65% | 74.68% | 75.84% | 4.25% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 50.48% | 8.11% | 59.65% | 74.68% | 75.84% | 4.31% |
| [minicbor 1.0.0][minicbor] | 11.73% | 3.43% | 59.65% | 74.67% | 75.84% | 4.35% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.33% | 1.43% | 59.65% | 74.69% | 75.89% | 4.33% |
| [nanoserde 0.2.1][nanoserde] | 16.25% | 45.81% | 80.78% | 90.18% | 90.65% | 26.32% |
| [nibblecode 0.1.0][nibblecode] | 99.80% | † | 80.78% | 90.18% | 90.65% | 27.76% |
| [postcard 1.1.1][postcard] | 83.22% | 19.97% | 80.78% | 90.18% | 90.65% | 26.98% |
| [pot 3.0.1][pot] | 1.26% | 0.66% | 47.88% | 71.17% | 70.73% | 3.60% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*4.13%\**</span> <span title="populate + encode">*3.64%\**</span> | 3.60% | 55.39% | 72.76% | 75.47% | 4.12% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*2.59%\**</span> <span title="populate + encode">*1.08%\**</span> | 1.30% | 55.39% | 72.76% | 75.47% | 3.70% |
| [rkyv 0.8.10][rkyv] | 100.00% | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*86.70%\**</span> | 80.78% | 90.18% | 90.65% | 26.31% |
| [ron 0.10.1][ron] | 0.22% | 0.09% | 21.84% | 54.07% | 59.56% | 1.76% |
| [savefile 0.18.6][savefile] | 84.46% | 100.00% | 80.78% | 90.17% | 90.65% | 27.09% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 10.61% | 8.58% | 80.78% | 90.18% | 90.65% | 27.93% |
| [serde-brief 0.1.1][serde-brief] | 2.75% | 1.29% | 30.77% | 60.44% | 71.14% | 2.89% |
| [serde_bare 0.5.0][serde_bare] | 7.78% | 7.48% | 80.78% | 90.18% | 90.65% | 26.40% |
| [speedy 0.8.7][speedy] | 85.25% | 84.66% | 80.78% | 90.18% | 90.65% | 26.32% |
| [wincode 0.2.4][wincode] | 86.82% | 86.87% | 80.78% | 90.18% | 90.65% | 27.29% |
| [wiring 0.2.4][wiring] | 22.29% | 94.88% | 80.78% | 90.17% | 90.65% | 26.73% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.23%\**</span> | <span title="validated on-demand with error">*1.47%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 8.03% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*74.82%\**</span> <span title="validated upfront with error">*3.55%\**</span> | <span title="unvalidated">*40.28%\**</span> <span title="validated upfront with error">*34.82%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*96.91%\**</span> <span title="validated upfront with error">*96.61%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="validated upfront with error">*30.16%\**</span> | <span title="unvalidated">*93.20%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*98.91%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*955.39 µs\**</span> <span title="prepend">*850.24 µs\**</span> | 3.1302 ms | 1.7352 ms | 489348 | 281173 | 249360 | 2.1810 ms |
| [bin-proto 0.12.7][bin-proto] | 1.6602 ms | 2.6988 ms | † | 566975 | 239350 | 231475 | 2.1050 ms |
| [bincode 2.0.1][bincode] | 318.50 µs | 1.8315 ms | 801.52 µs | 367413 | 221291 | 206242 | 1.6859 ms |
| [bincode 1.3.3][bincode1] | 502.60 µs | 1.8579 ms | 911.40 µs | 569975 | 240525 | 231884 | 2.0963 ms |
| [bitcode 0.6.6][bitcode] | 159.71 µs | 1.3992 ms | 182.47 µs | 327688 | 200947 | 182040 | 719.03 µs |
| [borsh 1.5.7][borsh] | 450.87 µs | 1.8641 ms | † | 446595 | 234236 | 209834 | 1.7131 ms |
| [capnp 0.23.2][capnp] | 453.04 µs | † | † | 803896 | 335606 | 280744 | 3.1340 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 689.63 µs | 4.8059 ms | 3.5545 ms | 1109831 | 344745 | 274333 | 3.1283 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.7831 ms | 8.5945 ms | † | 1109821 | 344751 | 274345 | 3.1322 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.5932 ms | 4.6408 ms | 3.4210 ms | 1109821 | 344751 | 274345 | 3.1628 ms |
| [columnar 0.11.1][columnar] | 277.34 µs | 2.0211 ms <span title="copy_from">*791.52 µs\**</span> | † | 563728 | 249696 | 217582 | 1.4772 ms |
| [compactly 0.1.6][compactly] | 12.590 ms | 11.524 ms | † | 149292 | 149433 | 149304 | 103.07 µs |
| [databuf 0.5.0][databuf] | 294.29 µs | 1.7416 ms | 784.75 µs | 356311 | 213062 | 198403 | 1.6459 ms |
| [dlhn 0.1.7][dlhn] | 562.72 µs | 2.6063 ms | † | 366496 | 220600 | 205586 | 1.7052 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.4113 ms | † | † | 849472 | 347816 | 294871 | 3.0031 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.0152 ms | 6.1556 ms | 4.8340 ms | 1187688 | 557642 | 553730 | 5.1395 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.3295 ms | 4.5582 ms | † | 1623191 | 466527 | 359157 | 5.0401 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.3488 ms | 7.1315 ms | † | 1623191 | 466527 | 359157 | 5.0914 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.0396 ms | 4.6216 ms | † | 1623191 | 466527 | 359157 | 5.0979 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 921.78 µs | 2.7589 ms | † | 391251 | 236877 | 220395 | 1.8057 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.3223 ms | 2.8670 ms | 1.6142 ms | 424533 | 245214 | 226077 | 1.8526 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 374.31 µs | 2.3018 ms | 1.0202 ms | 416025 | 243812 | 224965 | 1.8463 ms |
| [minicbor 1.0.0][minicbor] | 479.06 µs | 3.4166 ms | 1.9093 ms | 428773 | 249857 | 228630 | 1.8976 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 4.7710 ms | 3.8533 ms | 2.6198 ms | 449745 | 252432 | 230965 | 1.9073 ms |
| [nanoserde 0.2.1][nanoserde] | 258.84 µs | 1.9485 ms | † | 567975 | 239930 | 231872 | 2.1146 ms |
| [nibblecode 0.1.0][nibblecode] | 170.79 µs | † | † | 603928 | 428585 | 404102 | 2.8806 ms |
| [postcard 1.1.1][postcard] | 386.29 µs | 2.0621 ms | 834.25 µs | 367489 | 221913 | 207244 | 1.6731 ms |
| [pot 3.0.1][pot] | 2.0414 ms | 5.3286 ms | 4.0271 ms | 599125 | 299158 | 247675 | 2.2768 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.3303 ms\**</span> <span title="populate + encode">*3.3198 ms\**</span> | 3.5162 ms | † | 596811 | 305319 | 268737 | 2.5232 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2345 ms\**</span> <span title="populate + encode">*3.3660 ms\**</span> | 3.7881 ms | † | 596811 | 305319 | 268737 | 2.4874 ms |
| [rkyv 0.8.10][rkyv] | 299.83 µs | <span title="unvalidated">*1.5704 ms\**</span> <span title="validated upfront with error">*1.9452 ms\**</span> | † | 603776 | 254776 | 219421 | 1.9466 ms |
| [ron 0.10.1][ron] | 8.1673 ms | 22.120 ms | 20.380 ms | 1465223 | 434935 | 342907 | 4.8142 ms |
| [savefile 0.18.6][savefile] | 201.80 µs | 1.8325 ms | † | 566991 | 239362 | 231478 | 2.1135 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 481.27 µs | 2.0487 ms | † | 356311 | 212976 | 198423 | 1.6254 ms |
| [serde-brief 0.1.1][serde-brief] | 1.1350 ms | 5.2207 ms | 3.4782 ms | 1276014 | 373898 | 293384 | 3.3933 ms |
| [serde_bare 0.5.0][serde_bare] | 633.36 µs | 2.4018 ms | † | 356311 | 213062 | 198403 | 1.6392 ms |
| [speedy 0.8.7][speedy] | 267.19 µs | 1.7507 ms | 612.84 µs | 449595 | 234970 | 210192 | 1.7313 ms |
| [wincode 0.2.4][wincode] | 183.78 µs | 1.7135 ms | 683.32 µs | 566975 | 239350 | 231475 | 2.1167 ms |
| [wiring 0.2.4][wiring] | 205.10 µs | 2.0023 ms | † | 566975 | 247810 | 225086 | 2.1653 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*68.112 ns\**</span> | <span title="validated on-demand with error">*374.91 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 583.08 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*1.9231 ns\**</span> <span title="validated upfront with error">*1.9100 ms\**</span> | <span title="unvalidated">*1.2992 µs\**</span> <span title="validated upfront with error">*1.9158 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="validated upfront with error">*286.80 µs\**</span> | <span title="unvalidated">*99.759 ns\**</span> <span title="validated upfront with error">*287.29 µs\**</span> | <span title="unvalidated">*1.2360 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="validated upfront with error">*352.99 µs\**</span> | <span title="unvalidated">*100.11 ns\**</span> <span title="validated upfront with error">*353.94 µs\**</span> | <span title="unvalidated">*1.2358 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*16.72%\**</span> <span title="prepend">*18.78%\**</span> | 25.29% | 10.52% | 30.51% | 53.15% | 59.87% | 4.73% |
| [bin-proto 0.12.7][bin-proto] | 9.62% | 29.33% | † | 26.33% | 62.43% | 64.50% | 4.90% |
| [bincode 2.0.1][bincode] | 50.14% | 43.22% | 22.77% | 40.63% | 67.53% | 72.39% | 6.11% |
| [bincode 1.3.3][bincode1] | 31.78% | 42.60% | 20.02% | 26.19% | 62.13% | 64.39% | 4.92% |
| [bitcode 0.6.6][bitcode] | 100.00% | 56.57% | 100.00% | 45.56% | 74.36% | 82.02% | 14.34% |
| [borsh 1.5.7][borsh] | 35.42% | 42.46% | † | 33.43% | 63.80% | 71.15% | 6.02% |
| [capnp 0.23.2][capnp] | 35.25% | † | † | 18.57% | 44.53% | 53.18% | 3.29% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 23.16% | 16.47% | 5.13% | 13.45% | 43.35% | 54.42% | 3.29% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 5.74% | 9.21% | † | 13.45% | 43.35% | 54.42% | 3.29% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.02% | 17.06% | 5.33% | 13.45% | 43.35% | 54.42% | 3.26% |
| [columnar 0.11.1][columnar] | 57.59% | 39.16% <span title="copy_from">*100.00%\**</span> | † | 26.48% | 59.85% | 68.62% | 6.98% |
| [compactly 0.1.6][compactly] | 1.27% | 6.87% | † | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 54.27% | 45.45% | 23.25% | 41.90% | 70.14% | 75.25% | 6.26% |
| [dlhn 0.1.7][dlhn] | 28.38% | 30.37% | † | 40.73% | 67.74% | 72.62% | 6.04% |
| [flatbuffers 25.12.19][flatbuffers] | 4.68% | † | † | 17.57% | 42.96% | 50.63% | 3.43% |
| [flexbuffers 25.2.10][flexbuffers] | 2.28% | 12.86% | 3.77% | 12.57% | 26.80% | 26.96% | 2.01% |
| json:<br> [flexon 0.4.5][flexon] | 6.86% | 17.36% | † | 9.20% | 32.03% | 41.57% | 2.05% |
| json:<br> [serde_json 1.0.140][serde_json] | 4.77% | 11.10% | † | 9.20% | 32.03% | 41.57% | 2.02% |
| json:<br> [simd-json 0.15.1][simd-json] | 7.83% | 17.13% | † | 9.20% | 32.03% | 41.57% | 2.02% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 17.33% | 28.69% | † | 38.16% | 63.08% | 67.74% | 5.71% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 12.08% | 27.61% | 11.30% | 35.17% | 60.94% | 66.04% | 5.56% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 42.67% | 34.39% | 17.89% | 35.89% | 61.29% | 66.37% | 5.58% |
| [minicbor 1.0.0][minicbor] | 33.34% | 23.17% | 9.56% | 34.82% | 59.81% | 65.30% | 5.43% |
| [nachricht-serde 0.4.0][nachricht-serde] | 3.35% | 20.54% | 6.97% | 33.19% | 59.20% | 64.64% | 5.40% |
| [nanoserde 0.2.1][nanoserde] | 61.70% | 40.62% | † | 26.28% | 62.28% | 64.39% | 4.87% |
| [nibblecode 0.1.0][nibblecode] | 93.51% | † | † | 24.72% | 34.87% | 36.95% | 3.58% |
| [postcard 1.1.1][postcard] | 41.34% | 38.38% | 21.87% | 40.62% | 67.34% | 72.04% | 6.16% |
| [pot 3.0.1][pot] | 7.82% | 14.85% | 4.53% | 24.92% | 49.95% | 60.28% | 4.53% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*12.01%\**</span> <span title="populate + encode">*4.81%\**</span> | 22.51% | † | 25.01% | 48.94% | 55.56% | 4.09% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.94%\**</span> <span title="populate + encode">*4.74%\**</span> | 20.89% | † | 25.01% | 48.94% | 55.56% | 4.14% |
| [rkyv 0.8.10][rkyv] | 53.27% | <span title="unvalidated">*50.40%\**</span> <span title="validated upfront with error">*40.69%\**</span> | † | 24.73% | 58.65% | 68.04% | 5.30% |
| [ron 0.10.1][ron] | 1.96% | 3.58% | 0.90% | 10.19% | 34.36% | 43.54% | 2.14% |
| [savefile 0.18.6][savefile] | 79.14% | 43.19% | † | 26.33% | 62.43% | 64.50% | 4.88% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 33.19% | 38.64% | † | 41.90% | 70.16% | 75.25% | 6.34% |
| [serde-brief 0.1.1][serde-brief] | 14.07% | 15.16% | 5.25% | 11.70% | 39.97% | 50.89% | 3.04% |
| [serde_bare 0.5.0][serde_bare] | 25.22% | 32.96% | † | 41.90% | 70.14% | 75.25% | 6.29% |
| [speedy 0.8.7][speedy] | 59.77% | 45.21% | 29.77% | 33.21% | 63.60% | 71.03% | 5.95% |
| [wincode 0.2.4][wincode] | 86.90% | 46.19% | 26.70% | 26.33% | 62.43% | 64.50% | 4.87% |
| [wiring 0.2.4][wiring] | 77.87% | 39.53% | † | 26.33% | 60.30% | 66.33% | 4.76% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*2.82%\**</span> | <span title="validated on-demand with error">*26.61%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 0.33% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*7.68%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*99.98%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.65%\**</span> <span title="validated upfront with error">*0.03%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.6203 ms\**</span> <span title="prepend">*2.4907 ms\**</span> | 8.6312 ms | 1704643 | 1294259 | 1245668 | 9.7042 ms |
| [bin-proto 0.12.7][bin-proto] | 4.1591 ms | 5.9255 ms | 1791489 | 1127998 | 1051146 | 8.8349 ms |
| [bincode 2.0.1][bincode] | 1.0634 ms | 3.4034 ms | 1406257 | 1117802 | 1062438 | 7.6366 ms |
| [bincode 1.3.3][bincode1] | 2.6651 ms | 4.3039 ms | 1854234 | 1141994 | 1048745 | 8.7093 ms |
| [bitcode 0.6.6][bitcode] | 726.87 µs | 2.4905 ms | 971318 | 878034 | 850340 | 2.6116 ms |
| [borsh 1.5.7][borsh] | 2.0565 ms | 2.9227 ms | 1521989 | 1108471 | 1038528 | 8.2114 ms |
| [capnp 0.23.2][capnp] | 1.9125 ms | † | 2724288 | 1546992 | 1239111 | 12.438 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 2.2085 ms | 18.891 ms | 6012539 | 1695215 | 1464951 | 20.015 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 16.970 ms | 42.151 ms | 6012373 | 1695146 | 1465025 | 20.131 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 8.2665 ms | 19.174 ms | 6012373 | 1695146 | 1465025 | 21.312 ms |
| [columnar 0.11.1][columnar] | 909.77 µs | 3.6710 ms <span title="copy_from">*1.3260 ms\**</span> | 1544752 | 996728 | 897073 | 4.1946 ms |
| [compactly 0.1.6][compactly] | 70.127 ms | 59.668 ms | 802662 | 803238 | 802689 | 387.39 µs |
| [databuf 0.5.0][databuf] | 959.86 µs | 3.6735 ms | 1319999 | 1062631 | 1008334 | 7.2843 ms |
| [dlhn 0.1.7][dlhn] | 3.3308 ms | 7.5430 ms | 1311281 | 1077520 | 1046095 | 7.1316 ms |
| [flatbuffers 25.12.19][flatbuffers] | 4.7269 ms | † | 2325620 | 1439185 | 1268060 | 11.900 ms |
| [flexbuffers 25.2.10][flexbuffers] | 35.549 ms | 30.915 ms | 5352680 | 2658295 | 2777967 | 30.533 ms |
| json:<br> [flexon 0.4.5][flexon] | 12.062 ms | 23.698 ms | 9390461 | 2391679 | 1842767 | 35.065 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 18.841 ms | 32.089 ms | 9390461 | 2391679 | 1842767 | 35.062 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 10.404 ms | 28.017 ms | 9390461 | 2391679 | 1842767 | 34.079 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.9731 ms | 5.4082 ms | 1458773 | 1156055 | 1137788 | 8.1279 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.1975 ms | 9.6786 ms | 1745322 | 1261627 | 1228923 | 9.4152 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 1.7541 ms | 5.9001 ms | 1794467 | 1273669 | 1242301 | 9.4158 ms |
| [minicbor 1.0.0][minicbor] | 1.7485 ms | 10.669 ms | 1777386 | 1276218 | 1252558 | 10.393 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 26.615 ms | 15.507 ms | 1770060 | 1277755 | 1263362 | 10.361 ms |
| [nanoserde 0.2.1][nanoserde] | 923.66 µs | 2.7789 ms | 1812404 | 1134820 | 1053109 | 8.9546 ms |
| [nibblecode 0.1.0][nibblecode] | 511.38 µs | † | 2075936 | 1518443 | 1413193 | 11.487 ms |
| [postcard 1.1.1][postcard] | 1.6987 ms | 4.0731 ms | 1311281 | 1083900 | 1041434 | 7.1799 ms |
| [pot 3.0.1][pot] | 11.481 ms | 25.880 ms | 2604812 | 1482233 | 1298928 | 14.436 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.7000 ms\**</span> <span title="populate + encode">*10.038 ms\**</span> | 8.7994 ms | 1859886 | 1338076 | 1295351 | 10.335 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*4.9203 ms\**</span> <span title="populate + encode">*13.854 ms\**</span> | 13.259 ms | 1859886 | 1338076 | 1295351 | 10.259 ms |
| [rkyv 0.8.10][rkyv] | 891.90 µs | <span title="unvalidated">*2.1917 ms\**</span> <span title="validated upfront with error">*2.7189 ms\**</span> | 2075936 | 1383779 | 1210377 | 11.142 ms |
| [ron 0.10.1][ron] | 44.120 ms | 129.77 ms | 8677703 | 2233642 | 1826180 | 33.405 ms |
| [savefile 0.18.6][savefile] | 659.47 µs | 2.6323 ms | 1791505 | 1128012 | 1051153 | 8.7946 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.2401 ms | 3.2894 ms | 1319999 | 1064380 | 1010708 | 7.4638 ms |
| [serde-brief 0.1.1][serde-brief] | 4.5386 ms | 19.313 ms | 6951772 | 1796265 | 1567819 | 22.878 ms |
| [serde_bare 0.5.0][serde_bare] | 3.5539 ms | 5.0482 ms | 1319999 | 1062645 | 1008349 | 7.3314 ms |
| [speedy 0.8.7][speedy] | 756.80 µs | 2.5420 ms | 1584734 | 1119837 | 1037992 | 8.2293 ms |
| [wincode 0.2.4][wincode] | 546.40 µs | 2.4153 ms | 1791489 | 1127998 | 1051146 | 9.0755 ms |
| [wiring 0.2.4][wiring] | 632.13 µs | 2.8727 ms | 1791489 | 1156963 | 1082815 | 9.1439 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*67.991 ns\**</span> | <span title="validated on-demand with error">*775.10 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 49.403 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*1.9293 ns\**</span> <span title="validated upfront with error">*4.7575 ms\**</span> | <span title="unvalidated">*2.4986 µs\**</span> <span title="validated upfront with error">*4.7641 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="validated upfront with error">*433.61 µs\**</span> | <span title="unvalidated">*303.05 ns\**</span> <span title="validated upfront with error">*435.41 µs\**</span> | <span title="unvalidated">*355.04 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="validated upfront with error">*479.50 µs\**</span> | <span title="unvalidated">*304.67 ns\**</span> <span title="validated upfront with error">*479.95 µs\**</span> | <span title="unvalidated">*354.71 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.07%\**</span> <span title="prepend">*20.53%\**</span> | 15.36% | 47.09% | 62.06% | 64.44% | 3.99% |
| [bin-proto 0.12.7][bin-proto] | 12.30% | 22.38% | 44.80% | 71.21% | 76.36% | 4.38% |
| [bincode 2.0.1][bincode] | 48.09% | 38.96% | 57.08% | 71.86% | 75.55% | 5.07% |
| [bincode 1.3.3][bincode1] | 19.19% | 30.81% | 43.29% | 70.34% | 76.54% | 4.45% |
| [bitcode 0.6.6][bitcode] | 70.35% | 53.24% | 82.64% | 91.48% | 94.40% | 14.83% |
| [borsh 1.5.7][borsh] | 24.87% | 45.37% | 52.74% | 72.46% | 77.29% | 4.72% |
| [capnp 0.23.2][capnp] | 26.74% | † | 29.46% | 51.92% | 64.78% | 3.11% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 23.16% | 7.02% | 13.35% | 47.38% | 54.79% | 1.94% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.01% | 3.15% | 13.35% | 47.38% | 54.79% | 1.92% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.19% | 6.92% | 13.35% | 47.38% | 54.79% | 1.82% |
| [columnar 0.11.1][columnar] | 56.21% | 36.12% <span title="copy_from">*100.00%\**</span> | 51.96% | 80.59% | 89.48% | 9.24% |
| [compactly 0.1.6][compactly] | 0.73% | 2.22% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 53.28% | 36.10% | 60.81% | 75.59% | 79.61% | 5.32% |
| [dlhn 0.1.7][dlhn] | 15.35% | 17.58% | 61.21% | 74.55% | 76.73% | 5.43% |
| [flatbuffers 25.12.19][flatbuffers] | 10.82% | † | 34.51% | 55.81% | 63.30% | 3.26% |
| [flexbuffers 25.2.10][flexbuffers] | 1.44% | 4.29% | 15.00% | 30.22% | 28.89% | 1.27% |
| json:<br> [flexon 0.4.5][flexon] | 4.24% | 5.60% | 8.55% | 33.58% | 43.56% | 1.10% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.71% | 4.13% | 8.55% | 33.58% | 43.56% | 1.10% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.92% | 4.73% | 8.55% | 33.58% | 43.56% | 1.14% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 25.92% | 24.52% | 55.02% | 69.48% | 70.55% | 4.77% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 5.56% | 13.70% | 45.99% | 63.67% | 65.32% | 4.11% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 29.15% | 22.47% | 44.73% | 63.06% | 64.61% | 4.11% |
| [minicbor 1.0.0][minicbor] | 29.25% | 12.43% | 45.16% | 62.94% | 64.08% | 3.73% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.92% | 8.55% | 45.35% | 62.86% | 63.54% | 3.74% |
| [nanoserde 0.2.1][nanoserde] | 55.36% | 47.72% | 44.29% | 70.78% | 76.22% | 4.33% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 38.67% | 52.90% | 56.80% | 3.37% |
| [postcard 1.1.1][postcard] | 30.10% | 32.56% | 61.21% | 74.11% | 77.08% | 5.40% |
| [pot 3.0.1][pot] | 4.45% | 5.12% | 30.81% | 54.19% | 61.80% | 2.68% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*8.97%\**</span> <span title="populate + encode">*5.09%\**</span> | 15.07% | 43.16% | 60.03% | 61.97% | 3.75% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*10.39%\**</span> <span title="populate + encode">*3.69%\**</span> | 10.00% | 43.16% | 60.03% | 61.97% | 3.78% |
| [rkyv 0.8.10][rkyv] | 57.34% | <span title="unvalidated">*60.50%\**</span> <span title="validated upfront with error">*48.77%\**</span> | 38.67% | 58.05% | 66.32% | 3.48% |
| [ron 0.10.1][ron] | 1.16% | 1.02% | 9.25% | 35.96% | 43.95% | 1.16% |
| [savefile 0.18.6][savefile] | 77.54% | 50.37% | 44.80% | 71.21% | 76.36% | 4.40% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.83% | 40.31% | 60.81% | 75.47% | 79.42% | 5.19% |
| [serde-brief 0.1.1][serde-brief] | 11.27% | 6.87% | 11.55% | 44.72% | 51.20% | 1.69% |
| [serde_bare 0.5.0][serde_bare] | 14.39% | 26.27% | 60.81% | 75.59% | 79.60% | 5.28% |
| [speedy 0.8.7][speedy] | 67.57% | 52.16% | 50.65% | 71.73% | 77.33% | 4.71% |
| [wincode 0.2.4][wincode] | 93.59% | 54.90% | 44.80% | 71.21% | 76.36% | 4.27% |
| [wiring 0.2.4][wiring] | 80.90% | 46.16% | 44.80% | 69.43% | 74.13% | 4.24% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*2.84%\**</span> | <span title="validated on-demand with error">*39.10%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 3.91% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*12.13%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.07%\**</span> | <span title="unvalidated">*99.91%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.47%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
[zerompk]: https://crates.io/crates/zerompk/0.3.2


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
