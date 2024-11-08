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
pub struct AxisInformation {
    pub engineering_units: super::eu_information::EUInformation,
    pub eu_range: super::range::Range,
    pub title: opcua::types::localized_text::LocalizedText,
    pub axis_scale_type: super::enums::AxisScaleEnumeration,
    pub axis_steps: Option<Vec<f64>>,
}
impl opcua::types::MessageInfo for AxisInformation {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AxisInformation_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AxisInformation_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::AxisInformation_Encoding_DefaultXml
    }
}
impl opcua::types::BinaryEncodable for AxisInformation {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.engineering_units.byte_len();
        size += self.eu_range.byte_len();
        size += self.title.byte_len();
        size += self.axis_scale_type.byte_len();
        size += self.axis_steps.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(
        &self,
        stream: &mut S,
    ) -> opcua::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.engineering_units.encode(stream)?;
        size += self.eu_range.encode(stream)?;
        size += self.title.encode(stream)?;
        size += self.axis_scale_type.encode(stream)?;
        size += self.axis_steps.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &opcua::types::DecodingOptions,
    ) -> opcua::types::EncodingResult<Self> {
        Ok(Self {
            engineering_units: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            eu_range: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
            title: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
            axis_scale_type: opcua::types::BinaryEncodable::decode(
                stream,
                decoding_options,
            )?,
            axis_steps: opcua::types::BinaryEncodable::decode(stream, decoding_options)?,
        })
    }
}
