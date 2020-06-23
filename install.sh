#!/bin/bash

set -u

DOTENV_LINTER="dotenv-linter"
DOTENV_LINTER_REPO="${DOTENV_LINTER}/${DOTENV_LINTER}"
DOTENV_LINTER_GITHUB="https://github.com/${DOTENV_LINTER_REPO}"
DOTENV_LINTER_RELEASE_API="https://api.github.com/repos/${DOTENV_LINTER_REPO}/releases/latest"

main() {
    require_cmd uname
    require_cmd mkdir
    require_cmd grep
    require_cmd cut
    require_cmd rm

    local _target_version
    if [ $# -eq 0 ]; then
        get_latest_version || return 1
        _target_version="$RETVAL"
    else
        _target_version="$1"
    fi

    get_architecture || return 1
    local _arch="$RETVAL"

    local _ext
    case $_arch in
        win-*) _ext=".zip" ;;
        *) _ext=".tar.gz" ;;
    esac

    local _archive_name="${DOTENV_LINTER}-${_arch}${_ext}"
    local _url="${DOTENV_LINTER_GITHUB}/releases/download/${_target_version}/${_archive_name}"

    # Installation
    local _dir="$HOME/.${DOTENV_LINTER}"
    local _version_dir="${_dir}/${_target_version}"

    if [ ! -e "$_version_dir/${DOTENV_LINTER}" ]; then
        mkdir -p "$_version_dir"

        local _archive_path="${_version_dir}/${_archive_name}"
        download "${_url}" "${_archive_path}" || return 1

        case $_archive_path in
            *.zip)
                required_cmd unzip
                unzip "${_archive_path}" -d "${_version_dir}"
            ;;
            *)  require_cmd tar
                tar -xzf "${_archive_path}" -C "${_version_dir}"
            ;;
        esac

        rm "${_archive_path}"

        println "Successful download \"dotenv-linter\" to ${_version_dir}"!
    else
        println "WARN:: ${_version_dir} already exists! We just switch the version to ${_target_version}"
        println "Or you can manually remove dir: 'rm -rf ${_version_dir}' and reinstall."
    fi

    # Setup installed version as executable
    setup_executable "${_dir}" "${_target_version}" || return 1

    return 0;
}

# $1 - dotenv-linter dir, $2 - target version
setup_executable() {
    local _bin_dir="$1/bin"
    if [ ! -d "$_bin_dir" ]; then
	    mkdir -p "$_bin_dir"
    fi

    local _bin_file="${_bin_dir}/${DOTENV_LINTER}"
    ln -sf "$1/$2/${DOTENV_LINTER}" "${_bin_file}"

    local _shell_profile
    case ${SHELL-undefined} in
        /bin/zsh) _shell_profile=~/".zshrc" ;;
        *) _shell_profile=~/".bashrc" ;;
	esac

    if [ -e "$_shell_profile" ]; then
        case :$PATH: in
            *:$_bin_dir:*) ;; # do nothing, already exists
            *)  echo "export PATH=\"$_bin_dir:\$PATH\"" >> "${_shell_profile}"
                export PATH="${_bin_dir}:$PATH"
                println "Will be available in new terminal sessions or after update config: \". ${_shell_profile}\""
            ;;
        esac
    fi

    if ! cmd_exists dotenv-linter ; then
        println ""
	    println "Manually add the directory to your environment:"
        println "   export PATH=\"\$PATH:${_bin_dir}\""
        println ""
	fi

	println "Run '${DOTENV_LINTER} --help' to get started"
}

get_latest_version() {
    local _latest_version
    download "${DOTENV_LINTER_RELEASE_API}" || return 1
    _latest_version=$(
         echo "$RETVAL" | grep '"tag_name":' | cut -d'"' -f4
    )

    RETVAL=${_latest_version}
}

get_architecture() {
    local _ostype _cputype _clibtype
    _ostype="$(uname -s)"
    _cputype="$(uname -m)"
    _clibtype="gnu"

    if [ "$_cputype" != "x86_64" ]; then
	    err "Error: Unsupported architecture $_cputype. Only x86_64 binaries are available."
    fi

    if [ "$_ostype" = Linux ]; then
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
            _cputype="x64"
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
    if [ $# -eq 0 ]; then
        err "URL not specified"
    fi

    if cmd_exists curl; then
        if [ $# -eq 2 ]; then
            curl -sSfL "$1" -o "$2"
        else
            RETVAL=$(curl -sSfL "$1")
        fi

    elif cmd_exists wget; then
        if [ $# -eq 2 ] ; then
            wget -q "$1" -O "$2"
        else
            RETVAL=$(wget -q -O - "$1")
        fi

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
    printf '%s installer:   %s\n' "${DOTENV_LINTER}" "$1"
}

main "$@" || exit 1
