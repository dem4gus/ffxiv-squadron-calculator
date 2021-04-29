# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.163.1/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

ENV DEBIAN_FRONTEND=noninteractive
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install -y nodejs --no-install-recommends
RUN npm install -g yarn
RUN yarn set version berry
ENV DEBIAN_FRONTEND=interactive
