%global debug_package %{nil}

Name:    duckduckgo
Version: 0.3.1
Release: 1%{?dist}
Summary: A CLI, TUI, and SDK for instant DuckDuckGo searches
License: MIT
URL:     https://github.com/wiseaidotdev/%{name}
Source0: %{url}/archive/refs/tags/v%{version}.tar.gz

BuildRequires: cargo
BuildRequires: rust
BuildRequires: openssl-devel

Requires: openssl

%description
Duckduckgo is a multi-language toolkit for searching DuckDuckGo from
code or the command line. The core is written entirely in Rust and
compiled to a native extension. Features include Instant Answer, Lite, Images,
and News backends, along with a standalone CLI and TUI.

%prep
%autosetup -n %{name}-%{version}

%build
export RUSTFLAGS="%{build_rustflags} -C strip=symbols"
cargo build --release --features=rust-binary

%install
install -Dpm 0755 target/release/ddg -t %{buildroot}%{_bindir}/
install -Dpm 0644 README.md -t %{buildroot}%{_docdir}/%{name}/

%files
%license LICENSE
%doc %{_docdir}/%{name}/README.md
%{_bindir}/ddg

%changelog
* Mon Apr 20 2026 Mahmoud Harmouch <oss@wiseai.dev> - 0.3.1-1
- Initial release
