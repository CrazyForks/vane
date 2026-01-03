# Detailed Refactoring Plan: Vane 2.0

This plan breaks down the structural overhaul into atomic, verifiable steps.
**Rule of Thumb:** Run `cargo check` after every single sub-step to catch import errors immediately.

## Phase 1: Foundation (Common & Resources)
*Goal: Stabilize leaf dependencies first.*

- [ ] **1.1: Restructure `src/common/`**
    - [ ] Create dirs: `src/common/config`, `src/common/net`, `src/common/sys`.
    - [ ] Move/Rename `getconf.rs`, `getenv.rs`, `loader.rs` -> `src/common/config/`.
    - [ ] Move/Rename `ip.rs`, `portool.rs` -> `src/common/net/`.
    - [ ] Move/Rename `lifecycle.rs`, `system.rs`, `watcher.rs`, `hotswap.rs` -> `src/common/sys/`.
    - [ ] Update `src/common/mod.rs` and fix imports in `src/common/*`.
    - [ ] Fix global imports for `crate::common::...`.

- [ ] **1.2: Establish `src/resources/` (Part A: KV & Certs)**
    - [ ] Create `src/resources/`.
    - [ ] Move `src/modules/kv/` -> `src/resources/kv/`.
    - [ ] Move `src/modules/certs/` -> `src/resources/certs/`.
    - [ ] Fix global imports for `crate::modules::kv` and `crate::modules::certs`.

- [ ] **1.3: Establish `src/resources/` (Part B: Service Discovery & Templates)**
    - [ ] Move `src/modules/nodes/` -> `src/resources/service_discovery/`.
    - [ ] Move `src/modules/template/` -> `src/resources/templates/`.
    - [ ] Fix global imports.

## Phase 2: The Engine Core
*Goal: Centralize the "Contract" and "Executor" to break dependency cycles.*

- [ ] **2.1: Extract the Contract (Traits)**
    - [ ] Create `src/engine/`.
    - [ ] Move `src/modules/plugins/core/model.rs` -> `src/engine/contract.rs`.
    - [ ] **Crucial:** Update all Plugins and Layers to import traits from `crate::engine::contract`.

- [ ] **2.2: Move Flow Logic**
    - [ ] Move `src/modules/flow/engine.rs` -> `src/engine/executor.rs`.
    - [ ] Move `src/modules/flow/context.rs` -> `src/engine/context.rs`.
    - [ ] Move `src/modules/flow/key_scoping.rs` -> `src/engine/key_scoping.rs`.
    - [ ] Update `src/engine/mod.rs` to export these.
    - [ ] Fix global imports for `crate::modules::flow`.

## Phase 3: The Protocol Stack (Layers)
*Goal: Flatten the deep nesting of the network stack.*

- [ ] **3.1: Layer 4 (Transport)**
    - [ ] Create `src/layers/`.
    - [ ] Move `src/modules/stack/transport/` -> `src/layers/l4/`.
    - [ ] Rename `src/layers/l4/legacy/` -> `src/layers/l4/compat/`.
    - [ ] Fix imports in `src/layers/l4/`.

- [ ] **3.2: Layer 4+ (Carrier)**
    - [ ] Move `src/modules/stack/carrier/` -> `src/layers/l4p/`.
    - [ ] Fix imports in `src/layers/l4p/`.

- [ ] **3.3: Layer 7 (Application)**
    - [ ] Move `src/modules/stack/application/` -> `src/layers/l7/`.
    - [ ] Fix imports in `src/layers/l7/`.

## Phase 4: Ingress & Plugins
*Goal: Organize the "Entry" (Ingress) and the "Extensions" (Plugins).*

- [ ] **4.1: Ingress (formerly Ports)**
    - [ ] Move `src/modules/ports/` -> `src/ingress/`.
    - [ ] Fix imports for `crate::modules::ports`.

- [ ] **4.2: Ingress Task Split (Refactor)**
    - [ ] Create `src/ingress/tcp.rs` and `src/ingress/udp.rs`.
    - [ ] Extract logic from `src/ingress/tasks.rs` into these two files.
    - [ ] Keep shared logic in `src/ingress/lib.rs` (or similar).

- [ ] **4.3: Plugins - Group L4**
    - [ ] Create `src/plugins/l4/`.
    - [ ] Move `src/modules/plugins/terminators/transport/proxy/` -> `src/plugins/l4/proxy/`.
    - [ ] Move `src/modules/plugins/terminators/transport/abort.rs` -> `src/plugins/l4/abort.rs`.

- [ ] **4.4: Plugins - Group L7**
    - [ ] Create `src/plugins/l7/`.
    - [ ] Move `src/modules/plugins/l7/resource/` -> `src/plugins/l7/static_files/`.
    - [ ] Move `src/modules/plugins/l7/cgi/` -> `src/plugins/l7/cgi/`.
    - [ ] Move `src/modules/plugins/l7/upstream/` -> `src/plugins/l7/upstream/`.
    - [ ] Move `src/modules/plugins/terminators/response/` -> `src/plugins/l7/response/`.

- [ ] **4.5: Plugins - Group Protocol & System**
    - [ ] Create `src/plugins/protocol/` and move TLS/QUIC/Detect plugins there.
    - [ ] Create `src/plugins/system/` and move `exec.rs` (rename to `command.rs`) and `unix.rs`.
    - [ ] **Cleanup:** Delete the now empty `src/modules/plugins/` subdirectories.

## Phase 5: Server & API
*Goal: Separate startup from runtime management.*

- [ ] **5.1: API Extraction**
    - [ ] Create `src/api/`.
    - [ ] Move `src/core/console.rs` -> `src/api/server.rs`.
    - [ ] Move `src/core/router.rs` -> `src/api/router.rs`.
    - [ ] Move `src/core/root.rs` -> `src/api/handlers/root.rs`.
    - [ ] Move `src/core/response.rs` -> `src/api/response.rs`.
    - [ ] Move `src/middleware/` content -> `src/api/middleware/`.

- [ ] **5.2: Bootstrap Renaming**
    - [ ] Rename directory `src/core/` -> `src/bootstrap/`.
    - [ ] Update `src/main.rs` to use `crate::bootstrap`.

## Phase 6: Final Sweep
*Goal: Verify everything is clean.*

- [ ] **6.1: Delete Old Directories**
    - [ ] Verify `src/modules/` is empty and delete it.
    - [ ] Verify `src/middleware/` is empty and delete it.

- [ ] **6.2: Final Check**
    - [ ] Run `cargo check`.
    - [ ] Run `cargo test` (if applicable/safe).
    - [ ] Update `lib.rs` or `main.rs` module declarations.
