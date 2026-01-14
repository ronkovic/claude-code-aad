# AAD命名規則

AAD（自律型AI駆動開発）で使用するファイル、ブランチ、Issueの命名規則。

---

## 動的生成ファイル

### SPECファイル（仕様書）

| 項目 | 値 |
|------|-----|
| パターン | `SPEC-XXX.md` |
| 保存場所 | `.aad/specs/` |
| 連番形式 | 3桁ゼロ埋め（001, 002, ...） |
| 開始番号 | 001 |
| 生成元 | 手動作成（テンプレートからコピー） |

**例**: `SPEC-001.md`, `SPEC-010.md`, `SPEC-100.md`

### TASKファイル（タスク定義）

| 項目 | 値 |
|------|-----|
| パターン | `SPEC-XXX-TYY.md` |
| 保存場所 | `.aad/tasks/SPEC-XXX/` |
| SPEC連番 | 3桁ゼロ埋め |
| TASK連番 | 2桁ゼロ埋め（T01, T02, ...） |
| 開始番号 | T01 |
| 生成元 | `/aad:tasks SPEC-XXX` |

**例**: `SPEC-001-T01.md`, `SPEC-001-T10.md`

### RETROファイル（振り返り）

| 項目 | 値 |
|------|-----|
| パターン | `RETRO-SPEC-XXX-YYYYMMDD.md` |
| 保存場所 | `.aad/retrospectives/` |
| 日付形式 | YYYYMMDD（ハイフンなし） |
| 生成元 | `/aad:retro SPEC-XXX` |

**例**: `RETRO-SPEC-001-20260114.md`

---

## 固定名ファイル

| ファイル | 場所 | 説明 | 生成元 |
|---------|------|------|--------|
| `HANDOFF.md` | プロジェクトルート | セッション引き継ぎ | `/aad:handoff` |
| `CLAUDE.md` | プロジェクトルート | プロジェクト指示書 | `/aad:init` |

---

## Git関連

### ブランチ名

| 項目 | 値 |
|------|-----|
| パターン | `feature/SPEC-XXX-TYY` |
| 形式 | 小文字、ハイフン区切り |

**例**: `feature/SPEC-001-T01`

### worktreeディレクトリ

| 項目 | 値 |
|------|-----|
| パターン | `../[project-name]-TYY/` |
| 配置場所 | 親ディレクトリ |

**例**: `../my-project-T01/`

---

## GitHub連携（オプション）

### Issue

| 項目 | パターン |
|------|---------|
| タイトル | `[SPEC-XXX-TYY] タスク名` |

### ラベル

| ラベル | 用途 |
|--------|------|
| `spec-xxx` | SPEC識別（例: `spec-001`） |
| `priority-must` | Must優先度 |
| `priority-should` | Should優先度 |
| `priority-could` | Could優先度 |
| `size-s` | 小規模タスク（1-4時間） |
| `size-m` | 中規模タスク（4-8時間） |
| `size-l` | 大規模タスク（8時間以上） |

---

## 連番ルール

1. **開始値**: SPECは001、TASKはT01から開始
2. **連続性**: 連番は飛ばさない（削除されたファイルの番号は再利用しない）
3. **大文字小文字**:
   - 大文字: `SPEC`, `RETRO`, `HANDOFF`, `CLAUDE`
   - 小文字: `feature/`
4. **自動採番**: 既存の最大番号 + 1 を使用

---

## ディレクトリ構造

```
.
├── CLAUDE.md                    # プロジェクト指示書（固定名）
├── HANDOFF.md                   # セッション引き継ぎ（固定名）
├── .aad/
│   ├── specs/                   # SPEC-XXX.md
│   ├── tasks/                   # SPEC-XXX/TYY.md
│   │   └── SPEC-001/
│   │       ├── SPEC-001-T01.md
│   │       └── SPEC-001-T02.md
│   ├── retrospectives/          # RETRO-SPEC-XXX-YYYYMMDD.md
│   ├── templates/               # テンプレートファイル
│   └── container/               # Docker環境
└── .claude/
    └── commands/aad/            # スラッシュコマンド
```

---

**作成日**: 2026-01-14
