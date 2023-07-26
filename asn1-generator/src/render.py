#!/usr/bin/env python3

import sys
import unittest
from lark.visitors import Interpreter
from case import snake_case
from lark import Tree
from parse import parse_string, parse_file
from transform import transform


def bool_to_rust(b):
    return "true" if b else "false"


class TypeInfo:
    def __init__(self):
        self.constraints = "None, None, false"
        self.seqof = None
        self.typ = None
        self.criticality = None
        self.code = None
        self.inner_type_info = None
        self.is_ie = None
        self.rust_type = ""


# TODO - replace with a visitor?
def type_and_constraints(node):
    type_info = TypeInfo()
    type_info.typ = node

    if isinstance(type_info.typ, Tree):
        if type_info.typ.data in ["ie", "optional_ie"]:
            # The ProtocolIE-SingleContainer case where an IE is inside a sequence of or choice.
            parent = type_info.typ
            type_info = type_and_constraints(parent.children[3])
            type_info.is_ie = True
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
            if bounds[0] > 0:
                type_info.typ = f"NonEmpty<{type_info.inner_type_info.typ}>"
            else:
                type_info.typ = f"Vec<{type_info.inner_type_info.typ}>"

        fixed_size = False

        if len(bounds) > 1:
            fixed_size = (bounds[0] == bounds[1])
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

    type_info.rust_type = type_info.typ

    if type_info.typ in ["VisibleString", "PrintableString", "UTF8String"]:
        type_info.rust_type = "String"

    if type_info.typ == "OctetString":  # broaden to any vec
        if fixed_size:
            type_info.rust_type = f"[u8; {bounds[0]}]"
        else:
            type_info.rust_type = "Vec<u8>"

    return type_info


def decode_expression(tree):
    type_info = type_and_constraints(tree)
    if type_info.seqof:
        if type_info.rust_type.startswith("NonEmpty"):
            items = "NonEmpty::from_vec(items).unwrap()"
        else:
            items = "items"

        if type_info.seqof == "ie_container_sequence_of":
            ie_extra_lines_before = """
                let _ = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = decode::decode_length_determinent(data, None, None, false)?;"""
            ie_extra_lines_after = """
                data.decode_align()?;"""
        else:
            ie_extra_lines_before = ""
            ie_extra_lines_after = ""
        return f"""{{
            let length = decode::decode_length_determinent(data, {type_info.constraints})?;
            let mut items = vec![];
            for _ in 0..length {{\
{ie_extra_lines_before}
                items.push({(decode_expression(tree.children[2]))});\
{ie_extra_lines_after}
            }}
            {items}
        }}"""
    elif type_info.typ == "OctetString":
        if type_info.rust_type[0:3] == "Vec":
            return f"decode::decode_octetstring(data, {type_info.constraints})?"
        else:
            assert (type_info.rust_type[0:3] == "[u8")
            return f"decode::decode_octetstring(data, {type_info.constraints})?.try_into().unwrap()"
    elif type_info.typ == "BitString":
        return f"decode::decode_bitstring(data, {type_info.constraints})?"
    elif type_info.rust_type == "String":
        return f"decode::decode_{snake_case(type_info.typ)}(data, {type_info.constraints})?"
    elif type_info.typ == "i128":
        return f"decode::decode_integer(data, {type_info.constraints})?.0"
    elif is_non_i128_int_type(type_info.typ):
        return f"decode::decode_integer(data, {type_info.constraints})?.0 as {type_info.rust_type}"
    elif type_info.typ == "bool":
        return f"decode::decode_bool(data)?"
    else:
        return f"""{type_info.rust_type.replace("<", "::<")}::decode(data)?"""


# Returns a lambda (x, data="data", copy_type_deref=""), where x is the field name, data is the
# XperCodec data and copy_type_deref is a "*" if we are dealing with a reference.
# Applying the lambda produces a chunk of code to encode the given tree.
def encode_expression_fn(tree):
    type_info = type_and_constraints(tree)
    if type_info.seqof == "ie_container_sequence_of":
        return lambda x, data="data", copy_type_deref="": f"""\
encode::encode_length_determinent({data}, {type_info.constraints}, {x}.len())?;
        for x in &{x} {{
            let ie = &mut Allocator::new_codec_data();
            x.encode(ie)?;
            encode::encode_integer({data}, Some(0), Some(65535), false, {type_info.inner_type_info.code}, false)?;
            Criticality::{type_info.inner_type_info.criticality.title()}.encode({data})?;
            encode::encode_length_determinent({data}, None, None, false, ie.length_in_bytes())?;
            {data}.append_aligned(ie);
        }}
        Ok(())"""
    elif type_info.seqof:
        return lambda x, data="data", copy_type_deref="": f"""\
encode::encode_length_determinent({data}, {type_info.constraints}, {x}.len())?;
        for x in {copy_type_deref}&{x} {{
            {encode_expression_fn(tree.children[2])("x", data, "*")}?;
        }}
        Ok(())"""
    elif type_info.is_ie:
        return lambda x, data="data", copy_type_deref="": f"""\
encode::encode_integer({data}, Some(0), Some(65535), false, {type_info.code}, false)?;
                Criticality::{type_info.criticality.title()}.encode({data})?;
                let ie = &mut Allocator::new_codec_data();
                {encode_expression_fn(tree.children[2])("x", "ie")}?;
                encode::encode_length_determinent({data}, None, None, false, ie.length_in_bytes())?;
                Ok({data}.append_aligned(ie))"""
    if type_info.typ == "OctetString":
        if type_info.rust_type[0:3] == "Vec":
            format_string = f"encode::encode_octetstring({{data}}, {type_info.constraints}, &{{value}}, false)"
        else:
            assert (type_info.rust_type[0:3] == "[u8")
            format_string = f"encode::encode_octetstring({{data}}, {type_info.constraints}, &({{copy_type_deref}}{{value}}).into(), false)"
    elif type_info.rust_type == "BitString":
        format_string = f"encode::encode_bitstring({{data}}, {type_info.constraints}, &{{value}}, false)"
    elif type_info.rust_type == "String":
        format_string = f"encode::encode_{snake_case(type_info.typ)}({{data}}, {type_info.constraints}, &{{value}}, false)"
    elif type_info.rust_type == "i128":
        format_string = f"encode::encode_integer({{data}}, {type_info.constraints}, {{copy_type_deref}}{{value}}, false)"
    elif is_non_i128_int_type(type_info.rust_type):
        format_string = f"encode::encode_integer({{data}}, {type_info.constraints}, {{copy_type_deref}}{{value}} as i128, false)"
    elif type_info.rust_type == "bool":
        format_string = f"encode::encode_bool({{data}}, {{copy_type_deref}}{{value}})"
    else:
        format_string = f"""{{value}}.encode({{data}})"""

    return lambda x, data="data", copy_type_deref="":\
        format_string.format(value=x, data=data,
                             copy_type_deref=copy_type_deref)


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

    def extension_ies(self, tree):
        fields_from = IeFieldsFrom()
        fields_from.visit(tree.children[0])

        self.fields_from += f"""\

        // Process the extension container
{fields_from.mut_field_vars}
        if optionals[{self.optional_idx}] {{"""
        self.fields_from += decode_ies_string(fields_from)
        self.fields_from += "}"
        self.optional_idx += 1
        self.self_fields += fields_from.self_fields

    def empty_sequence_field(self, tree):
        self.optional_idx += 1


class StructFindOptionals(Interpreter):
    def __init__(self):
        self.find_optionals = ""
        self.num_optionals = 0
        self.has_empty_sequence = False

    def optional_field(self, tree):
        name = tree.children[0]
        self.find_optionals += f"""
        optionals.set({self.num_optionals}, self.{name}.is_some());
"""
        self.num_optionals += 1

    def extension_ies(self, tree):
        self.find_optionals += f"""
        optionals.set({self.num_optionals}, false);
"""
        self.num_optionals += 1

    def empty_sequence_field(self, tree):
        self.has_empty_sequence = True
        self.num_optionals += 1


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
        typ = type_and_constraints(tree.children[1])
        self.choice_fields += f"""\
    {name}{"("+typ.rust_type+")" if typ.rust_type != "null" else ""},
"""

    def choice_field_ie_container(self, tree):
        name = tree.children[0]
        typ = type_and_constraints(tree.children[1])
        self.choice_fields += f"""
            {name}{"("+typ.rust_type+")" if typ.rust_type != "null" else ""},
        """

    def choice_field_extension_ies(self, tree):
        for ie in tree.children:
            if ie.data != "extension_marker":
                type_info = type_and_constraints(ie.children[3])
                typ = type_info.typ
                self.choice_fields += f"""\
    {typ}{"("+typ+")" if typ != "null" else ""},
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
        self.choice_field_common(tree)

    def choice_field_ie_container(self, tree):
        self.choice_field_common(tree)

    def choice_field_extension_ies(self, tree):
        for ie in tree.children:
            if ie.data != "extension_marker":
                type_info = type_and_constraints(ie.children[3])
                typ = type_info.typ
                self.fields_to += f"""\
            Self::{typ}{"(x)" if typ != "null" else ""} => {{
                encode::encode_choice_idx(data, 0, {self.num_choices}, {bool_to_rust(self.extensible)}, {self.field_index}, false)?;
                {encode_expression_fn(ie)(
                    "x", copy_type_deref="*") if typ != "null" else "Ok(())"}
            }}
"""
        self.field_index += 1

    def choice_field_common(self, tree):
        name = tree.children[0]
        type_info = type_and_constraints(tree.children[1])

        self.fields_to += f"""\
            Self::{name}{"(x)" if type_info.typ != "null" else ""} => {{
                encode::encode_choice_idx(data, 0, {self.num_choices}, {bool_to_rust(self.extensible)}, {self.field_index}, false)?;
                {encode_expression_fn(tree.children[1])(
                    "x", copy_type_deref="*") if type_info.typ != "null" else "Ok(())"}
            }}
"""
        assert (not type_info.is_ie)
        self.field_index += 1

    def empty_sequence_field(self, tree):
        self.field_index += 1


class ChoiceFieldsFrom(Interpreter):
    def __init__(self):
        self.fields_from = ""
        self.field_index = 0

    def choice_field(self, tree):
        self.choice_field_common(tree)

    def choice_field_ie_container(self, tree):
        self.choice_field_common(tree)

    def choice_field_extension_ies(self, tree):
        self.fields_from += f"""\
            {self.field_index} => {{ 
                let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = decode::decode_length_determinent(data, None, None, false)?;
                let result = match id {{
"""
        for ie in tree.children:
            if ie.data != "extension_marker":
                type_info = type_and_constraints(ie)
                self.fields_from += f"""\
                    {type_info.code} => Ok(Self::{type_info.rust_type}({type_info.rust_type}::decode(data)?)),
"""
        self.fields_from += f"""\
                    x => Err(PerCodecError::new(format!("Unrecognised IE type {{}}", x))),
                }};
                data.decode_align()?;
                result
            }},
