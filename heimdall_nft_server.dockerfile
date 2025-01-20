FROM rust:latest

COPY ./ ./
COPY ./.ssh /root/.ssh

WORKDIR "/"

RUN apt-get update && apt-get install -y apt-transport-https 
RUN apt-get install -y build-essential \
    aptitude \
    git
RUN git clone https://github.com/CGuthmann/heimdall-libs.git /usr/lib/heimdall-libs

RUN mv ./sfnft-libs  /usr/lib/

RUN echo "[net]\ngit-fetch-with-cli = true" >> /usr/local/cargo/config.toml
#RUN echo "[net.ssh]\nknown-hosts = [\"$(ssh-keyscan -t ecdsa-sha2-nistp256 gitlab.cc-asp.fraunhofer.de)\"]"   >> /usr/local/cargo/config.toml
RUN echo "$(ssh-keyscan -t rsa gitlab.cc-asp.fraunhofer.de)" >> /root/.ssh/known_hosts
RUN echo "Host *\nAddKeysToAgent yes\nStrictHostKeyChecking no\nIdentityFile /root/.ssh/id_docker" >> /root/.ssh/config
#
RUN eval "$(ssh-agent -s)" && ssh-add /root/.ssh/id_docker
##
RUN cargo update

