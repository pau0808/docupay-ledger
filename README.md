# DocuPay Ledger 📄

> On-chain employee document and payroll verification for SMEs.

---

## Problem

An HR officer in a small business in the Philippines manually manages employee contracts and payroll records, causing delays, document tampering risks, and salary disputes when employees cannot verify their payslips or payment status.

---

## Project Description (Solution)

DocuPay Ledger allows HR staff to register employee document hashes on-chain and mark payroll records as paid using Soroban smart contracts. Employees can instantly verify that their documents and salary records are authentic, using Stellar’s fast, low-cost, and transparent blockchain.

---

## Stellar Features Used

| Feature | Purpose |
|--------|--------|
| Soroban smart contracts | Store document records and payroll status |
| USDC/XLM references | Represent payroll transactions |
| Trustlines | Enable asset support for future payroll integration |
| On-chain storage | Tamper-proof document verification |

---

## Timeline

| Phase | Description |
|------|------------|
| Day 1 | Smart contract design and data structure |
| Day 2 | Implement upload_doc and pay_salary |
| Day 3 | Testing and debugging |
| Day 4 | Deployment to Stellar testnet |
| Day 5 | README and demo preparation |

---

## Vision and Purpose

DocuPay Ledger solves a real-world problem faced by SMEs that lack proper HR systems.

By moving document verification and payroll records on-chain:
- HR staff reduce disputes and manual work  
- Employees gain verifiable proof of documents and salary  
- Businesses improve trust and transparency  
- Systems become tamper-resistant and auditable  

---

## Prerequisites

- Rust (latest stable)
- Stellar CLI
- Stellar testnet account

---

## Build

(( stellar contract build --manifest-path contracts/bootcamp_project/Cargo.toml ))

Output:
(( target/wasm32v1-none/release/docupay_ledger.wasm ))

---

## Test

```bash
cargo test
```
---

## Deploy to Testnet

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/docupay_ledger.wasm \
  --source stellar-ide-default \
  --network testnet
```

---

## CLI Usage Examples

### Upload employee document

```bash
stellar contract invoke \
  --id <contract_id> \
  --source stellar-ide-default \
  --network testnet \
  --fn upload_doc \
  -- \
  --employee <employee_address> \
  --doc_hash doc1
```

### Mark salary as paid

```bash
stellar contract invoke \
  --id <contract_id> \
  --source stellar-ide-default \
  --network testnet \
  --fn pay_salary \
  -- \
  --id 0
```

### Get employee record

```bash
stellar contract invoke \
  --id <contract_id> \
  --network testnet \
  --fn get_record \
  -- \
  --id 0
```

---

## Project Structure

```bash
docupay_ledger/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   └── test.rs
```

---

## Deployed Contract Details

Transaction:
https://stellar.expert/explorer/testnet/tx/493f1a44d06b59754488200eaa5900ba2c15bc14620c0f42cbc5fff6ad3bb9a8

Contract:
https://lab.stellar.org/r/testnet/contract/CBBHKQO7DTI5CHAGBMUVULZRCZJWPXFKP5IITIKHDESBF5CRJ6NKDPAGP

---

## Future Scope

- HR-only authentication system  
- Real USDC payroll integration  
- Employee wallet login  
- PDF document hashing  
- QR code verification  
- Web-based HR dashboard  

---

## License

MIT License — free to use, modify, and deploy for educational purposes.