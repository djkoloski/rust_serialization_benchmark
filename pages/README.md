# Rust Serialization Benchmark Interactive Site

## Inputs

* Bandwidth: in terabytes per month. 1 TB/Mo is 0.38 megabytes per second or 3.04 megabits per second
* CPU: fraction of CPU benchmarks were run on available for use (if > 1 assumes 0 overhead for parallelization)
* Dataset: (see ../README.md) changes messages/s to e.g. logs/s
  * log: logs (benchmark size divided by 10000, equal to individual logs in benchmark)
  * mesh: meshes (benchmark size)
  * minecraft_savedata: saves (benchmark size divided by 500, equal to individual player saves in benchmark)
  * mk48: updates (benchmark size divided by 1000, equal to individual updates in benchmark)
* Mode:
  * serialize: Bandwidth usage is size of compressed data, CPU usage is serialization + compression
  * deserialize: Bandwidth usage is size of compressed data, CPU usage is decompression + deserialization
  * round trip: Bandwidth/CPU usage is sum of Mode serialize and Mode deserialize
* zlib: allow using zlib as Compression
* zstd: allow using zstd as Compression

## Outputs

* Crate: which crate is being used for serialization/deserialization
* Compression: which compression algorithm is deemed the best (most messages/s) for that crate
* messages/s: how many messages could theoretically be sent per second based on available Bandwidth/CPU consumed by compressed data/serialization + compression
* Relative: normalized messages/s
* Bottleneck: whether Bandwidth or CPU runs out first (limiting messages/s)

## Assumptions

* zlib/zstd have a constant speed irrelevant of Dataset (hopefully we can fix this)
* 1 message of size 1000 takes the same Bandwidth/CPU as 1000 messages of size 1
* The amount of messages that need to be sent per second is constant (if each day you had all of your messages in a 1-hour interval, your real CPU requirement would be 24x)
