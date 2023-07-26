#!/usr/bin/env python3
import unittest
import re

KNOWN_WORDS = [(re.compile(x, re.IGNORECASE), "-"+x+"-")
               for x in ["ngran", "enb", "gnb", "eutran", "qos", "rlf", "iwf", "iot", "rrc"]]

ACRONYMS = re.compile(r"([A-Z,0-9]*)(?=(?=[A-Z][a-z]*)|$|-|_)")

KNOWN_WORDS_CASE_SENSITIVE = [(re.compile(x), "-"+x+"-")
                              for x in ["NR", "CU"]]

SPECIALS = [(re.compile("^DU"), "DU-"),
            (re.compile(r"PLMN(s?)"), r"-plmn\1-"),
            (re.compile(r"([^P])DU"), r"\1-DU-"),
            (re.compile(r"UE(s?)"), r"-ue\1-"),
            (re.compile(r"SRB(s?)"), r"-srb\1-"),
            (re.compile(r"type(s?)"), r"-type\1-"),
            (re.compile(r"DRB(s?)"), r"-drb\1-"),
            (re.compile(r"E-UTRAN"), r"EUTRAN"),
            (re.compile(r"([^I])DL"), r"\1-DL-"),
            (re.compile(r"([^SN])UL"), r"\1-UL-"),
            (re.compile(r"UPTNL"), r"-UP-TNL-"),
            (re.compile(r"S-NSSAI"), r"SNSSAI"),
            (re.compile(r"sib([^l])",flags=re.IGNORECASE), r"-sib-\1"),
            (re.compile(r"tobeupdated"), r"-to-be-updated-"),
            ]


def replace_rust_keywords(s):
    return 'typ' if s == 'type' else s


def capitalize_first_only(s):
    return s[0].capitalize() + s[1:]


def split_words(s):
    # Find the known words.  These are the cases where the ACRONYMS
    # regex isn't smart enough to identify the word.
    for (regex, replace) in KNOWN_WORDS + KNOWN_WORDS_CASE_SENSITIVE + SPECIALS:
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


class TestCase(unittest.TestCase):
    maxDiff = None

    def test_du(self):
        self.assertEqual(pascal_case("PDU"), "Pdu")
        self.assertEqual(pascal_case("DUtoCURRCContainer"),
                         "DuToCuRrcContainer")
        self.assertEqual(pascal_case("SomethingDU"),
                         "SomethingDu")

    def test_ues(self):
        self.assertEqual(snake_case("numberofActiveUEs"),
                         "numberof_active_ues")

    def test_srbs(self):
        self.assertEqual(pascal_case("SRBs-FailedToBeSetup-List"),
                         "SrbsFailedToBeSetupList")

    def test_eutran(self):
        self.assertEqual(pascal_case(
            "e-UTRAN-BearerContextSetupRequest"), "EutranBearerContextSetupRequest")

    def test_plmns(self):
        self.assertEqual(pascal_case(
            "SupportedPLMNs-List"), "SupportedPlmnsList")

    def test_snssai(self):
        self.assertEqual(pascal_case(
            "S-NSSAI"), "Snssai")

    def test_nid(self):
        self.assertEqual(pascal_case(
            "BroadcastNIDList"), "BroadcastNidList")

    def test_sul(self):
        self.assertEqual(pascal_case(
            "SULAccessIndication"), "SulAccessIndication")
        
    def test_sib(self):
        self.assertEqual(pascal_case(
            "SibtypetobeupdatedListItem"), "SibTypeToBeUpdatedListItem")
        
    def test_sib2(self):
        self.assertEqual(pascal_case(
            "AmfNameVisibleString"), "AmfNameVisibleString")

    def test_sib_message(self):
        self.assertEqual(snake_case(
            "sIBmessage"), "sib_message")


if __name__ == '__main__':
    unittest.main(failfast=True)
