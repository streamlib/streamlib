# Streamlib

Streamlib is a meta-player for video streams. The specification defines a standard for indexing video stream URLs, and the CLI enables quick searching of those streams. Once a stream is chosen for viewing, it is passed on to a standard stream player, defaulting to [mpv](https://mpv.io/).

## Usage

Streamlib is still a work-in-progress. Running it requires checking out the git repository and then running:

```bash
$ cargo run -- "Groove Salad"
```

Upcoming versions will include a console-based interactive interface and better library management.

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

The only real requirement in a Streamlib file is to have one or more top-level tables which at bare minimum includes a `url` key:

```toml
[secretagent]
url = "http://somafm.com/secretagent.pls"
```

All the other keys are used for indexing and querying upon playback, but are essentially optional. The top-level `title` string is used to hold metadata on the file itself.

Further fields might be added later on to the specification.

Currently all library TOML files are located under the [library](library) directory.
