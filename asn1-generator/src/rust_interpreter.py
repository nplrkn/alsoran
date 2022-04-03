#!/usr/bin/env python3

import sys
from pickle import FALSE
import unittest
from lark.visitors import Interpreter
from case import snake_case
from lark import Tree
from parser import parse_string, parse_file
from transformer import transform


def bool_to_rust(b):
    return "true" if b else "false"


class TypeInfo:
    def __init__(self):
        self.constraints = "None, None, false"
        self.extra_type = None
        self.seqof = None
        self.typ = None
        self.criticality = None
        self.code = None
        self.inner_type_info = None


# TODO - replace with a visitor?
def type_and_constraints(node):
    type_info = TypeInfo()
    type_info.typ = node

    if isinstance(type_info.typ, Tree):
        if type_info.typ.data in ["ie", "optional_ie"]:
            # The ProtocolIE-SingleContainer case where an IE is inside a sequence of.
            parent = type_info.typ
            type_info = type_and_constraints(parent.children[3])
            type_info.code = parent.children[1]
            type_info.criticality = parent.children[2]
            return type_info

        bounds = type_info.typ.children
        type_info.typ = type_info.typ.data
        ext = "false"

        if type_info.typ in ["sequence_of", "ie_container_sequence_of"]:
            type_info.seqof = type_info.typ
            inner = bounds[-1]
            type_info.inner_type_info = type_and_constraints(inner)
            bounds = bounds[0: -1]
            type_info.typ = f"Vec<{type_info.inner_type_info.typ}>"

        if len(bounds) > 1:
            MAX_U64 = 18446744073709551615
            MIN_I64 = -9223372036854775808
            lb = MAX_U64
            ub = MIN_I64
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

            if ub in [MAX_U64, MIN_I64]:
                ub = "None"
            else:
                ub = f"Some({ub})"

            type_info.constraints = f"Some({lb}), {ub}, {ext}"

    if type_info.typ in ["VisibleString", "PrintableString", "UTF8String"]:
        type_info.extra_type = snake_case(type_info.typ)
        type_info.typ = "String"

    return type_info


def decode_expression(tree):
    type_info = type_and_constraints(tree)
    if type_info.seqof:
        ie_extra_lines = """
                let _ = aper::decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = aper::decode::decode_length_determinent(data, None, None, false)?;""" if type_info.seqof == "ie_container_sequence_of" else ""
        return f"""{{
            let length = aper::decode::decode_length_determinent(data, {type_info.constraints})?;
            let mut items = vec![];
            for _ in 0..length {{\
{ie_extra_lines}
                items.push({(decode_expression(tree.children[2]))});
            }}
            items
        }}"""
    elif type_info.typ == "Vec<u8>":
        return f"aper::decode::decode_octetstring(data, {type_info.constraints})?"
    elif type_info.typ == "BitString":
        return f"aper::decode::decode_bitstring(data, {type_info.constraints})?"
    elif type_info.typ == "String":
        return f"aper::decode::decode_{type_info.extra_type}(data, {type_info.constraints})?"
    elif type_info.typ == "i128":
        return f"aper::decode::decode_integer(data, {type_info.constraints})?.0"
    elif type_info.typ in ["u8", "u16", "u32", "u64"]:
        return f"aper::decode::decode_integer(data, {type_info.constraints})?.0 as {type_info.typ}"
    elif type_info.typ == "bool":
        return f"aper::decode::decode_bool(data)?"
    else:
        return f"""{type_info.typ}::decode(data)?"""


