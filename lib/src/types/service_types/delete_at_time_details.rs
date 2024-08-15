// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeleteAtTimeDetails {
    pub node_id: crate::types::node_id::NodeId,
    pub req_times: Option<Vec<crate::types::date_time::DateTime>>,
}
impl crate::types::BinaryEncoder<DeleteAtTimeDetails> for DeleteAtTimeDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.node_id.byte_len();
        size += self.req_times.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.node_id.encode(stream)?;
        size += self.req_times.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let node_id = <crate::types::node_id::NodeId as crate::types::BinaryEncoder<
            crate::types::node_id::NodeId,
        >>::decode(stream, decoding_options)?;
        let req_times =
            <Option<Vec<crate::types::date_time::DateTime>> as crate::types::BinaryEncoder<
                Option<Vec<crate::types::date_time::DateTime>>,
            >>::decode(stream, decoding_options)?;
        Ok(Self { node_id, req_times })
    }
}
