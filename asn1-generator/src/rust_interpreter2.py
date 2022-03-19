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


def to_aper_unconstrained(t1):
    return to_aper(t1, "UNCONSTRAINED")


def to_aper(t1, constraints):
    return f"{t1}.to_aper(enc, {constraints})"


EXTENSION_TO = f"""
        {to_aper_unconstrained("false")}?;"""


def type_and_constraints(typ):
    constraints = "UNCONSTRAINED"

    if isinstance(typ, Tree):
        bounds = typ.children
        typ = typ.data
        ext = "false"

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

    return (typ, constraints)


def decode_expression(tree):
    (typ, constraints) = type_and_constraints(tree)
    return f"""{typ.replace("Vec<","Vec::<")}::from_aper(decoder, {constraints})?"""


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


class ChoiceFieldsTo(Interpreter):
    def __init__(self):
        self.fields_to = ""
        self.field_index = 0

    def choicefield(self, tree):
        name = tree.children[0]
        (typ, constraints) = type_and_constraints(tree.children[1])

        if typ != "null":
            self.fields_to += f"""\
            Self::{name}(x) => {{
                {to_aper_unconstrained(f"({self.field_index} as u8)")}?;
                {to_aper("x", constraints)}?;
            }}
"""
        else:
            self.fields_to += f"""\
            Self::{name} => {{
                {to_aper_unconstrained(f"({self.field_index} as u8)")}?;
            }}
"""
        self.field_index += 1

    def extension_container(self, tree):
        #         self.fields_to += f"""\
        #             Self::_Extended => return Err(EncodeError::NotImplemented),
        # """
        self.field_index += 1


class ChoiceFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.field_index = 0

    def choicefield(self, tree):
        name = tree.children[0]
        (typ, constraints) = type_and_constraints(tree.children[1])

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
            {self.field_index} => Err(DecodeError::NotImplemented),
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

    def extension_marker(self, tree):
        self.extensible = True

    def ie(self, tree):
        name = tree.children[0]
        id = tree.children[1]
        criticality = tree.children[2].capitalize()
        (typ, constraints) = type_and_constraints(tree.children[3])
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
        self.mandatory += f"""\
        let {name} = {name}.ok_or(DecodeError::InvalidChoice)?;
"""
        self.mandatory_fields_to += f"""\
        {to_aper_unconstrained(f"({id} as u16)")}?;
        {to_aper_unconstrained(f"Criticality::{criticality}")}?;
        {to_aper_unconstrained(f"self.{name}")}?;
"""

    def optional_ie(self, tree):
        name = tree.children[0]
        id = tree.children[1]
        criticality = tree.children[2].capitalize()
        typ = tree.children[3]
        (typ, constraints) = type_and_constraints(tree.children[3])
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

        self.optional_fields_to += f"""\
        if let Some(x) = &self.{name} {{
            {to_aper_unconstrained(f"({id} as u16)")}?;
            {to_aper_unconstrained(f"Criticality::{criticality}")}?;
            {to_aper_unconstrained(f"x")}?;
        }}
"""


class StructFieldsTo(Interpreter):
    def __init__(self):
        self.fields_to = ""

    def field(self, tree):
        name = tree.children[0]
        self.fields_to += f"""\
        {to_aper_unconstrained(f"self.{name}")}?;
"""

    def optional_field(self, tree):
        name = tree.children[0]
        self.fields_to += f"""\
        if let Some(x) = &self.{name} {{
            {to_aper_unconstrained("x")}?;
        }}
"""


class IeFieldsTo(Interpreter):
    def field(self, tree):
        pass


MUT_OPTIONALS = """let mut optionals = BitString::with_len({num_optionals});"""

