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

## Last updated: 2025-12-04 20:02:51

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.93.0-nightly (83e49b75e 2025-12-03)
binary: rustc
commit-hash: 83e49b75e7daf827e4390ae0ccbcb0d0e2c96493
commit-date: 2025-12-03
host: x86_64-unknown-linux-gnu
release: 1.93.0-nightly
LLVM version: 21.1.5
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*430.56 µs\**</span> <span title="prepend">*414.06 µs\**</span> | 2.5113 ms | 840.64 µs | 804955 | 328941 | 284849 | 4.2974 ms |
| [bincode 2.0.1][bincode] | 301.19 µs | 2.2351 ms | 712.03 µs | 741295 | 303944 | 256422 | 3.7470 ms |
| [bincode 1.3.3][bincode1] | 564.59 µs | 2.0683 ms | 567.34 µs | 1045784 | 373127 | 311553 | 4.5337 ms |
| [bitcode 0.6.6][bitcode] | 146.18 µs | 1.4381 ms | 60.211 µs | 703710 | 288826 | 227322 | 2.5429 ms |
| [borsh 1.5.7][borsh] | 553.81 µs | 2.1425 ms | † | 885780 | 362204 | 286248 | 4.1750 ms |
| [capnp 0.21.1][capnp] | 453.90 µs | † | † | 1443216 | 513986 | 426532 | 6.2890 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 636.96 µs | 4.9404 ms | 3.3232 ms | 1407835 | 403440 | 323561 | 5.0999 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1705 ms | 11.430 ms | † | 1407835 | 403440 | 323561 | 5.0472 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 2.0124 ms | 4.6894 ms | 2.9129 ms | 1407835 | 403440 | 323561 | 5.8238 ms |
| [databuf 0.5.0][databuf] | 258.25 µs | 1.9927 ms | 659.07 µs | 765778 | 311715 | 263914 | 3.4421 ms |
| [dlhn 0.1.7][dlhn] | 725.82 µs | 2.4819 ms | † | 724953 | 301446 | 253056 | 3.1604 ms |
| [flatbuffers 25.2.10][flatbuffers] | 1.0418 ms | † | † | 1276368 | 468539 | 388381 | 4.7388 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.5240 ms | 7.5242 ms | 5.7457 ms | 1829756 | 714318 | 691541 | 8.6194 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 4.0254 ms | 6.0979 ms | † | 1827461 | 470560 | 360727 | 5.5113 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1024 ms | 4.7493 ms | † | 1827461 | 470560 | 360727 | 6.3515 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.2922 ms | 2.5163 ms | † | 764996 | 315291 | 264212 | 3.5922 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.3781 ms | 3.0969 ms | 1.3814 ms | 784997 | 325384 | 277608 | 3.9757 ms |
| [minicbor 1.0.0][minicbor] | 459.89 µs | 2.8250 ms | 1.3468 ms | 817830 | 332671 | 284034 | 3.9993 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2834 ms | 4.7990 ms | 3.1451 ms | 818669 | 332556 | 284797 | 3.9946 ms |
| [nanoserde 0.2.1][nanoserde] | 243.43 µs | 2.0210 ms | † | 1045784 | 373127 | 311553 | 4.2016 ms |
| [nibblecode 0.1.0][nibblecode] | 187.01 µs | † | † | 1011487 | 487313 | 419843 | 5.9355 ms |
| [postcard 1.1.1][postcard] | 436.85 µs | 2.1985 ms | 614.82 µs | 724953 | 302399 | 252968 | 3.5041 ms |
| [pot 3.0.1][pot] | 2.4070 ms | 6.3566 ms | 4.8383 ms | 971922 | 372513 | 303636 | 4.3271 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*940.11 µs\**</span> <span title="populate + encode">*2.4301 ms\**</span> | 3.4756 ms | † | 884628 | 363130 | 314959 | 4.5283 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*948.14 µs\**</span> <span title="populate + encode">*2.7317 ms\**</span> | 3.7434 ms | † | 884628 | 363130 | 314959 | 4.4448 ms |
| [rkyv 0.8.10][rkyv] | 244.68 µs | <span title="unvalidated">*1.5416 ms\**</span> <span title="validated upfront with error">*1.9160 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6406 ms |
| [ron 0.10.1][ron] | 11.735 ms | 23.209 ms | 21.800 ms | 1607459 | 449158 | 349324 | 5.6517 ms |
| [savefile 0.18.6][savefile] | 191.98 µs | 2.1397 ms | † | 1045800 | 373139 | 311562 | 4.2482 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 659.28 µs | 2.4028 ms | † | 765778 | 311743 | 263822 | 3.5475 ms |
| [serde-brief 0.1.1][serde-brief] | 1.5413 ms | 4.8101 ms | 2.9048 ms | 1584946 | 413733 | 339964 | 4.9192 ms |
| [serde_bare 0.5.0][serde_bare] | 700.11 µs | 2.0372 ms | † | 765778 | 311715 | 263914 | 3.6295 ms |
| [speedy 0.8.7][speedy] | 200.71 µs | 1.7282 ms | 369.63 µs | 885780 | 362204 | 286248 | 4.0128 ms |
| [wincode 0.2.0][wincode] | 177.35 µs | 1.9139 ms | 449.70 µs | 1045784 | 373127 | 311553 | 4.2276 ms |
| [wiring 0.2.4][wiring] | 196.39 µs | 1.9038 ms | † | 1045784 | 337930 | 275808 | 3.6698 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*83.602 ns\**</span> | <span title="validated on-demand with error">*182.34 µs\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4901 ns\**</span> <span title="validated upfront with error">*2.1225 ms\**</span> | <span title="unvalidated">*51.329 µs\**</span> <span title="validated upfront with error">*2.0772 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2427 ns\**</span> <span title="validated upfront with error">*262.38 µs\**</span> | <span title="unvalidated">*10.336 µs\**</span> <span title="validated upfront with error">*273.27 µs\**</span> | <span title="unvalidated">*7.6174 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2436 ns\**</span> <span title="validated upfront with error">*377.77 µs\**</span> | <span title="unvalidated">*10.380 µs\**</span> <span title="validated upfront with error">*384.47 µs\**</span> | <span title="unvalidated">*7.6185 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*33.95%\**</span> <span title="prepend">*35.30%\**</span> | 57.27% | 7.16% | 87.42% | 87.80% | 79.80% | 59.17% |
| [bincode 2.0.1][bincode] | 48.53% | 64.34% | 8.46% | 94.93% | 95.03% | 88.65% | 67.86% |
| [bincode 1.3.3][bincode1] | 25.89% | 69.53% | 10.61% | 67.29% | 77.41% | 72.96% | 56.09% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 26.40% | 67.12% | † | 79.45% | 79.74% | 79.41% | 60.91% |
| [capnp 0.21.1][capnp] | 32.21% | † | † | 48.76% | 56.19% | 53.30% | 40.43% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 22.95% | 29.11% | 1.81% | 49.99% | 71.59% | 70.26% | 49.86% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.61% | 12.58% | † | 49.99% | 71.59% | 70.26% | 50.38% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.26% | 30.67% | 2.07% | 49.99% | 71.59% | 70.26% | 43.66% |
| [databuf 0.5.0][databuf] | 56.60% | 72.17% | 9.14% | 91.89% | 92.66% | 86.13% | 73.88% |
| [dlhn 0.1.7][dlhn] | 20.14% | 57.94% | † | 97.07% | 95.81% | 89.83% | 80.46% |
| [flatbuffers 25.2.10][flatbuffers] | 14.03% | † | † | 55.13% | 61.64% | 58.53% | 53.66% |
| [flexbuffers 25.2.10][flexbuffers] | 2.24% | 19.11% | 1.05% | 38.46% | 40.43% | 32.87% | 29.50% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.63% | 23.58% | † | 38.51% | 61.38% | 63.02% | 46.14% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.95% | 30.28% | † | 38.51% | 61.38% | 63.02% | 40.04% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 11.31% | 57.15% | † | 91.99% | 91.61% | 86.04% | 70.79% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.61% | 46.44% | 4.36% | 89.64% | 88.76% | 81.89% | 63.96% |
| [minicbor 1.0.0][minicbor] | 31.79% | 50.91% | 4.47% | 86.05% | 86.82% | 80.03% | 63.58% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.77% | 29.97% | 1.91% | 85.96% | 86.85% | 79.82% | 63.66% |
| [nanoserde 0.2.1][nanoserde] | 60.05% | 71.16% | † | 67.29% | 77.41% | 72.96% | 60.52% |
| [nibblecode 0.1.0][nibblecode] | 78.17% | † | † | 69.57% | 59.27% | 54.14% | 42.84% |
| [postcard 1.1.1][postcard] | 33.46% | 65.41% | 9.79% | 97.07% | 95.51% | 89.86% | 72.57% |
| [pot 3.0.1][pot] | 6.07% | 22.62% | 1.24% | 72.40% | 77.53% | 74.87% | 58.77% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*15.55%\**</span> <span title="populate + encode">*6.02%\**</span> | 41.38% | † | 79.55% | 79.54% | 72.18% | 56.16% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*15.42%\**</span> <span title="populate + encode">*5.35%\**</span> | 38.42% | † | 79.55% | 79.54% | 72.18% | 57.21% |
| [rkyv 0.8.10][rkyv] | 59.74% | <span title="unvalidated">*93.29%\**</span> <span title="validated upfront with error">*75.06%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.80% |
| [ron 0.10.1][ron] | 1.25% | 6.20% | 0.28% | 43.78% | 64.30% | 65.07% | 44.99% |
| [savefile 0.18.6][savefile] | 76.14% | 67.21% | † | 67.29% | 77.40% | 72.96% | 59.86% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.17% | 59.85% | † | 91.89% | 92.65% | 86.16% | 71.68% |
| [serde-brief 0.1.1][serde-brief] | 9.48% | 29.90% | 2.07% | 44.40% | 69.81% | 66.87% | 51.69% |
| [serde_bare 0.5.0][serde_bare] | 20.88% | 70.59% | † | 91.89% | 92.66% | 86.13% | 70.06% |
| [speedy 0.8.7][speedy] | 72.83% | 83.21% | 16.29% | 79.45% | 79.74% | 79.41% | 63.37% |
| [wincode 0.2.0][wincode] | 82.42% | 75.14% | 13.39% | 67.29% | 77.41% | 72.96% | 60.15% |
| [wiring 0.2.4][wiring] | 74.43% | 75.54% | † | 67.29% | 85.47% | 82.42% | 69.29% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.49%\**</span> | <span title="validated on-demand with error">*5.67%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.91%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.14%\**</span> <span title="validated upfront with error">*0.50%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*3.78%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.93%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.58%\**</span> <span title="validated upfront with error">*2.69%\**</span> | <span title="unvalidated">*99.99%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.3681 ms\**</span> <span title="prepend">*8.8436 ms\**</span> | 7.6270 ms | 8625005 | 6443961 | 6231572 | 72.988 ms |
| [bincode 2.0.1][bincode] | 2.8826 ms | 824.64 µs | 6000005 | 5378497 | 5346882 | 8.7138 ms |
| [bincode 1.3.3][bincode1] | 5.9018 ms | 4.7697 ms | 6000008 | 5378500 | 5346908 | 8.8424 ms |
| [bitcode 0.6.6][bitcode] | 1.2982 ms | 803.68 µs | 6000006 | 5182295 | 4921841 | 13.902 ms |
| [borsh 1.5.7][borsh] | 6.3087 ms | 4.1797 ms | 6000004 | 5378496 | 5346866 | 9.0351 ms |
| [capnp 0.21.1][capnp] | 5.3454 ms | † | 14000088 | 7130367 | 6046182 | 81.412 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 8.8459 ms | 48.092 ms | 13125016 | 7524114 | 6757437 | 92.390 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 65.323 ms | 113.61 ms | 13122324 | 7524660 | 6759128 | 91.418 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 32.571 ms | 40.481 ms | 13122324 | 7524660 | 6759128 | 91.658 ms |
| [databuf 0.5.0][databuf] | 2.4185 ms | 5.4012 ms | 6000003 | 5378495 | 5346897 | 8.9981 ms |
| [dlhn 0.1.7][dlhn] | 6.2115 ms | 7.2404 ms | 6000003 | 5378495 | 5346897 | 8.9213 ms |
| [flatbuffers 25.2.10][flatbuffers] | 978.50 µs | † | 6000024 | 5378434 | 5346878 | 8.7924 ms |
| [flexbuffers 25.2.10][flexbuffers] | 101.16 ms | 92.099 ms | 26609424 | 11901040 | 12486322 | 153.92 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 87.651 ms | 100.08 ms | 26192883 | 9566084 | 8584671 | 154.78 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 52.667 ms | 70.467 ms | 26192883 | 9566084 | 8584671 | 154.31 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 19.701 ms | 5.1320 ms | 7500005 | 6058442 | 6014500 | 10.639 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 15.462 ms | 17.767 ms | 8125006 | 6494876 | 6391037 | 70.421 ms |
| [minicbor 1.0.0][minicbor] | 5.1921 ms | 11.711 ms | 8125006 | 6494907 | 6390894 | 70.872 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 120.20 ms | 30.443 ms | 8125037 | 6493484 | 6386940 | 71.178 ms |
| [nanoserde 0.2.1][nanoserde] | 1.1161 ms | 888.94 µs | 6000008 | 5378500 | 5346908 | 8.5607 ms |
| [nibblecode 0.1.0][nibblecode] | 148.71 µs | † | 6000008 | 5378500 | 5346908 | 8.5994 ms |
| [postcard 1.1.1][postcard] | 480.01 µs | 1.3087 ms | 6000003 | 5378495 | 5346897 | 8.5999 ms |
| [pot 3.0.1][pot] | 39.668 ms | 68.638 ms | 10122342 | 6814618 | 6852252 | 81.541 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*7.8105 ms\**</span> <span title="populate + encode">*8.3790 ms\**</span> | 14.100 ms | 8750000 | 6665735 | 6421877 | 71.851 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.367 ms\**</span> <span title="populate + encode">*30.214 ms\**</span> | 29.432 ms | 8750000 | 6665735 | 6421877 | 78.463 ms |
| [rkyv 0.8.10][rkyv] | 153.98 µs | <span title="unvalidated">*200.92 µs\**</span> <span title="validated upfront with error">*200.60 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.5617 ms |
| [ron 0.10.1][ron] | 166.71 ms | 521.48 ms | 22192885 | 8970395 | 8137334 | 151.25 ms |
| [savefile 0.18.6][savefile] | 199.35 µs | 148.60 µs | 6000024 | 5378519 | 5346896 | 8.7222 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.1506 ms | 4.2248 ms | 6000004 | 5378496 | 5346866 | 8.8368 ms |
| [serde-brief 0.1.1][serde-brief] | 22.417 ms | 40.232 ms | 15750015 | 8024540 | 6813667 | 96.020 ms |
| [serde_bare 0.5.0][serde_bare] | 5.9778 ms | 4.8505 ms | 6000003 | 5378495 | 5346897 | 8.5588 ms |
| [speedy 0.8.7][speedy] | 148.61 µs | 188.40 µs | 6000004 | 5378496 | 5346866 | 8.9034 ms |
| [wincode 0.2.0][wincode] | 203.29 µs | 148.83 µs | 6000008 | 5378500 | 5346908 | 8.7945 ms |
| [wiring 0.2.4][wiring] | 201.39 µs | 320.35 µs | 6000008 | 5378952 | 5346905 | 8.6475 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*114.66 ns\**</span> | <span title="validated on-demand with error">*2.2915 ms\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4964 ns\**</span> <span title="validated upfront with error">*45.154 ns\**</span> | <span title="unvalidated">*78.337 µs\**</span> <span title="validated upfront with error">*77.771 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*1.8677 ns\**</span> | <span title="unvalidated">*38.851 µs\**</span> <span title="validated upfront with error">*38.852 µs\**</span> | <span title="unvalidated">*100.61 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2425 ns\**</span> <span title="validated upfront with error">*5.6001 ns\**</span> | <span title="unvalidated">*38.836 µs\**</span> <span title="validated upfront with error">*38.848 µs\**</span> | <span title="unvalidated">*99.857 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.02%\**</span> <span title="prepend">*1.68%\**</span> | 1.95% | 69.57% | 80.42% | 78.98% | 11.73% |
| [bincode 2.0.1][bincode] | 5.16% | 18.02% | 100.00% | 96.35% | 92.05% | 98.22% |
| [bincode 1.3.3][bincode1] | 2.52% | 3.12% | 100.00% | 96.35% | 92.05% | 96.79% |
| [bitcode 0.6.6][bitcode] | 11.45% | 18.49% | 100.00% | 100.00% | 100.00% | 61.57% |
| [borsh 1.5.7][borsh] | 2.36% | 3.56% | 100.00% | 96.35% | 92.05% | 94.73% |
| [capnp 0.21.1][capnp] | 2.78% | † | 42.86% | 72.68% | 81.40% | 10.51% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.68% | 0.31% | 45.71% | 68.88% | 72.84% | 9.26% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.23% | 0.13% | 45.72% | 68.87% | 72.82% | 9.36% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.46% | 0.37% | 45.72% | 68.87% | 72.82% | 9.34% |
| [databuf 0.5.0][databuf] | 6.14% | 2.75% | 100.00% | 96.35% | 92.05% | 95.12% |
| [dlhn 0.1.7][dlhn] | 2.39% | 2.05% | 100.00% | 96.35% | 92.05% | 95.94% |
| [flatbuffers 25.2.10][flatbuffers] | 15.19% | † | 100.00% | 96.35% | 92.05% | 97.34% |
| [flexbuffers 25.2.10][flexbuffers] | 0.15% | 0.16% | 22.55% | 43.54% | 39.42% | 5.56% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.15% | 22.91% | 54.17% | 57.33% | 5.53% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.28% | 0.21% | 22.91% | 54.17% | 57.33% | 5.55% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 0.75% | 2.90% | 80.00% | 85.54% | 81.83% | 80.45% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.96% | 0.84% | 73.85% | 79.79% | 77.01% | 12.15% |
| [minicbor 1.0.0][minicbor] | 2.86% | 1.27% | 73.85% | 79.79% | 77.01% | 12.08% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.49% | 73.85% | 79.81% | 77.06% | 12.02% |
| [nanoserde 0.2.1][nanoserde] | 13.32% | 16.72% | 100.00% | 96.35% | 92.05% | 99.98% |
| [nibblecode 0.1.0][nibblecode] | 99.93% | † | 100.00% | 96.35% | 92.05% | 99.53% |
| [postcard 1.1.1][postcard] | 30.96% | 11.35% | 100.00% | 96.35% | 92.05% | 99.52% |
| [pot 3.0.1][pot] | 0.37% | 0.22% | 59.27% | 76.05% | 71.83% | 10.50% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.90%\**</span> <span title="populate + encode">*1.77%\**</span> | 1.05% | 68.57% | 77.75% | 76.64% | 11.91% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.03%\**</span> <span title="populate + encode">*0.49%\**</span> | 0.50% | 68.57% | 77.75% | 76.64% | 10.91% |
| [rkyv 0.8.10][rkyv] | 96.51% | <span title="unvalidated">*73.96%\**</span> <span title="validated upfront with error">*74.08%\**</span> | 100.00% | 96.35% | 92.05% | 99.97% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 27.04% | 57.77% | 60.48% | 5.66% |
| [savefile 0.18.6][savefile] | 74.55% | 100.00% | 100.00% | 96.35% | 92.05% | 98.13% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 2.89% | 3.52% | 100.00% | 96.35% | 92.05% | 96.85% |
| [serde-brief 0.1.1][serde-brief] | 0.66% | 0.37% | 38.10% | 64.58% | 72.23% | 8.91% |
| [serde_bare 0.5.0][serde_bare] | 2.49% | 3.06% | 100.00% | 96.35% | 92.05% | 100.00% |
| [speedy 0.8.7][speedy] | 100.00% | 78.87% | 100.00% | 96.35% | 92.05% | 96.13% |
| [wincode 0.2.0][wincode] | 73.10% | 99.85% | 100.00% | 96.35% | 92.05% | 97.32% |
| [wiring 0.2.4][wiring] | 73.79% | 46.39% | 100.00% | 96.34% | 92.05% | 98.97% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.08%\**</span> | <span title="validated on-demand with error">*1.69%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*49.77%\**</span> <span title="validated upfront with error">*2.75%\**</span> | <span title="unvalidated">*49.58%\**</span> <span title="validated upfront with error">*49.94%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*66.53%\**</span> | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*99.96%\**</span> | <span title="unvalidated">*99.25%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*22.19%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.97%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*887.81 µs\**</span> <span title="prepend">*777.79 µs\**</span> | 3.1104 ms | 1.6844 ms | 489348 | 281173 | 249360 | 2.6380 ms |
| [bincode 2.0.1][bincode] | 283.92 µs | 1.8564 ms | 831.31 µs | 367413 | 221291 | 206242 | 2.0131 ms |
| [bincode 1.3.3][bincode1] | 563.45 µs | 1.8218 ms | 874.73 µs | 569975 | 240525 | 231884 | 2.4482 ms |
| [bitcode 0.6.6][bitcode] | 127.54 µs | 1.2560 ms | 169.86 µs | 327688 | 200947 | 182040 | 747.00 µs |
| [borsh 1.5.7][borsh] | 556.80 µs | 1.8215 ms | † | 446595 | 234236 | 209834 | 2.0546 ms |
| [capnp 0.21.1][capnp] | 457.41 µs | † | † | 803896 | 335606 | 280744 | 3.7626 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 740.27 µs | 4.4970 ms | 3.3506 ms | 1109831 | 344745 | 274333 | 3.4423 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.7258 ms | 10.372 ms | † | 1109821 | 344751 | 274345 | 3.4695 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8750 ms | 4.5232 ms | 3.2715 ms | 1109821 | 344751 | 274345 | 3.7552 ms |
| [databuf 0.5.0][databuf] | 293.60 µs | 1.7125 ms | 779.31 µs | 356311 | 213062 | 198403 | 1.9440 ms |
| [dlhn 0.1.7][dlhn] | 795.14 µs | 2.6037 ms | † | 366496 | 220600 | 205586 | 2.0024 ms |
| [flatbuffers 25.2.10][flatbuffers] | 3.2678 ms | † | † | 849472 | 347816 | 294871 | 3.4955 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.7991 ms | 6.8527 ms | 5.6758 ms | 1187688 | 557642 | 553730 | 6.2760 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.8206 ms | 6.9259 ms | † | 1623191 | 466527 | 359157 | 5.8143 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2334 ms | 4.6375 ms | † | 1623191 | 466527 | 359157 | 5.6686 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 960.26 µs | 2.8575 ms | † | 391251 | 236877 | 220395 | 2.1681 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4051 ms | 2.9565 ms | 1.6673 ms | 424533 | 245214 | 226077 | 2.4071 ms |
| [minicbor 1.0.0][minicbor] | 561.26 µs | 3.3440 ms | 1.8376 ms | 428773 | 249857 | 228630 | 2.2553 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.1143 ms | 4.2694 ms | 3.1114 ms | 449745 | 252432 | 230965 | 2.3155 ms |
| [nanoserde 0.2.1][nanoserde] | 267.30 µs | 1.9206 ms | † | 567975 | 239930 | 231872 | 2.4596 ms |
| [nibblecode 0.1.0][nibblecode] | 181.23 µs | † | † | 603928 | 430780 | 408322 | 3.6333 ms |
| [postcard 1.1.1][postcard] | 449.31 µs | 2.0728 ms | 820.67 µs | 367489 | 221913 | 207244 | 2.0360 ms |
| [pot 3.0.1][pot] | 2.4390 ms | 6.0871 ms | 5.0023 ms | 599125 | 299158 | 247675 | 2.6769 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*1.2669 ms\**</span> <span title="populate + encode">*2.9643 ms\**</span> | 3.5349 ms | † | 596811 | 305319 | 268737 | 2.9840 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0425 ms\**</span> <span title="populate + encode">*3.0025 ms\**</span> | 3.8515 ms | † | 596811 | 305319 | 268737 | 2.9928 ms |
| [rkyv 0.8.10][rkyv] | 330.50 µs | <span title="unvalidated">*1.4761 ms\**</span> <span title="validated upfront with error">*1.8408 ms\**</span> | † | 603776 | 254776 | 219421 | 2.6564 ms |
| [ron 0.10.1][ron] | 8.1390 ms | 24.488 ms | 24.944 ms | 1465223 | 434935 | 342907 | 5.6490 ms |
| [savefile 0.18.6][savefile] | 210.79 µs | 1.8254 ms | † | 566991 | 239362 | 231478 | 2.5412 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 627.98 µs | 2.1016 ms | † | 356311 | 212976 | 198423 | 1.9576 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3449 ms | 5.3123 ms | 3.6593 ms | 1276014 | 373898 | 293384 | 3.6178 ms |
| [serde_bare 0.5.0][serde_bare] | 766.50 µs | 2.3368 ms | † | 356311 | 213062 | 198403 | 2.4096 ms |
| [speedy 0.8.7][speedy] | 269.87 µs | 1.6694 ms | 559.21 µs | 449595 | 234970 | 210192 | 2.1008 ms |
| [wincode 0.2.0][wincode] | 199.01 µs | 1.6505 ms | 634.46 µs | 566975 | 239350 | 231475 | 2.5120 ms |
| [wiring 0.2.4][wiring] | 186.87 µs | 1.8355 ms | † | 566975 | 247810 | 225086 | 2.5204 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*82.786 ns\**</span> | <span title="validated on-demand with error">*423.84 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4850 ns\**</span> <span title="validated upfront with error">*2.3996 ms\**</span> | <span title="unvalidated">*1.4438 µs\**</span> <span title="validated upfront with error">*2.4042 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2436 ns\**</span> <span title="validated upfront with error">*261.51 µs\**</span> | <span title="unvalidated">*156.73 ns\**</span> <span title="validated upfront with error">*259.16 µs\**</span> | <span title="unvalidated">*724.49 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2430 ns\**</span> <span title="validated upfront with error">*348.28 µs\**</span> | <span title="unvalidated">*156.10 ns\**</span> <span title="validated upfront with error">*352.51 µs\**</span> | <span title="unvalidated">*753.08 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.37%\**</span> <span title="prepend">*16.40%\**</span> | 40.38% | 10.08% | 66.96% | 71.47% | 73.00% | 28.32% |
| [bincode 2.0.1][bincode] | 44.92% | 67.66% | 20.43% | 89.19% | 90.81% | 88.27% | 37.11% |
| [bincode 1.3.3][bincode1] | 22.64% | 68.94% | 19.42% | 57.49% | 83.55% | 78.50% | 30.51% |
| [bitcode 0.6.6][bitcode] | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 22.91% | 68.95% | † | 73.37% | 85.79% | 86.75% | 36.36% |
| [capnp 0.21.1][capnp] | 27.88% | † | † | 40.76% | 59.88% | 64.84% | 19.85% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 17.23% | 27.93% | 5.07% | 29.53% | 58.29% | 66.36% | 21.70% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.42% | 12.11% | † | 29.53% | 58.29% | 66.35% | 21.53% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.80% | 27.77% | 5.19% | 29.53% | 58.29% | 66.35% | 19.89% |
| [databuf 0.5.0][databuf] | 43.44% | 73.34% | 21.80% | 91.97% | 94.31% | 91.75% | 38.43% |
| [dlhn 0.1.7][dlhn] | 16.04% | 48.24% | † | 89.41% | 91.09% | 88.55% | 37.31% |
| [flatbuffers 25.2.10][flatbuffers] | 3.90% | † | † | 38.58% | 57.77% | 61.74% | 21.37% |
| [flexbuffers 25.2.10][flexbuffers] | 1.64% | 18.33% | 2.99% | 27.59% | 36.04% | 32.88% | 11.90% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.34% | 18.13% | † | 20.19% | 43.07% | 50.69% | 12.85% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.71% | 27.08% | † | 20.19% | 43.07% | 50.69% | 13.18% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 13.28% | 43.95% | † | 83.75% | 84.83% | 82.60% | 34.45% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.08% | 42.48% | 10.19% | 77.19% | 81.95% | 80.52% | 31.03% |
| [minicbor 1.0.0][minicbor] | 22.72% | 37.56% | 9.24% | 76.42% | 80.42% | 79.62% | 33.12% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.49% | 29.42% | 5.46% | 72.86% | 79.60% | 78.82% | 32.26% |
| [nanoserde 0.2.1][nanoserde] | 47.71% | 65.40% | † | 57.69% | 83.75% | 78.51% | 30.37% |
| [nibblecode 0.1.0][nibblecode] | 70.37% | † | † | 54.26% | 46.65% | 44.58% | 20.56% |
| [postcard 1.1.1][postcard] | 28.39% | 60.59% | 20.70% | 89.17% | 90.55% | 87.84% | 36.69% |
| [pot 3.0.1][pot] | 5.23% | 20.63% | 3.40% | 54.69% | 67.17% | 73.50% | 27.91% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*10.07%\**</span> <span title="populate + encode">*4.30%\**</span> | 35.53% | † | 54.91% | 65.82% | 67.74% | 25.03% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.23%\**</span> <span title="populate + encode">*4.25%\**</span> | 32.61% | † | 54.91% | 65.82% | 67.74% | 24.96% |
| [rkyv 0.8.10][rkyv] | 38.59% | <span title="unvalidated">*85.09%\**</span> <span title="validated upfront with error">*68.23%\**</span> | † | 54.27% | 78.87% | 82.96% | 28.12% |
| [ron 0.10.1][ron] | 1.57% | 5.13% | 0.68% | 22.36% | 46.20% | 53.09% | 13.22% |
| [savefile 0.18.6][savefile] | 60.51% | 68.81% | † | 57.79% | 83.95% | 78.64% | 29.40% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.31% | 59.76% | † | 91.97% | 94.35% | 91.74% | 38.16% |
| [serde-brief 0.1.1][serde-brief] | 9.48% | 23.64% | 4.64% | 25.68% | 53.74% | 62.05% | 20.65% |
| [serde_bare 0.5.0][serde_bare] | 16.64% | 53.75% | † | 91.97% | 94.31% | 91.75% | 31.00% |
| [speedy 0.8.7][speedy] | 47.26% | 75.24% | 30.37% | 72.89% | 85.52% | 86.61% | 35.56% |
| [wincode 0.2.0][wincode] | 64.09% | 76.10% | 26.77% | 57.80% | 83.96% | 78.64% | 29.74% |
| [wiring 0.2.4][wiring] | 68.25% | 68.43% | † | 57.80% | 81.09% | 80.88% | 29.64% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.50%\**</span> | <span title="validated on-demand with error">*36.83%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.81%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.95%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.60%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.04%\**</span> | <span title="unvalidated">*96.20%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5372 ms\**</span> <span title="prepend">*2.6528 ms\**</span> | 9.1431 ms | 1704643 | 1294259 | 1245668 | 11.918 ms |
| [bincode 2.0.1][bincode] | 1.3673 ms | 3.8618 ms | 1406257 | 1117802 | 1062438 | 10.527 ms |
| [bincode 1.3.3][bincode1] | 3.9155 ms | 4.1499 ms | 1854234 | 1141994 | 1048745 | 10.690 ms |
| [bitcode 0.6.6][bitcode] | 741.21 µs | 2.3724 ms | 971318 | 878034 | 850340 | 3.1346 ms |
| [borsh 1.5.7][borsh] | 2.9468 ms | 2.7935 ms | 1521989 | 1108471 | 1038528 | 10.177 ms |
| [capnp 0.21.1][capnp] | 2.2837 ms | † | 2724288 | 1546992 | 1239111 | 14.636 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.0439 ms | 18.967 ms | 6012539 | 1695215 | 1464951 | 21.463 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.263 ms | 56.162 ms | 6012373 | 1695146 | 1465025 | 21.614 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 10.307 ms | 20.740 ms | 6012373 | 1695146 | 1465025 | 21.096 ms |
| [databuf 0.5.0][databuf] | 1.3098 ms | 3.8127 ms | 1319999 | 1062631 | 1008334 | 8.8532 ms |
| [dlhn 0.1.7][dlhn] | 5.0989 ms | 6.7657 ms | 1311281 | 1077520 | 1046095 | 8.7341 ms |
| [flatbuffers 25.2.10][flatbuffers] | 5.2203 ms | † | 2325620 | 1439185 | 1268060 | 14.141 ms |
| [flexbuffers 25.2.10][flexbuffers] | 39.724 ms | 37.050 ms | 5352680 | 2658295 | 2777967 | 34.737 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 21.402 ms | 31.057 ms | 9390461 | 2391679 | 1842767 | 34.162 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.694 ms | 26.442 ms | 9390461 | 2391679 | 1842767 | 34.617 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.2365 ms | 6.2660 ms | 1458773 | 1156055 | 1137788 | 9.7907 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.8499 ms | 10.827 ms | 1745322 | 1261627 | 1228923 | 11.510 ms |
| [minicbor 1.0.0][minicbor] | 2.4483 ms | 11.403 ms | 1777386 | 1276218 | 1252558 | 12.798 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.363 ms | 20.590 ms | 1770060 | 1277755 | 1263362 | 12.491 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2385 ms | 2.8525 ms | 1812404 | 1134820 | 1053109 | 10.338 ms |
| [nibblecode 0.1.0][nibblecode] | 504.66 µs | † | 2075936 | 1518360 | 1412994 | 13.977 ms |
| [postcard 1.1.1][postcard] | 1.8885 ms | 4.1916 ms | 1311281 | 1083900 | 1041434 | 8.6405 ms |
| [pot 3.0.1][pot] | 14.575 ms | 29.829 ms | 2604812 | 1482233 | 1298928 | 16.140 ms |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*5.4203 ms\**</span> <span title="populate + encode">*9.5479 ms\**</span> | 9.0805 ms | 1859886 | 1338076 | 1295351 | 12.432 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.3331 ms\**</span> <span title="populate + encode">*12.570 ms\**</span> | 12.016 ms | 1859886 | 1338076 | 1295351 | 12.320 ms |
| [rkyv 0.8.10][rkyv] | 992.16 µs | <span title="unvalidated">*2.1662 ms\**</span> <span title="validated upfront with error">*2.5942 ms\**</span> | 2075936 | 1383779 | 1210377 | 12.869 ms |
| [ron 0.10.1][ron] | 42.154 ms | 160.23 ms | 8677703 | 2233642 | 1826180 | 34.615 ms |
| [savefile 0.18.6][savefile] | 857.37 µs | 2.7423 ms | 1791505 | 1128012 | 1051153 | 10.392 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1473 ms | 3.3562 ms | 1319999 | 1064380 | 1010708 | 9.0072 ms |
| [serde-brief 0.1.1][serde-brief] | 6.7733 ms | 20.985 ms | 6951772 | 1796265 | 1567819 | 24.286 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9081 ms | 4.7868 ms | 1319999 | 1062645 | 1008349 | 8.9214 ms |
| [speedy 0.8.7][speedy] | 740.70 µs | 2.4638 ms | 1584734 | 1119837 | 1037992 | 10.033 ms |
| [wincode 0.2.0][wincode] | 519.77 µs | 2.4382 ms | 1791489 | 1127998 | 1051146 | 10.192 ms |
| [wiring 0.2.4][wiring] | 671.65 µs | 2.7830 ms | 1791489 | 1156963 | 1082815 | 10.790 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*82.013 ns\**</span> | <span title="validated on-demand with error">*727.11 ns\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*2.4858 ns\**</span> <span title="validated upfront with error">*5.5272 ms\**</span> | <span title="unvalidated">*2.7397 µs\**</span> <span title="validated upfront with error">*5.5380 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2435 ns\**</span> <span title="validated upfront with error">*340.33 µs\**</span> | <span title="unvalidated">*378.61 ns\**</span> <span title="validated upfront with error">*340.62 µs\**</span> | <span title="unvalidated">*237.88 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2431 ns\**</span> <span title="validated upfront with error">*426.08 µs\**</span> | <span title="unvalidated">*399.51 ns\**</span> <span title="validated upfront with error">*424.40 µs\**</span> | <span title="unvalidated">*235.87 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.12%\**</span> <span title="prepend">*19.02%\**</span> | 23.69% | 56.98% | 67.84% | 68.26% | 26.30% |
| [bincode 2.0.1][bincode] | 36.91% | 56.09% | 69.07% | 78.55% | 80.04% | 29.78% |
| [bincode 1.3.3][bincode1] | 12.89% | 52.20% | 52.38% | 76.89% | 81.08% | 29.32% |
| [bitcode 0.6.6][bitcode] | 68.09% | 91.31% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 17.13% | 77.54% | 63.82% | 79.21% | 81.88% | 30.80% |
| [capnp 0.21.1][capnp] | 22.10% | † | 35.65% | 56.76% | 68.63% | 21.42% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.58% | 11.42% | 16.15% | 51.79% | 58.05% | 14.60% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.17% | 3.86% | 16.16% | 51.80% | 58.04% | 14.50% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 4.90% | 10.44% | 16.16% | 51.80% | 58.04% | 14.86% |
| [databuf 0.5.0][databuf] | 38.53% | 56.82% | 73.58% | 82.63% | 84.33% | 35.41% |
| [dlhn 0.1.7][dlhn] | 9.90% | 32.02% | 74.07% | 81.49% | 81.29% | 35.89% |
| [flatbuffers 25.2.10][flatbuffers] | 9.67% | † | 41.77% | 61.01% | 67.06% | 22.17% |
| [flexbuffers 25.2.10][flexbuffers] | 1.27% | 5.85% | 18.15% | 33.03% | 30.61% | 9.02% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.36% | 6.97% | 10.34% | 36.71% | 46.14% | 9.18% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.32% | 8.19% | 10.34% | 36.71% | 46.14% | 9.06% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 22.56% | 34.57% | 66.58% | 75.95% | 74.74% | 32.02% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 5.12% | 20.01% | 55.65% | 69.60% | 69.19% | 27.23% |
| [minicbor 1.0.0][minicbor] | 20.61% | 19.00% | 54.65% | 68.80% | 67.89% | 24.49% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.72% | 10.52% | 54.87% | 68.72% | 67.31% | 25.09% |
| [nanoserde 0.2.1][nanoserde] | 40.75% | 75.94% | 53.59% | 77.37% | 80.75% | 30.32% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 57.83% | 60.18% | 22.43% |
| [postcard 1.1.1][postcard] | 26.72% | 51.68% | 74.07% | 81.01% | 81.65% | 36.28% |
| [pot 3.0.1][pot] | 3.46% | 7.26% | 37.29% | 59.24% | 65.46% | 19.42% |
| protobuf:<br> [prost 0.13.5][prost] | <span title="encode">*9.31%\**</span> <span title="populate + encode">*5.29%\**</span> | 23.86% | 52.22% | 65.62% | 65.65% | 25.21% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*9.46%\**</span> <span title="populate + encode">*4.01%\**</span> | 18.03% | 52.22% | 65.62% | 65.65% | 25.44% |
| [rkyv 0.8.10][rkyv] | 50.86% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*83.50%\**</span> | 46.79% | 63.45% | 70.25% | 24.36% |
| [ron 0.10.1][ron] | 1.20% | 1.35% | 11.19% | 39.31% | 46.56% | 9.06% |
| [savefile 0.18.6][savefile] | 58.86% | 78.99% | 54.22% | 77.84% | 80.90% | 30.16% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.03% | 64.54% | 73.58% | 82.49% | 84.13% | 34.80% |
| [serde-brief 0.1.1][serde-brief] | 7.45% | 10.32% | 13.97% | 48.88% | 54.24% | 12.91% |
| [serde_bare 0.5.0][serde_bare] | 10.28% | 45.25% | 73.58% | 82.63% | 84.33% | 35.14% |
| [speedy 0.8.7][speedy] | 68.13% | 87.92% | 61.29% | 78.41% | 81.92% | 31.24% |
| [wincode 0.2.0][wincode] | 97.09% | 88.84% | 54.22% | 77.84% | 80.90% | 30.75% |
| [wiring 0.2.4][wiring] | 75.14% | 77.84% | 54.22% | 75.89% | 78.53% | 29.05% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.21.1][capnp] | <span title="validated on-demand with error">*1.52%\**</span> | <span title="validated on-demand with error">*52.07%\**</span> | ‡ |
| [flatbuffers 25.2.10][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.82%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*99.16%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*94.77%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

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
[wincode]: https://crates.io/crates/wincode/0.2.0
[wiring]: https://crates.io/crates/wiring/0.2.4


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
