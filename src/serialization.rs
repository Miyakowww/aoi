use crate::runtime::opcode::{AoArg, AoOpCode};
use crate::runtime::types::AoType;

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
            AoArg::DSB => {
                result.push(0x01);
            }
            AoArg::DST => {
                result.push(0x02);
            }
            AoArg::PC => {
                result.push(0x03);
            }
            AoArg::DP => {
                result.push(0x04);
            }
            AoArg::CA => {
                result.push(0x05);
            }
            AoArg::DS => {
                result.push(0x06);
            }
            AoArg::GVS => {
                result.push(0x07);
            }
            AoArg::Imm(value) => {
                result.push(0x08);
                result.extend_from_slice(&AoAsmSerializer::serialize_type(value));
            }
        }
        result
    }

    fn serialize_opcode(opcode: &AoOpCode) -> Vec<u8> {
        let mut result = Vec::new();
        match &opcode {
            AoOpCode::NOP => {
                result.push(0x00);
            }

            AoOpCode::CALL(addr) => {
                result.push(0x10);
                result.extend_from_slice(&addr.to_le_bytes());
            }
            AoOpCode::RET => {
                result.push(0x11);
            }
            AoOpCode::JMP(addr) => {
                result.push(0x12);
                result.extend_from_slice(&addr.to_le_bytes());
            }
            AoOpCode::JT(addr) => {
                result.push(0x13);
                result.extend_from_slice(&addr.to_le_bytes());
            }
            AoOpCode::JF(addr) => {
                result.push(0x14);
                result.extend_from_slice(&addr.to_le_bytes());
            }

            AoOpCode::MOV(dst, src) => {
                result.push(0x20);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(dst));
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::INT(id) => {
                result.push(0x21);
                result.extend_from_slice(&id.to_le_bytes());
            }

            AoOpCode::PUSH(src) => {
                result.push(0x30);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::POP(save) => {
                result.push(0x31);
                result.push(if *save { 0x01 } else { 0x00 });
            }

            AoOpCode::ADD(src) => {
                result.push(0x40);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::SUB(src) => {
                result.push(0x41);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::MUL(src) => {
                result.push(0x42);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::DIV(src) => {
                result.push(0x43);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::REM(src) => {
                result.push(0x44);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::INC => {
                result.push(0x45);
            }
            AoOpCode::DEC => {
                result.push(0x46);
            }

            AoOpCode::AND(src) => {
                result.push(0x50);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::OR(src) => {
                result.push(0x51);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::XOR(src) => {
                result.push(0x52);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::NOT => {
                result.push(0x53);
            }

            AoOpCode::BAND(src) => {
                result.push(0x60);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::BOR(src) => {
                result.push(0x61);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::BXOR(src) => {
                result.push(0x62);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::BNOT => {
                result.push(0x63);
            }

            AoOpCode::SHL(src) => {
                result.push(0x70);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::SHR(src) => {
                result.push(0x71);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }

            AoOpCode::EQU(src) => {
                result.push(0x80);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::NEQ(src) => {
                result.push(0x81);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::GT(src) => {
                result.push(0x82);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::LT(src) => {
                result.push(0x83);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::GE(src) => {
                result.push(0x84);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }
            AoOpCode::LE(src) => {
                result.push(0x85);
                result.extend_from_slice(&AoAsmSerializer::serialize_arg(src));
            }

            AoOpCode::CSI => {
                result.push(0x90);
            }
            AoOpCode::CSF => {
                result.push(0x91);
            }
            AoOpCode::CSP => {
                result.push(0x92);
            }
            AoOpCode::CSS => {
                result.push(0x93);
            }

            AoOpCode::ISB => {
                result.push(0xA0);
            }
            AoOpCode::ISI => {
                result.push(0xA1);
            }
            AoOpCode::ISF => {
                result.push(0xA2);
            }
            AoOpCode::ISP => {
                result.push(0xA3);
            }
            AoOpCode::ISS => {
                result.push(0xA4);
            }

            AoOpCode::ARG(offset) => {
                result.push(0xB0);
                result.extend_from_slice(&offset.to_le_bytes());
            }
            AoOpCode::CNF(argc) => {
                result.push(0xB1);
                result.extend_from_slice(&argc.to_le_bytes());
            }
        }
        result
    }

    pub fn serialize(asm: &Vec<AoOpCode>) -> Vec<u8> {
        let mut result = Vec::new();
        for opcode in asm {
            result.extend_from_slice(&AoAsmSerializer::serialize_opcode(opcode));
        }
        result
    }

    fn deserialize_type(value: &[u8], offset: &mut usize) -> Option<AoType> {
        match value[*offset] {
            0x01 => {
                *offset += 2;
                Some(AoType::AoBool(value[*offset - 1] != 0x00))
            }
            0x02 => {
                *offset += 5;
                Some(AoType::AoInt(i32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x03 => {
                *offset += 5;
                Some(AoType::AoFloat(f32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x04 => {
                *offset += 5;
                Some(AoType::AoPtr(u32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x05 => {
                let str_len =
                    u32::from_le_bytes(value[*offset + 1..*offset + 5].try_into().unwrap())
                        as usize;
                *offset += 5 + str_len;
                Some(AoType::AoString(
                    String::from_utf8(value[*offset - str_len..*offset].to_vec()).unwrap(),
                ))
            }
            _ => None,
        }
    }

    fn deserialize_arg(value: &[u8], offset: &mut usize) -> Option<AoArg> {
        *offset += 1;
        match value[*offset - 1] {
            0x01 => Some(AoArg::DSB),
            0x02 => Some(AoArg::DST),
            0x03 => Some(AoArg::PC),
            0x04 => Some(AoArg::DP),
            0x05 => Some(AoArg::CA),
            0x06 => Some(AoArg::DS),
            0x07 => Some(AoArg::GVS),
            0x08 => Some(AoArg::Imm(
                AoAsmSerializer::deserialize_type(value, offset).unwrap(),
            )),
            _ => None,
        }
    }

    fn deserialize_opcode(value: &[u8], offset: &mut usize) -> Option<AoOpCode> {
        *offset += 1;
        match value[*offset - 1] {
            0x00 => Some(AoOpCode::NOP),

            0x10 => {
                *offset += 4;
                Some(AoOpCode::CALL(u32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x11 => Some(AoOpCode::RET),
            0x12 => {
                *offset += 4;
                Some(AoOpCode::JMP(u32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x13 => {
                *offset += 4;
                Some(AoOpCode::JT(u32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0x14 => {
                *offset += 4;
                Some(AoOpCode::JF(u32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }

            0x20 => {
                let dst = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::MOV(dst, src))
            }
            0x21 => {
                *offset += 1;
                Some(AoOpCode::INT(value[*offset - 1]))
            }

            0x30 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::PUSH(src))
            }
            0x31 => {
                *offset += 1;
                Some(AoOpCode::POP(value[*offset - 1] != 0x00))
            }

            0x40 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::ADD(src))
            }
            0x41 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::SUB(src))
            }
            0x42 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::MUL(src))
            }
            0x43 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::DIV(src))
            }
            0x44 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::REM(src))
            }
            0x45 => Some(AoOpCode::INC),
            0x46 => Some(AoOpCode::DEC),

            0x50 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::AND(src))
            }
            0x51 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::OR(src))
            }
            0x52 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::XOR(src))
            }
            0x53 => Some(AoOpCode::NOT),

            0x60 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::BAND(src))
            }
            0x61 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::BOR(src))
            }
            0x62 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::BXOR(src))
            }
            0x63 => Some(AoOpCode::BNOT),

            0x70 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::SHL(src))
            }
            0x71 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::SHR(src))
            }

            0x80 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::EQU(src))
            }
            0x81 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::NEQ(src))
            }
            0x82 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::GT(src))
            }
            0x83 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::LT(src))
            }
            0x84 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::GE(src))
            }
            0x85 => {
                let src = AoAsmSerializer::deserialize_arg(value, offset).unwrap();
                Some(AoOpCode::LE(src))
            }
            
            0x90 => Some(AoOpCode::CSI),
            0x91 => Some(AoOpCode::CSF),
            0x92 => Some(AoOpCode::CSP),
            0x93 => Some(AoOpCode::CSS),

            0xA0 => Some(AoOpCode::ISB),
            0xA1 => Some(AoOpCode::ISI),
            0xA2 => Some(AoOpCode::ISF),
            0xA3 => Some(AoOpCode::ISP),
            0xA4 => Some(AoOpCode::ISS),

            0xB0 => {
                *offset += 4;
                Some(AoOpCode::ARG(u32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }
            0xB1 => {
                *offset += 4;
                Some(AoOpCode::CNF(u32::from_le_bytes(
                    value[*offset - 4..*offset].try_into().unwrap(),
                )))
            }

            _ => None,
        }
    }

    pub fn deserialize(value: &[u8]) -> Option<Vec<AoOpCode>> {
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
