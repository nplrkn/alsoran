#!/usr/bin/env python3

import sys
from pickle import FALSE
import unittest
from lark.visitors import Interpreter
from case import pascal_case, snake_case
from lark.lexer import Token
from lark import Tree, Lark
from parser import parse_string, parse_file
from transformer import transform


def bool_to_rust(b):
    return "true" if b else "false"


def type_and_constraints(typ):
    constraints = "UNCONSTRAINED"
    string_type = None

    if isinstance(typ, Tree):
        bounds = typ.children
        typ = typ.data
        ext = "false"

        if typ == 'String':
            string_type = bounds[-1]
            del bounds[-1]

        if len(bounds) > 1:
            MAX_I64 = 9223372036854775807
            MIN_I64 = -9223372036854775808
            lb = MAX_I64  # i64::MAX
            ub = MIN_I64  # i64::MIN
            for i in range(0, len(bounds), 2):
                if isinstance(bounds[i], Tree):
                    assert bounds[i].data == "extension_marker"
                    ext = "true"
                    break
                lb = min(lb, int(bounds[i]))
                ub = max(lb, int(bounds[i]))
                if bounds[i+1] is not None:
                    lb = min(lb, int(bounds[i+1]))
                    ub = max(ub, int(bounds[i+1]))

            if ub > MAX_I64:
                print("Warning: upper bound exceeds i64 - incorrect coding will result")
                ub = MAX_I64

            if ub in [MAX_I64, MIN_I64]:
                ub = "None"
            else:
                ub = f"Some({ub})"

            if typ in ["u8", "u16", "u32", "u64"]:
                constraints = f"Constraints::value(Some({lb}), {ub}, {ext})"
            else:
                constraints = f"Constraints::size(Some({lb}), {ub}, {ext})"

    return (typ, constraints, string_type)


def decode_expression(tree):
    (typ, constraints, _) = type_and_constraints(tree)
    if typ == "Vec<u8>":
        return "aper::decode::decode_octetstring(data, None, None, false)?"
    elif typ == "BitString":
        return "aper::decode::decode_bitstring(data, None, None, false)?"
    else:
        return f"""{typ}::decode(data)?"""


class StructFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.self_fields = ""

    def field(self, tree):
        name = tree.children[0]
        self.self_fields += f"            {name},\n"
        self.fields_from += f"""\
        let {name} = {decode_expression(tree.children[1])};
"""

    def optional_field(self, tree):
        name = tree.children[0]
        self.self_fields += f"            {name},\n"
        self.fields_from += f"""\
        let {name} = if optionals.is_set(0) {{
            Some({decode_expression(tree.children[1])})
        }} else {{
            None
        }};
"""


class StructFindOptionals(Interpreter):
    def __init__(self):
        self.find_optionals = ""
        self.num_optionals = 0
        self.has_extension_container = False

    def optional_field(self, tree):
        name = tree.children[0]
        self.find_optionals += f"""\
        optionals.set({self.num_optionals}, self.{name}.is_some());
"""
        self.num_optionals += 1

    def extension_container(self, tree):
        self.find_optionals += f"""\
        optionals.set({self.num_optionals}, false);
"""
        self.num_optionals += 1
        self.has_extension_container = True


class EnumFields(Interpreter):
    def __init__(self):
        self.enum_fields = ""
        self.variants = 0
        self.extensible = False

    def enum_item(self, tree):
        self.variants += 1
        self.enum_fields += f"""\
    {tree.children[0]},
"""

    def extension_marker(self, _tree):
        self.extensible = True
#         self.enum_fields += f"""\
#     _Extended,
# """

    def extended_items(self, _tree):
        pass


class ChoiceFields(Interpreter):
    def __init__(self):
        self.choice_fields = ""

    def choicefield(self, tree):
        name = tree.children[0]
        typ = tree.children[1]

        if isinstance(typ, Tree):
            typ = typ.data
        self.choice_fields += f"""\
    {name}{"("+typ+")" if typ != "null" else ""},
"""

#     def extension_container(self, tree):
#         self.choice_fields += f"""\
#     _Extended,
# """

    def extension_marker(self, tree):
        print("Warning - extensible CHOICE not implemented")


