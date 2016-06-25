use super::parser::Command;
use super::parser::Value;
use std::collections::HashMap;

mod syscalls;
mod instructions;

// ram value
// 0: instruction pointer
//
//
//
pub struct Vm<'a> {
    prog: &'a Vec<Command<'a>>,
    ram: Vec<u32>,

    // a stack for func calls. records where the instruction pointer was when a func was called,
    // and returns to that instruction when ret is encountered
    call_stack: Vec<usize>,

    instruction_pointer: usize,

    // translates label names to the instruction pointer where that function begins
    label_table: HashMap<&'a str, usize>,
}

impl<'a> Vm<'a> {
    pub fn new<'b>(new_prog: &'b Vec<Command>) -> Vm<'b> {
        Vm {
            prog: new_prog,
            ram: vec![0;32],
            call_stack: Vec::new(),
            instruction_pointer: 0,
            label_table: HashMap::new(),
        }
    }

    pub fn exec(&mut self) {
        self.build_label_table();

        while self.instruction_pointer < self.prog.len() {
            let next_command = &self.prog[self.instruction_pointer];
            self.exec_single_command(next_command);
            self.instruction_pointer += 1;
        }
    }

    // returns u32 of a value field.
    // needs to be &mut self because retrieving values can cause ram to grow
    pub fn get_value(&mut self, value: &Value) -> u32 {
        match *value {
            Value::U32(n) => n,
            Value::Address(ref address) => {
                let address_val = self.get_value(address);
                self.get_ram(address_val as usize)
            }
        }
    }

    // retirives the <index> value of ram.
    // If it is ouside the vector length, auto grows vector
    pub fn get_ram(&mut self, index: usize) -> u32 {
        while index > self.ram.len() {
            self.ram.push(0);
        }
        self.ram[index]
    }

    pub fn set_ram(&mut self, index: usize, value: u32) {
        while index > self.ram.len() {
            self.ram.push(0);
        }
        self.ram[index] = value;
    }


    fn build_label_table(&mut self) {
        // for some reason we cannot use enumerate() here, so we must use a manual increment iterator
        let mut iter = 0;
        for command in self.prog {
            if let Command::Label(name) = *command {
                self.label_table.insert(name, iter);
            }
            iter += 1;
        }
    }
}
