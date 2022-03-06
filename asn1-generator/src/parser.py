from lark import Lark


def parse_string(s):
    file = open("grammar.lark")
    grammar = file.read()
    file.close()
    print("---- Parsing ----")
    parser = Lark(grammar,  start='document')
    return parser.parse(s)


def parse_file(f):
    file = open(f)
    input = file.read()
    file.close()
    return parse_string(input)
