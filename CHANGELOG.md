# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2026-10-20

### Changed
- アプリケーションおよび各種監視状態（CPU、ネットワーク、ディスク、ユーザー入力、停止中、特定プロセス、マルチメディア）のタスクトレイアイコンのデザインを一新。

### Fixed
- Windows 向けクロスコンパイル環境等で exe にアプリケーションアイコンが正しく埋め込まれない問題を修正（ビルド時に `default.ico` から Windows リソースバイナリを自動生成してリンカーに渡す機構を追加）。

## [0.1.2] - 2026-10-16

### Fixed
- Explorer（タスクバー）再起動時にタスクトレイのアイコンが消えてしまう不具合を修正（`TaskbarCreated` メッセージの受信時にアイコンを自動再登録）。

## [0.1.1] - 2026-07-18

### Added
- アイコンのソース形式として PNG ファイル（`Default.png`, `CPU.png` など）の直接読み込みサポートを追加。
- ビルド時に PNG 画像からアプリケーション用の Windows アイコンファイル（`default.ico`）を自動生成するロジックを追加。

### Changed
- ライトモードとダークモードのアイコン個別対応を廃止し、すべてのモードで同一のアイコンセットを使用するように統一。

### Removed
- 古い `.ico` 形式のアイコンファイルを削除。
- 未使用のダークモード反転画像（`_dark`）の生成ロジック、および関連コードを削除。

## [0.1.0] - 2026-07-18

### Added
- 初期リリース。

[0.1.3]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cwatanab/sleeptool-rs/releases/tag/v0.1.0
