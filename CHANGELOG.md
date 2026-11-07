# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] - 2026-11-07

### Changed
- リソース監視 HUD（ウィジェット）のビジュアルを Windows 11 Fluent Design / Modern SaaS スタイルに刷新。
  - OS レベルの角丸ウィンドウクリッピング（`SetWindowRgn`）を適用し、Layered Window 特有の四隅の黒三角アーティファクトを解消。
  - 外枠スリムボーダーとインナーハイライトによる微細なデュアルフレーム構造を導入。
  - スパークライン折れ線下に軽量なエリア塗りつぶしを追加し、枠内への角丸クリッピング（`SelectClipRgn`）を適用。
  - スリープ抑制しきい値の点線ガイドラインと最新値インジケータードット（3×3）を追加。
  - 各メトリクスに洗練されたカラー（CPU: アンバー、NET: エメラルド、DISK: スカイシアン）を割り当て、しきい値超過アラート時はコーラルレッドへ統一切り替え。
  - Meiryo UI から現代的な Segoe UI（Semibold、ClearType 有効）へ刷新し、数値の視認性と等幅アライメントを向上。
- タスクトレイのツールチップ表示プレフィックスを "SleepTool v..." に調整。

## [0.1.5] - 2026-11-07

### Added
- README にアプリケーションアイコンを追加。

### Changed
- アプリケーションおよび各種監視状態（CPU、ネットワーク、ディスク、ユーザー入力、停止中、特定プロセス、サウンド）のアイコンを、自然な月光色（ゴールデンイエロー）と高コントラストな状態バッジを組み合わせたデザインに刷新。
- リソース監視 HUD のスパークライン折れ線を 1px のシャープな細線に調整し、グラフエリアの視認性を向上。

## [0.1.4] - 2026-11-05

### Added
- CPU、ネットワーク、ディスク書き込みの履歴を表示するリソース監視 HUD を追加。しきい値以上ではアイコンの色を変更。

### Changed
- トレイメニューを Windows のアプリテーマに対応。
- 音声監視でアクティブな再生デバイスとオーディオセッションを確認し、再生終了後の猶予時間を追加。
- アプリと監視状態のアイコンを更新。
- 設定とログの保存先を `%LOCALAPPDATA%\\SleepTool` に変更。

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

[0.1.6]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/cwatanab/sleeptool-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cwatanab/sleeptool-rs/releases/tag/v0.1.0
