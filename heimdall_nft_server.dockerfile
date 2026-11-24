FROM rust:1.86

COPY ./ ./
COPY ./.ssh /root/.ssh

WORKDIR "/"

RUN apt-get update && apt-get install -y apt-transport-https 
RUN apt-get install -y build-essential \
    aptitude \
    git
RUN git clone https://github.com/CGuthmann/heimdall-libs.git /usr/lib/heimdall-libs

RUN mv ./sfnft-libs  /usr/lib/

##
RUN cargo update

