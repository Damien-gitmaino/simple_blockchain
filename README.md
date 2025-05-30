# simple_blockchain

## Start the server

```bash
cargo run
```

## Test with curl or Postman

### 📥 Add a block

```bash
curl -X POST http://localhost:8080/mine -H "Content-Type: application/json" -d '{"data": "Your DATA !"}'
```

### 📤 View all blocks

```bash
curl http://localhost:8080/blocks
```# simple_blockchain
