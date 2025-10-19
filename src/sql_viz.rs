use sql2viz::vizcreate;

pub fn run() {
    let query = "SELECT 'A' as x, 10 as y UNION ALL SELECT 'B', 20";
    vizcreate(query.to_string()).unwrap();
}