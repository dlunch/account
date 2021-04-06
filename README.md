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

4. run scrap service

(run astx if required)

```
docker build . --file Dockerfile.scrap --tag scrap
docker run -it -p 55920:55930 --entrypoint /bin/sh scrap -c '/opt/AhnLab/ASTx/astxdaemon & socat TCP-LISTEN:55930,fork TCP:127.0.0.1:55920'
```

install geckodriver if not installed(package named firefox-geckodriver or similar)

```
geckodriver &
env WEBDRIVER_HOST="http://127.0.0.1:4444" WEBDRIVER_HEADED=TRUE cargo run --bin scrap
```

5. run server

```
cargo run --bin server
```

6. run client

```
npm install
npm start
```

7. Visit `http://localhost:8080` in your web browser
