# Tech Steering: claude-code-aad v2

## Language & Runtime

- **Rust Edition**: 2021
- **Async Runtime**: tokio
- **Minimum Rust Version**: 1.70+

## Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| ratatui | 0.28 | TUI framework |
| tokio | 1.x | Async runtime |
| clap | 4.x | CLI parser |
| git2 | 0.19 | Git operations |
| serde | 1.x | Serialization |
| handlebars | 5.x | Templates |
| toml | 0.8 | Config parsing |
| anyhow | 1.x | Error handling |
| thiserror | 1.x | Custom errors |
| async-trait | 0.1 | Async traits |

## Build Commands

```bash
# ビルド
cargo build --all

# テスト
cargo test --all

# Lint
cargo clippy --all -- -D warnings

# フォーマット
cargo fmt --all

# カバレッジ
cargo llvm-cov --html
```

## Conventions

### Naming Rules

- **Files**: `snake_case.rs`
- **Types**: `PascalCase`
- **Functions**: `snake_case`
- **Constants**: `UPPER_SNAKE_CASE`
- **Modules**: `snake_case`

### Code Style

- **インデント**: 4スペース
- **最大行長**: 100文字
- **rustfmt**: 標準設定準拠

### Error Handling

```rust
// Application層: anyhow
use anyhow::{Context, Result};

pub fn execute(&self) -> Result<()> {
    self.validate().context("Validation failed")?;
    Ok(())
}

// Domain/Infrastructure層: thiserror
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Invalid spec ID: {0}")]
    InvalidSpecId(String),
}
```

### Async Guidelines

- **全てのI/O操作**: async/await
- **並列処理**: `tokio::spawn`
- **タイムアウト**: `tokio::time::timeout`

```rust
use tokio::time::{timeout, Duration};

async fn call_api(&self) -> Result<Response> {
    timeout(Duration::from_secs(30), self.client.get())
        .await?
        .context("API timeout")
}
```

### Testing Standards

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ユニットテスト: 各クレート内
    #[test]
    fn test_entity_creation() { }

    // 統合テスト: tests/ ディレクトリ
    #[tokio::test]
    async fn test_use_case_flow() { }
}
```

### Documentation

- **パブリックAPI**: `///` ドキュメントコメント必須
- **複雑なロジック**: `//` インラインコメント
- **TODO**: `// TODO(@username #issue): 説明`

```rust
/// ワークフローセッションを表すドメインエンティティ
///
/// # Example
/// ```
/// let session = Session::new(spec_id, Phase::Spec);
/// ```
pub struct Session { }
```

## Dependency Injection

```rust
// ポート定義（Application層）
#[async_trait]
pub trait GitPort {
    async fn create_worktree(&self, branch: &str) -> Result<()>;
}

// アダプター実装（Infrastructure層）
pub struct Git2Adapter { }

#[async_trait]
impl GitPort for Git2Adapter {
    async fn create_worktree(&self, branch: &str) -> Result<()> { }
}

// DIコンテナ（CLI/TUI層）
let git_port: Arc<dyn GitPort> = Arc::new(Git2Adapter::new());
let use_case = CreateWorktreeUseCase::new(git_port);
```

## Performance Guidelines

- **ヒープアロケーション最小化**: `&str` over `String`
- **クローン回避**: `Arc` for shared ownership
- **並列処理**: CPU bound tasks use `tokio::task::spawn_blocking`

## Security Considerations

- **API Key管理**: 環境変数のみ、コードに埋め込まない
- **入力検証**: 全てのユーザー入力をバリデート
- **依存関係**: `cargo audit` で定期チェック
