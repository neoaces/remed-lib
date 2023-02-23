use chrono::prelude::*;
// Create a struct that will represent the clients
enum Membership {
    MemberAuthority,
    MemberRegular,
    NonMember
}
struct Client {
    first_name: String,
    last_name: String,
    member_join_date: DateTime<Utc>,
    membership: Membership,
}


fn main() {
    println!("Hello, world!");
    
}