"""
        self.field_index += 1

    def choice_field_common(self, tree):
        name = tree.children[0]
        type_info = type_and_constraints(tree.children[1])

        if type_info.typ != "null":
            self.fields_from += f"""\
            {self.field_index} => Ok(Self::{name}({decode_expression(tree.children[1])})),
"""
        else:
            self.fields_from += f"""\
            {self.field_index} => Ok(Self::{name}),
"""
        self.field_index += 1

    def empty_sequence_field(self, tree):
        self.fields_from += f"""\
            {self.field_index} => Err(PerCodecError::new("Choice extension container not implemented")),
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
        typ = type_and_constraints(tree.children[1])
        self.struct_fields += f"""\
    pub {name}: {typ.rust_type},
"""

    def optional_field(self, tree):
        name = tree.children[0]
        typ = type_and_constraints(tree.children[1])
        self.struct_fields += f"""\
    pub {name}: Option<{typ.rust_type}>,
"""

    def extension_ies(self, tree):
        field_interpreter = IeFields()
        field_interpreter.visit(tree.children[0])
        self.struct_fields += field_interpreter.struct_fields


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
        typ = type_and_constraints(tree.children[3])
        self.struct_fields += f"""\
    pub {name}: {typ.rust_type},
"""
        self.mut_field_vars += f"""\
        let mut {name}: Option<{typ.rust_type}> = None;
"""
        self.self_fields += f"            {name},\n"
        self.matches += f"""\
                {id} => {name} = Some({decode_expression(tree.children[3])}),
"""
        self.num_mandatory_fields += 1
        self.mandatory += f"""\
        let {name} = {name}.ok_or(PerCodecError::new(format!(
            "Missing mandatory IE {name}"
        )))?;
"""
        self.fields_to += f"""
        let ie = &mut Allocator::new_codec_data();
        {encode_expression_fn(tree.children[3])("self."+ name, "ie")}?;
        encode::encode_integer(ies, Some(0), Some(65535), false, {id}, false)?;
        Criticality::{criticality.title()}.encode(ies)?;
        encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
        ies.append_aligned(ie);
        num_ies += 1;
"""

    def optional_ie(self, tree):
        name = tree.children[0]
        id = tree.children[1]
        criticality = tree.children[2].capitalize()
        typ = tree.children[3]
        typ = type_and_constraints(tree.children[3])
        self.struct_fields += f"""\
    pub {name}: Option<{typ.rust_type}>,
"""
        self.mut_field_vars += f"""\
        let mut {name}: Option<{typ.rust_type}> = None;
"""
        self.self_fields += f"            {name},\n"
        self.matches += f"""\
                {id} => {name} = Some({decode_expression(tree.children[3])}),
"""
        self.optionals_presence_list += f"self.{name}.is_some(),"
        self.fields_to += f"""
        if let Some(x) = &self.{name} {{
            let ie = &mut Allocator::new_codec_data();
            {encode_expression_fn(tree.children[3])("x", "ie", copy_type_deref="*")}?;
            encode::encode_integer(ies, Some(0), Some(65535), false, {id}, false)?;
            Criticality::{criticality.title()}.encode(ies)?;
            encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
            ies.append_aligned(ie);
            num_ies += 1;
        }}
"""


class IeFieldsFrom(Interpreter):
    def __init__(self):
        self.extensible = False
        self.mut_field_vars = ""
        self.self_fields = ""
        self.matches = ""
        self.mandatory = ""
        self.optionals_presence_list = ""
        self.num_mandatory_fields = 0

    def extension_marker(self, tree):
        self.extensible = True

    def common(self, tree):
        name = tree.children[0]
        id = tree.children[1]
        typ = type_and_constraints(tree.children[3])
        self.mut_field_vars += f"""\
        let mut {name}: Option<{typ.rust_type}> = None;
"""
        self.self_fields += f"            {name},\n"
        self.matches += f"""\
                {id} => {name} = Some({decode_expression(tree.children[3])}),
"""
        return name

    def ie(self, tree):
        name = self.common(tree)
        self.num_mandatory_fields += 1
        self.mandatory += f"""\
        let {name} = {name}.ok_or(PerCodecError::new(format!(
            "Missing mandatory IE {name}"
        )))?;
"""

    def optional_ie(self, tree):
        name = self.common(tree)
        self.optionals_presence_list += f"self.{name}.is_some(),"


def is_non_i128_int_type(t):
    return t in ["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"]


def is_copy_type(t):
    return is_non_i128_int_type(t) or t in ["i128", "bool"]


class StructFieldsTo(Interpreter):
    def __init__(self):
        self.fields_to = ""
        self.extension_ies_fields_to = ""
        self.found_optionals = False
        self.optional_bitfield = """\
        let optionals = BitString::new();
"""

    def add_optional_to_bitfield(self, expression):
        if not self.found_optionals:
            self.optional_bitfield = """\
        let mut optionals = BitString::new();
"""
            self.found_optionals = True

        self.optional_bitfield += f"""\
        optionals.push({expression});
"""

    def field(self, tree):
        name = tree.children[0]
        self.fields_to += f"""\
        {encode_expression_fn(tree.children[1])("self." + name, "data")}?;
"""

    def optional_field(self, tree):
        name = tree.children[0]
        self.add_optional_to_bitfield(f"self.{name}.is_some()")
        self.fields_to += f"""\
        if let Some(x) = &self.{name} {{
            {encode_expression_fn(tree.children[1])("x", "data", "*")}?;
        }}
"""

    def extension_ies(self, tree):
        field_interpreter = IeFields()
        field_interpreter.visit(tree.children[0])
        if field_interpreter.fields_to == "":
            # ASN.1 extension container contains no fields - never add
            self.add_optional_to_bitfield("false")
            return

        # Add extension container if any of the fields are present.
        self.add_optional_to_bitfield("num_ies != 0")
        self.extension_ies_fields_to += """
        let mut num_ies = 0;
        let ies = &mut Allocator::new_codec_data();"""
        self.extension_ies_fields_to += field_interpreter.fields_to
        self.fields_to += """\
        if num_ies != 0 {
            encode::encode_length_determinent(data, Some(1), Some(65535), false, num_ies)?;
            data.append_aligned(ies);
        }"""

    def empty_sequence_field(self, tree):
        self.add_optional_to_bitfield("false")


XPER_CODEC_IMPL_FORMAT = """\
impl PerCodec for {name} {{
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {{
        {name}::decode_inner(data).map_err(|mut e: PerCodecError| {{e.push_context("{name}"); e}})
    }}
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {{
        self.encode_inner(data).map_err(|mut e: PerCodecError| {{e.push_context("{name}"); e}})
    }}
}}"""


class Procedure:
    def __init__(self, tree):
        self.name = next(tree.find_data("procedure_name")
                         ).children[0] + "Procedure"
        assert (self.name != "Procedure")
        self.family = next(tree.find_data("family")).children[0]
        self.initiating = next(tree.find_data("initiating")).children[0]
        self.code = next(tree.find_data("procedure_code")).children[0]
        successful = next(tree.find_data("successful"), None)
        self.successful = successful and successful.children[0]
        unsuccessful = next(tree.find_data("unsuccessful"), None)
        self.unsuccessful = unsuccessful and unsuccessful.children[0]
        self.criticality = next(tree.find_data("criticality")).children[0]


class TopLevelEnums:
    def __init__(self):
        self.initiating_encode_matches = ""
        self.initiating_decode_matches = ""
        self.initiating_enum = """\
# [derive(Clone, Debug)]
pub enum InitiatingMessage {
"""
        self.successful_encode_matches = ""
        self.successful_decode_matches = ""
        self.successful_enum = """\
# [derive(Clone, Debug)]
pub enum SuccessfulOutcome {
"""
        self.unsuccessful_encode_matches = ""
        self.unsuccessful_decode_matches = ""
        self.unsuccessful_enum = """\
# [derive(Clone, Debug)]
pub enum UnsuccessfulOutcome {
"""

    def add_procedure(self, p):
        self.initiating_enum += f"    {p.initiating}({p.initiating}),\n"
        self.initiating_decode_matches += f"""\
            {p.code} => Ok(Self::{p.initiating}({p.initiating}::decode(data)?)),
"""
        self.initiating_encode_matches += f"""\
            Self::{p.initiating}(x) => {{
                encode::encode_integer(data, Some(0), Some(255), false, {p.code}, false)?;
                Criticality::{p.criticality.title()}.encode(data)?;
                let container = &mut Allocator::new_codec_data();
                x.encode(container)?;
                encode::encode_length_determinent(data, None, None, false, container.length_in_bytes())?;
                data.append_aligned(container);
            }}
"""
        if p.successful:
            self.successful_enum += f"    {p.successful}({p.successful}),\n"
            self.successful_decode_matches += f"""\
            {p.code} => Ok(Self::{p.successful}({p.successful}::decode(data)?)),
"""
            self.successful_encode_matches += f"""\
            Self::{p.successful}(x) => {{
                encode::encode_integer(data, Some(0), Some(255), false, {p.code}, false)?;
                Criticality::{p.criticality.title()}.encode(data)?;
                let container = &mut Allocator::new_codec_data();
                x.encode(container)?;
                encode::encode_length_determinent(data, None, None, false, container.length_in_bytes())?;
                data.append_aligned(container);
            }}
"""
        if p.unsuccessful:
            self.unsuccessful_enum += f"    {p.unsuccessful}({p.unsuccessful}),\n"
            self.unsuccessful_decode_matches += f"""\
            {p.code} => Ok(Self::{p.unsuccessful}({p.unsuccessful}::decode(data)?)),
"""
            self.unsuccessful_encode_matches += f"""\
            Self::{p.unsuccessful}(x) => {{
                encode::encode_integer(data, Some(0), Some(255), false, {p.code}, false)?;
                Criticality::{p.criticality.title()}.encode(data)?;
                let container = &mut Allocator::new_codec_data();
                x.encode(container)?;
                encode::encode_length_determinent(data, None, None, false, container.length_in_bytes())?;
                data.append_aligned(container);
            }}
"""

    def generate(self):
        impl = """
impl {name} {{
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {{
        let (id, _ext) = decode::decode_integer(data, Some(0), Some(255), false)?;
        let _ = Criticality::decode(data)?;
        let _ = decode::decode_length_determinent(data, None, None, false)?;
        match id {{
{decode_matches}\
            x => return Err(PerCodecError::new(format!("Unrecognised procedure code {{}}", x)))
        }}
    }}
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {{
        match self {{
{encode_matches}\
        }}
        Ok(())
    }}
}}

""" + XPER_CODEC_IMPL_FORMAT
        return f"""
{self.initiating_enum}}}
{impl.format(name="InitiatingMessage",decode_matches=self.initiating_decode_matches,
             encode_matches=self.initiating_encode_matches)}

{self.successful_enum}}}
{impl.format(name="SuccessfulOutcome", decode_matches=self.successful_decode_matches,
             encode_matches=self.successful_encode_matches)}

{self.unsuccessful_enum}}}
{impl.format(name="UnsuccessfulOutcome",decode_matches=self.unsuccessful_decode_matches,
             encode_matches=self.unsuccessful_encode_matches)}
"""


