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
pub struct SecurityGroupDataType {
    pub name: opcua::types::string::UAString,
    pub security_group_folder: Option<Vec<opcua::types::string::UAString>>,
    pub key_lifetime: f64,
    pub security_policy_uri: opcua::types::string::UAString,
    pub max_future_key_count: u32,
    pub max_past_key_count: u32,
    pub security_group_id: opcua::types::string::UAString,
    pub role_permissions: Option<Vec<super::role_permission_type::RolePermissionType>>,
    pub group_properties: Option<Vec<super::key_value_pair::KeyValuePair>>,
}
impl opcua::types::MessageInfo for SecurityGroupDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SecurityGroupDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SecurityGroupDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SecurityGroupDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::SecurityGroupDataType
    }
}
