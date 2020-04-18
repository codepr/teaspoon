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

trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(buf: &Vec<u8>) -> Self;
}

struct TsHeader {
    byte: u8,
    size: usize,
}

struct TsPacket<T: Serializable> {
    header: TsHeader,
    packet: T,
}

struct TsCreate {
    name: String,
    retention: i32,
}

impl Serializable for TsCreate {
    fn serialize(&self) -> Vec<u8> {
        // TODO
        return Vec::new();
    }

    fn deserialize(buf: &Vec<u8>) -> TsCreate {
        // TODO
        return TsCreate {};
    }
}

#[test]
fn test_create_serialize() {
    let c = TsCreate { "ts-test".to_string(), 3000 };
    let b = c.serialize();
    assert_eq!(b.len(), 14);
}

#[test]
fn test_create_deserialize() {
    let c = TsCreate { "ts-test".to_string(), 3000 };
    let b = c.serialize();
    assert_eq!(b.len(), 14);
    let d = TsCreate::deserialize(b);
    assert_eq!(d, c);
}
