// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct ModifySubscriptionResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub revised_publishing_interval: f64,
    pub revised_lifetime_count: u32,
    pub revised_max_keep_alive_count: u32,
}
impl opcua::types::MessageInfo for ModifySubscriptionResponse {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModifySubscriptionResponse_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModifySubscriptionResponse_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModifySubscriptionResponse_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ModifySubscriptionResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.revised_publishing_interval.byte_len();
        size += self.revised_lifetime_count.byte_len();
        size += self.revised_max_keep_alive_count.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.revised_publishing_interval.encode(stream)?;
        size += self.revised_lifetime_count.encode(stream)?;
        size += self.revised_max_keep_alive_count.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let response_header: opcua::types::response_header::ResponseHeader = opcua::types::BinaryEncodable::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = response_header.request_handle;
        Ok(Self {
            response_header,
            revised_publishing_interval: opcua::types::BinaryEncodable::decode(
                    stream,
                    decoding_options,
                )
                .map_err(|e| e.with_request_handle(__request_handle))?,
            revised_lifetime_count: opcua::types::BinaryEncodable::decode(
                    stream,
                    decoding_options,
                )
                .map_err(|e| e.with_request_handle(__request_handle))?,
            revised_max_keep_alive_count: opcua::types::BinaryEncodable::decode(
                    stream,
                    decoding_options,
                )
                .map_err(|e| e.with_request_handle(__request_handle))?,
        })
    }
}
