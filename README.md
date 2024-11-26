# Kindle Server

A small project to make handling images on a jailbroken Kindle easier.

This includes a simple web server and UI for displaying and managing images in `kindle_server/`, a CLI tool in `kindle_cli/` as well as an underlying library for managing the Kindle in `kindle_manager/`.

To run this project you need a jailbroken Kindle, configured to allow SSH connections (over wifi or usb). For more information check out the [MobileRead Forums](https://www.mobileread.com/forums/showthread.php?t=320564). This project was only tested with a PW2 Kindle.

![Kindle Server UI](https://github.com/user-attachments/assets/6dfc2c4f-db49-4109-ab05-ee4b71e561e6)

---

You will also need a computer acting as a server, here's how I run the docker container:

```bash
docker build . -t kindle_server
docker run -d -p 8000:8000 -v $SSH_AUTH_SOCK:/ssh-agent -e SSH_AUTH_SOCK=/ssh-agent -v ~/.ssh/id_ed25519:/root/.ssh/id_ed25519:ro -v ~/.ssh/known_hosts:/root/.ssh/known_hosts:ro --rm --name kindle kindle_server
```

**Note:** This will copy your `id_ed25519` key to the container, and forward your ssh-agent to avoid having to input a passphrase inside the container as well, adapt the command to fit your needs. Depending on how your server is set-up you might need to manually login into it and unlock the ssh key before running the server.

Make sure to run `cargo run -p kindle_cli -- prep` to avoid having the kindle fall asleep.

--- 

For development, inside the `kindle_server` folder:

To re-build tailwind stylesheets after changes (Requires Tailwind CLI):
```bash
./tailwind -i ./kindle_server/static/res/style.css -o ./kindle_server/static/res/tw.css --watch
```

To run the server:
```bash
cargo run -p kindle_server
```

To run the CLI:
```bash
cargo run -p kindle_cli -- help
```

To constantly re-run the server after changes:
```bash
cargo-watch -q -c -x 'run -p kindle_server'
```

Thanks to the following projects for the inspiration and in helping to understand how to deal with Kindle devices:

- https://github.com/forestpurnell/kindletron
- https://github.com/mattzzw/kindle-clock