def added_variant(name):
    if name == "PrivateMessage":
        # Since we don't generate structs for empty containers we use
        # this hack to avoid referencing a non-existent type.
        return f"    //{name}({name}),\n"
    else:
        return f"    {name}({name}),\n"


class RustInterpreter(Interpreter):

    def __init__(self):
        self.outfile = ""
        self.top_level_enums = None

    def extended_items(self, tree):
        pass

    def procedure_def(self, tree):
        p = Procedure(tree)
        if p.initiating == "PrivateMessage":
            return

        # Extend the top level enums (InitiatingMessage, SuccessfulOutcome and UnsuccessfulOutcome)
        # We output them at the end.
        self.top_level_enums = self.top_level_enums or TopLevelEnums()
        self.top_level_enums.add_procedure(p)

        top_pdu = p.family[0] + p.family[1:4].lower() + "Pdu"

        # Output a new struct that impls the Procedure trait.
        # The decode function depends on whether there is a successful and unsuccessful response defined.
        if p.successful:
            self.output_procedure(p)
        else:
            self.output_indication(p)

    def output_procedure(self, p):
        top_pdu = p.family[0] + p.family[1:4].lower() + "Pdu"
        unsuccessful_match_arm = f"""\
            {top_pdu}::UnsuccessfulOutcome(UnsuccessfulOutcome::{p.unsuccessful}(x)) => {{
                Err(RequestError::UnsuccessfulOutcome(x))
            }}""" if p.unsuccessful else ""

        self.outfile += f"""
pub struct {p.name} {{}}

# [async_trait]
impl Procedure for {p.name} {{
    type TopPdu = {top_pdu};
    type Request = {p.initiating};
    type Success = {p.successful};
    type Failure = {p.unsuccessful or "()"};
    const CODE: u8 = {p.code};

    async fn call_provider<T: RequestProvider<Self>>(
        provider: &T,
        req: {p.initiating},
        logger: &Logger,
    ) -> Option<ResponseAction<{top_pdu}>> {{
        match <T as RequestProvider<{p.name}>>::request(provider, req, logger).await {{
            Ok((r, f)) => Some(({top_pdu}::SuccessfulOutcome(SuccessfulOutcome::{p.successful}(r)),f)),
            Err(_) => todo!(),
        }}
    }}

    fn encode_request(r: Self::Request) -> Result<Vec<u8>, PerCodecError> {{
        {top_pdu}::InitiatingMessage(InitiatingMessage::{p.initiating}(r)).into_bytes()
    }}

    fn decode_response(bytes: &[u8]) -> Result<Self::Success, RequestError<Self::Failure>> {{
        let response_pdu = Self::TopPdu::from_bytes(bytes)?;
        match response_pdu {{
            {top_pdu}::SuccessfulOutcome(SuccessfulOutcome::{p.successful}(x)) => Ok(x),
{unsuccessful_match_arm}
            _ => Err(RequestError::Other("Unexpected pdu contents".to_string())),
        }}
    }}
}}
"""

    def output_indication(self, p):
        top_pdu = p.family[0] + p.family[1:4].lower() + "Pdu"

        self.outfile += f"""
pub struct {p.name} {{}}

# [async_trait]
impl Indication for {p.name} {{
    type TopPdu = {top_pdu};
    type Request = {p.initiating};
    const CODE: u8 = {p.code};

    async fn call_provider<T: IndicationHandler<Self>>(
        provider: &T,
        req: {p.initiating},
        logger: &Logger,
    ) {{
        <T as IndicationHandler<{p.name}>>::handle(provider, req, logger).await;
    }}

    fn encode_request(r: Self::Request) -> Result<Vec<u8>, PerCodecError> {{
        {top_pdu}::InitiatingMessage(InitiatingMessage::{p.initiating}(r)).into_bytes()
    }}
}}
"""

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

impl {name} {{
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {{
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some({field_interpreter.variants - 1}), {bool_to_rust(field_interpreter.extensible)})?;
        if extended {{
            return Err(PerCodecError::new("Extended enum not implemented"));
        }}
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }}
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {{
        encode::encode_enumerated(data, Some(0), Some({field_interpreter.variants - 1}), {bool_to_rust(field_interpreter.extensible)}, *self as i128, false)
    }}
}}

""" + XPER_CODEC_IMPL_FORMAT.format(name=name)
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

impl {name} {{
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {{
        let (idx, extended) = decode::decode_choice_idx(data, 0, {fields_from_interpreter.field_index - 1}, {bool_to_rust(field_interpreter.extensible)})?;
        if extended {{
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }}
        match idx {{
{fields_from_interpreter.fields_from}\
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }}
    }}
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {{
        match self {{
{fields_to_interpreter.fields_to}\
        }}
    }}
}}

""" + XPER_CODEC_IMPL_FORMAT.format(name=name)

    def primitive_def(self, tree):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name
        inner = type_and_constraints(tree.children[1])
        assert (inner.rust_type != "OctetString")
        clone_copy = "Copy, " if is_copy_type(inner.rust_type) else ""
        self.outfile += f"""
// {orig_name}
# [derive(Clone, {clone_copy}Debug)]
pub struct {name}(pub {inner.rust_type});

impl {name} {{
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {{
        Ok(Self({decode_expression(tree.children[1])}))
    }}
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {{
        {encode_expression_fn(tree.children[1])("self.0")}
    }}
}}

""" + XPER_CODEC_IMPL_FORMAT.format(name=name)

    def ie(self, tree):
        pass

    def protocol_ie_container(self, tree):
        self.protocol_ie_container2(tree, is_sequence=False)

    def pdu(self, tree):
        self.protocol_ie_container2(tree, is_sequence=True)

    def protocol_ie_container2(self, tree, is_sequence):
        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name

        fields = IeFields()
        fields_from = IeFieldsFrom()
        for i in [fields, fields_from]:
            i.visit(tree.children[1])

        #   ProtocolIE-Container {NGAP-PROTOCOL-IES : IEsSetParam} ::=
        # 	SEQUENCE (SIZE (0..maxProtocolIEs)) OF
        # 	ProtocolIE-Field {{IEsSetParam}}

        # ProtocolIE-Field {NGAP-PROTOCOL-IES : IEsSetParam} ::= SEQUENCE {
        # 	id				NGAP-PROTOCOL-IES.&id				({IEsSetParam}),
        # 	criticality		NGAP-PROTOCOL-IES.&criticality		({IEsSetParam}{@id}),
        # 	value			NGAP-PROTOCOL-IES.&Value			({IEsSetParam}{@id})
        # }

        mut = "" if fields.struct_fields == "" else "mut "
        decode_align = "" if fields_from.matches == "" else """
            data.decode_align()?;"""

        self.outfile += f"""
// {orig_name}
# [derive(Clone, Debug)]
pub struct {name} {{
{fields.struct_fields}\
}}

impl {orig_name} {{
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {{"""

        if is_sequence:
            self.outfile += f"""
        let _ = decode::decode_sequence_header(data, true, 0)?;"""

        self.outfile += f"""
        let num_ies = decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

{fields_from.mut_field_vars}
        for _ in 0..num_ies {{
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = decode::decode_length_determinent(data, None, None, false)?;
            match id {{
{fields_from.matches}\
                x => return Err(PerCodecError::new(format!("Unrecognised IE type {{}}", x)))
            }}\
{decode_align}
        }}
{fields_from.mandatory}\
        Ok(Self {{
{fields_from.self_fields}\
        }})
    }}
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {{
        let {mut}num_ies = 0;
        let ies = &mut Allocator::new_codec_data();
{fields.fields_to}"""

        if is_sequence:
            self.outfile += f"""
        encode::encode_sequence_header(data, true, &BitString::new(), false)?;"""

        self.outfile += f"""
        encode::encode_length_determinent(data, Some(0), Some(65535), false, num_ies)?;
        data.append_aligned(ies);
        Ok(())
    }}
}}

""" + XPER_CODEC_IMPL_FORMAT.format(name=name)

    def sequence_def(self, tree):
        if tree.children[1].data == "ie_container_sequence":
            self.pdu(tree)
            return

        orig_name = tree.children[0]
        print(orig_name)
        name = orig_name

        # fields = [
        #     child for child in tree.children[1].children if child.data in ["field", "optional_field", "extension_container"]]

        field_interpreter = StructFields()
        fields_from_interpreter = StructFieldsFrom()
        find_opt_interpreter = StructFindOptionals()
        fields_to_interpreter = StructFieldsTo()
        for i in [field_interpreter, fields_from_interpreter, find_opt_interpreter, fields_to_interpreter]:
            i.visit(tree.children[1])

        num_optionals = find_opt_interpreter.num_optionals
        optionals_var = "optionals"
        if num_optionals == 0 or (num_optionals == 1 and find_opt_interpreter.has_empty_sequence):
            optionals_var = "_optionals"

        self.outfile += f"""
// {orig_name}
# [derive(Clone, Debug)]
pub struct {name} {{
{field_interpreter.struct_fields}\
}}

impl {orig_name} {{
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {{
        let ({optionals_var}, _extensions_present) = decode::decode_sequence_header(data, {bool_to_rust(field_interpreter.extensible)}, {num_optionals})?;
{fields_from_interpreter.fields_from}
        Ok(Self {{
{fields_from_interpreter.self_fields}\
        }})
    }}
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {{\
{fields_to_interpreter.extension_ies_fields_to}
{fields_to_interpreter.optional_bitfield}
        encode::encode_sequence_header(data, {bool_to_rust(field_interpreter.extensible)}, &optionals, false)?;
{fields_to_interpreter.fields_to}
        Ok(())
    }}
}}

""" + XPER_CODEC_IMPL_FORMAT.format(name=name)

        return name

    def comment(self, tree, comment=""):
        if comment != "":
            comment = " - " + comment
        self.outfile += "// " + tree.children[0] + comment + "\n"

    # def object_def(self, tree):
    #     print("Warning - object_def not implemented")

    def extension_container(self, tree):
        assert (False)
        pass

    def extended_item(self, tree):
        assert (False)

    def generate_top_level_enums(self):
        if self.top_level_enums:
            self.outfile += self.top_level_enums.generate()


def decode_ies_string(fields_from):
    return f"""
        let num_ies = decode::decode_length_determinent(data, Some(1), Some(65535), false)?;
        for _ in 0..num_ies {{
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _criticality = Criticality::decode(data)?;
            let ie_length = decode::decode_length_determinent(data, None, None, false)?;
            match id {{
{fields_from.matches}\
                _ => data.advance_maybe_err(ie_length, false)?,
            }}
            data.decode_align()?;
        }}"""


