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
pub struct SimpleAttributeOperand {
    pub type_definition_id: opcua::types::node_id::NodeId,
    pub browse_path: Option<Vec<opcua::types::qualified_name::QualifiedName>>,
    pub attribute_id: u32,
    pub index_range: opcua::types::string::UAString,
}
impl opcua::types::MessageInfo for SimpleAttributeOperand {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SimpleAttributeOperand_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SimpleAttributeOperand_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SimpleAttributeOperand_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for SimpleAttributeOperand {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.type_definition_id.byte_len();
        size += self.browse_path.byte_len();
        size += self.attribute_id.byte_len();
        size += self.index_range.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.type_definition_id.encode(stream)?;
        size += self.browse_path.encode(stream)?;
        size += self.attribute_id.encode(stream)?;
        size += self.index_range.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let type_definition_id = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let browse_path = <Option<
            Vec<opcua::types::qualified_name::QualifiedName>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let attribute_id = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let index_range = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            type_definition_id,
            browse_path,
            attribute_id,
            index_range,
        })
    }
}
