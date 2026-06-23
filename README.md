# Gothic 1 Remake: Save manipulation tool

CLI tool for decompression, modification, and the recompression Gothic 1 Remake Save Files. In pre-alpha state. Doesn't do anything. Might never.

Done to make my life easier while the devs work through their bugs.

## Remaining Work for Alpha

1. Oodle FFI, decompress/rebuild
2. Validate + export lists
3. Apply pipeline + pack

## Proposed E2E Alpha flow

```mermaid
graph LR
  sav["game.sav"] --> expCmd["export"]
  expCmd --> yaml["game.sav.edit.yaml"]
  yaml --> userEdit["User edits file"]
  userEdit --> packCmd["pack"]
  sav --> packCmd
  packCmd --> out["game.fixed.sav"]

  subgraph core ["gothic-save"]
    expCmd
    packCmd
  end
```

### Export pipeline

```mermaid

flowchart TB
  sav["game.sav"]
  yaml["game.sav.edit.yaml"]

  subgraph export_flow ["export"]
    E1["Read .sav"]
    E2["Container parse + Oodle decompress"]
    E3["List attributes, inventory, quests, passages"]
    E4["Build Sidecar, write YAML"]
    E1 --> E2 --> E3 --> E4
  end

  sav --> export_flow
  export_flow --> yaml

```

### Pack Pipeline

```mermaid
flowchart TB
  sav["game.sav"]
  yaml["game.sav.edit.yaml"]
  out["game.fixed.sav"]

  subgraph pack_flow ["pack"]
    P1["Read original .sav and YAML"]
    P2["Decompress payload"]
    P3["Diff sidecar vs live lists"]
    P4["Apply edits in fixed order"]
    P5["Validate payload"]
    P6["Rebuild container and Oodle compress"]
    P7["Atomic write temp then rename"]
    P1 --> P2 --> P3 --> P4 --> P5 --> P6 --> P7
  end

  sav --> pack_flow
  yaml --> pack_flow
  pack_flow --> out
```

## File Structure

```text
gothic-remake-rust-save-manipulation-tool/
├── Cargo.toml              # workspace: gothic-save + gothic-save-cli
├── rust-toolchain.toml     # Rust 1.85.0
├── crates/
│   ├── gothic-save/        # library
│   │   └── src/
│   │       ├── lib.rs      # re-exports Save, Error
│   │       ├── save.rs     # raw byte read/write only
│   │       └── error.rs    # Io + InvalidFormat
│   └── gothic-save-cli/    # binary (planned: g1r-cli)
│       └── src/main.rs     # `gothic-save info <path>`
└── README.md
```

- [Idea "stolen" from Xetoxyc's web application](https://github.com/Xetoxyc/gothic-remake-savegame-editor/tree/main)
