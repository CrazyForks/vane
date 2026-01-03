# Detailed Refactoring Plan: Vane 2.0 (Granular)

This plan breaks down the structural overhaul into atomic, verifiable steps.
**Golden Rule:** Run `cargo check` after *every single checkbox* to catch errors immediately.

# Detailed Refactoring Plan: Vane 2.0 (Granular)

This plan breaks down the structural overhaul into atomic, verifiable steps.
**Golden Rule:** Run `cargo check` after *every single checkbox* to catch errors immediately.

## Phase 1: Foundation (Common & Resources)
*Goal: Stabilize leaf dependencies first.*

- [x] **1.1: Restructure `src/common/` (Config)**
    - [x] **1.1.1: Setup Config Module**
        - [x] Create `src/common/config/` directory.
        - [x] Create `src/common/config/mod.rs`.
        - [x] Add `pub mod config;` to `src/common/mod.rs`.
    - [x] **1.1.2: Migrate `getconf.rs`**
        - [x] Move `src/common/getconf.rs` -> `src/common/config/getconf.rs`.
        - [x] Fix imports.
        - [x] `cargo check`.
    - [x] **1.1.3: Migrate `getenv.rs`**
        - [x] Move `src/common/getenv.rs` -> `src/common/config/getenv.rs`.
        - [x] Fix imports.
        - [x] `cargo check`.
    - [x] **1.1.4: Migrate `loader.rs`**
        - [x] Move `src/common/loader.rs` -> `src/common/config/loader.rs`.
        - [x] Fix imports.
        - [x] `cargo check`.

- [ ] **1.2: Restructure `src/common/` (Net & Sys)**
    - [x] **1.2.1: Setup Net & Sys Modules**
        - [x] Create `src/common/net/` and `src/common/sys/`.
        - [x] Create `src/common/net/mod.rs` and `src/common/sys/mod.rs`.
        - [x] Update `src/common/mod.rs` (add `net`, `sys`).
        - [x] `cargo check`.
    - [x] **1.2.2: Migrate `ip.rs` (Net)**
        - [x] Move `src/common/ip.rs` -> `src/common/net/ip.rs`.
        - [x] Update mods.
        - [x] Search & Replace `crate::common::ip` -> `crate::common::net::ip`.
        - [x] `cargo check`.
    - [x] **1.2.3: Migrate `portool.rs` (Net)**
        - [x] Move `src/common/portool.rs` -> `src/common/net/portool.rs`.
        - [x] Update mods.
        - [x] Search & Replace `crate::common::portool` -> `crate::common::net::portool`.
        - [x] `cargo check`.
    - [x] **1.2.4: Migrate `lifecycle.rs` (Sys)**
        - [x] Move `src/common/lifecycle.rs` -> `src/common/sys/lifecycle.rs`.
        - [x] Update mods.
        - [x] Search & Replace `crate::common::lifecycle` -> `crate::common::sys::lifecycle`.
        - [x] `cargo check`.
    - [x] **1.2.5: Migrate `system.rs` (Sys)**
        - [x] Move `src/common/system.rs` -> `src/common/sys/system.rs`.
        - [x] Update mods.
        - [x] Search & Replace `crate::common::system` -> `crate::common::sys::system`.
        - [x] `cargo check`.
    - [x] **1.2.6: Migrate `watcher.rs` (Sys)**
        - [x] Move `src/common/watcher.rs` -> `src/common/sys/watcher.rs`.
        - [x] Update mods.
        - [x] Search & Replace `crate::common::watcher` -> `crate::common::sys::watcher`.
        - [x] `cargo check`.
    - [x] **1.2.7: Migrate `hotswap.rs` (Sys)**
        - [x] Move `src/common/hotswap.rs` -> `src/common/sys/hotswap.rs`.
        - [x] Update mods.
        - [x] Search & Replace `crate::common::hotswap` -> `crate::common::sys::hotswap`.
        - [x] `cargo check`.

- [ ] **1.3: Establish `src/resources/kv`**
    - [x] **1.3.1: Setup Resources Module**
        - [x] Create `src/resources/` directory.
        - [x] Create `src/resources/mod.rs`.
        - [x] Add `pub mod resources;` to `src/lib.rs` (or main.rs/mod structure).
        - [x] `cargo check`.
    - [x] **1.3.2: Move KV Module**
        - [x] Move `src/modules/kv/` -> `src/resources/kv/`.
        - [x] Add `pub mod kv;` to `src/resources/mod.rs`.
        - [x] Remove `kv` from `src/modules/mod.rs`.
        - [x] Search & Replace `crate::modules::kv` -> `crate::resources::kv`.
        - [x] `cargo check`.