# class ChoiceFieldsTo(Interpreter):
#     def __init__(self):
#         self.fields_to = ""
#         self.field_index = 0

#     def choicefield(self, tree):
#         name = tree.children[0]
#         (typ, constraints, _) = type_and_constraints(tree.children[1])

#         if typ != "null":
#             self.fields_to += f"""\
#             Self::{name}(x) => {{
#                 {to_aper_unconstrained(f"({self.field_index} as u8)")}?;
#                 {to_aper("x", constraints)}?;
#             }}
# """
#         else:
#             self.fields_to += f"""\
#             Self::{name} => {{
#                 {to_aper_unconstrained(f"({self.field_index} as u8)")}?;
#             }}
# """
#         self.field_index += 1

#     def extension_container(self, tree):
#         #         self.fields_to += f"""\
#         #             Self::_Extended => return Err(EncodeError::NotImplemented),
#         # """
#         self.field_index += 1


class ChoiceFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.field_index = 0

    def choicefield(self, tree):
        name = tree.children[0]
        (typ, constraints, _) = type_and_constraints(tree.children[1])

        if typ != "null":
            self.fields_from += f"""\
            {self.field_index} => Ok(Self::{name}({decode_expression(tree.children[1])})),
"""
        else:
            self.fields_from += f"""\
            {self.field_index} => Ok(Self::{name}),
"""
        self.field_index += 1

    def extension_container(self, tree):
        self.fields_from += f"""\
            {self.field_index} => Err(AperCodecError::new("Choice extension container not implemented")),
"""
        self.field_index += 1


class StructFields(Interpreter):
    def __init__(self):
        self.struct_fields = ""
        self.extensible = False

    def extension_marker(self, tree):
        self.extensible = True

    def field(self, tree):
        name = tree.children[0]
        typ = tree.children[1]
        if isinstance(typ, Tree):
            typ = typ.data
        self.struct_fields += f"""\
    pub {name}: {typ},
"""

    def optional_field(self, tree):
        name = tree.children[0]
        typ = tree.children[1]
        if isinstance(typ, Tree):
            typ = typ.data
        self.struct_fields += f"""\
    pub {name}: Option<{typ}>,
"""


class IeFields(Interpreter):
    def __init__(self):
        self.struct_fields = ""
        self.extensible = False
        self.mut_field_vars = ""
        self.self_fields = ""
        self.matches = ""
        self.mandatory = ""
        self.mandatory_fields_to = ""
        self.optionals_presence_list = ""
        self.optional_fields_to = ""
        self.num_mandatory_fields = 0

    def extension_marker(self, tree):
        self.extensible = True

    def ie(self, tree):
        name = tree.children[0]
        id = tree.children[1]
        criticality = tree.children[2].capitalize()
        (typ, constraints, _) = type_and_constraints(tree.children[3])
        self.struct_fields += f"""\
    pub {name}: {typ},
"""
        self.mut_field_vars += f"""\
        let mut {name}: Option<{typ}> = None;
"""
        self.self_fields += f"            {name},\n"
        self.matches += f"""\
                {id} => {{
                    {name} = Some({decode_expression(tree.children[3])});
                }}
"""
        self.num_mandatory_fields += 1
        self.mandatory += f"""\
        let {name} = {name}.ok_or(aper::AperCodecError::new(format!(
            "Missing mandatory IE {name}"
        )))?;
"""
        self.mandatory_fields_to += f"""\
        {to_aper_unconstrained(f"({id} as u16)")}?;
        {to_aper_unconstrained(f"Criticality::{criticality}")}?;
        enc.append_open(&self.{name})?;
"""

    def optional_ie(self, tree):
        name = tree.children[0]
        id = tree.children[1]
        criticality = tree.children[2].capitalize()
        typ = tree.children[3]
        (typ, constraints, _) = type_and_attributes(tree.children[3])
        self.struct_fields += f"""\
    pub {name}: Option<{typ}>,
"""
        self.mut_field_vars += f"""\
        let mut {name}: Option<{typ}> = None;
"""
        self.self_fields += f"            {name},\n"
        self.matches += f"""\
                {id} => {{
                    {name} = Some({decode_expression(tree.children[3])});
                }}
"""
        self.optionals_presence_list += f"self.{name}.is_some(),"

