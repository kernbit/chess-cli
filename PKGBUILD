# Maintainer: kernbit kernbit@protonmail.com
pkgname=chess-cli
pkgver=0.1.0
pkgrel=1
pkgdesc="CLI Chess Game with Stockfish Integration"
arch=('x86_64')
url="https://github.com/kernbit/chess-cli"
license=('MIT')
depends=('stockfish')
makedepends=('rust' 'cargo')
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/kernbit/${pkgname}/archive/v${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$srcdir/$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
    install -Dm0644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm0644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}