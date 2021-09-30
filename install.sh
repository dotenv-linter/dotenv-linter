#!/bin/sh

set -e

DOTENV_LINTER="dotenv-linter"
DOTENV_LINTER_REPO="${DOTENV_LINTER}/${DOTENV_LINTER}"
DOTENV_LINTER_GITHUB="https://github.com/${DOTENV_LINTER_REPO}"
DOTENV_LINTER_RELEASES="${DOTENV_LINTER_GITHUB}/releases"

usage() {
    require_cmd cat
    this=$1
    cat 1>&2 <<EOF
$this: download binaries for dotenv-linter

USAGE:
    $this [FLAGS] [OPTIONS] <tag>

FLAGS:
    -h, --help      Prints help information

OPTIONS:
    -b, --bindir <DIR_PATH>     Sets bindir or installation directory. Defaults to ./bin

ARGS:
    <tag>       is a tag from ${DOTENV_LINTER_RELEASES}. If tag is missing, then the latest will be used.
EOF
    exit 2
}

parse_args() {
  BINDIR=${BINDIR:-./bin}
  while [ "$#" -gt 0 ]; do
    case $1 in
        -h|--help)
            usage "$0"
            shift # past argument
            ;;
        -b|--bindir)
            BINDIR="$2"
            shift # past argument
            shift # past value
            ;;
        *) TAG=$1
            shift # past argument
            ;;
    esac
  done
}

main() {
    parse_args "$@"

    require_cmd uname
    require_cmd mktemp
    require_cmd grep
    require_cmd rm

    get_architecture || return 1
    _arch="$RETVAL"

    case $_arch in
        win-*) _ext=".zip" ;;
        *) _ext=".tar.gz" ;;
    esac

    _archive_name="${DOTENV_LINTER}-${_arch}${_ext}"

    if [ -z "${TAG}" ]; then
        println "The latest version will be installed."
        _url="${DOTENV_LINTER_RELEASES}/latest/download/${_archive_name}"
    else
        println "Version ${TAG} will be installed"
        _url="${DOTENV_LINTER_RELEASES}/download/${TAG}/${_archive_name}"
    fi

    # Installation
    _temp_dir=$(mktemp -d)
    _archive_path="${_temp_dir}/${_archive_name}"

    download "${_url}" "${_archive_path}" || return 1

    case $_archive_path in
        *.zip)
            require_cmd unzip
            unzip "${_archive_path}" -d "${_temp_dir}"
            _file_name="${DOTENV_LINTER}.exe"
        ;;
        *)  require_cmd tar
            tar -xzf "${_archive_path}" -C "${_temp_dir}"
            _file_name="${DOTENV_LINTER}"
        ;;
    esac

    test ! -d "${BINDIR}" && install -d "${BINDIR}"

    install "${_temp_dir}/${_file_name}" "${BINDIR}/" || err "Failed to install"
    _exe_path=${BINDIR}/${_file_name}

    println "Successfully installed $($_exe_path -v) to ${_exe_path}"

    rm -rf "${_temp_dir}"

    return 0;
}

get_architecture() {
    _ostype="$(uname -s)"
    _cputype="$(uname -m)"
    _clibtype="gnu"

    if [ "$_cputype" != "x86_64" ] && [ "$_cputype" != "aarch64" ] && [ "$_cputype" != "arm64" ]; then
	    err "Error: Unsupported architecture $_cputype. Only 'x86_64' and 'aarch64/arm64' binaries are available."
    fi

    if [ "$_ostype" = "Linux" ]; then
        if ldd --version 2>&1 | grep -q 'musl'; then
            _clibtype="musl"
        fi
    fi

    case "$_ostype" in

        Linux)
            if [ "$_clibtype" != "musl" ]; then
                _ostype=linux
            else
                # For 'musl' architecture we release alpine version
                _ostype=alpine
            fi
        ;;

        Darwin)
            _ostype=darwin
        ;;

        CYGWIN*|MINGW32*|MSYS*|MINGW*) # windows systems
            _ostype=win
            if [ "$_cputype" = "x86_64" ]; then
                _cputype="x64"
            fi
        ;;

        *)
            err "Unsupported OS type: $_ostype"
        ;;

    esac

    RETVAL="${_ostype}-${_cputype}"
}

# $1 - url for download. $2 - path to download
# Wrapper function for curl/wget
download() {
    if [ ! $# -eq 2 ]; then
        err "URL or target path not specified"
    fi

    if cmd_exists curl; then
        curl -sSfL "$1" -o "$2"
    elif cmd_exists wget; then
        wget -q "$1" -O "$2"
    else
        err "Not found download command. 'curl' or 'wget' is required."
    fi

    if [ $# -eq 2 ] && [ ! -f "$2" ]; then
        err "Failed to download file $1"
    fi
}

require_cmd() {
    if ! cmd_exists "$1"; then
        err "'$1' is required (command not found)."
    fi
}

cmd_exists() {
    command -v "$1" > /dev/null 2>&1
}

err() {
    println "$1" >&2
    exit 1
}

println() {
    printf '%s installer: %s\n' "${DOTENV_LINTER}" "$1"
}

main "$@" || exit 1
