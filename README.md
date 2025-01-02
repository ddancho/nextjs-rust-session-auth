# Auth Demo App with Rust server and NextJS client

This fullstack project is auth demo app for rust [actix-web](https://actix.rs/) framework as server component and
[next.js](https://nextjs.org/) framework as client component with ssr utilization (server components, server actions).

Authentication on the server is done with session management built on top of actix-web
with [actix-session](https://github.com/actix/actix-extras/tree/master/actix-session) and persistent CookieSessionStore
as built-in storage backend.

Because of nextjs ssr feature extra care is done on the client side with the cookie management.

## Getting Started

For the server you will need to install rust:

- [rust](https://www.rust-lang.org/tools/install)

and docker-compose for running the postgres database, adminer (db viewer) images in the containers:

- [Docker Compose Install](https://docs.docker.com/compose/install/)

For the client you will need a node.js runtime environment:

- [Node.js](https://nodejs.org/en/)

## First step:

- clone or download repository

## Next step (Server):

- in the server folder copy/paste .env.example and rename to .env and write down required information values,
  local development environment defaults are
  host = localhost and
  port = 5001
  ,be aware that same values are defaults for the client local development data fetching

- run docker-compose file, it will pull required images from docker hub and
  start in the detached mode, and also it will create local persistent postgres volume:

```bash
docker-compose up -d
```

- stop docker services with

```bash
docker-compose down
```

- compile packages with

```bash
cargo build
```

- run the app

```bash
cargo run
```

- on the first run you will notice in the terminal informations that migrations are executed by [sqlx](https://crates.io/crates/sqlx), if you are planing to add any extra migration please read the documentations about that topic

## Next step (Client):

- in the client folder copy/paste .env.example and rename to .env

- install packages with

```bash
npm install
```

- run the client with

```bash
npm run dev
```

## Adminer

For the adminer open [http://localhost:8080](http://localhost:8080) with your browser to see the adminer login page,
the server name in the login page is the name of the postgres service from docker-compose.yml file, so pg_auth_bck

## App

Open [http://localhost:3000](http://localhost:3000) with your browser to see the app,
open your browser development tools and observe cookies.
