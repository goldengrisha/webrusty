
# webrusty

Please follow the instructions:

## Folder `server`:

```bash
npm i
node index.js
```

## Folder `rust` (if building wasm is needed):

```bash
wasm-pack build --target web --out-dir ../client/pkg
```

## Folder `client`:

```bash
# Please install python3
python3 -m http.server
```