- [ ] **1.4: Establish `src/resources/certs`**
    - [x] **1.4.1: Move Certs Module**
        - [x] Move `src/modules/certs/` -> `src/resources/certs/`.
        - [x] Add `pub mod certs;` to `src/resources/mod.rs`.
        - [x] Remove `certs` from `src/modules/mod.rs`.
        - [x] Search & Replace `crate::modules::certs` -> `crate::resources::certs`.
        - [x] `cargo check`.

- [ ] **1.5: Establish `src/resources/service_discovery`**
    - [ ] **1.5.1: Move Nodes Module**
        - [ ] Move `src/modules/nodes/` -> `src/resources/service_discovery/`.
        - [ ] Add `pub mod service_discovery;` to `src/resources/mod.rs`.
        - [ ] Remove `nodes` from `src/modules/mod.rs`.
        - [ ] Search & Replace `crate::modules::nodes` -> `crate::resources::service_discovery`.
        - [ ] `cargo check`.

- [ ] **1.6: Establish `src/resources/templates`**
    - [ ] **1.6.1: Move Template Module**
        - [ ] Move `src/modules/template/` -> `src/resources/templates/`.
        - [ ] Add `pub mod templates;` to `src/resources/mod.rs`.
        - [ ] Remove `template` from `src/modules/mod.rs`.
        - [ ] Search & Replace `crate::modules::template` -> `crate::resources::templates`.
        - [ ] `cargo check`.

## Phase 2: The Engine Core
*Goal: Centralize the "Contract" and "Executor".*

- [ ] **2.1: Extract Traits (Contract)**
    - [ ] **2.1.1: Setup Engine Module**
        - [ ] Create `src/engine/` directory.
        - [ ] Create `src/engine/mod.rs`.
        - [ ] Add `pub mod engine;` to `src/lib.rs`.
        - [ ] `cargo check`.
    - [ ] **2.1.2: Move Model/Contract**
        - [ ] Move `src/modules/plugins/core/model.rs` -> `src/engine/contract.rs`.
        - [ ] Expose `contract` in `src/engine/mod.rs`.
        - [ ] Remove `model` from `src/modules/plugins/core/mod.rs`.
        - [ ] `cargo check` (expect errors).
    - [ ] **2.1.3: Fix Contract Imports**
        - [ ] Search & Replace `crate::modules::plugins::core::model` -> `crate::engine::contract`.
        - [ ] Fix internal imports in `contract.rs`.
        - [ ] `cargo check`.

- [ ] **2.2: Move Flow Logic**
    - [ ] **2.2.1: Move Context**
        - [ ] Move `src/modules/flow/context.rs` -> `src/engine/context.rs`.
        - [ ] Search & Replace `crate::modules::flow::context` -> `crate::engine::context`.
        - [ ] `cargo check`.
    - [ ] **2.2.2: Move Key Scoping**
        - [ ] Move `src/modules/flow/key_scoping.rs` -> `src/engine/key_scoping.rs`.
        - [ ] Search & Replace `crate::modules::flow::key_scoping` -> `crate::engine::key_scoping`.
        - [ ] `cargo check`.
    - [ ] **2.2.3: Move Executor (Engine)**
        - [ ] Move `src/modules/flow/engine.rs` -> `src/engine/executor.rs`.
        - [ ] Search & Replace `crate::modules::flow::engine` -> `crate::engine::executor`.
        - [ ] `cargo check`.
    - [ ] **2.2.4: Cleanup Flow Module**
        - [ ] Remove `src/modules/flow`.
        - [ ] Remove `flow` from `src/modules/mod.rs`.
        - [ ] `cargo check`.

## Phase 3: The Protocol Stack (Layers)
*Goal: Flatten the network stack.*

- [ ] **3.1: Layer 4 (Transport)**
    - [ ] **3.1.1: Setup Layers**
        - [ ] Create `src/layers/` and `src/layers/mod.rs`.
        - [ ] Add `pub mod layers;` to `src/lib.rs`.
        - [ ] `cargo check`.
    - [ ] **3.1.2: Move Transport**
        - [ ] Move `src/modules/stack/transport/` -> `src/layers/l4/`.
        - [ ] Add `l4` to `src/layers/mod.rs`.
        - [ ] Search & Replace `crate::modules::stack::transport` -> `crate::layers::l4`.
        - [ ] `cargo check`.

