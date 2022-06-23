# Releasing

## Arch User Repository (AUR)

### [dotenv-linter-git](https://aur.archlinux.org/packages/dotenv-linter-git/)

There is no need to release this package because when you install it, it builds the current master with `cargo build` and puts the resulting binary in `/usr/bin`.

### [dotenv-linter-bin](https://aur.archlinux.org/packages/dotenv-linter-bin/)

In order to release the version you need access to the AUR-package.
Contact [@mstruebing](https://github.com/mstruebing) or [@mgrachev](https://github.com/mgrachev) if you need access.

1. Clone the repository: `git clone ssh://aur@aur.archlinux.org/dotenv-linter dotenv-linter-bin`.
2. Edit the file `PKGBUILD` and update the version number in `pkgver` and save.
3. Run `makepkg --printsrcinfo > .SRCINFO` to update
4. Add and commit your changes: `git add . && git commit -m 'release new version'`
5. Push your changes: `git push origin master`

If you change how the package builds (i.e. Updating the `package`-function)
you should also update the `pkgrel` number by one.