EXTENSION_FROM = """
        if bool::from_aper(decoder, UNCONSTRAINED)? {
            return Err(DecodeError::NotImplemented)
        }"""


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
        typ = "u8"
        constraints = f"Constraints::value(Some(0), Some({num_variants - 1}), false)"

        self.outfile += f"""\
// {orig_name}
# [derive(Clone, Copy, FromPrimitive)]
pub enum {name} {{
{field_interpreter.enum_fields}\
}}

impl APerElement for {name} {{
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {{\
{EXTENSION_FROM if field_interpreter.extensible else ""}
        let v = u8::from_aper(decoder, {constraints})?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }}
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {{\
{EXTENSION_TO if field_interpreter.extensible else ""}
        {to_aper(f"(*self as u8)", constraints)}?;
        Ok(())
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
        fields_to_interpreter = ChoiceFieldsTo()
        fields_to_interpreter.visit(tree.children[1])

        self.outfile += f"""\
// {orig_name}
# [derive(Clone)]
pub enum {name} {{
{field_interpreter.choice_fields}\
}}

impl APerElement for {name} {{
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {{
        match u8::from_aper(decoder, UNCONSTRAINED)? {{
{fields_from_interpreter.fields_from}\
            _ => Err(DecodeError::InvalidChoice),
        }}
    }}
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {{
        match self {{
{fields_to_interpreter.fields_to}\
        }}
        Ok(())
    }}
}}


"""

    def tuple_struct(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        (inner, constraints) = type_and_constraints(tree.children[1])
        # inner = tree.children[1].data
        ub = None
        lb = None
        if len(tree.children[1].children) > 2:
            ub = tree.children[1].children[1]
        if len(tree.children[1].children) > 1:
            lb = tree.children[1].children[0]
        if ub == None:
            ub = lb

        output = f"""\
// {orig_name}
# [derive(Clone)]
pub struct {name}(pub {inner});

impl APerElement for {name} {{
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {{
        Ok(Self({decode_expression(tree.children[1])}))
    }}
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {{
        {to_aper("self.0", constraints)}
    }}
}}

"""
        self.outfile += output

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

        self.outfile += f"""\
// {orig_name}
# [derive(Clone)]
pub struct {name} {{
{field_interpreter.struct_fields}\
}}

