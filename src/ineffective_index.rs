use std::collections::BTreeMap;

// =======================================================
// 1. データ構造とインデックス
// =======================================================

#[derive(Debug, Clone)]
struct Member {
    id: u32,
    name: String,
}

type MemberTable = Vec<Member>;

// 名前列に対するB-treeインデックス
type NameIndex = BTreeMap<String, u32>; // 名前はユニークと仮定

// =======================================================
// 2. データセットアップ
// =======================================================

fn setup_data() -> (MemberTable, NameIndex) {
    let mut table = MemberTable::new();
    let mut index = NameIndex::new();

    let members = vec![
        Member { id: 1, name: "Taro".to_string() },
        Member { id: 2, name: "Jiro".to_string() },
        Member { id: 3, name: "Saburo".to_string() },
        Member { id: 4, name: "Shiro".to_string() },
    ];

    for member in members {
        index.insert(member.name.clone(), member.id);
        table.push(member);
    }
    (table, index)
}

// =======================================================
// 3. 検索処理のシミュレーション
// =======================================================

fn simulate_searches(_table: &MemberTable, _index: &NameIndex) {
    println!("\n=======================================================");
    println!("   インデックスが「効く」検索と「効かない」検索");
    println!("=======================================================");

    // --- ケース1: インデックスが効果的に使われる検索 ---
    println!("\n--- ✅ ケース1: インデックスが効く検索 (完全一致) ---");
    println!("SQL: SELECT * FROM members WHERE name = 'Taro'");
    println!("[実行計画のシミュレーション]");
    println!("  -> `name`列のB-treeインデックスを使い、キー'Taro'を直接検索 (O(log N))。");
    println!("  -> 見つかったIDを元に、テーブルへ1回だけアクセスする。非常に高速。");

    // --- ケース2: インデックス列に関数を適用したため、効かなくなる ---
    println!("\n--- ❌ ケース2: インデックスが効かない検索 (列に関数を適用) ---");
    println!("SQL: SELECT * FROM members WHERE UPPER(name) = 'TARO'");
    println!("[実行計画のシミュレーション]");
    println!("  -> `name`列の値に`UPPER`関数を適用する必要があるため、インデックスは使えない。");
    println!("  -> **フルテーブルスキャン**が発生。テーブルの全行に対して`UPPER(name)`を実行し、結果を比較する。低速。");
    println!("  -> [対策] DBによっては関数インデックスを作成するか、検索前にアプリ側で値を大文字に変換する。");

    // --- ケース3: LIKE検索で前方がワイルドカードのため、効かなくなる ---
    println!("\n--- ❌ ケース3: インデックスが効かない検索 (LIKEの前方不一致) ---");
    println!("SQL: SELECT * FROM members WHERE name LIKE '%ro'");
    println!("[実行計画のシミュレーション]");
    println!("  -> `name`列のインデックスは先頭文字からソートされているため、'%ro'のような後方一致検索では使えない。");
    println!("  -> **フルテーブルスキャン**が発生。全行の`name`列をチェックする。低速。");

    // --- ケース4: LIKE検索でも前方が一致していれば、インデックスが効く ---
    println!("\n--- ✅ ケース4: インデックスが効く検索 (LIKEの前方一致) ---");
    println!("SQL: SELECT * FROM members WHERE name LIKE 'S%'");
    println!("[実行計画のシミュレーション]");
    println!("  -> インデックスを使い、キーが'S'で始まる範囲を効率的に検索できる (範囲スキャン)。");
    println!("  -> フルテーブルスキャンよりもずっと高速。");
}

// =======================================================
// 4. 実行
// =======================================================

pub fn run() {
    println!("=======================================================");
    println!("   インデックスが効かないケースのデモ 🚀");
    println!("=======================================================");
    
    let (table, index) = setup_data();

    simulate_searches(&table, &index);

    println!("\n=======================================================");
    println!("   結論");
    println!("=======================================================");
    println!("インデックスを有効に活用するには、検索クエリがインデックスの構造に合うように書く必要があります。");
    println!("特にWHERE句でインデックス列に関数を適用したり、前方不一致のLIKE検索を使うとインデックスが効かなくなるので注意が必要です。");
    println!("=======================================================");
}
