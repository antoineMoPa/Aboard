# Aboard
Get inside docker containers quickly.

# Introduction
Aboard allows you to start a shell inside a container by name or by ID. Unlike docker exec, a user matching your username is created automatically so you can create files in mounted volumes and these files will still be accessible to your current user.

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

Go inside container, as root:

    aboard frontend --root

Type `CTRL+D` or `exit` to exit.
