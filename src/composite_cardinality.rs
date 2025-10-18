use std::collections::BTreeMap;

// =======================================================
// 1. データ構造の定義
// =======================================================

#[derive(Debug, Clone)]
struct Customer {
    id: u32,
    name: String,
    prefecture: String, // 中程度のカーディナリティ
    gender: String,     // 低カーディナリティ
    last_login: String, // 高カーディナリティ
}

type CustomerTable = Vec<Customer>;

// === 複合インデックスの定義 ===
// (都道府県, 性別) - あまり効果的でない複合インデックス
type PrefGenderIndex = BTreeMap<(String, String), Vec<u32>>;

// (都道府県, 最終ログイン日) - 効果的な複合インデックス
type PrefLoginIndex = BTreeMap<(String, String), Vec<u32>>;

// =======================================================
// 2. データとインデックスのセットアップ
// =======================================================

fn setup_data() -> (CustomerTable, PrefGenderIndex, PrefLoginIndex) {
    let mut table = CustomerTable::new();
    let mut pref_gender_index = PrefGenderIndex::new();
    let mut pref_login_index = PrefLoginIndex::new();

    let total_data = 1000;
    let prefectures = vec!["北海道", "東京", "大阪", "福岡", "沖縄"]; // 5種類
    let genders = vec!["男性", "女性"]; // 2種類

    for i in 0..total_data {
        let prefecture = prefectures[i % prefectures.len()].to_string();
        let gender = genders[i % genders.len()].to_string();
        let last_login = format!("2023-10-{:02}", (i % 30) + 1); // 30種類

        let customer = Customer {
            id: i as u32 + 1,
            name: format!("Customer{}", i + 1),
            prefecture: prefecture.clone(),
            gender: gender.clone(),
            last_login: last_login.clone(),
        };
        let row_index = i as u32;
        table.push(customer);

        // (都道府県, 性別)インデックス構築
        pref_gender_index.entry((prefecture.clone(), gender)).or_insert_with(Vec::new).push(row_index);
        
        // (都道府県, 最終ログイン日)インデックス構築
        pref_login_index.entry((prefecture, last_login)).or_insert_with(Vec::new).push(row_index);
    }

    (table, pref_gender_index, pref_login_index)
}

// =======================================================
// 3. インデックス構造の可視化と説明
// =======================================================

fn print_indices(pref_gender_index: &PrefGenderIndex, pref_login_index: &PrefLoginIndex) {
    println!("\n=======================================================");
    println!("   複合インデックスとカーディナリティの関係");
    println!("=======================================================");

    // --- 低カーディナリティの複合インデックス ---
    println!("\n--- A. 低いカーディナリティの複合インデックス (都道府県, 性別) ---");
    println!("-> 複合キーの種類: {} 通り (5 prefectures * 2 genders = 10)", pref_gender_index.len());
    println!("----------------------------------------------------------------");
    for (key, indices) in pref_gender_index.iter() {
        println!("【キー: ({}, {})】 -> {} 件ヒット", key.0, key.1, indices.len());
    }
    println!("\n[解説]");
    println!("  - `prefecture` (中) と `gender` (低) の組み合わせでは、キーの多様性が増えない。");
    println!("  - 1つのキーあたりに約100件 (1000/10) のデータが紐付き、絞り込み効果が非常に薄い。");
    println!("  - このインデックスは `WHERE prefecture = '東京' AND gender = '男性'` のような検索でも、あまり効率的ではない。");

    // --- 高カーディナリティの複合インデックス ---
    println!("\n--- B. 高いカーディナリティの複合インデックス (都道府県, 最終ログイン日) ---");
    println!("-> 複合キーの種類: {} 通り (5 prefectures * 30 days = 150)", pref_login_index.len());
    println!("-------------------------------------------------------------------");
    let mut count = 0;
    for (key, indices) in pref_login_index.iter() {
        if count < 5 { // 最初の5件だけ表示
            println!("【キー: ({}, {})】 -> {} 件ヒット", key.0, key.1, indices.len());
        }
        count += 1;
    }
    println!("... (多様なキーが続く) ...");
    println!("\n[解説]");
    println!("  - `prefecture` (中) と `last_login` (高) を組み合わせると、キーの多様性が大幅に向上する。");
    println!("  - 1つのキーあたりに紐づくデータが約6-7件 (1000/150) となり、十分に絞り込めている。");
    println!("  - `WHERE prefecture = '東京' AND last_login = '2023-10-01'` のような検索で絶大な効果を発揮する。");
    println!("  - **重要**: 複合インデックスでは、カーディナリティが高い列を後方に含めると、全体のカーディナリティが高まり、インデックスの価値が上がる。");
}

