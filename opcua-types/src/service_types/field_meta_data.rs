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
pub struct FieldMetaData {
    pub name: opcua::types::string::UAString,
    pub description: opcua::types::localized_text::LocalizedText,
    pub field_flags: super::enums::DataSetFieldFlags,
    pub built_in_type: u8,
    pub data_type: opcua::types::node_id::NodeId,
    pub value_rank: i32,
    pub array_dimensions: Option<Vec<u32>>,
    pub max_string_length: u32,
    pub data_set_field_id: opcua::types::guid::Guid,
    pub properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
}
impl opcua::types::MessageInfo for FieldMetaData {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FieldMetaData_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FieldMetaData_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::FieldMetaData_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for FieldMetaData {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.name.byte_len();
        size += self.description.byte_len();
        size += self.field_flags.byte_len();
        size += self.built_in_type.byte_len();
        size += self.data_type.byte_len();
        size += self.value_rank.byte_len();
        size += self.array_dimensions.byte_len();
        size += self.max_string_length.byte_len();
        size += self.data_set_field_id.byte_len();
        size += self.properties.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.name.encode(stream)?;
        size += self.description.encode(stream)?;
        size += self.field_flags.encode(stream)?;
        size += self.built_in_type.encode(stream)?;
        size += self.data_type.encode(stream)?;
        size += self.value_rank.encode(stream)?;
        size += self.array_dimensions.encode(stream)?;
        size += self.max_string_length.encode(stream)?;
        size += self.data_set_field_id.encode(stream)?;
        size += self.properties.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let name = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let description = <opcua::types::localized_text::LocalizedText as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let field_flags = <super::enums::DataSetFieldFlags as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let built_in_type = <u8 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let data_type = <opcua::types::node_id::NodeId as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let value_rank = <i32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let array_dimensions = <Option<
            Vec<u32>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        let max_string_length = <u32 as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let data_set_field_id = <opcua::types::guid::Guid as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let properties = <Option<
            Vec<super::key_value_pair::KeyValuePair>,
        > as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
        Ok(Self {
            name,
            description,
            field_flags,
            built_in_type,
            data_type,
            value_rank,
            array_dimensions,
            max_string_length,
            data_set_field_id,
            properties,
        })
    }
}
