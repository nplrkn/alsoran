#!/usr/bin/env python3

from pickle import FALSE
import unittest
from lark.visitors import Interpreter
from case import pascal_case, snake_case
from lark.lexer import Token
from lark import Tree, Lark
from parser import parse_string, parse_file
from transformer import transform


EXTENSION_TO = """
        enc.append(&false.to_aper(UNCONSTRAINED)?)?;"""
OPTIONALS_TO = """enc.append(&optionals.to_aper(Self::CONSTRAINTS)?)?;"""
EXTENSION_FROM = """let _extended = bool::from_aper(decoder, UNCONSTRAINED)?;"""
OPTIONALS_FROM = """let optionals = BitString::from_aper(decoder, Self::CONSTRAINTS)?;"""
BOUNDED_CONSTRAINTS = \
    """const CONSTRAINTS: Constraints = Constraints {{
        value: None,
        size: Some(Constraint {{
            min: Some({lb}),
            max: Some({ub}),
        }}),
    }};"""
UNCONSTRAINED_CONSTRAINTS = """const CONSTRAINTS: Constraints = UNCONSTRAINED;"""


class StructFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.self_fields = ""

    def field(self, tree):
        name = tree.children[0]
        self.self_fields += f"            {name},\n"
        typ = tree.children[1]
        if isinstance(typ, Tree):
            typ = typ.data
        self.fields_from += f"""\
        let {name} = {typ}::from_aper(decoder, UNCONSTRAINED)?;
"""

    def optional_field(self, tree):
        name = tree.children[0]
        self.self_fields += f"            {name},\n"
        typ = tree.children[1]
        if isinstance(typ, Tree):
            typ = typ.data

        self.fields_from += f"""\
        let {name} = if optionals.is_set(0) {{
            Some({typ}::from_aper(decoder, UNCONSTRAINED)?)
        }} else {{
            None
        }};
"""


class StructFindOptionals(Interpreter):
    def __init__(self):
        self.find_optionals = ""
        self.num_optionals = 0

    def optional_field(self, tree):
        name = tree.children[0]
        self.find_optionals += f"""\
        optionals.set({self.num_optionals}, self.{name}.is_some());"""
        self.num_optionals += 1

    def extension_container(self, tree):
        self.num_optionals += 1


class EnumFields(Interpreter):
    def __init__(self):
        self.enum_fields = ""
        self.extensible = False

    def enum_item(self, tree):
        self.enum_fields += f"""\
    {tree.children[0]},
"""

    def extension_marker(self, _tree):
        self.extensible = True
        self.enum_fields += f"""\
    _Extended,
"""

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
    {name}{"("+typ+")" if typ != "Null" else ""},
"""

    def extension_container(self, tree):
        self.choice_fields += f"""\
    _Extended,
"""

    def extension_marker(self, tree):
        print("Warning - extensible CHOICE not implemented")


class ChoiceFieldsTo(Interpreter):
    def __init__(self):
        self.fields_to = ""
        self.field_index = 0

    def choicefield(self, tree):
        name = tree.children[0]
        typ = tree.children[1]
        if typ != "Null":
            self.fields_to += f"""\
            Self::{name}(x) => {{
                enc.append(&({self.field_index} as u8).to_aper(UNCONSTRAINED)?);
                enc.append(&x.to_aper(UNCONSTRAINED)?); }}
"""
        else:
            self.fields_to += f"""\
            Self::{name} => {{
                enc.append(&({self.field_index} as u8).to_aper(UNCONSTRAINED)?); }}
"""
        self.field_index += 1

    def extension_container(self, tree):
        self.fields_to += f"""\
            Self::_Extended => Err(EncodeError::NotImplemented)
"""
        self.field_index += 1


class ChoiceFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.field_index = 0

    def choicefield(self, tree):
        name = tree.children[0]
        typ = tree.children[1]
        if isinstance(typ, Tree):
            typ = typ.data

        if typ != "Null":
            self.fields_from += f"""\
            {self.field_index} => Ok(Self::{name}({typ}::from_aper(decoder, UNCONSTRAINED)?)),
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


