# Streamlib

Streamlib is a meta-player for video streams. The specification defines a standard for indexing video stream URLs, and the CLI enables quick searching of those streams. Once a stream is chosen for viewing, it is passed on to a standard media player, defaulting to [mpv](https://mpv.io/) which is the recommended player.

## Usage

Streamlib is still a work-in-progress. Running it requires checking out the git repository and then running:

```bash
$ cargo run -- groove
```

The single argument will run a case-insensitive match against all known metadata and pick the first matching stream. Upcoming versions will include a console-based interactive interface.

Entire library can be dumped with the `-L` flag:

```bash
$ cargo run -- -L
```

An alternate player can be set with the `-p` flag:

```bash
$ cargo run -- -p vlc groove
```

But note that some features critical for playing streams (such as customizing HTTP headers) are unavailable and will cause some streams to fail.

Use the `--library` flag to point streamlib to a local library directory for testing new additions:

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
