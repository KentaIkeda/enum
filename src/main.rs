// enumキーワードを使用して、列挙型を作成
// 型が違くてもOK!
// enum IpAddressKind {
//     V4(u8, u8, u8, u8),
//     V6(String)
// }

// 奇麗なバージョン
struct IpV4Address {
    
}
struct IpV6Address {
// daily
}

enum IpAddress {
    V4(IpV4Address),
    V6(IpV6Address)
}

fn main() {
    // enumを使用する際には::演算子を使用して、アクセスする
    // IpAddressKind型を受け取る関数にパラメータとして渡せる
    let home = IpAddressKind::V4(127, 0, 0, 1);
    let loopback = IpAddressKind::V6(String::from("::1"));
    route(home);
    route(loopback);
}

fn route(ip: IpAddressKind) -> IpAddressKind {
    ip
}
