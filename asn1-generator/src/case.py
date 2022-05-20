import re

KNOWN_WORDS = [(re.compile(x, re.IGNORECASE), "-"+x+"-")
               for x in ["ngran", "enb", "gnb", "eutran", "plmn", "qos", "rlf", "iwf", "iot"]]

ACRONYMS = re.compile(r"([A-Z,0-9]*)(?=(?=[A-Z][a-z]*)|$|-|_)")


def replace_rust_keywords(s):
    return 'typ' if s == 'type' else s


def capitalize_first_only(s):
    return s[0].capitalize() + s[1:]


def split_words(s):
    # Find the known words.  These are the cases where the ACRONYMS
    # regex isn't smart enough to identify the word.
    for (regex, replace) in KNOWN_WORDS:
        s = regex.sub(replace, s)

    return [word for word in
            [word.replace("-", "") for word in ACRONYMS.split(s)]
            if word not in ('')]


def snake_case(s):
    s = replace_rust_keywords(s)
    s = capitalize_first_only(s)
    words = [word.lower() for word in split_words(s)]
    s = '_'.join(words)
    return s


def pascal_case(s):
    s = capitalize_first_only(s)
    words = [word.capitalize() for word in split_words(s)]
    s = ''.join(words)
    return s
