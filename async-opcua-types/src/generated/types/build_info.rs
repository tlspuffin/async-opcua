// This file was autogenerated from schemas/1.05/Opc.Ua.NodeSet2.Services.xml by async-opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v105/Core/docs/Part5/12.4
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuildInfo {
    pub product_uri: opcua::types::string::UAString,
    pub manufacturer_name: opcua::types::string::UAString,
    pub product_name: opcua::types::string::UAString,
    pub software_version: opcua::types::string::UAString,
    pub build_number: opcua::types::string::UAString,
    pub build_date: opcua::types::data_types::UtcTime,
}
impl opcua::types::MessageInfo for BuildInfo {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BuildInfo_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::BuildInfo
    }
}