def generate(tree, constants=dict(), verbose=False, strip_xxap=False):
    tree = transform(tree, constants, strip_xxap)
    if verbose:
        print(tree.pretty())
    visited = RustInterpreter()
    print("---- Generating ----")
    visited.visit(tree)
    visited.generate_top_level_enums()
    return visited.outfile


def generate_from_file(input_file, constants=dict(), verbose=False):
    tree = parse_file(input_file)
    if verbose:
        print(tree.pretty())
    return generate(tree, constants, print, False)


class TestGenerator(unittest.TestCase):
    maxDiff = None

    def should_generate(self, input, expected, constants=dict(), strip_xxap=False):
        output = ""
        tree = parse_string(input)
        try:
            output = generate(tree, constants, True, strip_xxap)
            print(output)
            self.assertEqual(output, expected)
        finally:
            if output != expected:
                print("!!! Unexpected output !!!: dumping original parsed tree")
                print(tree.pretty())

    def test_procedure(self):
        self.should_generate("""\
aMFConfigurationUpdate NGAP-ELEMENTARY-PROCEDURE ::= {
	INITIATING MESSAGE		AMFConfigurationUpdate
	SUCCESSFUL OUTCOME		AMFConfigurationUpdateAcknowledge
	UNSUCCESSFUL OUTCOME	AMFConfigurationUpdateFailure
	PROCEDURE CODE			id-AMFConfigurationUpdate
	CRITICALITY				reject
}

handoverNotification NGAP-ELEMENTARY-PROCEDURE ::= {
	INITIATING MESSAGE		HandoverNotify
	PROCEDURE CODE			id-HandoverNotification
	CRITICALITY				ignore
}
""", """\

pub struct AmfConfigurationUpdateProcedure {}

# [async_trait]
impl Procedure for AmfConfigurationUpdateProcedure {
    type TopPdu = NgapPdu;
    type Request = AmfConfigurationUpdate;
    type Success = AmfConfigurationUpdateAcknowledge;
    type Failure = AmfConfigurationUpdateFailure;
    const CODE: u8 = 0;

    async fn call_provider<T: RequestProvider<Self>>(
        provider: &T,
        req: AmfConfigurationUpdate,
        logger: &Logger,
    ) -> Option<ResponseAction<NgapPdu>> {
        match <T as RequestProvider<AmfConfigurationUpdateProcedure>>::request(provider, req, logger).await {
            Ok((r, f)) => Some((NgapPdu::SuccessfulOutcome(SuccessfulOutcome::AmfConfigurationUpdateAcknowledge(r)),f)),
            Err(_) => todo!(),
        }
    }

    fn encode_request(r: Self::Request) -> Result<Vec<u8>, PerCodecError> {
        NgapPdu::InitiatingMessage(InitiatingMessage::AmfConfigurationUpdate(r)).into_bytes()
    }

    fn decode_response(bytes: &[u8]) -> Result<Self::Success, RequestError<Self::Failure>> {
        let response_pdu = Self::TopPdu::from_bytes(bytes)?;
        match response_pdu {
            NgapPdu::SuccessfulOutcome(SuccessfulOutcome::AmfConfigurationUpdateAcknowledge(x)) => Ok(x),
            NgapPdu::UnsuccessfulOutcome(UnsuccessfulOutcome::AmfConfigurationUpdateFailure(x)) => {
                Err(RequestError::UnsuccessfulOutcome(x))
            }
            _ => Err(RequestError::Other("Unexpected pdu contents".to_string())),
        }
    }
}

pub struct HandoverNotificationProcedure {}

# [async_trait]
impl Indication for HandoverNotificationProcedure {
    type TopPdu = NgapPdu;
    type Request = HandoverNotify;
    const CODE: u8 = 11;

    async fn call_provider<T: IndicationHandler<Self>>(
        provider: &T,
        req: HandoverNotify,
        logger: &Logger,
    ) {
        <T as IndicationHandler<HandoverNotificationProcedure>>::handle(provider, req, logger).await;
    }

    fn encode_request(r: Self::Request) -> Result<Vec<u8>, PerCodecError> {
        NgapPdu::InitiatingMessage(InitiatingMessage::HandoverNotify(r)).into_bytes()
    }
}

# [derive(Clone, Debug)]
pub enum InitiatingMessage {
    AmfConfigurationUpdate(AmfConfigurationUpdate),
    HandoverNotify(HandoverNotify),
}

impl InitiatingMessage {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (id, _ext) = decode::decode_integer(data, Some(0), Some(255), false)?;
        let _ = Criticality::decode(data)?;
        let _ = decode::decode_length_determinent(data, None, None, false)?;
        match id {
            0 => Ok(Self::AmfConfigurationUpdate(AmfConfigurationUpdate::decode(data)?)),
            11 => Ok(Self::HandoverNotify(HandoverNotify::decode(data)?)),
            x => return Err(PerCodecError::new(format!("Unrecognised procedure code {}", x)))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::AmfConfigurationUpdate(x) => {
                encode::encode_integer(data, Some(0), Some(255), false, 0, false)?;
                Criticality::Reject.encode(data)?;
                let container = &mut Allocator::new_codec_data();
                x.encode(container)?;
                encode::encode_length_determinent(data, None, None, false, container.length_in_bytes())?;
                data.append_aligned(container);
            }
            Self::HandoverNotify(x) => {
                encode::encode_integer(data, Some(0), Some(255), false, 11, false)?;
                Criticality::Ignore.encode(data)?;
                let container = &mut Allocator::new_codec_data();
                x.encode(container)?;
                encode::encode_length_determinent(data, None, None, false, container.length_in_bytes())?;
                data.append_aligned(container);
            }
        }
        Ok(())
    }
}

impl PerCodec for InitiatingMessage {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        InitiatingMessage::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("InitiatingMessage"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("InitiatingMessage"); e})
    }
}

# [derive(Clone, Debug)]
pub enum SuccessfulOutcome {
    AmfConfigurationUpdateAcknowledge(AmfConfigurationUpdateAcknowledge),
}

impl SuccessfulOutcome {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (id, _ext) = decode::decode_integer(data, Some(0), Some(255), false)?;
        let _ = Criticality::decode(data)?;
        let _ = decode::decode_length_determinent(data, None, None, false)?;
        match id {
            0 => Ok(Self::AmfConfigurationUpdateAcknowledge(AmfConfigurationUpdateAcknowledge::decode(data)?)),
            x => return Err(PerCodecError::new(format!("Unrecognised procedure code {}", x)))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::AmfConfigurationUpdateAcknowledge(x) => {
                encode::encode_integer(data, Some(0), Some(255), false, 0, false)?;
                Criticality::Reject.encode(data)?;
                let container = &mut Allocator::new_codec_data();
                x.encode(container)?;
                encode::encode_length_determinent(data, None, None, false, container.length_in_bytes())?;
                data.append_aligned(container);
            }
        }
        Ok(())
    }
}

impl PerCodec for SuccessfulOutcome {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        SuccessfulOutcome::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SuccessfulOutcome"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SuccessfulOutcome"); e})
    }
}

# [derive(Clone, Debug)]
pub enum UnsuccessfulOutcome {
    AmfConfigurationUpdateFailure(AmfConfigurationUpdateFailure),
}

impl UnsuccessfulOutcome {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (id, _ext) = decode::decode_integer(data, Some(0), Some(255), false)?;
        let _ = Criticality::decode(data)?;
        let _ = decode::decode_length_determinent(data, None, None, false)?;
        match id {
            0 => Ok(Self::AmfConfigurationUpdateFailure(AmfConfigurationUpdateFailure::decode(data)?)),
            x => return Err(PerCodecError::new(format!("Unrecognised procedure code {}", x)))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::AmfConfigurationUpdateFailure(x) => {
                encode::encode_integer(data, Some(0), Some(255), false, 0, false)?;
                Criticality::Reject.encode(data)?;
                let container = &mut Allocator::new_codec_data();
                x.encode(container)?;
                encode::encode_length_determinent(data, None, None, false, container.length_in_bytes())?;
                data.append_aligned(container);
            }
        }
        Ok(())
    }
}

impl PerCodec for UnsuccessfulOutcome {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        UnsuccessfulOutcome::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("UnsuccessfulOutcome"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("UnsuccessfulOutcome"); e})
    }
}
""", constants={"id-AMFConfigurationUpdate": 0, "id-HandoverNotification": 11})

    def test_simple_integer(self):
        self.should_generate("""\
ProcedureCode		::= INTEGER (0..255)
""", """\

// ProcedureCode
# [derive(Clone, Copy, Debug)]
pub struct ProcedureCode(pub u8);

impl ProcedureCode {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(decode::decode_integer(data, Some(0), Some(255), false)?.0 as u8))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_integer(data, Some(0), Some(255), false, self.0 as i128, false)
    }
}

impl PerCodec for ProcedureCode {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        ProcedureCode::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("ProcedureCode"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("ProcedureCode"); e})
    }
}""")

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

impl TriggeringMessage {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some(2), false)?;
        if extended {
            return Err(PerCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_enumerated(data, Some(0), Some(2), false, *self as i128, false)
    }
}

