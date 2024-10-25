// This file was autogenerated from schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua { pub use crate as types; }#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
#[cfg_attr(feature = "xml", derive(opcua::types::FromXml))]
#[derive(Default)]
pub struct IssuedIdentityToken {
    pub policy_id: opcua::types::string::UAString,
    pub token_data: opcua::types::byte_string::ByteString,
    pub encryption_algorithm: opcua::types::string::UAString,
}
impl opcua::types::MessageInfo for IssuedIdentityToken {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::IssuedIdentityToken_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::IssuedIdentityToken_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::IssuedIdentityToken_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncoder for IssuedIdentityToken {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.policy_id.byte_len();
        size += self.token_data.byte_len();
        size += self.encryption_algorithm.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.policy_id.encode(stream)?;
        size += self.token_data.encode(stream)?;
        size += self.encryption_algorithm.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        let policy_id = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let token_data = <opcua::types::byte_string::ByteString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        let encryption_algorithm = <opcua::types::string::UAString as opcua::types::BinaryEncoder>::decode(
            stream,
            decoding_options,
        )?;
        Ok(Self {
            policy_id,
            token_data,
            encryption_algorithm,
        })
    }
}
