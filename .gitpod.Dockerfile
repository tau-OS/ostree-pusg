FROM registry.fedoraproject.org/fedora:latest

USER root

RUN useradd -l -u 33333 -G wheel -md /home/gitpod -s /bin/bash -p gitpod gitpod \
    && sed -i.bkp -e 's/%wheel\s\+ALL=(ALL\(:ALL\)\?)\s\+ALL/%wheel ALL=NOPASSWD:ALL/g' /etc/sudoers

RUN dnf install -y dnf-plugins-core git-core ostree ostree-devel openssl-devel make automake gcc gcc-c++ kernel-devel \
    && dnf clean all \
    && rm -rf /var/cache/yum

USER gitpod

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH