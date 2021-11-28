# write CLI util imitation of HTTPie with Rust

## main function
1. The first is to do command line parsing, processing subcommands and various arguments, validating user input, and converting that input into arguments that we can understand internally
2. Then, send an HTTP request base on the parsed params and get the response
3. Finally, output the response in a user-friendly manner

## dependencies
- command line parsing --> [clap](https://github.com/clap-rs/clap)
- HTTP client --> [reqwest](https://github.com/seanmonstar/reqwest)
- The command line is displayed in color --> [colored](https://github.com/mackwic/colored)
- Error handling --> `anyhow`
- JSON format --> `jsonxf`
- Mime type handling --> `mime`
- async handling --> `tokio`

## how to use
`cargo build --release`
help: `httpie -h`
post: `httpie post https://httpbin.org/post a=1 b=2`