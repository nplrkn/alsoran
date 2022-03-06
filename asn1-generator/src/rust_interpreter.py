#!/usr/bin/env python3

from pickle import FALSE
import unittest
from lark.visitors import Interpreter
from case import pascal_case, snake_case
from lark.lexer import Token
from lark import Tree, Lark
from parser import parse_string, parse_file

global_name_dict = dict()


def unique_type_name(name):
    global global_name_dict
    name = pascal_case(name)
    existing = global_name_dict.get(name)
    global_name_dict[name] = (existing or 0) + 1
    return (name + str(existing)) if existing is not None else name


EXTENSION_TO = """enc.append( & false.to_aper(UNCONSTRAINED)?)?;"""
OPTIONALS_TO = """enc.append(&optionals.to_aper(Self::CONSTRAINTS)?)?;"""
EXTENSION_FROM = """let _extended = bool::from_aper(decoder, UNCONSTRAINED)?;"""
OPTIONALS_FROM = """let optionals = BitString::from_aper(decoder, Self::CONSTRAINTS)?;"""
FIXED_SIZE_CONSTRAINTS = \
    """const CONSTRAINTS: Constraints = Constraints {{
        value: None,
        size: Some(Constraint::new(Some({num_optionals}), Some({num_optionals}))),
    }};"""
UNCONSTRAINED_CONSTRAINTS = """const CONSTRAINTS: Constraints = UNCONSTRAINED;"""


class TypeInterpreter(Interpreter):
    def __init__(self):
        self.output = ""
        self.lb = None
        self.ub = None

    def sequenceof(self, tree):
        item = tree.children[2]
        if isinstance(item, Tree):
            # It must be a container
            assert(item.data == "container")
            item = item.children[1].replace("IEs", "")
        self.output = "Vec<" + pascal_case(item) + ">"

    def get_bounds(self, tree):
        if len(tree.children) > 1:
            self.lb = int(tree.children[0])
            self.ub = tree.children[1]
            if self.ub is None:
                self.ub = self.lb
            else:
                self.ub = int(self.ub)

    def integer(self, tree):
        self.get_bounds(tree)
        if self.ub < 256:
            self.output = "u8"
        elif self.ub < 65536:
            self.output = "u16"
        elif self.ub < 4294967295:
            self.output = "u32"
        else:
            self.output = "u64"

    def bits(self, tree):
        self.output = "BitString"
        self.get_bounds(tree)

    def string(self, _tree):
        self.output = "String"

    def bytes(self, _tree):
        self.output = "Vec<u8>"

    def boolean(self, _tree):
        self.output = "bool"


def fix_up_turbofish(s):
    # For example, turn "Vec<u8>::from_aper" -> "Vec::<u8>::from_aper"
    return s.replace("Vec<u8>::from_aper", "Vec::<u8>::from_aper")


class StructFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.self_fields = ""

    def field(self, tree):
        name = tree.children[0]
        self.self_fields += f"            {name},\n"
        typ = tree.children[1]
        self.fields_from += f"""\
        let {name} = {typ}::from_aper(decoder, UNCONSTRAINED)?;
"""

    def optional_field(self, tree):
        name = tree.children[0]
        self.self_fields += f"            {name},\n"
        typ = tree.children[1]

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
    optionals.set({self.num_optionals}, self.{name}.is_some());
"""
        self.num_optionals += 1

    def optional_extension_container(self, tree):
        self.num_optionals += 1


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


MUT_OPTIONALS = """
        let mut optionals = BitString::with_len({num_optionals});
