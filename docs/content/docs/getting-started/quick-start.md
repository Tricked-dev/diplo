+++
title = "Quick Start"
description = "Simple guide on how to get started with using Diplo."
date = 2021-05-01T08:20:00+00:00
updated = 2021-05-01T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Simple guide on how to get started with using Diplo."
toc = true
top = false
+++

## Requirements

[Diplo](./installing)

## Initializing a project

```sh
$ diplo init
```

Output:

```
name : Diplo
load_env (false): true
import_map (false): false
Successfully wrote changes to diplo.toml

> Done in 7s 519ms 150us
```

## Adding a dependency

```sh
$ diplo add natico
```

Output:

```
Successfully added natico@3.0.2 to the dependencies

> Done in 609ms 122us
```

## Creating a script

Edit your diplo.toml file to add a script

```toml
name= "Diplo"
load_env=true
import_map=false
[watcher]
[dependencies]
natico = "https://deno.land/x/natico@3.0.2/mod.ts"
[scripts]
start="deno run -A mod.ts"
```

Run the script

```sh
$ diplo run start
```

## Updating dependencies

```sh
$ diplo update
```

Output:

```
updated natico to 3.0.2 from 3.0.0

> Done in 436ms 924us
```
