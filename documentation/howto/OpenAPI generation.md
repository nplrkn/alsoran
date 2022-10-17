# OpenAPI generation

openapitools was installed using the launcher script at [https://github.com/OpenAPITools/openapi-generator#1---installation].

To run,

```bash
~/bin/openapitools/openapi-generator-cli generate --generate-alias-as-model -i node-control.yaml -g rust-server --additional-properties=packageName="node-control-api"
```

```bash
~/bin/openapitools/openapi-generator-cli generate --generate-alias-as-model -i connection-api.yaml -g rust-server --additional-properties=packageName="connection-api"
```

# h2 compile error

To fix compile error
```
error[E0107]: this struct takes 3 generic arguments but 2 generic arguments were supplied
```

add the following to the rustflags in node-control-api/.cargo/config
```
    "--cfg=has_std"
```