"""


# class StructFields(Interpreter):
#     def __init__(self):
#         self.fields = ""

#     def field(self, tree):
#         name = tree.children[0]
#         typ = tree.children[1]
#         self.fields += f"""\
#     pub {name}: {typ}
# """


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
        name = unique_type_name(orig_name)
        self.in_enum = True
        self.visit(tree.children[1])
        self.in_enum = False
        fields = self.output
        self.output = ""

        assert(len(tree.children[1].children) <= 256)
        typ = "u8"

        self.outfile += f"""\
// {orig_name}
# [derive(Clone, Copy, FromPrimitive)]
pub enum {name} {{
{fields}\
}}

impl APerElement for {name} {{
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {{
        if bool::from_aper(decoder, Self::CONSTRAINTS)? {{
            Ok({name}::Extended)
        }} else {{
            let v = {typ}::from_aper(decoder, Self::CONSTRAINTS)?;
            FromPrimitive::from_{typ}(v).ok_or(DecodeError::MalformedInt)
        }}
    }}
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {{
        let mut enc = Encoding::new();
        enc.append(&false.to_aper(Self::CONSTRAINTS)?)?;
        enc.append(&(*self as {typ}).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
    }}
}}

"""
        return name

    def choicedef(self, tree):
        self.enum(tree)

    # def enum(self, tree):
    #     self.comment(tree)
    #     name = unique_type_name(tree.children[0])
    #     self.output += "pub enum " + name
    #     self.in_enum = True
    #     self.field_block(tree.children[1])
    #     self.in_enum = False
    #     return name

    def tuple_struct(self, tree):
        orig_name = tree.children[0]
        name = unique_type_name(orig_name)
        (inner, lb, ub) = self.get_type(tree)

        if lb is not None:
            output = BOUNDED_NEWTYPE_FORMAT.format(
                orig_name=orig_name, name=name, type=inner, lb=lb, ub=ub)
        else:
            output = UNBOUNDED_NEWTYPE_FORMAT.format(
                orig_name=orig_name, name=name, type=inner)

        self.outfile = fix_up_turbofish(output)

        return name

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
        name = unique_type_name(orig_name)
        self.visit(tree.children[1])
        fields = self.output
        num_optionals = 3
        extensible = True

        # self.comment(tree)
        # name = unique_type_name(tree.children[0])
        # self.struct_start(name)
        # self.field_block(tree.children[1])

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
{fields}\
}}

