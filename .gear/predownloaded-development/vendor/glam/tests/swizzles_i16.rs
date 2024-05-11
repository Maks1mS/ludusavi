// Generated by swizzlegen. Do not edit.
#[macro_use]
mod support;
use glam::*;

glam_test!(test_i16vec4_swizzles, {
    let v = i16vec4(1_i16, 2_i16, 3_i16, 4_i16);
    assert_eq!(v, v.xyzw());
    assert_eq!(v.xxxx(), i16vec4(1_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.xxxy(), i16vec4(1_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.xxxz(), i16vec4(1_i16, 1_i16, 1_i16, 3_i16));
    assert_eq!(v.xxxw(), i16vec4(1_i16, 1_i16, 1_i16, 4_i16));
    assert_eq!(v.xxyx(), i16vec4(1_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.xxyy(), i16vec4(1_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.xxyz(), i16vec4(1_i16, 1_i16, 2_i16, 3_i16));
    assert_eq!(v.xxyw(), i16vec4(1_i16, 1_i16, 2_i16, 4_i16));
    assert_eq!(v.xxzx(), i16vec4(1_i16, 1_i16, 3_i16, 1_i16));
    assert_eq!(v.xxzy(), i16vec4(1_i16, 1_i16, 3_i16, 2_i16));
    assert_eq!(v.xxzz(), i16vec4(1_i16, 1_i16, 3_i16, 3_i16));
    assert_eq!(v.xxzw(), i16vec4(1_i16, 1_i16, 3_i16, 4_i16));
    assert_eq!(v.xxwx(), i16vec4(1_i16, 1_i16, 4_i16, 1_i16));
    assert_eq!(v.xxwy(), i16vec4(1_i16, 1_i16, 4_i16, 2_i16));
    assert_eq!(v.xxwz(), i16vec4(1_i16, 1_i16, 4_i16, 3_i16));
    assert_eq!(v.xxww(), i16vec4(1_i16, 1_i16, 4_i16, 4_i16));
    assert_eq!(v.xyxx(), i16vec4(1_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.xyxy(), i16vec4(1_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.xyxz(), i16vec4(1_i16, 2_i16, 1_i16, 3_i16));
    assert_eq!(v.xyxw(), i16vec4(1_i16, 2_i16, 1_i16, 4_i16));
    assert_eq!(v.xyyx(), i16vec4(1_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.xyyy(), i16vec4(1_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.xyyz(), i16vec4(1_i16, 2_i16, 2_i16, 3_i16));
    assert_eq!(v.xyyw(), i16vec4(1_i16, 2_i16, 2_i16, 4_i16));
    assert_eq!(v.xyzx(), i16vec4(1_i16, 2_i16, 3_i16, 1_i16));
    assert_eq!(v.xyzy(), i16vec4(1_i16, 2_i16, 3_i16, 2_i16));
    assert_eq!(v.xyzz(), i16vec4(1_i16, 2_i16, 3_i16, 3_i16));
    assert_eq!(v.xywx(), i16vec4(1_i16, 2_i16, 4_i16, 1_i16));
    assert_eq!(v.xywy(), i16vec4(1_i16, 2_i16, 4_i16, 2_i16));
    assert_eq!(v.xywz(), i16vec4(1_i16, 2_i16, 4_i16, 3_i16));
    assert_eq!(v.xyww(), i16vec4(1_i16, 2_i16, 4_i16, 4_i16));
    assert_eq!(v.xzxx(), i16vec4(1_i16, 3_i16, 1_i16, 1_i16));
    assert_eq!(v.xzxy(), i16vec4(1_i16, 3_i16, 1_i16, 2_i16));
    assert_eq!(v.xzxz(), i16vec4(1_i16, 3_i16, 1_i16, 3_i16));
    assert_eq!(v.xzxw(), i16vec4(1_i16, 3_i16, 1_i16, 4_i16));
    assert_eq!(v.xzyx(), i16vec4(1_i16, 3_i16, 2_i16, 1_i16));
    assert_eq!(v.xzyy(), i16vec4(1_i16, 3_i16, 2_i16, 2_i16));
    assert_eq!(v.xzyz(), i16vec4(1_i16, 3_i16, 2_i16, 3_i16));
    assert_eq!(v.xzyw(), i16vec4(1_i16, 3_i16, 2_i16, 4_i16));
    assert_eq!(v.xzzx(), i16vec4(1_i16, 3_i16, 3_i16, 1_i16));
    assert_eq!(v.xzzy(), i16vec4(1_i16, 3_i16, 3_i16, 2_i16));
    assert_eq!(v.xzzz(), i16vec4(1_i16, 3_i16, 3_i16, 3_i16));
    assert_eq!(v.xzzw(), i16vec4(1_i16, 3_i16, 3_i16, 4_i16));
    assert_eq!(v.xzwx(), i16vec4(1_i16, 3_i16, 4_i16, 1_i16));
    assert_eq!(v.xzwy(), i16vec4(1_i16, 3_i16, 4_i16, 2_i16));
    assert_eq!(v.xzwz(), i16vec4(1_i16, 3_i16, 4_i16, 3_i16));
    assert_eq!(v.xzww(), i16vec4(1_i16, 3_i16, 4_i16, 4_i16));
    assert_eq!(v.xwxx(), i16vec4(1_i16, 4_i16, 1_i16, 1_i16));
    assert_eq!(v.xwxy(), i16vec4(1_i16, 4_i16, 1_i16, 2_i16));
    assert_eq!(v.xwxz(), i16vec4(1_i16, 4_i16, 1_i16, 3_i16));
    assert_eq!(v.xwxw(), i16vec4(1_i16, 4_i16, 1_i16, 4_i16));
    assert_eq!(v.xwyx(), i16vec4(1_i16, 4_i16, 2_i16, 1_i16));
    assert_eq!(v.xwyy(), i16vec4(1_i16, 4_i16, 2_i16, 2_i16));
    assert_eq!(v.xwyz(), i16vec4(1_i16, 4_i16, 2_i16, 3_i16));
    assert_eq!(v.xwyw(), i16vec4(1_i16, 4_i16, 2_i16, 4_i16));
    assert_eq!(v.xwzx(), i16vec4(1_i16, 4_i16, 3_i16, 1_i16));
    assert_eq!(v.xwzy(), i16vec4(1_i16, 4_i16, 3_i16, 2_i16));
    assert_eq!(v.xwzz(), i16vec4(1_i16, 4_i16, 3_i16, 3_i16));
    assert_eq!(v.xwzw(), i16vec4(1_i16, 4_i16, 3_i16, 4_i16));
    assert_eq!(v.xwwx(), i16vec4(1_i16, 4_i16, 4_i16, 1_i16));
    assert_eq!(v.xwwy(), i16vec4(1_i16, 4_i16, 4_i16, 2_i16));
    assert_eq!(v.xwwz(), i16vec4(1_i16, 4_i16, 4_i16, 3_i16));
    assert_eq!(v.xwww(), i16vec4(1_i16, 4_i16, 4_i16, 4_i16));
    assert_eq!(v.yxxx(), i16vec4(2_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.yxxy(), i16vec4(2_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.yxxz(), i16vec4(2_i16, 1_i16, 1_i16, 3_i16));
    assert_eq!(v.yxxw(), i16vec4(2_i16, 1_i16, 1_i16, 4_i16));
    assert_eq!(v.yxyx(), i16vec4(2_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.yxyy(), i16vec4(2_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.yxyz(), i16vec4(2_i16, 1_i16, 2_i16, 3_i16));
    assert_eq!(v.yxyw(), i16vec4(2_i16, 1_i16, 2_i16, 4_i16));
    assert_eq!(v.yxzx(), i16vec4(2_i16, 1_i16, 3_i16, 1_i16));
    assert_eq!(v.yxzy(), i16vec4(2_i16, 1_i16, 3_i16, 2_i16));
    assert_eq!(v.yxzz(), i16vec4(2_i16, 1_i16, 3_i16, 3_i16));
    assert_eq!(v.yxzw(), i16vec4(2_i16, 1_i16, 3_i16, 4_i16));
    assert_eq!(v.yxwx(), i16vec4(2_i16, 1_i16, 4_i16, 1_i16));
    assert_eq!(v.yxwy(), i16vec4(2_i16, 1_i16, 4_i16, 2_i16));
    assert_eq!(v.yxwz(), i16vec4(2_i16, 1_i16, 4_i16, 3_i16));
    assert_eq!(v.yxww(), i16vec4(2_i16, 1_i16, 4_i16, 4_i16));
    assert_eq!(v.yyxx(), i16vec4(2_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.yyxy(), i16vec4(2_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.yyxz(), i16vec4(2_i16, 2_i16, 1_i16, 3_i16));
    assert_eq!(v.yyxw(), i16vec4(2_i16, 2_i16, 1_i16, 4_i16));
    assert_eq!(v.yyyx(), i16vec4(2_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.yyyy(), i16vec4(2_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.yyyz(), i16vec4(2_i16, 2_i16, 2_i16, 3_i16));
    assert_eq!(v.yyyw(), i16vec4(2_i16, 2_i16, 2_i16, 4_i16));
    assert_eq!(v.yyzx(), i16vec4(2_i16, 2_i16, 3_i16, 1_i16));
    assert_eq!(v.yyzy(), i16vec4(2_i16, 2_i16, 3_i16, 2_i16));
    assert_eq!(v.yyzz(), i16vec4(2_i16, 2_i16, 3_i16, 3_i16));
    assert_eq!(v.yyzw(), i16vec4(2_i16, 2_i16, 3_i16, 4_i16));
    assert_eq!(v.yywx(), i16vec4(2_i16, 2_i16, 4_i16, 1_i16));
    assert_eq!(v.yywy(), i16vec4(2_i16, 2_i16, 4_i16, 2_i16));
    assert_eq!(v.yywz(), i16vec4(2_i16, 2_i16, 4_i16, 3_i16));
    assert_eq!(v.yyww(), i16vec4(2_i16, 2_i16, 4_i16, 4_i16));
    assert_eq!(v.yzxx(), i16vec4(2_i16, 3_i16, 1_i16, 1_i16));
    assert_eq!(v.yzxy(), i16vec4(2_i16, 3_i16, 1_i16, 2_i16));
    assert_eq!(v.yzxz(), i16vec4(2_i16, 3_i16, 1_i16, 3_i16));
    assert_eq!(v.yzxw(), i16vec4(2_i16, 3_i16, 1_i16, 4_i16));
    assert_eq!(v.yzyx(), i16vec4(2_i16, 3_i16, 2_i16, 1_i16));
    assert_eq!(v.yzyy(), i16vec4(2_i16, 3_i16, 2_i16, 2_i16));
    assert_eq!(v.yzyz(), i16vec4(2_i16, 3_i16, 2_i16, 3_i16));
    assert_eq!(v.yzyw(), i16vec4(2_i16, 3_i16, 2_i16, 4_i16));
    assert_eq!(v.yzzx(), i16vec4(2_i16, 3_i16, 3_i16, 1_i16));
    assert_eq!(v.yzzy(), i16vec4(2_i16, 3_i16, 3_i16, 2_i16));
    assert_eq!(v.yzzz(), i16vec4(2_i16, 3_i16, 3_i16, 3_i16));
    assert_eq!(v.yzzw(), i16vec4(2_i16, 3_i16, 3_i16, 4_i16));
    assert_eq!(v.yzwx(), i16vec4(2_i16, 3_i16, 4_i16, 1_i16));
    assert_eq!(v.yzwy(), i16vec4(2_i16, 3_i16, 4_i16, 2_i16));
    assert_eq!(v.yzwz(), i16vec4(2_i16, 3_i16, 4_i16, 3_i16));
    assert_eq!(v.yzww(), i16vec4(2_i16, 3_i16, 4_i16, 4_i16));
    assert_eq!(v.ywxx(), i16vec4(2_i16, 4_i16, 1_i16, 1_i16));
    assert_eq!(v.ywxy(), i16vec4(2_i16, 4_i16, 1_i16, 2_i16));
    assert_eq!(v.ywxz(), i16vec4(2_i16, 4_i16, 1_i16, 3_i16));
    assert_eq!(v.ywxw(), i16vec4(2_i16, 4_i16, 1_i16, 4_i16));
    assert_eq!(v.ywyx(), i16vec4(2_i16, 4_i16, 2_i16, 1_i16));
    assert_eq!(v.ywyy(), i16vec4(2_i16, 4_i16, 2_i16, 2_i16));
    assert_eq!(v.ywyz(), i16vec4(2_i16, 4_i16, 2_i16, 3_i16));
    assert_eq!(v.ywyw(), i16vec4(2_i16, 4_i16, 2_i16, 4_i16));
    assert_eq!(v.ywzx(), i16vec4(2_i16, 4_i16, 3_i16, 1_i16));
    assert_eq!(v.ywzy(), i16vec4(2_i16, 4_i16, 3_i16, 2_i16));
    assert_eq!(v.ywzz(), i16vec4(2_i16, 4_i16, 3_i16, 3_i16));
    assert_eq!(v.ywzw(), i16vec4(2_i16, 4_i16, 3_i16, 4_i16));
    assert_eq!(v.ywwx(), i16vec4(2_i16, 4_i16, 4_i16, 1_i16));
    assert_eq!(v.ywwy(), i16vec4(2_i16, 4_i16, 4_i16, 2_i16));
    assert_eq!(v.ywwz(), i16vec4(2_i16, 4_i16, 4_i16, 3_i16));
    assert_eq!(v.ywww(), i16vec4(2_i16, 4_i16, 4_i16, 4_i16));
    assert_eq!(v.zxxx(), i16vec4(3_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.zxxy(), i16vec4(3_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.zxxz(), i16vec4(3_i16, 1_i16, 1_i16, 3_i16));
    assert_eq!(v.zxxw(), i16vec4(3_i16, 1_i16, 1_i16, 4_i16));
    assert_eq!(v.zxyx(), i16vec4(3_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.zxyy(), i16vec4(3_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.zxyz(), i16vec4(3_i16, 1_i16, 2_i16, 3_i16));
    assert_eq!(v.zxyw(), i16vec4(3_i16, 1_i16, 2_i16, 4_i16));
    assert_eq!(v.zxzx(), i16vec4(3_i16, 1_i16, 3_i16, 1_i16));
    assert_eq!(v.zxzy(), i16vec4(3_i16, 1_i16, 3_i16, 2_i16));
    assert_eq!(v.zxzz(), i16vec4(3_i16, 1_i16, 3_i16, 3_i16));
    assert_eq!(v.zxzw(), i16vec4(3_i16, 1_i16, 3_i16, 4_i16));
    assert_eq!(v.zxwx(), i16vec4(3_i16, 1_i16, 4_i16, 1_i16));
    assert_eq!(v.zxwy(), i16vec4(3_i16, 1_i16, 4_i16, 2_i16));
    assert_eq!(v.zxwz(), i16vec4(3_i16, 1_i16, 4_i16, 3_i16));
    assert_eq!(v.zxww(), i16vec4(3_i16, 1_i16, 4_i16, 4_i16));
    assert_eq!(v.zyxx(), i16vec4(3_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.zyxy(), i16vec4(3_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.zyxz(), i16vec4(3_i16, 2_i16, 1_i16, 3_i16));
    assert_eq!(v.zyxw(), i16vec4(3_i16, 2_i16, 1_i16, 4_i16));
    assert_eq!(v.zyyx(), i16vec4(3_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.zyyy(), i16vec4(3_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.zyyz(), i16vec4(3_i16, 2_i16, 2_i16, 3_i16));
    assert_eq!(v.zyyw(), i16vec4(3_i16, 2_i16, 2_i16, 4_i16));
    assert_eq!(v.zyzx(), i16vec4(3_i16, 2_i16, 3_i16, 1_i16));
    assert_eq!(v.zyzy(), i16vec4(3_i16, 2_i16, 3_i16, 2_i16));
    assert_eq!(v.zyzz(), i16vec4(3_i16, 2_i16, 3_i16, 3_i16));
    assert_eq!(v.zyzw(), i16vec4(3_i16, 2_i16, 3_i16, 4_i16));
    assert_eq!(v.zywx(), i16vec4(3_i16, 2_i16, 4_i16, 1_i16));
    assert_eq!(v.zywy(), i16vec4(3_i16, 2_i16, 4_i16, 2_i16));
    assert_eq!(v.zywz(), i16vec4(3_i16, 2_i16, 4_i16, 3_i16));
    assert_eq!(v.zyww(), i16vec4(3_i16, 2_i16, 4_i16, 4_i16));
    assert_eq!(v.zzxx(), i16vec4(3_i16, 3_i16, 1_i16, 1_i16));
    assert_eq!(v.zzxy(), i16vec4(3_i16, 3_i16, 1_i16, 2_i16));
    assert_eq!(v.zzxz(), i16vec4(3_i16, 3_i16, 1_i16, 3_i16));
    assert_eq!(v.zzxw(), i16vec4(3_i16, 3_i16, 1_i16, 4_i16));
    assert_eq!(v.zzyx(), i16vec4(3_i16, 3_i16, 2_i16, 1_i16));
    assert_eq!(v.zzyy(), i16vec4(3_i16, 3_i16, 2_i16, 2_i16));
    assert_eq!(v.zzyz(), i16vec4(3_i16, 3_i16, 2_i16, 3_i16));
    assert_eq!(v.zzyw(), i16vec4(3_i16, 3_i16, 2_i16, 4_i16));
    assert_eq!(v.zzzx(), i16vec4(3_i16, 3_i16, 3_i16, 1_i16));
    assert_eq!(v.zzzy(), i16vec4(3_i16, 3_i16, 3_i16, 2_i16));
    assert_eq!(v.zzzz(), i16vec4(3_i16, 3_i16, 3_i16, 3_i16));
    assert_eq!(v.zzzw(), i16vec4(3_i16, 3_i16, 3_i16, 4_i16));
    assert_eq!(v.zzwx(), i16vec4(3_i16, 3_i16, 4_i16, 1_i16));
    assert_eq!(v.zzwy(), i16vec4(3_i16, 3_i16, 4_i16, 2_i16));
    assert_eq!(v.zzwz(), i16vec4(3_i16, 3_i16, 4_i16, 3_i16));
    assert_eq!(v.zzww(), i16vec4(3_i16, 3_i16, 4_i16, 4_i16));
    assert_eq!(v.zwxx(), i16vec4(3_i16, 4_i16, 1_i16, 1_i16));
    assert_eq!(v.zwxy(), i16vec4(3_i16, 4_i16, 1_i16, 2_i16));
    assert_eq!(v.zwxz(), i16vec4(3_i16, 4_i16, 1_i16, 3_i16));
    assert_eq!(v.zwxw(), i16vec4(3_i16, 4_i16, 1_i16, 4_i16));
    assert_eq!(v.zwyx(), i16vec4(3_i16, 4_i16, 2_i16, 1_i16));
    assert_eq!(v.zwyy(), i16vec4(3_i16, 4_i16, 2_i16, 2_i16));
    assert_eq!(v.zwyz(), i16vec4(3_i16, 4_i16, 2_i16, 3_i16));
    assert_eq!(v.zwyw(), i16vec4(3_i16, 4_i16, 2_i16, 4_i16));
    assert_eq!(v.zwzx(), i16vec4(3_i16, 4_i16, 3_i16, 1_i16));
    assert_eq!(v.zwzy(), i16vec4(3_i16, 4_i16, 3_i16, 2_i16));
    assert_eq!(v.zwzz(), i16vec4(3_i16, 4_i16, 3_i16, 3_i16));
    assert_eq!(v.zwzw(), i16vec4(3_i16, 4_i16, 3_i16, 4_i16));
    assert_eq!(v.zwwx(), i16vec4(3_i16, 4_i16, 4_i16, 1_i16));
    assert_eq!(v.zwwy(), i16vec4(3_i16, 4_i16, 4_i16, 2_i16));
    assert_eq!(v.zwwz(), i16vec4(3_i16, 4_i16, 4_i16, 3_i16));
    assert_eq!(v.zwww(), i16vec4(3_i16, 4_i16, 4_i16, 4_i16));
    assert_eq!(v.wxxx(), i16vec4(4_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.wxxy(), i16vec4(4_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.wxxz(), i16vec4(4_i16, 1_i16, 1_i16, 3_i16));
    assert_eq!(v.wxxw(), i16vec4(4_i16, 1_i16, 1_i16, 4_i16));
    assert_eq!(v.wxyx(), i16vec4(4_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.wxyy(), i16vec4(4_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.wxyz(), i16vec4(4_i16, 1_i16, 2_i16, 3_i16));
    assert_eq!(v.wxyw(), i16vec4(4_i16, 1_i16, 2_i16, 4_i16));
    assert_eq!(v.wxzx(), i16vec4(4_i16, 1_i16, 3_i16, 1_i16));
    assert_eq!(v.wxzy(), i16vec4(4_i16, 1_i16, 3_i16, 2_i16));
    assert_eq!(v.wxzz(), i16vec4(4_i16, 1_i16, 3_i16, 3_i16));
    assert_eq!(v.wxzw(), i16vec4(4_i16, 1_i16, 3_i16, 4_i16));
    assert_eq!(v.wxwx(), i16vec4(4_i16, 1_i16, 4_i16, 1_i16));
    assert_eq!(v.wxwy(), i16vec4(4_i16, 1_i16, 4_i16, 2_i16));
    assert_eq!(v.wxwz(), i16vec4(4_i16, 1_i16, 4_i16, 3_i16));
    assert_eq!(v.wxww(), i16vec4(4_i16, 1_i16, 4_i16, 4_i16));
    assert_eq!(v.wyxx(), i16vec4(4_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.wyxy(), i16vec4(4_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.wyxz(), i16vec4(4_i16, 2_i16, 1_i16, 3_i16));
    assert_eq!(v.wyxw(), i16vec4(4_i16, 2_i16, 1_i16, 4_i16));
    assert_eq!(v.wyyx(), i16vec4(4_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.wyyy(), i16vec4(4_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.wyyz(), i16vec4(4_i16, 2_i16, 2_i16, 3_i16));
    assert_eq!(v.wyyw(), i16vec4(4_i16, 2_i16, 2_i16, 4_i16));
    assert_eq!(v.wyzx(), i16vec4(4_i16, 2_i16, 3_i16, 1_i16));
    assert_eq!(v.wyzy(), i16vec4(4_i16, 2_i16, 3_i16, 2_i16));
    assert_eq!(v.wyzz(), i16vec4(4_i16, 2_i16, 3_i16, 3_i16));
    assert_eq!(v.wyzw(), i16vec4(4_i16, 2_i16, 3_i16, 4_i16));
    assert_eq!(v.wywx(), i16vec4(4_i16, 2_i16, 4_i16, 1_i16));
    assert_eq!(v.wywy(), i16vec4(4_i16, 2_i16, 4_i16, 2_i16));
    assert_eq!(v.wywz(), i16vec4(4_i16, 2_i16, 4_i16, 3_i16));
    assert_eq!(v.wyww(), i16vec4(4_i16, 2_i16, 4_i16, 4_i16));
    assert_eq!(v.wzxx(), i16vec4(4_i16, 3_i16, 1_i16, 1_i16));
    assert_eq!(v.wzxy(), i16vec4(4_i16, 3_i16, 1_i16, 2_i16));
    assert_eq!(v.wzxz(), i16vec4(4_i16, 3_i16, 1_i16, 3_i16));
    assert_eq!(v.wzxw(), i16vec4(4_i16, 3_i16, 1_i16, 4_i16));
    assert_eq!(v.wzyx(), i16vec4(4_i16, 3_i16, 2_i16, 1_i16));
    assert_eq!(v.wzyy(), i16vec4(4_i16, 3_i16, 2_i16, 2_i16));
    assert_eq!(v.wzyz(), i16vec4(4_i16, 3_i16, 2_i16, 3_i16));
    assert_eq!(v.wzyw(), i16vec4(4_i16, 3_i16, 2_i16, 4_i16));
    assert_eq!(v.wzzx(), i16vec4(4_i16, 3_i16, 3_i16, 1_i16));
    assert_eq!(v.wzzy(), i16vec4(4_i16, 3_i16, 3_i16, 2_i16));
    assert_eq!(v.wzzz(), i16vec4(4_i16, 3_i16, 3_i16, 3_i16));
    assert_eq!(v.wzzw(), i16vec4(4_i16, 3_i16, 3_i16, 4_i16));
    assert_eq!(v.wzwx(), i16vec4(4_i16, 3_i16, 4_i16, 1_i16));
    assert_eq!(v.wzwy(), i16vec4(4_i16, 3_i16, 4_i16, 2_i16));
    assert_eq!(v.wzwz(), i16vec4(4_i16, 3_i16, 4_i16, 3_i16));
    assert_eq!(v.wzww(), i16vec4(4_i16, 3_i16, 4_i16, 4_i16));
    assert_eq!(v.wwxx(), i16vec4(4_i16, 4_i16, 1_i16, 1_i16));
    assert_eq!(v.wwxy(), i16vec4(4_i16, 4_i16, 1_i16, 2_i16));
    assert_eq!(v.wwxz(), i16vec4(4_i16, 4_i16, 1_i16, 3_i16));
    assert_eq!(v.wwxw(), i16vec4(4_i16, 4_i16, 1_i16, 4_i16));
    assert_eq!(v.wwyx(), i16vec4(4_i16, 4_i16, 2_i16, 1_i16));
    assert_eq!(v.wwyy(), i16vec4(4_i16, 4_i16, 2_i16, 2_i16));
    assert_eq!(v.wwyz(), i16vec4(4_i16, 4_i16, 2_i16, 3_i16));
    assert_eq!(v.wwyw(), i16vec4(4_i16, 4_i16, 2_i16, 4_i16));
    assert_eq!(v.wwzx(), i16vec4(4_i16, 4_i16, 3_i16, 1_i16));
    assert_eq!(v.wwzy(), i16vec4(4_i16, 4_i16, 3_i16, 2_i16));
    assert_eq!(v.wwzz(), i16vec4(4_i16, 4_i16, 3_i16, 3_i16));
    assert_eq!(v.wwzw(), i16vec4(4_i16, 4_i16, 3_i16, 4_i16));
    assert_eq!(v.wwwx(), i16vec4(4_i16, 4_i16, 4_i16, 1_i16));
    assert_eq!(v.wwwy(), i16vec4(4_i16, 4_i16, 4_i16, 2_i16));
    assert_eq!(v.wwwz(), i16vec4(4_i16, 4_i16, 4_i16, 3_i16));
    assert_eq!(v.wwww(), i16vec4(4_i16, 4_i16, 4_i16, 4_i16));
    assert_eq!(v.xxx(), i16vec3(1_i16, 1_i16, 1_i16));
    assert_eq!(v.xxy(), i16vec3(1_i16, 1_i16, 2_i16));
    assert_eq!(v.xxz(), i16vec3(1_i16, 1_i16, 3_i16));
    assert_eq!(v.xxw(), i16vec3(1_i16, 1_i16, 4_i16));
    assert_eq!(v.xyx(), i16vec3(1_i16, 2_i16, 1_i16));
    assert_eq!(v.xyy(), i16vec3(1_i16, 2_i16, 2_i16));
    assert_eq!(v.xyz(), i16vec3(1_i16, 2_i16, 3_i16));
    assert_eq!(v.xyw(), i16vec3(1_i16, 2_i16, 4_i16));
    assert_eq!(v.xzx(), i16vec3(1_i16, 3_i16, 1_i16));
    assert_eq!(v.xzy(), i16vec3(1_i16, 3_i16, 2_i16));
    assert_eq!(v.xzz(), i16vec3(1_i16, 3_i16, 3_i16));
    assert_eq!(v.xzw(), i16vec3(1_i16, 3_i16, 4_i16));
    assert_eq!(v.xwx(), i16vec3(1_i16, 4_i16, 1_i16));
    assert_eq!(v.xwy(), i16vec3(1_i16, 4_i16, 2_i16));
    assert_eq!(v.xwz(), i16vec3(1_i16, 4_i16, 3_i16));
    assert_eq!(v.xww(), i16vec3(1_i16, 4_i16, 4_i16));
    assert_eq!(v.yxx(), i16vec3(2_i16, 1_i16, 1_i16));
    assert_eq!(v.yxy(), i16vec3(2_i16, 1_i16, 2_i16));
    assert_eq!(v.yxz(), i16vec3(2_i16, 1_i16, 3_i16));
    assert_eq!(v.yxw(), i16vec3(2_i16, 1_i16, 4_i16));
    assert_eq!(v.yyx(), i16vec3(2_i16, 2_i16, 1_i16));
    assert_eq!(v.yyy(), i16vec3(2_i16, 2_i16, 2_i16));
    assert_eq!(v.yyz(), i16vec3(2_i16, 2_i16, 3_i16));
    assert_eq!(v.yyw(), i16vec3(2_i16, 2_i16, 4_i16));
    assert_eq!(v.yzx(), i16vec3(2_i16, 3_i16, 1_i16));
    assert_eq!(v.yzy(), i16vec3(2_i16, 3_i16, 2_i16));
    assert_eq!(v.yzz(), i16vec3(2_i16, 3_i16, 3_i16));
    assert_eq!(v.yzw(), i16vec3(2_i16, 3_i16, 4_i16));
    assert_eq!(v.ywx(), i16vec3(2_i16, 4_i16, 1_i16));
    assert_eq!(v.ywy(), i16vec3(2_i16, 4_i16, 2_i16));
    assert_eq!(v.ywz(), i16vec3(2_i16, 4_i16, 3_i16));
    assert_eq!(v.yww(), i16vec3(2_i16, 4_i16, 4_i16));
    assert_eq!(v.zxx(), i16vec3(3_i16, 1_i16, 1_i16));
    assert_eq!(v.zxy(), i16vec3(3_i16, 1_i16, 2_i16));
    assert_eq!(v.zxz(), i16vec3(3_i16, 1_i16, 3_i16));
    assert_eq!(v.zxw(), i16vec3(3_i16, 1_i16, 4_i16));
    assert_eq!(v.zyx(), i16vec3(3_i16, 2_i16, 1_i16));
    assert_eq!(v.zyy(), i16vec3(3_i16, 2_i16, 2_i16));
    assert_eq!(v.zyz(), i16vec3(3_i16, 2_i16, 3_i16));
    assert_eq!(v.zyw(), i16vec3(3_i16, 2_i16, 4_i16));
    assert_eq!(v.zzx(), i16vec3(3_i16, 3_i16, 1_i16));
    assert_eq!(v.zzy(), i16vec3(3_i16, 3_i16, 2_i16));
    assert_eq!(v.zzz(), i16vec3(3_i16, 3_i16, 3_i16));
    assert_eq!(v.zzw(), i16vec3(3_i16, 3_i16, 4_i16));
    assert_eq!(v.zwx(), i16vec3(3_i16, 4_i16, 1_i16));
    assert_eq!(v.zwy(), i16vec3(3_i16, 4_i16, 2_i16));
    assert_eq!(v.zwz(), i16vec3(3_i16, 4_i16, 3_i16));
    assert_eq!(v.zww(), i16vec3(3_i16, 4_i16, 4_i16));
    assert_eq!(v.wxx(), i16vec3(4_i16, 1_i16, 1_i16));
    assert_eq!(v.wxy(), i16vec3(4_i16, 1_i16, 2_i16));
    assert_eq!(v.wxz(), i16vec3(4_i16, 1_i16, 3_i16));
    assert_eq!(v.wxw(), i16vec3(4_i16, 1_i16, 4_i16));
    assert_eq!(v.wyx(), i16vec3(4_i16, 2_i16, 1_i16));
    assert_eq!(v.wyy(), i16vec3(4_i16, 2_i16, 2_i16));
    assert_eq!(v.wyz(), i16vec3(4_i16, 2_i16, 3_i16));
    assert_eq!(v.wyw(), i16vec3(4_i16, 2_i16, 4_i16));
    assert_eq!(v.wzx(), i16vec3(4_i16, 3_i16, 1_i16));
    assert_eq!(v.wzy(), i16vec3(4_i16, 3_i16, 2_i16));
    assert_eq!(v.wzz(), i16vec3(4_i16, 3_i16, 3_i16));
    assert_eq!(v.wzw(), i16vec3(4_i16, 3_i16, 4_i16));
    assert_eq!(v.wwx(), i16vec3(4_i16, 4_i16, 1_i16));
    assert_eq!(v.wwy(), i16vec3(4_i16, 4_i16, 2_i16));
    assert_eq!(v.wwz(), i16vec3(4_i16, 4_i16, 3_i16));
    assert_eq!(v.www(), i16vec3(4_i16, 4_i16, 4_i16));
    assert_eq!(v.xx(), i16vec2(1_i16, 1_i16));
    assert_eq!(v.xy(), i16vec2(1_i16, 2_i16));
    assert_eq!(v.xz(), i16vec2(1_i16, 3_i16));
    assert_eq!(v.xw(), i16vec2(1_i16, 4_i16));
    assert_eq!(v.yx(), i16vec2(2_i16, 1_i16));
    assert_eq!(v.yy(), i16vec2(2_i16, 2_i16));
    assert_eq!(v.yz(), i16vec2(2_i16, 3_i16));
    assert_eq!(v.yw(), i16vec2(2_i16, 4_i16));
    assert_eq!(v.zx(), i16vec2(3_i16, 1_i16));
    assert_eq!(v.zy(), i16vec2(3_i16, 2_i16));
    assert_eq!(v.zz(), i16vec2(3_i16, 3_i16));
    assert_eq!(v.zw(), i16vec2(3_i16, 4_i16));
    assert_eq!(v.wx(), i16vec2(4_i16, 1_i16));
    assert_eq!(v.wy(), i16vec2(4_i16, 2_i16));
    assert_eq!(v.wz(), i16vec2(4_i16, 3_i16));
    assert_eq!(v.ww(), i16vec2(4_i16, 4_i16));
});

glam_test!(test_i16vec3_swizzles, {
    let v = i16vec3(1_i16, 2_i16, 3_i16);
    assert_eq!(v, v.xyz());
    assert_eq!(v.xxxx(), i16vec4(1_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.xxxy(), i16vec4(1_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.xxxz(), i16vec4(1_i16, 1_i16, 1_i16, 3_i16));
    assert_eq!(v.xxyx(), i16vec4(1_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.xxyy(), i16vec4(1_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.xxyz(), i16vec4(1_i16, 1_i16, 2_i16, 3_i16));
    assert_eq!(v.xxzx(), i16vec4(1_i16, 1_i16, 3_i16, 1_i16));
    assert_eq!(v.xxzy(), i16vec4(1_i16, 1_i16, 3_i16, 2_i16));
    assert_eq!(v.xxzz(), i16vec4(1_i16, 1_i16, 3_i16, 3_i16));
    assert_eq!(v.xyxx(), i16vec4(1_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.xyxy(), i16vec4(1_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.xyxz(), i16vec4(1_i16, 2_i16, 1_i16, 3_i16));
    assert_eq!(v.xyyx(), i16vec4(1_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.xyyy(), i16vec4(1_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.xyyz(), i16vec4(1_i16, 2_i16, 2_i16, 3_i16));
    assert_eq!(v.xyzx(), i16vec4(1_i16, 2_i16, 3_i16, 1_i16));
    assert_eq!(v.xyzy(), i16vec4(1_i16, 2_i16, 3_i16, 2_i16));
    assert_eq!(v.xyzz(), i16vec4(1_i16, 2_i16, 3_i16, 3_i16));
    assert_eq!(v.xzxx(), i16vec4(1_i16, 3_i16, 1_i16, 1_i16));
    assert_eq!(v.xzxy(), i16vec4(1_i16, 3_i16, 1_i16, 2_i16));
    assert_eq!(v.xzxz(), i16vec4(1_i16, 3_i16, 1_i16, 3_i16));
    assert_eq!(v.xzyx(), i16vec4(1_i16, 3_i16, 2_i16, 1_i16));
    assert_eq!(v.xzyy(), i16vec4(1_i16, 3_i16, 2_i16, 2_i16));
    assert_eq!(v.xzyz(), i16vec4(1_i16, 3_i16, 2_i16, 3_i16));
    assert_eq!(v.xzzx(), i16vec4(1_i16, 3_i16, 3_i16, 1_i16));
    assert_eq!(v.xzzy(), i16vec4(1_i16, 3_i16, 3_i16, 2_i16));
    assert_eq!(v.xzzz(), i16vec4(1_i16, 3_i16, 3_i16, 3_i16));
    assert_eq!(v.yxxx(), i16vec4(2_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.yxxy(), i16vec4(2_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.yxxz(), i16vec4(2_i16, 1_i16, 1_i16, 3_i16));
    assert_eq!(v.yxyx(), i16vec4(2_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.yxyy(), i16vec4(2_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.yxyz(), i16vec4(2_i16, 1_i16, 2_i16, 3_i16));
    assert_eq!(v.yxzx(), i16vec4(2_i16, 1_i16, 3_i16, 1_i16));
    assert_eq!(v.yxzy(), i16vec4(2_i16, 1_i16, 3_i16, 2_i16));
    assert_eq!(v.yxzz(), i16vec4(2_i16, 1_i16, 3_i16, 3_i16));
    assert_eq!(v.yyxx(), i16vec4(2_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.yyxy(), i16vec4(2_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.yyxz(), i16vec4(2_i16, 2_i16, 1_i16, 3_i16));
    assert_eq!(v.yyyx(), i16vec4(2_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.yyyy(), i16vec4(2_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.yyyz(), i16vec4(2_i16, 2_i16, 2_i16, 3_i16));
    assert_eq!(v.yyzx(), i16vec4(2_i16, 2_i16, 3_i16, 1_i16));
    assert_eq!(v.yyzy(), i16vec4(2_i16, 2_i16, 3_i16, 2_i16));
    assert_eq!(v.yyzz(), i16vec4(2_i16, 2_i16, 3_i16, 3_i16));
    assert_eq!(v.yzxx(), i16vec4(2_i16, 3_i16, 1_i16, 1_i16));
    assert_eq!(v.yzxy(), i16vec4(2_i16, 3_i16, 1_i16, 2_i16));
    assert_eq!(v.yzxz(), i16vec4(2_i16, 3_i16, 1_i16, 3_i16));
    assert_eq!(v.yzyx(), i16vec4(2_i16, 3_i16, 2_i16, 1_i16));
    assert_eq!(v.yzyy(), i16vec4(2_i16, 3_i16, 2_i16, 2_i16));
    assert_eq!(v.yzyz(), i16vec4(2_i16, 3_i16, 2_i16, 3_i16));
    assert_eq!(v.yzzx(), i16vec4(2_i16, 3_i16, 3_i16, 1_i16));
    assert_eq!(v.yzzy(), i16vec4(2_i16, 3_i16, 3_i16, 2_i16));
    assert_eq!(v.yzzz(), i16vec4(2_i16, 3_i16, 3_i16, 3_i16));
    assert_eq!(v.zxxx(), i16vec4(3_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.zxxy(), i16vec4(3_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.zxxz(), i16vec4(3_i16, 1_i16, 1_i16, 3_i16));
    assert_eq!(v.zxyx(), i16vec4(3_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.zxyy(), i16vec4(3_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.zxyz(), i16vec4(3_i16, 1_i16, 2_i16, 3_i16));
    assert_eq!(v.zxzx(), i16vec4(3_i16, 1_i16, 3_i16, 1_i16));
    assert_eq!(v.zxzy(), i16vec4(3_i16, 1_i16, 3_i16, 2_i16));
    assert_eq!(v.zxzz(), i16vec4(3_i16, 1_i16, 3_i16, 3_i16));
    assert_eq!(v.zyxx(), i16vec4(3_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.zyxy(), i16vec4(3_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.zyxz(), i16vec4(3_i16, 2_i16, 1_i16, 3_i16));
    assert_eq!(v.zyyx(), i16vec4(3_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.zyyy(), i16vec4(3_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.zyyz(), i16vec4(3_i16, 2_i16, 2_i16, 3_i16));
    assert_eq!(v.zyzx(), i16vec4(3_i16, 2_i16, 3_i16, 1_i16));
    assert_eq!(v.zyzy(), i16vec4(3_i16, 2_i16, 3_i16, 2_i16));
    assert_eq!(v.zyzz(), i16vec4(3_i16, 2_i16, 3_i16, 3_i16));
    assert_eq!(v.zzxx(), i16vec4(3_i16, 3_i16, 1_i16, 1_i16));
    assert_eq!(v.zzxy(), i16vec4(3_i16, 3_i16, 1_i16, 2_i16));
    assert_eq!(v.zzxz(), i16vec4(3_i16, 3_i16, 1_i16, 3_i16));
    assert_eq!(v.zzyx(), i16vec4(3_i16, 3_i16, 2_i16, 1_i16));
    assert_eq!(v.zzyy(), i16vec4(3_i16, 3_i16, 2_i16, 2_i16));
    assert_eq!(v.zzyz(), i16vec4(3_i16, 3_i16, 2_i16, 3_i16));
    assert_eq!(v.zzzx(), i16vec4(3_i16, 3_i16, 3_i16, 1_i16));
    assert_eq!(v.zzzy(), i16vec4(3_i16, 3_i16, 3_i16, 2_i16));
    assert_eq!(v.zzzz(), i16vec4(3_i16, 3_i16, 3_i16, 3_i16));
    assert_eq!(v.xxx(), i16vec3(1_i16, 1_i16, 1_i16));
    assert_eq!(v.xxy(), i16vec3(1_i16, 1_i16, 2_i16));
    assert_eq!(v.xxz(), i16vec3(1_i16, 1_i16, 3_i16));
    assert_eq!(v.xyx(), i16vec3(1_i16, 2_i16, 1_i16));
    assert_eq!(v.xyy(), i16vec3(1_i16, 2_i16, 2_i16));
    assert_eq!(v.xzx(), i16vec3(1_i16, 3_i16, 1_i16));
    assert_eq!(v.xzy(), i16vec3(1_i16, 3_i16, 2_i16));
    assert_eq!(v.xzz(), i16vec3(1_i16, 3_i16, 3_i16));
    assert_eq!(v.yxx(), i16vec3(2_i16, 1_i16, 1_i16));
    assert_eq!(v.yxy(), i16vec3(2_i16, 1_i16, 2_i16));
    assert_eq!(v.yxz(), i16vec3(2_i16, 1_i16, 3_i16));
    assert_eq!(v.yyx(), i16vec3(2_i16, 2_i16, 1_i16));
    assert_eq!(v.yyy(), i16vec3(2_i16, 2_i16, 2_i16));
    assert_eq!(v.yyz(), i16vec3(2_i16, 2_i16, 3_i16));
    assert_eq!(v.yzx(), i16vec3(2_i16, 3_i16, 1_i16));
    assert_eq!(v.yzy(), i16vec3(2_i16, 3_i16, 2_i16));
    assert_eq!(v.yzz(), i16vec3(2_i16, 3_i16, 3_i16));
    assert_eq!(v.zxx(), i16vec3(3_i16, 1_i16, 1_i16));
    assert_eq!(v.zxy(), i16vec3(3_i16, 1_i16, 2_i16));
    assert_eq!(v.zxz(), i16vec3(3_i16, 1_i16, 3_i16));
    assert_eq!(v.zyx(), i16vec3(3_i16, 2_i16, 1_i16));
    assert_eq!(v.zyy(), i16vec3(3_i16, 2_i16, 2_i16));
    assert_eq!(v.zyz(), i16vec3(3_i16, 2_i16, 3_i16));
    assert_eq!(v.zzx(), i16vec3(3_i16, 3_i16, 1_i16));
    assert_eq!(v.zzy(), i16vec3(3_i16, 3_i16, 2_i16));
    assert_eq!(v.zzz(), i16vec3(3_i16, 3_i16, 3_i16));
    assert_eq!(v.xx(), i16vec2(1_i16, 1_i16));
    assert_eq!(v.xy(), i16vec2(1_i16, 2_i16));
    assert_eq!(v.xz(), i16vec2(1_i16, 3_i16));
    assert_eq!(v.yx(), i16vec2(2_i16, 1_i16));
    assert_eq!(v.yy(), i16vec2(2_i16, 2_i16));
    assert_eq!(v.yz(), i16vec2(2_i16, 3_i16));
    assert_eq!(v.zx(), i16vec2(3_i16, 1_i16));
    assert_eq!(v.zy(), i16vec2(3_i16, 2_i16));
    assert_eq!(v.zz(), i16vec2(3_i16, 3_i16));
});

glam_test!(test_i16vec2_swizzles, {
    let v = i16vec2(1_i16, 2_i16);
    assert_eq!(v, v.xy());
    assert_eq!(v.xxxx(), i16vec4(1_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.xxxy(), i16vec4(1_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.xxyx(), i16vec4(1_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.xxyy(), i16vec4(1_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.xyxx(), i16vec4(1_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.xyxy(), i16vec4(1_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.xyyx(), i16vec4(1_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.xyyy(), i16vec4(1_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.yxxx(), i16vec4(2_i16, 1_i16, 1_i16, 1_i16));
    assert_eq!(v.yxxy(), i16vec4(2_i16, 1_i16, 1_i16, 2_i16));
    assert_eq!(v.yxyx(), i16vec4(2_i16, 1_i16, 2_i16, 1_i16));
    assert_eq!(v.yxyy(), i16vec4(2_i16, 1_i16, 2_i16, 2_i16));
    assert_eq!(v.yyxx(), i16vec4(2_i16, 2_i16, 1_i16, 1_i16));
    assert_eq!(v.yyxy(), i16vec4(2_i16, 2_i16, 1_i16, 2_i16));
    assert_eq!(v.yyyx(), i16vec4(2_i16, 2_i16, 2_i16, 1_i16));
    assert_eq!(v.yyyy(), i16vec4(2_i16, 2_i16, 2_i16, 2_i16));
    assert_eq!(v.xxx(), i16vec3(1_i16, 1_i16, 1_i16));
    assert_eq!(v.xxy(), i16vec3(1_i16, 1_i16, 2_i16));
    assert_eq!(v.xyx(), i16vec3(1_i16, 2_i16, 1_i16));
    assert_eq!(v.xyy(), i16vec3(1_i16, 2_i16, 2_i16));
    assert_eq!(v.yxx(), i16vec3(2_i16, 1_i16, 1_i16));
    assert_eq!(v.yxy(), i16vec3(2_i16, 1_i16, 2_i16));
    assert_eq!(v.yyx(), i16vec3(2_i16, 2_i16, 1_i16));
    assert_eq!(v.yyy(), i16vec3(2_i16, 2_i16, 2_i16));
    assert_eq!(v.xx(), i16vec2(1_i16, 1_i16));
    assert_eq!(v.yx(), i16vec2(2_i16, 1_i16));
    assert_eq!(v.yy(), i16vec2(2_i16, 2_i16));
});
