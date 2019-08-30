/// 標準入力の簡易入力関数モジュール
pub mod input;
pub use input::*;

/// 任意の素数を法とする変数
pub mod modint;
pub use modint::*;

/// 素数関係のライブラリ
pub mod prime_number;
pub use prime_number::*;

///グラフ関係のライブラリ
pub mod graph;
pub use graph::*;

///数学関係のライブラリ
pub mod math;
pub use math::*;

///配列に関する補助関数ライブラリ
pub mod vectools;
pub use vectools::*;

/// Ordを実装したf64ラッパー
pub mod ordfloat;
pub use ordfloat::*;
