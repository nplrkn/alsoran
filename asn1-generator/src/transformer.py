#!/usr/bin/env python3

import unittest
from lark.visitors import Transformer, Visitor, Discard
from case import pascal_case, snake_case
from lark.lexer import Token
from lark import Tree, v_args
from parser import parse_string, parse_file


# Add a new type name and ensure it is unique
def add_type_name(orig_typename, name_dict):
    name = pascal_case(orig_typename)
    existing = name_dict.get(name)
    name_dict[name] = (existing or 0) + 1
    if existing:
        name = name + str(existing)
    return name


class TypeNameFinder(Visitor):
    def __init__(self):
        self.name_dict = dict()
        self.convert = dict()
        self.ie_dict = dict()

    def add(self, orig_typename):
        name = add_type_name(orig_typename, self.name_dict)
        print(f"{orig_typename} -> {name}")
        self.convert[orig_typename] = name

    def choicedef(self, tree):
        self.add(tree.children[0])

    def tuple_struct(self, tree):
        self.add(tree.children[0])

    def enumdef(self, tree):
        self.add(tree.children[0])

    def struct(self, tree):
        self.add(tree.children[0])

    def protocol_ies(self, tree):
        self.ie_dict[tree.children[0]] = tree.children[1]


# Convert a structure like
#    sequence
#      ie_container      PDUSessionResourceSetupRequestIEs
#      extension_marker
#  protocol_ies
#    PDUSessionResourceSetupRequestIEs
#    ies
#      ie
#
# into
#
#    ie_container_sequence
#      ies
#        ie
#      extension_marker
@v_args(tree=True)
class IeContainerMerger(Transformer):
    def __init__(self, ies=dict()):
        self.ie_dict = ies

    # def ie_container(self, tree):
    #     tree.children[0] = self.ie_dict[tree.children[0]]
    #     return tree

    def sequence(self, tree):
        if tree.children[0].data == "ie_container":
            tree.children[0] = self.ie_dict[tree.children[0].children[0]]
            tree.data = "ie_container_sequence"
        return tree

    def protocol_ies(self, tree):
        return Discard


@v_args(tree=True)
class Remover(Transformer):
    def objectdef(self, tree):
        print("Removing objectdef ", tree.children[0])
        return Discard


@v_args(tree=True)
class TypeTransformer(Transformer):
    def __init__(self, constants=dict(), name_dict=dict(), convert=dict()):
        self.extra_defs = []
        self.name_dict = name_dict
        self.convert_dict = convert
        self.constants = constants

    def unique_type_name(self, name):
        return add_type_name(name, self.name_dict)

    def convert(self, orig):
        if orig not in self.convert_dict:
            print(
                f"Warning: unknown type {orig} - guessing name as {pascal_case(orig)}")
            name = pascal_case(orig)
        else:
            name = self.convert_dict[orig]
        return name

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

    def choicefield(self, tree):
        tree = self.transform_type(tree)
        tree.children[0] = pascal_case(tree.children[0])
        return tree

    def optional_field(self, tree):
        tree = self.transform_type(tree)
        tree.children[0] = snake_case(tree.children[0])
        return tree

    def tuple_struct(self, tree):
        tree.children[0] = self.convert(tree.children[0])
        return tree

    def choicedef(self, tree):
        tree.children[0] = self.convert(tree.children[0])
        return tree

    def enumdef(self, tree):
        tree.children[0] = self.convert(tree.children[0])
        return tree

    def struct(self, tree):
        tree.children[0] = self.convert(tree.children[0])
        return tree

    def transform_type(self, tree, type_index=1):
        orig_name = tree.children[0]
        typ = tree.children[type_index]
        if isinstance(typ, Token):
            typename = tree.children[type_index].value
            tree.children[type_index] = self.convert(typename)
        elif typ.data == 'enumerated':
            name = self.unique_type_name(orig_name)
            new_def = Tree('enumdef', [name, typ])
            self.extra_defs.append(new_def)
            tree.children[type_index] = name
        elif typ.data == 'sequence':
            name = self.unique_type_name(orig_name)
            new_def = Tree('struct', [name, typ])
            self.extra_defs.append(new_def)
            tree.children[type_index] = name
        elif typ.data == 'null':
            tree.children[type_index] = 'null'
        else:
            pass

        return tree

    def sequenceof(self, tree):
        item = tree.children[2]
        self.transform_bounds(tree)
        if isinstance(item, Tree):
            # It must be a container
            assert(item.data == "container")
            item = item.children[1].replace("IEs", "")
        return Tree(
            "Vec<" + self.convert(item) + ">", [tree.children[0], tree.children[1]])

    def transform_bounds(self, tree):
        ub = 18446744073709551615
        lb = 0
        extensible = False
        if len(tree.children) <= 1:
            print("Warning: no bounds")
            return (None, None, False)
        else:
            lb = tree.children[0]
            try:
                lb = int(lb)
            except:
                lb = self.constants.get(lb)
                if lb is None:
                    print("Error: unknown constant ", tree.children[0])

            ub = tree.children[1]
            if ub is None:
                ub = lb
            else:
                try:
                    ub = int(ub)
                except:
                    ub = self.constants.get(ub)
                    if ub is None:
                        print("Error: unknown constant ", tree.children[1])

            for idx in range(2, len(tree.children)-1):
                item = tree.children[idx]
                if isinstance(item, Tree) and item.data == "extension_marker":
                    extensible = True

        tree.children[0] = lb
        tree.children[1] = ub

        return (lb, ub, extensible)

    def namedvalues(self, tree):
        return Discard

    def integer(self, tree):
        (lb, ub, extensible) = self.transform_bounds(tree)

        if extensible:
            t = "i128"
        else:
            try:
                range = ub-lb
            except:
                print("Warning: unable to determine size - using u8")
                range = 255
            if range < 256:
                t = "u8"
            elif range < 65536:
                t = "u16"
            elif range < 4294967295:
                t = "u32"
            else:
                t = "u64"
        return Tree(t, tree.children)

    def bits(self, tree):
        self.transform_bounds(tree)
        return Tree("BitString", tree.children)

    def printablestring(self, tree):
        self.transform_bounds(tree)
        # tree.children.append("PrintableString")
        return Tree("PrintableString", tree.children)

    def utf8string(self, tree):
        self.transform_bounds(tree)
        # tree.children.append("UTF8String")
        return Tree("UTF8String", tree.children)

    def visiblestring(self, tree):
        self.transform_bounds(tree)
        # tree.children.append("VisibleString")
        return Tree("VisibleString", tree.children)

    def bytes(self, tree):
        if tree.children != None:
            self.transform_bounds(tree)
        return Tree("Vec<u8>", tree.children)

    def boolean(self, tree):
        return Tree("bool", tree.children)

    def ie(self, tree):
        id = tree.children[0].value
        tree.children[0] = snake_case(id.replace("id-", ""))
        tree.children.insert(1, self.constants[id])
        self.transform_type(tree, 3)
        return tree

    def optional_ie(self, tree):
        return self.ie(tree)


