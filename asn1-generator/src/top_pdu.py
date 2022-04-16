#!/usr/bin/env python3
import unittest
import parser
from parser import parse_file, parse_string
from lark import Visitor
from case import pascal_case


def add_to_enum(leaf):
    name = pascal_case(leaf)
    return f"""\
    {name}({name}),
"""


class TopPduVisitor(Visitor):
    def __init__(self):
        self.initiating_enum = """\
#[derive(Clone, Debug)]
enum InitiatingMessage {
"""
        self.successful_enum = """\
#[derive(Clone, Debug)]
enum SuccessfulOutcome {
"""
        self.unsuccessful_enum = """\
#[derive(Clone, Debug)]
enum UnsuccessfulOutcome {
"""

    def ignored(self, _tree):
        pass

    def initiating(self, tree):
        self.initiating_enum += add_to_enum(tree.children[0])

    def successful(self, tree):
        self.successful_enum += add_to_enum(tree.children[0])

    def unsuccessful(self, tree):
        self.unsuccessful_enum += add_to_enum(tree.children[0])

    def procedure_def(self, tree):
        pass

    def generate(self):
        output = f"""\
#[derive(Clone, Debug)]
enum Pdu {{
    InitiatingMessage(InitiatingMessage),
    SuccessfulOutcome(SuccessfulOutcome),
    UnsuccessfulOutcome(UnsuccessfulOutcome),
}}

{self.initiating_enum}}}

{self.successful_enum}}} 

{self.unsuccessful_enum}}}
"""
        return output


def generate_top_pdu(tree):
    v = TopPduVisitor()
    v.visit(tree)
    return v.generate()


class TestTransformer(unittest.TestCase):
    maxDiff = None

    def should_generate(self, input, expected, constants=dict()):
        output = ""
        tree = parse_string(input, grammar="procedures.lark")
        try:
            output = generate_top_pdu(tree)
            self.assertEqual(output, expected)
        finally:
            if output != expected:
                print(tree.pretty())

    def test(self):
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
#[derive(Clone, Debug)]
enum Pdu {
    InitiatingMessage(InitiatingMessage),
    SuccessfulOutcome(SuccessfulOutcome),
    UnsuccessfulOutcome(UnsuccessfulOutcome),
}

#[derive(Clone, Debug)]
enum InitiatingMessage {
    AmfConfigurationUpdate(AmfConfigurationUpdate),
    HandoverNotify(HandoverNotify),
}

#[derive(Clone, Debug)]
enum SuccessfulOutcome {
    AmfConfigurationUpdateAcknowledge(AmfConfigurationUpdateAcknowledge),
} 

#[derive(Clone, Debug)]
enum UnsuccessfulOutcome {
    AmfConfigurationUpdateFailure(AmfConfigurationUpdateFailure),
}
""")


if __name__ == '__main__':
    unittest.main(failfast=True)

# input_file = "ngap/asn1/NGAP-PDU-Descriptions.asn"
# tree = parse_file(input_file, "procedures.lark")
# v = TopPduVisitor()
# v.visit(tree)
