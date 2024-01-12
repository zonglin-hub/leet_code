# 新增 miri

```yml
jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Install Miri
      run: |
        rustup toolchain install nightly --component miri
        rustup override set nightly
        cargo miri setup

    - name: Test with Miri
      run: cargo miri test
```
