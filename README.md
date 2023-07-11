# Shlonk - A simple, fast, URL shortener

[Shlink](https://shlink.io/) is another self-hosted URL shortener. Though it does too much: it has a REST API, a web GUI, and many other features you are probably never going to use.

Shlonk tries to do only one thing, redirect clients to other sites, and does it well with its Yaml configuration file. Here's an example:

```yaml
urls:
  home: # /home redirects to...
    url: https://git.renn.es/shlonk
  example: # /example redirects to...
    url: https://example.com/
    permanent: true
port: 8080 # Default value
address: 0.0.0.0 # Default value
```

To launch Shlonk, write this config to a file, say `config.yml`, and then run:
```bash
shlonk -c ./config.yml
```

That's it. That's the documentation for Shlonk. And guess what: the code is less than 200 lines long.

## Installation

Cargo:
```bash
cargo install shlonk
```

## Docker installation

See the example [docker-compose](https://github.com/tarneaux/shlonk/blob/main/docker-compose.yml).

## Why the name?

It is a variation on Shlink, and here is a definition from the [urban dictionary](https://www.urbandictionary.com/define.php?term=Shlonk):

> V. To do anything.
>
> Shlonking is a lifestyle choice, a way to express how much better you are than any person who doesn't shlonk the same way you do. Use it as a replacement for any verb in the hopes that more individuals realize the importance of gang shlonk.

I just had to choose that name.
