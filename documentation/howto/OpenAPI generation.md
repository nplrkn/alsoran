# OpenAPI generation

openapitools was installed using the launcher script at [https://github.com/OpenAPITools/openapi-generator#1---installation].

To run,

```bash
~/bin/openapitools/openapi-generator-cli generate --generate-alias-as-model -i coordination-api.yaml -g rust-server --additional-properties=packageName="coordination-api"
```

```bash
~/bin/openapitools/openapi-generator-cli generate --generate-alias-as-model -i connection-api.yaml -g rust-server --additional-properties=packageName="connection-api"
```
