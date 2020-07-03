# 👨‍💻 Install

You can install the pre-compiled binary (in several different ways), use Docker or compile from source.

**Binary**

```shell script
# Linux / macOS / Windows (MINGW and etc)
$ curl -sSfL "https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh" | sh -s

# Alpine Linux (wget)
$ wget -q -O - "https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh" | sh -s

# Specific version
$ curl -sSfL "https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh" | sh -s v1.2.0
```

**Homebrew / Linuxbrew**

```shell script
$ brew install dotenv-linter/tap/dotenv-linter
```

**Arch Linux / AUR**

```shell script
# use your favourite AUR-helper
$ trizen -S dotenv-linter-bin # for the binary distribution
$ trizen -S dotenv-linter-git # for the current master branch
```

**Windows / Scoop** (will be available in [v2.1.0](https://github.com/dotenv-linter/dotenv-linter/issues/217))

```shell script
$ scoop bucket add dotenv-linter https://github.com/dotenv-linter/scoop.git
$ scoop install dotenv-linter/dotenv-linter
````

**Docker**

```shell script
$ docker run --rm -v `pwd`:/app -w /app dotenvlinter/dotenv-linter
```

**Cargo**

If you are a **Rust** programmer, you can install `dotenv-linter` via `cargo`:

```shell script
$ cargo install dotenv-linter
```