def encode_expression_fn(tree):
    type_info = type_and_constraints(tree)
    if type_info.seqof == "ie_container_sequence_of":
        return lambda x, data="data": f"""\
aper::encode::encode_length_determinent({data}, {type_info.constraints}, self.0.len())?;
        for x in {x} {{
            let ie = &mut AperCodecData::new();
            {encode_expression_fn(tree.children[2])("x", "ie")}?;
            aper::encode::encode_integer({data}, Some(0), Some(65535), false, ({type_info.inner_type_info.code}, false))?;
            Criticality::{type_info.inner_type_info.criticality.title()}.encode({data})?;
            aper::encode::encode_length_determinent({data}, None, None, false, ie.length_in_bytes())?;
            {data}.append_aligned(ie)?;
        }}
        Ok(())"""
    elif type_info.seqof:
        return lambda x, data="data": f"""\
aper::encode::encode_length_determinent({data}, {type_info.constraints}, self.0.len())?;
        for x in {x} {{
            {encode_expression_fn(tree.children[2])("x", data)}?;
        }}
        Ok(())"""

    if type_info.typ == "Vec<u8>":
        format_string = f"aper::encode::encode_octetstring({{data}}, {type_info.constraints}, &{{value}})"
    elif type_info.typ == "BitString":
        format_string = f"aper::encode::encode_bitstring({{data}}, {type_info.constraints}, &{{value}})"
    elif type_info.typ == "String":
        format_string = f"aper::encode::encode_{type_info.extra_type}({{data}}, {type_info.constraints}, &{{value}})"
    elif type_info.typ == "i128":
        format_string = f"aper::encode::encode_integer({{data}}, {type_info.constraints}, ({{value}}, false))"
    elif type_info.typ in ["u8", "u16", "u32", "u64"]:
        format_string = f"aper::encode::encode_integer({{data}}, {type_info.constraints}, ({{value}}, false))"
    elif type_info.typ == "bool":
        format_string = f"aper::encode::encode_bool({{data}}, {{value}})"
    else:
        format_string = f"""{{value}}.encode({{data}})"""

    return lambda x, data="data": format_string.format(value=x, data=data)


class StructFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.self_fields = ""
        self.optional_idx = 0

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
        let {name} = if optionals[{self.optional_idx}] {{
            Some({decode_expression(tree.children[1])})
        }} else {{
            None
        }};
"""
        self.optional_idx += 1


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

    def enum_field(self, tree):
        self.variants += 1
        self.enum_fields += f"""\
    {tree.children[0]},
"""

    def extension_marker(self, _tree):
        self.extensible = True

    def extended_items(self, _tree):
        pass


class ChoiceFields(Interpreter):
    def __init__(self):
        self.choice_fields = ""
        self.extensible = False

    def choice_field(self, tree):
        name = tree.children[0]
        typ = tree.children[1]

        if isinstance(typ, Tree):
            typ = typ.data
        self.choice_fields += f"""\
    {name}{"("+typ+")" if typ != "null" else ""},
"""

    def extension_marker(self, tree):
        self.extensible = True


class ChoiceFieldsTo(Interpreter):
    def __init__(self, num_choices, extensible):
        self.fields_to = ""
        self.field_index = 0
        self.num_choices = num_choices
        self.extensible = extensible

    def choice_field(self, tree):
        name = tree.children[0]
        type_info = type_and_constraints(tree.children[1])

        self.fields_to += f"""\
            Self::{name}(x) => {{
                aper::encode::encode_choice_idx(data, 0, {self.num_choices}, {bool_to_rust(self.extensible)}, ({self.field_index}, false))?;
                {encode_expression_fn(tree.children[1])("x") if type_info.typ != "null" else ""}
            }}
"""
        self.field_index += 1

    def extension_container(self, tree):
        self.field_index += 1


class ChoiceFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.field_index = 0

    def choice_field(self, tree):
        name = tree.children[0]
        typ = type_and_constraints(tree.children[1]).typ

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
        typ = type_and_constraints(tree.children[1]).typ
        self.struct_fields += f"""\
    pub {name}: {typ},
"""

    def optional_field(self, tree):
        name = tree.children[0]
        typ = type_and_constraints(tree.children[1]).typ
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
        self.fields_to = ""

    def extension_marker(self, tree):
        self.extensible = True

    def ie(self, tree):
        name = tree.children[0]
        id = tree.children[1]
        criticality = tree.children[2].capitalize()
        typ = type_and_constraints(tree.children[3]).typ
        self.struct_fields += f"""\
    pub {name}: {typ},
"""
        self.mut_field_vars += f"""\
        let mut {name}: Option<{typ}> = None;
"""
        self.self_fields += f"            {name},\n"
        self.matches += f"""\
                {id} => {name} = Some({decode_expression(tree.children[3])}),
"""
        self.num_mandatory_fields += 1
        self.mandatory += f"""\
        let {name} = {name}.ok_or(aper::AperCodecError::new(format!(
            "Missing mandatory IE {name}"
        )))?;
"""
        self.fields_to += f"""
        let ie = &mut AperCodecData::new();
        self.{name}.encode(&mut ie)?;
        aper::encode::encode_integer(ies, Some(0), Some(65535), false, ({id}, false))?;
        Criticality::{criticality.title()}.encode(ies)?;
        aper::encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
        ies.append_aligned(ie)?;
        num_ies += 1;
