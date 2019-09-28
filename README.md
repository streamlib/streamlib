# Streamlib

Streamlib is a meta-player for video streams. The specification defines a standard for indexing video stream URLs, and the CLI enables quick searching of those streams. Once a stream is chosen for viewing, it is passed on to a standard media player, defaulting to [mpv](https://mpv.io/) which is currently the only supported player.

## Usage

Streamlib is still a work-in-progress. Running it requires checking out the git repository and then running:

```bash
$ cargo run -- groove
```

The single argument will run a case-insensitive match against all known metadata and pick the first matching stream. Upcoming versions will include a console-based interactive interface and better library management.

Entire library can be dumped with the `-L` flag:

```bash
$ cargo run -- -L
```

### Testing

```bash
$ cargo test
```

## Library Specification

Streamlib library files are nothing more than [TOML](https://github.com/toml-lang/toml) files which adhere to the following specification, given by example:

```toml
[groovesalad]
name = "Groove Salad"
description = "A nicely chilled plate of ambient/downtempo beats and grooves"
url = "http://somafm.com/groovesalad.pls"
tags = ["somafm", "radio", "ambient", "groove"]
http_header = "User-Agent: foo"
```

The only real requirement in a streamlib file is to have one or more top-level tables which at bare minimum includes a `url` key:

```toml
[secretagent]
url = "http://somafm.com/secretagent.pls"
```

All the other keys are used for indexing and querying upon playback, but are essentially optional.

Further fields might be added later on to the specification.

Currently all library TOML files are located under the [library](library) directory.
