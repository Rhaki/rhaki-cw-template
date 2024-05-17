## CosmWasm contracts template workspace enable

Tool to generate CosmWasm smart contract workspace enable template project

### Create new repo:

```ssh
cargo generate --git https://github.com/Rhaki/rhaki-cw-template.git --name project_name
```

The repo will be created with `workspace` defined on the primary `Cargo.toml`

### Add a contract into the project:
```
cargo make add-contract contract_name
```

This command launches a python script that:
- Create a new folder structure and files for the contract at `./contracts/`;
- Adds the new contract to members in the `workspace`;
- Create a new `mod` file inside `./package/src/`
- Insert the export of the new file created in `./package/src/lib.rs`
