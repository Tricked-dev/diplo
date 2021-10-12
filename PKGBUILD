# Maintainer: Tricked-dev
pkgname=diplo
pkgver=0.3.1
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Diplo is a script runner and dependency manager made in rust"
url="https://tricked.pro/diplo"
license=('Apache-2.0')

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://github.com/Tricked-dev/diplo
}
