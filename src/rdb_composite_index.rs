use std::collections::BTreeMap;
use std::time::Instant;

// 仮のRDBテーブルの行構造
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    age: u8,
    city: String,
}

// ユーザーテーブル全体 (メモリ上の配列で代用)
type UsersTable = Vec<User>;

// === 複合インデックスの定義 ===
// キーをタプル (City, Age) にすることで、都市と年齢の組み合わせでインデックスを作成
type CompositeIndex = BTreeMap<(String, u8), Vec<u32>>;

// ダミーデータとインデックスのセットアップ関数
fn setup_data() -> (UsersTable, CompositeIndex) {
    let mut table = UsersTable::new();
    let mut index = CompositeIndex::new();

    let total_data = 100;
    let cities = vec!["東京", "大阪", "福岡", "札幌"];
    
    for i in 0..total_data {
        let city = cities[i % cities.len()].to_string();
        let age = (20 + i % 10) as u8;
        
        let user = User {
            id: i as u32 + 1,
            name: format!("User{}", i + 1),
            age,
            city: city.clone(),
        };
        
        let row_index = i as u32; 
        table.push(user);

        // === 複合インデックスの構築 ===
        // (city, age) のタプルをキーとして使用
        index.entry((city, age))
             .or_insert_with(Vec::new)
             .push(row_index);
    }
    (table, index)
}

fn search_with_composite_index(table: &UsersTable, index: &CompositeIndex, city: &str, age: u8) -> Vec<User> {
    println!("\n=======================================================");
    println!(" B. 処理: **複合インデックス** を使用 (都市と年齢で検索)");
    println!("=======================================================");
    let start_time = Instant::now();
    
    let mut results = Vec::new();

    // 1. 複合キー (city, age) でBTreeMapを検索
    match index.get(&(city.to_string(), age)) {
        Some(row_indices) => {
            for &i in row_indices {
                if let Some(user) = table.get(i as usize) {
                    results.push(user.clone());
                }
            }
            
            let duration = start_time.elapsed();

            println!("--- 🔍 検索コスト (N={}) ---", table.len());
            println!("  -> ステップ1: **B-tree** 検索 (複合キー): O(log N) - **高速**");
            println!("  -> ステップ2: テーブルへのアクセス回数: **{} 回** (結果件数分のみ)", row_indices.len());
            println!("  -> 処理時間 (目安): **{:?}**", duration);
        }
        None => {
            let duration = start_time.elapsed();
            println!("--- 🔍 検索コスト ---");
            println!("  -> B-tree 検索の結果、データは見つかりませんでした。");
            println!("  -> 処理時間 (目安): **{:?}**", duration);
        }
    }

    println!("--- ✅ 検索結果 ---");
    println!("  -> 見つかった件数: **{} 件**", results.len());
    
    results
}

fn search_without_index(table: &UsersTable, city: &str, age: u8) -> Vec<User> {
    println!("\n=======================================================");
    println!(" A. 処理: インデックスなし (フルテーブルスキャン)");
    println!("=======================================================");
    let start_time = Instant::now(); 
    
    let mut results = Vec::new();
    
    for user in table.iter() {
        if user.city == city && user.age == age {
            results.push(user.clone());
        }
    }

    let duration = start_time.elapsed(); 
    
    println!("--- 🔍 検索コスト (N={}) ---", table.len());
    println!("  -> 比較/チェックした行数: **{} 行** (テーブル全件スキャン)", table.len()); 
    println!("  -> 処理時間 (目安): **{:?}**", duration);
    
    println!("--- ✅ 検索結果 ---");
    println!("  -> 見つかった件数: **{} 件**", results.len());
    
    results
}


// =======================================================
// run 関数
// =======================================================
pub fn run() {
    println!("=======================================================");
    println!("   RDB複合インデックスによる検索コスト比較デモ 🚀");
    println!("=======================================================");
    
    let (users_table, composite_index) = setup_data();
    let search_city = "東京";
    let search_age = 22;

    println!("総データ件数: **{} 件**", users_table.len());
    println!("検索条件: **都市='{}' AND 年齢={}**", search_city, search_age);

    // ---------------------------------------------------
    // ⭐ 複合インデックスの中身を出力
    // ---------------------------------------------------
    println!("\n=======================================================");
    println!("   **複合インデックス** の構造（中身の可視化）");
    println!("   (キーが (都市, 年齢) のタプルでソートされています)");
    println!("=======================================================");
    
    for ((city, age), indices) in composite_index.iter() {
        let sample_indices = if indices.len() > 5 {
            format!("{:?}...", &indices[..5]) // 5件まで表示し、省略
        } else {
            format!("{:?}", indices)
        };
        
        println!("【キー: (都市: {}, 年齢: {})】 -> 件数: {}件, 行番号: {}", 
                 city, age, indices.len(), sample_indices);
    }
    println!("=======================================================");

    // 実行比較
    let _ = search_without_index(&users_table, search_city, search_age);
    let _ = search_with_composite_index(&users_table, &composite_index, search_city, search_age);
    
    println!("\n=======================================================");
    println!("   最終結果確認");
    println!("=======================================================");
    println!("（複合インデックスにより、複数条件での検索も高速化されます）");
    println!("=======================================================");
}
