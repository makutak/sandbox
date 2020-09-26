// 同じパッケージ内のモジュールで、そのモジュールを使うときは
// crate を使う
// crate を使うと絶対パスになる

// 自身の中にあるので self を使える
use self::module_c::func_c;
use crate::module_b;
use crate::module_b::module_c;
