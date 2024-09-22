
turtle:
  wasm-pack build --target web --debug

watch:
  watchexec -e rs -- just turtle

web:
  cd pkg && python3 -m http.server 8000
