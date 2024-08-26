use std::fmt::format;
use std::thread::panicking;

use crate::cpu::instructions::{Instruction, ArithmeticTarget, JumpTest};
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
        let prefix_instr = instruction_byte == 0xCB;
        if prefix_instr {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefix_instr){
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefix_instr {"cb"} else {""}, instruction_byte);
            panic!("Unknown instruction found for: 0x{}", description);
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
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }
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

    // The address we jump to is located in the two bytes following the instruction identifier as:
    // [instr. id.] [least sig. byte] [most sig. byte]
    fn jump(&self, is_jump: bool) -> u16 {
        // Gameboy is little endian so pc +2 is the most significant bit and +1 the least
        if is_jump {
            let lsb = self.bus.read_byte(self.pc + 1) as u16;
            let msb = self.bus.read_byte(self.pc + 2) as u16;
            (msb << 8) | lsb
        } else {
            self.pc.wrapping_add(3)
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