class StructFieldsTo(Interpreter):
    def __init__(self):
        self.fields_to = ""

    def field(self, tree):
        name = tree.children[0]
        self.fields_to += f"""\
        enc.append(&self.{name}.to_aper(UNCONSTRAINED)?);
"""

    def optional_field(self, tree):
        name = tree.children[0]
        self.fields_to += f"""\
        if let Some(x) = self.{name} {{
            enc.append(&x.to_aper(UNCONSTRAINED)?);
        }}
"""


MUT_OPTIONALS = """let mut optionals = BitString::with_len({num_optionals});"""

ENUM_EXTENSION_FROM = """
        if bool::from_aper(decoder, Self::CONSTRAINTS)? {{
            return Ok({name}::_Extended)
        }}"""


class StructInterpreter(Interpreter):

    def __init__(self):
        self.output = ""
        self.outfile = ""
        self.in_enum = False

    def struct_start(self, s):
        self.output += "pub struct " + s

    def extended_items(self, tree):
        pass

    def enumdef(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        field_interpreter = EnumFields()
        field_interpreter.visit(tree.children[1])

        assert(len(tree.children[1].children) <= 256)
        typ = "u8"

        self.outfile += f"""\
// {orig_name}
#[derive(Clone, Copy, FromPrimitive)]
pub enum {name} {{
{field_interpreter.enum_fields}\
}}

impl APerElement for {name} {{
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {{\
{ENUM_EXTENSION_FROM.format(name=name) if field_interpreter.extensible else ""}
        let v = {typ}::from_aper(decoder, Self::CONSTRAINTS)?;
        FromPrimitive::from_{typ}(v).ok_or(DecodeError::MalformedInt)
    }}
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {{
        let mut enc = Encoding::new();\
{EXTENSION_TO if field_interpreter.extensible else ""}
        enc.append(&(*self as {typ}).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
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
pub enum {name} {{
{field_interpreter.choice_fields}\
}}

impl APerElement for {name} {{
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {{
        match u8::from_aper(decoder, UNCONSTRAINED)? {{
{fields_from_interpreter.fields_from}\
            _ => Err(DecodeError::InvalidChoice)
        }}
    }}
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {{
        let mut enc = Encoding::new();
        match self {{
{fields_to_interpreter.fields_to}\
        }}
        Ok(enc)
    }}
}}


"""

    def tuple_struct(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        inner = tree.children[1].data
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
pub struct {name}(pub {inner});

impl APerElement for {name} {{
    {BOUNDED_CONSTRAINTS.format(
        lb=lb, ub=ub) if lb is not None else UNCONSTRAINED_CONSTRAINTS}
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {{
        Ok(Self({"Vec::<u8>" if inner == "Vec<u8>" else inner}::from_aper(decoder, Self::CONSTRAINTS)?))
    }}
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {{
        let mut enc = Encoding::new();
        enc.append(&(self.0).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
    }}
}}

"""
        self.outfile = output

    def ie(self, tree):
        name = snake_case(tree.children[0])
        self.output += "  pub " + name + ": "
        s = StructInterpreter()
        self.output += s.get_type(tree) + ",\n"
        assert(s.outfile == "")  # Can't handle inline enum here

    def struct(self, tree):
        fields = [
            child for child in tree.children[1].children if child.data in ["field", "optional_field"]]

        # Omit if there are 0 fields, as is normally the case for extension IEs
        if len(fields) == 0:
            self.comment(tree, "omitted\n")
            return

        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        field_interpreter = StructFields()
        field_interpreter.visit(tree.children[1])

        fields_from_interpreter = StructFieldsFrom()
        fields_from_interpreter.visit(tree.children[1])

        find_opt_interpreter = StructFindOptionals()
        find_opt_interpreter.visit(tree.children[1])
        num_optionals = find_opt_interpreter.num_optionals

        fields_to_interpreter = StructFieldsTo()
        fields_to_interpreter.visit(tree.children[1])

        self.outfile += f"""\
// {orig_name}
pub struct {name} {{
{field_interpreter.struct_fields}\
}}

impl APerElement for {name} {{
    {BOUNDED_CONSTRAINTS.format(
        lb=num_optionals, ub=num_optionals) if num_optionals > 0 else UNCONSTRAINED_CONSTRAINTS}
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {{
        {EXTENSION_FROM if field_interpreter.extensible else ""}
        {OPTIONALS_FROM if num_optionals > 0 else ""}
{fields_from_interpreter.fields_from}
        Ok(Self {{
{fields_from_interpreter.self_fields}\
        }})
    }}
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {{
        let mut enc = Encoding::new();
        {MUT_OPTIONALS.format(num_optionals=num_optionals)
                              if num_optionals > 0 else ""}
{find_opt_interpreter.find_optionals if num_optionals > 0 else ""}
{EXTENSION_TO if field_interpreter.extensible else ""}
        {OPTIONALS_TO if num_optionals > 0 else ""}
{fields_to_interpreter.fields_to}
        Ok(enc)
    }}
}}

"""
        return name

    def comment(self, tree, comment=""):
        if comment != "":
            comment = " - " + comment
        self.output += "// " + tree.children[0] + comment + "\n"

    def objectdef(self, tree):
        print("Warning - objectdef not implemented")
        # name = tree.children[0].replace("IEs", "")

        # ies = [child for child in tree.children[2].children if child.data == "ie"]

        # # Omit if there are 0, as is normally the case for extension IEs
        # if len(ies) == 0:
        #     self.comment(tree, "omitted\n")
        #     return

        # # If this is a list item container, then it will have a single ie with the same name.
        # # Omit it in this case.
        # if len(ies) == 1 and ies[0].children[0] == name:
        #     self.comment(tree, "omitted\n")
        #     return

        # assert(False)

        # self.comment(tree)
        # self.struct_start(name)
        # self.field_block(tree.children[2])

    # def field_block(self, tree):
    #     self.output += " {\n"
    #     self.visit(tree)
    #     self.output += "}\n\n"
    #     self.flush()

    def extension_container(self, tree):
        pass

    def extended_item(self, tree):
        assert(False)

    def extension_marker(self, tree):
        if self.in_enum:
            self.output += "    _Extended,\n"

    # def field(self, tree):
    #     name = tree.children[0]
    #     typ = tree.children[1]
    #     if typ is None:
    #         # Enumerated
    #         name = pascal_case(name)
    #         self.output += "    " + name
    #     else:
    #         if self.in_enum:
    #             # Choice
    #             name = pascal_case(name)
    #             self.output += "    " + name
    #             if typ != "Null":
    #                 self.output += "(" + typ + ")"
    #         else:
    #             # Sequence
    #             name = snake_case(name)
    #             self.output += "    pub " + name + ": " + typ
    #     self.output += ",\n"

    # def optional_field(self, tree):
    #     assert (not self.in_enum)
    #     self.field(tree, optional=True)

    # def flush(self):
    #     # print(self.output)
    #     self.outfile += self.output
    #     self.output = ""


def generate(tree, constants=dict()):
    tree = transform(tree, constants)
    # print(tree.pretty())
    visited = StructInterpreter()
    print("---- Generating ----")
    visited.visit(tree)
    return visited.outfile


def generate_structs(input_file="f1ap/asn1/F1AP-PDU-Contents.asn", constants=dict()):
    tree = parse_file(input_file)
    # print(tree.pretty())
    return generate(tree, constants)


class TestGenerator(unittest.TestCase):
    maxDiff = None

    def should_generate(self, input, expected):
        output = ""
        tree = parse_string(input)
        try:
            output = generate(tree)
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
#[derive(Clone, Copy, FromPrimitive)]
pub enum TriggeringMessage {
    InitiatingMessage,
    SuccessfulOutcome,
    UnsuccessfulOutcome,
}

impl APerElement for TriggeringMessage {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        let v = u8::from_aper(decoder, Self::CONSTRAINTS)?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&(*self as u8).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
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
pub struct WlanMeasurementConfiguration {
    pub wlan_meas_config: WlanMeasConfig,
    pub wlan_rtt: Option<WlanRtt>,
}

impl APerElement for WlanMeasurementConfiguration {
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: Some(Constraint {
            min: Some(2),
            max: Some(2),
        }),
    };
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        let _extended = bool::from_aper(decoder, UNCONSTRAINED)?;
        let optionals = BitString::from_aper(decoder, Self::CONSTRAINTS)?;
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
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        let mut optionals = BitString::with_len(2);
        optionals.set(0, self.wlan_rtt.is_some());

        enc.append(&false.to_aper(UNCONSTRAINED)?)?;
        enc.append(&optionals.to_aper(Self::CONSTRAINTS)?)?;
        enc.append(&self.wlan_meas_config.to_aper(UNCONSTRAINED)?);
        if let Some(x) = self.wlan_rtt {
            enc.append(&x.to_aper(UNCONSTRAINED)?);
        }

        Ok(enc)
    }
}

// WlanRtt
#[derive(Clone, Copy, FromPrimitive)]
pub enum WlanRtt {
    Thing1,
    _Extended,
}

impl APerElement for WlanRtt {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, Self::CONSTRAINTS)? {
            return Ok(WlanRtt::_Extended)
        }
        let v = u8::from_aper(decoder, Self::CONSTRAINTS)?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&false.to_aper(UNCONSTRAINED)?)?;
        enc.append(&(*self as u8).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
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
pub struct LteueRlfReportContainer(pub Vec<u8>);

impl APerElement for LteueRlfReportContainer {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        Ok(Self(Vec::<u8>::from_aper(decoder, Self::CONSTRAINTS)?))
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&(self.0).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
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
pub struct MaximumDataBurstVolume(pub u16);

impl APerElement for MaximumDataBurstVolume {
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: Some(Constraint {
            min: Some(0),
            max: Some(4095),
        }),
    };
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        Ok(Self(u16::from_aper(decoder, Self::CONSTRAINTS)?))
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&(self.0).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
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
pub struct MobilityInformation(pub BitString);

impl APerElement for MobilityInformation {
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: Some(Constraint {
            min: Some(16),
            max: Some(16),
        }),
    };
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        Ok(Self(BitString::from_aper(decoder, Self::CONSTRAINTS)?))
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&(self.0).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
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
#[derive(Clone, Copy, FromPrimitive)]
pub enum MaximumIntegrityProtectedDataRate {
    Bitrate64kbs,
    MaximumUeRate,
    _Extended,
}

impl APerElement for MaximumIntegrityProtectedDataRate {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, Self::CONSTRAINTS)? {
            return Ok(MaximumIntegrityProtectedDataRate::_Extended)
        }
        let v = u8::from_aper(decoder, Self::CONSTRAINTS)?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&false.to_aper(UNCONSTRAINED)?)?;
        enc.append(&(*self as u8).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
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
pub enum EventTrigger {
    OutOfCoverage(OutOfCoverage),
    EventL1LoggedMdtConfig,
    ShortMacroEnbId(BitString),
    _Extended,
}

impl APerElement for EventTrigger {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        match u8::from_aper(decoder, UNCONSTRAINED)? {
            0 => Ok(Self::OutOfCoverage(OutOfCoverage::from_aper(decoder, UNCONSTRAINED)?)),
            1 => Ok(Self::EventL1LoggedMdtConfig),
            2 => Ok(Self::ShortMacroEnbId(BitString::from_aper(decoder, UNCONSTRAINED)?)),
            3 => Err(DecodeError::NotImplemented),
            _ => Err(DecodeError::InvalidChoice)
        }
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        match self {
            Self::OutOfCoverage(x) => {
                enc.append(&(0 as u8).to_aper(UNCONSTRAINED)?);
                enc.append(&x.to_aper(UNCONSTRAINED)?); }
            Self::EventL1LoggedMdtConfig => {
                enc.append(&(1 as u8).to_aper(UNCONSTRAINED)?); }
            Self::ShortMacroEnbId(x) => {
                enc.append(&(2 as u8).to_aper(UNCONSTRAINED)?);
                enc.append(&x.to_aper(UNCONSTRAINED)?); }
            Self::_Extended => Err(EncodeError::NotImplemented)
        }
        Ok(enc)
    }
}


// OutOfCoverage
#[derive(Clone, Copy, FromPrimitive)]
pub enum OutOfCoverage {
    True,
    _Extended,
}

impl APerElement for OutOfCoverage {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, Self::CONSTRAINTS)? {
            return Ok(OutOfCoverage::_Extended)
        }
        let v = u8::from_aper(decoder, Self::CONSTRAINTS)?;
        FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&false.to_aper(UNCONSTRAINED)?)?;
        enc.append(&(*self as u8).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
    }
}

""")


if __name__ == '__main__':
    # unittest.main()
    unittest.main(failfast=True)
