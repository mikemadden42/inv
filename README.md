# inv

System inventory program written in rust

This leverages the excellent [sysinfo](https://github.com/GuillaumeGomez/sysinfo) crate.

MSRV: 1.88

## Example output

```
$ cargo run -q
system hostname: bill.local
system type: Darwin
system os: 27.2.1
system kernel: 27.2.0
logical CPUs: 8
physical cores: 8
total memory: 24.00 GiB
used memory : 14.21 GiB
total swap  : 0.00 GiB
used swap   : 0.00 GiB
temperatures:
  PMU2 tdie2 temperature: 33.755814°C (max: 33.755814°C / critical: unknown)
  ...
```
