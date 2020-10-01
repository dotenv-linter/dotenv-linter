# 👨‍💻 Install

You can install the pre-compiled binary (in several different ways), use Docker or compile from source.

**Binary**

```shell script
# Linux / macOS / Windows (MINGW and etc). Installs it into ./bin/ by default.
$ curl -sSfL https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh | sh -s

# Specify installation directory and version.
$ curl -sSfL https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh | sh -s -- -b usr/local/bin v2.0.0

# Alpine Linux (wget)
$ wget -q -O - https://raw.githubusercontent.com/dotenv-linter/dotenv-linter/master/install.sh | sh -s
```

**Homebrew / Linuxbrew**

```shell script
# Installs the latest stable release
$ brew install dotenv-linter/tap/dotenv-linter

# Builds the latest version from the repository
$ brew install --HEAD dotenv-linter/tap/dotenv-linter
```

**Arch Linux / AUR**

```shell script
# Use your favourite AUR-helper, e.g. trizen

# Installs the latest stable release
$ trizen -S dotenv-linter-bin

# Builds the latest version from the repository
$ trizen -S dotenv-linter-git
```

**Windows / Scoop**

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
