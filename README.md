# Account

## Running

Run `docker-compose up --build` and open `http://localhost:8080` in your web browser.

## Local development

1. run envoy proxy

```
docker run --network host -v `pwd`/envoy.yaml:/etc/envoy/envoy.yaml envoyproxy/envoy:v1.17-latest
```

2. run server

```
cargo run --bin server
```

3. run client

```
npm start
```

4. Visit `http://localhost:8080` in your web browser