impl PerCodec for TriggeringMessage {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        TriggeringMessage::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("TriggeringMessage"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("TriggeringMessage"); e})
    }
}"""
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

WLANMeasurementConfiguration-ExtIEs NGAP-PROTOCOL-EXTENSION ::= {
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

impl WlanMeasurementConfiguration {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (optionals, _extensions_present) = decode::decode_sequence_header(data, true, 2)?;
        let wlan_meas_config = WlanMeasConfig::decode(data)?;
        let wlan_rtt = if optionals[0] {
            Some(WlanRtt::decode(data)?)
        } else {
            None
        };

        // Process the extension container

        if optionals[1] {
        let num_ies = decode::decode_length_determinent(data, Some(1), Some(65535), false)?;
        for _ in 0..num_ies {
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _criticality = Criticality::decode(data)?;
            let ie_length = decode::decode_length_determinent(data, None, None, false)?;
            match id {
                _ => data.advance_maybe_err(ie_length, false)?,
            }
            data.decode_align()?;
        }}
        Ok(Self {
            wlan_meas_config,
            wlan_rtt,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut optionals = BitString::new();
        optionals.push(self.wlan_rtt.is_some());
        optionals.push(false);

        encode::encode_sequence_header(data, true, &optionals, false)?;
        self.wlan_meas_config.encode(data)?;
        if let Some(x) = &self.wlan_rtt {
            x.encode(data)?;
        }

        Ok(())
    }
}

impl PerCodec for WlanMeasurementConfiguration {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        WlanMeasurementConfiguration::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("WlanMeasurementConfiguration"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("WlanMeasurementConfiguration"); e})
    }
}
// WlanRtt
# [derive(Clone, Debug, Copy, TryFromPrimitive)]
# [repr(u8)]
pub enum WlanRtt {
    Thing1,
}

impl WlanRtt {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some(0), true)?;
        if extended {
            return Err(PerCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_enumerated(data, Some(0), Some(0), true, *self as i128, false)
    }
}

impl PerCodec for WlanRtt {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        WlanRtt::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("WlanRtt"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("WlanRtt"); e})
    }
}"""
        self.should_generate(input, output)

    def test_bounded_octet_string(self):
        input = """\
Test ::= OCTET STRING (SIZE(3))
"""
        output = """\

// Test
# [derive(Clone, Debug)]
pub struct Test(pub [u8; 3]);

impl Test {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(decode::decode_octetstring(data, Some(3), Some(3), false)?.try_into().unwrap()))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_octetstring(data, Some(3), Some(3), false, &(self.0).into(), false)
    }
}

impl PerCodec for Test {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Test::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("Test"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("Test"); e})
    }
}"""
        self.should_generate(input, output)

    def test_unbounded_octet_string(self):
        input = """\
LTEUERLFReportContainer::= OCTET STRING (CONTAINING Foo)
"""
        output = """\

// LteUeRlfReportContainer
# [derive(Clone, Debug)]
pub struct LteUeRlfReportContainer(pub Vec<u8>);

impl LteUeRlfReportContainer {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(decode::decode_octetstring(data, None, None, false)?))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_octetstring(data, None, None, false, &self.0, false)
    }
}

impl PerCodec for LteUeRlfReportContainer {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        LteUeRlfReportContainer::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("LteUeRlfReportContainer"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("LteUeRlfReportContainer"); e})
    }
}"""
        self.should_generate(input, output)

    def test_bounded_int_newtype(self):
        input = """\
MaximumDataBurstVolume::= INTEGER(0..4095, ..., 4096.. 2000000)
"""
        output = """\

// MaximumDataBurstVolume
# [derive(Clone, Copy, Debug)]
pub struct MaximumDataBurstVolume(pub i128);

impl MaximumDataBurstVolume {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(decode::decode_integer(data, Some(0), Some(4095), true)?.0))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_integer(data, Some(0), Some(4095), true, self.0, false)
    }
}

impl PerCodec for MaximumDataBurstVolume {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        MaximumDataBurstVolume::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("MaximumDataBurstVolume"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("MaximumDataBurstVolume"); e})
    }
}"""
        self.should_generate(input, output)

    def test_newtype(self):
        input = """
MobilityInformation ::= BIT STRING(SIZE(16))
"""
        output = """\

// MobilityInformation
# [derive(Clone, Debug)]
pub struct MobilityInformation(pub BitString);

impl MobilityInformation {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(decode::decode_bitstring(data, Some(16), Some(16), false)?))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_bitstring(data, Some(16), Some(16), false, &self.0, false)
    }
}

impl PerCodec for MobilityInformation {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        MobilityInformation::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("MobilityInformation"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("MobilityInformation"); e})
    }
}"""
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

impl MaximumIntegrityProtectedDataRate {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some(1), true)?;
        if extended {
            return Err(PerCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_enumerated(data, Some(0), Some(1), true, *self as i128, false)
    }
}

impl PerCodec for MaximumIntegrityProtectedDataRate {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        MaximumIntegrityProtectedDataRate::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("MaximumIntegrityProtectedDataRate"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("MaximumIntegrityProtectedDataRate"); e})
    }
}"""
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

impl EventTrigger {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 3, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::OutOfCoverage(OutOfCoverage::decode(data)?)),
            1 => Ok(Self::EventL1LoggedMdtConfig),
            2 => Ok(Self::ShortMacroEnbId(decode::decode_bitstring(data, Some(18), Some(18), false)?)),
            3 => Err(PerCodecError::new("Choice extension container not implemented")),
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::OutOfCoverage(x) => {
                encode::encode_choice_idx(data, 0, 3, false, 0)?;
                x.encode(data)
            },
            Self::EventL1LoggedMdtConfig => {
                encode::encode_choice_idx(data, 0, 3, false, 1)?;
                Ok(())
            },
            Self::ShortMacroEnbId(x) => {
                encode::encode_choice_idx(data, 0, 3, false, 2)?;
                encode::encode_bitstring(data, Some(18), Some(18), false, &x, false)
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

impl OutOfCoverage {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_enumerated(data, Some(0), Some(0), true)?;
        if extended {
            return Err(PerCodecError::new("Extended enum not implemented"));
        }
        Self::try_from(idx as u8).map_err(|_| PerCodecError::new("Unknown enum variant"))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_enumerated(data, Some(0), Some(0), true, *self as i128, false)
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
	{ ID id-RANPagingPriority						CRITICALITY ignore	TYPE OCTET STRING (SIZE(1))							PRESENCE optional		}|
	...
}
""", """\

// PduSessionResourceSetupRequest
# [derive(Clone, Debug)]
pub struct PduSessionResourceSetupRequest {
    pub amf_ue_ngap_id: AmfUeNgapId,
    pub ran_paging_priority: Option<[u8; 1]>,
}

impl PduSessionResourceSetupRequest {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let _ = decode::decode_sequence_header(data, true, 0)?;
        let num_ies = decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

        let mut amf_ue_ngap_id: Option<AmfUeNgapId> = None;
        let mut ran_paging_priority: Option<[u8; 1]> = None;

        for _ in 0..num_ies {
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = decode::decode_length_determinent(data, None, None, false)?;
            match id {
                10 => amf_ue_ngap_id = Some(AmfUeNgapId::decode(data)?),
                83 => ran_paging_priority = Some(decode::decode_octetstring(data, Some(1), Some(1), false)?.try_into().unwrap()),
                x => return Err(PerCodecError::new(format!("Unrecognised IE type {}", x)))
            }
            data.decode_align()?;
        }
        let amf_ue_ngap_id = amf_ue_ngap_id.ok_or(PerCodecError::new(format!(
            "Missing mandatory IE amf_ue_ngap_id"
        )))?;
        Ok(Self {
            amf_ue_ngap_id,
            ran_paging_priority,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut num_ies = 0;
        let ies = &mut Allocator::new_codec_data();

        let ie = &mut Allocator::new_codec_data();
        self.amf_ue_ngap_id.encode(ie)?;
        encode::encode_integer(ies, Some(0), Some(65535), false, 10, false)?;
        Criticality::Reject.encode(ies)?;
        encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
        ies.append_aligned(ie);
        num_ies += 1;

        if let Some(x) = &self.ran_paging_priority {
            let ie = &mut Allocator::new_codec_data();
            encode::encode_octetstring(ie, Some(1), Some(1), false, &(*x).into(), false)?;
            encode::encode_integer(ies, Some(0), Some(65535), false, 83, false)?;
            Criticality::Ignore.encode(ies)?;
            encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
            ies.append_aligned(ie);
            num_ies += 1;
        }

        encode::encode_sequence_header(data, true, &BitString::new(), false)?;
        encode::encode_length_determinent(data, Some(0), Some(65535), false, num_ies)?;
        data.append_aligned(ies);
        Ok(())
    }
}

impl PerCodec for PduSessionResourceSetupRequest {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        PduSessionResourceSetupRequest::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("PduSessionResourceSetupRequest"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("PduSessionResourceSetupRequest"); e})
    }
}""", constants={"id-AMF-UE-NGAP-ID": 10, "id-RANPagingPriority": 83})

    def test_bit_string(self):
        self.should_generate("""\
GNB-ID ::= CHOICE {
	gNB-ID		BIT STRING (SIZE (22..32)),
	choice-Extensions		ProtocolIE-SingleContainer { {GNB-ID-ExtIEs } }
}
GNB-ID-ExtIEs NGAP-PROTOCOL-IES ::= {
	...
}

""", """\

// GnbId
# [derive(Clone, Debug)]
pub enum GnbId {
    GnbId(BitString),
}

impl GnbId {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::GnbId(decode::decode_bitstring(data, Some(22), Some(32), false)?)),
            1 => { 
                let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = decode::decode_length_determinent(data, None, None, false)?;
                let result = match id {
                    x => Err(PerCodecError::new(format!("Unrecognised IE type {}", x))),
                };
                data.decode_align()?;
                result
            },
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::GnbId(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 0, false)?;
                encode::encode_bitstring(data, Some(22), Some(32), false, &x, false)
            }
        }
    }
}

impl PerCodec for GnbId {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        GnbId::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("GnbId"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("GnbId"); e})
    }
}""")

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

impl PrivateIeId {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::Local(decode::decode_integer(data, Some(0), Some(65535), false)?.0 as u16)),
            1 => Ok(Self::Global(decode::decode_octetstring(data, None, None, false)?)),
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::Local(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 0, false)?;
                encode::encode_integer(data, Some(0), Some(65535), false, *x as i128, false)
            }
            Self::Global(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 1, false)?;
                encode::encode_octetstring(data, None, None, false, &x, false)
            }
        }
    }
}

impl PerCodec for PrivateIeId {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        PrivateIeId::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("PrivateIeId"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("PrivateIeId"); e})
    }
}""")

    def test_int_options(self):
        self.should_generate("""\
ExpectedActivityPeriod ::= INTEGER (1..30|40|50, ..., -1..70)
""", """\

// ExpectedActivityPeriod
# [derive(Clone, Copy, Debug)]
pub struct ExpectedActivityPeriod(pub i128);

impl ExpectedActivityPeriod {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(decode::decode_integer(data, Some(1), Some(50), true)?.0))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_integer(data, Some(1), Some(50), true, self.0, false)
    }
}

impl PerCodec for ExpectedActivityPeriod {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        ExpectedActivityPeriod::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("ExpectedActivityPeriod"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("ExpectedActivityPeriod"); e})
    }
}""")

    def test_simple_visible_string(self):
        self.should_generate("""\
URI-address ::= VisibleString
""", """\

// UriAddress
# [derive(Clone, Debug)]
pub struct UriAddress(pub String);

impl UriAddress {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self(decode::decode_visible_string(data, None, None, false)?))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_visible_string(data, None, None, false, &self.0, false)
    }
}

impl PerCodec for UriAddress {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        UriAddress::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("UriAddress"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("UriAddress"); e})
    }
}""")

    def test_seq_of_non_primitive(self):
        self.should_generate("""\
AdditionalDLUPTNLInformationForHOList ::= SEQUENCE (SIZE (1..50)) OF AdditionalDLUPTNLInformationForHOItem
""", """
// AdditionalDlUpTnlInformationForHoList
# [derive(Clone, Debug)]
pub struct AdditionalDlUpTnlInformationForHoList(pub NonEmpty<AdditionalDlUpTnlInformationForHoItem>);

impl AdditionalDlUpTnlInformationForHoList {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self({
            let length = decode::decode_length_determinent(data, Some(1), Some(50), false)?;
            let mut items = vec![];
            for _ in 0..length {
                items.push(AdditionalDlUpTnlInformationForHoItem::decode(data)?);
            }
            NonEmpty::from_vec(items).unwrap()
        }))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_length_determinent(data, Some(1), Some(50), false, self.0.len())?;
        for x in &self.0 {
            x.encode(data)?;
        }
        Ok(())
    }
}

