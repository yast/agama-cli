# dinstaller-rs

This project aims to build an alternative command-line interface for
[D-Installer](https://github.com/yast/d-installer), a service-based Linux installer featuring a nice
web interface. The point of this project is to move away from [Ruby](https://ruby-lang.org/) (that
we all love :heart:) and try something different like [Rust](https://rust-lang.org/).

We are building this project in the context of [Hack Week
22](https://hackweek.opensuse.org/22/projects/rewrite-the-d-installer) so, if you are interested, do
not hesitate to join us.

We have set up [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) with
two packages:

* [dinstaller-lib](./dinstaller-lib): which contains the code that can be reused to access the [D-Installer DBus
  API](https://github.com/yast/d-installer/blob/master/doc/dbus_api.md).
* [dinstaller-cli](./dinstaller-cli): which contains the code specific to the command line interface.

## Running

This project is far from being finished. However, you can use it to set some basic installation
options. Have into account that you need to run `dinstaller-cli` as root. You can compile the
project and run it as root:

```
$ cargo build
$ sudo ./target/debug/dinstaller-cli --help
Usage: dinstaller-cli [OPTIONS] <COMMAND>
...
```

Assuming that the D-Installer D-Bus service is running, the next command prints the current
configuration using JSON (you can use YAML if you like it more):

```
$ sudo ./target/debug/dinstaller-cli --format json config show
{"user":{"full_name":"","user_name":"","password":"","autologin":false},"software":{"product":""}}
```

To set one or multiple parameters, just use the `config set` command:

```
$ sudo ./target/debug/dinstaller-cli config set software.product=Tumbleweed user.full_name="Jane Doe" \
    user.user_name="jane.doe" user.password="12345" user.autologin=true
```

The following operation can take some time. Please, make sure to read the *Caveats* section for more
information.

```
$ sudo ./target/debug/dinstaller-cli config show
{"user":{"full_name":"Jane Doe","user_name":"jane.doe","password":"","autologin":true},"software":{"product":"Tumbleweed"}}
```

## Caveats

If you give `dinstaller-rs` a try, beware that after merging
[imobachgs/dinstaller-rs#12](https://github.com/imobachgs/dinstaller-rs/pull/12), setting any option
is rather slow. The problem is that we are sending all the configuration settings, including the
selected product, which is a slow operation. To avoid this problem we could 1) stop sending
unchanged data and/or 2) do not reload the repositories if the product did not change (in the
D-Installer side).
