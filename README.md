# Diplo <!-- omit in toc -->

Diplo is a deno script runner made in rust.

- [Features](#features)
  - [Script running](#script-running)
  - [Dependencies](#dependencies)
  - [Dotenv Support](#dotenv-support)
  - [Example Config](#example-config)

# Features

## Script running

create a file called diplo.json and put the following code in there

```json
{
	"scripts": {
		"start": "deno run -A mod.ts",
		"node": "node index.js"
	}
}
```

## Dependencies

Dependencies allow putting all your dependencies into one json file it also supports making [import maps](https://deno.land/manual@v1.14.3/npm_nodejs/import_maps#overriding-imports)

You can now import the dependencies from .diplo/deps.ts

```json
{
	"dependencies": {
		"natico": "https://deno.land/x/natico/mod.ts",
		"server": "https://deno.land/std@0.110.0/http/server.ts"
	}
}
```

for import map support

- note Diplo will automatically append `--import-map="./.diplo/import_map.json` after deno run.

```json
{
	"import_map": true
}
```

## Dotenv Support

Diplo can automatically add environment variables using the rust dotenv module instead of the deno based one

```json
{
	"load_env": true
}
```

## Example Config

```json
{
	"scripts": {
		"test": "deno run -A mod.ts",
		"build": "deno bundle a.ts"
	},
	"import_map": false,
	"load_env": true,
	"dependencies": {
		"natico": "https://deno.land/x/natico/mod.ts"
	}
}
```

## Donating <!-- omit in toc -->

You can support the project by donating to my xmr address `89prBkdG58KU15jv5LTbP3MgdJ2ikrcyu1vmdTKTGEVdhKRvbxgRN671jfFn3Uivk4Er1JXsc1xFZFbmFCGzVZNLPQeEwZc`

## License <!-- omit in toc -->

This project is licensed under the terms of the [Apache License 2.0](./LICENSE)
