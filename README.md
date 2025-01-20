# heimdall-nft-server

## locale setup

1. start the ssh-agent with `eval "$(ssh-agent -s)"`
2. add your key to the agent with `ssh-add PATH-TO-PRIVATE_KEY`
3. start the server with `cargo run --release`


## docker-setup

1. Fetch submodules:  `git submodule init` then `git submodule update`
2. Add a ssh-key called `id_docker` to `.ssh`, which will be used to fetch the files from the Fraunhofer gitlab
3. Invoke `docker-compose up`