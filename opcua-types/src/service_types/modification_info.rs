// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
pub struct ModificationInfo {
    pub modification_time: opcua::types::date_time::DateTime,
    pub update_type: super::enums::HistoryUpdateType,
    pub user_name: opcua::types::string::UAString,
}
impl opcua::types::MessageInfo for ModificationInfo {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModificationInfo_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModificationInfo_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::ModificationInfo_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for ModificationInfo {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.modification_time.byte_len();
        size += self.update_type.byte_len();
        size += self.user_name.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.modification_time.encode(stream)?;
        size += self.update_type.encode(stream)?;
        size += self.user_name.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            modification_time: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            update_type: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            user_name: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
        })
    }
}
