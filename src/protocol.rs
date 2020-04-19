// BSD 2-Clause License
//
// Copyright (c) 2020, Andrea Giacomo Baldan
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
//   list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
//   this list of conditions and the following disclaimer in the documentation
//   and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    OpTsCreate,
    OpTsDelete,
    OpTsAddPoint,
    OpTsMaddPoint,
    OpTsQuery,
}

enum Status {
    TsOk,
    TsNotFount,
    TsExists,
    TsUnknownCmd,
}

trait AsOpcode {
    fn as_opcode(self) -> Option<OpCode>;
}

impl AsOpcode for u8 {
    fn as_opcode(self) -> Option<OpCode> {
        match self {
            0 => Some(OpCode::OpTsCreate),
            1 => Some(OpCode::OpTsDelete),
            2 => Some(OpCode::OpTsAddPoint),
            3 => Some(OpCode::OpTsMaddPoint),
            4 => Some(OpCode::OpTsQuery),
            _ => None,
        }
    }
}

struct TsPacket<'a, T>
where
    T: Serialize,
    T: Deserialize<'a>,
{
    header: TsHeader,
    packet: T,
    phantom: PhantomData<&'a T>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TsHeader {
    byte: u8,
    size: usize,
}

impl TsHeader {
    pub fn opcode(&self) -> Option<OpCode> {
        return (self.byte >> 4).as_opcode();
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TsCreate {
    name: String,
    retention: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TsDelete(String);

pub fn serialize<T: Serialize>(o: &T) -> Vec<u8> {
    return bincode::serialize(o).unwrap();
}

pub fn deserialize<'a, T: Deserialize<'a>>(b: &'a Vec<u8>) -> T {
    return bincode::deserialize(&b[..]).unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_header_opcode() {
        let h = TsHeader {
            byte: 0x00,
            size: 0,
        };
        assert_eq!(h.opcode(), Some(OpCode::OpTsCreate));
    }

    #[test]
    fn test_header_serialize() {
        let c = TsHeader {
            byte: 0x01,
            size: 255,
        };
        let b = serialize(&c);
        assert_eq!(b.len(), 9);
    }

    #[test]
    fn test_header_deserialize() {
        let c = TsHeader {
            byte: 0x01,
            size: 255,
        };
        let b = serialize(&c);
        assert_eq!(b.len(), 9);
        let d: TsHeader = deserialize(&b);
        assert_eq!(d, c);
    }

    #[test]
    fn test_create_serialize() {
        let c = TsCreate {
            name: "ts-test".to_string(),
            retention: 3000,
        };
        let b = serialize(&c);
        assert_eq!(b.len(), 19);
    }

    #[test]
    fn test_create_deserialize() {
        let c = TsCreate {
            name: "ts-test".to_string(),
            retention: 3000,
        };
        let b = serialize(&c);
        assert_eq!(b.len(), 19);
        let d: TsCreate = deserialize(&b);
        assert_eq!(d, c);
    }

    #[test]
    fn test_delete_serialize() {
        let c = TsDelete {
            0: "ts-test".to_string(),
        };
        let b = serialize(&c);
        assert_eq!(b.len(), 15);
    }

    #[test]
    fn test_delete_deserialize() {
        let c = TsDelete {
            0: "ts-test".to_string(),
        };
        let b = serialize(&c);
        assert_eq!(b.len(), 15);
        let d: TsDelete = deserialize(&b);
        assert_eq!(d, c);
    }
}
