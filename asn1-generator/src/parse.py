from lark import Lark


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