"""

    def optional_ie(self, tree):
        name = tree.children[0]
        id = tree.children[1]
        criticality = tree.children[2].capitalize()
        typ = tree.children[3]
        typ = type_and_constraints(tree.children[3]).typ
        self.struct_fields += f"""\
    pub {name}: Option<{typ}>,
"""
        self.mut_field_vars += f"""\
        let mut {name}: Option<{typ}> = None;
"""
        self.self_fields += f"            {name},\n"
        self.matches += f"""\
                {id} => {name} = Some({decode_expression(tree.children[3])}),
"""
        self.optionals_presence_list += f"self.{name}.is_some(),"
        self.fields_to += f"""
        if let Some(x) = &self.{name} {{
            let ie = &mut AperCodecData::new();
            {encode_expression_fn(tree.children[3])("x", "ie")}?;
            aper::encode::encode_integer(ies, Some(0), Some(65535), false, ({id}, false))?;
            Criticality::{criticality.title()}.encode(ies)?;
            aper::encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
            ies.append_aligned(ie)?;
            num_ies += 1;
        }}
"""


class StructFieldsTo(Interpreter):
    def __init__(self):
        self.fields_to = ""
        self.optional_bitfield = """\
        let mut optionals = BitVec::new();
"""

    def field(self, tree):
        name = tree.children[0]
        self.fields_to += f"""\
        {encode_expression_fn(tree.children[1])("self." + name, "data")}?;
"""

    def optional_field(self, tree):
        name = tree.children[0]
        self.optional_bitfield += f"""\
        optionals.push(self.{name}.is_some());
"""
        self.fields_to += f"""\
        if let Some(x) = &self.{name} {{
            {encode_expression_fn(tree.children[1])("x", "data")}?;
        }}
"""

    def extension_container(self, tree):
        self.optional_bitfield += f"""\
        optionals.push(false);
"""

        # class IeFieldsTo(Interpreter):
        #     def field(self, tree):
        #         pass


MUT_OPTIONALS = """let mut optionals = BitVec::new();"""


class RustInterpreter(Interpreter):

    def __init__(self):
        # self.output = ""
        self.outfile = ""
        self.in_enum = False

    def extended_items(self, tree):
        pass

    def enum_def(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        field_interpreter = EnumFields()
        field_interpreter.visit(tree.children[1])

        self.outfile += f"""
// {orig_name}
# [derive(Clone, Debug, Copy, TryFromPrimitive)]
# [repr(u8)]
pub enum {name} {{
{field_interpreter.enum_fields}\
}}

impl AperCodec for {name} {{
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some({field_interpreter.variants - 1}), {bool_to_rust(field_interpreter.extensible)})?;
        if extended {{
            return Err(aper::AperCodecError::new("Extended enum not implemented"));
        }}
        Self::try_from(idx as u8).map_err(|_| AperCodecError::new("Unknown enum variant"))
    }}
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {{
        aper::encode::encode_enumerated(data, Some(0), Some({field_interpreter.variants - 1}), {bool_to_rust(field_interpreter.extensible)}, (*self as i128, false))
    }}
}}

"""
        return name

    def choice_def(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        field_interpreter = ChoiceFields()
        field_interpreter.visit(tree.children[1])

        fields_from_interpreter = ChoiceFieldsFrom()
        fields_from_interpreter.visit(tree.children[1])
        fields_to_interpreter = ChoiceFieldsTo(
            fields_from_interpreter.field_index - 1, field_interpreter.extensible)
        fields_to_interpreter.visit(tree.children[1])

        self.outfile += f"""
// {orig_name}
# [derive(Clone, Debug)]
pub enum {name} {{
{field_interpreter.choice_fields}\
}}

impl AperCodec for {name} {{
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        let (idx, extended) = aper::decode::decode_choice_idx(data, 0, {fields_from_interpreter.field_index - 1}, {bool_to_rust(field_interpreter.extensible)})?;
        if extended {{
            return Err(aper::AperCodecError::new("CHOICE additions not implemented"))
        }}
        match idx {{
{fields_from_interpreter.fields_from}\
            _ => Err(AperCodecError::new("Unknown choice idx"))
        }}
    }}
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {{
        match self {{
{fields_to_interpreter.fields_to}\
        }}
    }}
}}

"""

    def tuple_struct(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        inner = type_and_constraints(tree.children[1]).typ
        self.outfile += f"""
// {orig_name}
# [derive(Clone, Debug)]
pub struct {name}(pub {inner});

impl AperCodec for {name} {{
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        Ok(Self({decode_expression(tree.children[1])}))
    }}
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {{
        {encode_expression_fn(tree.children[1])("self.0")}
    }}
}}

