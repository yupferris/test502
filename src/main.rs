const MEMORY_SIZE: usize = 65536;
type Memory<'a> = &'a [u8; MEMORY_SIZE];

#[derive(Debug)]
struct Status {
    half_cycle: usize,
    phi_0: bool,
    address_bus: u16,
    data_bus: u8,
    read_write: bool,
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    p: u8,
    ir: u8
}

trait TestModule {
    fn step(&mut self);
    fn status(&self) -> Status;

    fn memory(&self) -> &Memory;
    fn set_memory(&mut self, mem: &Memory);
}

fn main() {
    println!("Hello, world!");
}
