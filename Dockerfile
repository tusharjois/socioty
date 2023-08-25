FROM ubuntu:23.04

LABEL maintainer="lkostic1@jh.edu" 
RUN apt-get -y update 
RUN apt-get install -y \ 
  vim \
  build-essential \
  curl \
  keychain \
  git \
  python3 \
  python3-numpy \
  python3-matplotlib \
  pip \
  gcc-arm-linux-gnueabihf
# pip install virtualenv

RUN apt-get update

# Things to create the 
COPY gen_ssh_file.sh .
COPY ssh_pi.configs .
RUN chmod +x ./gen_ssh_file.sh
RUN mkdir ~/.ssh && ./gen_ssh_file.sh
RUN ssh-keygen -f ~/.ssh/socioty_nodes -t rsa
#RUN cat ~/.ssh/socioty_nodes.pub 

## Setting up rust environment
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN /bin/bash -c "rustup target add armv7-unknown-linux-musleabihf"
RUN /bin/bash -c "rustup target add arm-unknown-linux-musleabihf"

## Moving files over
WORKDIR socioty/
COPY .cargo/ .cargo/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src/ src/
COPY implementations/ implementations/
COPY benches/ benches

### BUILD LOCAL BINARIES ###
RUN cargo build --release -p socioty_coap
RUN cargo build --release -p socioty_mqtt
### BUILD PI BINARIES ###
### microbenchmarks ###
RUN cargo build --release -p socioty_benchmark --target arm-unknown-linux-musleabihf
RUN cargo build --release -p socioty_benchmark --target armv7-unknown-linux-musleabihf
### coap ###
RUN cargo build --release -p socioty_coap --target arm-unknown-linux-musleabihf
RUN cargo build --release -p socioty_coap --target armv7-unknown-linux-musleabihf
### mqtt ###
#RUN cargo build --release -p socioty_mqtt --target arm-unknown-linux-musleabihf
#RUN cargo build --release -p socioty_mqtt --target armv7-unknown-linux-musleabihf

##WORKDIR implementations/
#WORKDIR benches/
#RUN /bin/bash -c "chmod +x ./microbenchmark_deployment"
#RUN /bin/bash -c "/bin/bash ./microbenchmark_deployment build none"
#RUN chmod +x ./microbenchmark_deployment
#RUN ./microbenchmark_deployment build none



#
##CMD ["/usr/sbin/nginx", "-g", "daemon off;"]
