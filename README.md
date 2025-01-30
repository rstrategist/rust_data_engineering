# Rust Data Engineering

## Certificate of Completion (63 hours)
([Coursera: Duke University - Data Engineering with Rust](https://coursera.org/share/f04194e4f39022e1b9e933cc1da33c8f))

![image](https://github.com/user-attachments/assets/da663ab9-907d-4141-93d9-8f6dce1984af)
Image sourced from: https://github.com/noahgift/rust-mlops-template

## Modules:
 1. Rust development ecosystem, and data structures: Collections. ✅
 2. Rust safety, security and concurrency.                        ✅
 3. Rust data engineering libraries and tools.                    ✅
 4. Designing data processessing systems in Rust.                 ✅

## Learning Objectives
### ✅ Module 1 - Rust development ecosystem, and data structures: Collections
- Utilise AI coding tools like GitHub Copilot to boost Rust productivity
- Set up continuous integration with Rust and GitHub Actions
- Launch cloud-based Rust development environments with GitHub Codespace/GCP/AWS

### ✅ Module 2 - Rust safety, security and concurrency
- Implement multi-factor authentication and encryption techniques in Rust
- Segment networks and restrict access between zones to improve security
- Write concurrent Rust code using threads and synchronisation primitives
- Build chatbots and GPU-enabled tools by leveraging Rust's speed and efficiency
- Analyse performance of Rust vs. Python for metrics like energy use

### ✅ Module 3 - Rust data engineering libraries and tools
- Read and write CSV, Parquet files with Rust
- Scrape websites and handle APIs asynchronously
- Use data manipulation libraries like Polars, Arrow
- Pass messages for distributed data systems
- Interact with web services via REST, gRPC

### ✅ Module 4 - Designing data processessing systems in Rust
- Learn LLMOps interfaces to Rust
- Map data pipeline concepts using real-world analogies
- Evaluate open source data engineering tools
- Break down key parts of a pipeline - ingestion, processing, storage
- Build AWS Step Functions workflows with Rust
- Create async S3 Lambdas for cloud data tasks
- Explain and use “distroless” containers

# Run with GitHub Actions
GitHub Actions uses a Makefile to simplify automation.
To run everything locally do: `make all`.
```
name: Rust CI/CD Pipeline
on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]
env:
  CARGO_TERM_COLOR: always
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v1
    - uses: actions-rs/toolchain@v1
      with:
          toolchain: stable
          profile: minimal
          components: clippy, rustfmt
          override: true
    - name: update linux
      run: sudo apt update 
    - name: update Rust
      run: make install
    - name: Check Rust versions
      run: make rust-version
    - name: Format
      run: make format
    - name: Lint
      run: make lint
    - name: Test
      run: make test
```

