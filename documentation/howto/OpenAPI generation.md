# OpenAPI generation

(Not currently in use in project.)

openapitools was installed using the launcher script at [https://github.com/OpenAPITools/openapi-generator#1---installation].

To run,

```bash
~/bin/openapitools/openapi-generator-cli generate --generate-alias-as-model -i node-control.yaml -g rust-server --additional-properties=packageName="node-control-api"
```
