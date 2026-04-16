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

## Last updated: 2026-04-15 22:30:57

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.97.0-nightly (a5c825cd8 2026-04-14)
binary: rustc
commit-hash: a5c825cd824ee0ef9463021078a2f464b4cc1a0d
commit-date: 2026-04-14
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
BogoMIPS:                                4890.85
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
| [bilrost 0.1013.0][bilrost] | <span title="encode">*444.55 µs\**</span> <span title="prepend">*421.17 µs\**</span> | 2.5827 ms | 854.25 µs | 804955 | 328941 | 284849 | 4.1594 ms |
| [bin-proto 0.12.7][bin-proto] | 4.1144 ms | 4.7503 ms | † | 1045784 | 373127 | 311553 | 4.5306 ms |
| [bincode 2.0.1][bincode] | 333.11 µs | 2.1312 ms | 694.88 µs | 741295 | 303944 | 256422 | 3.6543 ms |
| [bincode 1.3.3][bincode1] | 550.23 µs | 2.1028 ms | 615.21 µs | 1045784 | 373127 | 311553 | 4.4915 ms |
| [bitcode 0.6.6][bitcode] | 146.01 µs | 1.4584 ms | 64.361 µs | 703710 | 288826 | 227322 | 2.5689 ms |
| [borsh 1.5.7][borsh] | 551.25 µs | 2.1423 ms | † | 885780 | 362204 | 286248 | 4.1450 ms |
| [capnp 0.23.2][capnp] | 464.55 µs | † | † | 1443216 | 513986 | 426532 | 6.1884 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 617.05 µs | 5.1121 ms | 3.5399 ms | 1407835 | 403440 | 323561 | 4.9923 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.1242 ms | 11.685 ms | † | 1407835 | 403440 | 323561 | 5.0589 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.9770 ms | 4.6680 ms | 3.0361 ms | 1407835 | 403440 | 323561 | 5.0385 ms |
| [columnar 0.11.1][columnar] | 261.14 µs | 2.1852 ms <span title="copy_from">*801.95 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.2238 ms |
| [databuf 0.5.0][databuf] | 266.96 µs | 2.0544 ms | 666.94 µs | 765778 | 311715 | 263914 | 3.5344 ms |
| [dlhn 0.1.7][dlhn] | 692.34 µs | 2.5638 ms | † | 724953 | 301446 | 253056 | 3.3241 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0370 ms | † | † | 1276368 | 468539 | 388381 | 4.8048 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.7394 ms | 7.4035 ms | 5.8222 ms | 1829756 | 714318 | 691541 | 8.7081 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.8886 ms | 4.0221 ms | † | 1827461 | 470560 | 360727 | 5.4841 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.9143 ms | 5.9774 ms | † | 1827461 | 470560 | 360727 | 6.0215 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1560 ms | 4.7202 ms | † | 1827461 | 470560 | 360727 | 6.0397 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 951.77 µs | 2.6467 ms | † | 764996 | 315291 | 264212 | 3.8614 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4001 ms | 3.1102 ms | 1.4783 ms | 784997 | 325384 | 277608 | 3.7923 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 363.47 µs | 2.4984 ms | 846.34 µs | 784997 | 325384 | 277608 | 3.8331 ms |
| [minicbor 1.0.0][minicbor] | 530.76 µs | 3.0344 ms | 1.3953 ms | 817830 | 332671 | 284034 | 4.1099 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3389 ms | 4.1232 ms | 2.5336 ms | 818669 | 332556 | 284797 | 4.3777 ms |
| [nanoserde 0.2.1][nanoserde] | 268.23 µs | 2.1162 ms | † | 1045784 | 373127 | 311553 | 4.3250 ms |
| [nibblecode 0.1.0][nibblecode] | 197.84 µs | † | † | 1011487 | 492874 | 427512 | 5.8925 ms |
| [postcard 1.1.1][postcard] | 428.22 µs | 2.2636 ms | 762.92 µs | 724953 | 302399 | 252968 | 3.5000 ms |
| [pot 3.0.1][pot] | 2.2940 ms | 6.3943 ms | 4.7691 ms | 971922 | 372513 | 303636 | 4.4881 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*940.29 µs\**</span> <span title="populate + encode">*2.5005 ms\**</span> | 3.4868 ms | † | 884628 | 363130 | 314959 | 4.4188 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.2236 ms\**</span> <span title="populate + encode">*3.0615 ms\**</span> | 3.9994 ms | † | 884628 | 363130 | 314959 | 4.5273 ms |
| [rkyv 0.8.10][rkyv] | 257.18 µs | <span title="unvalidated">*1.5636 ms\**</span> <span title="validated upfront with error">*1.9157 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.6059 ms |
| [ron 0.10.1][ron] | 11.097 ms | 23.525 ms | 21.741 ms | 1607459 | 449158 | 349324 | 5.5751 ms |
| [savefile 0.18.6][savefile] | 196.80 µs | 2.0740 ms | † | 1045800 | 373139 | 311562 | 4.2511 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 667.01 µs | 2.4465 ms | † | 765778 | 311743 | 263822 | 3.7826 ms |
| [serde-brief 0.1.1][serde-brief] | 1.4691 ms | 4.5810 ms | 2.8496 ms | 1584946 | 413733 | 339964 | 4.8939 ms |
| [serde_bare 0.5.0][serde_bare] | 704.39 µs | 2.0694 ms | † | 765778 | 311715 | 263914 | 3.5349 ms |
| [speedy 0.8.7][speedy] | 202.60 µs | 1.7680 ms | 370.70 µs | 885780 | 362204 | 286248 | 3.9517 ms |
| [wincode 0.2.4][wincode] | 186.56 µs | 1.9846 ms | 491.08 µs | 1045784 | 373127 | 311553 | 4.1907 ms |
| [wiring 0.2.4][wiring] | 202.12 µs | 2.1076 ms | † | 1045784 | 337930 | 275808 | 3.7846 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*82.153 ns\**</span> | <span title="validated on-demand with error">*136.11 µs\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 23.389 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4952 ns\**</span> <span title="validated upfront with error">*2.0474 ms\**</span> | <span title="unvalidated">*52.170 µs\**</span> <span title="validated upfront with error">*2.1010 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2443 ns\**</span> <span title="validated upfront with error">*240.11 µs\**</span> | <span title="unvalidated">*10.395 µs\**</span> <span title="validated upfront with error">*250.80 µs\**</span> | <span title="unvalidated">*7.4712 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*354.64 µs\**</span> | <span title="unvalidated">*10.358 µs\**</span> <span title="validated upfront with error">*365.03 µs\**</span> | <span title="unvalidated">*7.8219 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*32.84%\**</span> <span title="prepend">*34.67%\**</span> | 31.05% | 7.53% | 87.42% | 87.80% | 79.80% | 61.76% |
| [bin-proto 0.12.7][bin-proto] | 3.55% | 16.88% | † | 67.29% | 77.41% | 72.96% | 56.70% |
| [bincode 2.0.1][bincode] | 43.83% | 37.63% | 9.26% | 94.93% | 95.03% | 88.65% | 70.30% |
| [bincode 1.3.3][bincode1] | 26.54% | 38.14% | 10.46% | 67.29% | 77.41% | 72.96% | 57.19% |
| [bitcode 0.6.6][bitcode] | 100.00% | 54.99% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 26.49% | 37.43% | † | 79.45% | 79.74% | 79.41% | 61.98% |
| [capnp 0.23.2][capnp] | 31.43% | † | † | 48.76% | 56.19% | 53.30% | 41.51% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 23.66% | 15.69% | 1.82% | 49.99% | 71.59% | 70.26% | 51.46% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.67% | 6.86% | † | 49.99% | 71.59% | 70.26% | 50.78% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.39% | 17.18% | 2.12% | 49.99% | 71.59% | 70.26% | 50.99% |
| [columnar 0.11.1][columnar] | 55.91% | 36.70% <span title="copy_from">*100.00%\**</span> | † | 67.28% | 78.02% | 77.34% | 60.82% |
| [databuf 0.5.0][databuf] | 54.69% | 39.04% | 9.65% | 91.89% | 92.66% | 86.13% | 72.68% |
| [dlhn 0.1.7][dlhn] | 21.09% | 31.28% | † | 97.07% | 95.81% | 89.83% | 77.28% |
| [flatbuffers 25.12.19][flatbuffers] | 14.08% | † | † | 55.13% | 61.64% | 58.53% | 53.47% |
| [flexbuffers 25.2.10][flexbuffers] | 2.17% | 10.83% | 1.11% | 38.46% | 40.43% | 32.87% | 29.50% |
| json:<br> [flexon 0.4.5][flexon] | 5.05% | 19.94% | † | 38.51% | 61.38% | 63.02% | 46.84% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.73% | 13.42% | † | 38.51% | 61.38% | 63.02% | 42.66% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.77% | 16.99% | † | 38.51% | 61.38% | 63.02% | 42.53% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 15.34% | 30.30% | † | 91.99% | 91.61% | 86.04% | 66.53% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.43% | 25.78% | 4.35% | 89.64% | 88.76% | 81.89% | 67.74% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 40.17% | 32.10% | 7.60% | 89.64% | 88.76% | 81.89% | 67.02% |
| [minicbor 1.0.0][minicbor] | 27.51% | 26.43% | 4.61% | 86.05% | 86.82% | 80.03% | 62.51% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.73% | 19.45% | 2.54% | 85.96% | 86.85% | 79.82% | 58.68% |
| [nanoserde 0.2.1][nanoserde] | 54.43% | 37.90% | † | 67.29% | 77.41% | 72.96% | 59.40% |
| [nibblecode 0.1.0][nibblecode] | 73.80% | † | † | 69.57% | 58.60% | 53.17% | 43.60% |
| [postcard 1.1.1][postcard] | 34.10% | 35.43% | 8.44% | 97.07% | 95.51% | 89.86% | 73.40% |
| [pot 3.0.1][pot] | 6.36% | 12.54% | 1.35% | 72.40% | 77.53% | 74.87% | 57.24% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*15.53%\**</span> <span title="populate + encode">*5.84%\**</span> | 23.00% | † | 79.55% | 79.54% | 72.18% | 58.14% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.93%\**</span> <span title="populate + encode">*4.77%\**</span> | 20.05% | † | 79.55% | 79.54% | 72.18% | 56.74% |
| [rkyv 0.8.10][rkyv] | 56.77% | <span title="unvalidated">*51.29%\**</span> <span title="validated upfront with error">*41.86%\**</span> | † | 69.57% | 73.39% | 69.74% | 55.77% |
| [ron 0.10.1][ron] | 1.32% | 3.41% | 0.30% | 43.78% | 64.30% | 65.07% | 46.08% |
| [savefile 0.18.6][savefile] | 74.19% | 38.67% | † | 67.29% | 77.40% | 72.96% | 60.43% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 21.89% | 32.78% | † | 91.89% | 92.65% | 86.16% | 67.91% |
| [serde-brief 0.1.1][serde-brief] | 9.94% | 17.51% | 2.26% | 44.40% | 69.81% | 66.87% | 52.49% |
| [serde_bare 0.5.0][serde_bare] | 20.73% | 38.75% | † | 91.89% | 92.66% | 86.13% | 72.67% |
| [speedy 0.8.7][speedy] | 72.07% | 45.36% | 17.36% | 79.45% | 79.74% | 79.41% | 65.01% |
| [wincode 0.2.4][wincode] | 78.26% | 40.41% | 13.11% | 67.29% | 77.41% | 72.96% | 61.30% |
| [wiring 0.2.4][wiring] | 72.24% | 38.05% | † | 67.29% | 85.47% | 82.42% | 67.88% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.51%\**</span> | <span title="validated on-demand with error">*7.61%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.32% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.87%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*19.85%\**</span> <span title="validated upfront with error">*0.49%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.64%\**</span> <span title="validated upfront with error">*4.13%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.84%\**</span> | <span title="unvalidated">*95.52%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.3239 ms\**</span> <span title="prepend">*8.9992 ms\**</span> | 7.8653 ms | 8625005 | 6443961 | 6231572 | 77.720 ms |
| [bin-proto 0.12.7][bin-proto] | 8.2529 ms | 10.797 ms | 6000008 | 5378500 | 5346908 | 9.0451 ms |
| [bincode 2.0.1][bincode] | 2.8934 ms | 1.0848 ms | 6000005 | 5378497 | 5346882 | 9.0823 ms |
| [bincode 1.3.3][bincode1] | 5.8911 ms | 4.5439 ms | 6000008 | 5378500 | 5346908 | 8.7997 ms |
| [bitcode 0.6.6][bitcode] | 1.3614 ms | 822.48 µs | 6000006 | 5182295 | 4921841 | 14.763 ms |
| [borsh 1.5.7][borsh] | 5.8622 ms | 4.2733 ms | 6000004 | 5378496 | 5346866 | 10.597 ms |
| [capnp 0.23.2][capnp] | 6.5692 ms | † | 14000088 | 7130367 | 6046182 | 86.202 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 8.9775 ms | 50.551 ms | 13125016 | 7524114 | 6757437 | 99.128 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 64.632 ms | 127.32 ms | 13122324 | 7524660 | 6759128 | 99.316 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 36.012 ms | 40.796 ms | 13122324 | 7524660 | 6759128 | 92.568 ms |
| [columnar 0.11.1][columnar] | 1.8979 ms | 1.4736 ms <span title="copy_from">*686.89 µs\**</span> | 6000120 | 5378435 | 5347039 | 9.1115 ms |
| [databuf 0.5.0][databuf] | 2.4198 ms | 5.4109 ms | 6000003 | 5378495 | 5346897 | 9.0265 ms |
| [dlhn 0.1.7][dlhn] | 5.8063 ms | 7.0549 ms | 6000003 | 5378495 | 5346897 | 9.2181 ms |
| [flatbuffers 25.12.19][flatbuffers] | 521.62 µs | † | 6000024 | 5378434 | 5346878 | 9.4709 ms |
| [flexbuffers 25.2.10][flexbuffers] | 108.08 ms | 92.331 ms | 26609424 | 11901040 | 12486322 | 161.33 ms |
| json:<br> [flexon 0.4.5][flexon] | 76.569 ms | 58.098 ms | 26192883 | 9566084 | 8584671 | 165.21 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 88.510 ms | 101.37 ms | 26192883 | 9566084 | 8584671 | 157.09 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 54.040 ms | 67.460 ms | 26192883 | 9566084 | 8584671 | 160.54 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 3.1622 ms | 5.1479 ms | 7500005 | 6058442 | 6014500 | 10.774 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 18.388 ms | 16.540 ms | 8125006 | 6494876 | 6391037 | 74.685 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 734.12 µs | 5.2375 ms | 8125006 | 6494876 | 6391037 | 76.166 ms |
| [minicbor 1.0.0][minicbor] | 5.2054 ms | 11.579 ms | 8125006 | 6494907 | 6390894 | 72.385 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 120.34 ms | 28.929 ms | 8125037 | 6493484 | 6386940 | 77.429 ms |
| [nanoserde 0.2.1][nanoserde] | 1.6828 ms | 892.21 µs | 6000008 | 5378500 | 5346908 | 8.6604 ms |
| [nibblecode 0.1.0][nibblecode] | 153.44 µs | † | 6000008 | 5378500 | 5346908 | 9.0807 ms |
| [postcard 1.1.1][postcard] | 481.45 µs | 1.0251 ms | 6000003 | 5378495 | 5346897 | 9.0071 ms |
| [pot 3.0.1][pot] | 39.325 ms | 70.694 ms | 10122342 | 6814618 | 6852252 | 83.529 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*8.2060 ms\**</span> <span title="populate + encode">*8.8091 ms\**</span> | 11.745 ms | 8750000 | 6665735 | 6421877 | 74.331 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.534 ms\**</span> <span title="populate + encode">*31.208 ms\**</span> | 29.884 ms | 8750000 | 6665735 | 6421877 | 83.096 ms |
| [rkyv 0.8.10][rkyv] | 154.70 µs | <span title="unvalidated">*178.23 µs\**</span> <span title="validated upfront with error">*190.14 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.8361 ms |
| [ron 0.10.1][ron] | 163.38 ms | 506.33 ms | 22192885 | 8970395 | 8137334 | 154.00 ms |
| [savefile 0.18.6][savefile] | 197.18 µs | 198.54 µs | 6000024 | 5378519 | 5346896 | 8.8910 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 5.0508 ms | 4.2523 ms | 6000004 | 5378496 | 5346866 | 8.6683 ms |
| [serde-brief 0.1.1][serde-brief] | 17.798 ms | 36.597 ms | 15750015 | 8024540 | 6813667 | 94.887 ms |
| [serde_bare 0.5.0][serde_bare] | 6.0677 ms | 4.7647 ms | 6000003 | 5378495 | 5346897 | 9.0107 ms |
| [speedy 0.8.7][speedy] | 188.78 µs | 194.70 µs | 6000004 | 5378496 | 5346866 | 8.7089 ms |
| [wincode 0.2.4][wincode] | 201.02 µs | 200.98 µs | 6000008 | 5378500 | 5346908 | 8.7703 ms |
| [wiring 0.2.4][wiring] | 202.33 µs | 340.24 µs | 6000008 | 5378952 | 5346905 | 8.8720 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*110.94 ns\**</span> | <span title="validated on-demand with error">*2.0463 ms\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 21.928 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4899 ns\**</span> <span title="validated upfront with error">*46.133 ns\**</span> | <span title="unvalidated">*77.920 µs\**</span> <span title="validated upfront with error">*77.937 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*1.8684 ns\**</span> | <span title="unvalidated">*38.911 µs\**</span> <span title="validated upfront with error">*38.897 µs\**</span> | <span title="unvalidated">*100.56 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2587 ns\**</span> <span title="validated upfront with error">*5.3589 ns\**</span> | <span title="unvalidated">*39.127 µs\**</span> <span title="validated upfront with error">*38.983 µs\**</span> | <span title="unvalidated">*100.82 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.10%\**</span> <span title="prepend">*1.71%\**</span> | 2.27% | 69.57% | 80.42% | 78.98% | 11.14% |
| [bin-proto 0.12.7][bin-proto] | 1.86% | 1.65% | 100.00% | 96.35% | 92.05% | 95.75% |
| [bincode 2.0.1][bincode] | 5.30% | 16.43% | 100.00% | 96.35% | 92.05% | 95.35% |
| [bincode 1.3.3][bincode1] | 2.60% | 3.92% | 100.00% | 96.35% | 92.05% | 98.42% |
| [bitcode 0.6.6][bitcode] | 11.27% | 21.67% | 100.00% | 100.00% | 100.00% | 58.66% |
| [borsh 1.5.7][borsh] | 2.62% | 4.17% | 100.00% | 96.35% | 92.05% | 81.73% |
| [capnp 0.23.2][capnp] | 2.34% | † | 42.86% | 72.68% | 81.40% | 10.05% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.71% | 0.35% | 45.71% | 68.88% | 72.84% | 8.74% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.24% | 0.14% | 45.72% | 68.87% | 72.82% | 8.72% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.43% | 0.44% | 45.72% | 68.87% | 72.82% | 9.36% |
| [columnar 0.11.1][columnar] | 8.08% | 12.09% <span title="copy_from">*25.95%\**</span> | 100.00% | 96.35% | 92.05% | 95.05% |
| [databuf 0.5.0][databuf] | 6.34% | 3.29% | 100.00% | 96.35% | 92.05% | 95.94% |
| [dlhn 0.1.7][dlhn] | 2.64% | 2.53% | 100.00% | 96.35% | 92.05% | 93.95% |
| [flatbuffers 25.12.19][flatbuffers] | 29.42% | † | 100.00% | 96.35% | 92.05% | 91.44% |
| [flexbuffers 25.2.10][flexbuffers] | 0.14% | 0.19% | 22.55% | 43.54% | 39.42% | 5.37% |
| json:<br> [flexon 0.4.5][flexon] | 0.20% | 0.31% | 22.91% | 54.17% | 57.33% | 5.24% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.18% | 22.91% | 54.17% | 57.33% | 5.51% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.28% | 0.26% | 22.91% | 54.17% | 57.33% | 5.39% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 4.85% | 3.46% | 80.00% | 85.54% | 81.83% | 80.39% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.83% | 1.08% | 73.85% | 79.79% | 77.01% | 11.60% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 20.90% | 3.40% | 73.85% | 79.79% | 77.01% | 11.37% |
| [minicbor 1.0.0][minicbor] | 2.95% | 1.54% | 73.85% | 79.79% | 77.01% | 11.96% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.13% | 0.62% | 73.85% | 79.81% | 77.06% | 11.18% |
| [nanoserde 0.2.1][nanoserde] | 9.12% | 19.98% | 100.00% | 96.35% | 92.05% | 100.00% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 100.00% | 96.35% | 92.05% | 95.37% |
| [postcard 1.1.1][postcard] | 31.87% | 17.39% | 100.00% | 96.35% | 92.05% | 96.15% |
| [pot 3.0.1][pot] | 0.39% | 0.25% | 59.27% | 76.05% | 71.83% | 10.37% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.87%\**</span> <span title="populate + encode">*1.74%\**</span> | 1.52% | 68.57% | 77.75% | 76.64% | 11.65% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.06%\**</span> <span title="populate + encode">*0.49%\**</span> | 0.60% | 68.57% | 77.75% | 76.64% | 10.42% |
| [rkyv 0.8.10][rkyv] | 99.19% | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*93.74%\**</span> | 100.00% | 96.35% | 92.05% | 98.01% |
| [ron 0.10.1][ron] | 0.09% | 0.04% | 27.04% | 57.77% | 60.48% | 5.62% |
| [savefile 0.18.6][savefile] | 77.82% | 89.77% | 100.00% | 96.35% | 92.05% | 97.41% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.04% | 4.19% | 100.00% | 96.35% | 92.05% | 99.91% |
| [serde-brief 0.1.1][serde-brief] | 0.86% | 0.49% | 38.10% | 64.58% | 72.23% | 9.13% |
| [serde_bare 0.5.0][serde_bare] | 2.53% | 3.74% | 100.00% | 96.35% | 92.05% | 96.11% |
| [speedy 0.8.7][speedy] | 81.28% | 91.54% | 100.00% | 96.35% | 92.05% | 99.44% |
| [wincode 0.2.4][wincode] | 76.33% | 88.68% | 100.00% | 96.35% | 92.05% | 98.75% |
| [wiring 0.2.4][wiring] | 75.84% | 52.38% | 100.00% | 96.34% | 92.05% | 97.61% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.12%\**</span> | <span title="validated on-demand with error">*1.90%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.68% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*2.70%\**</span> | <span title="unvalidated">*49.92%\**</span> <span title="validated upfront with error">*49.91%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*66.62%\**</span> | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*100.00%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*98.89%\**</span> <span title="validated upfront with error">*23.23%\**</span> | <span title="unvalidated">*99.41%\**</span> <span title="validated upfront with error">*99.78%\**</span> | <span title="unvalidated">*99.74%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*879.32 µs\**</span> <span title="prepend">*794.71 µs\**</span> | 3.1188 ms | 1.6958 ms | 489348 | 281173 | 249360 | 2.6366 ms |
| [bin-proto 0.12.7][bin-proto] | 1.7523 ms | 2.7847 ms | † | 566975 | 239350 | 231475 | 2.4609 ms |
| [bincode 2.0.1][bincode] | 316.87 µs | 1.8841 ms | 780.27 µs | 367413 | 221291 | 206242 | 2.0371 ms |
| [bincode 1.3.3][bincode1] | 586.47 µs | 1.8658 ms | 861.29 µs | 569975 | 240525 | 231884 | 2.4883 ms |
| [bitcode 0.6.6][bitcode] | 123.27 µs | 1.3363 ms | 171.88 µs | 327688 | 200947 | 182040 | 748.90 µs |
| [borsh 1.5.7][borsh] | 558.42 µs | 1.7895 ms | † | 446595 | 234236 | 209834 | 2.0474 ms |
| [capnp 0.23.2][capnp] | 438.47 µs | † | † | 803896 | 335606 | 280744 | 3.5805 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 774.90 µs | 4.6478 ms | 3.3360 ms | 1109831 | 344745 | 274333 | 3.4857 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.6289 ms | 10.350 ms | † | 1109821 | 344751 | 274345 | 3.4530 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8145 ms | 4.5454 ms | 3.3403 ms | 1109821 | 344751 | 274345 | 3.4581 ms |
| [columnar 0.11.1][columnar] | 286.33 µs | 1.8789 ms <span title="copy_from">*752.45 µs\**</span> | † | 563728 | 249696 | 217582 | 1.5834 ms |
| [databuf 0.5.0][databuf] | 283.87 µs | 1.7482 ms | 780.86 µs | 356311 | 213062 | 198403 | 2.0187 ms |
| [dlhn 0.1.7][dlhn] | 707.58 µs | 2.6008 ms | † | 366496 | 220600 | 205586 | 2.1099 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.3265 ms | † | † | 849472 | 347816 | 294871 | 3.5749 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.9743 ms | 7.1364 ms | 5.9521 ms | 1187688 | 557642 | 553730 | 6.2966 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.7038 ms | 4.5722 ms | † | 1623191 | 466527 | 359157 | 5.9133 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6465 ms | 6.7949 ms | † | 1623191 | 466527 | 359157 | 5.6903 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2068 ms | 4.5962 ms | † | 1623191 | 466527 | 359157 | 5.7721 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 741.33 µs | 2.9038 ms | † | 391251 | 236877 | 220395 | 2.2139 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.5006 ms | 2.9958 ms | 1.7202 ms | 424533 | 245214 | 226077 | 2.2560 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 394.01 µs | 2.2562 ms | 934.04 µs | 416025 | 243812 | 224965 | 2.2463 ms |
| [minicbor 1.0.0][minicbor] | 561.05 µs | 3.4865 ms | 1.9237 ms | 428773 | 249857 | 228630 | 2.3639 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.2099 ms | 3.9566 ms | 2.7543 ms | 449745 | 252432 | 230965 | 2.4289 ms |
| [nanoserde 0.2.1][nanoserde] | 280.43 µs | 1.8685 ms | † | 567975 | 239930 | 231872 | 2.4751 ms |
| [nibblecode 0.1.0][nibblecode] | 180.22 µs | † | † | 603928 | 431235 | 408436 | 3.5993 ms |
| [postcard 1.1.1][postcard] | 444.04 µs | 2.0797 ms | 800.74 µs | 367489 | 221913 | 207244 | 2.0754 ms |
| [pot 3.0.1][pot] | 2.4182 ms | 6.1426 ms | 5.0282 ms | 599125 | 299158 | 247675 | 2.7481 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.2761 ms\**</span> <span title="populate + encode">*3.0071 ms\**</span> | 3.5972 ms | † | 596811 | 305319 | 268737 | 3.0204 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0510 ms\**</span> <span title="populate + encode">*3.0214 ms\**</span> | 3.9598 ms | † | 596811 | 305319 | 268737 | 3.0165 ms |
| [rkyv 0.8.10][rkyv] | 330.84 µs | <span title="unvalidated">*1.5092 ms\**</span> <span title="validated upfront with error">*1.8589 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3357 ms |
| [ron 0.10.1][ron] | 7.7978 ms | 24.138 ms | 22.769 ms | 1465223 | 434935 | 342907 | 5.5429 ms |
| [savefile 0.18.6][savefile] | 214.57 µs | 1.7998 ms | † | 566991 | 239362 | 231478 | 2.4638 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 621.04 µs | 2.0924 ms | † | 356311 | 212976 | 198423 | 1.9378 ms |
| [serde-brief 0.1.1][serde-brief] | 1.2448 ms | 5.3360 ms | 3.6183 ms | 1276014 | 373898 | 293384 | 3.6597 ms |
| [serde_bare 0.5.0][serde_bare] | 761.47 µs | 2.3436 ms | † | 356311 | 213062 | 198403 | 1.9415 ms |
| [speedy 0.8.7][speedy] | 267.07 µs | 1.6735 ms | 549.62 µs | 449595 | 234970 | 210192 | 2.0930 ms |
| [wincode 0.2.4][wincode] | 200.75 µs | 1.6747 ms | 635.51 µs | 566975 | 239350 | 231475 | 2.4273 ms |
| [wiring 0.2.4][wiring] | 211.06 µs | 1.9264 ms | † | 566975 | 247810 | 225086 | 2.5172 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*87.145 ns\**</span> | <span title="validated on-demand with error">*428.16 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 870.29 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.5114 ns\**</span> <span title="validated upfront with error">*2.2239 ms\**</span> | <span title="unvalidated">*1.4371 µs\**</span> <span title="validated upfront with error">*2.2295 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2446 ns\**</span> <span title="validated upfront with error">*258.77 µs\**</span> | <span title="unvalidated">*156.26 ns\**</span> <span title="validated upfront with error">*254.48 µs\**</span> | <span title="unvalidated">*751.90 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2470 ns\**</span> <span title="validated upfront with error">*330.69 µs\**</span> | <span title="unvalidated">*156.34 ns\**</span> <span title="validated upfront with error">*326.67 µs\**</span> | <span title="unvalidated">*750.23 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.02%\**</span> <span title="prepend">*15.51%\**</span> | 24.13% | 10.14% | 66.96% | 71.47% | 73.00% | 28.40% |
| [bin-proto 0.12.7][bin-proto] | 7.03% | 27.02% | † | 57.80% | 83.96% | 78.64% | 30.43% |
| [bincode 2.0.1][bincode] | 38.90% | 39.94% | 22.03% | 89.19% | 90.81% | 88.27% | 36.76% |
| [bincode 1.3.3][bincode1] | 21.02% | 40.33% | 19.96% | 57.49% | 83.55% | 78.50% | 30.10% |
| [bitcode 0.6.6][bitcode] | 100.00% | 56.31% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 22.07% | 42.05% | † | 73.37% | 85.79% | 86.75% | 36.58% |
| [capnp 0.23.2][capnp] | 28.11% | † | † | 40.76% | 59.88% | 64.84% | 20.92% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 15.91% | 16.19% | 5.15% | 29.53% | 58.29% | 66.36% | 21.48% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.40% | 7.27% | † | 29.53% | 58.29% | 66.35% | 21.69% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.79% | 16.55% | 5.15% | 29.53% | 58.29% | 66.35% | 21.66% |
| [columnar 0.11.1][columnar] | 43.05% | 40.05% <span title="copy_from">*100.00%\**</span> | † | 58.13% | 80.48% | 83.67% | 47.30% |
| [databuf 0.5.0][databuf] | 43.42% | 43.04% | 22.01% | 91.97% | 94.31% | 91.75% | 37.10% |
| [dlhn 0.1.7][dlhn] | 17.42% | 28.93% | † | 89.41% | 91.09% | 88.55% | 35.49% |
| [flatbuffers 25.12.19][flatbuffers] | 3.71% | † | † | 38.58% | 57.77% | 61.74% | 20.95% |
| [flexbuffers 25.2.10][flexbuffers] | 1.55% | 10.54% | 2.89% | 27.59% | 36.04% | 32.88% | 11.89% |
| json:<br> [flexon 0.4.5][flexon] | 4.56% | 16.46% | † | 20.19% | 43.07% | 50.69% | 12.66% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.38% | 11.07% | † | 20.19% | 43.07% | 50.69% | 13.16% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.59% | 16.37% | † | 20.19% | 43.07% | 50.69% | 12.97% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 16.63% | 25.91% | † | 83.75% | 84.83% | 82.60% | 33.83% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.21% | 25.12% | 9.99% | 77.19% | 81.95% | 80.52% | 33.20% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 31.29% | 33.35% | 18.40% | 78.77% | 82.42% | 80.92% | 33.34% |
| [minicbor 1.0.0][minicbor] | 21.97% | 21.58% | 8.93% | 76.42% | 80.42% | 79.62% | 31.68% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.37% | 19.02% | 6.24% | 72.86% | 79.60% | 78.82% | 30.83% |
| [nanoserde 0.2.1][nanoserde] | 43.96% | 40.27% | † | 57.69% | 83.75% | 78.51% | 30.26% |
| [nibblecode 0.1.0][nibblecode] | 68.40% | † | † | 54.26% | 46.60% | 44.57% | 20.81% |
| [postcard 1.1.1][postcard] | 27.76% | 36.18% | 21.47% | 89.17% | 90.55% | 87.84% | 36.08% |
| [pot 3.0.1][pot] | 5.10% | 12.25% | 3.42% | 54.69% | 67.17% | 73.50% | 27.25% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.66%\**</span> <span title="populate + encode">*4.10%\**</span> | 20.92% | † | 54.91% | 65.82% | 67.74% | 24.79% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.73%\**</span> <span title="populate + encode">*4.08%\**</span> | 19.00% | † | 54.91% | 65.82% | 67.74% | 24.83% |
| [rkyv 0.8.10][rkyv] | 37.26% | <span title="unvalidated">*49.86%\**</span> <span title="validated upfront with error">*40.48%\**</span> | † | 54.27% | 78.87% | 82.96% | 32.06% |
| [ron 0.10.1][ron] | 1.58% | 3.12% | 0.75% | 22.36% | 46.20% | 53.09% | 13.51% |
| [savefile 0.18.6][savefile] | 57.45% | 41.81% | † | 57.79% | 83.95% | 78.64% | 30.40% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 19.85% | 35.96% | † | 91.97% | 94.35% | 91.74% | 38.65% |
| [serde-brief 0.1.1][serde-brief] | 9.90% | 14.10% | 4.75% | 25.68% | 53.74% | 62.05% | 20.46% |
| [serde_bare 0.5.0][serde_bare] | 16.19% | 32.11% | † | 91.97% | 94.31% | 91.75% | 38.57% |
| [speedy 0.8.7][speedy] | 46.16% | 44.96% | 31.27% | 72.89% | 85.52% | 86.61% | 35.78% |
| [wincode 0.2.4][wincode] | 61.40% | 44.93% | 27.05% | 57.80% | 83.96% | 78.64% | 30.85% |
| [wiring 0.2.4][wiring] | 58.41% | 39.06% | † | 57.80% | 81.09% | 80.88% | 29.75% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.43%\**</span> | <span title="validated on-demand with error">*36.50%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 0.14% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.56%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.87%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*99.78%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.81%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.95%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5808 ms\**</span> <span title="prepend">*2.5574 ms\**</span> | 8.5113 ms | 1704643 | 1294259 | 1245668 | 11.723 ms |
| [bin-proto 0.12.7][bin-proto] | 5.1010 ms | 6.8134 ms | 1791489 | 1127998 | 1051146 | 10.481 ms |
| [bincode 2.0.1][bincode] | 1.4817 ms | 3.9245 ms | 1406257 | 1117802 | 1062438 | 9.5914 ms |
| [bincode 1.3.3][bincode1] | 3.8335 ms | 4.3966 ms | 1854234 | 1141994 | 1048745 | 10.638 ms |
| [bitcode 0.6.6][bitcode] | 683.38 µs | 2.3860 ms | 971318 | 878034 | 850340 | 2.9179 ms |
| [borsh 1.5.7][borsh] | 2.9076 ms | 2.8945 ms | 1521989 | 1108471 | 1038528 | 10.072 ms |
| [capnp 0.23.2][capnp] | 2.2178 ms | † | 2724288 | 1546992 | 1239111 | 14.494 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.2215 ms | 18.063 ms | 6012539 | 1695215 | 1464951 | 21.366 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.456 ms | 55.462 ms | 6012373 | 1695146 | 1465025 | 21.387 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.9002 ms | 20.579 ms | 6012373 | 1695146 | 1465025 | 21.411 ms |
| [columnar 0.11.1][columnar] | 898.53 µs | 3.7738 ms <span title="copy_from">*1.2105 ms\**</span> | 1544752 | 996728 | 897073 | 4.6659 ms |
| [databuf 0.5.0][databuf] | 1.3169 ms | 3.7753 ms | 1319999 | 1062631 | 1008334 | 8.8794 ms |
| [dlhn 0.1.7][dlhn] | 4.3486 ms | 7.3240 ms | 1311281 | 1077520 | 1046095 | 8.7425 ms |
| [flatbuffers 25.12.19][flatbuffers] | 4.8875 ms | † | 2325620 | 1439185 | 1268060 | 13.563 ms |
| [flexbuffers 25.2.10][flexbuffers] | 40.982 ms | 36.465 ms | 5352680 | 2658295 | 2777967 | 34.486 ms |
| json:<br> [flexon 0.4.5][flexon] | 14.954 ms | 23.979 ms | 9390461 | 2391679 | 1842767 | 35.075 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.759 ms | 31.584 ms | 9390461 | 2391679 | 1842767 | 35.885 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 11.816 ms | 25.522 ms | 9390461 | 2391679 | 1842767 | 35.787 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.7107 ms | 5.9622 ms | 1458773 | 1156055 | 1137788 | 9.7762 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.804 ms | 11.112 ms | 1745322 | 1261627 | 1228923 | 11.915 ms |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 1.8280 ms | 5.8471 ms | 1794467 | 1273669 | 1242301 | 11.839 ms |
| [minicbor 1.0.0][minicbor] | 2.2523 ms | 11.315 ms | 1777386 | 1276218 | 1252558 | 12.599 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 30.027 ms | 16.684 ms | 1770060 | 1277755 | 1263362 | 13.517 ms |
| [nanoserde 0.2.1][nanoserde] | 1.3020 ms | 2.8025 ms | 1812404 | 1134820 | 1053109 | 10.600 ms |
| [nibblecode 0.1.0][nibblecode] | 504.00 µs | † | 2075936 | 1503435 | 1396519 | 14.028 ms |
| [postcard 1.1.1][postcard] | 1.7888 ms | 4.1831 ms | 1311281 | 1083900 | 1041434 | 8.7286 ms |
| [pot 3.0.1][pot] | 14.159 ms | 30.306 ms | 2604812 | 1482233 | 1298928 | 16.279 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.4701 ms\**</span> <span title="populate + encode">*9.3887 ms\**</span> | 9.0814 ms | 1859886 | 1338076 | 1295351 | 12.302 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.5589 ms\**</span> <span title="populate + encode">*12.876 ms\**</span> | 12.281 ms | 1859886 | 1338076 | 1295351 | 12.276 ms |
| [rkyv 0.8.10][rkyv] | 992.65 µs | <span title="unvalidated">*2.2184 ms\**</span> <span title="validated upfront with error">*2.6362 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.239 ms |
| [ron 0.10.1][ron] | 42.286 ms | 148.36 ms | 8677703 | 2233642 | 1826180 | 35.441 ms |
| [savefile 0.18.6][savefile] | 880.31 µs | 2.5951 ms | 1791505 | 1128012 | 1051153 | 10.595 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1592 ms | 3.3163 ms | 1319999 | 1064380 | 1010708 | 9.2436 ms |
| [serde-brief 0.1.1][serde-brief] | 6.1706 ms | 21.444 ms | 6951772 | 1796265 | 1567819 | 23.562 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9671 ms | 5.0387 ms | 1319999 | 1062645 | 1008349 | 9.0885 ms |
| [speedy 0.8.7][speedy] | 754.25 µs | 2.4804 ms | 1584734 | 1119837 | 1037992 | 10.096 ms |
| [wincode 0.2.4][wincode] | 575.83 µs | 2.3763 ms | 1791489 | 1127998 | 1051146 | 10.552 ms |
| [wiring 0.2.4][wiring] | 644.30 µs | 2.7561 ms | 1791489 | 1156963 | 1082815 | 10.758 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*84.616 ns\**</span> | <span title="validated on-demand with error">*722.98 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 57.915 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4896 ns\**</span> <span title="validated upfront with error">*5.6477 ms\**</span> | <span title="unvalidated">*2.7423 µs\**</span> <span title="validated upfront with error">*5.6543 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2456 ns\**</span> <span title="validated upfront with error">*341.32 µs\**</span> | <span title="unvalidated">*377.32 ns\**</span> <span title="validated upfront with error">*341.88 µs\**</span> | <span title="unvalidated">*237.63 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2454 ns\**</span> <span title="validated upfront with error">*417.55 µs\**</span> | <span title="unvalidated">*385.73 ns\**</span> <span title="validated upfront with error">*417.96 µs\**</span> | <span title="unvalidated">*238.29 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.00%\**</span> <span title="prepend">*19.71%\**</span> | 14.22% | 56.98% | 67.84% | 68.26% | 24.89% |
| [bin-proto 0.12.7][bin-proto] | 9.88% | 17.77% | 54.22% | 77.84% | 80.90% | 27.84% |
| [bincode 2.0.1][bincode] | 34.01% | 30.84% | 69.07% | 78.55% | 80.04% | 30.42% |
| [bincode 1.3.3][bincode1] | 13.15% | 27.53% | 52.38% | 76.89% | 81.08% | 27.43% |
| [bitcode 0.6.6][bitcode] | 73.75% | 50.73% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 17.33% | 41.82% | 63.82% | 79.21% | 81.88% | 28.97% |
| [capnp 0.23.2][capnp] | 22.73% | † | 35.65% | 56.76% | 68.63% | 20.13% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 15.64% | 6.70% | 16.15% | 51.79% | 58.05% | 13.66% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.15% | 2.18% | 16.16% | 51.80% | 58.04% | 13.64% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.09% | 5.88% | 16.16% | 51.80% | 58.04% | 13.63% |
| [columnar 0.11.1][columnar] | 56.09% | 32.08% <span title="copy_from">*100.00%\**</span> | 62.88% | 88.09% | 94.79% | 62.54% |
| [databuf 0.5.0][databuf] | 38.27% | 32.06% | 73.58% | 82.63% | 84.33% | 32.86% |
| [dlhn 0.1.7][dlhn] | 11.59% | 16.53% | 74.07% | 81.49% | 81.29% | 33.38% |
| [flatbuffers 25.12.19][flatbuffers] | 10.31% | † | 41.77% | 61.01% | 67.06% | 21.51% |
| [flexbuffers 25.2.10][flexbuffers] | 1.23% | 3.32% | 18.15% | 33.03% | 30.61% | 8.46% |
| json:<br> [flexon 0.4.5][flexon] | 3.37% | 5.05% | 10.34% | 36.71% | 46.14% | 8.32% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.43% | 3.83% | 10.34% | 36.71% | 46.14% | 8.13% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.27% | 4.74% | 10.34% | 36.71% | 46.14% | 8.15% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 29.46% | 20.30% | 66.58% | 75.95% | 74.74% | 29.85% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 4.66% | 10.89% | 55.65% | 69.60% | 69.19% | 24.49% |
| messagepack:<br> [zerompk 0.3.2][zerompk] | 27.57% | 20.70% | 54.13% | 68.94% | 68.45% | 24.65% |
| [minicbor 1.0.0][minicbor] | 22.38% | 10.70% | 54.65% | 68.80% | 67.89% | 23.16% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.68% | 7.26% | 54.87% | 68.72% | 67.31% | 21.59% |
| [nanoserde 0.2.1][nanoserde] | 38.71% | 43.19% | 53.59% | 77.37% | 80.75% | 27.53% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 58.40% | 60.89% | 20.80% |
| [postcard 1.1.1][postcard] | 28.18% | 28.94% | 74.07% | 81.01% | 81.65% | 33.43% |
| [pot 3.0.1][pot] | 3.56% | 3.99% | 37.29% | 59.24% | 65.46% | 17.92% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.21%\**</span> <span title="populate + encode">*5.37%\**</span> | 13.33% | 52.22% | 65.62% | 65.65% | 23.72% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*9.07%\**</span> <span title="populate + encode">*3.91%\**</span> | 9.86% | 52.22% | 65.62% | 65.65% | 23.77% |
| [rkyv 0.8.10][rkyv] | 50.77% | <span title="unvalidated">*54.57%\**</span> <span title="validated upfront with error">*45.92%\**</span> | 46.79% | 63.45% | 70.25% | 22.04% |
| [ron 0.10.1][ron] | 1.19% | 0.82% | 11.19% | 39.31% | 46.56% | 8.23% |
| [savefile 0.18.6][savefile] | 57.25% | 46.65% | 54.22% | 77.84% | 80.90% | 27.54% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 15.95% | 36.50% | 73.58% | 82.49% | 84.13% | 31.57% |
| [serde-brief 0.1.1][serde-brief] | 8.17% | 5.64% | 13.97% | 48.88% | 54.24% | 12.38% |
| [serde_bare 0.5.0][serde_bare] | 10.15% | 24.02% | 73.58% | 82.63% | 84.33% | 32.11% |
| [speedy 0.8.7][speedy] | 66.82% | 48.80% | 61.29% | 78.41% | 81.92% | 28.90% |
| [wincode 0.2.4][wincode] | 87.53% | 50.94% | 54.22% | 77.84% | 80.90% | 27.65% |
| [wiring 0.2.4][wiring] | 78.22% | 43.92% | 54.22% | 75.89% | 78.53% | 27.12% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.47%\**</span> | <span title="validated on-demand with error">*52.19%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 2.15% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.02%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.76%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.11%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.82%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*99.72%\**</span> |

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
