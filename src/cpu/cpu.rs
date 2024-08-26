use std::thread::panicking;

use crate::cpu::instructions::{Instruction, ArithmeticTarget};
use crate::cpu::registers::{Registers, Flags};


struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

struct MemoryBus{
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, addr: u16) -> u8{
        self.memory[addr as usize]
    }
}

impl CPU {

    fn step(&mut self){
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte){
            self.execute(instruction)
        } else {
            panic!("Unknown instruction found for: 0x{:x}", instruction_byte);
        };

        //TODO: Get the next pc from executions
        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let val = self.registers.c;
                        let new_val = self.add(val);
                        self.registers.a = new_val;

                        self.pc.wrapping_add(1)
                    },
                    _ => {/* TODO: Targets */ self.pc}
                }
            },
            /*
            Instruction::INC(target) => {
                match target {
                    ArithmeticTarget::A => self.registers.a += 1,
                    ArithmeticTarget::B => self.registers.b += 1,
                    ArithmeticTarget::C => self.registers.c += 1,
                    ArithmeticTarget::D => self.registers.d += 1,
                    ArithmeticTarget::E => self.registers.e += 1,
                    ArithmeticTarget::H => self.registers.h += 1,
                    ArithmeticTarget::L => self.registers.l += 1,
                }
            },
             */
            _ => {/* TODO: instructions*/ self.pc}
        }
    }

    fn add(&mut self, val: u8) -> u8 {
        //Is this A register alright?
        let (new_val, was_overflow) = self.registers.a.overflowing_add(val);

        // Set flags. Why am i using a Flags struct within a Registers one?
        self.registers.f.zero = new_val == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = was_overflow;
        // The result has overflowed into another register. Is this alright?
        self.registers.f.half_carry = (self.registers.a & 0xF) + (val & 0xF) > 0xF;

        new_val
    }
}