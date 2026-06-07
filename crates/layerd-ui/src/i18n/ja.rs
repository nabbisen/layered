//! Japanese catalog (RFC-043).
//!
//! Entries MUST stay sorted by key and unique, and every key MUST also exist
//! in the English catalog; `i18n_tests` enforces all three.

pub(super) static CATALOG: &[(&str, &str)] = &[
    ("app.title", "layerd"),
    ("breadcrumb.root", "ドキュメント"),
    (
        "dialog.discard.body",
        "保存されていない変更を破棄しますか？",
    ),
    ("dialog.discard.cancel", "キャンセル"),
    ("dialog.discard.confirm", "破棄"),
    ("dialog.discard.title", "未保存の変更"),
    ("editor.body.placeholder", "このセクションの本文を入力…"),
    ("editor.preview", "プレビュー"),
    ("editor.source", "ソース"),
    ("error.open_failed", "ファイルを開けませんでした。"),
    ("error.save_failed", "ファイルを保存できませんでした。"),
    (
        "error.stale_edit",
        "ドキュメントが変更されたため、編集は適用されませんでした。",
    ),
    ("focus.children", "サブセクション"),
    (
        "focus.empty_body",
        "このセクションにはまだ本文がありません。",
    ),
    ("menu.file", "ファイル"),
    ("menu.file.new", "新規作成"),
    ("menu.file.open", "開く…"),
    ("menu.file.save", "保存"),
    ("menu.file.save_as", "名前を付けて保存…"),
    ("menu.help", "ヘルプ"),
    ("menu.help.about", "layerd について"),
    ("menu.language", "言語"),
    ("nav.back", "戻る"),
    ("nav.forward", "進む"),
    ("nav.up", "上の階層へ"),
    (
        "outline.empty",
        "セクションはまだありません — 文書全体がひとつの層です。",
    ),
    ("outline.title", "アウトライン"),
    ("status.ready", "準備完了"),
    ("status.saved", "保存しました"),
    ("status.unsaved", "未保存の変更"),
    ("toolbar.edit", "編集"),
    ("toolbar.redo", "やり直し"),
    ("toolbar.undo", "元に戻す"),
];
