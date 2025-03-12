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
///https://reference.opcfoundation.org/v105/Core/docs/Part4/7.25.3
#[derive(Debug, Clone, PartialEq, Default)]
pub struct EventNotificationList {
    pub events: Option<Vec<super::event_field_list::EventFieldList>>,
}
impl opcua::types::MessageInfo for EventNotificationList {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventNotificationList_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventNotificationList_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::EventNotificationList_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::EventNotificationList
    }
}
