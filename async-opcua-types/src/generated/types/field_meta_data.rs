// This file was autogenerated from schemas/1.05/Opc.Ua.Types.bsd by async-opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[derive(Debug, Clone, PartialEq, opcua::types::BinaryEncodable, opcua::types::BinaryDecodable)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
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
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::FieldMetaData
    }
}