"""

    def ie(self, tree):
        pass

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
# [derive(Clone, Debug)]
pub struct {name} {{
{field_interpreter.struct_fields}\
}}

impl AperCodec for {orig_name} {{
    type Output = {orig_name};
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        let _ = aper::decode::decode_length_determinent(data, None, None, false)?;
        let _ = aper::decode::decode_sequence_header(data, true, 0)?;
        let len = aper::decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

{field_interpreter.mut_field_vars}
        for _ in 0..len {{
            let (id, _ext) = aper::decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = aper::decode::decode_length_determinent(data, None, None, false)?;
            match id {{
{field_interpreter.matches}\
                x => return Err(aper::AperCodecError::new(format!("Unrecognised IE type {{}}", x)))
            }}
        }}
{field_interpreter.mandatory}\
        Ok(Self {{
{field_interpreter.self_fields}\
        }})
    }}
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {{
        let mut num_ies = 0;
        let len = 0;
        let ies = &mut AperCodecData::new();
{field_interpreter.fields_to}
        let container = &mut AperCodecData::new();
        aper::encode::encode_sequence_header(container, true, 0, (BitVec::new(),false))?;
        aper::encode::encode_length_determinent(container, Some(0), Some(65535), false, num_ies)?;
        container.append_aligned(ies)?;
        aper::encode::encode_length_determinent(data, Some(0), Some(65535), false, container.length_in_bytes())?;
        data.append_aligned(container)
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

        # for i in [field_interpreter, fields_from_interpreter, find_opt_interpreter, fields_to_interpreter]:
        #     i.visit(tree.children[1])
        # field_interpreter.visit(tree.children[1])
        # fields_from_interpreter.visit(tree.children[1])
        # find_opt_interpreter.visit(tree.children[1])
        # fields_to_interpreter.visit(tree.children[1])
        num_optionals = find_opt_interpreter.num_optionals
        optionals_var = "optionals"
        if num_optionals == 0 or (num_optionals == 1 and find_opt_interpreter.has_extension_container):
            optionals_var = "_optionals"

        self.outfile += f"""
// {orig_name}
# [derive(Clone, Debug)]
pub struct {name} {{
{field_interpreter.struct_fields}\
}}

