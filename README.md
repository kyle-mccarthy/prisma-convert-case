# prisma convert case

- converts case of models to CamelCase
- converts case of fields to mixedCase

## usage

if a path isn't specified, it will attempt to load the schema at ./schema.prisma or ./prisma/schema.prisma

```
cargo run
```

optionally, a path can be specified

```
cargo run -- path/to/prisma.schema
```
