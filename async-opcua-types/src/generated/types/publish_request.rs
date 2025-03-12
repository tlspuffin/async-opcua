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
///https://reference.opcfoundation.org/v105/Core/docs/Part4/5.14.5/#5.14.5.2
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PublishRequest {
    pub request_header: opcua::types::request_header::RequestHeader,
    pub subscription_acknowledgements:
        Option<Vec<super::subscription_acknowledgement::SubscriptionAcknowledgement>>,
}
impl opcua::types::MessageInfo for PublishRequest {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishRequest_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishRequest_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::PublishRequest_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::PublishRequest
    }
}
