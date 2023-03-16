# d-installer-cli

This project aims to build a command-line interface for
[D-Installer](https://github.com/yast/d-installer), a service-based Linux installer featuring a nice
web interface.

## Code organization

We have set up [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) with
three packages:

* [dinstaller-lib](./dinstaller-lib): code that can be reused to access the
  [D-Installer DBus API](https://github.com/yast/d-installer/blob/master/doc/dbus_api.md) and a
  model for the configuration settings.
* [dinstaller-cli](./dinstaller-cli): code specific to the command line interface.
* [dinstaller-derive](./dinstaller-derive): includes a [procedural
  macro](https://doc.rust-lang.org/reference/procedural-macros.html) to reduce the boilerplate code.

## Status

`d-installer-cli` is still a work in progress, although it is already capable of doing a few things:

* Querying and setting the configuration for the users, storage and software services.
* Handling the auto-installation profiles.
* Triggering the *probing* and the *installation* processes.

## Installation

You can grab the [RPM package](https://build.opensuse.org/package/show/YaST:Head:D-Installer/d-installer-cli) from
the [YaST:Head:D-Installer](https://build.opensuse.org/project/show/YaST:Head:D-Installer) project.

If you prefer, you can install it from sources with [Cargo](https://doc.rust-lang.org/cargo/):

```
git clone https://github.com/yast/d-installer-cli
cargo install --path .
```

## Running

Take into account that you need to run `dinstaller-cli` as root when you want to query or change the
D-Installer configuration. Assuming that the D-Installer D-Bus service is running, the next command
prints the current settings using JSON (hint: you can use `jq` to make result look better):

```
$ sudo dinstaller --format json config show
{"user":{"fullName":"","userName":"","password":"","autologin":false},"software":{"product":""}}
```

To set one or multiple parameters, just use the `config set` command:

```
$ sudo dinstaller config set software.product=Tumbleweed user.fullName="Jane Doe" user.userName="jane.doe" user.password="12345" user.autologin=true
```

The following operation can take some time. Please, make sure to read the *Caveats* section for more
information.

```
$ sudo dinstaller config show
{"user":{"fullName":"Jane Doe","userName":"jane.doe","password":"","autologin":true},"software":{"product":"Tumbleweed"}}
```

If, at some point you want to force a new probing, you can ask D-Installer to repeat the process again:

```
$ sudo dinstaller probe
```

It is possible to handle auto-installation profiles too:

```
$ dinstaller profile download http://192.168.122.1/profile.jsonnet
$ dinstaller profile evaluate profile.jsonnet > profile.json
$ dinstaller profile validate profile.json
```

Now that you have a ready to use profile, you can load it into D-Installer:

```
$ sudo dinstaller config load profile.json
```

## Building and running

You can build and run the project using the `cargo` command:

```
cargo build
sudo ./target/debug/dinstaller --help
```

## Caveats

* If no product is selected, the `probe` command fails.
