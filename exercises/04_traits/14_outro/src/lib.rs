// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

// TODO: 新しい SaturatingU16 型を定義してください。
// この型は u16 値を保持するべきです。
// u16、u8、&u16、そして &u8 からの変換を提供する必要があります。
// また、右辺が SaturatingU16、u16、&u16、そして &SaturatingU16 の場合の加算をサポートする必要があります。加算は u16 の最大値で飽和するべきです。
// 他の SaturatingU16 や u16 と比較できるようにする必要があります。
// また、そのデバッグ表現を出力することができるようにする必要があります。

// テストは tests フォルダにあります。型やメソッドの可視性に注意してください。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        (*value).into()
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        (*value).into()
    }
}

impl std::ops::Add for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.value
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        self + *rhs
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        let sum = self.value.saturating_add(rhs);
        Self { value: sum }
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        self + *rhs
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}
