# Aboard
Get inside a container as current user to avoid wrecking file permissions.

# Introduction
Docker is a nice way to create a local development environment, especially with mounted volumes. However, since commands in Docker are run as root by default, any file created during a shell session with `docker exec` will be owned by root. This can be annoying with database migration scripts and code generators.

# Installation

The first linux pre-release is available here with instructions:

https://github.com/antoineMoPa/Aboard/releases/tag/0.1.0

# Examples

List containers:

    docker ps
    # CONTAINER ID        IMAGE [...]
    # 9394848b            frontend
    # 0929084a            backend

Go inside frontend:

    aboard frontend
    # run commands in frontend

Go inside backend, with container id instead of image name:

    aboard 0929084a
    # run commands in backend
    
Type `CTRL+D` or `exit` to exit.
