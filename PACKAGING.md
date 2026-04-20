# Linux Packaging Guide

This document describes how to package `duckduckgo` for major Linux distributions like Debian/Ubuntu and RedHat/Fedora.

## Debian/Ubuntu (.deb)

The `debian/` directory handles building `.deb` packages using standard `debhelper` and `cargo` infrastructure.

### Build Instructions

1. Install build dependencies:
   ```sh
   sudo apt-get update
   sudo apt-get install build-essential debhelper dh-cargo devscripts pkg-config libssl-dev cargo rustc
   ```
1. Build the package in the root directory:
   ```sh
   debuild -us -uc -b
   ```
1. Install the generated package:
   ```sh
   sudo dpkg -i ../ddg-rs_0.3.0-1_amd64.deb
   ```

## RedHat/Fedora (.rpm)

The `rpm/` directory manages `.rpm` package configurations through the `ddg-rs.spec` file.

### Build Instructions

1. Install build dependencies:
   ```sh
   sudo dnf install rpm-build rpmdevtools cargo rust pkgconfig openssl-devel
   ```
1. Set up the `rpmbuild` workspace tree:
   ```sh
   rpmdev-setuptree
   cp rpm/ddg-rs.spec ~/rpmbuild/SPECS/
   ```
1. Archive the module and place it into the `SOURCES` directory:
   ```sh
   tar -czvf ~/rpmbuild/SOURCES/ddg-rs-0.3.0.tar.gz .
   ```
1. Build the RPM:
   ```sh
   rpmbuild -bb ~/rpmbuild/SPECS/ddg-rs.spec
   ```
1. Install the built RPM:
   ```sh
   sudo dnf install ~/rpmbuild/RPMS/x86_64/ddg-rs-0.3.0-1.x86_64.rpm
   ```
