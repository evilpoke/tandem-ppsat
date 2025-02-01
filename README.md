

Implementing the ppsat algorithm (https://github.com/PP-FM/ppsat) in garble 



1. Start the server in `tandem_http_server` with `cargo run --features="bin"` after building it with `cargo build --features="bin"`
2. Run a client request in `tandem_http_client` with:

```
tandem_http_client tests/program.garble.rs --function main --url http://localhost:8000/ --input "( [([false,true,true,false],[false,false,false,false]), ([false,true,false,true],[false,false,false,false]) , ([false,true,false,false],[false,false,false,true]) , ([true,false,false,false],[false,false,false,false])] , [2usize, 2usize, 2usize, 1usize])" --metadata "( [([false,true,true,false],[false,false,false,false]), ([false,true,false,true],[false,false,false,false]) , ([false,true,false,false],[false,false,false,true]) , ([true,false,false,false],[false,false,false,false])] , [2usize, 2usize, 2usize, 1usize])"
```

after building it with `cargo build --features="bin"`


Currently, this results in a rocket error

> Data limit reached while reading the request body.