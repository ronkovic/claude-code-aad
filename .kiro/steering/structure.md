# Structure Steering: claude-code-aad v2

## Clean Architecture (4 Layers)

```
Presentation → Application → Domain ← Infrastructure
    (CLI/TUI)    (UseCases)   (Core)    (Adapters)
```

### Dependency Rule

**内側へ向かってのみ依存**

- **Domain**: 外部依存なし（純粋なビジネスロジック）
- **Application**: Domain のみに依存
- **Infrastructure**: Domain + Application に依存
- **Presentation (CLI/TUI)**: 全層に依存（DIで組み立て）

## Directory Structure

```
claude-code-aad/
├── Cargo.toml           # ワークスペース定義
├── crates/
│   ├── domain/          # ドメイン層（純粋）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── entities/
│   │       │   ├── mod.rs
│   │       │   ├── spec.rs         # Spec, SpecMetadata
│   │       │   ├── task.rs         # Task, TaskId
│   │       │   ├── session.rs      # Session, SessionId
│   │       │   └── workflow.rs     # Workflow
│   │       ├── value_objects/
│   │       │   ├── mod.rs
│   │       │   ├── spec_id.rs      # SpecId
│   │       │   ├── phase.rs        # Phase enum
│   │       │   ├── status.rs       # Status enum
│   │       │   └── priority.rs     # Priority enum
│   │       ├── events/
│   │       │   ├── mod.rs
│   │       │   └── domain_events.rs # PhaseCompleted, TaskStarted
│   │       └── repositories/
│   │           ├── mod.rs
│   │           ├── spec_repository.rs    # trait
│   │           ├── session_repository.rs # trait
│   │           └── workflow_repository.rs # trait
│   │
│   ├── application/     # アプリケーション層
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── use_cases/
│   │       │   ├── mod.rs
│   │       │   ├── init_project.rs
│   │       │   ├── create_spec.rs
│   │       │   ├── split_tasks.rs
│   │       │   ├── create_worktree.rs
│   │       │   ├── orchestrate.rs
│   │       │   ├── run_gate.rs
│   │       │   └── integrate_pr.rs
│   │       ├── services/
│   │       │   ├── mod.rs
│   │       │   ├── orchestrator.rs      # Master orchestration
│   │       │   ├── session_manager.rs   # Session lifecycle
│   │       │   └── template_service.rs  # Template rendering
│   │       ├── ports/
│   │       │   ├── mod.rs
│   │       │   ├── git_port.rs          # trait
│   │       │   ├── github_port.rs       # trait
│   │       │   ├── claude_port.rs       # trait
│   │       │   └── file_port.rs         # trait
│   │       └── dto/
│   │           ├── mod.rs
│   │           └── orchestration_request.rs
│   │
│   ├── infrastructure/  # インフラストラクチャ層
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── adapters/
│   │       │   ├── mod.rs
│   │       │   ├── git/
│   │       │   │   ├── mod.rs
│   │       │   │   └── git2_adapter.rs
│   │       │   ├── github/
│   │       │   │   ├── mod.rs
│   │       │   │   └── gh_adapter.rs
│   │       │   ├── claude/
│   │       │   │   ├── mod.rs
│   │       │   │   └── mcp_adapter.rs
│   │       │   └── file/
│   │       │       ├── mod.rs
│   │       │       └── fs_adapter.rs
│   │       ├── persistence/
│   │       │   ├── mod.rs
│   │       │   ├── json_spec_repository.rs
│   │       │   ├── json_session_repository.rs
│   │       │   └── json_workflow_repository.rs
│   │       └── config/
│   │           ├── mod.rs
│   │           ├── aad_config.rs        # .aad/config.toml
│   │           └── template_loader.rs
│   │
│   ├── cli/             # CLIプレゼンテーション層
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── commands/
│   │       │   ├── mod.rs
│   │       │   ├── init.rs
│   │       │   ├── spec.rs
│   │       │   ├── tasks.rs
│   │       │   ├── worktree.rs
│   │       │   ├── orchestrate.rs
│   │       │   ├── gate.rs
│   │       │   ├── integrate.rs
│   │       │   ├── retro.rs
│   │       │   └── status.rs
│   │       └── di/
│   │           ├── mod.rs
│   │           └── container.rs         # DI container
│   │
│   └── tui/             # TUIプレゼンテーション層
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── app.rs              # TUI application state
│           ├── widgets/
│           │   ├── mod.rs
│           │   ├── status_panel.rs
│           │   ├── task_list.rs
│           │   ├── log_viewer.rs
│           │   └── progress_bar.rs
│           └── views/
│               ├── mod.rs
│               ├── dashboard.rs
│               └── detail.rs
│
├── templates/           # Handlebarsテンプレート
│   ├── spec.md.hbs
│   ├── tasks.md.hbs
│   └── retro.md.hbs
│
└── tests/              # 統合テスト
    ├── integration/
    │   ├── orchestration_test.rs
    │   └── workflow_test.rs
    └── fixtures/
        └── sample_project/
```

