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
///https://reference.opcfoundation.org/v105/Core/docs/Part5/12.2.12/#12.2.12.2
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CurrencyUnitType {
    pub numeric_code: i16,
    pub exponent: i8,
    pub alphabetic_code: opcua::types::string::UAString,
    pub currency: opcua::types::localized_text::LocalizedText,
}
impl opcua::types::MessageInfo for CurrencyUnitType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CurrencyUnitType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CurrencyUnitType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::CurrencyUnitType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::CurrencyUnitType
    }
}
