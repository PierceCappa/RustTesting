FROM fedora:latest

WORKDIR /Workspace

RUN command sudo dnf install -y git gcc-c++ make cmake python3 python3-pip wget unzip rust cargo