- [ ] **3.2: Layer 4+ (Carrier)**
    - [ ] **3.2.1: Move Carrier**
        - [ ] Move `src/modules/stack/carrier/` -> `src/layers/l4p/`.
        - [ ] Add `l4p` to `src/layers/mod.rs`.
        - [ ] Search & Replace `crate::modules::stack::carrier` -> `crate::layers::l4p`.
        - [ ] `cargo check`.

- [ ] **3.3: Layer 7 (Application)**
    - [ ] **3.3.1: Move Application**
        - [ ] Move `src/modules/stack/application/` -> `src/layers/l7/`.
        - [ ] Add `l7` to `src/layers/mod.rs`.
        - [ ] Search & Replace `crate::modules::stack::application` -> `crate::layers::l7`.
        - [ ] `cargo check`.

- [ ] **3.4: Cleanup Stack**
    - [ ] Remove `src/modules/stack`.
    - [ ] Remove `stack` from `src/modules/mod.rs`.
    - [ ] `cargo check`.

## Phase 4: Ingress & Plugins
*Goal: Organize Entry and Extensions.*

- [ ] **4.1: Ingress**
    - [ ] **4.1.1: Move Ports to Ingress**
        - [ ] Create `src/ingress/`.
        - [ ] Move `src/modules/ports/` contents -> `src/ingress/`.
        - [ ] Add `pub mod ingress;` to `src/lib.rs`.
        - [ ] Search & Replace `crate::modules::ports` -> `crate::ingress`.
        - [ ] `cargo check`.
    - [ ] **4.1.2: Refactor Tasks (Split TCP/UDP)**
        - [ ] Extract `src/ingress/tcp.rs` from `tasks.rs`.
        - [ ] Extract `src/ingress/udp.rs` from `tasks.rs`.
        - [ ] Update `ingress/mod.rs`.
        - [ ] `cargo check`.

- [ ] **4.2: Plugins Organization**
    - [ ] **4.2.1: Setup Plugins Dirs**
        - [ ] Create `src/plugins/l4/`, `l7/`, `protocol/`, `system/`.
        - [ ] Create `src/plugins/mod.rs`.
        - [ ] Add `pub mod plugins;` to `src/lib.rs`.
    - [ ] **4.2.2: Move L4 Plugins**
        - [ ] Move `terminators/transport/proxy` -> `src/plugins/l4/proxy`.
        - [ ] Move `terminators/transport/abort.rs` -> `src/plugins/l4/abort.rs`.
        - [ ] `cargo check`.
    - [ ] **4.2.3: Move L7 Plugins**
        - [ ] Move `l7/resource` -> `src/plugins/l7/static_files`.
        - [ ] Move `l7/cgi` -> `src/plugins/l7/cgi`.
        - [ ] Move `l7/upstream` -> `src/plugins/l7/upstream`.
        - [ ] Move `terminators/response` -> `src/plugins/l7/response`.
        - [ ] `cargo check`.
    - [ ] **4.2.4: Move System/Protocol**
        - [ ] Move TLS/QUIC -> `src/plugins/protocol/`.
        - [ ] Move `exec.rs`, `unix.rs` -> `src/plugins/system/`.
        - [ ] `cargo check`.
    - [ ] **4.2.5: Cleanup Plugins**
        - [ ] Remove `src/modules/plugins`.
        - [ ] Remove `plugins` from `src/modules/mod.rs`.
        - [ ] `cargo check`.

## Phase 5: Server & API
*Goal: Separate startup from runtime.*

- [ ] **5.1: API**
    - [ ] **5.1.1: Setup API**
        - [ ] Create `src/api/`.
        - [ ] Move `src/core/router.rs` -> `src/api/router.rs`.
        - [ ] Move `src/core/root.rs` -> `src/api/handlers/root.rs`.
        - [ ] Move `src/core/response.rs` -> `src/api/response.rs`.
        - [ ] Move `src/middleware/` -> `src/api/middleware/`.
        - [ ] Fix imports.
        - [ ] `cargo check`.

- [ ] **5.2: Bootstrap**
    - [ ] **5.2.1: Rename Core**
        - [ ] Rename `src/core` -> `src/bootstrap`.
        - [ ] Update imports `crate::core` -> `crate::bootstrap`.
        - [ ] `cargo check`.

## Phase 6: Final Cleanup
- [ ] **6.1: Remove Empty Directories**
    - [ ] Remove `src/modules`.
    - [ ] Remove `src/middleware`.
- [ ] **6.2: Documentation Update**
    - [ ] Update path references in `docs/`.
