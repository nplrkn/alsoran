#!/usr/bin/env python3
from lark import Lark
import sys


def parse_string(s, grammar='grammar.lark'):
    file = open(grammar)
    grammar = file.read()
    file.close()
    print("---- Parsing ----")
    parser = Lark(grammar,  start='document')
    return parser.parse(s)


def parse_file(f, grammar='grammar.lark'):
    file = open(f)
    input = file.read()
    file.close()
    return parse_string(input, grammar)


if __name__ == '__main__':
    file = sys.argv[1]
    print(parse_file(file).pretty())
