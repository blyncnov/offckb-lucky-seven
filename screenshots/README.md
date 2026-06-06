# Screenshots

Drop the 4 deploy-proof screenshots in this folder using these exact filenames. The main README already links to them.

| Filename | What to capture |
|---|---|
| `01-offckb-node-running.png` | Terminal running `offckb node` — must show the line `CKB devnet RPC Proxy server running on http://127.0.0.1:28114` |
| `02-build-success.png` | Terminal showing `make build` finishing clean **and** the output of `file build/release/lucky-seven` showing `UCB RISC-V` |
| `03-deploy-success.png` | Terminal showing `offckb deploy ... --yes` with the tx hash `0xccd879...` and the line `tx committed.` |
| `04-scripts-json.png` | Terminal output of `cat deployment/scripts.json` showing the `codeHash` and `txHash` |

After dropping them in, run from the repo root:

```
git add screenshots/
git commit -m "Add deploy proof screenshots"
git push
```
