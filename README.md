# Account

## Environment setup

Create `.env` with following content:

```
POSTGRES_USER=<dbuser>
POSTGRES_PASSWORD=<dbpassword>
POSTGRES_DB=<dbname>

LISTEN_ADDR=0.0.0.0:9090
DATABASE_URL=postgres://<dbuser>:<dbpassword>@localhost/<dbname>
REDIS_URL=redis://<redis_host>/
PASSWORD_SALT=<random secret string>
TOKEN_SECRET=<random secret string>
```

## Running

Run `docker-compose up --build` and open `http://localhost:8080` in your web browser.

## Local development

1. run envoy proxy

```
docker run --network host -v `pwd`/envoy.yaml:/etc/envoy/envoy.yaml envoyproxy/envoy:v1.17-latest
```

2. start database

Install postgres and setup user/password same as `.env` file.

or

```
docker-compose up -d db
```

3. start redis

Install redis

or

```
docker-compose up -d redis
```

4. run server

```
cargo run --bin server
```

5. run client

```
npm install
npm start
```

6. Visit `http://localhost:8080` in your web browser
