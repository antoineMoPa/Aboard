# Aboard
Get inside a container as current user to avoid wrecking file permissions.

# Introduction
Docker is a nice way to create a local development environment, especially with mounted volumes. However, since commands in Docker are run as root by default, any file created during a shell session with `docker exec` will be owned by root. This can be annoying with database migration scripts and code generators.

# Installation

TODO

# Examples

    docker ps
    # CONTAINER ID        IMAGE [...]
    # 9394848b            frontend
    # 0929084a            backend

    aboard frontend
    # run commands in frontend
    
    aboard 0929084a
    # run commands in backend
    
 Type `CTRL+D` or `exit` to exit.
