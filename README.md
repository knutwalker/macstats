[![crates.io](https://img.shields.io/crates/v/macstats.svg)](https://crates.io/crates/macstats)

# macstats

SMC command line util for macOS

Read cpu temperatures and fan speeds from macOS SMC

## Examples

```bash,ignore
> macstats
... a lot of output

> macstats all
... even more output

> macstats temp
... only temperatures

> macstats cpu
... only CPU temperatures

> macstats gpu
... only GPU temperatures

> macstats other
... only other temperatures

> macstats fan
... only fan speeds

> macstats battery
... only battery info

> macstats power
... only power info

> macstats debug
... dump all knwon symbols
```

The functionality is provided by [`macsmc`](https://docs.rs/macsmc/*/macsmc/).

## Build

```
make
make install
```

This will put the binary into `/usr/local/bin/`, which can be changed with `$PREFIX`,
e.g. `PREFIX=/opt make install` to put it in `/opt/bin`.


License: MIT OR Apache-2.0
