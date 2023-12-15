## Aoraki-labs Decentralized contract demo relayer

### Build

Run `cargo build --release` to build the binary.
And then, 
`cp ./target/release/zkpool-demo-relayer .`

### Run

Run like this:
```
	./zkpool-demo-relayer -a xxxxxx -k xxxxxx -s xxxxxx -b xxxxxx

```
You can also refer to the usage help (`./zkpool-demo-relayer -h`) or app.yml(under ./src/ directory)
```
    -a, --api <api>                Set the self server api endpoint [default: 0.0.0.0:6789]
    -k, --key <key>                Set the private key to sign the blockchain request [default:xx]
    -s, --scheduler <scheduler>    The scheduler rpc endpoint [default: http://35.234.20.15:8786/aleo-new-task]
    -b, --start_num <start_num>    The start block num when start relayer [default: 0]
```




