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

## Last updated: 2026-06-30 23:56:04

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.98.0-nightly (096694416 2026-06-29)
binary: rustc
commit-hash: 096694416a41840709140eb0fd0ca193d1a3e6ba
commit-date: 2026-06-29
host: x86_64-unknown-linux-gnu
release: 1.98.0-nightly
LLVM version: 22.1.8
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
| [bilrost 0.1015.0][bilrost] | <span title="encode">*425.60 µs\**</span> <span title="prepend">*421.53 µs\**</span> | 2.6305 ms | 824.42 µs | 804955 | 328941 | 284849 | 4.2433 ms |
| [bin-proto 0.12.8][bin-proto] | 4.3354 ms | 4.6134 ms | † | 1045784 | 373127 | 311553 | 4.5531 ms |
| [bincode 2.0.1][bincode] | 366.99 µs | 2.1880 ms | 705.62 µs | 741295 | 303944 | 256422 | 3.6572 ms |
| [bincode 1.3.3][bincode1] | 525.37 µs | 2.0340 ms | 597.51 µs | 1045784 | 373127 | 311553 | 4.4869 ms |
| [bitcode 0.6.9][bitcode] | 139.38 µs | 1.4552 ms | 60.352 µs | 703710 | 288826 | 227322 | 2.5910 ms |
| [borsh 1.7.0][borsh] | 545.96 µs | 2.3590 ms | † | 885780 | 362204 | 286248 | 3.9283 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 600.02 µs <span title="packed">*1.5038 ms\**</span> | † | † | 1443216 <span title="packed">*1046865\**</span> | 513986 <span title="packed">*481681\**</span> | 426532 <span title="packed">*458024\**</span> | 6.2675 ms <span title="packed">*5.3795 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 615.54 µs | 5.0146 ms | 3.4506 ms | 1407835 | 403440 | 323561 | 5.0626 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.9778 ms | 11.064 ms | † | 1407835 | 403440 | 323561 | 5.0151 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 2.0027 ms | 4.5084 ms | 2.9334 ms | 1407835 | 403440 | 323561 | 4.7413 ms |
| [columnar 0.13.1][columnar] | 248.25 µs | 2.0964 ms <span title="copy_from">*727.09 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.2901 ms |
| [compactly 0.1.7][compactly] | 24.490 ms | 16.153 ms | † | 239520 | 239720 | 239532 | 162.85 µs |
| [databuf 0.5.0][databuf] | 260.49 µs | 2.0275 ms | 675.37 µs | 765778 | 311715 | 263914 | 3.5535 ms |
| [dlhn 0.1.7][dlhn] | 678.54 µs | 2.6030 ms | † | 724953 | 301446 | 253056 | 3.2934 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0422 ms | † | † | 1276368 | 468539 | 388381 | 4.8370 ms |
| [flexbuffers 25.12.19][flexbuffers] | 6.6317 ms | 7.6457 ms | 5.3553 ms | 1829756 | 714318 | 691541 | 8.5944 ms |
| json:<br> [flexon 0.4.6][flexon] | 2.7808 ms | 3.8150 ms | † | 1827461 | 470560 | 360727 | 5.4848 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 3.7975 ms | 6.0755 ms | † | 1827461 | 470560 | 360727 | 5.5540 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 2.1312 ms | 4.7409 ms | † | 1827461 | 470560 | 360727 | 5.5572 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 365.53 µs | 2.6190 ms | 916.35 µs | 764996 | 315291 | 264212 | 3.5882 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 1.5393 ms | 3.1531 ms | 1.4917 ms | 784997 | 325384 | 277608 | 3.7567 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 355.78 µs | 2.2727 ms | 814.46 µs | 784997 | 325384 | 277608 | 3.7978 ms |
| [minicbor 2.2.2][minicbor] | 503.45 µs | 2.8860 ms | 1.4040 ms | 817830 | 332671 | 284034 | 3.9920 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3837 ms | 4.0053 ms | 2.5509 ms | 818669 | 332556 | 284797 | 4.0250 ms |
| [nanoserde 0.2.1][nanoserde] | 254.63 µs | 2.0917 ms | † | 1045784 | 373127 | 311553 | 4.3041 ms |
| [nibblecode 0.1.0][nibblecode] | 193.23 µs | † | † | 1011487 | 473999 | 404669 | 5.3737 ms |
| [postcard 1.1.3][postcard] | 423.36 µs | 2.3217 ms | 632.52 µs | 724953 | 302399 | 252968 | 3.3434 ms |
| [pot 3.0.1][pot] | 2.3453 ms | 6.0920 ms | 4.6101 ms | 971922 | 372513 | 303636 | 4.4027 ms |
| protobuf:<br> [buffa 0.8.0][buffa] | <span title="encode">*766.68 µs\**</span> <span title="populate + encode">*2.5340 ms\**</span> | 3.3227 ms | 2.1691 ms | 884628 | 363130 | 314959 | 4.4146 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*949.73 µs\**</span> <span title="populate + encode">*2.4769 ms\**</span> | 3.4334 ms | † | 884628 | 363130 | 314959 | 4.5032 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2359 ms\**</span> <span title="populate + encode">*3.0721 ms\**</span> | 3.8614 ms | † | 884628 | 363130 | 314959 | 4.4463 ms |
| [rkyv 0.8.16][rkyv] | 244.41 µs | <span title="unvalidated">*1.5443 ms\**</span> <span title="validated upfront with error">*1.9151 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6545 ms |
| [ron 0.12.2][ron] | 12.233 ms | 26.373 ms | 23.926 ms | 1607459 | 449158 | 349324 | 5.5966 ms |
| [savefile 0.20.4][savefile] | 197.06 µs | 2.2850 ms | † | 1045800 | 373139 | 311562 | 4.3178 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 658.52 µs | 2.3291 ms | † | 765778 | 311743 | 263822 | 3.5008 ms |
| [serde-brief 0.2.0][serde-brief] | 1.3844 ms | 4.6120 ms | 2.8286 ms | 1584946 | 413733 | 339964 | 4.9151 ms |
| [serde_bare 0.5.0][serde_bare] | 704.40 µs | 2.0792 ms | † | 765778 | 311715 | 263914 | 3.5429 ms |
| [speedy 0.8.7][speedy] | 195.99 µs | 1.7595 ms | 372.05 µs | 885780 | 362204 | 286248 | 3.8763 ms |
| [wincode 0.5.5][wincode] | 172.17 µs | 1.7615 ms | 365.56 µs | 1045784 | 373127 | 311553 | 4.2164 ms |
| [wiring 0.2.4][wiring] | 193.18 µs | 2.0679 ms | † | 1045784 | 337930 | 275808 | 3.7103 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*76.039 ns\**</span> | <span title="packed">*1.0340 ms\**</span> <span title="validated on-demand with error">*144.65 µs\**</span> | ‡ |
| [columnar 0.13.1][columnar] | 20.300 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4901 ns\**</span> <span title="validated upfront with error">*2.1006 ms\**</span> | <span title="unvalidated">*48.653 µs\**</span> <span title="validated upfront with error">*2.1453 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2445 ns\**</span> <span title="validated upfront with error">*236.81 µs\**</span> | <span title="unvalidated">*10.423 µs\**</span> <span title="validated upfront with error">*246.12 µs\**</span> | <span title="unvalidated">*7.3880 µs\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2446 ns\**</span> <span title="validated upfront with error">*365.92 µs\**</span> | <span title="unvalidated">*10.381 µs\**</span> <span title="validated upfront with error">*371.92 µs\**</span> | <span title="unvalidated">*7.6182 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1015.0][bilrost] | <span title="encode">*32.75%\**</span> <span title="prepend">*33.07%\**</span> | 27.64% | 7.32% | 29.76% | 72.88% | 79.80% | 3.84% |
| [bin-proto 0.12.8][bin-proto] | 3.21% | 15.76% | † | 22.90% | 64.25% | 72.96% | 3.58% |
| [bincode 2.0.1][bincode] | 37.98% | 33.23% | 8.55% | 32.31% | 78.87% | 88.65% | 4.45% |
| [bincode 1.3.3][bincode1] | 26.53% | 35.75% | 10.10% | 22.90% | 64.25% | 72.96% | 3.63% |
| [bitcode 0.6.9][bitcode] | 100.00% | 49.96% | 100.00% | 34.04% | 83.00% | 100.00% | 6.29% |
| [borsh 1.7.0][borsh] | 25.53% | 30.82% | † | 27.04% | 66.18% | 79.41% | 4.15% |
| capnp:<br> [capnp 0.26.0][capnp] | 23.23% <span title="packed">*9.27%\**</span> | † | † | 16.60% <span title="packed">*22.88%\**</span> | 46.64% <span title="packed">*49.77%\**</span> | 53.30% <span title="packed">*49.63%\**</span> | 2.60% <span title="packed">*3.03%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 22.64% | 14.50% | 1.75% | 17.01% | 59.42% | 70.26% | 3.22% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.50% | 6.57% | † | 17.01% | 59.42% | 70.26% | 3.25% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.96% | 16.13% | 2.06% | 17.01% | 59.42% | 70.26% | 3.43% |
| [columnar 0.13.1][columnar] | 56.15% | 34.68% <span title="copy_from">*100.00%\**</span> | † | 22.90% | 64.75% | 77.34% | 3.80% |
| [compactly 0.1.7][compactly] | 0.57% | 4.50% | † | 100.00% | 100.00% | 94.90% | 100.00% |
| [databuf 0.5.0][databuf] | 53.51% | 35.86% | 8.94% | 31.28% | 76.90% | 86.13% | 4.58% |
| [dlhn 0.1.7][dlhn] | 20.54% | 27.93% | † | 33.04% | 79.52% | 89.83% | 4.94% |
| [flatbuffers 25.12.19][flatbuffers] | 13.37% | † | † | 18.77% | 51.16% | 58.53% | 3.37% |
| [flexbuffers 25.12.19][flexbuffers] | 2.10% | 9.51% | 1.13% | 13.09% | 33.56% | 32.87% | 1.89% |
| json:<br> [flexon 0.4.6][flexon] | 5.01% | 19.06% | † | 13.11% | 50.94% | 63.02% | 2.97% |
| json:<br> [serde_json 1.0.150][serde_json] | 3.67% | 11.97% | † | 13.11% | 50.94% | 63.02% | 2.93% |
| json:<br> [simd-json 0.17.0][simd-json] | 6.54% | 15.34% | † | 13.11% | 50.94% | 63.02% | 2.93% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 38.13% | 27.76% | 6.59% | 31.31% | 76.03% | 86.04% | 4.54% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 9.05% | 23.06% | 4.05% | 30.51% | 73.67% | 81.89% | 4.34% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 39.18% | 31.99% | 7.41% | 30.51% | 73.67% | 81.89% | 4.29% |
| [minicbor 2.2.2][minicbor] | 27.68% | 25.19% | 4.30% | 29.29% | 72.06% | 80.03% | 4.08% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.59% | 18.15% | 2.37% | 29.26% | 72.08% | 79.82% | 4.05% |
| [nanoserde 0.2.1][nanoserde] | 54.74% | 34.76% | † | 22.90% | 64.25% | 72.96% | 3.78% |
| [nibblecode 0.1.0][nibblecode] | 72.13% | † | † | 23.68% | 50.57% | 56.17% | 3.03% |
| [postcard 1.1.3][postcard] | 32.92% | 31.32% | 9.54% | 33.04% | 79.27% | 89.86% | 4.87% |
| [pot 3.0.1][pot] | 5.94% | 11.94% | 1.31% | 24.64% | 64.35% | 74.87% | 3.70% |
| protobuf:<br> [buffa 0.8.0][buffa] | <span title="encode">*18.18%\**</span> <span title="populate + encode">*5.50%\**</span> | 21.88% | 2.78% | 27.08% | 66.01% | 72.18% | 3.69% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*14.68%\**</span> <span title="populate + encode">*5.63%\**</span> | 21.18% | † | 27.08% | 66.01% | 72.18% | 3.62% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.28%\**</span> <span title="populate + encode">*4.54%\**</span> | 18.83% | † | 27.08% | 66.01% | 72.18% | 3.66% |
| [rkyv 0.8.16][rkyv] | 57.03% | <span title="unvalidated">*47.08%\**</span> <span title="validated upfront with error">*37.97%\**</span> | † | 23.68% | 60.92% | 69.74% | 3.50% |
| [ron 0.12.2][ron] | 1.14% | 2.76% | 0.25% | 14.90% | 53.37% | 65.07% | 2.91% |
| [savefile 0.20.4][savefile] | 70.73% | 31.82% | † | 22.90% | 64.24% | 72.96% | 3.77% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 21.17% | 31.22% | † | 31.28% | 76.90% | 86.16% | 4.65% |
| [serde-brief 0.2.0][serde-brief] | 10.07% | 15.77% | 2.13% | 15.11% | 57.94% | 66.87% | 3.31% |
| [serde_bare 0.5.0][serde_bare] | 19.79% | 34.97% | † | 31.28% | 76.90% | 86.13% | 4.60% |
| [speedy 0.8.7][speedy] | 71.12% | 41.32% | 16.22% | 27.04% | 66.18% | 79.41% | 4.20% |
| [wincode 0.5.5][wincode] | 80.95% | 41.28% | 16.51% | 22.90% | 64.25% | 72.96% | 3.86% |
| [wiring 0.2.4][wiring] | 72.15% | 35.16% | † | 22.90% | 70.94% | 82.42% | 4.39% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.64%\**</span> | <span title="packed">*1.00%\**</span> <span title="validated on-demand with error">*7.18%\**</span> | ‡ |
| [columnar 0.13.1][columnar] | 6.13% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*21.34%\**</span> <span title="validated upfront with error">*0.48%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.60%\**</span> <span title="validated upfront with error">*4.22%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.79%\**</span> | <span title="unvalidated">*96.98%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1015.0][bilrost] | <span title="encode">*7.2132 ms\**</span> <span title="prepend">*8.6258 ms\**</span> | 7.7042 ms | 8625005 | 6443961 | 6231572 | 72.021 ms |
| [bin-proto 0.12.8][bin-proto] | 8.3250 ms | 10.396 ms | 6000008 | 5378500 | 5346908 | 8.8482 ms |
| [bincode 2.0.1][bincode] | 2.9056 ms | 788.35 µs | 6000005 | 5378497 | 5346882 | 8.9613 ms |
| [bincode 1.3.3][bincode1] | 5.6362 ms | 4.7897 ms | 6000008 | 5378500 | 5346908 | 9.0316 ms |
| [bitcode 0.6.9][bitcode] | 1.3073 ms | 805.06 µs | 6000006 | 5182295 | 4921841 | 13.475 ms |
| [borsh 1.7.0][borsh] | 6.1528 ms | 4.2085 ms | 6000004 | 5378496 | 5346866 | 8.8632 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 9.7229 ms <span title="packed">*19.463 ms\**</span> | † | 14000088 <span title="packed">*10401737\**</span> | 7130367 <span title="packed">*7308001\**</span> | 6046182 <span title="packed">*7922110\**</span> | 83.914 ms <span title="packed">*66.848 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 8.9125 ms | 44.556 ms | 13125016 | 7524114 | 6757437 | 92.826 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 67.417 ms | 111.02 ms | 13122324 | 7524660 | 6759128 | 93.519 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 35.989 ms | 40.593 ms | 13122324 | 7524660 | 6759128 | 90.851 ms |
| [columnar 0.13.1][columnar] | 1.7365 ms | 1.4426 ms <span title="copy_from">*669.39 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.7771 ms |
| [compactly 0.1.7][compactly] | 355.72 ms | 161.94 ms | 4846788 | 4850067 | 4846905 | 1.6500 ms |
| [databuf 0.5.0][databuf] | 2.4129 ms | 5.3953 ms | 6000003 | 5378495 | 5346897 | 9.0490 ms |
| [dlhn 0.1.7][dlhn] | 6.0225 ms | 7.0221 ms | 6000003 | 5378495 | 5346897 | 8.9640 ms |
| [flatbuffers 25.12.19][flatbuffers] | 449.90 µs | † | 6000024 | 5378434 | 5346878 | 8.7064 ms |
| [flexbuffers 25.12.19][flexbuffers] | 103.96 ms | 76.385 ms | 26609424 | 11901040 | 12486322 | 149.76 ms |
| json:<br> [flexon 0.4.6][flexon] | 76.391 ms | 55.565 ms | 26192883 | 9566084 | 8584671 | 154.89 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 92.640 ms | 100.26 ms | 26192883 | 9566084 | 8584671 | 155.45 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 53.117 ms | 66.906 ms | 26192883 | 9566084 | 8584671 | 155.47 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 670.78 µs | 5.2685 ms | 7500005 | 6058442 | 6014500 | 10.487 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 19.829 ms | 16.491 ms | 8125006 | 6494876 | 6391037 | 73.418 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 735.22 µs | 5.2009 ms | 8125006 | 6494876 | 6391037 | 75.933 ms |
| [minicbor 2.2.2][minicbor] | 5.1932 ms | 11.660 ms | 8125006 | 6494907 | 6390894 | 69.653 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.82 ms | 25.868 ms | 8125037 | 6493484 | 6386940 | 73.136 ms |
| [nanoserde 0.2.1][nanoserde] | 1.6160 ms | 892.93 µs | 6000008 | 5378500 | 5346908 | 8.9869 ms |
| [nibblecode 0.1.0][nibblecode] | 148.53 µs | † | 6000008 | 5378500 | 5346908 | 8.8317 ms |
| [postcard 1.1.3][postcard] | 494.25 µs | 951.65 µs | 6000003 | 5378495 | 5346897 | 8.6873 ms |
| [pot 3.0.1][pot] | 38.178 ms | 64.267 ms | 10122342 | 6814618 | 6852252 | 81.452 ms |
| protobuf:<br> [buffa 0.8.0][buffa] | <span title="encode">*8.7110 ms\**</span> <span title="populate + encode">*25.222 ms\**</span> | 26.097 ms | 8750000 | 6665735 | 6421877 | 72.488 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*8.0763 ms\**</span> <span title="populate + encode">*8.6600 ms\**</span> | 14.861 ms | 8750000 | 6665735 | 6421877 | 76.246 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*16.561 ms\**</span> <span title="populate + encode">*32.932 ms\**</span> | 30.328 ms | 8750000 | 6665735 | 6421877 | 78.490 ms |
| [rkyv 0.8.16][rkyv] | 188.69 µs | <span title="unvalidated">*151.81 µs\**</span> <span title="validated upfront with error">*151.78 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.9102 ms |
| [ron 0.12.2][ron] | 181.30 ms | 576.73 ms | 22192885 | 8970395 | 8137334 | 148.34 ms |
| [savefile 0.20.4][savefile] | 148.26 µs | 148.54 µs | 6000024 | 5378519 | 5346896 | 8.7455 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1669 ms | 4.6893 ms | 6000004 | 5378496 | 5346866 | 8.8210 ms |
| [serde-brief 0.2.0][serde-brief] | 17.988 ms | 35.999 ms | 15750015 | 8024540 | 6813667 | 95.457 ms |
| [serde_bare 0.5.0][serde_bare] | 5.9093 ms | 4.8860 ms | 6000003 | 5378495 | 5346897 | 9.0753 ms |
| [speedy 0.8.7][speedy] | 148.22 µs | 149.66 µs | 6000004 | 5378496 | 5346866 | 8.8796 ms |
| [wincode 0.5.5][wincode] | 149.50 µs | 149.05 µs | 6000008 | 5378500 | 5346908 | 8.8608 ms |
| [wiring 0.2.4][wiring] | 149.01 µs | 335.88 µs | 6000008 | 5378952 | 5346905 | 8.9195 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*106.41 ns\**</span> | <span title="packed">*13.390 ms\**</span> <span title="validated on-demand with error">*2.1422 ms\**</span> | ‡ |
| [columnar 0.13.1][columnar] | 20.984 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4903 ns\**</span> <span title="validated upfront with error">*44.272 ns\**</span> | <span title="unvalidated">*77.838 µs\**</span> <span title="validated upfront with error">*77.878 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2449 ns\**</span> <span title="validated upfront with error">*1.5568 ns\**</span> | <span title="unvalidated">*38.881 µs\**</span> <span title="validated upfront with error">*38.895 µs\**</span> | <span title="unvalidated">*78.668 µs\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2452 ns\**</span> <span title="validated upfront with error">*5.6107 ns\**</span> | <span title="unvalidated">*38.897 µs\**</span> <span title="validated upfront with error">*38.921 µs\**</span> | <span title="unvalidated">*75.419 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1015.0][bilrost] | <span title="encode">*2.05%\**</span> <span title="prepend">*1.72%\**</span> | 1.93% | 56.19% | 75.27% | 77.78% | 2.29% |
| [bin-proto 0.12.8][bin-proto] | 1.78% | 1.43% | 80.78% | 90.18% | 90.65% | 18.65% |
| [bincode 2.0.1][bincode] | 5.10% | 18.84% | 80.78% | 90.18% | 90.65% | 18.41% |
| [bincode 1.3.3][bincode1] | 2.63% | 3.10% | 80.78% | 90.18% | 90.65% | 18.27% |
| [bitcode 0.6.9][bitcode] | 11.34% | 18.45% | 80.78% | 93.59% | 98.48% | 12.25% |
| [borsh 1.7.0][borsh] | 2.41% | 3.53% | 80.78% | 90.18% | 90.65% | 18.62% |
| capnp:<br> [capnp 0.26.0][capnp] | 1.52% <span title="packed">*0.76%\**</span> | † | 34.62% <span title="packed">*46.60%\**</span> | 68.02% <span title="packed">*66.37%\**</span> | 80.16% <span title="packed">*61.18%\**</span> | 1.97% <span title="packed">*2.47%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 1.66% | 0.33% | 36.93% | 64.46% | 71.73% | 1.78% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.22% | 0.13% | 36.94% | 64.46% | 71.71% | 1.76% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.41% | 0.37% | 36.94% | 64.46% | 71.71% | 1.82% |
| [columnar 0.13.1][columnar] | 8.54% | 10.30% <span title="copy_from">*22.19%\**</span> | 80.78% | 90.18% | 90.65% | 18.80% |
| [compactly 0.1.7][compactly] | 0.04% | 0.09% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 6.14% | 2.75% | 80.78% | 90.18% | 90.65% | 18.23% |
| [dlhn 0.1.7][dlhn] | 2.46% | 2.12% | 80.78% | 90.18% | 90.65% | 18.41% |
| [flatbuffers 25.12.19][flatbuffers] | 32.95% | † | 80.78% | 90.18% | 90.65% | 18.95% |
| [flexbuffers 25.12.19][flexbuffers] | 0.14% | 0.19% | 18.21% | 40.75% | 38.82% | 1.10% |
| json:<br> [flexon 0.4.6][flexon] | 0.19% | 0.27% | 18.50% | 50.70% | 56.46% | 1.07% |
| json:<br> [serde_json 1.0.150][serde_json] | 0.16% | 0.15% | 18.50% | 50.70% | 56.46% | 1.06% |
| json:<br> [simd-json 0.17.0][simd-json] | 0.28% | 0.22% | 18.50% | 50.70% | 56.46% | 1.06% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 22.10% | 2.82% | 64.62% | 80.05% | 80.59% | 15.73% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 0.75% | 0.90% | 59.65% | 74.68% | 75.84% | 2.25% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 20.16% | 2.86% | 59.65% | 74.68% | 75.84% | 2.17% |
| [minicbor 2.2.2][minicbor] | 2.85% | 1.27% | 59.65% | 74.67% | 75.84% | 2.37% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.57% | 59.65% | 74.69% | 75.89% | 2.26% |
| [nanoserde 0.2.1][nanoserde] | 9.17% | 16.64% | 80.78% | 90.18% | 90.65% | 18.36% |
| [nibblecode 0.1.0][nibblecode] | 99.79% | † | 80.78% | 90.18% | 90.65% | 18.68% |
| [postcard 1.1.3][postcard] | 29.99% | 15.61% | 80.78% | 90.18% | 90.65% | 18.99% |
| [pot 3.0.1][pot] | 0.39% | 0.23% | 47.88% | 71.17% | 70.73% | 2.03% |
| protobuf:<br> [buffa 0.8.0][buffa] | <span title="encode">*1.70%\**</span> <span title="populate + encode">*0.59%\**</span> | 0.57% | 55.39% | 72.76% | 75.47% | 2.28% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*1.84%\**</span> <span title="populate + encode">*1.71%\**</span> | 1.00% | 55.39% | 72.76% | 75.47% | 2.16% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*0.89%\**</span> <span title="populate + encode">*0.45%\**</span> | 0.49% | 55.39% | 72.76% | 75.47% | 2.10% |
| [rkyv 0.8.16][rkyv] | 78.55% | <span title="unvalidated">*97.85%\**</span> <span title="validated upfront with error">*97.87%\**</span> | 80.78% | 90.18% | 90.65% | 18.52% |
| [ron 0.12.2][ron] | 0.08% | 0.03% | 21.84% | 54.07% | 59.56% | 1.11% |
| [savefile 0.20.4][savefile] | 99.97% | 100.00% | 80.78% | 90.17% | 90.65% | 18.87% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.87% | 3.17% | 80.78% | 90.18% | 90.65% | 18.71% |
| [serde-brief 0.2.0][serde-brief] | 0.82% | 0.41% | 30.77% | 60.44% | 71.14% | 1.73% |
| [serde_bare 0.5.0][serde_bare] | 2.51% | 3.04% | 80.78% | 90.18% | 90.65% | 18.18% |
| [speedy 0.8.7][speedy] | 100.00% | 99.25% | 80.78% | 90.18% | 90.65% | 18.58% |
| [wincode 0.5.5][wincode] | 99.14% | 99.66% | 80.78% | 90.18% | 90.65% | 18.62% |
| [wiring 0.2.4][wiring] | 99.47% | 44.22% | 80.78% | 90.17% | 90.65% | 18.50% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.17%\**</span> | <span title="packed">*0.29%\**</span> <span title="validated on-demand with error">*1.82%\**</span> | ‡ |
| [columnar 0.13.1][columnar] | 5.93% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*2.81%\**</span> | <span title="unvalidated">*49.95%\**</span> <span title="validated upfront with error">*49.93%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.97%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.96%\**</span> | <span title="unvalidated">*95.87%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*22.19%\**</span> | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*99.90%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1015.0][bilrost] | <span title="encode">*869.97 µs\**</span> <span title="prepend">*795.02 µs\**</span> | 3.1625 ms | 1.7225 ms | 489348 | 281173 | 249360 | 2.6788 ms |
| [bin-proto 0.12.8][bin-proto] | 1.8581 ms | 2.8534 ms | † | 566975 | 239350 | 231475 | 2.5009 ms |
| [bincode 2.0.1][bincode] | 340.49 µs | 1.8403 ms | 799.17 µs | 367413 | 221291 | 206242 | 2.2088 ms |
| [bincode 1.3.3][bincode1] | 596.41 µs | 1.8399 ms | 858.28 µs | 569975 | 240525 | 231884 | 2.4433 ms |
| [bitcode 0.6.9][bitcode] | 127.11 µs | 1.2638 ms | 171.04 µs | 327688 | 200947 | 182040 | 809.45 µs |
| [borsh 1.7.0][borsh] | 562.42 µs | 1.7960 ms | † | 446595 | 234236 | 209834 | 2.6498 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 483.28 µs <span title="packed">*1.0448 ms\**</span> | † | † | 803896 <span title="packed">*489017\**</span> | 335606 <span title="packed">*293127\**</span> | 280744 <span title="packed">*271528\**</span> | 3.6613 ms <span title="packed">*2.6272 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 772.91 µs | 4.4026 ms | 3.2666 ms | 1109831 | 344745 | 274333 | 3.4813 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.6679 ms | 9.8464 ms | † | 1109821 | 344751 | 274345 | 3.4456 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8529 ms | 4.5582 ms | 3.3017 ms | 1109821 | 344751 | 274345 | 3.5084 ms |
| [columnar 0.13.1][columnar] | 259.10 µs | 1.8571 ms <span title="copy_from">*691.64 µs\**</span> | † | 563392 | 249619 | 217632 | 1.6632 ms |
| [compactly 0.1.7][compactly] | 9.1664 ms | 6.5123 ms | † | 148461 | 148599 | 148473 | 176.95 µs |
| [databuf 0.5.0][databuf] | 291.85 µs | 1.7485 ms | 810.60 µs | 356311 | 213062 | 198403 | 2.0769 ms |
| [dlhn 0.1.7][dlhn] | 703.08 µs | 2.6210 ms | † | 366496 | 220600 | 205586 | 2.1051 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2855 ms | † | † | 849472 | 347816 | 294871 | 3.4216 ms |
| [flexbuffers 25.12.19][flexbuffers] | 7.7614 ms | 7.1447 ms | 5.9004 ms | 1187688 | 557642 | 553730 | 6.1821 ms |
| json:<br> [flexon 0.4.6][flexon] | 2.7657 ms | 4.5734 ms | † | 1623191 | 466527 | 359157 | 5.7253 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 3.5850 ms | 6.8250 ms | † | 1623191 | 466527 | 359157 | 5.7849 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 2.2158 ms | 4.5936 ms | † | 1623191 | 466527 | 359157 | 5.7210 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 329.96 µs | 2.7877 ms | 1.3065 ms | 391251 | 236877 | 220395 | 2.2728 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 1.5528 ms | 3.0632 ms | 1.7579 ms | 424533 | 245214 | 226077 | 2.3003 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 348.24 µs | 2.0912 ms | 870.42 µs | 416025 | 243812 | 224965 | 2.3230 ms |
| [minicbor 2.2.2][minicbor] | 574.66 µs | 3.3763 ms | 1.8706 ms | 428773 | 249857 | 228630 | 2.3253 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1074 ms | 3.8666 ms | 2.7368 ms | 449745 | 252432 | 230965 | 2.3740 ms |
| [nanoserde 0.2.1][nanoserde] | 265.46 µs | 1.8806 ms | † | 567975 | 239930 | 231872 | 2.4857 ms |
| [nibblecode 0.1.0][nibblecode] | 178.91 µs | † | † | 603928 | 429392 | 404860 | 3.6788 ms |
| [postcard 1.1.3][postcard] | 446.35 µs | 2.1828 ms | 847.33 µs | 367489 | 221913 | 207244 | 2.0978 ms |
| [pot 3.0.1][pot] | 2.3778 ms | 5.8141 ms | 4.7710 ms | 599125 | 299158 | 247675 | 2.7355 ms |
| protobuf:<br> [buffa 0.8.0][buffa] | <span title="encode">*1.1061 ms\**</span> <span title="populate + encode">*3.1301 ms\**</span> | 3.4990 ms | 2.6427 ms | 596811 | 305319 | 268737 | 3.0525 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*1.2643 ms\**</span> <span title="populate + encode">*3.0398 ms\**</span> | 3.5880 ms | † | 596811 | 305319 | 268737 | 3.0461 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.1214 ms\**</span> <span title="populate + encode">*3.1052 ms\**</span> | 3.7928 ms | † | 596811 | 305319 | 268737 | 3.0967 ms |
| [rkyv 0.8.16][rkyv] | 328.69 µs | <span title="unvalidated">*1.5110 ms\**</span> <span title="validated upfront with error">*1.8712 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3558 ms |
| [ron 0.12.2][ron] | 8.4553 ms | 26.664 ms | 24.928 ms | 1465223 | 434935 | 342907 | 5.5141 ms |
| [savefile 0.20.4][savefile] | 210.11 µs | 2.0100 ms | † | 566991 | 239362 | 231478 | 2.5104 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 641.91 µs | 2.0710 ms | † | 356311 | 212976 | 198423 | 2.0175 ms |
| [serde-brief 0.2.0][serde-brief] | 1.1684 ms | 5.1362 ms | 3.4752 ms | 1276014 | 373898 | 293384 | 3.6423 ms |
| [serde_bare 0.5.0][serde_bare] | 736.93 µs | 2.3287 ms | † | 356311 | 213062 | 198403 | 2.0069 ms |
| [speedy 0.8.7][speedy] | 270.80 µs | 1.6770 ms | 547.34 µs | 449595 | 234970 | 210192 | 2.1806 ms |
| [wincode 0.5.5][wincode] | 208.17 µs | 1.6343 ms | 554.95 µs | 566975 | 239350 | 231475 | 2.5309 ms |
| [wiring 0.2.4][wiring] | 207.69 µs | 1.9000 ms | † | 566975 | 247810 | 225086 | 2.6798 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*76.563 ns\**</span> | <span title="packed">*583.53 µs\**</span> <span title="validated on-demand with error">*420.17 ns\**</span> | ‡ |
| [columnar 0.13.1][columnar] | 733.18 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4897 ns\**</span> <span title="validated upfront with error">*2.1659 ms\**</span> | <span title="unvalidated">*1.4275 µs\**</span> <span title="validated upfront with error">*2.1640 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*253.35 µs\**</span> | <span title="unvalidated">*156.18 ns\**</span> <span title="validated upfront with error">*253.88 µs\**</span> | <span title="unvalidated">*730.74 ns\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2456 ns\**</span> <span title="validated upfront with error">*340.15 µs\**</span> | <span title="unvalidated">*156.22 ns\**</span> <span title="validated upfront with error">*340.19 µs\**</span> | <span title="unvalidated">*726.64 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1015.0][bilrost] | <span title="encode">*14.61%\**</span> <span title="prepend">*15.99%\**</span> | 21.87% | 9.93% | 30.34% | 52.85% | 59.54% | 6.61% |
| [bin-proto 0.12.8][bin-proto] | 6.84% | 24.24% | † | 26.18% | 62.08% | 64.14% | 7.08% |
| [bincode 2.0.1][bincode] | 37.33% | 37.58% | 21.40% | 40.41% | 67.15% | 71.99% | 8.01% |
| [bincode 1.3.3][bincode1] | 21.31% | 37.59% | 19.93% | 26.05% | 61.78% | 64.03% | 7.24% |
| [bitcode 0.6.9][bitcode] | 100.00% | 54.73% | 100.00% | 45.31% | 73.95% | 81.56% | 21.86% |
| [borsh 1.7.0][borsh] | 22.60% | 38.51% | † | 33.24% | 63.44% | 70.76% | 6.68% |
| capnp:<br> [capnp 0.26.0][capnp] | 26.30% <span title="packed">*12.17%\**</span> | † | † | 18.47% <span title="packed">*30.36%\**</span> | 44.28% <span title="packed">*50.69%\**</span> | 52.89% <span title="packed">*54.68%\**</span> | 4.83% <span title="packed">*6.74%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 16.45% | 15.71% | 5.24% | 13.38% | 43.10% | 54.12% | 5.08% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.47% | 7.02% | † | 13.38% | 43.10% | 54.12% | 5.14% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.86% | 15.17% | 5.18% | 13.38% | 43.10% | 54.12% | 5.04% |
| [columnar 0.13.1][columnar] | 49.06% | 37.24% <span title="copy_from">*100.00%\**</span> | † | 26.35% | 59.53% | 68.22% | 10.64% |
| [compactly 0.1.7][compactly] | 1.39% | 10.62% | † | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 43.55% | 39.56% | 21.10% | 41.67% | 69.74% | 74.83% | 8.52% |
| [dlhn 0.1.7][dlhn] | 18.08% | 26.39% | † | 40.51% | 67.36% | 72.22% | 8.41% |
| [flatbuffers 25.12.19][flatbuffers] | 3.87% | † | † | 17.48% | 42.72% | 50.35% | 5.17% |
| [flexbuffers 25.12.19][flexbuffers] | 1.64% | 9.68% | 2.90% | 12.50% | 26.65% | 26.81% | 2.86% |
| json:<br> [flexon 0.4.6][flexon] | 4.60% | 15.12% | † | 9.15% | 31.85% | 41.34% | 3.09% |
| json:<br> [serde_json 1.0.150][serde_json] | 3.55% | 10.13% | † | 9.15% | 31.85% | 41.34% | 3.06% |
| json:<br> [simd-json 0.17.0][simd-json] | 5.74% | 15.06% | † | 9.15% | 31.85% | 41.34% | 3.09% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 38.52% | 24.81% | 13.09% | 37.95% | 62.73% | 67.37% | 7.79% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 8.19% | 22.58% | 9.73% | 34.97% | 60.60% | 65.67% | 7.69% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 36.50% | 33.07% | 19.65% | 35.69% | 60.95% | 66.00% | 7.62% |
| [minicbor 2.2.2][minicbor] | 22.12% | 20.49% | 9.14% | 34.62% | 59.47% | 64.94% | 7.61% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.49% | 17.89% | 6.25% | 33.01% | 58.87% | 64.28% | 7.45% |
| [nanoserde 0.2.1][nanoserde] | 47.88% | 36.78% | † | 26.14% | 61.93% | 64.03% | 7.12% |
| [nibblecode 0.1.0][nibblecode] | 71.05% | † | † | 24.58% | 34.61% | 36.67% | 4.81% |
| [postcard 1.1.3][postcard] | 28.48% | 31.69% | 20.19% | 40.40% | 66.96% | 71.64% | 8.44% |
| [pot 3.0.1][pot] | 5.35% | 11.90% | 3.58% | 24.78% | 49.67% | 59.95% | 6.47% |
| protobuf:<br> [buffa 0.8.0][buffa] | <span title="encode">*11.49%\**</span> <span title="populate + encode">*4.06%\**</span> | 19.77% | 6.47% | 24.88% | 48.67% | 55.25% | 5.80% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*10.05%\**</span> <span title="populate + encode">*4.18%\**</span> | 19.28% | † | 24.88% | 48.67% | 55.25% | 5.81% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.33%\**</span> <span title="populate + encode">*4.09%\**</span> | 18.24% | † | 24.88% | 48.67% | 55.25% | 5.71% |
| [rkyv 0.8.16][rkyv] | 38.67% | <span title="unvalidated">*45.77%\**</span> <span title="validated upfront with error">*36.96%\**</span> | † | 24.59% | 58.33% | 67.67% | 7.51% |
| [ron 0.12.2][ron] | 1.50% | 2.59% | 0.69% | 10.13% | 34.17% | 43.30% | 3.21% |
| [savefile 0.20.4][savefile] | 60.50% | 34.41% | † | 26.18% | 62.08% | 64.14% | 7.05% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 19.80% | 33.40% | † | 41.67% | 69.77% | 74.83% | 8.77% |
| [serde-brief 0.2.0][serde-brief] | 10.88% | 13.47% | 4.92% | 11.63% | 39.74% | 50.61% | 4.86% |
| [serde_bare 0.5.0][serde_bare] | 17.25% | 29.70% | † | 41.67% | 69.74% | 74.83% | 8.82% |
| [speedy 0.8.7][speedy] | 46.94% | 41.24% | 31.25% | 33.02% | 63.24% | 70.64% | 8.11% |
| [wincode 0.5.5][wincode] | 61.06% | 42.32% | 30.82% | 26.18% | 62.08% | 64.14% | 6.99% |
| [wiring 0.2.4][wiring] | 61.20% | 36.40% | † | 26.18% | 59.96% | 65.96% | 6.60% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.63%\**</span> | <span title="packed">*0.03%\**</span> <span title="validated on-demand with error">*37.17%\**</span> | ‡ |
| [columnar 0.13.1][columnar] | 0.17% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.94%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*99.44%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.93%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1015.0][bilrost] | <span title="encode">*4.5262 ms\**</span> <span title="prepend">*2.5028 ms\**</span> | 8.5466 ms | 1704643 | 1294259 | 1245668 | 11.642 ms |
| [bin-proto 0.12.8][bin-proto] | 5.4619 ms | 6.7146 ms | 1791489 | 1127998 | 1051146 | 10.223 ms |
| [bincode 2.0.1][bincode] | 1.2842 ms | 3.8056 ms | 1406257 | 1117802 | 1062438 | 9.5018 ms |
| [bincode 1.3.3][bincode1] | 3.7870 ms | 4.5051 ms | 1854234 | 1141994 | 1048745 | 10.367 ms |
| [bitcode 0.6.9][bitcode] | 701.08 µs | 2.3760 ms | 971318 | 878034 | 850340 | 2.9191 ms |
| [borsh 1.7.0][borsh] | 2.8105 ms | 2.8504 ms | 1521989 | 1108471 | 1038528 | 9.9238 ms |
| capnp:<br> [capnp 0.26.0][capnp] | 2.6850 ms <span title="packed">*4.5496 ms\**</span> | † | 2724288 <span title="packed">*1616255\**</span> | 1546992 <span title="packed">*1278764\**</span> | 1239111 <span title="packed">*1125654\**</span> | 14.381 ms <span title="packed">*8.6907 ms\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 3.4675 ms | 16.942 ms | 6012539 | 1695215 | 1464951 | 21.235 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.999 ms | 53.105 ms | 6012373 | 1695146 | 1465025 | 21.394 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.314 ms | 19.947 ms | 6012373 | 1695146 | 1465025 | 21.093 ms |
| [columnar 0.13.1][columnar] | 891.30 µs | 3.7179 ms <span title="copy_from">*1.2761 ms\**</span> | 1544720 | 996718 | 896213 | 4.7816 ms |
| [compactly 0.1.7][compactly] | 53.147 ms | 33.436 ms | 800772 | 801349 | 800799 | 490.01 µs |
| [databuf 0.5.0][databuf] | 1.3065 ms | 3.7824 ms | 1319999 | 1062631 | 1008334 | 8.8591 ms |
| [dlhn 0.1.7][dlhn] | 4.4202 ms | 6.5524 ms | 1311281 | 1077520 | 1046095 | 8.8112 ms |
| [flatbuffers 25.12.19][flatbuffers] | 5.2671 ms | † | 2325620 | 1439185 | 1268060 | 13.577 ms |
| [flexbuffers 25.12.19][flexbuffers] | 40.133 ms | 36.770 ms | 5352680 | 2658295 | 2777967 | 34.780 ms |
| json:<br> [flexon 0.4.6][flexon] | 15.331 ms | 24.067 ms | 9390461 | 2391679 | 1842767 | 34.545 ms |
| json:<br> [serde_json 1.0.150][serde_json] | 19.898 ms | 31.002 ms | 9390461 | 2391679 | 1842767 | 34.645 ms |
| json:<br> [simd-json 0.17.0][simd-json] | 12.121 ms | 25.106 ms | 9390461 | 2391679 | 1842767 | 34.430 ms |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 935.58 µs | 5.5019 ms | 1458773 | 1156055 | 1137788 | 9.7079 ms |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 10.903 ms | 11.491 ms | 1745322 | 1261627 | 1228923 | 11.539 ms |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 1.1203 ms | 4.1926 ms | 1717696 | 1234725 | 1195988 | 11.136 ms |
| [minicbor 2.2.2][minicbor] | 2.3084 ms | 11.519 ms | 1777386 | 1276218 | 1252558 | 12.618 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.077 ms | 16.155 ms | 1770060 | 1277755 | 1263362 | 12.575 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2944 ms | 2.7556 ms | 1812404 | 1134820 | 1053109 | 10.371 ms |
| [nibblecode 0.1.0][nibblecode] | 505.31 µs | † | 2075936 | 1518443 | 1413193 | 14.034 ms |
| [postcard 1.1.3][postcard] | 1.8074 ms | 4.1911 ms | 1311281 | 1083900 | 1041434 | 8.7018 ms |
| [pot 3.0.1][pot] | 13.809 ms | 28.920 ms | 2604812 | 1482233 | 1298928 | 15.974 ms |
| protobuf:<br> [buffa 0.8.0][buffa] | <span title="encode">*3.6219 ms\**</span> <span title="populate + encode">*11.075 ms\**</span> | 14.918 ms | 1859886 | 1338076 | 1295351 | 12.408 ms |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*5.4766 ms\**</span> <span title="populate + encode">*9.5001 ms\**</span> | 8.6310 ms | 1859886 | 1338076 | 1295351 | 12.967 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.7179 ms\**</span> <span title="populate + encode">*13.024 ms\**</span> | 11.971 ms | 1859886 | 1338076 | 1295351 | 12.390 ms |
| [rkyv 0.8.16][rkyv] | 979.11 µs | <span title="unvalidated">*2.1818 ms\**</span> <span title="validated upfront with error">*2.6611 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.079 ms |
| [ron 0.12.2][ron] | 46.073 ms | 170.77 ms | 8677703 | 2233642 | 1826180 | 34.273 ms |
| [savefile 0.20.4][savefile] | 856.22 µs | 2.8340 ms | 1791505 | 1128012 | 1051153 | 10.344 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1627 ms | 3.3369 ms | 1319999 | 1064380 | 1010708 | 8.9164 ms |
| [serde-brief 0.2.0][serde-brief] | 5.4023 ms | 20.759 ms | 6951772 | 1796265 | 1567819 | 23.404 ms |
| [serde_bare 0.5.0][serde_bare] | 4.0930 ms | 4.9713 ms | 1319999 | 1062645 | 1008349 | 8.9199 ms |
| [speedy 0.8.7][speedy] | 762.04 µs | 2.4455 ms | 1584734 | 1119837 | 1037992 | 10.042 ms |
| [wincode 0.5.5][wincode] | 568.01 µs | 2.3163 ms | 1791489 | 1127998 | 1051146 | 10.309 ms |
| [wiring 0.2.4][wiring] | 636.33 µs | 2.7939 ms | 1791489 | 1156963 | 1082815 | 10.603 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*77.715 ns\**</span> | <span title="packed">*1.9882 ms\**</span> <span title="validated on-demand with error">*725.64 ns\**</span> | ‡ |
| [columnar 0.13.1][columnar] | 52.440 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4898 ns\**</span> <span title="validated upfront with error">*6.1054 ms\**</span> | <span title="unvalidated">*2.7486 µs\**</span> <span title="validated upfront with error">*6.1062 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2449 ns\**</span> <span title="validated upfront with error">*370.89 µs\**</span> | <span title="unvalidated">*385.03 ns\**</span> <span title="validated upfront with error">*375.27 µs\**</span> | <span title="unvalidated">*237.93 ns\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*1.2450 ns\**</span> <span title="validated upfront with error">*459.44 µs\**</span> | <span title="unvalidated">*385.76 ns\**</span> <span title="validated upfront with error">*459.63 µs\**</span> | <span title="unvalidated">*235.58 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1015.0][bilrost] | <span title="encode">*11.16%\**</span> <span title="prepend">*20.19%\**</span> | 14.93% | 46.98% | 61.92% | 64.29% | 4.21% |
| [bin-proto 0.12.8][bin-proto] | 9.25% | 19.00% | 44.70% | 71.04% | 76.18% | 4.79% |
| [bincode 2.0.1][bincode] | 39.35% | 33.53% | 56.94% | 71.69% | 75.37% | 5.16% |
| [bincode 1.3.3][bincode1] | 13.34% | 28.33% | 43.19% | 70.17% | 76.36% | 4.73% |
| [bitcode 0.6.9][bitcode] | 72.08% | 53.71% | 82.44% | 91.27% | 94.17% | 16.79% |
| [borsh 1.7.0][borsh] | 17.98% | 44.77% | 52.61% | 72.29% | 77.11% | 4.94% |
| capnp:<br> [capnp 0.26.0][capnp] | 18.82% <span title="packed">*11.11%\**</span> | † | 29.39% <span title="packed">*49.54%\**</span> | 51.80% <span title="packed">*62.67%\**</span> | 64.63% <span title="packed">*71.14%\**</span> | 3.41% <span title="packed">*5.64%\**</span> |
| cbor:<br> [cbor4ii 1.2.2][cbor4ii] | 14.57% | 7.53% | 13.32% | 47.27% | 54.66% | 2.31% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.11% | 2.40% | 13.32% | 47.27% | 54.66% | 2.29% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 4.90% | 6.40% | 13.32% | 47.27% | 54.66% | 2.32% |
| [columnar 0.13.1][columnar] | 56.69% | 34.32% <span title="copy_from">*100.00%\**</span> | 51.84% | 80.40% | 89.35% | 10.25% |
| [compactly 0.1.7][compactly] | 0.95% | 3.82% | 100.00% | 100.00% | 100.00% | 100.00% |
| [databuf 0.5.0][databuf] | 38.68% | 33.74% | 60.66% | 75.41% | 79.42% | 5.53% |
| [dlhn 0.1.7][dlhn] | 11.43% | 19.48% | 61.07% | 74.37% | 76.55% | 5.56% |
| [flatbuffers 25.12.19][flatbuffers] | 9.59% | † | 34.43% | 55.68% | 63.15% | 3.61% |
| [flexbuffers 25.12.19][flexbuffers] | 1.26% | 3.47% | 14.96% | 30.15% | 28.83% | 1.41% |
| json:<br> [flexon 0.4.6][flexon] | 3.30% | 5.30% | 8.53% | 33.51% | 43.46% | 1.42% |
| json:<br> [serde_json 1.0.150][serde_json] | 2.54% | 4.12% | 8.53% | 33.51% | 43.46% | 1.41% |
| json:<br> [simd-json 0.17.0][simd-json] | 4.17% | 5.08% | 8.53% | 33.51% | 43.46% | 1.42% |
| messagepack:<br> [msgpacker 0.7.1][msgpacker] | 54.01% | 23.19% | 54.89% | 69.32% | 70.38% | 5.05% |
| messagepack:<br> [rmp-serde 1.3.1][rmp-serde] | 4.63% | 11.11% | 45.88% | 63.52% | 65.16% | 4.25% |
| messagepack:<br> [zerompk 0.6.0][zerompk] | 45.10% | 30.44% | 46.62% | 64.90% | 66.96% | 4.40% |
| [minicbor 2.2.2][minicbor] | 21.89% | 11.08% | 45.05% | 62.79% | 63.93% | 3.88% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.68% | 7.90% | 45.24% | 62.72% | 63.39% | 3.90% |
| [nanoserde 0.2.1][nanoserde] | 39.04% | 46.31% | 44.18% | 70.61% | 76.04% | 4.72% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 38.57% | 52.77% | 56.67% | 3.49% |
| [postcard 1.1.3][postcard] | 27.96% | 30.45% | 61.07% | 73.93% | 76.89% | 5.63% |
| [pot 3.0.1][pot] | 3.66% | 4.41% | 30.74% | 54.06% | 61.65% | 3.07% |
| protobuf:<br> [buffa 0.8.0][buffa] | <span title="encode">*13.95%\**</span> <span title="populate + encode">*4.56%\**</span> | 8.55% | 43.05% | 59.89% | 61.82% | 3.95% |
| protobuf:<br> [prost 0.14.4][prost] | <span title="encode">*9.23%\**</span> <span title="populate + encode">*5.32%\**</span> | 14.79% | 43.05% | 59.89% | 61.82% | 3.78% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*8.84%\**</span> <span title="populate + encode">*3.88%\**</span> | 10.66% | 43.05% | 59.89% | 61.82% | 3.95% |
| [rkyv 0.8.16][rkyv] | 51.61% | <span title="unvalidated">*58.49%\**</span> <span title="validated upfront with error">*47.95%\**</span> | 38.57% | 57.91% | 66.16% | 3.75% |
| [ron 0.12.2][ron] | 1.10% | 0.75% | 9.23% | 35.88% | 43.85% | 1.43% |
| [savefile 0.20.4][savefile] | 59.02% | 45.03% | 44.70% | 71.04% | 76.18% | 4.74% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 15.98% | 38.24% | 60.66% | 75.29% | 79.23% | 5.50% |
| [serde-brief 0.2.0][serde-brief] | 9.35% | 6.15% | 11.52% | 44.61% | 51.08% | 2.09% |
| [serde_bare 0.5.0][serde_bare] | 12.35% | 25.67% | 60.66% | 75.41% | 79.42% | 5.49% |
| [speedy 0.8.7][speedy] | 66.31% | 52.18% | 50.53% | 71.56% | 77.15% | 4.88% |
| [wincode 0.5.5][wincode] | 88.96% | 55.09% | 44.70% | 71.04% | 76.18% | 4.75% |
| [wiring 0.2.4][wiring] | 79.41% | 45.67% | 44.70% | 69.26% | 73.96% | 4.62% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| capnp:<br> [capnp 0.26.0][capnp] | <span title="validated on-demand with error">*1.60%\**</span> | <span title="packed">*0.02%\**</span> <span title="validated on-demand with error">*53.06%\**</span> | ‡ |
| [columnar 0.13.1][columnar] | 2.37% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*14.01%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*99.01%\**</span> |
| [rkyv 0.8.16][rkyv] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.81%\**</span> <span title="validated upfront with error">*0.08%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1015.0
[bin-proto]: https://crates.io/crates/bin-proto/0.12.8
[bincode]: https://crates.io/crates/bincode/2.0.1
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.9
[borsh]: https://crates.io/crates/borsh/1.7.0
[buffa]: https://crates.io/crates/buffa/0.8.0
[capnp]: https://crates.io/crates/capnp/0.26.0
[cbor4ii]: https://crates.io/crates/cbor4ii/1.2.2
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[columnar]: https://crates.io/crates/columnar/0.13.1
[compactly]: https://crates.io/crates/compactly/0.1.7
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.12.19
[flexbuffers]: https://crates.io/crates/flexbuffers/25.12.19
[flexon]: https://crates.io/crates/flexon/0.4.6
[minicbor]: https://crates.io/crates/minicbor/2.2.2
[msgpacker]: https://crates.io/crates/msgpacker/0.7.1
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.2.1
[nibblecode]: https://crates.io/crates/nibblecode/0.1.0
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.7.5
[postcard]: https://crates.io/crates/postcard/1.1.3
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.14.4
[protobuf]: https://crates.io/crates/protobuf/3.7.2
[rkyv]: https://crates.io/crates/rkyv/0.8.16
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.1
[ron]: https://crates.io/crates/ron/0.12.2
[savefile]: https://crates.io/crates/savefile/0.20.4
[serde-brief]: https://crates.io/crates/serde-brief/0.2.0
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.150
[simd-json]: https://crates.io/crates/simd-json/0.17.0
[speedy]: https://crates.io/crates/speedy/0.8.7
[wincode]: https://crates.io/crates/wincode/0.5.5
[wiring]: https://crates.io/crates/wiring/0.2.4
[zerompk]: https://crates.io/crates/zerompk/0.6.0


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
