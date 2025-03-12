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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ReferenceListEntryDataType {
    pub reference_type: opcua::types::node_id::NodeId,
    pub is_forward: bool,
    pub target_node: opcua::types::expanded_node_id::ExpandedNodeId,
}
impl opcua::types::MessageInfo for ReferenceListEntryDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReferenceListEntryDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReferenceListEntryDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ReferenceListEntryDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::ReferenceListEntryDataType
    }
}