// =======================================================
// 4. 仮のSQL WHERE句での絞り込みシミュレーション
// =======================================================

// A. 低カーディナリティインデックスを使った検索
fn search_with_low_cardinality_index(index: &PrefGenderIndex, prefecture: &str, gender: &str) {
    println!("\n--- 検索シミュレーション A ---");
    println!("SQL: SELECT * FROM customers WHERE prefecture = '{}' AND gender = '{}'", prefecture, gender);
    println!("-> 低カーディナリティのインデックス (prefecture, gender) を使用");
    
    match index.get(&(prefecture.to_string(), gender.to_string())) {
        Some(indices) => {
            println!("[結果] インデックス検索により {} 件の候補が見つかりました。", indices.len());
            println!("       -> その後、テーブルへ {} 回アクセスしてデータを取得します。", indices.len());
            println!("       (絞り込み効果が薄い...)");
        }
        None => println!("[結果] データが見つかりませんでした。"),
    }
}

// B. 高カーディナリティインデックスを使った検索
fn search_with_high_cardinality_index(index: &PrefLoginIndex, prefecture: &str, login_date: &str) {
    println!("\n--- 検索シミュレーション B ---");
    println!("SQL: SELECT * FROM customers WHERE prefecture = '{}' AND last_login = '{}'", prefecture, login_date);
    println!("-> 高カーディナリティのインデックス (prefecture, last_login) を使用");

    match index.get(&(prefecture.to_string(), login_date.to_string())) {
        Some(indices) => {
            println!("[結果] インデックス検索により {} 件の候補が見つかりました。", indices.len());
            println!("       -> その後、テーブルへ {} 回アクセスしてデータを取得します。", indices.len());
            println!("       (絞り込み効果が高い！)");
        }
        None => println!("[結果] データが見つかりませんでした。"),
    }
}

// C. インデックスを使わない検索 (フルテーブルスキャン)
fn search_full_scan(table: &CustomerTable, prefecture: &str, gender: &str) {
    println!("\n--- 検索シミュレーション C ---");
    println!("SQL: SELECT * FROM customers WHERE prefecture = '{}' AND gender = '{}'", prefecture, gender);
    println!("-> インデックス未使用 (フルテーブルスキャン)");

    let mut found_count = 0;
    for _ in table.iter().filter(|c| c.prefecture == prefecture && c.gender == gender) {
        found_count += 1;
    }
    println!("[結果] テーブルを {} 件スキャンし、{} 件のデータが見つかりました。", table.len(), found_count);
}


// =======================================================
// 5. 実行
// =======================================================

pub fn run() {
    println!("=======================================================");
    println!("   複合インデックスとカーディナリティのデモ 🚀");
    println!("=======================================================");
    
    let (table, pref_gender_index, pref_login_index) = setup_data();

    // インデックス構造の表示
    print_indices(&pref_gender_index, &pref_login_index);

    // 検索シミュレーションの実行
    println!("\n=======================================================");
    println!("   実践的な検索シミュレーション");
    println!("=======================================================");

    search_with_low_cardinality_index(&pref_gender_index, "東京", "男性");
    search_with_high_cardinality_index(&pref_login_index, "東京", "2023-10-02");
    search_full_scan(&table, "東京", "男性");

    println!("\n=======================================================");
    println!("   結論");
    println!("=======================================================");
    println!("複合インデックスを設計する際は、各列のカーディナリティを意識することが重要です。");
    println!("カーディナリティが低い列同士の組み合わせは避け、カーディナリティが高い列を含めることで、インデックスの効率が最大化されます。");
    println!("=======================================================");
}
