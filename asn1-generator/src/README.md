# ASN.1 generator

This is a Python program that reads the ASN.1 files defining the RAN reference points and outputs Rust models that implement Aligned PER coding.

# Run the autogenerator

```
./autogen.py
```

The above command reads the NGAP, F1AP, E1AP and RRC ASN.1 files and overwrites various files in the corresponding directories.

# Parse an ASN.1 file
Useful to debug problems with the grammar.

```
./parse.py ../asn/something/something.asn
```

# Test the first phase (ASN.1 syntax transform)

The ASN.1 is parsed and the resulting parse tree is then reorganized into a new simplified tree that is more conducive for Rust rendering.  The following command runs the tests that focus on this first transform phase. 

```
./transform.py
```

# Test the second phase (Rust rendering)

The tree output by the transform phase is then converted to Rust.  The following command runs the tests that focus on this second rendering phase. 

```
./render.py
```
