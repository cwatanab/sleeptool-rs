# SleepTool Rust (sleeptool)

Windows版 SleepTool を Rust + 純粋な Win32 API で極限まで軽量・高速に再実装した常駐ツールです。
`winit` や `tray-icon` などの重い GUI フレームワークや `tokio` 非同期ランタイムを一切使用せず、Windows のネイティブシステムコールとメッセージループのみで動作します。

## ✨ 特徴と最適化ポイント

- 📦 **極小のバイナリサイズ**: リリースバイナリはわずか **約498 KB**
- 🧠 **超軽量なメモリフットプリント**: 実行時の物理メモリ使用量は **3MB 〜 4MB** 程度で安定稼働
- ⚡ **徹底的な低CPU負荷**: 
  - Windows PDH クエリのキャッシュ化（同一更新フレーム内でのクエリ重複排除）
  - プロセス名マッチング時の無駄なヒープアロケーションを99%削減
- 🎵 **遅延ゼロ・ブロックなしのサウンド監視**:
  - ループバック録音の代わりに `IAudioMeterInformation` API を使用して、再生音のピークボリュームをミリ秒以下の遅延かつアロケーションフリーで検知
- 🔒 **リソースリーク対策**: プロセスハンドルやプリンタハンドルのクローズ処理を徹底し、長時間の連続稼働に対する高い安定性を確保

## 🛠️ 主な機能

- **タスクトレイ常駐**: 右クリックによる全設定メニューへのアクセス
- **スリープ移行時間の設定**: 1分〜180分の間で変更可能
- **各種センサーによるスリープ抑止**:
  - **CPU使用率**: 指定閾値以上のCPUアクティビティを検知
  - **ディスク書き込み量**: ディスク書き込み速度の検知
  - **ネットワーク使用量**: 通信速度の検知
  - **プロセス実行**: 指定されたプロセス名（複数可）が起動しているかを検知
  - **サウンド出力**: システムから音声が出力されているかを検知
- **ログイン時自動起動**: レジストリへの登録による自動起動設定に対応
- **スリープ復帰時画面オフ**: スリープ復帰後に自動的に画面をオフにする機能
- **動的トレイアイコン表示**: 
  - コーヒーカップをモチーフにしたトレイアイコン
  - 「アイコンによる状態表示」を有効にすると、スリープを抑止している原因（CPU、ネットワーク、ディスク等）に応じてトレイアイコンがリアルタイムに変化

## 🔧 ビルド方法

ビルドには Rust ツールチェーンが必要です。

### 1. Windows 環境（MSVC ネイティブ）

Visual Studio C++ Build Tools および Rust ツールチェーン（`x86_64-pc-windows-msvc`）が必要です。

```powershell
# ターゲット追加（未追加の場合）
rustup target add x86_64-pc-windows-msvc

# リリース用バイナリのビルド
cargo build --release --target x86_64-pc-windows-msvc
# （デフォルトターゲットが MSVC の場合は cargo build --release で可）
```

### 2. Linux / macOS 環境からのクロスコンパイル（MSVC）

Linux などの非 Windows 環境から MSVC ターゲット向けにビルドする場合は、[`cargo-xwin`](https://github.com/rust-cross/cargo-xwin) を使用します。必要な Windows SDK や CRT を自動的に取得してリンクします。

```bash
# cargo-xwin のインストールとターゲット追加
cargo install cargo-xwin
rustup target add x86_64-pc-windows-msvc

# MSVC リリースバイナリのビルド
cargo xwin build --target x86_64-pc-windows-msvc --release
```

ビルド完了後、バイナリは `target/x86_64-pc-windows-msvc/release/sleeptool.exe` に出力されます。

## 🚀 実行方法

```powershell
.\target\x86_64-pc-windows-msvc\release\sleeptool.exe
```

- **コマンドライン引数**:
  - `-H` / `--hibernate`: スリープの代わりに休止状態（ハイバネーション）に移行します。
  - `-L` / `--legacy-input`: 入力監視に `GetLastInputInfo` ではなく、キーボードおよびマウス入力をポーリング検知するレガシーモードを使用します。

## 📂 設定ファイル

設定は以下のパスに自動的に作成・保存されます。

- **設定ファイル**: `%LOCALAPPDATA%\SleepTool\config.toml`
- **ログファイル**: `%LOCALAPPDATA%\SleepTool\logs\`
