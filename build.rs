extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("perfect6502/perfect6502.c")
        .file("perfect6502/runtime.c")
        .file("perfect6502/runtime_init.c")
        .file("perfect6502/plugin.c")
        .file("perfect6502/console.c")
        .file("perfect6502/emu.c")
        //.flag("-Wall")
        .compile("libperfect6502.a");
}