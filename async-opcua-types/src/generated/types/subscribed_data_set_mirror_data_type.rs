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
///https://reference.opcfoundation.org/v105/Core/docs/Part14/6.2.10/#6.2.10.3.4
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SubscribedDataSetMirrorDataType {
    pub parent_node_name: opcua::types::string::UAString,
    pub role_permissions: Option<Vec<super::role_permission_type::RolePermissionType>>,
}
impl opcua::types::MessageInfo for SubscribedDataSetMirrorDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscribedDataSetMirrorDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscribedDataSetMirrorDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SubscribedDataSetMirrorDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::SubscribedDataSetMirrorDataType
    }
}