impl APerElement for {name} {{
    {FIXED_SIZE_CONSTRAINTS.format(
        num_optionals=num_optionals) if num_optionals > 0 else UNCONSTRAINED_CONSTRAINTS}
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {{
        {EXTENSION_FROM if extensible else "" }
        {OPTIONALS_FROM if num_optionals > 0 else "" }
{fields_from_interpreter.fields_from}
        Ok(Self {{
{fields_from_interpreter.self_fields}
        }})
    }}
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {{
        let mut enc = Encoding::new();
        {MUT_OPTIONALS.format(num_optionals=num_optionals)
                              if num_optionals > 0 else ""}
{find_opt_interpreter.find_optionals if num_optionals > 0 else ""}
        {EXTENSION_TO if extensible else ""}
        {OPTIONALS_TO if num_optionals > 0 else ""}
{fields_to_interpreter.fields_to}
        Ok(enc)
    }}
}}
"""
        return name

    def get_type(self, tree, optional):
        if len(tree.children) < 2:
            return (None, None, None)
        typ = tree.children[1]
        lb = None
        ub = None
        if isinstance(typ, Token):
            name = pascal_case(typ)
        elif typ.data == 'enumerated':
            # inline anonymous enumerated
            i = StructInterpreter()
            name = i.enumdef(tree)
            self.outfile += i.outfile
        elif typ.data == 'sequence':
            # inline anonymous sequence
            i = StructInterpreter()
            name = i.struct(tree)
            self.outfile += i.outfile
        else:
            i = TypeInterpreter()
            i.visit(typ)
            name = i.output
            lb = i.lb
            ub = i.ub
        if optional:
            name = "Option<" + name + ">"
        return (name, lb, ub)

    def comment(self, tree, comment=""):
        if comment != "":
            comment = " - " + comment
        self.output += "// " + tree.children[0] + comment + "\n"

    def objectdef(self, tree):
        name = tree.children[0].replace("IEs", "")

        ies = [child for child in tree.children[2].children if child.data == "ie"]

        # Omit if there are 0, as is normally the case for extension IEs
        if len(ies) == 0:
            self.comment(tree, "omitted\n")
            return

        # If this is a list item container, then it will have a single ie with the same name.
        # Omit it in this case.
        if len(ies) == 1 and ies[0].children[0] == name:
            self.comment(tree, "omitted\n")
            return

        name = unique_type_name(name)
        self.comment(tree)
        self.struct_start(name)
        self.field_block(tree.children[2])

    def field_block(self, tree):
        self.output += " {\n"
        self.visit(tree)
        self.output += "}\n\n"
        self.flush()

    def optional_extension_container(self, tree):
        pass

    def extended_item(self, tree):
        assert(False)

    def extension_marker(self, tree):
        if self.in_enum:
            self.output += "    Extended,\n"

    def field(self, tree, optional=False):
        name = tree.children[0]
        (typ, _lb, _ub) = self.get_type(tree, optional)
        if typ is None:
            # Enumerated
            name = pascal_case(name)
            self.output += "    " + name
        else:
            if self.in_enum:
                # Choice
                name = pascal_case(name)
                self.output += "    " + name
                if typ != "Null":
                    self.output += "(" + typ + ")"
            else:
                # Sequence
                name = snake_case(name)
                self.output += "    pub " + name + ": " + typ
        self.output += ",\n"

    def optional_field(self, tree):
        assert (not self.in_enum)
        self.field(tree, optional=True)

    def flush(self):
        # print(self.output)
        self.outfile += self.output
        self.output = ""


def generate(tree):
    global global_name_dict
    global_name_dict = dict()  # TODO get rid of this global
    visited = StructInterpreter()
    visited.visit(tree)
    return visited.outfile


def generate_structs(input_file="f1ap/asn1/F1AP-PDU-Contents.asn"):
    tree = parse_file(input_file)
    # print(tree.pretty())
    return generate(tree)


# print(generate_structs())


class TestGenerator(unittest.TestCase):
    maxDiff = None

    def should_generate(self, input, expected):
        output = ""
        tree = parse_string(input)
        try:
            output = generate(tree)
            self.assertEqual(output, expected)
        finally:
            if output != expected:
                print(tree.pretty())

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
// wlan-rtt
# [derive(Clone, Copy, FromPrimitive)]
pub enum WlanRtt {
    Thing1,
    Extended,
}

impl APerElement for WlanRtt {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, Self::CONSTRAINTS)? {
            Ok(WlanRtt::Extended)
        } else {
            let v = u8::from_aper(decoder, Self::CONSTRAINTS)?;
            FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
        }
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&false.to_aper(Self::CONSTRAINTS)?)?;
        enc.append(&(*self as u8).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
    }
}

// WLANMeasurementConfiguration
pub struct WlanMeasurementConfiguration {
    pub wlan_meas_config: WlanMeasConfig,
    pub wlan_rtt: Option<WlanRtt>,
}

impl APerElement for WlanMeasurementConfiguration {
    const CONSTRAINTS: Constraints = Constraints {
        value: None,
        size: Some(Constraint::new(Some(2), Some(2))),
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

        enc.append(& false.to_aper(UNCONSTRAINED)?)?;
        enc.append(&optionals.to_aper(Self::CONSTRAINTS)?)?;
        enc.append(& self.wlan_meas_config.to_aper(UNCONSTRAINED)?);
        if let Some(x) = self.wlan_rtt {
            enc.append(&x.to_aper(UNCONSTRAINED)?); }
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
// LTEUERLFReportContainer
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
# [derive(Clone, Copy, FromPrimitive)]
pub enum MaximumIntegrityProtectedDataRate {
    Bitrate64kbs,
    MaximumUeRate,
    Extended,
}

impl APerElement for MaximumIntegrityProtectedDataRate {
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {
        if bool::from_aper(decoder, Self::CONSTRAINTS)? {
            Ok(MaximumIntegrityProtectedDataRate::Extended)
        } else {
            let v = u8::from_aper(decoder, Self::CONSTRAINTS)?;
            FromPrimitive::from_u8(v).ok_or(DecodeError::MalformedInt)
        }
    }
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {
        let mut enc = Encoding::new();
        enc.append(&false.to_aper(Self::CONSTRAINTS)?)?;
        enc.append(&(*self as u8).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
    }
}

"""
        self.should_generate(input, output)


BOUNDED_NEWTYPE_FORMAT = """\
// {orig_name}
pub struct {name}(pub {type});

impl APerElement for {name} {{
    const CONSTRAINTS: Constraints = Constraints {{
        value: None,
        size: Some(Constraint {{
            min: Some({lb}),
            max: Some({ub}),
        }}),
    }};
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {{
        Ok(Self({type}::from_aper(decoder, Self::CONSTRAINTS)?))
    }}
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {{
        let mut enc = Encoding::new();
        enc.append(&(self.0).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
    }}
}}

"""

UNBOUNDED_NEWTYPE_FORMAT = """\
// {orig_name}
pub struct {name}(pub {type});

impl APerElement for {name} {{
    const CONSTRAINTS: Constraints = UNCONSTRAINED;
    fn from_aper(decoder: &mut Decoder, constraints: Constraints) -> Result<Self, DecodeError> {{
        Ok(Self({type}::from_aper(decoder, Self::CONSTRAINTS)?))
    }}
    fn to_aper(&self, constraints: Constraints) -> Result<Encoding, EncodeError> {{
        let mut enc = Encoding::new();
        enc.append(&(self.0).to_aper(Self::CONSTRAINTS)?)?;
        Ok(enc)
    }}
}}

"""

if __name__ == '__main__':
    unittest.main()
