# s3-bucket-filename-validator

Small script that checks whether every folder in the bucket `reports` contains exactly one `.txt` file and, aside from that, only JSON files matching the format `yyyy-mm-dd.json`.

---

## 1. Prerequisites

- Rust / Rustup installed
- S3-compatible server with a bucket named `reports`

---

## 2. Environment Variables

Define the following environment variables either in `/etc/environment` or in a local `.env` file:

```env
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
AWS_REGION=
AWS_ENDPOINT_URL=
```

---

## 3. Execution

### 3.1 Instant Run

```bash
cargo run
```

---

### 3.2 Build + Run

Build the binary:

```bash
cargo build
```

Run the executable:

```bash
./target/debug/s3-bucket-filename-validator
```
