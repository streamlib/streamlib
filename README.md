# Streamlib

Streamlib is a meta-player for video streams. The specification defines a standard for indexing video stream URLs, and the CLI enables quick searching of those streams. Once a stream is chosen for viewing, it is passed on to a standard stream player, defaulting to [mpv](https://mpv.io/).

## Specification

Streamlib files are [TOML](https://github.com/toml-lang/toml) files that adhere to the following specification, which is given by example:

```toml
[groovesalad]
name = "Groove Salad"
description = "A nicely chilled plate of ambient/downtempo beats and grooves"
url = "http://somafm.com/groovesalad.pls"
tags = ["somafm", "radio", "ambient", "groove"]
http_header = "User-Agent: foo"
```

The only real requirement in a Streamlib file is to have one or more top-level tables which at bare minimum includes a `url` key:

```toml
[secretagent]
url = "http://somafm.com/secretagent.pls"
```

All the other keys are used for indexing and querying upon playback, but are essentially optional. The top-level `title` string is used to hold metadata on the file itself.

Further fields might be added later on to the specification.

Currently all library TOML files are located under the [library](library) directory;

## CLI

The `streamlib` CLI tool is the main interface for users, allowing to search for various streams according to their metadata and pass their respective URLs to the media player.

## Dev

```bash
$ cargo build
$ cargo run
```