## Module Organization Rules

### 1. One Responsibility per Module

各モジュールは単一の責務を持つ。

```rust
// ❌ Bad: 複数の責務が混在
mod user {
    struct User { }
    fn send_email() { }  // これはnotificationモジュールへ
}

// ✅ Good: 責務が分離
mod user {
    struct User { }
}
mod notification {
    fn send_email() { }
}
```

### 2. Re-export Pattern

`mod.rs` でモジュールをre-exportし、外部からのアクセスを簡潔にする。

```rust
// entities/mod.rs
mod spec;
mod task;
mod session;

pub use spec::Spec;
pub use task::Task;
pub use session::Session;
```

### 3. Crate API Definition

`lib.rs` でクレートのパブリックAPIを定義。

```rust
// domain/src/lib.rs
pub mod entities;
pub mod value_objects;
pub mod repositories;

// 外部クレートからは以下のように使用
use domain::entities::Spec;
use domain::value_objects::Phase;
```

## Layer Communication Patterns

### Domain → Application

Application層はDomain層のエンティティとバリューオブジェクトを使用。

```rust
// application/src/use_cases/create_spec.rs
use domain::entities::Spec;
use domain::value_objects::SpecId;
use domain::repositories::SpecRepository;

pub struct CreateSpecUseCase<R: SpecRepository> {
    repo: R,
}
```

### Application → Infrastructure

Infrastructure層はApplication層のポートを実装。

```rust
// infrastructure/src/adapters/git/git2_adapter.rs
use application::ports::GitPort;
use async_trait::async_trait;

pub struct Git2Adapter { }

#[async_trait]
impl GitPort for Git2Adapter { }
```

### CLI → All Layers

CLI層はDIコンテナで全てを組み立て。

```rust
// cli/src/di/container.rs
use application::use_cases::CreateSpecUseCase;
use infrastructure::adapters::git::Git2Adapter;

pub fn build_container() -> Container {
    let git_port = Arc::new(Git2Adapter::new());
    let use_case = CreateSpecUseCase::new(git_port);
    // ...
}
```

## Testing Strategy

| テスト種別 | 場所 | 依存関係 |
|-----------|------|----------|
| ユニットテスト | 各クレート内 `#[cfg(test)]` | モック不要（純粋関数） |
| 統合テスト | `tests/` ディレクトリ | 実装またはモック |
| E2Eテスト | `tests/e2e/` | 実環境 |

```rust
// Domain層: ユニットテスト（モック不要）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_creation() {
        let spec = Spec::new("feature-x");
        assert_eq!(spec.name, "feature-x");
    }
}

// Application層: ユニットテスト（モック使用）
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        SpecRepo {}
        impl SpecRepository for SpecRepo { }
    }

    #[tokio::test]
    async fn create_spec_use_case() {
        let mut mock_repo = MockSpecRepo::new();
        // ...
    }
}
```