impl APerElement for {name} {{
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {{\
{EXTENSION_FROM if field_interpreter.extensible else ""}
        let len = decoder.decode_length()?;
{field_interpreter.mut_field_vars}
        for _ in 0..len {{
            let id = u16::from_aper(decoder, UNCONSTRAINED)?;
            let criticality = Criticality::from_aper(decoder, UNCONSTRAINED)?;
            match id {{
{field_interpreter.matches}\
                _ => {{
                    if let Criticality::Reject = criticality {{
                        return Err(DecodeError::InvalidChoice);
                    }}
                }}
            }}
        }}
{field_interpreter.mandatory}\
        Ok(Self {{
{field_interpreter.self_fields}\
        }})
    }}
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {{
        let num_ies = [{field_interpreter.optionals_presence_list}]
            .iter()
            .filter(|&x| *x)
            .count();

        {to_aper_unconstrained("false")}?;
        enc.append(&encode_length(num_ies)?)?;
{field_interpreter.mandatory_fields_to}\
{field_interpreter.optional_fields_to}
        Ok(())
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
        fields_to_interpreter = StructFieldsTo()

        for i in [field_interpreter, fields_from_interpreter, find_opt_interpreter, fields_to_interpreter]:
            i.visit(tree.children[1])
        # field_interpreter.visit(tree.children[1])
        # fields_from_interpreter.visit(tree.children[1])
        # find_opt_interpreter.visit(tree.children[1])
        # fields_to_interpreter.visit(tree.children[1])
        num_optionals = find_opt_interpreter.num_optionals

        optionals_from = f"""let {"_" if num_optionals == 1 else ""}optionals = BitString::from_aper(decoder, Constraints::size(Some({num_optionals}), Some({num_optionals}), false))?;"""

        self.outfile += f"""\
// {orig_name}
# [derive(Clone)]
pub struct {name} {{
{field_interpreter.struct_fields}\
}}

impl APerElement for {name} {{
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {{\
{EXTENSION_FROM if field_interpreter.extensible else ""}
        {optionals_from if num_optionals > 0 else ""}
{fields_from_interpreter.fields_from}
        Ok(Self {{
{fields_from_interpreter.self_fields}\
        }})
    }}
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {{
        {f"let mut optionals = BitString::with_len({num_optionals});" if num_optionals > 0 else ""}
{find_opt_interpreter.find_optionals if num_optionals > 0 else ""}\
{EXTENSION_TO if field_interpreter.extensible else ""}
        {(to_aper("optionals", f"Constraints::size(Some({num_optionals}), Some({num_optionals}), false)") + "?;")
                 if num_optionals > 0 else ""}
{fields_to_interpreter.fields_to}
        Ok(())
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
# [derive(Clone, Copy, FromPrimitive)]
pub enum TriggeringMessage {
    InitiatingMessage,
    SuccessfulOutcome,
    UnsuccessfulOutcome,
}

impl APerElement for TriggeringMessage {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        let v = u8::from_aper(decoder, Constraints::value(Some(0), Some(2), false))?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        (*self as u8).to_aper(enc, Constraints::value(Some(0), Some(2), false))?;
        Ok(())
    }
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
# [derive(Clone)]
pub struct WlanMeasurementConfiguration {
    pub wlan_meas_config: WlanMeasConfig,
    pub wlan_rtt: Option<WlanRtt>,
}

impl APerElement for WlanMeasurementConfiguration {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, UNCONSTRAINED)? {
            return Err(DecodeError::NotImplemented)
        }
        let optionals = BitString::from_aper(decoder, Constraints::size(Some(2), Some(2), false))?;
        let wlan_meas_config = WlanMeasConfig::from_aper(decoder, UNCONSTRAINED)?;
        let wlan_rtt = if optionals.is_set(0) {
            Some(WlanRtt::from_aper(decoder, UNCONSTRAINED)?)
        } else {
            None
        };

        Ok(Self {
            wlan_meas_config,
            wlan_rtt,
        })
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        let mut optionals = BitString::with_len(2);
        optionals.set(0, self.wlan_rtt.is_some());
        optionals.set(1, false);

        false.to_aper(enc, UNCONSTRAINED)?;
        optionals.to_aper(enc, Constraints::size(Some(2), Some(2), false))?;
        self.wlan_meas_config.to_aper(enc, UNCONSTRAINED)?;
        if let Some(x) = &self.wlan_rtt {
            x.to_aper(enc, UNCONSTRAINED)?;
        }

        Ok(())
    }
}

// WlanRtt
# [derive(Clone, Copy, FromPrimitive)]
pub enum WlanRtt {
    Thing1,
}

impl APerElement for WlanRtt {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, UNCONSTRAINED)? {
            return Err(DecodeError::NotImplemented)
        }
        let v = u8::from_aper(decoder, Constraints::value(Some(0), Some(0), false))?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        false.to_aper(enc, UNCONSTRAINED)?;
        (*self as u8).to_aper(enc, Constraints::value(Some(0), Some(0), false))?;
        Ok(())
    }
}

"""
        self.should_generate(input, output)

    def test_unbounded_octet_string(self):
        input = """\
LTEUERLFReportContainer::= OCTET STRING
"""
        output = """\
// LteueRlfReportContainer
# [derive(Clone)]
pub struct LteueRlfReportContainer(pub Vec<u8>);

impl APerElement for LteueRlfReportContainer {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        Ok(Self(Vec::<u8>::from_aper(decoder, UNCONSTRAINED)?))
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        self.0.to_aper(enc, UNCONSTRAINED)
    }
}

"""
        self.should_generate(input, output)

    def test_bounded_int_newtype(self):
        input = """\
MaximumDataBurstVolume::= INTEGER(0..4095, ..., 4096.. 2000000)
"""
        output = """\
// MaximumDataBurstVolume
# [derive(Clone)]
pub struct MaximumDataBurstVolume(pub u16);

impl APerElement for MaximumDataBurstVolume {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        Ok(Self(u16::from_aper(decoder, Constraints::value(Some(0), Some(4095), true))?))
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        self.0.to_aper(enc, Constraints::value(Some(0), Some(4095), true))
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
# [derive(Clone)]
pub struct MobilityInformation(pub BitString);

impl APerElement for MobilityInformation {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        Ok(Self(BitString::from_aper(decoder, Constraints::size(Some(16), Some(16), false))?))
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        self.0.to_aper(enc, Constraints::size(Some(16), Some(16), false))
    }
}

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
# [derive(Clone, Copy, FromPrimitive)]
pub enum MaximumIntegrityProtectedDataRate {
    Bitrate64kbs,
    MaximumUeRate,
}

impl APerElement for MaximumIntegrityProtectedDataRate {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, UNCONSTRAINED)? {
            return Err(DecodeError::NotImplemented)
        }
        let v = u8::from_aper(decoder, Constraints::value(Some(0), Some(1), false))?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        false.to_aper(enc, UNCONSTRAINED)?;
        (*self as u8).to_aper(enc, Constraints::value(Some(0), Some(1), false))?;
        Ok(())
    }
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
# [derive(Clone)]
pub enum EventTrigger {
    OutOfCoverage(OutOfCoverage),
    EventL1LoggedMdtConfig,
    ShortMacroEnbId(BitString),
}

impl APerElement for EventTrigger {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        match u8::from_aper(decoder, UNCONSTRAINED)? {
            0 => Ok(Self::OutOfCoverage(OutOfCoverage::from_aper(decoder, UNCONSTRAINED)?)),
            1 => Ok(Self::EventL1LoggedMdtConfig),
            2 => Ok(Self::ShortMacroEnbId(BitString::from_aper(decoder, Constraints::size(Some(18), Some(18), false))?)),
            3 => Err(DecodeError::NotImplemented),
            _ => Err(DecodeError::InvalidChoice),
        }
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        match self {
            Self::OutOfCoverage(x) => {
                (0 as u8).to_aper(enc, UNCONSTRAINED)?;
                x.to_aper(enc, UNCONSTRAINED)?;
            }
            Self::EventL1LoggedMdtConfig => {
                (1 as u8).to_aper(enc, UNCONSTRAINED)?;
            }
            Self::ShortMacroEnbId(x) => {
                (2 as u8).to_aper(enc, UNCONSTRAINED)?;
                x.to_aper(enc, Constraints::size(Some(18), Some(18), false))?;
            }
        }
        Ok(())
    }
}


// OutOfCoverage
# [derive(Clone, Copy, FromPrimitive)]
pub enum OutOfCoverage {
    True,
}

impl APerElement for OutOfCoverage {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, UNCONSTRAINED)? {
            return Err(DecodeError::NotImplemented)
        }
        let v = u8::from_aper(decoder, Constraints::value(Some(0), Some(0), false))?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        false.to_aper(enc, UNCONSTRAINED)?;
        (*self as u8).to_aper(enc, Constraints::value(Some(0), Some(0), false))?;
        Ok(())
    }
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
# [derive(Clone)]
pub struct PduSessionResourceSetupRequest {
    pub amf_ue_ngap_id: AmfUeNgapId,
    pub ran_paging_priority: Option<Vec<u8>>,
}

impl APerElement for PduSessionResourceSetupRequest {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, UNCONSTRAINED)? {
            return Err(DecodeError::NotImplemented)
        }
        let len = decoder.decode_length()?;
        let mut amf_ue_ngap_id: Option<AmfUeNgapId> = None;
        let mut ran_paging_priority: Option<Vec<u8>> = None;

        for _ in 0..len {
            let id = u16::from_aper(decoder, UNCONSTRAINED)?;
            let criticality = Criticality::from_aper(decoder, UNCONSTRAINED)?;
            match id {
                10 => {
                    amf_ue_ngap_id = Some(AmfUeNgapId::from_aper(decoder, UNCONSTRAINED)?);
                }
                83 => {
                    ran_paging_priority = Some(Vec::<u8>::from_aper(decoder, UNCONSTRAINED)?);
                }
                _ => {
                    if let Criticality::Reject = criticality {
                        return Err(DecodeError::InvalidChoice);
                    }
                }
            }
        }
        let amf_ue_ngap_id = amf_ue_ngap_id.ok_or(DecodeError::InvalidChoice)?;
        Ok(Self {
            amf_ue_ngap_id,
            ran_paging_priority,
        })
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        let num_ies = [self.ran_paging_priority.is_some(),]
            .iter()
            .filter(|&x| *x)
            .count();

        false.to_aper(enc, UNCONSTRAINED)?;
        enc.append(&encode_length(num_ies)?)?;
        (10 as u16).to_aper(enc, UNCONSTRAINED)?;
        Criticality::Reject.to_aper(enc, UNCONSTRAINED)?;
        self.amf_ue_ngap_id.to_aper(enc, UNCONSTRAINED)?;
        if let Some(x) = &self.ran_paging_priority {
            (83 as u16).to_aper(enc, UNCONSTRAINED)?;
            Criticality::Ignore.to_aper(enc, UNCONSTRAINED)?;
            x.to_aper(enc, UNCONSTRAINED)?;
        }

        Ok(())
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
# [derive(Clone)]
pub enum GnbId {
    GnbId(BitString),
}

impl APerElement for GnbId {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        match u8::from_aper(decoder, UNCONSTRAINED)? {
            0 => Ok(Self::GnbId(BitString::from_aper(decoder, Constraints::size(Some(22), Some(32), false))?)),
            1 => Err(DecodeError::NotImplemented),
            _ => Err(DecodeError::InvalidChoice),
        }
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        match self {
            Self::GnbId(x) => {
                (0 as u8).to_aper(enc, UNCONSTRAINED)?;
                x.to_aper(enc, Constraints::size(Some(22), Some(32), false))?;
            }
        }
        Ok(())
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
# [derive(Clone)]
pub enum PrivateIeId {
    Local(u16),
    Global(Vec<u8>),
}

impl APerElement for PrivateIeId {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        match u8::from_aper(decoder, UNCONSTRAINED)? {
            0 => Ok(Self::Local(u16::from_aper(decoder, Constraints::value(Some(0), Some(65535), false))?)),
            1 => Ok(Self::Global(Vec::<u8>::from_aper(decoder, UNCONSTRAINED)?)),
            _ => Err(DecodeError::InvalidChoice),
        }
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        match self {
            Self::Local(x) => {
                (0 as u8).to_aper(enc, UNCONSTRAINED)?;
                x.to_aper(enc, Constraints::value(Some(0), Some(65535), false))?;
            }
            Self::Global(x) => {
                (1 as u8).to_aper(enc, UNCONSTRAINED)?;
                x.to_aper(enc, UNCONSTRAINED)?;
            }
        }
        Ok(())
    }
}


""")

    def test_int_options(self):
        self.should_generate("""\
ExpectedActivityPeriod ::= INTEGER (1..30|40|50, ..., -1..70)
""", """\
// ExpectedActivityPeriod
# [derive(Clone)]
pub struct ExpectedActivityPeriod(pub u8);

impl APerElement for ExpectedActivityPeriod {
    fn from_aper(decoder: &mut Decoder, _constraints: Constraints) -> Result<Self, DecodeError> {
        Ok(Self(u8::from_aper(decoder, Constraints::value(Some(1), Some(50), true))?))
    }
    fn to_aper(&self, enc: &mut Encoding, _constraints: Constraints) -> Result<(), EncodeError> {
        self.0.to_aper(enc, Constraints::value(Some(1), Some(50), true))
    }
}

""")


if __name__ == '__main__':
    if len(sys.argv) == 2:
        print(generate_structs(sys.argv[1], verbose=True))
    else:
        unittest.main(failfast=True)
