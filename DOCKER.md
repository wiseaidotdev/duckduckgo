# Docker Usage Guide

`duckduckgo` provides a heavily optimized Docker container based on Alpine Linux.

## Running the Container

The default entrypoint is set to the CLI (`ddg`).

```sh
# Display help menu
docker run -it wiseaidev/ddg --help

# Perform a text search
docker run -it wiseaidev/ddg --query "rust programming language"
```

## Pulling the Image

You can pull the official pre-built image from Docker Hub:

```sh
docker pull wiseaidev/ddg:latest
```

## Building the Container Locally

Install the [docker buildx plugin](https://docs.docker.com/build/concepts/overview/):

```sh
sudo apt-get update
sudo apt-get install docker-buildx-plugin
```

Once installed, you can use BuildKit natively using:

```sh
docker buildx build -t local/ddg .
docker run -it local/ddg --help
```

Enter TUI mode:

```sh
docker run -it local/ddg
```

You can alias the command for convenience in your shell profile (e.g. `~/.bashrc` or `~/.zshrc`):

```sh
alias ddg="docker run -it wiseaidev/ddg"
```
