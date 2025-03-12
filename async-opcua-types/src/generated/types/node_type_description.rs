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
///https://reference.opcfoundation.org/v105/Core/docs/Part4/5.10.3/#5.10.3.1
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NodeTypeDescription {
    pub type_definition_node: opcua::types::expanded_node_id::ExpandedNodeId,
    pub include_sub_types: bool,
    pub data_to_return: Option<Vec<super::query_data_description::QueryDataDescription>>,
}
impl opcua::types::MessageInfo for NodeTypeDescription {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::NodeTypeDescription_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::NodeTypeDescription_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::NodeTypeDescription_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::NodeTypeDescription
    }
}
