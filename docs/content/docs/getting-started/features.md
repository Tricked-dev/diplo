+++
title = "Features"
description = "All features of diplo."
date = 2021-05-01T08:20:00+00:00
updated = 2021-05-01T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "All features of diplo."
toc = true
top = false
+++

## Scripts

use `diplo run --help` for more info

```toml
[scripts]
start = "deno run -A mod.ts"
```

## Dependencies

```toml
[dependencies]
natico = "https://deno.land/x/natico@3.0.1/mod.ts"
```

#### Custom exports

You can define custom exports by adding the `exports` string to a dependency

```toml
[dependencies]
natico = { url="https://deno.land/x/natico@3.0.1/mod.ts", exports="* as natico" }
```

```toml
[dependencies]
natico = { url="https://deno.land/x/natico@3.0.1/mod.ts", exports="NaticoCommandHandler" }
```

```toml
[dependencies]
natico = { url="https://deno.land/x/natico@3.0.1/mod.ts", exports="{ NaticoCommandHandler }" }
```

#### Locking dependencies

Locking dependencies makes it so `diplo upgrade` wont update them anymore

```toml
[dependencies]
natico = { url="https://deno.land/x/natico@3.0.1/mod.ts", locked=true }
```

#### Adding dependency types

This is sometimes needed when a dependency has .d.ts types, Diplo will automatically add `// @deno-types="https://url/to/types.d.ts"` to these dependencies

```toml
[dependencies]
natico = { url="https://deno.land/x/natico@3.0.1/mod.ts", typss="https://url/to/types.d.ts" }
```

## Loading env

this will open the .env file in the current directory and add the environment values to the process env

```toml
load_env = true
```

## Import maps

Diplo will automatically create import maps from the dependencies and append the import map to `deno run`

```toml
import_map = true
```
