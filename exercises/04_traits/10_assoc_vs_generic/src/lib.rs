// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

// TODO: Powerという新しいトレイトを定義してください。このトレイトにはpowerというメソッドがあり、selfをn乗します。
// トレイトの定義とその実装がテストのコンパイルと成功に十分であるべきです。

// 推奨事項: 一度にすべてのケースを扱うために汎用的な実装を書くことに誘惑されるかもしれません。しかし、これはかなり複雑であり、追加のクレート（例えばnum-traits）の使用が必要になります。
// それでも、高度に汎用的な実装の複雑さを避けるために、単純なマクロを使用する方が好ましいかもしれません。
// もし興味があれば、「Little book of Rust macros」(https://veykril.github.io/tlborm/)をチェックして、それについてもっと学ぶことができます。
// ただし、必ずしもそうする必要はありません。手動で三つの別々の実装を書くことも完全に問題ありません。興味がある場合のみさらに進んでください。

pub trait Power<T> {
    type Output;

    fn power(&self, n: T) -> Self::Output;
}

impl Power<u16> for u32 {
    type Output = u32;

    fn power(&self, n: u16) -> Self::Output {
        self.pow(n.into())
    }
}

impl Power<&u32> for u32 {
    type Output = u32;

    fn power(&self, n: &u32) -> Self::Output {
        self.power(*n)
    }
}

impl Power<u32> for u32 {
    type Output = u32;

    fn power(&self, n: u32) -> Self::Output {
        self.pow(n)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
