use super::Vm;
use std::char;


//implement all syscalls of the Vm
impl Vm {
    pub fn syscall(&mut self, name: &str) {
        match name {
            "print" => self.print(),
            "print_ascii" => self.print_ascii(),
            _ => panic!("unknown syscall"),
        }
    }


    // print the value in address 0 as a decimal
    fn print(&mut self) {
        println!("{}", self.get_ram(0));
    }

    // print the value in address 0 as an ascii char
    fn print_ascii(&mut self) {
        println!("{}",
                 char::from_u32(self.get_ram(0)).expect("char is not a valid character"));
    }
}