impl PerCodec for AdditionalDlUpTnlInformationForHoList {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        AdditionalDlUpTnlInformationForHoList::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("AdditionalDlUpTnlInformationForHoList"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("AdditionalDlUpTnlInformationForHoList"); e})
    }
}""")

    def test_sequence_of(self):
        self.should_generate("""\
DLPRSResourceCoordinates ::= SEQUENCE {
	listofDL-PRSResourceSetARP		SEQUENCE (SIZE (1.. maxnoofPRS-ResourceSets)) OF DLPRSResourceSetARP,
    foo                             INTEGER (-5..5) OPTIONAL,
	iE-Extensions					ProtocolExtensionContainer { { DLPRSResourceCoordinates-ExtIEs } } OPTIONAL
}

DLPRSResourceCoordinates-ExtIEs F1AP-PROTOCOL-EXTENSION ::= {
	...
}

""", """\

// DlprsResourceCoordinates
# [derive(Clone, Debug)]
pub struct DlprsResourceCoordinates {
    pub listof_dl_prs_resource_set_arp: NonEmpty<DlprsResourceSetArp>,
    pub foo: Option<i8>,
}

impl DlprsResourceCoordinates {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (optionals, _extensions_present) = decode::decode_sequence_header(data, false, 2)?;
        let listof_dl_prs_resource_set_arp = {
            let length = decode::decode_length_determinent(data, Some(1), Some(2), false)?;
            let mut items = vec![];
            for _ in 0..length {
                items.push(DlprsResourceSetArp::decode(data)?);
            }
            NonEmpty::from_vec(items).unwrap()
        };
        let foo = if optionals[0] {
            Some(decode::decode_integer(data, Some(-5), Some(5), false)?.0 as i8)
        } else {
            None
        };

        // Process the extension container

        if optionals[1] {
        let num_ies = decode::decode_length_determinent(data, Some(1), Some(65535), false)?;
        for _ in 0..num_ies {
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _criticality = Criticality::decode(data)?;
            let ie_length = decode::decode_length_determinent(data, None, None, false)?;
            match id {
                _ => data.advance_maybe_err(ie_length, false)?,
            }
            data.decode_align()?;
        }}
        Ok(Self {
            listof_dl_prs_resource_set_arp,
            foo,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut optionals = BitString::new();
        optionals.push(self.foo.is_some());
        optionals.push(false);

        encode::encode_sequence_header(data, false, &optionals, false)?;
        encode::encode_length_determinent(data, Some(1), Some(2), false, self.listof_dl_prs_resource_set_arp.len())?;
        for x in &self.listof_dl_prs_resource_set_arp {
            x.encode(data)?;
        }
        Ok(())?;
        if let Some(x) = &self.foo {
            encode::encode_integer(data, Some(-5), Some(5), false, *x as i128, false)?;
        }

        Ok(())
    }
}

impl PerCodec for DlprsResourceCoordinates {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        DlprsResourceCoordinates::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("DlprsResourceCoordinates"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("DlprsResourceCoordinates"); e})
    }
}""", constants={"maxnoofPRS-ResourceSets": 2})

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
pub struct UeAssociatedLogicalF1ConnectionListRes(pub NonEmpty<UeAssociatedLogicalF1ConnectionItem>);

impl UeAssociatedLogicalF1ConnectionListRes {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self({
            let length = decode::decode_length_determinent(data, Some(1), Some(63356), false)?;
            let mut items = vec![];
            for _ in 0..length {
                let _ = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = decode::decode_length_determinent(data, None, None, false)?;
                items.push(UeAssociatedLogicalF1ConnectionItem::decode(data)?);
                data.decode_align()?;
            }
            NonEmpty::from_vec(items).unwrap()
        }))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_length_determinent(data, Some(1), Some(63356), false, self.0.len())?;
        for x in &self.0 {
            let ie = &mut Allocator::new_codec_data();
            x.encode(ie)?;
            encode::encode_integer(data, Some(0), Some(65535), false, 80, false)?;
            Criticality::Reject.encode(data)?;
            encode::encode_length_determinent(data, None, None, false, ie.length_in_bytes())?;
            data.append_aligned(ie);
        }
        Ok(())
    }
}

impl PerCodec for UeAssociatedLogicalF1ConnectionListRes {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        UeAssociatedLogicalF1ConnectionListRes::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("UeAssociatedLogicalF1ConnectionListRes"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("UeAssociatedLogicalF1ConnectionListRes"); e})
    }
}""", constants={"maxnoofIndividualF1ConnectionsToReset": 63356, "id-UE-associatedLogicalF1-ConnectionItem": 80})

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

impl BapMappingConfiguration {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let _ = decode::decode_sequence_header(data, true, 0)?;
        let num_ies = decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

        let mut bh_routing_information_added_list: Option<BhRoutingInformationAddedList> = None;

        for _ in 0..num_ies {
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = decode::decode_length_determinent(data, None, None, false)?;
            match id {
                283 => bh_routing_information_added_list = Some(BhRoutingInformationAddedList::decode(data)?),
                x => return Err(PerCodecError::new(format!("Unrecognised IE type {}", x)))
            }
            data.decode_align()?;
        }
        Ok(Self {
            bh_routing_information_added_list,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut num_ies = 0;
        let ies = &mut Allocator::new_codec_data();

        if let Some(x) = &self.bh_routing_information_added_list {
            let ie = &mut Allocator::new_codec_data();
            x.encode(ie)?;
            encode::encode_integer(ies, Some(0), Some(65535), false, 283, false)?;
            Criticality::Ignore.encode(ies)?;
            encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
            ies.append_aligned(ie);
            num_ies += 1;
        }

        encode::encode_sequence_header(data, true, &BitString::new(), false)?;
        encode::encode_length_determinent(data, Some(0), Some(65535), false, num_ies)?;
        data.append_aligned(ies);
        Ok(())
    }
}

impl PerCodec for BapMappingConfiguration {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        BapMappingConfiguration::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("BapMappingConfiguration"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("BapMappingConfiguration"); e})
    }
}
// BhRoutingInformationAddedList
# [derive(Clone, Debug)]
pub struct BhRoutingInformationAddedList(pub NonEmpty<BhRoutingInformationAddedListItem>);

impl BhRoutingInformationAddedList {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        Ok(Self({
            let length = decode::decode_length_determinent(data, Some(1), Some(1024), false)?;
            let mut items = vec![];
            for _ in 0..length {
                let _ = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = decode::decode_length_determinent(data, None, None, false)?;
                items.push(BhRoutingInformationAddedListItem::decode(data)?);
                data.decode_align()?;
            }
            NonEmpty::from_vec(items).unwrap()
        }))
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        encode::encode_length_determinent(data, Some(1), Some(1024), false, self.0.len())?;
        for x in &self.0 {
            let ie = &mut Allocator::new_codec_data();
            x.encode(ie)?;
            encode::encode_integer(data, Some(0), Some(65535), false, 284, false)?;
            Criticality::Ignore.encode(data)?;
            encode::encode_length_determinent(data, None, None, false, ie.length_in_bytes())?;
            data.append_aligned(ie);
        }
        Ok(())
    }
}

impl PerCodec for BhRoutingInformationAddedList {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        BhRoutingInformationAddedList::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("BhRoutingInformationAddedList"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("BhRoutingInformationAddedList"); e})
    }
}""", constants={"maxnoofRoutingEntries": 1024, "id-BH-Routing-Information-Added-List": 283, "id-BH-Routing-Information-Added-List-Item": 284})

    def test_(self):
        self.should_generate("""\
GNB-CUSystemInformation ::= SEQUENCE {
	sibtypetobeupdatedlist	SEQUENCE (SIZE (1.. maxnoofSIBTypes)) OF SibtypetobeupdatedListItem,
	iE-Extensions					ProtocolExtensionContainer { { GNB-CUSystemInformation-ExtIEs } } OPTIONAL,
	...
}

GNB-CUSystemInformation-ExtIEs F1AP-PROTOCOL-EXTENSION ::= {
	{ID id-systemInformationAreaID  CRITICALITY ignore	EXTENSION SystemInformationAreaID PRESENCE optional } ,
	...
}
""", """\

// GnbCuSystemInformation
# [derive(Clone, Debug)]
pub struct GnbCuSystemInformation {
    pub sib_type_to_be_updated_list: NonEmpty<SibTypeToBeUpdatedListItem>,
    pub system_information_area_id: Option<SystemInformationAreaId>,
}

impl GnbCuSystemInformation {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (optionals, _extensions_present) = decode::decode_sequence_header(data, true, 1)?;
        let sib_type_to_be_updated_list = {
            let length = decode::decode_length_determinent(data, Some(1), Some(32), false)?;
            let mut items = vec![];
            for _ in 0..length {
                items.push(SibTypeToBeUpdatedListItem::decode(data)?);
            }
            NonEmpty::from_vec(items).unwrap()
        };

        // Process the extension container
        let mut system_information_area_id: Option<SystemInformationAreaId> = None;

        if optionals[0] {
        let num_ies = decode::decode_length_determinent(data, Some(1), Some(65535), false)?;
        for _ in 0..num_ies {
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _criticality = Criticality::decode(data)?;
            let ie_length = decode::decode_length_determinent(data, None, None, false)?;
            match id {
                239 => system_information_area_id = Some(SystemInformationAreaId::decode(data)?),
                _ => data.advance_maybe_err(ie_length, false)?,
            }
            data.decode_align()?;
        }}
        Ok(Self {
            sib_type_to_be_updated_list,
            system_information_area_id,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut num_ies = 0;
        let ies = &mut Allocator::new_codec_data();
        if let Some(x) = &self.system_information_area_id {
            let ie = &mut Allocator::new_codec_data();
            x.encode(ie)?;
            encode::encode_integer(ies, Some(0), Some(65535), false, 239, false)?;
            Criticality::Ignore.encode(ies)?;
            encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
            ies.append_aligned(ie);
            num_ies += 1;
        }

        let mut optionals = BitString::new();
        optionals.push(num_ies != 0);

        encode::encode_sequence_header(data, true, &optionals, false)?;
        encode::encode_length_determinent(data, Some(1), Some(32), false, self.sib_type_to_be_updated_list.len())?;
        for x in &self.sib_type_to_be_updated_list {
            x.encode(data)?;
        }
        Ok(())?;
        if num_ies != 0 {
            encode::encode_length_determinent(data, Some(1), Some(65535), false, num_ies)?;
            data.append_aligned(ies);
        }
        Ok(())
    }
}

impl PerCodec for GnbCuSystemInformation {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        GnbCuSystemInformation::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("GnbCuSystemInformation"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("GnbCuSystemInformation"); e})
    }
}""", constants={"maxnoofSIBTypes": 32, "id-systemInformationAreaID": 239})

    def test_inline_choice(self):
        self.should_generate("""\
