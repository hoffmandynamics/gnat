# gnat
A tiny Rust HTTP static file server

## Compile
`RK1`
```
$ RUSTFLAGS='-C target-feature=+crt-static' cross build \
    --release --target armv7-unknown-linux-gnueabi
```

`TuringPi2 - AllWinner`
```
$ RUSTFLAGS='-C target-feature=+crt-static' cross build \
    --release --target armv7-unknown-linux-gnueabihf
```

## Copy to RK1 FIT Image
`Host`
```
$ cd target/armv7-unknown-linux-gnueabi/release
$ python3 -m http.server 8080
```

`RK1`
```
# :> gnat;wget http://<host_address>:8080/gnat;chmod +x gnat
# ./gnat
Now listening on 0.0.0.0:64000
```

## Gather files from RK1 FIT Image

Copy any files you would like hosted into the same path as the `gnat` binary.

`Host`
```
$ wget http://<rk1_address>:64000/<file>
```