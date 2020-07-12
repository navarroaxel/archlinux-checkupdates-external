# archlinux-checkupdates-external

![Rust](https://github.com/navarroaxel/archlinux-checkupdates-external/workflows/Rust/badge.svg)
![Lint Code Base](https://github.com/navarroaxel/archlinux-aur-contrib/workflows/Lint%20Code%20Base/badge.svg)

This app checks updates from the Google Chrome and JetBrains upstream
repositories for the Arch Linux AUR packages. The output format is similar
to the Arch Linux checkupdates command.

```bash
<package> <current_ver> -> <new_version>
```

For Google Chrome the app fetch the gzip file with metadata for the official
Yum repository.

For JetBrains, downloads the updates XML and checks for specifics product, in
the release and EAP channels.

```bash
$ checkupdates-external
rider 2020.1.3 -> 2020.1.4
```
