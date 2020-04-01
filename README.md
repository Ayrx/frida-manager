# frida-manager

`frida-manager` is a command line utility for managing Frida updates. It
currently fetches the latest `frida-server` binaries from GitHub and caches
them in the `~/.fridamanager` directory.

## Usage

```
➜ ./frida-manager
frida-manager 0.1.0
Terry Chia <terrycwk1994@gmail.com>


USAGE:
    frida-manager [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clean     Clean cached artifacts.
    fetch     Download Frida artifacts.
    help      Prints this message or the help of the given subcommand(s)
    status    Check Frida status.
```

### Status

The `status` subcommand checks the installed Frida version against the latest
available version.

```
➜ ./frida-manager status
[+] Latest Frida Release: 12.8.20
[+] Currently installed Frida: 12.8.0

Currently installed version of Frida is not the latest version. Please update!
```

### Fetch

The `fetch` subcommand downloads the latest versions of `frida-server`.
If a specific version is needed, the `--version` option can be used. Check the
`-h` command for details.

```
➜ ./frida-manager fetch
[+] Frida Version: 12.8.20
[+] 12 frida-server binaries found.
[+] frida-server-12.8.20-android-arm.xz is cached.
[+] frida-server-12.8.20-android-arm64.xz is cached.
[+] frida-server-12.8.20-android-x86.xz is cached.
[+] Downloading frida-server-12.8.20-android-x86_64.xz.
```

Binaries that were downloaded previously are cached and will not be fetched
again.

### Clean

The `clean` subcommand clears the cached binaries in `~/.fridamanager`.

## Building

`frida-manager` can be compiled to a Linux static binary with
[muslrust][muslrust].

Run the following commands:

```
➜ docker pull clux/muslrust
➜ docker run -v $PWD:/volume --rm -t clux/muslrust cargo build
```

Refer to the muslrust documentation for more information and options.

[muslrust]: https://github.com/clux/muslrust
