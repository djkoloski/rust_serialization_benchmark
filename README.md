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

## Last updated: 2025-12-29 14:41:09

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.94.0-nightly (21cf7fb3f 2025-12-28)
binary: rustc
commit-hash: 21cf7fb3ff9159b0b562431312969dd548ae8782
commit-date: 2025-12-28
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*430.81 µs\**</span> <span title="prepend">*414.41 µs\**</span> | 2.5859 ms | 904.57 µs | 804955 | 328941 | 284849 | 4.1368 ms |
| [bin-proto 0.12.3][bin-proto] | 4.5709 ms | 4.8591 ms | † | 1045784 | 373127 | 311553 | 4.5470 ms |
| [bincode 2.0.1][bincode] | 340.35 µs | 2.2388 ms | 698.32 µs | 741295 | 303944 | 256422 | 3.8231 ms |
| [bincode 1.3.3][bincode1] | 549.47 µs | 1.9867 ms | 608.77 µs | 1045784 | 373127 | 311553 | 4.4861 ms |
| [bitcode 0.6.6][bitcode] | 145.20 µs | 1.4493 ms | 62.747 µs | 703710 | 288826 | 227322 | 2.5470 ms |
| [borsh 1.5.7][borsh] | 545.83 µs | 2.1496 ms | † | 885780 | 362204 | 286248 | 4.2676 ms |
| [capnp 0.23.2][capnp] | 454.27 µs | † | † | 1443216 | 513986 | 426532 | 6.2450 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 678.77 µs | 5.0249 ms | 3.4191 ms | 1407835 | 403440 | 323561 | 5.0361 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1406 ms | 11.655 ms | † | 1407835 | 403440 | 323561 | 5.0506 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9456 ms | 4.9684 ms | 3.2609 ms | 1407835 | 403440 | 323561 | 4.9734 ms |
| [databuf 0.5.0][databuf] | 259.34 µs | 2.0292 ms | 657.03 µs | 765778 | 311715 | 263914 | 3.5990 ms |
| [dlhn 0.1.7][dlhn] | 717.60 µs | 2.5684 ms | † | 724953 | 301446 | 253056 | 3.2456 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0304 ms | † | † | 1276368 | 468539 | 388381 | 4.9345 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.6189 ms | 7.8878 ms | 6.1374 ms | 1829756 | 714318 | 691541 | 8.8413 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.7257 ms | 5.9091 ms | † | 1827461 | 470560 | 360727 | 5.4699 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2358 ms | 4.6964 ms | † | 1827461 | 470560 | 360727 | 5.8961 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.2788 ms | 2.7031 ms | † | 764996 | 315291 | 264212 | 3.6950 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4524 ms | 3.0689 ms | 1.4010 ms | 784997 | 325384 | 277608 | 4.0573 ms |
| [minicbor 1.0.0][minicbor] | 578.65 µs | 2.9922 ms | 1.3338 ms | 817830 | 332671 | 284034 | 3.9399 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3769 ms | 4.5379 ms | 3.1894 ms | 818669 | 332556 | 284797 | 5.3090 ms |
| [nanoserde 0.2.1][nanoserde] | 254.11 µs | 2.0784 ms | † | 1045784 | 373127 | 311553 | 4.1242 ms |
| [nibblecode 0.1.0][nibblecode] | 196.01 µs | † | † | 1011487 | 491495 | 425439 | 5.7824 ms |
| [postcard 1.1.1][postcard] | 419.91 µs | 2.1330 ms | 618.10 µs | 724953 | 302399 | 252968 | 3.2056 ms |
| [pot 3.0.1][pot] | 2.4104 ms | 6.3442 ms | 4.8468 ms | 971922 | 372513 | 303636 | 4.3614 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*941.54 µs\**</span> <span title="populate + encode">*2.5378 ms\**</span> | 3.4290 ms | † | 884628 | 363130 | 314959 | 4.3921 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*973.89 µs\**</span> <span title="populate + encode">*2.7627 ms\**</span> | 3.7833 ms | † | 884628 | 363130 | 314959 | 4.6557 ms |
| [rkyv 0.8.10][rkyv] | 245.01 µs | <span title="unvalidated">*1.5414 ms\**</span> <span title="validated upfront with error">*1.9260 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5898 ms |
| [ron 0.10.1][ron] | 11.420 ms | 23.638 ms | 21.934 ms | 1607459 | 449158 | 349324 | 5.5900 ms |
| [savefile 0.18.6][savefile] | 190.25 µs | 2.1682 ms | † | 1045800 | 373139 | 311562 | 4.1349 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 652.78 µs | 2.3333 ms | † | 765778 | 311743 | 263822 | 3.8241 ms |
| [serde-brief 0.1.1][serde-brief] | 1.5138 ms | 4.8668 ms | 3.0576 ms | 1584946 | 413733 | 339964 | 5.5539 ms |
| [serde_bare 0.5.0][serde_bare] | 688.04 µs | 2.0935 ms | † | 765778 | 311715 | 263914 | 3.4521 ms |
| [speedy 0.8.7][speedy] | 204.27 µs | 1.7399 ms | 387.16 µs | 885780 | 362204 | 286248 | 3.8323 ms |
| [wincode 0.2.4][wincode] | 180.58 µs | 1.8689 ms | 484.92 µs | 1045784 | 373127 | 311553 | 4.2150 ms |
| [wiring 0.2.4][wiring] | 196.97 µs | 1.9673 ms | † | 1045784 | 337930 | 275808 | 3.5979 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*84.167 ns\**</span> | <span title="validated on-demand with error">*182.67 µs\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4865 ns\**</span> <span title="validated upfront with error">*2.1443 ms\**</span> | <span title="unvalidated">*49.339 µs\**</span> <span title="validated upfront with error">*2.1716 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2429 ns\**</span> <span title="validated upfront with error">*261.86 µs\**</span> | <span title="unvalidated">*10.453 µs\**</span> <span title="validated upfront with error">*273.30 µs\**</span> | <span title="unvalidated">*7.5352 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2444 ns\**</span> <span title="validated upfront with error">*372.89 µs\**</span> | <span title="unvalidated">*10.380 µs\**</span> <span title="validated upfront with error">*382.72 µs\**</span> | <span title="unvalidated">*7.5335 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*33.70%\**</span> <span title="prepend">*35.04%\**</span> | 56.05% | 6.94% | 87.42% | 87.80% | 79.80% | 61.57% |
| [bin-proto 0.12.3][bin-proto] | 3.18% | 29.83% | † | 67.29% | 77.41% | 72.96% | 56.01% |
| [bincode 2.0.1][bincode] | 42.66% | 64.74% | 8.99% | 94.93% | 95.03% | 88.65% | 66.62% |
| [bincode 1.3.3][bincode1] | 26.43% | 72.95% | 10.31% | 67.29% | 77.41% | 72.96% | 56.78% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 26.60% | 67.42% | † | 79.45% | 79.74% | 79.41% | 59.68% |
| [capnp 0.23.2][capnp] | 31.96% | † | † | 48.76% | 56.19% | 53.30% | 40.78% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 21.39% | 28.84% | 1.84% | 49.99% | 71.59% | 70.26% | 50.57% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.62% | 12.44% | † | 49.99% | 71.59% | 70.26% | 50.43% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.46% | 29.17% | 1.92% | 49.99% | 71.59% | 70.26% | 51.21% |
| [databuf 0.5.0][databuf] | 55.99% | 71.42% | 9.55% | 91.89% | 92.66% | 86.13% | 70.77% |
| [dlhn 0.1.7][dlhn] | 20.23% | 56.43% | † | 97.07% | 95.81% | 89.83% | 78.48% |
| [flatbuffers 25.12.19][flatbuffers] | 14.09% | † | † | 55.13% | 61.64% | 58.53% | 51.62% |
| [flexbuffers 25.2.10][flexbuffers] | 2.19% | 18.37% | 1.02% | 38.46% | 40.43% | 32.87% | 28.81% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.90% | 24.53% | † | 38.51% | 61.38% | 63.02% | 46.56% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.49% | 30.86% | † | 38.51% | 61.38% | 63.02% | 43.20% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 11.35% | 53.62% | † | 91.99% | 91.61% | 86.04% | 68.93% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.00% | 47.23% | 4.48% | 89.64% | 88.76% | 81.89% | 62.78% |
| [minicbor 1.0.0][minicbor] | 25.09% | 48.44% | 4.70% | 86.05% | 86.82% | 80.03% | 64.65% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.70% | 31.94% | 1.97% | 85.96% | 86.85% | 79.82% | 47.98% |
| [nanoserde 0.2.1][nanoserde] | 57.14% | 69.73% | † | 67.29% | 77.41% | 72.96% | 61.76% |
| [nibblecode 0.1.0][nibblecode] | 74.08% | † | † | 69.57% | 58.76% | 53.43% | 44.05% |
| [postcard 1.1.1][postcard] | 34.58% | 67.95% | 10.15% | 97.07% | 95.51% | 89.86% | 79.45% |
| [pot 3.0.1][pot] | 6.02% | 22.84% | 1.29% | 72.40% | 77.53% | 74.87% | 58.40% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*15.42%\**</span> <span title="populate + encode">*5.72%\**</span> | 42.27% | † | 79.55% | 79.54% | 72.18% | 57.99% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.91%\**</span> <span title="populate + encode">*5.26%\**</span> | 38.31% | † | 79.55% | 79.54% | 72.18% | 54.71% |
| [rkyv 0.8.10][rkyv] | 59.26% | <span title="unvalidated">*94.02%\**</span> <span title="validated upfront with error">*75.25%\**</span> | † | 69.57% | 73.39% | 69.74% | 55.49% |
| [ron 0.10.1][ron] | 1.27% | 6.13% | 0.29% | 43.78% | 64.30% | 65.07% | 45.56% |
| [savefile 0.18.6][savefile] | 76.32% | 66.84% | † | 67.29% | 77.40% | 72.96% | 61.60% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.24% | 62.11% | † | 91.89% | 92.65% | 86.16% | 66.60% |
| [serde-brief 0.1.1][serde-brief] | 9.59% | 29.78% | 2.05% | 44.40% | 69.81% | 66.87% | 45.86% |
| [serde_bare 0.5.0][serde_bare] | 21.10% | 69.23% | † | 91.89% | 92.66% | 86.13% | 73.78% |
| [speedy 0.8.7][speedy] | 71.08% | 83.30% | 16.21% | 79.45% | 79.74% | 79.41% | 66.46% |
| [wincode 0.2.4][wincode] | 80.41% | 77.55% | 12.94% | 67.29% | 77.41% | 72.96% | 60.43% |
| [wiring 0.2.4][wiring] | 73.72% | 73.67% | † | 67.29% | 85.47% | 82.42% | 70.79% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.48%\**</span> | <span title="validated on-demand with error">*5.68%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.04%\**</span> <span title="validated upfront with error">*0.48%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.30%\**</span> <span title="validated upfront with error">*3.80%\**</span> | <span title="unvalidated">*99.98%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.88%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.71%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.2769 ms\**</span> <span title="prepend">*8.5980 ms\**</span> | 7.8609 ms | 8625005 | 6443961 | 6231572 | 73.310 ms |
| [bin-proto 0.12.3][bin-proto] | 8.8588 ms | 10.413 ms | 6000008 | 5378500 | 5346908 | 8.5061 ms |
| [bincode 2.0.1][bincode] | 2.4132 ms | 988.30 µs | 6000005 | 5378497 | 5346882 | 8.6649 ms |
| [bincode 1.3.3][bincode1] | 5.1495 ms | 4.4583 ms | 6000008 | 5378500 | 5346908 | 8.5939 ms |
| [bitcode 0.6.6][bitcode] | 1.3176 ms | 796.17 µs | 6000006 | 5182295 | 4921841 | 13.308 ms |
| [borsh 1.5.7][borsh] | 6.0776 ms | 4.3110 ms | 6000004 | 5378496 | 5346866 | 8.4757 ms |
| [capnp 0.23.2][capnp] | 5.7657 ms | † | 14000088 | 7130367 | 6046182 | 84.102 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 9.9884 ms | 48.533 ms | 13125016 | 7524114 | 6757437 | 93.355 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 66.679 ms | 115.97 ms | 13122324 | 7524660 | 6759128 | 92.862 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 35.798 ms | 42.119 ms | 13122324 | 7524660 | 6759128 | 92.248 ms |
| [databuf 0.5.0][databuf] | 2.4116 ms | 5.3645 ms | 6000003 | 5378495 | 5346897 | 9.0422 ms |
| [dlhn 0.1.7][dlhn] | 6.2088 ms | 7.0234 ms | 6000003 | 5378495 | 5346897 | 8.7886 ms |
| [flatbuffers 25.12.19][flatbuffers] | 982.33 µs | † | 6000024 | 5378434 | 5346878 | 8.8820 ms |
| [flexbuffers 25.2.10][flexbuffers] | 102.90 ms | 102.85 ms | 26609424 | 11901040 | 12486322 | 155.91 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 88.548 ms | 100.37 ms | 26192883 | 9566084 | 8584671 | 157.02 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 53.451 ms | 71.457 ms | 26192883 | 9566084 | 8584671 | 156.81 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 20.418 ms | 5.1360 ms | 7500005 | 6058442 | 6014500 | 10.336 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 15.744 ms | 16.686 ms | 8125006 | 6494876 | 6391037 | 73.284 ms |
| [minicbor 1.0.0][minicbor] | 6.0563 ms | 11.692 ms | 8125006 | 6494907 | 6390894 | 71.226 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 122.33 ms | 31.066 ms | 8125037 | 6493484 | 6386940 | 72.778 ms |
| [nanoserde 0.2.1][nanoserde] | 1.4592 ms | 826.75 µs | 6000008 | 5378500 | 5346908 | 8.7467 ms |
| [nibblecode 0.1.0][nibblecode] | 200.09 µs | † | 6000008 | 5378500 | 5346908 | 8.9453 ms |
| [postcard 1.1.1][postcard] | 479.75 µs | 1.2762 ms | 6000003 | 5378495 | 5346897 | 8.8743 ms |
| [pot 3.0.1][pot] | 39.736 ms | 67.252 ms | 10122342 | 6814618 | 6852252 | 83.464 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*7.9009 ms\**</span> <span title="populate + encode">*8.7724 ms\**</span> | 11.314 ms | 8750000 | 6665735 | 6421877 | 74.157 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.617 ms\**</span> <span title="populate + encode">*31.429 ms\**</span> | 29.066 ms | 8750000 | 6665735 | 6421877 | 79.410 ms |
| [rkyv 0.8.10][rkyv] | 200.30 µs | <span title="unvalidated">*169.42 µs\**</span> <span title="validated upfront with error">*188.63 µs\**</span> | 6000008 | 5378500 | 5346872 | 9.2098 ms |
| [ron 0.10.1][ron] | 165.98 ms | 526.90 ms | 22192885 | 8970395 | 8137334 | 155.04 ms |
| [savefile 0.18.6][savefile] | 174.76 µs | 204.96 µs | 6000024 | 5378519 | 5346896 | 8.7369 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1547 ms | 4.1892 ms | 6000004 | 5378496 | 5346866 | 9.2388 ms |
| [serde-brief 0.1.1][serde-brief] | 22.656 ms | 34.690 ms | 15750015 | 8024540 | 6813667 | 96.880 ms |
| [serde_bare 0.5.0][serde_bare] | 6.0142 ms | 5.6380 ms | 6000003 | 5378495 | 5346897 | 9.2268 ms |
| [speedy 0.8.7][speedy] | 200.19 µs | 174.09 µs | 6000004 | 5378496 | 5346866 | 9.0264 ms |
| [wincode 0.2.4][wincode] | 193.61 µs | 192.57 µs | 6000008 | 5378500 | 5346908 | 8.5993 ms |
| [wiring 0.2.4][wiring] | 190.25 µs | 321.77 µs | 6000008 | 5378952 | 5346905 | 8.8553 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*115.57 ns\**</span> | <span title="validated on-demand with error">*2.2234 ms\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4856 ns\**</span> <span title="validated upfront with error">*46.874 ns\**</span> | <span title="unvalidated">*77.758 µs\**</span> <span title="validated upfront with error">*77.822 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2433 ns\**</span> <span title="validated upfront with error">*1.5574 ns\**</span> | <span title="unvalidated">*38.836 µs\**</span> <span title="validated upfront with error">*38.856 µs\**</span> | <span title="unvalidated">*78.742 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*4.9820 ns\**</span> | <span title="unvalidated">*38.843 µs\**</span> <span title="validated upfront with error">*38.846 µs\**</span> | <span title="unvalidated">*84.487 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.40%\**</span> <span title="prepend">*2.03%\**</span> | 2.16% | 69.57% | 80.42% | 78.98% | 11.56% |
| [bin-proto 0.12.3][bin-proto] | 1.97% | 1.63% | 100.00% | 96.35% | 92.05% | 99.64% |
| [bincode 2.0.1][bincode] | 7.24% | 17.14% | 100.00% | 96.35% | 92.05% | 97.82% |
| [bincode 1.3.3][bincode1] | 3.39% | 3.80% | 100.00% | 96.35% | 92.05% | 98.62% |
| [bitcode 0.6.6][bitcode] | 13.26% | 21.28% | 100.00% | 100.00% | 100.00% | 63.69% |
| [borsh 1.5.7][borsh] | 2.88% | 3.93% | 100.00% | 96.35% | 92.05% | 100.00% |
| [capnp 0.23.2][capnp] | 3.03% | † | 42.86% | 72.68% | 81.40% | 10.08% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.75% | 0.35% | 45.71% | 68.88% | 72.84% | 9.08% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.26% | 0.15% | 45.72% | 68.87% | 72.82% | 9.13% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.49% | 0.40% | 45.72% | 68.87% | 72.82% | 9.19% |
| [databuf 0.5.0][databuf] | 7.25% | 3.16% | 100.00% | 96.35% | 92.05% | 93.73% |
| [dlhn 0.1.7][dlhn] | 2.81% | 2.41% | 100.00% | 96.35% | 92.05% | 96.44% |
| [flatbuffers 25.12.19][flatbuffers] | 17.79% | † | 100.00% | 96.35% | 92.05% | 95.43% |
| [flexbuffers 25.2.10][flexbuffers] | 0.17% | 0.16% | 22.55% | 43.54% | 39.42% | 5.44% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.20% | 0.17% | 22.91% | 54.17% | 57.33% | 5.40% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.33% | 0.24% | 22.91% | 54.17% | 57.33% | 5.41% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 0.86% | 3.30% | 80.00% | 85.54% | 81.83% | 82.00% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.11% | 1.02% | 73.85% | 79.79% | 77.01% | 11.57% |
| [minicbor 1.0.0][minicbor] | 2.89% | 1.45% | 73.85% | 79.79% | 77.01% | 11.90% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.14% | 0.55% | 73.85% | 79.81% | 77.06% | 11.65% |
| [nanoserde 0.2.1][nanoserde] | 11.98% | 20.49% | 100.00% | 96.35% | 92.05% | 96.90% |
| [nibblecode 0.1.0][nibblecode] | 87.34% | † | 100.00% | 96.35% | 92.05% | 94.75% |
| [postcard 1.1.1][postcard] | 36.43% | 13.28% | 100.00% | 96.35% | 92.05% | 95.51% |
| [pot 3.0.1][pot] | 0.44% | 0.25% | 59.27% | 76.05% | 71.83% | 10.15% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*2.21%\**</span> <span title="populate + encode">*1.99%\**</span> | 1.50% | 68.57% | 77.75% | 76.64% | 11.43% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.20%\**</span> <span title="populate + encode">*0.56%\**</span> | 0.58% | 68.57% | 77.75% | 76.64% | 10.67% |
| [rkyv 0.8.10][rkyv] | 87.25% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*89.82%\**</span> | 100.00% | 96.35% | 92.05% | 92.03% |
| [ron 0.10.1][ron] | 0.11% | 0.03% | 27.04% | 57.77% | 60.48% | 5.47% |
| [savefile 0.18.6][savefile] | 100.00% | 82.66% | 100.00% | 96.35% | 92.05% | 97.01% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.39% | 4.04% | 100.00% | 96.35% | 92.05% | 91.74% |
| [serde-brief 0.1.1][serde-brief] | 0.77% | 0.49% | 38.10% | 64.58% | 72.23% | 8.75% |
| [serde_bare 0.5.0][serde_bare] | 2.91% | 3.00% | 100.00% | 96.35% | 92.05% | 91.86% |
| [speedy 0.8.7][speedy] | 87.30% | 97.32% | 100.00% | 96.35% | 92.05% | 93.90% |
| [wincode 0.2.4][wincode] | 90.26% | 87.98% | 100.00% | 96.35% | 92.05% | 98.56% |
| [wiring 0.2.4][wiring] | 91.86% | 52.65% | 100.00% | 96.34% | 92.05% | 95.71% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.08%\**</span> | <span title="validated on-demand with error">*1.75%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*2.65%\**</span> | <span title="unvalidated">*49.94%\**</span> <span title="validated upfront with error">*49.90%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*79.81%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.95%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.95%\**</span> | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*99.97%\**</span> | <span title="unvalidated">*93.20%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*890.36 µs\**</span> <span title="prepend">*787.21 µs\**</span> | 3.1539 ms | 1.7368 ms | 489348 | 281173 | 249360 | 2.7350 ms |
| [bin-proto 0.12.3][bin-proto] | 1.9281 ms | 2.8615 ms | † | 566975 | 239350 | 231475 | 2.4848 ms |
| [bincode 2.0.1][bincode] | 318.64 µs | 1.8738 ms | 861.46 µs | 367413 | 221291 | 206242 | 2.0510 ms |
| [bincode 1.3.3][bincode1] | 605.50 µs | 1.8236 ms | 862.87 µs | 569975 | 240525 | 231884 | 2.4767 ms |
| [bitcode 0.6.6][bitcode] | 127.24 µs | 1.2749 ms | 171.49 µs | 327688 | 200947 | 182040 | 784.53 µs |
| [borsh 1.5.7][borsh] | 549.79 µs | 1.8261 ms | † | 446595 | 234236 | 209834 | 2.0635 ms |
| [capnp 0.23.2][capnp] | 460.77 µs | † | † | 803896 | 335606 | 280744 | 3.5116 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 724.69 µs | 4.5508 ms | 3.3487 ms | 1109831 | 344745 | 274333 | 3.4898 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.8521 ms | 10.274 ms | † | 1109821 | 344751 | 274345 | 3.4476 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8531 ms | 4.5701 ms | 3.3313 ms | 1109821 | 344751 | 274345 | 3.4268 ms |
| [databuf 0.5.0][databuf] | 283.96 µs | 1.7149 ms | 767.99 µs | 356311 | 213062 | 198403 | 1.9687 ms |
| [dlhn 0.1.7][dlhn] | 782.71 µs | 2.6007 ms | † | 366496 | 220600 | 205586 | 2.0088 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2639 ms | † | † | 849472 | 347816 | 294871 | 3.4648 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.9735 ms | 7.5094 ms | 6.3908 ms | 1187688 | 557642 | 553730 | 6.3472 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6372 ms | 6.8207 ms | † | 1623191 | 466527 | 359157 | 5.6877 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2283 ms | 4.6271 ms | † | 1623191 | 466527 | 359157 | 5.7133 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 972.12 µs | 2.8196 ms | † | 391251 | 236877 | 220395 | 2.1519 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.5379 ms | 3.0283 ms | 1.7620 ms | 424533 | 245214 | 226077 | 2.3648 ms |
| [minicbor 1.0.0][minicbor] | 538.29 µs | 3.3274 ms | 1.8587 ms | 428773 | 249857 | 228630 | 2.4702 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0718 ms | 4.1989 ms | 3.1363 ms | 449745 | 252432 | 230965 | 2.3565 ms |
| [nanoserde 0.2.1][nanoserde] | 268.47 µs | 1.8857 ms | † | 567975 | 239930 | 231872 | 2.4663 ms |
| [nibblecode 0.1.0][nibblecode] | 182.97 µs | † | † | 603928 | 381627 | 336701 | 3.6948 ms |
| [postcard 1.1.1][postcard] | 447.04 µs | 2.0270 ms | 800.88 µs | 367489 | 221913 | 207244 | 2.0241 ms |
| [pot 3.0.1][pot] | 2.4145 ms | 6.1492 ms | 5.1047 ms | 599125 | 299158 | 247675 | 2.7402 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.3010 ms\**</span> <span title="populate + encode">*3.0173 ms\**</span> | 3.3749 ms | † | 596811 | 305319 | 268737 | 3.0060 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0376 ms\**</span> <span title="populate + encode">*3.0204 ms\**</span> | 3.8920 ms | † | 596811 | 305319 | 268737 | 2.9636 ms |
| [rkyv 0.8.10][rkyv] | 328.20 µs | <span title="unvalidated">*1.5066 ms\**</span> <span title="validated upfront with error">*1.8465 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3345 ms |
| [ron 0.10.1][ron] | 7.8096 ms | 24.612 ms | 22.545 ms | 1465223 | 434935 | 342907 | 5.5646 ms |
| [savefile 0.18.6][savefile] | 211.57 µs | 1.8396 ms | † | 566991 | 239362 | 231478 | 2.4814 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 618.27 µs | 2.0888 ms | † | 356311 | 212976 | 198423 | 1.9427 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3587 ms | 5.3586 ms | 3.7372 ms | 1276014 | 373898 | 293384 | 3.6674 ms |
| [serde_bare 0.5.0][serde_bare] | 757.00 µs | 2.3232 ms | † | 356311 | 213062 | 198403 | 1.9341 ms |
| [speedy 0.8.7][speedy] | 264.96 µs | 1.6752 ms | 571.98 µs | 449595 | 234970 | 210192 | 2.1094 ms |
| [wincode 0.2.4][wincode] | 199.06 µs | 1.6452 ms | 628.04 µs | 566975 | 239350 | 231475 | 2.4498 ms |
| [wiring 0.2.4][wiring] | 206.79 µs | 1.8549 ms | † | 566975 | 247810 | 225086 | 2.4765 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*80.994 ns\**</span> | <span title="validated on-demand with error">*427.79 ns\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4857 ns\**</span> <span title="validated upfront with error">*2.4308 ms\**</span> | <span title="unvalidated">*1.4212 µs\**</span> <span title="validated upfront with error">*2.4330 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2435 ns\**</span> <span title="validated upfront with error">*259.14 µs\**</span> | <span title="unvalidated">*156.00 ns\**</span> <span title="validated upfront with error">*260.04 µs\**</span> | <span title="unvalidated">*745.04 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2434 ns\**</span> <span title="validated upfront with error">*323.05 µs\**</span> | <span title="unvalidated">*156.04 ns\**</span> <span title="validated upfront with error">*322.02 µs\**</span> | <span title="unvalidated">*732.77 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.29%\**</span> <span title="prepend">*16.16%\**</span> | 40.42% | 9.87% | 66.96% | 71.47% | 73.00% | 28.68% |
| [bin-proto 0.12.3][bin-proto] | 6.60% | 44.55% | † | 57.80% | 83.96% | 78.64% | 31.57% |
| [bincode 2.0.1][bincode] | 39.93% | 68.04% | 19.91% | 89.19% | 90.81% | 88.27% | 38.25% |
| [bincode 1.3.3][bincode1] | 21.01% | 69.91% | 19.87% | 57.49% | 83.55% | 78.50% | 31.68% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 23.14% | 69.82% | † | 73.37% | 85.79% | 86.75% | 38.02% |
| [capnp 0.23.2][capnp] | 27.61% | † | † | 40.76% | 59.88% | 64.84% | 22.34% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 17.56% | 28.01% | 5.12% | 29.53% | 58.29% | 66.36% | 22.48% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.30% | 12.41% | † | 29.53% | 58.29% | 66.35% | 22.76% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.87% | 27.90% | 5.15% | 29.53% | 58.29% | 66.35% | 22.89% |
| [databuf 0.5.0][databuf] | 44.81% | 74.34% | 22.33% | 91.97% | 94.31% | 91.75% | 39.85% |
| [dlhn 0.1.7][dlhn] | 16.26% | 49.02% | † | 89.41% | 91.09% | 88.55% | 39.05% |
| [flatbuffers 25.12.19][flatbuffers] | 3.90% | † | † | 38.58% | 57.77% | 61.74% | 22.64% |
| [flexbuffers 25.2.10][flexbuffers] | 1.60% | 16.98% | 2.68% | 27.59% | 36.04% | 32.88% | 12.36% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.50% | 18.69% | † | 20.19% | 43.07% | 50.69% | 13.79% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.71% | 27.55% | † | 20.19% | 43.07% | 50.69% | 13.73% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 13.09% | 45.22% | † | 83.75% | 84.83% | 82.60% | 36.46% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.27% | 42.10% | 9.73% | 77.19% | 81.95% | 80.52% | 33.18% |
| [minicbor 1.0.0][minicbor] | 23.64% | 38.32% | 9.23% | 76.42% | 80.42% | 79.62% | 31.76% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.51% | 30.36% | 5.47% | 72.86% | 79.60% | 78.82% | 33.29% |
| [nanoserde 0.2.1][nanoserde] | 47.39% | 67.61% | † | 57.69% | 83.75% | 78.51% | 31.81% |
| [nibblecode 0.1.0][nibblecode] | 69.54% | † | † | 54.26% | 52.66% | 54.07% | 21.23% |
| [postcard 1.1.1][postcard] | 28.46% | 62.90% | 21.41% | 89.17% | 90.55% | 87.84% | 38.76% |
| [pot 3.0.1][pot] | 5.27% | 20.73% | 3.36% | 54.69% | 67.17% | 73.50% | 28.63% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.78%\**</span> <span title="populate + encode">*4.22%\**</span> | 37.78% | † | 54.91% | 65.82% | 67.74% | 26.10% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.26%\**</span> <span title="populate + encode">*4.21%\**</span> | 32.76% | † | 54.91% | 65.82% | 67.74% | 26.47% |
| [rkyv 0.8.10][rkyv] | 38.77% | <span title="unvalidated">*84.62%\**</span> <span title="validated upfront with error">*69.04%\**</span> | † | 54.27% | 78.87% | 82.96% | 33.61% |
| [ron 0.10.1][ron] | 1.63% | 5.18% | 0.76% | 22.36% | 46.20% | 53.09% | 14.10% |
| [savefile 0.18.6][savefile] | 60.14% | 69.30% | † | 57.79% | 83.95% | 78.64% | 31.62% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.58% | 61.04% | † | 91.97% | 94.35% | 91.74% | 40.38% |
| [serde-brief 0.1.1][serde-brief] | 9.36% | 23.79% | 4.59% | 25.68% | 53.74% | 62.05% | 21.39% |
| [serde_bare 0.5.0][serde_bare] | 16.81% | 54.88% | † | 91.97% | 94.31% | 91.75% | 40.56% |
| [speedy 0.8.7][speedy] | 48.02% | 76.10% | 29.98% | 72.89% | 85.52% | 86.61% | 37.19% |
| [wincode 0.2.4][wincode] | 63.92% | 77.49% | 27.31% | 57.80% | 83.96% | 78.64% | 32.02% |
| [wiring 0.2.4][wiring] | 61.53% | 68.73% | † | 57.80% | 81.09% | 80.88% | 31.68% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.54%\**</span> | <span title="validated on-demand with error">*36.47%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.98%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*98.35%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5491 ms\**</span> <span title="prepend">*2.6557 ms\**</span> | 8.7362 ms | 1704643 | 1294259 | 1245668 | 11.663 ms |
| [bin-proto 0.12.3][bin-proto] | 5.9609 ms | 6.8890 ms | 1791489 | 1127998 | 1051146 | 10.488 ms |
| [bincode 2.0.1][bincode] | 1.1960 ms | 3.8151 ms | 1406257 | 1117802 | 1062438 | 9.3621 ms |
| [bincode 1.3.3][bincode1] | 3.9742 ms | 4.1445 ms | 1854234 | 1141994 | 1048745 | 10.482 ms |
| [bitcode 0.6.6][bitcode] | 730.75 µs | 2.3367 ms | 971318 | 878034 | 850340 | 2.9895 ms |
| [borsh 1.5.7][borsh] | 2.8780 ms | 2.8733 ms | 1521989 | 1108471 | 1038528 | 10.104 ms |
| [capnp 0.23.2][capnp] | 2.1999 ms | † | 2724288 | 1546992 | 1239111 | 14.551 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.0132 ms | 18.070 ms | 6012539 | 1695215 | 1464951 | 21.374 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 24.115 ms | 52.854 ms | 6012373 | 1695146 | 1465025 | 21.527 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.9547 ms | 20.687 ms | 6012373 | 1695146 | 1465025 | 21.206 ms |
| [databuf 0.5.0][databuf] | 1.3220 ms | 3.7131 ms | 1319999 | 1062631 | 1008334 | 8.8519 ms |
| [dlhn 0.1.7][dlhn] | 5.0931 ms | 7.7895 ms | 1311281 | 1077520 | 1046095 | 8.5752 ms |
| [flatbuffers 25.12.19][flatbuffers] | 5.2955 ms | † | 2325620 | 1439185 | 1268060 | 13.498 ms |
| [flexbuffers 25.2.10][flexbuffers] | 42.347 ms | 38.111 ms | 5352680 | 2658295 | 2777967 | 33.880 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.632 ms | 31.027 ms | 9390461 | 2391679 | 1842767 | 34.291 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.960 ms | 26.468 ms | 9390461 | 2391679 | 1842767 | 34.728 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.2505 ms | 5.9036 ms | 1458773 | 1156055 | 1137788 | 10.003 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.159 ms | 10.914 ms | 1745322 | 1261627 | 1228923 | 11.372 ms |
| [minicbor 1.0.0][minicbor] | 2.1891 ms | 11.146 ms | 1777386 | 1276218 | 1252558 | 12.607 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.943 ms | 20.108 ms | 1770060 | 1277755 | 1263362 | 12.753 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2658 ms | 2.7927 ms | 1812404 | 1134820 | 1053109 | 10.261 ms |
| [nibblecode 0.1.0][nibblecode] | 506.27 µs | † | 2075936 | 1518506 | 1413251 | 14.132 ms |
| [postcard 1.1.1][postcard] | 1.8113 ms | 4.2150 ms | 1311281 | 1083900 | 1041434 | 8.8463 ms |
| [pot 3.0.1][pot] | 14.047 ms | 29.846 ms | 2604812 | 1482233 | 1298928 | 16.154 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.4825 ms\**</span> <span title="populate + encode">*9.3695 ms\**</span> | 8.8269 ms | 1859886 | 1338076 | 1295351 | 12.439 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.4935 ms\**</span> <span title="populate + encode">*12.685 ms\**</span> | 12.019 ms | 1859886 | 1338076 | 1295351 | 12.487 ms |
| [rkyv 0.8.10][rkyv] | 988.80 µs | <span title="unvalidated">*2.1570 ms\**</span> <span title="validated upfront with error">*2.6172 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.034 ms |
| [ron 0.10.1][ron] | 41.498 ms | 149.69 ms | 8677703 | 2233642 | 1826180 | 34.745 ms |
| [savefile 0.18.6][savefile] | 886.13 µs | 2.7554 ms | 1791505 | 1128012 | 1051153 | 10.277 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1388 ms | 3.3295 ms | 1319999 | 1064380 | 1010708 | 8.9045 ms |
| [serde-brief 0.1.1][serde-brief] | 6.5414 ms | 22.357 ms | 6951772 | 1796265 | 1567819 | 23.694 ms |
| [serde_bare 0.5.0][serde_bare] | 4.8401 ms | 4.8297 ms | 1319999 | 1062645 | 1008349 | 8.7556 ms |
| [speedy 0.8.7][speedy] | 744.61 µs | 2.4621 ms | 1584734 | 1119837 | 1037992 | 10.116 ms |
| [wincode 0.2.4][wincode] | 546.66 µs | 2.3498 ms | 1791489 | 1127998 | 1051146 | 10.246 ms |
| [wiring 0.2.4][wiring] | 645.21 µs | 2.8011 ms | 1791489 | 1156963 | 1082815 | 10.632 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*81.175 ns\**</span> | <span title="validated on-demand with error">*721.89 ns\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4866 ns\**</span> <span title="validated upfront with error">*5.5965 ms\**</span> | <span title="unvalidated">*2.7354 µs\**</span> <span title="validated upfront with error">*5.6022 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*366.82 µs\**</span> | <span title="unvalidated">*375.68 ns\**</span> <span title="validated upfront with error">*368.32 µs\**</span> | <span title="unvalidated">*237.25 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2428 ns\**</span> <span title="validated upfront with error">*428.58 µs\**</span> | <span title="unvalidated">*384.80 ns\**</span> <span title="validated upfront with error">*429.40 µs\**</span> | <span title="unvalidated">*238.06 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.13%\**</span> <span title="prepend">*19.06%\**</span> | 24.69% | 56.98% | 67.84% | 68.26% | 25.63% |
| [bin-proto 0.12.3][bin-proto] | 8.49% | 31.31% | 54.22% | 77.84% | 80.90% | 28.50% |
| [bincode 2.0.1][bincode] | 42.33% | 56.54% | 69.07% | 78.55% | 80.04% | 31.93% |
| [bincode 1.3.3][bincode1] | 12.74% | 52.04% | 52.38% | 76.89% | 81.08% | 28.52% |
| [bitcode 0.6.6][bitcode] | 69.28% | 92.31% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 17.59% | 75.07% | 63.82% | 79.21% | 81.88% | 29.59% |
| [capnp 0.23.2][capnp] | 23.01% | † | 35.65% | 56.76% | 68.63% | 20.54% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.80% | 11.94% | 16.15% | 51.79% | 58.05% | 13.99% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.10% | 4.08% | 16.16% | 51.80% | 58.04% | 13.89% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.09% | 10.43% | 16.16% | 51.80% | 58.04% | 14.10% |
| [databuf 0.5.0][databuf] | 38.30% | 58.09% | 73.58% | 82.63% | 84.33% | 33.77% |
| [dlhn 0.1.7][dlhn] | 9.94% | 27.69% | 74.07% | 81.49% | 81.29% | 34.86% |
| [flatbuffers 25.12.19][flatbuffers] | 9.56% | † | 41.77% | 61.01% | 67.06% | 22.15% |
| [flexbuffers 25.2.10][flexbuffers] | 1.20% | 5.66% | 18.15% | 33.03% | 30.61% | 8.82% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.45% | 6.95% | 10.34% | 36.71% | 46.14% | 8.72% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.23% | 8.15% | 10.34% | 36.71% | 46.14% | 8.61% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 22.50% | 36.54% | 66.58% | 75.95% | 74.74% | 29.89% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 4.98% | 19.76% | 55.65% | 69.60% | 69.19% | 26.29% |
| [minicbor 1.0.0][minicbor] | 23.13% | 19.35% | 54.65% | 68.80% | 67.89% | 23.71% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.69% | 10.73% | 54.87% | 68.72% | 67.31% | 23.44% |
| [nanoserde 0.2.1][nanoserde] | 40.00% | 77.24% | 53.59% | 77.37% | 80.75% | 29.13% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 57.82% | 60.17% | 21.15% |
| [postcard 1.1.1][postcard] | 27.95% | 51.17% | 74.07% | 81.01% | 81.65% | 33.79% |
| [pot 3.0.1][pot] | 3.60% | 7.23% | 37.29% | 59.24% | 65.46% | 18.51% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.23%\**</span> <span title="populate + encode">*5.40%\**</span> | 24.44% | 52.22% | 65.62% | 65.65% | 24.03% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*9.22%\**</span> <span title="populate + encode">*3.99%\**</span> | 17.95% | 52.22% | 65.62% | 65.65% | 23.94% |
| [rkyv 0.8.10][rkyv] | 51.20% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*82.42%\**</span> | 46.79% | 63.45% | 70.25% | 22.94% |
| [ron 0.10.1][ron] | 1.22% | 1.44% | 11.19% | 39.31% | 46.56% | 8.60% |
| [savefile 0.18.6][savefile] | 57.13% | 78.28% | 54.22% | 77.84% | 80.90% | 29.09% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.13% | 64.78% | 73.58% | 82.49% | 84.13% | 33.57% |
| [serde-brief 0.1.1][serde-brief] | 7.74% | 9.65% | 13.97% | 48.88% | 54.24% | 12.62% |
| [serde_bare 0.5.0][serde_bare] | 10.46% | 44.66% | 73.58% | 82.63% | 84.33% | 34.14% |
| [speedy 0.8.7][speedy] | 67.99% | 87.61% | 61.29% | 78.41% | 81.92% | 29.55% |
| [wincode 0.2.4][wincode] | 92.61% | 91.80% | 54.22% | 77.84% | 80.90% | 29.18% |
| [wiring 0.2.4][wiring] | 78.47% | 77.01% | 54.22% | 75.89% | 78.53% | 28.12% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.53%\**</span> | <span title="validated on-demand with error">*52.04%\**</span> | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.73%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.63%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*99.66%\**</span> |

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
