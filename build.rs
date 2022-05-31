fn main() {
	// Building the C code
	println!("cargo:rerun-if-changed=src/module.c");
	cc::Build::new()
		.file("src/module.c")
		.file("src/mount.c")
		.file("src/tty.c")
		.compile("solfege")
}
