#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // add unix dependencies below
    println!("cargo:rustc-flags=-l readline -l ncurses");
}

#[cfg(target_os = "macos")]
fn main() {
    // add macos dependencies below
    println!("cargo:rustc-flags=-l edit -l ncurses");
}
