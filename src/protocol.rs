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

#[derive(Debug, PartialEq)]
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
struct TsDelete {
    name: String,
}

impl<'a, T> TsPacket<'a, T>
where
    T: Serialize,
    T: Deserialize<'a>,
{
    pub fn from_binary(b: &'a Vec<u8>) -> Result<TsPacket<'a, T>, Box<bincode::ErrorKind>> {
        if b.len() < 9 {
            return Err(Box::new(bincode::ErrorKind::Custom(
                "Not enough bytes".to_string(),
            )));
        }
        let header: TsHeader = match bincode::deserialize(&b[..9]) {
            Ok(h) => h,
            Err(e) => return Err(e),
        };
        let packet = match bincode::deserialize(&b[9..]) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        return Ok(TsPacket {
            header: header,
            packet: packet,
            phantom: PhantomData,
        });
    }

    pub fn to_binary(&self) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        let mut h = match bincode::serialize(&self.header) {
            Ok(h) => h,
            Err(e) => return Err(e),
        };
        let mut p = match bincode::serialize(&self.packet) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        h.append(&mut p);
        return Ok(h);
    }
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
    fn test_ts_packet_to_binary() {
        let tsp = TsPacket {
            header: TsHeader {
                byte: OpCode::OpTsCreate as u8,
                size: 10,
            },
            packet: TsCreate {
                name: "ts-test".to_string(),
                retention: 3000,
            },
            phantom: PhantomData,
        };
        let binary = tsp.to_binary().unwrap();
        let decoded = TsPacket::from_binary(&binary);
        assert_eq!(tsp, decoded.unwrap());
    }

    #[test]
    fn test_ts_packet_from_binary() {
        test_ts_packet_to_binary();
    }
}
