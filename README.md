
# webrusty

Please follow the instructions:

## Folder **node_example**:

```bash
npm i
node index.js
```

## Folder **rust** (if building wasm is needed):

```bash
wasm-pack build --target web --out-dir ../pkg
```

## Folder **root**:

```bash
# Please install python3
python3 -m http.server
```