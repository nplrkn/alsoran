# OpenAPI generation

openapitools was installed using the launcher script at [https://github.com/OpenAPITools/openapi-generator#1---installation].

To run,

```bash
# Run this from within the coordination-api directory, to overwrite the existing code
~/bin/openapitools/openapi-generator-cli generate --generate-alias-as-model -i coordination-api.yaml -g rust-server --additional-properties=packageName="coordination-api"

# Delete the files we don't need
rm -rf api docs examples .gitignore .openapi-generator .openapi-generator-ignore README.md

# Remove the examples targets from Cargo.toml
awk '/\[\[example\]\]/{exit}' Cargo.toml > tempfile
mv tempfile Cargo.toml
```

```bash
# Run this from within the connection-api directory, to overwrite the existing code
~/bin/openapitools/openapi-generator-cli generate --generate-alias-as-model -i connection-api.yaml -g rust-server --additional-properties=packageName="connection-api"

# Delete the files we don't need
rm -rf api docs examples .gitignore .openapi-generator .openapi-generator-ignore README.md

# Remove the examples targets from Cargo.toml
awk '/\[\[example\]\]/{exit}' Cargo.toml > tempfile
mv tempfile Cargo.toml
```
