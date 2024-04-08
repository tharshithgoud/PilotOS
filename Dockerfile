# Use an Ubuntu 22.04 base image
FROM ubuntu:22.04

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install QEMU and other necessary tools
RUN apt-get update && apt-get install -y \
    qemu-system-x86 \
    build-essential \
    gdb \
    nasm \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /PilotOS

# Set the default command to open a bash shell
CMD ["/bin/bash"]