def transform(mut_tree, constants):
    try:
        print("---- Removing ignored objectdefs ----")
        mut_tree = Remover().transform(mut_tree)

        print("---- Finding typenames ----")
        tnf = TypeNameFinder()
        tnf.visit(mut_tree)

        print("---- Merging IE containers ----")
        mut_tree = IeContainerMerger(tnf.ie_dict).transform(mut_tree)

        print("---- Transforming ----")
        return TypeTransformer(constants, tnf.name_dict, tnf.convert).transform(mut_tree)

    except Exception as e:
        print(mut_tree.pretty())
        raise e


class TestGenerator(unittest.TestCase):
    maxDiff = None

    def should_generate(self, input, expected, constants=dict()):
        output = ""
        tree = parse_string(input)
        try:
            output = transform(tree, constants).pretty()
            # print(output)
            self.assertEqual(output, expected)
        finally:
            if output != expected:
                print(tree.pretty())

    def test_discard_integer_named_values(self):
        self.should_generate("""\
        PriorityLevel ::= INTEGER { spare (0), highest (1), lowest (14), no-priority (15) } (0..15)
""", """\
document
  None
  tuple_struct
    PriorityLevel
    u8
      0
      15
""")

    def test3(self):
        self.should_generate("""\
EventTrigger ::= CHOICE {
	blah-bla		NULL,
	short-macroENB-ID 		    BIT STRING (SIZE (18)),
}
""", """\
document
  None
  choicedef
    EventTrigger
    choice
      choicefield
        BlahBla
        null
      choicefield
        ShortMacroEnbId
        BitString
          18
          18
""")

    def test2(self):
        input = """\
MaximumDataBurstVolume::= INTEGER(-234..maxFoo, ..., 4096.. 2000000)
"""
        output = """\
document
  None
  tuple_struct
    MaximumDataBurstVolume
    i128
      -234
      255
      extension_marker
      4096
      2000000
"""
        self.should_generate(input, output, constants={"maxFoo": 255})

    def test1(self):
        input = """\
WLANMeasurementConfiguration ::= SEQUENCE {
	wlanMeasConfig             	WLANMeasConfig,
    um-are-you-sure             PrintableString (SIZE (1..150, ...)),
    someNum                     INTEGER (0..1099511627775),
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
        PrintableString
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
          3
      optional_field
        wlan_rtt
        WlanRtt
      field
        n2
        N2
      extension_container
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
        self.should_generate(input, output, constants={"maxnoofSliceItems": 3})

    def test_optional_octet_string(self):
        self.should_generate("""\
