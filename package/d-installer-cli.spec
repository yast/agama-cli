#
# spec file for package d-installer-cli
#
# Copyright (c) 2023 SUSE LINUX GmbH, Nuernberg, Germany.
#
# All modifications and additions to the file contributed by third parties
# remain the property of their copyright owners, unless otherwise agreed
# upon. The license for this file, and modifications and additions to the
# file, is the same license as for the pristine package itself (unless the
# license for the pristine package is not an Open Source License, in which
# case the license is the MIT License). An "Open Source License" is a
# license that conforms to the Open Source Definition (Version 1.9)
# published by the Open Source Initiative.

# Please submit bugfixes or comments via http://bugs.opensuse.org/
#

Name:           d-installer-cli
#               This will be set by osc services, that will run after this.
Version:        0
Release:        0
Summary:        D-Installer command line interface
#               If you know the license, put it's SPDX string here.
#               Alternately, you can use cargo lock2rpmprovides to help generate this.
License:        GPL-2.0-only
Url:            https://github.com/yast/d-installer-cli
Source0:        %{name}-%{version}.tar.zst
Source1:        vendor.tar.zst
Source2:        cargo_config
BuildRequires:  cargo-packaging
BuildRequires:  pkgconfig(openssl)
Requires:       jsonnet
Requires:       lshw
# Disable this line if you wish to support all platforms.
# In most situations, you will likely only target tier1 arches for user facing components.
# ExclusiveArch:  %{rust_tier1_arches}

%description
Command line program to interact with the D-Installer service.

%prep
%autosetup -a1
mkdir .cargo
cp %{SOURCE2} .cargo/config
# Remove exec bits to prevent an issue in fedora shebang checking. Uncomment only if required.
# find vendor -type f -name \*.rs -exec chmod -x '{}' \;

%build
%{cargo_build}

%install
install -D -d -m 0755 %{buildroot}%{_bindir}
install -m 0755 %{_builddir}/%{name}-%{version}/target/release/dinstaller %{buildroot}%{_bindir}/dinstaller
install -D -d -m 0755 %{buildroot}%{_datadir}/d-installer-cli
install -m 0644 %{_builddir}/%{name}-%{version}/dinstaller-lib/share/profile.schema.json %{buildroot}%{_datadir}/d-installer-cli
 
%check
%{cargo_test}

%files
%{_bindir}/dinstaller
%dir %{_datadir}/d-installer-cli
%{_datadir}/d-installer-cli/profile.schema.json

%changelog
