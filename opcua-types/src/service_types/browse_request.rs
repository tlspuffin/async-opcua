// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct BrowseRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub view: super::view_description::ViewDescription,
    pub requested_max_references_per_node: u32,
    pub nodes_to_browse: Option<Vec<super::browse_description::BrowseDescription>>,
}
impl opcua::types::MessageInfo for BrowseRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseRequest_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for BrowseRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.request_header.byte_len();
        size += self.view.byte_len();
        size += self.requested_max_references_per_node.byte_len();
        size += self.nodes_to_browse.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.request_header.encode(stream)?;
        size += self.view.encode(stream)?;
        size += self.requested_max_references_per_node.encode(stream)?;
        size += self.nodes_to_browse.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let request_header = <opcua::types::request_header::RequestHeader as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let __request_handle = request_header.request_handle;
        let view = <super::view_description::ViewDescription as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let requested_max_references_per_node = <u32 as opcua::types::BinaryEncoder>::decode(
                stream,
                decoding_options,
            )
            .map_err(|e| e.with_request_handle(__request_handle))?;
        let nodes_to_browse = <Option<
            Vec<super::browse_description::BrowseDescription>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
            .map_err(|e| e.with_request_handle(__request_handle))?;
        Ok(Self {
            request_header,
            view,
            requested_max_references_per_node,
            nodes_to_browse,
        })
    }
}
