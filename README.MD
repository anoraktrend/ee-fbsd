Hi! This is a version of EE extracted from freebsd. You can build this with meson and Ninja. Simply run ``meson setup build``, ``ninja -c build``, ``meson install build``. This should work on most linux distros, macos, and freebsd. The meson version is untested on other bsds, other unixlikes or unixes, and windows 11 but it should work on all of those. 
Along with this version with an updated build system, there is an additional rust conversion.
To install the rust version, simply run:

``cargo +nightly install --git https://github.com/anoraktrend/ee-fbsd ee-rust``

or

``git clone https://github.com/anoraktrend/ee-fbsd && cd ee-fbsd && cargo install --path .``

Preferably, though, you should build the C version with make or meson & ninja. The C version is well tested and known to work.
