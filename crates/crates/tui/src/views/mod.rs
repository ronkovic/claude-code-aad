//! ビュー（スタブ）

/// ビュー種別
pub enum View {
    /// ダッシュボード
    Dashboard,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_enum() {
        let _view = View::Dashboard;
    }
}
