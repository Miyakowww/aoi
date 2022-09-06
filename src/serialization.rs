use crate::opcodes::*;
use crate::AoArg;
use crate::AoProgram;
use crate::AoType;

/// Serializer for serializing and deserializing the Aoi assembly.
pub enum AoAsmSerializer {}

impl AoAsmSerializer {
    fn serialize_type(value: &AoType) -> Vec<u8> {
        let mut result = Vec::new();
        match value {
            AoType::AoBool(value) => {
                result.push(0x01);
                result.push(if *value { 0x01 } else { 0x00 });
            }
            AoType::AoInt(value) => {
                result.push(0x02);
                result.extend_from_slice(&value.to_le_bytes());
            }
            AoType::AoFloat(value) => {
                result.push(0x03);
                result.extend_from_slice(&value.to_le_bytes());
            }
            AoType::AoPtr(value) => {
                result.push(0x04);
                result.extend_from_slice(&value.to_le_bytes());
            }
            AoType::AoString(value) => {
                result.push(0x05);
                result.extend_from_slice(&(value.len() as u32).to_le_bytes());
                result.extend_from_slice(value.as_bytes());
            }
        }
        result
    }

    fn serialize_arg(value: &AoArg) -> Vec<u8> {
        let mut result = Vec::new();
        match value {
            AoArg::PC => {
                result.push(0x01);
            }
            AoArg::DP => {
                result.push(0x02);
            }
            AoArg::MP => {
                result.push(0x03);
            }
            AoArg::DSB => {
                result.push(0x11);
            }
            AoArg::DST => {
                result.push(0x12);
            }
            AoArg::CA => {
                result.push(0x21);
            }
            AoArg::DS => {
                result.push(0xE1);
            }
            AoArg::MEM => {
                result.push(0xE2);
            }
            AoArg::Imm(value) => {
                result.push(0xFF);
                result.extend_from_slice(&AoAsmSerializer::serialize_type(value));
            }
        }
        result
    }

    fn serialize_opcode(opcode: &dyn AoOpcode) -> Vec<u8> {
        let mut result = vec![opcode.get_id()];
        match opcode.get_args() {
            OpcodeArgType::NoArg => (),
            OpcodeArgType::u8(value) => {
                result.extend_from_slice(value.to_le_bytes().as_ref());
            }
            OpcodeArgType::i32(value) => {
                result.extend_from_slice(value.to_le_bytes().as_ref());
            }
            OpcodeArgType::u32(value) => {
                result.extend_from_slice(value.to_le_bytes().as_ref());
            }
            OpcodeArgType::bool(value) => {
                result.push(if value { 0x01 } else { 0x00 });
            }
            OpcodeArgType::AoArg(value) => {
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(&value));
            }
            OpcodeArgType::AoArg2(value1, value2) => {
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(&value1));
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(&value2));
            }
        }
        result
    }

    pub fn serialize(asm: &[Box<dyn AoOpcode>]) -> Vec<u8> {
        let mut result = Vec::new();
        for opcode in asm {
            result.extend_from_slice(&AoAsmSerializer::serialize_opcode(opcode.as_ref()));
        }
        result
    }

    fn deserialize_type(bin: &[u8], offset: &mut usize) -> Option<AoType> {
        match bin[*offset] {
            0x01 => {
                *offset += 2;
                Some(AoType::AoBool(bin[*offset - 1] != 0x00))
            }
            0x02 => {
                *offset += 5;
                Some(AoType::AoInt(i32::from_le_bytes(
                    bin[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x03 => {
                *offset += 5;
                Some(AoType::AoFloat(f32::from_le_bytes(
                    bin[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x04 => {
                *offset += 5;
                Some(AoType::AoPtr(u32::from_le_bytes(
                    bin[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x05 => {
                let str_len =
                    u32::from_le_bytes(bin[*offset + 1..*offset + 5].try_into().unwrap()) as usize;
                *offset += 5 + str_len;
                Some(AoType::AoString(
                    String::from_utf8(bin[*offset - str_len..*offset].to_vec()).unwrap(),
                ))
            }
            _ => None,
        }
    }

    fn deserialize_arg(bin: &[u8], offset: &mut usize) -> Option<AoArg> {
        *offset += 1;
        match bin[*offset - 1] {
            0x01 => Some(AoArg::PC),
            0x02 => Some(AoArg::DP),
            0x03 => Some(AoArg::MP),
            0x11 => Some(AoArg::DSB),
            0x12 => Some(AoArg::DST),
            0x21 => Some(AoArg::CA),
            0xE1 => Some(AoArg::DS),
            0xE2 => Some(AoArg::MEM),
            0xFF => Some(AoArg::Imm(
                AoAsmSerializer::deserialize_type(bin, offset).unwrap(),
            )),
            _ => None,
        }
    }

    fn deserialize_opcode(bin: &[u8], offset: &mut usize) -> Option<Box<dyn AoOpcode>> {
        *offset += 1;
        let opcode = create_opcode_by_id(bin[*offset - 1]);
        opcode.as_ref()?;
        let mut opcode = opcode.unwrap();

        match opcode.get_args() {
            OpcodeArgType::NoArg => (),
            OpcodeArgType::u8(_) => {
                *offset += 1;
                opcode.set_args(OpcodeArgType::u8(bin[*offset - 1]));
            }
            OpcodeArgType::i32(_) => {
                *offset += 4;
                opcode.set_args(OpcodeArgType::i32(i32::from_le_bytes(
                    bin[*offset - 4..*offset].try_into().unwrap(),
                )));
            }
            OpcodeArgType::u32(_) => {
                *offset += 4;
                opcode.set_args(OpcodeArgType::u32(u32::from_le_bytes(
                    bin[*offset - 4..*offset].try_into().unwrap(),
                )));
            }
            OpcodeArgType::bool(_) => {
                *offset += 1;
                opcode.set_args(OpcodeArgType::bool(bin[*offset - 1] != 0x00));
            }
            OpcodeArgType::AoArg(_) => {
                let value = AoAsmSerializer::deserialize_arg(bin, offset).unwrap();
                opcode.set_args(OpcodeArgType::AoArg(value));
            }
            OpcodeArgType::AoArg2(_, _) => {
                let value1 = AoAsmSerializer::deserialize_arg(bin, offset).unwrap();
                let value2 = AoAsmSerializer::deserialize_arg(bin, offset).unwrap();
                opcode.set_args(OpcodeArgType::AoArg2(value1, value2));
            }
        }

        Some(opcode)
    }

    pub fn deserialize(value: &[u8]) -> Option<AoProgram> {
        let mut result = Vec::new();
        let mut offset = 0;
        while offset < value.len() {
            let opcode = AoAsmSerializer::deserialize_opcode(value, &mut offset);
            if let Some(opcode) = opcode {
                result.push(opcode);
            } else {
                return None;
            }
        }
        Some(result)
    }
}
