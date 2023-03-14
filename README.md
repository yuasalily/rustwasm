# rust wasm
nodeでwasmの動作確認をする場合
```
wasm-pack build --target nodejs
node test_wasm.js
```

webに組み込む場合
```
wasm-pack build --target web
```