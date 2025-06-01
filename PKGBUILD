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
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
