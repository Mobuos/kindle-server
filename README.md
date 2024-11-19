# Kindle Server

A small project to make handling images on a jailbroken Kindle easier.

This includes a simple web server and UI for displaying and managing images in `kindle_server/`, a CLI tool in `kindle_cli/` as well as an underlying library for managing the Kindle in `kindle_manager/`.

To run this project you need a jailbroken Kindle, configured to allow SSH connections (over wifi or usb). For more information check out the [MobileRead Forums](https://www.mobileread.com/forums/showthread.php?t=320564). This project was only tested with a PW2 Kindle.

![Kindle Server UI](https://github.com/user-attachments/assets/d0a9d76d-494c-45ba-b956-5575b51752ce)

<!-- TODO: Adapt the Dockerfile to work with the newest changes -->
<!-- 
---

You will also need a computer acting as a server, here's how I run the docker container:

```bash
docker build kindle_server/ -t kindle_server
docker run -d -p 7070:8000 -v $SSH_AUTH_SOCK:/ssh-agent -e SSH_AUTH_SOCK=/ssh-agent -v ~/.ssh/id_ed25519:/root/.ssh/id_ed25519:ro -v ~/.ssh/known_hosts:/root/.ssh/known_hosts:ro --name kindle kindle_server
```

**Note:** This will copy your `id_ed25519` key to the container, and forward your ssh-agent to avoid having to input a passphrase inside the container as well, adapt the command to fit your needs. Depending on how your server is set-up you might need to manually login into it and unlock the ssh key before running the server.
-->
--- 

For development, inside the `kindle_server` folder:

To re-build tailwind stylesheets after changes:
```bash
./tailwind -i static/style.css -o static/tw.css --watch
```

To run the server:
```bash
cargo run -p kindle_server
```

To constantly re-run the server after changes:
```bash
cargo-watch -q -c -x 'run -p kindle_server'
```

Thanks to the following projects for the inspiration and in helping to understand how to deal with Kindle devices:

- https://github.com/forestpurnell/kindletron
- https://github.com/mattzzw/kindle-clock