Child-Node-Cells-List-Item ::= SEQUENCE {
	cSI-RS-Configuration				OCTET STRING	OPTIONAL,
-- Here is a random comment    --
  	sR-Configuration					OCTET STRING	OPTIONAL,
	pDCCH-ConfigSIB1					OCTET STRING	OPTIONAL,
	sCS-Common							OCTET STRING	OPTIONAL,
}
""", """\
document
  None
  struct
    ChildNodeCellsListItem
    sequence
      optional_field
        csi_rs_configuration
        Vec<u8>\tNone
      optional_field
        sr_configuration
        Vec<u8>\tNone
      optional_field
        pdcch_config_sib1
        Vec<u8>\tNone
      optional_field
        scs_common
        Vec<u8>\tNone
""")

    def test_inline_name_clash(self):
        self.should_generate("""\
ActiveULBWP ::= SEQUENCE {
	subcarrierSpacing           ENUMERATED { kHz15, kHz30, kHz60, kHz120,... } ,
}
SubcarrierSpacing ::= ENUMERATED { kHz15, kHz30, kHz60, kHz120, kHz240, spare3, spare2, spare1, ... }
""", """\
document
  None
  struct
    ActiveUlbwp
    sequence
      field
        subcarrier_spacing
        SubcarrierSpacing1
  enumdef
    SubcarrierSpacing
    enumerated
      enum_item\tKHz15
      enum_item\tKHz30
      enum_item\tKHz60
      enum_item\tKHz120
      enum_item\tKHz240
      enum_item\tSpare3
      enum_item\tSpare2
      enum_item\tSpare1
      extension_marker
  enumdef
    SubcarrierSpacing1
    enumerated
      enum_item\tKHz15
      enum_item\tKHz30
      enum_item\tKHz60
      enum_item\tKHz120
      extension_marker
""")

    def test_clashing_type_names(self):
        self.should_generate("""\
Foo ::= INTEGER(1..16)
SRSConfig ::= SEQUENCE {
	a SRSResourceSet-List,
	b SRSResourceSetList,
}
SRSResourceSet-List ::= SEQUENCE (SIZE (1..2)) OF Foo
SRSResourceSetList ::= SEQUENCE (SIZE (1.. 3)) OF Foo
""", """\
document
  None
  tuple_struct
    Foo
    u8
      1
      16
  struct
    SrsConfig
    sequence
      field
        a
        SrsResourceSetList
      field
        b
        SrsResourceSetList1
  tuple_struct
    SrsResourceSetList
    Vec<Foo>
      1
      2
  tuple_struct
    SrsResourceSetList1
    Vec<Foo>
      1
      3
""")

    def test_pdu_contents(self):
        self.should_generate("""\
PDUSessionResourceSetupRequest ::= SEQUENCE {
	protocolIEs		ProtocolIE-Container		{ {PDUSessionResourceSetupRequestIEs} },
	...
}

PDUSessionResourceSetupRequestIEs NGAP-PROTOCOL-IES ::= {
	{ ID id-AMF-UE-NGAP-ID							CRITICALITY reject	TYPE AMF-UE-NGAP-ID								PRESENCE mandatory	}|
	{ ID id-RANPagingPriority						CRITICALITY ignore	TYPE RANPagingPriority							PRESENCE optional		}|
	...
}
""", """\
document
  None
  struct
    PduSessionResourceSetupRequest
    ie_container_sequence
      ies
        ie
          amf_ue_ngap_id
          10
          reject
          AmfUeNgapId
        optional_ie
          ran_paging_priority
          83
          ignore
          RanPagingPriority
        extension_marker
      extension_marker
""", constants={"id-AMF-UE-NGAP-ID": 10, "id-RANPagingPriority": 83})

    def test_unconstrained_visible_string(self):
        self.should_generate(
            "URI-address ::= VisibleString", """\
document
  None
  tuple_struct
    UriAddress
    VisibleString
""")

    def test_octet_string(self):
        self.should_generate("""\
SNSSAI ::= SEQUENCE {
	sD			OCTET STRING (SIZE (3)) 	OPTIONAL	,
}
""", """\
document
  None
  struct
    Snssai
    sequence
      optional_field
        sd
        Vec<u8>
          3
          3
""")

    def test_size_constrained_vec(self):
        self.should_generate("""\
Activated-Cells-to-be-Updated-List ::= SEQUENCE (SIZE (1..maxnoofServedCellsIAB)) OF Activated-Cells-to-be-Updated-List-Item
""", """\
document
  None
  tuple_struct
    ActivatedCellsToBeUpdatedList
    Vec<ActivatedCellsToBeUpdatedListItem>
      1
      512
""", constants={"maxnoofServedCellsIAB": 512})


if __name__ == '__main__':
    unittest.main(failfast=True)
