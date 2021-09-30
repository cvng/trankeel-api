# Piteo

[![ci](https://github.com/piteo-team/piteo/workflows/ci/badge.svg?branch=main&event=push)](https://github.com/piteo-team/piteo/actions)
[![codecov](https://codecov.io/gh/piteo-team/piteo/branch/main/graph/badge.svg?token=1EC07KNEFJ)](https://codecov.io/gh/piteo-team/piteo)
[![storybook](https://raw.githubusercontent.com/storybookjs/brand/master/badge/badge-storybook.svg)](https://main--5f3a2a460ab9350022c1e244.chromatic.com)

<img align="right" src="https://avatars.githubusercontent.com/u/64363572?s=200&v=4" height="150px">

Piteo is a _simple_, _modern_, and _featured_ lease management application for
**individual** and **professional** lender that uses GraphQL and is built in
Rust.

### Features

- Lease management
- Receipt editing
- Tenant onboarding

### Install

HTTPS (Git):

```sh
git clone https://github.com/piteo-team/piteo.git
```

SSH (Git):

```sh
git clone git@github.com:piteo-team/piteo.git
```

[GitHub CLI](https://cli.github.com):

```sh
gh repo clone piteo-team/piteo
```

### Getting started

Try running the web application:

```sh
cargo run --bin piteo-web
```

And the API server:

```sh
cargo run
```

You can find a deeper introduction, examples, and environment setup guides in
each individual package:

- [`piteo-api`](https://github.com/piteo-team/piteo/blob/main/packages/api/README.md)
- [`piteo-web`](https://github.com/piteo-team/piteo/blob/main/packages/web/README.md)

The complete GraphQL API reference is available at the online
[playground](https://piteo-api.herokuapp.com/graphql).

### Contributing

We appreciate your help!

To contribute, please read our [contribution](https://github.com/piteo-team/piteo/blob/main/CONTRIBUTING.md) guide.