#         self.optional_fields_to += f"""\
#         if let Some(x) = &self.{name} {{
#             {to_aper_unconstrained(f"({id} as u16)")}?;
#             {to_aper_unconstrained(f"Criticality::{criticality}")}?;
#             enc.append_open(&self.x)?;
#         }}
# """


# class StructFieldsTo(Interpreter):
#     def __init__(self):
#         self.fields_to = ""

#     def field(self, tree):
#         name = tree.children[0]
#         self.fields_to += f"""\
#         {to_aper_unconstrained(f"self.{name}")}?;
# """

#     def optional_field(self, tree):
#         name = tree.children[0]
#         self.fields_to += f"""\
#         if let Some(x) = &self.{name} {{
#             {to_aper_unconstrained("x")}?;
#         }}
# """


# class IeFieldsTo(Interpreter):
#     def field(self, tree):
#         pass


MUT_OPTIONALS = """let mut optionals = BitString::with_len({num_optionals});"""


class StructInterpreter(Interpreter):

    def __init__(self):
        # self.output = ""
        self.outfile = ""
        self.in_enum = False

    def extended_items(self, tree):
        pass

    def enumdef(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        field_interpreter = EnumFields()
        field_interpreter.visit(tree.children[1])

        num_variants = field_interpreter.variants

        assert(num_variants <= 256)

        self.outfile += f"""
// {orig_name}
#[derive(Clone, Copy, TryFromPrimitive)]
pub enum {name} {{
{field_interpreter.enum_fields}\
}}

impl AperCodec for {name} {{
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{\
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some({num_variants - 1}, false)?;
        if extended {{
            return Err(aper::AperCodecError::new("Extended enum not implemented"))
        }}
        Self::try_from(idx).ok_or(AperCodecError::new("Unknown enum variant"))
    }}
}}

"""
        return name

    def choicedef(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        field_interpreter = ChoiceFields()
        field_interpreter.visit(tree.children[1])

        fields_from_interpreter = ChoiceFieldsFrom()
        fields_from_interpreter.visit(tree.children[1])
        #fields_to_interpreter = ChoiceFieldsTo()
        # fields_to_interpreter.visit(tree.children[1])

        self.outfile += f"""
// {orig_name}
#[derive(Clone)]
pub enum {name} {{
{field_interpreter.choice_fields}\
}}

impl AperCodec for {name} {{
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        let (idx, extended) = aper::decode::decode_choice_idx(data, 0, {fields_from_interpreter.field_index - 1}, false)?;
        if extended {{
            return Err(aper::AperCodecError::new("CHOICE additions not implemented"))
        }}
        match idx {{
{fields_from_interpreter.fields_from}\
            _ => Err(AperCodecError::new("Unknown choice idx"))
        }}
    }}
}}

"""

    def tuple_struct(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        (inner, constraints, _) = type_and_constraints(tree.children[1])
        # inner = tree.children[1].data
        ub = None
        lb = None
        if len(tree.children[1].children) > 2:
            ub = tree.children[1].children[1]
        if len(tree.children[1].children) > 1:
            lb = tree.children[1].children[0]
        if ub == None:
            ub = lb

        self.outfile += f"""
// {orig_name}
#[derive(Clone)]
pub struct {name}(pub {inner});

impl AperCodec for {name} {{
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        let (val, extended) = aper::decode::decode_integer(data, Some(0), Some(4095), true)?;
        if extended {{
            return Err(aper::AperCodecError::new("Integer extension not implemented"))
        }}
        Ok(Self(val as {inner}))
    }}
}}

"""

    def ie(self, tree):
        pass
        # name = snake_case(tree.children[0])
        # self.output += "  pub " + name + ": "
        # s = StructInterpreter()
        # self.output += s.get_type(tree) + ",\n"
        # assert(s.outfile == "")  # Can't handle inline enum here

    def pdu(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name

        field_interpreter = IeFields()
        for i in [field_interpreter]:
            i.visit(tree.children[1])

        #   ProtocolIE-Container {NGAP-PROTOCOL-IES : IEsSetParam} ::=
        # 	SEQUENCE (SIZE (0..maxProtocolIEs)) OF
        # 	ProtocolIE-Field {{IEsSetParam}}

        # ProtocolIE-Field {NGAP-PROTOCOL-IES : IEsSetParam} ::= SEQUENCE {
        # 	id				NGAP-PROTOCOL-IES.&id				({IEsSetParam}),
        # 	criticality		NGAP-PROTOCOL-IES.&criticality		({IEsSetParam}{@id}),
        # 	value			NGAP-PROTOCOL-IES.&Value			({IEsSetParam}{@id})
        # }

        self.outfile += f"""
// {orig_name}
#[derive(Clone)]
pub struct {name} {{
{field_interpreter.struct_fields}\
}}

impl AperCodec for {orig_name} {{
    type Output = {orig_name};
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        let _length = aper::decode::decode_length_determinent(data, None, None, false)?;
        let _ = aper::decode::decode_sequence_header(data, true, 0)?;
        let len = aper::decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

{field_interpreter.mut_field_vars}
        for _ in 0..len {{
            let (id, _ext) = aper::decode::decode_integer(data, Some(0), Some(65535), false)?;
            let criticality = Criticality::decode(data)?;
            let _length = aper::decode::decode_length_determinent(data, None, None, false)?;
            match id {{
{field_interpreter.matches}\
                x => {{
                    if let Criticality::Reject = criticality {{
                        return Err(aper::AperCodecError::new(format!(
                            "Unrecognised IE type {{}} with criticality reject",
                            x
                        )));
                    }}
                }}
            }}
        }}
{field_interpreter.mandatory}\
        Ok(Self {{
{field_interpreter.self_fields}\
        }})
    }}
}}

"""

    def struct(self, tree):
        if tree.children[1].data == "ie_container_sequence":
            self.pdu(tree)
            return

        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name

        fields = [
            child for child in tree.children[1].children if child.data in ["field", "optional_field"]]

        # Omit if there are 0 fields, as is normally the case for extension IEs
        if len(fields) == 0:
            self.comment(tree, "omitted\n")
            return

        field_interpreter = StructFields()
        fields_from_interpreter = StructFieldsFrom()
        find_opt_interpreter = StructFindOptionals()
        for i in [field_interpreter, fields_from_interpreter, find_opt_interpreter]:
            i.visit(tree.children[1])

        #fields_to_interpreter = StructFieldsTo()

        # for i in [field_interpreter, fields_from_interpreter, find_opt_interpreter, fields_to_interpreter]:
        #     i.visit(tree.children[1])
        # field_interpreter.visit(tree.children[1])
        # fields_from_interpreter.visit(tree.children[1])
        # find_opt_interpreter.visit(tree.children[1])
        # fields_to_interpreter.visit(tree.children[1])
        num_optionals = find_opt_interpreter.num_optionals

        optionals_from = f"""let {"_" if num_optionals == 1 else ""}optionals = BitString::from_aper(decoder, Constraints::size(Some({num_optionals}), Some({num_optionals}), false))?;"""

        self.outfile += f"""
// {orig_name}
#[derive(Clone)]
pub struct {name} {{
{field_interpreter.struct_fields}\
}}

impl AperCodec for {orig_name} {{
    type Output = {orig_name};
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        let (bitmap, _extensions_present) = aper::decode::decode_sequence_header(data, {bool_to_rust(field_interpreter.extensible)}, {num_optionals})?;
        {optionals_from if num_optionals > 0 else ""}
{fields_from_interpreter.fields_from}
        Ok(Self {{
{fields_from_interpreter.self_fields}\
        }})
    }}
}}

"""
        return name

    def comment(self, tree, comment=""):
        if comment != "":
            comment = " - " + comment
        self.outfile += "// " + tree.children[0] + comment + "\n"

    def objectdef(self, tree):
        print("Warning - objectdef not implemented")

    def extension_container(self, tree):
        pass

    def extended_item(self, tree):
        assert(False)


def generate(tree, constants=dict(), verbose=False):
    tree = transform(tree, constants)
    if verbose:
        print(tree.pretty())
    visited = StructInterpreter()
    print("---- Generating ----")
    visited.visit(tree)
    return visited.outfile


def generate_structs(input_file="f1ap/asn1/F1AP-PDU-Contents.asn", constants=dict(), verbose=False):
    tree = parse_file(input_file)
    if verbose:
        print(tree.pretty())
    return generate(tree, constants, print)


class TestGenerator(unittest.TestCase):
    maxDiff = None

    def should_generate(self, input, expected, constants=dict()):
        output = ""
        tree = parse_string(input)
        try:
            output = generate(tree, constants, True)
            print(output)
            self.assertEqual(output, expected)
        finally:
            if output != expected:
                print(tree.pretty())

    def test_enum_unextensible(self):
        input = """\
TriggeringMessage	::= ENUMERATED { initiating-message, successful-outcome, unsuccessful-outcome }
"""
        output = """\

// TriggeringMessage
#[derive(Clone, Copy)]
pub enum TriggeringMessage {
    InitiatingMessage,
    SuccessfulOutcome,
    UnsuccessfulOutcome,
}
"""
        self.should_generate(input, output)

    def test_sequence(self):
        # Following has been adapted from the original to produce more interesting and
        # shorter output.
        input = """\
WLANMeasurementConfiguration ::= SEQUENCE {
	wlanMeasConfig             	WLANMeasConfig,
	wlan-rtt                   	ENUMERATED {thing1, ..., thing2} OPTIONAL,
	iE-Extensions		ProtocolExtensionContainer {{WLANMeasurementConfiguration-ExtIEs}} 	OPTIONAL,
	...
}
"""
        output = """\

// WlanMeasurementConfiguration
#[derive(Clone)]
pub struct WlanMeasurementConfiguration {
    pub wlan_meas_config: WlanMeasConfig,
    pub wlan_rtt: Option<WlanRtt>,
}

// WlanRtt
#[derive(Clone, Copy)]
pub enum WlanRtt {
    Thing1,
}
"""
        self.should_generate(input, output)

    def test_unbounded_octet_string(self):
        input = """\
LTEUERLFReportContainer::= OCTET STRING
"""
        output = """\

// LteueRlfReportContainer
#[derive(Clone)]
pub struct LteueRlfReportContainer(pub Vec<u8>);
"""
        self.should_generate(input, output)

    def test_bounded_int_newtype(self):
        input = """\
MaximumDataBurstVolume::= INTEGER(0..4095, ..., 4096.. 2000000)
"""
        output = """\

// MaximumDataBurstVolume
#[derive(Clone)]
pub struct MaximumDataBurstVolume(pub u16);

impl AperCodec for MaximumDataBurstVolume {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (val, extended) = aper::decode::decode_integer(data, Some(0), Some(4095), true)?;
        if extended {
            return Err(aper::AperCodecError::new("Integer extension not implemented"))
        }
        Ok(Self(val as u16))
    }
}

"""
        self.should_generate(input, output)

    def test_newtype(self):
        input = """
MobilityInformation ::= BIT STRING(SIZE(16))
"""
        output = """\

// MobilityInformation
#[derive(Clone)]
pub struct MobilityInformation(pub BitString);
"""
        self.should_generate(input, output)

    def test_enumerated(self):
        input = """\
MaximumIntegrityProtectedDataRate ::= ENUMERATED {
	bitrate64kbs,
	maximum-UE-rate,
	...
}
"""
        output = """\

// MaximumIntegrityProtectedDataRate
#[derive(Clone, Copy)]
pub enum MaximumIntegrityProtectedDataRate {
    Bitrate64kbs,
    MaximumUeRate,
}
"""
        self.should_generate(input, output)

    def test_choice(self):
        self.should_generate("""\
EventTrigger ::= CHOICE {
	outOfCoverage				ENUMERATED { true, ... } ,
	eventL1LoggedMDTConfig		NULL,
	short-macroENB-ID 		    BIT STRING (SIZE (18)),
	choice-Extensions		    ProtocolIE-SingleContainer { { EventTrigger-ExtIEs } }
}
""", """\

// EventTrigger
#[derive(Clone)]
pub enum EventTrigger {
    OutOfCoverage(OutOfCoverage),
    EventL1LoggedMdtConfig,
    ShortMacroEnbId(BitString),
}

// OutOfCoverage
#[derive(Clone, Copy)]
pub enum OutOfCoverage {
    True,
}
""")

    def test_pdu_contents(self):
        self.should_generate("""\
PDUSessionResourceSetupRequest ::= SEQUENCE {
	protocolIEs		ProtocolIE-Container		{ {PDUSessionResourceSetupRequestIEs} },
	...
}

PDUSessionResourceSetupRequestIEs NGAP-PROTOCOL-IES ::= {
	{ ID id-AMF-UE-NGAP-ID							CRITICALITY reject	TYPE AMF-UE-NGAP-ID								PRESENCE mandatory	}|
	{ ID id-RANPagingPriority						CRITICALITY ignore	TYPE OCTET STRING							PRESENCE optional		}|
	...
}
""", """\

// PduSessionResourceSetupRequest
#[derive(Clone)]
pub struct PduSessionResourceSetupRequest {
    pub amf_ue_ngap_id: AmfUeNgapId,
    pub ran_paging_priority: Option<Vec<u8>>,
}

impl AperCodec for PduSessionResourceSetupRequest {
    type Output = PduSessionResourceSetupRequest;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let _length = aper::decode::decode_length_determinent(data, None, None, false)?;
        let _ = aper::decode::decode_sequence_header(data, true, 0)?;
        let len = aper::decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

        let mut amf_ue_ngap_id: Option<AmfUeNgapId> = None;
        let mut ran_paging_priority: Option<Vec<u8>> = None;

        for _ in 0..len {
            let (id, _ext) = aper::decode::decode_integer(data, Some(0), Some(65535), false)?;
            let criticality = Criticality::decode(data)?;
            let _length = aper::decode::decode_length_determinent(data, None, None, false)?;
            match id {
                10 => {
                    amf_ue_ngap_id = Some(AmfUeNgapId::decode(data)?);
                }
                83 => {
                    ran_paging_priority = Some(aper::decode::decode_octetstring(data, None, None, false)?);
                }
                x => {
                    if let Criticality::Reject = criticality {
                        return Err(aper::AperCodecError::new(format!(
                            "Unrecognised IE type {} with criticality reject",
                            x
                        )));
                    }
                }
            }
        }
        let amf_ue_ngap_id = amf_ue_ngap_id.ok_or(aper::AperCodecError::new(format!(
            "Missing mandatory IE amf_ue_ngap_id"
        )))?;
        Ok(Self {
            amf_ue_ngap_id,
            ran_paging_priority,
        })
    }
}

""", constants={"id-AMF-UE-NGAP-ID": 10, "id-RANPagingPriority": 83})

    def test_bit_string(self):
        self.should_generate("""\
GNB-ID ::= CHOICE {
	gNB-ID		BIT STRING (SIZE (22..32)),
	choice-Extensions		ProtocolIE-SingleContainer { {GNB-ID-ExtIEs } }
}
""", """\

// GnbId
#[derive(Clone)]
pub enum GnbId {
    GnbId(BitString),
}

impl AperCodec for GnbId {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (idx, extended) = aper::decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(aper::AperCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::GnbId(aper::decode::decode_bitstring(data, None, None, false)?)),
            1 => Err(AperCodecError::new("Choice extension container not implemented")),
            _ => Err(AperCodecError::new("Unknown choice idx"))
        }
    }
}

""")

    def test_bug(self):
        self.should_generate("""\
PrivateIE-ID	::= CHOICE {
	local				INTEGER (0..65535),
	global				OBJECT IDENTIFIER
}
""", """\

// PrivateIeId
#[derive(Clone)]
pub enum PrivateIeId {
    Local(u16),
    Global(Vec<u8>),
}

impl AperCodec for PrivateIeId {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (idx, extended) = aper::decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(aper::AperCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::Local(aper::decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16,
            1 => Ok(Self::Global(aper::decode::decode_octetstring(data, None, None, false)?)),
            _ => Err(AperCodecError::new("Unknown choice idx"))
        }
    }
}

""")

    def test_int_options(self):
        self.should_generate("""\
ExpectedActivityPeriod ::= INTEGER (1..30|40|50, ..., -1..70)
""", """\

// ExpectedActivityPeriod
#[derive(Clone)]
pub struct ExpectedActivityPeriod(pub u8);
""")


if __name__ == '__main__':
    if len(sys.argv) == 2:
        print(generate_structs(sys.argv[1], verbose=True))
    else:
        unittest.main(failfast=True)
