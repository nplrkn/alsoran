#!/usr/bin/env python3

import unittest
from lark.visitors import Interpreter
from lark.lexer import Token
from lark import Tree, v_args
from parser import parse_string, parse_file


class Constants(Interpreter):
    def __init__(self):
        self.constants = dict()

    def constant_def(self, tree):
        key = tree.children[0].value
        int_value = int(tree.children[1].value)
        self.constants[key] = int_value


def get_constants(tree):
    i = Constants()
    i.visit(tree)
    return i.constants


def get_constants_from_file(input_file):
    tree = parse_file(input_file)
    return get_constants(tree)


if __name__ == '__main__':
    input_file = "f1ap/asn1/F1AP-Constants.asn"
    tree = parse_file(input_file)
    print(tree.pretty())
    d = get_constants(tree)
    print(d)
