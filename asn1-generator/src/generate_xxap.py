#!/usr/bin/env python3

from rust_interpreter2 import generate_structs
from constants import get_constants_from_file
# from pdu import generate_pdus

COPYRIGHT = "// Copyright (c) Nicholas Larkin\n"
AUTOGENERATED = "// Autogenerated from "
USE_ASN1 = """
#[allow(unused_imports)]
use asn1::BitString;
use asn1::aper::*;
#[allow(unused_imports)]
use num_derive::FromPrimitive;
#[allow(unused_imports)]
use num_traits::FromPrimitive;"""
USE_COMMON = "use super::common::*;\n"
USE_IES = "use super::ies::*;"
COMMON_STRING_DEFS = """
pub struct VisibleString(pub String);
impl APerElement for VisibleString {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        unimplemented!()
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        unimplemented!()
    }
}

pub struct Utf8String(pub String);
impl APerElement for Utf8String {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        unimplemented!()
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        unimplemented!()
    }
}

pub struct PrintableString(pub String);
impl APerElement for PrintableString {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        unimplemented!()
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        unimplemented!()
    }
}
"""


def generate(protocol):
    protocol_lower = protocol.lower()
    protocol_upper = protocol.upper()

    def open_output_file(filename):
        return open(protocol_lower + "/src/" + filename, "w")

    def input_filename(filename_part):
        f = protocol_upper + "-" + filename_part + ".asn"
        print("Processing " + f)
        return f

    def input_file_path(filename):
        return protocol_lower + "/asn1/" + filename

    f = open_output_file("lib.rs")
    f.write(COPYRIGHT)
    f.write("""
pub mod common;
pub mod ies;
pub mod pdu;
""")

    f = open_output_file("common.rs")
    i = input_filename("CommonDataTypes")
    f.write(COPYRIGHT + AUTOGENERATED + i +
            USE_ASN1 + COMMON_STRING_DEFS + "\n\n")
    f.write(generate_structs(input_file_path(i)))

    i = input_filename("Constants")
    constants = get_constants_from_file(input_file_path(i))

    f = open_output_file("ies.rs")
    i = input_filename("IEs")
    f.write(COPYRIGHT + AUTOGENERATED + i +
            "\n" + USE_COMMON + USE_ASN1 + "\n")
    f.write(generate_structs(input_file_path(i), constants))

    f = open_output_file("pdu.rs")
    i = input_filename("PDU-Contents")
    f.write(COPYRIGHT
            + AUTOGENERATED
            + i
            + "\n"
            + USE_ASN1
            + USE_COMMON
            + USE_IES
            + "\n\n")
    f.write(generate_structs(input_file_path(i), constants))


generate("f1ap")
generate("ngap")