SBCCH-SL-BCH-MessageType::=     CHOICE {
    c1                              CHOICE {
        masterInformationBlockSidelink              MasterInformationBlockSidelink,
        spare1 NULL
    },
    messageClassExtension   SEQUENCE {}
}""", """\

// SbcchSlBchMessageType
# [derive(Clone, Debug)]
pub enum SbcchSlBchMessageType {
    C1(C1),
}

impl SbcchSlBchMessageType {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::C1(C1::decode(data)?)),
            1 => Err(PerCodecError::new("Choice extension container not implemented")),
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::C1(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 0, false)?;
                x.encode(data)
            }
        }
    }
}

impl PerCodec for SbcchSlBchMessageType {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        SbcchSlBchMessageType::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SbcchSlBchMessageType"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SbcchSlBchMessageType"); e})
    }
}
// C1
# [derive(Clone, Debug)]
pub enum C1 {
    MasterInformationBlockSidelink(MasterInformationBlockSidelink),
    Spare1,
}

impl C1 {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::MasterInformationBlockSidelink(MasterInformationBlockSidelink::decode(data)?)),
            1 => Ok(Self::Spare1),
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::MasterInformationBlockSidelink(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 0, false)?;
                x.encode(data)
            }
            Self::Spare1 => {
                encode::encode_choice_idx(data, 0, 1, false, 1, false)?;
                Ok(())
            }
        }
    }
}

impl PerCodec for C1 {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        C1::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("C1"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("C1"); e})
    }
}""")

    def test_empty_pdu(self):
        self.should_generate("""\
OverloadStop ::= SEQUENCE {
    protocolIEs		ProtocolIE-Container		{ {OverloadStopIEs} },
	...
}

OverloadStopIEs NGAP-PROTOCOL-IES ::= {
	...
}
""", """\

// OverloadStop
# [derive(Clone, Debug)]
pub struct OverloadStop {
}

impl OverloadStop {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let _ = decode::decode_sequence_header(data, true, 0)?;
        let num_ies = decode::decode_length_determinent(data, Some(0), Some(65535), false)?;


        for _ in 0..num_ies {
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = decode::decode_length_determinent(data, None, None, false)?;
            match id {
                x => return Err(PerCodecError::new(format!("Unrecognised IE type {}", x)))
            }
        }
        Ok(Self {
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let num_ies = 0;
        let ies = &mut Allocator::new_codec_data();

        encode::encode_sequence_header(data, true, &BitString::new(), false)?;
        encode::encode_length_determinent(data, Some(0), Some(65535), false, num_ies)?;
        data.append_aligned(ies);
        Ok(())
    }
}

impl PerCodec for OverloadStop {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        OverloadStop::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("OverloadStop"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("OverloadStop"); e})
    }
}""")

    def test_rrc_setup_release(self):
        self.should_generate("""\
LocationMeasurementIndication-IEs ::=       SEQUENCE {
    measurementIndication                       SetupRelease {LocationMeasurementInfo},
    nonCriticalExtension                        SEQUENCE{}                                                              OPTIONAL
}
""", """\

// LocationMeasurementIndicationIEs
# [derive(Clone, Debug)]
pub struct LocationMeasurementIndicationIEs {
    pub measurement_indication: SetupRelease<LocationMeasurementInfo>,
}

impl LocationMeasurementIndicationIEs {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (_optionals, _extensions_present) = decode::decode_sequence_header(data, false, 1)?;
        let measurement_indication = SetupRelease::<LocationMeasurementInfo>::decode(data)?;

        Ok(Self {
            measurement_indication,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut optionals = BitString::new();
        optionals.push(false);

        encode::encode_sequence_header(data, false, &optionals, false)?;
        self.measurement_indication.encode(data)?;

        Ok(())
    }
}

impl PerCodec for LocationMeasurementIndicationIEs {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        LocationMeasurementIndicationIEs::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("LocationMeasurementIndicationIEs"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("LocationMeasurementIndicationIEs"); e})
    }
}""")

    def test_parameterized_choice_def(self):
        self.should_generate("""\
SetupRelease { ElementTypeParam } ::= CHOICE {
    release         NULL,
    setup           ElementTypeParam
}
""", "")

    def test_seq_of_constrained_int(self):
        self.should_generate("""\
AvailabilityCombination-r16 ::=         SEQUENCE {
    resourceAvailability-r16                SEQUENCE (SIZE (1..maxNrofResourceAvailabilityPerCombination-r16)) OF INTEGER (0..7)
}
""", """\

// AvailabilityCombinationR16
# [derive(Clone, Debug)]
pub struct AvailabilityCombinationR16 {
    pub resource_availability_r_16: NonEmpty<u8>,
}

impl AvailabilityCombinationR16 {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (_optionals, _extensions_present) = decode::decode_sequence_header(data, false, 0)?;
        let resource_availability_r_16 = {
            let length = decode::decode_length_determinent(data, Some(1), Some(5), false)?;
            let mut items = vec![];
            for _ in 0..length {
                items.push(decode::decode_integer(data, Some(0), Some(7), false)?.0 as u8);
            }
            NonEmpty::from_vec(items).unwrap()
        };

        Ok(Self {
            resource_availability_r_16,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let optionals = BitString::new();

        encode::encode_sequence_header(data, false, &optionals, false)?;
        encode::encode_length_determinent(data, Some(1), Some(5), false, self.resource_availability_r_16.len())?;
        for x in &self.resource_availability_r_16 {
            encode::encode_integer(data, Some(0), Some(7), false, *x as i128, false)?;
        }
        Ok(())?;

        Ok(())
    }
}

impl PerCodec for AvailabilityCombinationR16 {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        AvailabilityCombinationR16::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("AvailabilityCombinationR16"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("AvailabilityCombinationR16"); e})
    }
}""", constants={"maxNrofResourceAvailabilityPerCombination-r16": 5})

    def test_seq_of_choice(self):
        self.should_generate("""\
SystemInformation-IEs ::=           SEQUENCE {
    sib-TypeAndInfo                     SEQUENCE (SIZE (1..3)) OF CHOICE {
        sib2                                SIB2,
        sib3                                SIB3,
        ...,
        sib10-v1610                         SIB10-r16,
    },
}
""", """\

// SystemInformationIEs
# [derive(Clone, Debug)]
pub struct SystemInformationIEs {
    pub sib_type_and_info: NonEmpty<SibTypeAndInfo>,
}

impl SystemInformationIEs {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (_optionals, _extensions_present) = decode::decode_sequence_header(data, false, 0)?;
        let sib_type_and_info = {
            let length = decode::decode_length_determinent(data, Some(1), Some(3), false)?;
            let mut items = vec![];
            for _ in 0..length {
                items.push(SibTypeAndInfo::decode(data)?);
            }
            NonEmpty::from_vec(items).unwrap()
        };

        Ok(Self {
            sib_type_and_info,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let optionals = BitString::new();

        encode::encode_sequence_header(data, false, &optionals, false)?;
        encode::encode_length_determinent(data, Some(1), Some(3), false, self.sib_type_and_info.len())?;
        for x in &self.sib_type_and_info {
            x.encode(data)?;
        }
        Ok(())?;

        Ok(())
    }
}

impl PerCodec for SystemInformationIEs {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        SystemInformationIEs::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SystemInformationIEs"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SystemInformationIEs"); e})
    }
}
// SibTypeAndInfo
# [derive(Clone, Debug)]
pub enum SibTypeAndInfo {
    Sib2(Sib2),
    Sib3(Sib3),
}

impl SibTypeAndInfo {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 1, true)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::Sib2(Sib2::decode(data)?)),
            1 => Ok(Self::Sib3(Sib3::decode(data)?)),
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::Sib2(x) => {
                encode::encode_choice_idx(data, 0, 1, true, 0, false)?;
                x.encode(data)
            }
            Self::Sib3(x) => {
                encode::encode_choice_idx(data, 0, 1, true, 1, false)?;
                x.encode(data)
            }
        }
    }
}

impl PerCodec for SibTypeAndInfo {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        SibTypeAndInfo::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SibTypeAndInfo"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SibTypeAndInfo"); e})
    }
}""")

    def test_seq_of_copy_type(self):
        self.should_generate("""\
CSI-AssociatedReportConfigInfo ::=  SEQUENCE {
    nzp-CSI-RS                          SEQUENCE {
        qcl-info                            SEQUENCE (SIZE(1..2)) OF TCI-StateId OPTIONAL
    },
}
""", """\

// CsiAssociatedReportConfigInfo
# [derive(Clone, Debug)]
pub struct CsiAssociatedReportConfigInfo {
    pub nzp_csi_rs: NzpCsiRs,
}

impl CsiAssociatedReportConfigInfo {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (_optionals, _extensions_present) = decode::decode_sequence_header(data, false, 0)?;
        let nzp_csi_rs = NzpCsiRs::decode(data)?;

        Ok(Self {
            nzp_csi_rs,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let optionals = BitString::new();

        encode::encode_sequence_header(data, false, &optionals, false)?;
        self.nzp_csi_rs.encode(data)?;

        Ok(())
    }
}

impl PerCodec for CsiAssociatedReportConfigInfo {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        CsiAssociatedReportConfigInfo::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("CsiAssociatedReportConfigInfo"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("CsiAssociatedReportConfigInfo"); e})
    }
}
// NzpCsiRs
# [derive(Clone, Debug)]
pub struct NzpCsiRs {
    pub qcl_info: Option<NonEmpty<TciStateId>>,
}

impl NzpCsiRs {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (optionals, _extensions_present) = decode::decode_sequence_header(data, false, 1)?;
        let qcl_info = if optionals[0] {
            Some({
            let length = decode::decode_length_determinent(data, Some(1), Some(2), false)?;
            let mut items = vec![];
            for _ in 0..length {
                items.push(TciStateId::decode(data)?);
            }
            NonEmpty::from_vec(items).unwrap()
        })
        } else {
            None
        };

        Ok(Self {
            qcl_info,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut optionals = BitString::new();
        optionals.push(self.qcl_info.is_some());

        encode::encode_sequence_header(data, false, &optionals, false)?;
        if let Some(x) = &self.qcl_info {
            encode::encode_length_determinent(data, Some(1), Some(2), false, x.len())?;
        for x in *&x {
            x.encode(data)?;
        }
        Ok(())?;
        }

        Ok(())
    }
}

impl PerCodec for NzpCsiRs {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        NzpCsiRs::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("NzpCsiRs"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("NzpCsiRs"); e})
    }
}""")

    def test_choice_of_protocol_ie_container(self):
        self.should_generate("""\
System-BearerContextSetupRequest ::= CHOICE {
	e-UTRAN-BearerContextSetupRequest		ProtocolIE-Container { {EUTRAN-BearerContextSetupRequest } },
	nG-RAN-BearerContextSetupRequest		ProtocolIE-Container { {NG-RAN-BearerContextSetupRequest } },
	choice-extension						ProtocolIE-SingleContainer { {System-BearerContextSetupRequest-ExtIEs }  }
}

