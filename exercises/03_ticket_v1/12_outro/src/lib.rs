// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

// TODO: 新しい `Order` タイプを定義してください。
//   この型は3つの情報を追跡する必要があります: `product_name`, `quantity`, `unit_price`。
//   商品名は空であってはならず、また300バイトを超えてはなりません。
//   数量はゼロよりも厳密に大きくなければなりません。
//   単価はセント単位で、ゼロよりも厳密に大きくなければなりません。
//   `Order` は `total` という名前のメソッドを含む必要があり、注文の合計価格を返します。
//   `Order` は各フィールドに対してセッターとゲッターを提供する必要があります。
//
// テストは今回、異なる場所—`tests` フォルダにあります。
// `tests` フォルダは `cargo` にとって特別な場所であり、**統合テスト**を探す場所です。
// ここでの統合には非常に具体的な意味があります: プロジェクトの**公開API**をテストします。
// 型やメソッドの可視性に注意を払う必要があります; 統合テストは private または `pub(crate)` アイテムにアクセスできません。

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        validate_product_name(&product_name);
        validate_quantity(&quantity);
        validate_unit_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> u32 {
        let result = &self.quantity * &self.unit_price;
        result
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_value: String) {
        validate_product_name(&new_value);
        self.product_name = new_value;
    }
    pub fn set_quantity(&mut self, new_value: u32) {
        validate_quantity(&new_value);
        if new_value == 0 {
            panic!();
        }
        self.quantity = new_value;
    }
    pub fn set_unit_price(&mut self, new_value: u32) {
        validate_unit_price(&new_value);
        if new_value == 0 {
            panic!();
        }
        self.unit_price = new_value;
    }
}

fn validate_product_name(product_name: &String) {
    if product_name.is_empty() {
        panic!();
    }
    if product_name.len() > 300 {
        panic!();
    }
}
fn validate_quantity(quantity: &u32) {
    if quantity == &0 {
        panic!();
    }
}
fn validate_unit_price(unit_price: &u32) {
    if unit_price == &0 {
        panic!();
    }
}
