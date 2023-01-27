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
