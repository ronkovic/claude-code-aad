# タスク: [SPEC-XXX-TXX] - [タスク名]

> **📝 このテンプレートについて:**
> タスクは通常、`/aad:tasks`コマンドで自動生成されます。このテンプレートは参照用です。

**SPEC**: [SPEC-XXX](../specs/SPEC-XXX.md)

**タスクID**: SPEC-XXX-TXX

**複雑度**: S / M / L / XL

**優先度**: Must / Should / Could / Won't

**GitHub Issue**: #XX（オプション）

**担当**: [名前 または AI Worker]

**ステータス**: 未着手 / 進行中 / レビュー中 / 完了

---

## 📋 タスク概要

[このタスクで実装する内容を簡潔に記述]

---

## 🎯 実装目標

### 成果物

- [ ] [成果物1]
- [ ] [成果物2]
- [ ] [成果物3]

### 受け入れ基準

1. **機能要件**
   - [ ] [条件1]
   - [ ] [条件2]

2. **品質要件**
   - [ ] テストカバレッジ80%以上
   - [ ] Lint通過
   - [ ] 全テストgreen

3. **ドキュメント**
   - [ ] コード内コメント
   - [ ] 必要に応じてREADME更新

---

## 📐 設計方針

### アプローチ

[実装のアプローチを記述]

### 影響範囲

**変更ファイル**:
- `path/to/file1.ts`
- `path/to/file2.ts`

**依存タスク**:
- [SPEC-XXX-TYY] - [タスク名]

**ブロック対象**:
- [SPEC-XXX-TZZ] - [タスク名]

---

## ✅ 実装チェックリスト

### 1. TDD (Test-Driven Development)

- [ ] テストケース作成
- [ ] Red: テスト失敗確認
- [ ] Green: 実装してテスト通過
- [ ] Refactor: リファクタリング

### 2. 品質ゲート

- [ ] `/aad:gate TDD` 実行
- [ ] カバレッジ80%以上
- [ ] Lint通過
- [ ] 全テストgreen

### 3. PR作成

- [ ] `gh pr create --draft` でDraft PR作成
- [ ] PR説明に実装内容を記載
- [ ] 関連Issueをリンク

### 4. レビュー

- [ ] AI自己レビュー実施
- [ ] CI green確認
- [ ] 人間レビュー依頼

---

## 📝 実装メモ

### 技術的な検討事項

[実装時の技術的な検討事項や注意点]

### 参考資料

- [リンク1]
- [リンク2]

---

## 🔗 関連リンク

- **SPEC**: [SPEC-XXX](../specs/SPEC-XXX.md)
- **GitHub Issue**: [#XX](https://github.com/your-org/your-repo/issues/XX)（オプション）
- **PR**: [#YY](https://github.com/your-org/your-repo/pull/YY)
- **Worktree**: `../[project-name]-TXX/`
- **Branch**: `feature/SPEC-XXX-TXX`

---

**作成日**: YYYY-MM-DD

**更新日**: YYYY-MM-DD
