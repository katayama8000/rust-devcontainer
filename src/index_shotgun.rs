use std::collections::BTreeMap;

// =======================================================
// 1. データ構造と "ショットガン" されたインデックス
// =======================================================

#[derive(Debug, Clone)]
struct Product {
    id: u32,
    name: String,
    category: String, // カテゴリ (中カーディナリティ)
    price: u32,       // 価格 (高カーディナリティ)
    is_active: bool,  // 有効フラグ (低カーディナリティ)
}

type ProductTable = Vec<Product>;

// --- インデックスショットガンの状態 ---
// 手当たり次第に各列へ個別のインデックスを作成してしまっている
type CategoryIndex = BTreeMap<String, Vec<u32>>;
type PriceIndex = BTreeMap<u32, Vec<u32>>;
type IsActiveIndex = BTreeMap<bool, Vec<u32>>;

// --- 理想的な状態 ---
// よくある検索条件を考慮した複合インデックス
type CategoryPriceIndex = BTreeMap<(String, u32), Vec<u32>>;


// =======================================================
// 2. 書き込み処理のシミュレーション
// =======================================================

// ショットガンアプローチでのデータ追加
fn add_product_shotgun(
    table: &mut ProductTable, 
    cat_idx: &mut CategoryIndex, 
    price_idx: &mut PriceIndex, 
    is_active_idx: &mut IsActiveIndex, 
    product: Product
) {
    let row_index = table.len() as u32;
    
    // 各インデックスを一つずつ更新... コストが高い！
    cat_idx.entry(product.category.clone()).or_insert_with(Vec::new).push(row_index);
    price_idx.entry(product.price).or_insert_with(Vec::new).push(row_index);
    is_active_idx.entry(product.is_active).or_insert_with(Vec::new).push(row_index);
    
    table.push(product);
}

// 複合インデックスアプローチでのデータ追加
fn add_product_composite(
    table: &mut ProductTable, 
    cat_price_idx: &mut CategoryPriceIndex, 
    product: Product
) {
    let row_index = table.len() as u32;
    
    // 更新は一つの複合インデックスだけで済む
    cat_price_idx.entry((product.category.clone(), product.price)).or_insert_with(Vec::new).push(row_index);

    table.push(product);
}


// =======================================================
// 3. 検索処理のシミュレーション
// =======================================================

fn simulate_searches() {
    println!("\n--- 検索クエリのシミュレーション ---");
    println!("SQL: SELECT * FROM products WHERE category = '家電' AND price < 50000");

    println!("\n[ショットガンアプローチの場合]");
    println!("1. `category`インデックスを使い、「家電」のIDリストを取得 (多数ヒット)");
    println!("2. `price`インデックスを使い、「50000未満」のIDリストを取得 (多数ヒット)");
    println!("3. 2つのIDリストの共通部分を計算する (メモリ上で重い処理)");
    println!("-> オプティマイザが混乱し、非効率な実行計画になる可能性が高い");

    println!("\n[複合インデックスアプローチの場合]");
    println!("1. `(category, price)`の複合インデックスを使い、キー「('家電', 0)」から「('家電', 49999)」までを範囲スキャン");
    println!("-> 非常に効率的に目的のIDリストを取得できる");
}


// =======================================================
// 4. 実行と解説
// =======================================================

pub fn run() {
    println!("=======================================================");
    println!("   「インデックスショットガン」アンチパターンのデモ 🚀");
    println!("=======================================================");

    // --- ショットガンアプローチのセットアップ ---
    let mut shotgun_table = ProductTable::new();
    let mut cat_idx = CategoryIndex::new();
    let mut price_idx = PriceIndex::new();
    let mut is_active_idx = IsActiveIndex::new();

    println!("\n--- A. ショットガンアプローチでのデータ書き込み ---");
    println!("1件のデータを追加するたびに、3つのインデックスを全て更新する必要があります...");
    add_product_shotgun(&mut shotgun_table, &mut cat_idx, &mut price_idx, &mut is_active_idx, Product { id: 1, name: "TV".into(), category: "家電".into(), price: 80000, is_active: true });
    println!("  -> INSERT Product 1: 3つのインデックスを更新しました");
    add_product_shotgun(&mut shotgun_table, &mut cat_idx, &mut price_idx, &mut is_active_idx, Product { id: 2, name: "デスク".into(), category: "家具".into(), price: 40000, is_active: true });
    println!("  -> INSERT Product 2: 3つのインデックスを更新しました");
    println!("[解説] 書き込みコストがインデックスの数だけ増加し、性能が劣化します。\n");

    // --- 複合インデックスアプローチのセットアップ ---
    let mut composite_table = ProductTable::new();
    let mut cat_price_idx = CategoryPriceIndex::new();
    
    println!("--- B. 複合インデックスアプローチでのデータ書き込み ---");
    println!("1件のデータを追加する際に、更新するインデックスは1つだけです。");
    add_product_composite(&mut composite_table, &mut cat_price_idx, Product { id: 1, name: "TV".into(), category: "家電".into(), price: 80000, is_active: true });
    println!("  -> INSERT Product 1: 1つの複合インデックスを更新しました");
    add_product_composite(&mut composite_table, &mut cat_price_idx, Product { id: 2, name: "デスク".into(), category: "家具".into(), price: 40000, is_active: true });
    println!("  -> INSERT Product 2: 1つの複合インデックスを更新しました");
    println!("[解説] 書き込みコストが低く抑えられます。\n");

    // --- 検索シミュレーション ---
    simulate_searches();

    println!("\n=======================================================");
    println!("   結論");
    println!("=======================================================");
    println!("インデックスは、よく使われる検索クエリを分析し、複合インデックスを主体に設計するのが効果的です。");
    println!("手当たり次第にインデックスを作成すると、書き込み性能の低下や、予期せぬ検索性能の悪化を招きます。");
    println!("=======================================================");
}
