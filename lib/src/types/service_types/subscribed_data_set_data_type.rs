// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SubscribedDataSetDataType {}
impl crate::types::MessageInfo for SubscribedDataSetDataType {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::SubscribedDataSetDataType_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<SubscribedDataSetDataType> for SubscribedDataSetDataType {
    fn byte_len(&self) -> usize {
        0usize
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        Ok(0)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        Ok(Self {})
    }
}
