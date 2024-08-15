// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RegisterNodesResponse {
    pub response_header: crate::types::response_header::ResponseHeader,
    pub registered_node_ids: Option<Vec<crate::types::node_id::NodeId>>,
}
impl crate::types::MessageInfo for RegisterNodesResponse {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::RegisterNodesResponse_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<RegisterNodesResponse> for RegisterNodesResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.registered_node_ids.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.registered_node_ids.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let response_header =
            <crate::types::response_header::ResponseHeader as crate::types::BinaryEncoder<
                crate::types::response_header::ResponseHeader,
            >>::decode(stream, decoding_options)?;
        let registered_node_ids =
            <Option<Vec<crate::types::node_id::NodeId>> as crate::types::BinaryEncoder<
                Option<Vec<crate::types::node_id::NodeId>>,
            >>::decode(stream, decoding_options)?;
        Ok(Self {
            response_header,
            registered_node_ids,
        })
    }
}