impl AperCodec for {orig_name} {{
    type Output = {orig_name};
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {{
        let ({optionals_var}, _extensions_present) = aper::decode::decode_sequence_header(data, {bool_to_rust(field_interpreter.extensible)}, {num_optionals})?;
{fields_from_interpreter.fields_from}
        Ok(Self {{
{fields_from_interpreter.self_fields}\
        }})
    }}
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {{
{fields_to_interpreter.optional_bitfield}
        aper::encode::encode_sequence_header(data, {bool_to_rust(field_interpreter.extensible)}, {num_optionals}, (optionals, false))?;
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

    # def object_def(self, tree):
    #     print("Warning - object_def not implemented")

    def extension_container(self, tree):
        pass

    def extended_item(self, tree):
        assert(False)


def generate(tree, constants=dict(), verbose=False):
    tree = transform(tree, constants)
    if verbose:
        print(tree.pretty())
    visited = RustInterpreter()
    print("---- Generating ----")
    visited.visit(tree)
    return visited.outfile


def generate_structs(input_file, constants=dict(), verbose=False):
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
# [derive(Clone, Debug, Copy, TryFromPrimitive)]
# [repr(u8)]
pub enum TriggeringMessage {
    InitiatingMessage,
    SuccessfulOutcome,
    UnsuccessfulOutcome,
}

impl AperCodec for TriggeringMessage {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(aper::AperCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| AperCodecError::new("Unknown enum variant"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_enumerated(data, Some(0), Some(2), false, (*self as i128, false))
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
# [derive(Clone, Debug)]
pub struct WlanMeasurementConfiguration {
    pub wlan_meas_config: WlanMeasConfig,
    pub wlan_rtt: Option<WlanRtt>,
}

impl AperCodec for WlanMeasurementConfiguration {
    type Output = WlanMeasurementConfiguration;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (optionals, _extensions_present) = aper::decode::decode_sequence_header(data, true, 2)?;
        let wlan_meas_config = WlanMeasConfig::decode(data)?;
        let wlan_rtt = if optionals[0] {
            Some(WlanRtt::decode(data)?)
        } else {
            None
        };

        Ok(Self {
            wlan_meas_config,
            wlan_rtt,
        })
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        let mut optionals = BitVec::new();
        optionals.push(self.wlan_rtt.is_some());
        optionals.push(false);

        aper::encode::encode_sequence_header(data, true, 2, (optionals, false))?;
        self.wlan_meas_config.encode(data)?;
        if let Some(x) = &self.wlan_rtt {
            x.encode(data)?;
        }

        Ok(())
    }
}


// WlanRtt
# [derive(Clone, Debug, Copy, TryFromPrimitive)]
# [repr(u8)]
pub enum WlanRtt {
    Thing1,
}

impl AperCodec for WlanRtt {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some(0), true)?;
        if extended {
            return Err(aper::AperCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| AperCodecError::new("Unknown enum variant"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_enumerated(data, Some(0), Some(0), true, (*self as i128, false))
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
# [derive(Clone, Debug)]
pub struct LteueRlfReportContainer(pub Vec<u8>);

impl AperCodec for LteueRlfReportContainer {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        Ok(Self(aper::decode::decode_octetstring(data, None, None, false)?))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_octetstring(data, None, None, false, &self.0)
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
# [derive(Clone, Debug)]
pub struct MaximumDataBurstVolume(pub i128);

impl AperCodec for MaximumDataBurstVolume {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        Ok(Self(aper::decode::decode_integer(data, Some(0), Some(4095), true)?.0))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_integer(data, Some(0), Some(4095), true, (self.0, false))
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
# [derive(Clone, Debug)]
pub struct MobilityInformation(pub BitString);

impl AperCodec for MobilityInformation {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        Ok(Self(aper::decode::decode_bitstring(data, Some(16), Some(16), false)?))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_bitstring(data, Some(16), Some(16), false, &self.0)
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
# [derive(Clone, Debug, Copy, TryFromPrimitive)]
# [repr(u8)]
pub enum MaximumIntegrityProtectedDataRate {
    Bitrate64kbs,
    MaximumUeRate,
}

impl AperCodec for MaximumIntegrityProtectedDataRate {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some(1), true)?;
        if extended {
            return Err(aper::AperCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| AperCodecError::new("Unknown enum variant"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_enumerated(data, Some(0), Some(1), true, (*self as i128, false))
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
# [derive(Clone, Debug)]
pub enum EventTrigger {
    OutOfCoverage(OutOfCoverage),
    EventL1LoggedMdtConfig,
    ShortMacroEnbId(BitString),
}

impl AperCodec for EventTrigger {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (idx, extended) = aper::decode::decode_choice_idx(data, 0, 3, false)?;
        if extended {
            return Err(aper::AperCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::OutOfCoverage(OutOfCoverage::decode(data)?)),
            1 => Ok(Self::EventL1LoggedMdtConfig),
            2 => Ok(Self::ShortMacroEnbId(aper::decode::decode_bitstring(data, Some(18), Some(18), false)?)),
            3 => Err(AperCodecError::new("Choice extension container not implemented")),
            _ => Err(AperCodecError::new("Unknown choice idx"))
        }
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        match self {
            Self::OutOfCoverage(x) => {
                aper::encode::encode_choice_idx(data, 0, 3, false, 0)?;
                x.encode(data)
            },
            Self::EventL1LoggedMdtConfig => {
                aper::encode::encode_choice_idx(data, 0, 3, false, 1)?
                Ok(())
            },
            Self::ShortMacroEnbId(x) => {
                aper::encode::encode_choice_idx(data, 0, 3, false, 2)?;
                aper::encode::encode_bitstring(data, Some(18), Some(18), false, &x)
            }
        }
    }
}

// OutOfCoverage
# [derive(Clone, Debug, Copy, TryFromPrimitive)]
# [repr(u8)]
pub enum OutOfCoverage {
    True,
}

impl AperCodec for OutOfCoverage {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (idx, extended) = aper::decode::decode_enumerated(data, Some(0), Some(0), true)?;
        if extended {
            return Err(aper::AperCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| AperCodecError::new("Unknown enum variant"))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_enumerated(data, Some(0), Some(0), true, (*self as i128, false))
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
# [derive(Clone, Debug)]
pub struct PduSessionResourceSetupRequest {
    pub amf_ue_ngap_id: AmfUeNgapId,
    pub ran_paging_priority: Option<Vec<u8>>,
}

impl AperCodec for PduSessionResourceSetupRequest {
    type Output = PduSessionResourceSetupRequest;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let _ = aper::decode::decode_length_determinent(data, None, None, false)?;
        let _ = aper::decode::decode_sequence_header(data, true, 0)?;
        let len = aper::decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

        let mut amf_ue_ngap_id: Option<AmfUeNgapId> = None;
        let mut ran_paging_priority: Option<Vec<u8>> = None;

        for _ in 0..len {
            let (id, _ext) = aper::decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = aper::decode::decode_length_determinent(data, None, None, false)?;
            match id {
                10 => amf_ue_ngap_id = Some(AmfUeNgapId::decode(data)?),
                83 => ran_paging_priority = Some(aper::decode::decode_octetstring(data, None, None, false)?),
                x => return Err(aper::AperCodecError::new(format!("Unrecognised IE type {}", x)))
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
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        let mut num_ies = 0;
        let len = 0;
        let ies = &mut AperCodecData::new();

        let ie = &mut AperCodecData::new();
        self.amf_ue_ngap_id.encode(&mut ie)?;
        aper::encode::encode_integer(ies, Some(0), Some(65535), false, (10, false))?;
        Criticality::Reject.encode(ies)?;
        aper::encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
        ies.append_aligned(ie)?;
        num_ies += 1;

        if let Some(x) = &self.ran_paging_priority {
            let ie = &mut AperCodecData::new();
            aper::encode::encode_octetstring(ie, None, None, false, &x)?;
            aper::encode::encode_integer(ies, Some(0), Some(65535), false, (83, false))?;
            Criticality::Ignore.encode(ies)?;
            aper::encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
            ies.append_aligned(ie)?;
            num_ies += 1;
        }

        let container = &mut AperCodecData::new();
        aper::encode::encode_sequence_header(container, true, 0, (BitVec::new(),false))?;
        aper::encode::encode_length_determinent(container, Some(0), Some(65535), false, num_ies)?;
        container.append_aligned(ies)?;
        aper::encode::encode_length_determinent(data, Some(0), Some(65535), false, container.length_in_bytes())?;
        data.append_aligned(container)
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
# [derive(Clone, Debug)]
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
            0 => Ok(Self::GnbId(aper::decode::decode_bitstring(data, Some(22), Some(32), false)?)),
            1 => Err(AperCodecError::new("Choice extension container not implemented")),
            _ => Err(AperCodecError::new("Unknown choice idx"))
        }
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        match self {
            Self::GnbId(x) => {
                aper::encode::encode_choice_idx(data, 0, 1, false, (0, false))?;
                aper::encode::encode_bitstring(data, Some(22), Some(32), false, &x)
            }
        }
    }
}

""")

    def test_choice(self):
        self.should_generate("""\
PrivateIE-ID	::= CHOICE {
	local				INTEGER (0..65535),
	global				OBJECT IDENTIFIER
}
""", """\

// PrivateIeId
# [derive(Clone, Debug)]
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
            0 => Ok(Self::Local(aper::decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16)),
            1 => Ok(Self::Global(aper::decode::decode_octetstring(data, None, None, false)?)),
            _ => Err(AperCodecError::new("Unknown choice idx"))
        }
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        match self {
            Self::Local(x) => {
                aper::encode::encode_choice_idx(data, 0, 1, false, (0, false))?;
                aper::encode::encode_integer(data, Some(0), Some(65535), false, (x, false))
            }
            Self::Global(x) => {
                aper::encode::encode_choice_idx(data, 0, 1, false, (1, false))?;
                aper::encode::encode_octetstring(data, None, None, false, &x)
            }
        }
    }
}

""")

    def test_int_options(self):
        self.should_generate("""\
ExpectedActivityPeriod ::= INTEGER (1..30|40|50, ..., -1..70)
""", """\

// ExpectedActivityPeriod
# [derive(Clone, Debug)]
pub struct ExpectedActivityPeriod(pub i128);

impl AperCodec for ExpectedActivityPeriod {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        Ok(Self(aper::decode::decode_integer(data, Some(1), Some(50), true)?.0))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_integer(data, Some(1), Some(50), true, (self.0, false))
    }
}

""")

    def test_simple_visible_string(self):
        self.should_generate("""\
URI-address ::= VisibleString
""", """\

// UriAddress
# [derive(Clone, Debug)]
pub struct UriAddress(pub String);

impl AperCodec for UriAddress {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        Ok(Self(aper::decode::decode_visible_string(data, None, None, false)?))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_visible_string(data, None, None, false, &self.0)
    }
}

""")

    def test_seq_of_non_primitive(self):
        self.should_generate("""\
AdditionalDLUPTNLInformationForHOList ::= SEQUENCE (SIZE (1..50)) OF AdditionalDLUPTNLInformationForHOItem
""", """
// AdditionalDluptnlInformationForHoList
# [derive(Clone, Debug)]
pub struct AdditionalDluptnlInformationForHoList(pub Vec<AdditionalDluptnlInformationForHoItem>);

impl AperCodec for AdditionalDluptnlInformationForHoList {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        Ok(Self({
            let length = aper::decode::decode_length_determinent(data, Some(1), Some(50), false)?;
            let mut items = vec![];
            for _ in 0..length {
                items.push(AdditionalDluptnlInformationForHoItem::decode(data)?);
            }
            items
        }))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_length_determinent(data, Some(1), Some(50), false, self.0.len())?;
        for x in self.0 {
            x.encode(data)?;
        }
        Ok(())
    }
}

""")

    def test_sequence_of(self):
        self.should_generate("""\
DLPRSResourceCoordinates ::= SEQUENCE {
	listofDL-PRSResourceSetARP		SEQUENCE (SIZE (1.. maxnoofPRS-ResourceSets)) OF DLPRSResourceSetARP,
	iE-Extensions					ProtocolExtensionContainer { { DLPRSResourceCoordinates-ExtIEs } } OPTIONAL
}
""", """\

// DlprsResourceCoordinates
# [derive(Clone, Debug)]
pub struct DlprsResourceCoordinates {
    pub listof_dl_prs_resource_set_arp: Vec<DlprsResourceSetArp>,
}

impl AperCodec for DlprsResourceCoordinates {
    type Output = DlprsResourceCoordinates;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let (_optionals, _extensions_present) = aper::decode::decode_sequence_header(data, false, 1)?;
        let listof_dl_prs_resource_set_arp = {
            let length = aper::decode::decode_length_determinent(data, Some(1), Some(2), false)?;
            let mut items = vec![];
            for _ in 0..length {
                items.push(DlprsResourceSetArp::decode(data)?);
            }
            items
        };

        Ok(Self {
            listof_dl_prs_resource_set_arp,
        })
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        let mut optionals = BitVec::new();
        optionals.push(false);

        aper::encode::encode_sequence_header(data, false, 1, (optionals, false))?;
        aper::encode::encode_length_determinent(data, Some(1), Some(2), false, self.0.len())?;
        for x in self.listof_dl_prs_resource_set_arp {
            x.encode(data)?;
        }
        Ok(())
    }
}

""", constants={"maxnoofPRS-ResourceSets": 2})

    def test_seq_of_ie(self):
        self.should_generate("""\
UE-associatedLogicalF1-ConnectionListRes ::= SEQUENCE (SIZE (1.. maxnoofIndividualF1ConnectionsToReset)) OF ProtocolIE-SingleContainer { { UE-associatedLogicalF1-ConnectionItemRes } }

UE-associatedLogicalF1-ConnectionItemRes F1AP-PROTOCOL-IES ::= {
	{ ID id-UE-associatedLogicalF1-ConnectionItem	CRITICALITY reject	TYPE UE-associatedLogicalF1-ConnectionItem	PRESENCE mandatory } ,
	...
}
""", """\

// UeAssociatedLogicalF1ConnectionListRes
# [derive(Clone, Debug)]
pub struct UeAssociatedLogicalF1ConnectionListRes(pub Vec<UeAssociatedLogicalF1ConnectionItem>);

impl AperCodec for UeAssociatedLogicalF1ConnectionListRes {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        Ok(Self({
            let length = aper::decode::decode_length_determinent(data, Some(1), Some(63356), false)?;
            let mut items = vec![];
            for _ in 0..length {
                let _ = aper::decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = aper::decode::decode_length_determinent(data, None, None, false)?;
                items.push(UeAssociatedLogicalF1ConnectionItem::decode(data)?);
            }
            items
        }))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_length_determinent(data, Some(1), Some(63356), false, self.0.len())?;
        for x in self.0 {
            let ie = &mut AperCodecData::new();
            x.encode(ie)?;
            aper::encode::encode_integer(data, Some(0), Some(65535), false, (80, false))?;
            Criticality::Reject.encode(data)?;
            aper::encode::encode_length_determinent(data, None, None, false, ie.length_in_bytes())?;
            data.append_aligned(ie)?;
        }
        Ok(())
    }
}

""", constants={"maxnoofIndividualF1ConnectionsToReset": 63356, "id-UE-associatedLogicalF1-ConnectionItem": 80})

    def test_nested_containers(self):
        self.should_generate("""\
BAPMappingConfiguration ::= SEQUENCE {
	protocolIEs			ProtocolIE-Container { {BAPMappingConfiguration-IEs } },
	...
 }

BAPMappingConfiguration-IEs F1AP-PROTOCOL-IES ::= {
	{ ID id-BH-Routing-Information-Added-List		CRITICALITY ignore	TYPE	BH-Routing-Information-Added-List	PRESENCE optional } |
	...
}

BH-Routing-Information-Added-List ::= SEQUENCE (SIZE (1.. maxnoofRoutingEntries))	OF ProtocolIE-SingleContainer { { BH-Routing-Information-Added-List-ItemIEs } }

BH-Routing-Information-Added-List-ItemIEs	F1AP-PROTOCOL-IES ::= {
	{ ID id-BH-Routing-Information-Added-List-Item				CRITICALITY ignore	TYPE BH-Routing-Information-Added-List-Item						PRESENCE optional } ,
	...
}
""", """\

// BapMappingConfiguration
# [derive(Clone, Debug)]
pub struct BapMappingConfiguration {
    pub bh_routing_information_added_list: Option<BhRoutingInformationAddedList>,
}

impl AperCodec for BapMappingConfiguration {
    type Output = BapMappingConfiguration;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        let _ = aper::decode::decode_length_determinent(data, None, None, false)?;
        let _ = aper::decode::decode_sequence_header(data, true, 0)?;
        let len = aper::decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

        let mut bh_routing_information_added_list: Option<BhRoutingInformationAddedList> = None;

        for _ in 0..len {
            let (id, _ext) = aper::decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = aper::decode::decode_length_determinent(data, None, None, false)?;
            match id {
                283 => bh_routing_information_added_list = Some(BhRoutingInformationAddedList::decode(data)?),
                x => return Err(aper::AperCodecError::new(format!("Unrecognised IE type {}", x)))
            }
        }
        Ok(Self {
            bh_routing_information_added_list,
        })
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        let mut num_ies = 0;
        let len = 0;
        let ies = &mut AperCodecData::new();

        if let Some(x) = &self.bh_routing_information_added_list {
            let ie = &mut AperCodecData::new();
            x.encode(ie)?;
            aper::encode::encode_integer(ies, Some(0), Some(65535), false, (283, false))?;
            Criticality::Ignore.encode(ies)?;
            aper::encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
            ies.append_aligned(ie)?;
            num_ies += 1;
        }

        let container = &mut AperCodecData::new();
        aper::encode::encode_sequence_header(container, true, 0, (BitVec::new(),false))?;
        aper::encode::encode_length_determinent(container, Some(0), Some(65535), false, num_ies)?;
        container.append_aligned(ies)?;
        aper::encode::encode_length_determinent(data, Some(0), Some(65535), false, container.length_in_bytes())?;
        data.append_aligned(container)
    }
}


// BhRoutingInformationAddedList
# [derive(Clone, Debug)]
pub struct BhRoutingInformationAddedList(pub Vec<BhRoutingInformationAddedListItem>);

impl AperCodec for BhRoutingInformationAddedList {
    type Output = Self;
    fn decode(data: &mut AperCodecData) -> Result<Self::Output, AperCodecError> {
        Ok(Self({
            let length = aper::decode::decode_length_determinent(data, Some(1), Some(1024), false)?;
            let mut items = vec![];
            for _ in 0..length {
                let _ = aper::decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = aper::decode::decode_length_determinent(data, None, None, false)?;
                items.push(BhRoutingInformationAddedListItem::decode(data)?);
            }
            items
        }))
    }
    fn encode(&self, data: &mut AperCodecData) -> Result<(), AperCodecError> {
        aper::encode::encode_length_determinent(data, Some(1), Some(1024), false, self.0.len())?;
        for x in self.0 {
            let ie = &mut AperCodecData::new();
            x.encode(ie)?;
            aper::encode::encode_integer(data, Some(0), Some(65535), false, (284, false))?;
            Criticality::Ignore.encode(data)?;
            aper::encode::encode_length_determinent(data, None, None, false, ie.length_in_bytes())?;
            data.append_aligned(ie)?;
        }
        Ok(())
    }
}

""", constants={"maxnoofRoutingEntries": 1024, "id-BH-Routing-Information-Added-List": 283, "id-BH-Routing-Information-Added-List-Item": 284})


if __name__ == '__main__':
    if len(sys.argv) == 2:
        print(generate_structs(sys.argv[1], verbose=True))
    else:
        unittest.main(failfast=True)
