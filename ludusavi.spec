%define  metainfo_name com.github.mtkennerly.ludusavi

Name:    ludusavi
Version: 0.23.0
Release: alt1

Summary: Backup tool for PC game saves
License: MIT
Group:   Games/Other
Url:     https://github.com/mtkennerly/ludusavi

Packager: Maxim Slipenko <no-reply@maxim.slipenko.com>

# Source-url: https://github.com/mtkennerly/ludusavi/archive/refs/tags/v%version.tar.gz
Source: %name-%version.tar

Source1: %name-development-%version.tar

BuildRequires(pre): rpm-macros-rust
BuildRequires: rpm-build-rust
BuildRequires: /proc

%description
%summary

%prep
%setup -a 1

mkdir -p .cargo
cat >> .cargo/config <<EOF
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

%build
%rust_build

%install
%rust_install

install -Dm644 assets/%name.desktop %buildroot%_desktopdir/%name.desktop
install -Dm644 assets/%metainfo_name.metainfo.xml %buildroot%_datadir/metainfo/%metainfo_name.metainfo.xml
install -Dm644 assets/icon.svg %buildroot%_iconsdir/hicolor/scalable/apps/%name.svg
install -Dm644 assets/icon.png %buildroot%_iconsdir/hicolor/64x64/apps/%name.svg

%check
%rust_test

%files
%doc *.md
%_bindir/*
%_desktopdir/*
%_datadir/metainfo/*
%_iconsdir/hicolor/*

%changelog
* Sat May 11 2024 Maxim Slipenko <no-reply@maxim.slipenko.com> 0.23.0-alt1
- Initial build for Sisyphus
