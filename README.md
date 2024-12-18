# heimdall-nft-server

## setup

1. start the ssh-agent with `eval "$(ssh-agent -s)"`
2. add your key to the agent with `ssh-agent PATH-TO-PRIVATE_KEY`
3. start the server with `cargo run --release`