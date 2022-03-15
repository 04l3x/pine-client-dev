# dev environment for frontend

clone workspace
```bash
git clone --recurse-submodules https://gitlab.com/0al3x/pine-client-dev.git
```

build frontend
```bash
cd ./pine-client && wasm-pack build --target web
```

run server
```bash
cd .. && cargo run
```

frontend will run in `localhost:9500/`
