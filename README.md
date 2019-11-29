# Streamlib

![](https://github.com/streamlib/streamlib/workflows/Test/badge.svg)
[![](https://img.shields.io/crates/v/streamlib.svg)](https://crates.io/crates/streamlib)

Streamlib is a meta-player for media streams. The streamlib CLI works against a curated [library](https://github.com/streamlib/library) of video and audio streams, and enables querying and playback through a standard media player, defaulting to [mpv](https://mpv.io/) which is the recommended player.

The most important feature Streamlib provides is the ability to generate timestamped authentication tokens on streams that require some basic level of authentication to them. See the [Queries](https://github.com/streamlib/library#queries) section for more details.

## Usage

Streamlib is still moving fast and has yet to been officially packaged, but the basic functionality works!

Running it requires checking out the git repository and then calling:

```bash
$ cargo run -- groove
```

The single argument will run a case-insensitive match against all known metadata and pick the first matching stream. Upcoming versions will include a console-based interactive interface.

Entire library can be dumped with the `-L` flag:

```bash
$ cargo run -- -L
```

Or filtered to show a specific query:

```bash
$ cargo run -- -L somafm
```

An alternate player can be set with the `-p`/`--player` flag:

```bash
$ cargo run -- -p vlc groove
```

But note that some features critical for playing streams (such as customizing HTTP headers) are unavailable and will cause some streams to fail.

Use the `-l`/`--library` flag to point streamlib to a local library directory for testing new additions:

```bash
$ cargo run -- --library /path/to/local/library groove
```

### Testing

```bash
$ cargo test
```

## Library

To add new content and test it you'll need to clone both repositories:

```bash
$ git clone https://github.com/streamlib/streamlib
$ git clone https://github.com/streamlib/library
# add any files you want to the library
$ cd streamlib
$ cargo run -- --library ../library groove
```

See https://github.com/streamlib/library for more details.

## License

[GPLv3](LICENSE)
