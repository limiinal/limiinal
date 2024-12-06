FROM ubuntu:24.10

RUN apt-get update
RUN apt-get install -y openssh-server build-essential netcat-traditional strace ltrace gcc gcc-multilib vim gdb python-is-python3 python3-pip python3-ropgadget make curl wget git cargo iputils-ping

RUN export PATH=$PATH:/root/.cargo/bin

RUN cargo install libp2p-lookup

RUN echo "root:root" | chpasswd
RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config

WORKDIR /root

COPY relay-server-example /root/relay-server-example

RUN chmod +x /root/relay-server-example

EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]\
