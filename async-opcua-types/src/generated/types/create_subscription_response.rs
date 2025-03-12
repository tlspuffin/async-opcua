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
///https://reference.opcfoundation.org/v105/Core/docs/Part4/5.14.2/#5.14.2.2
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CreateSubscriptionResponse {
    pub response_header: opcua::types::response_header::ResponseHeader,
    pub subscription_id: opcua::types::IntegerId,
    pub revised_publishing_interval: opcua::types::data_types::Duration,
    pub revised_lifetime_count: opcua::types::Counter,
    pub revised_max_keep_alive_count: opcua::types::Counter,
}
impl opcua::types::MessageInfo for CreateSubscriptionResponse {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSubscriptionResponse_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSubscriptionResponse_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CreateSubscriptionResponse_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::CreateSubscriptionResponse
    }
}
