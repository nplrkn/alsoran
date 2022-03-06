#!/usr/bin/env python3

import unittest
from lark.visitors import Transformer
from case import pascal_case, snake_case
from lark.lexer import Token
from lark import Tree, v_args
from parser import parse_string, parse_file


@v_args(tree=True)
class TypeTransformer(Transformer):
    def __init__(self):
        self.extra_defs = []
        self.name_dict = dict()  # TODO get rid of this global

    def unique_type_name(self, name):
        name = pascal_case(name)
        existing = self.name_dict.get(name)
        self.name_dict[name] = (existing or 0) + 1
        return (name + str(existing)) if existing is not None else name

    def document(self, tree):
        tree.children += self.extra_defs
        return tree

    def enum_item(self, tree):
        tree.children[0] = pascal_case(tree.children[0])
        return tree

    def field(self, tree):
        tree = self.transform_type(tree)
        tree.children[0] = snake_case(tree.children[0])
        return tree

    def optional_field(self, tree):
        tree = self.transform_type(tree)
        tree.children[0] = snake_case(tree.children[0])
        return tree

    def tuple_struct(self, tree):
        tree.children[0] = pascal_case(tree.children[0])
        return tree

    def struct(self, tree):
        tree.children[0] = pascal_case(tree.children[0])
        return tree

    def transform_type(self, tree):
        orig_name = tree.children[0]
        typ = tree.children[1]
        if isinstance(typ, Token):
            tree.children[1] = pascal_case(typ)
        elif typ.data == 'enumerated':
            name = self.unique_type_name(orig_name)
            new_def = Tree('enumdef', [name, typ])
            self.extra_defs.append(new_def)
            tree.children[1] = name
        elif typ.data == 'sequence':
            name = self.unique_type_name(orig_name)
            new_def = Tree('struct', [name, typ])
            self.extra_defs.append(new_def)
            tree.children[1] = name
        else:
            pass

        return tree

    def sequenceof(self, tree):
        item = tree.children[2]
        if isinstance(item, Tree):
            # It must be a container
            assert(item.data == "container")
            item = item.children[1].replace("IEs", "")
        return Tree(
            "Vec<" + pascal_case(item) + ">", [tree.children[0], tree.children[1]])

    def get_bounds(self, tree):
        ub = 0
        lb = 0
        if len(tree.children) > 1:
            lb = tree.children[0]
            ub = tree.children[1]
            if ub is None:
                ub = lb
            else:
                ub = ub
        return (lb, ub)

    def integer(self, tree):
        (lb, ub) = self.get_bounds(tree)
        try:
            ub = int(ub) - int(lb)
        except:
            print("Warning: const bounds not implemented properly")
            ub = 256
        if ub < 256:
            t = "u8"
        elif ub < 65536:
            t = "u16"
        elif ub < 4294967295:
            t = "u32"
        else:
            t = "u64"
        return Tree(t, tree.children)

    def bits(self, tree):
        return Tree("BitString", tree.children)

    def string(self, tree):
        return Tree("String", tree.children)

    def bytes(self, tree):
        return Tree("Vec<u8>", tree.children)

    def boolean(self, tree):
        return Tree("bool", tree.children)


def transform(mut_tree):
    return TypeTransformer().transform(mut_tree)


class TestGenerator(unittest.TestCase):
    maxDiff = None

    def should_generate(self, input, expected):
        output = ""
        tree = parse_string(input)
        try:
            output = TypeTransformer().transform(tree).pretty()
            # print(output)
            self.assertEqual(output, expected)
        finally:
            if output != expected:
                print(tree.pretty())

    def test2(self):
        input = """\
MaximumDataBurstVolume::= INTEGER(0..4095, ..., 4096.. 2000000)
"""
        output = """\
document
  None
  tuple_struct
    MaximumDataBurstVolume
    u16
      0
      4095
      extension_marker
      4096
      2000000
"""
        self.should_generate(input, output)

    def test1(self):
        input = """\
WLANMeasurementConfiguration ::= SEQUENCE {
	wlanMeasConfig             	WLANMeasConfig,
    um-are-you-sure             PrintableString (SIZE (1..150, ...)),
    someNum                     INTEGER (0..1099511627775)
	foo                         SEQUENCE (SIZE (1..maxnoofSliceItems)) OF OverloadStartNSSAIItem,
	wlan-rtt                   	ENUMERATED {thing1, ..., thing2} OPTIONAL,
	n2    SEQUENCE {
            combOffset-n2              INTEGER (0..1),
            cyclicShift-n2             INTEGER (0..7)
        },
	iE-Extensions		ProtocolExtensionContainer {{WLANMeasurementConfiguration-ExtIEs}} 	OPTIONAL,
	...
}
"""
        output = """\
document
  None
  struct
    WlanMeasurementConfiguration
    sequence
      field
        wlan_meas_config
        WlanMeasConfig
      field
        um_are_you_sure
        String
          1
          150
          extension_marker
      field
        some_num
        u64
          0
          1099511627775
      field
        foo
        Vec<OverloadStartNssaiItem>
          1
          maxnoofSliceItems
      optional_field
        wlan_rtt
        WlanRtt
      field
        n2
        N2
      optional_extension_container
        iE-Extensions
        container
          ProtocolExtensionContainer
          WLANMeasurementConfiguration-ExtIEs
      extension_marker
  enumdef
    WlanRtt
    enumerated
      enum_item\tThing1
      extension_marker
      extended_items
        enum_item\tThing2
  struct
    N2
    sequence
      field
        comb_offset_n_2
        u8
          0
          1
      field
        cyclic_shift_n_2
        u8
          0
          7
"""
        self.should_generate(input, output)


if __name__ == '__main__':
    unittest.main(failfast=True)
