# macstats

Read cpu temperatures and fan speeds from macOS SMC

## Build

```
make
make install
```

This will put the binary into `/usr/local/bin/`, which can be changed with `$PREFIX`,
e.g. `PREFIX=/opt make install` to put it in `/opt/bin`.
