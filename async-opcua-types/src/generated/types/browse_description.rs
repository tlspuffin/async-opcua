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
///https://reference.opcfoundation.org/v105/Core/docs/Part4/5.9.2/#5.9.2.2
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BrowseDescription {
    pub node_id: opcua::types::node_id::NodeId,
    pub browse_direction: super::enums::BrowseDirection,
    pub reference_type_id: opcua::types::node_id::NodeId,
    pub include_subtypes: bool,
    pub node_class_mask: u32,
    pub result_mask: u32,
}
impl opcua::types::MessageInfo for BrowseDescription {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseDescription_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseDescription_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::BrowseDescription_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::BrowseDescription
    }
}
