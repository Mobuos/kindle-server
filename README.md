# Kindle Server

This a small project that aims to provide a simple web UI for displaying and managing images on a jailbroken Kindle.

![Kindle Server UI](https://github.com/user-attachments/assets/d0a9d76d-494c-45ba-b956-5575b51752ce)

> Website UI is currently going through a redesign

Here's how I run the docker container:


```bash
docker build . -t kindle_server
docker run -d -p 7070:8000 -v $SSH_AUTH_SOCK:/ssh-agent -e SSH_AUTH_SOCK=/ssh-agent -v ~/.ssh/id_ed25519:/root/.ssh/id_ed25519:ro -v ~/.ssh/known_hosts:/root/.ssh/known_hosts:ro --name kindle kindle_server
```

**Note:** This will copy your `id_ed25519` key to the container, and forward your ssh-agent to avoid having to input a passphrase inside the container as well, adapt the command to fit your needs. Depending on how your server is set-up you might need to manually login into it and unlock the ssh key before running the server.

---

For development:

```bash
./tailwind -i static/style.css -o static/tw.css --watch
```

```bash
cargo-watch -q -c -x 'run'
```
