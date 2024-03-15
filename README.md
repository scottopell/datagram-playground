Datagram Playground

## `rust-datagram-sender`

For each packet size requested, its filled with `BEGIN1234567890123...567  END`.

First 5 bytes will be `BEGIN` and last 5 bytes will be `  END`.

```
# Send a packet of length 65507 to port 7878
cargo run -- udp 7878 65507
```

## `rust-datagram-receiver`

Prints out the length of the datagram received and the first and last 5 bytes
(as ascii encoded text)

```
# Listen on port 7878 for datagrams
cargo run udp 7878
```