EUTRAN-BearerContextSetupRequest E1AP-PROTOCOL-IES ::= {
	{ ID id-SubscriberProfileIDforRFP		CRITICALITY ignore	 TYPE INTEGER (1..4095, ...)		PRESENCE optional } |
	...
}

NG-RAN-BearerContextSetupRequest E1AP-PROTOCOL-IES ::= {
	{ ID id-PDU-Session-Resource-To-Setup-List		CRITICALITY reject	 TYPE PDU-Session-Resource-To-Setup-List		PRESENCE mandatory } ,
	...
}
System-BearerContextSetupRequest-ExtIEs E1AP-PROTOCOL-IES ::= {
	...
}
""", """
// SystemBearerContextSetupRequest
# [derive(Clone, Debug)]
pub enum SystemBearerContextSetupRequest {
    EutranBearerContextSetupRequest(EutranBearerContextSetupRequest),
    NgRanBearerContextSetupRequest(NgRanBearerContextSetupRequest),
}

impl SystemBearerContextSetupRequest {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 2, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::EutranBearerContextSetupRequest(EutranBearerContextSetupRequest::decode(data)?)),
            1 => Ok(Self::NgRanBearerContextSetupRequest(NgRanBearerContextSetupRequest::decode(data)?)),
            2 => { 
                let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = decode::decode_length_determinent(data, None, None, false)?;
                let result = match id {
                    x => Err(PerCodecError::new(format!("Unrecognised IE type {}", x))),
                };
                data.decode_align()?;
                result
            },
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::EutranBearerContextSetupRequest(x) => {
                encode::encode_choice_idx(data, 0, 2, false, 0, false)?;
                x.encode(data)
            }
            Self::NgRanBearerContextSetupRequest(x) => {
                encode::encode_choice_idx(data, 0, 2, false, 1, false)?;
                x.encode(data)
            }
        }
    }
}

impl PerCodec for SystemBearerContextSetupRequest {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        SystemBearerContextSetupRequest::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SystemBearerContextSetupRequest"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("SystemBearerContextSetupRequest"); e})
    }
}
// EutranBearerContextSetupRequest
# [derive(Clone, Debug)]
pub struct EutranBearerContextSetupRequest {
    pub subscriber_profile_i_dfor_rfp: Option<u16>,
}

impl EutranBearerContextSetupRequest {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let num_ies = decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

        let mut subscriber_profile_i_dfor_rfp: Option<u16> = None;

        for _ in 0..num_ies {
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = decode::decode_length_determinent(data, None, None, false)?;
            match id {
                43 => subscriber_profile_i_dfor_rfp = Some(decode::decode_integer(data, Some(1), Some(4095), true)?.0 as u16),
                x => return Err(PerCodecError::new(format!("Unrecognised IE type {}", x)))
            }
            data.decode_align()?;
        }
        Ok(Self {
            subscriber_profile_i_dfor_rfp,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut num_ies = 0;
        let ies = &mut Allocator::new_codec_data();

        if let Some(x) = &self.subscriber_profile_i_dfor_rfp {
            let ie = &mut Allocator::new_codec_data();
            encode::encode_integer(ie, Some(1), Some(4095), true, *x as i128, false)?;
            encode::encode_integer(ies, Some(0), Some(65535), false, 43, false)?;
            Criticality::Ignore.encode(ies)?;
            encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
            ies.append_aligned(ie);
            num_ies += 1;
        }

        encode::encode_length_determinent(data, Some(0), Some(65535), false, num_ies)?;
        data.append_aligned(ies);
        Ok(())
    }
}

impl PerCodec for EutranBearerContextSetupRequest {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        EutranBearerContextSetupRequest::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("EutranBearerContextSetupRequest"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("EutranBearerContextSetupRequest"); e})
    }
}
// NgRanBearerContextSetupRequest
# [derive(Clone, Debug)]
pub struct NgRanBearerContextSetupRequest {
    pub pdu_session_resource_to_setup_list: PduSessionResourceToSetupList,
}

impl NgRanBearerContextSetupRequest {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let num_ies = decode::decode_length_determinent(data, Some(0), Some(65535), false)?;

        let mut pdu_session_resource_to_setup_list: Option<PduSessionResourceToSetupList> = None;

        for _ in 0..num_ies {
            let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
            let _ = Criticality::decode(data)?;
            let _ = decode::decode_length_determinent(data, None, None, false)?;
            match id {
                321 => pdu_session_resource_to_setup_list = Some(PduSessionResourceToSetupList::decode(data)?),
                x => return Err(PerCodecError::new(format!("Unrecognised IE type {}", x)))
            }
            data.decode_align()?;
        }
        let pdu_session_resource_to_setup_list = pdu_session_resource_to_setup_list.ok_or(PerCodecError::new(format!(
            "Missing mandatory IE pdu_session_resource_to_setup_list"
        )))?;
        Ok(Self {
            pdu_session_resource_to_setup_list,
        })
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        let mut num_ies = 0;
        let ies = &mut Allocator::new_codec_data();

        let ie = &mut Allocator::new_codec_data();
        self.pdu_session_resource_to_setup_list.encode(ie)?;
        encode::encode_integer(ies, Some(0), Some(65535), false, 321, false)?;
        Criticality::Reject.encode(ies)?;
        encode::encode_length_determinent(ies, None, None, false, ie.length_in_bytes())?;
        ies.append_aligned(ie);
        num_ies += 1;

        encode::encode_length_determinent(data, Some(0), Some(65535), false, num_ies)?;
        data.append_aligned(ies);
        Ok(())
    }
}

impl PerCodec for NgRanBearerContextSetupRequest {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        NgRanBearerContextSetupRequest::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("NgRanBearerContextSetupRequest"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("NgRanBearerContextSetupRequest"); e})
    }
}""", constants={"id-DRB-To-Setup-List-EUTRAN": 42, "id-SubscriberProfileIDforRFP": 43, "id-AdditionalRRMPriorityIndex": 123, "id-PDU-Session-Resource-To-Setup-List": 321})

    def test_choice_extension(self):
        self.should_generate("""\
QoSInformation ::= CHOICE {
	eUTRANQoS					EUTRANQoS,
	choice-extension			ProtocolIE-SingleContainer { { QoSInformation-ExtIEs } }
}

QoSInformation-ExtIEs F1AP-PROTOCOL-IES ::= {
	{	ID id-DRB-Information		CRITICALITY ignore TYPE DRB-Information		PRESENCE mandatory } ,
	{	ID id-Another-One		CRITICALITY ignore TYPE AnOtherType		PRESENCE mandatory } ,
	...
}""", """\

// QosInformation
# [derive(Clone, Debug)]
pub enum QosInformation {
    EutranQos(EutranQos),
    DrbInformation(DrbInformation),
    AnOtherType(AnOtherType),
}

impl QosInformation {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::EutranQos(EutranQos::decode(data)?)),
            1 => { 
                let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = decode::decode_length_determinent(data, None, None, false)?;
                let result = match id {
                    164 => Ok(Self::DrbInformation(DrbInformation::decode(data)?)),
                    156 => Ok(Self::AnOtherType(AnOtherType::decode(data)?)),
                    x => Err(PerCodecError::new(format!("Unrecognised IE type {}", x))),
                };
                data.decode_align()?;
                result
            },
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::EutranQos(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 0, false)?;
                x.encode(data)
            }
            Self::DrbInformation(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 1, false)?;
                encode::encode_integer(data, Some(0), Some(65535), false, 164, false)?;
                Criticality::Ignore.encode(data)?;
                let ie = &mut Allocator::new_codec_data();
                x.encode(ie)?;
                encode::encode_length_determinent(data, None, None, false, ie.length_in_bytes())?;
                Ok(data.append_aligned(ie))
            }
            Self::AnOtherType(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 1, false)?;
                encode::encode_integer(data, Some(0), Some(65535), false, 156, false)?;
                Criticality::Ignore.encode(data)?;
                let ie = &mut Allocator::new_codec_data();
                x.encode(ie)?;
                encode::encode_length_determinent(data, None, None, false, ie.length_in_bytes())?;
                Ok(data.append_aligned(ie))
            }
        }
    }
}

impl PerCodec for QosInformation {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        QosInformation::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("QosInformation"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("QosInformation"); e})
    }
}""", constants={"id-DRB-Information": 164, "id-Another-One": 156})

    def test_common_xxap_struct(self):
        self.should_generate("""\
UPTransportLayerInformation ::= CHOICE {
	gTPTunnel				GTPTunnel,
	choice-Extensions		ProtocolIE-SingleContainer { {UPTransportLayerInformation-ExtIEs } }
}

UPTransportLayerInformation-ExtIEs NGAP-PROTOCOL-IES ::= {
	...
}

GTPTunnel ::= SEQUENCE {
	transportLayerAddress		TransportLayerAddress,
	gTP-TEID					GTP-TEID,
	iE-Extensions		ProtocolExtensionContainer { {GTPTunnel-ExtIEs } } OPTIONAL,
	...
}

GTPTunnel-ExtIEs NGAP-PROTOCOL-EXTENSION ::= {
	...
}""", """\

// UpTransportLayerInformation
# [derive(Clone, Debug)]
pub enum UpTransportLayerInformation {
    GtpTunnel(GtpTunnel),
}

impl UpTransportLayerInformation {
    fn decode_inner(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        let (idx, extended) = decode::decode_choice_idx(data, 0, 1, false)?;
        if extended {
            return Err(PerCodecError::new("CHOICE additions not implemented"))
        }
        match idx {
            0 => Ok(Self::GtpTunnel(GtpTunnel::decode(data)?)),
            1 => { 
                let (id, _ext) = decode::decode_integer(data, Some(0), Some(65535), false)?;
                let _ = Criticality::decode(data)?;
                let _ = decode::decode_length_determinent(data, None, None, false)?;
                let result = match id {
                    x => Err(PerCodecError::new(format!("Unrecognised IE type {}", x))),
                };
                data.decode_align()?;
                result
            },
            _ => Err(PerCodecError::new("Unknown choice idx"))
        }
    }
    fn encode_inner(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        match self {
            Self::GtpTunnel(x) => {
                encode::encode_choice_idx(data, 0, 1, false, 0, false)?;
                x.encode(data)
            }
        }
    }
}

impl PerCodec for UpTransportLayerInformation {
    type Allocator = Allocator;
    fn decode(data: &mut PerCodecData) -> Result<Self, PerCodecError> {
        UpTransportLayerInformation::decode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("UpTransportLayerInformation"); e})
    }
    fn encode(&self, data: &mut PerCodecData) -> Result<(), PerCodecError> {
        self.encode_inner(data).map_err(|mut e: PerCodecError| {e.push_context("UpTransportLayerInformation"); e})
    }
}""", constants=dict(), strip_xxap=True)


if __name__ == '__main__':
    if len(sys.argv) == 2:
        print(generate_from_file(sys.argv[1], verbose=True))
    else:
        unittest.main(failfast=True)
