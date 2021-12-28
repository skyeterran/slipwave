use std::io;
use std::fs;
use std::fs::File;
use std::io::Read;

enum VCROperationKind {
    Generic,
    Type,
    Value
}

pub struct ComputeObject {
    instructions: Vec<u8>,
    values: Vec<f32>
}

impl ComputeObject {
    pub fn from_file(file_path: &String) -> ComputeObject {
        ComputeObject {
            instructions: get_file_as_byte_vec(file_path),
            values: vec![]
        }
    }

    pub fn execute(&mut self) -> &Vec<f32> {
        //println!("Executing bytecode instructions...\n");
        let mut iter = self.instructions.iter();
        
        // Handle literals
        let mut op_type = VCROperationKind::Generic;
        let mut literal = [0u8; 4];
        let mut lit_digit = 0;
    
        // Handle each opcode in order
        loop {
            let byte_opt = iter.next();
    
            // Contextually handle opcodes depending on their type
            match op_type {
                // Treat the opcode as a literal type declaration
                VCROperationKind::Type => {
                    // Consume the literal type
                    let byte = byte_opt.unwrap();
    
                    // The following opcodes are expected to be a literal value
                    op_type = VCROperationKind::Value;
                },
                // Treat the opcode as a literal value
                VCROperationKind::Value => {
                    if byte_opt.is_some() {
                        let byte = byte_opt.unwrap();
                        // Record another of the literal's bytes
                        literal[lit_digit] = *byte;
        
                        // Continue consuming the literal
                        if lit_digit >= 3 {
                            let num = f32::from_bits(as_u32_be(&literal));
                            self.values.push(num);
                            //println!("LIT {:?}", num);
                            //println!("Values: {:?}\n", self.values);
                            op_type = VCROperationKind::Generic;
                            lit_digit = 0;
                        } else {
                            lit_digit += 1;
                        }
                    } else {
                        break;
                    }
                },
                // Treat the opcode as a generic command
                _ => {
                    if byte_opt.is_some() {
                        let byte = byte_opt.unwrap();
                        match byte {
                            0x01 => op_type = VCROperationKind::Type,
                            0x02 => swap(&mut self.values),
                            0x03 => del(&mut self.values),
                            0x04 => copy(&mut self.values),
                            0x10 => add(&mut self.values),
                            0x11 => sub(&mut self.values),
                            0x12 => mul(&mut self.values),
                            0x19 => floor(&mut self.values),
                            _ => break
                        }
                        // DEBUG - show the value stack upon every generic command
                        if let VCROperationKind::Generic = op_type {
                            //println!("Values: {:?}\n", self.values);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        self.instructions.clear();
        &self.values
    }
}


fn add(values: &mut Vec<f32>) {
    //println!("ADD");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a + b);
    } else {
        //println!("Not enough values.");
    }
}

fn sub(values: &mut Vec<f32>) {
    //println!("SUB");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a - b);
    } else {
        //println!("Not enough values.");
    }
}

fn mul(values: &mut Vec<f32>) {
    //println!("MUL");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(a * b);
    } else {
        //println!("Not enough values.");
    }
}

fn del(values: &mut Vec<f32>) {
    //println!("DEL");
    values.pop();
}

fn copy(values: &mut Vec<f32>) {
    //println!("COPY");
    let a_opt = values.pop();
    if a_opt.is_some() {
        let a = a_opt.unwrap();
        values.push(a);
        values.push(a);
    } else {
        //println!("Not enough values.");
    }
}

fn swap(values: &mut Vec<f32>) {
    //println!("SWAP");
    let b_opt = values.pop();
    let a_opt = values.pop();
    if a_opt.is_some() && b_opt.is_some() {
        let a = a_opt.unwrap();
        let b = b_opt.unwrap();
        values.push(b);
        values.push(a);
    } else {
        //println!("Not enough values.");
    }
}

fn floor(values: &mut Vec<f32>) {
    //println!("FLOOR");
    let a_opt = values.pop();
    if a_opt.is_some() {
        let a = a_opt.unwrap();
        let a = a.floor();
        values.push(a);
    }
}

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}