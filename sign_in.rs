fn main() {
    let user1 = User{username: String::from("张三"),email: String::from("1"), infomation: String::from("1")};
    add_user(&user1);
    add_email(&user1);
    add_infomation(&user1);
}

struct User {
    username: String,
    email: String,
    infomation: String,
}


fn add_user(user: &User){
    println!("添加用户{}成功！",user.username);
}

fn add_email(user: &User){
    println!("添加邮箱{}成功！",user.email);
}

fn add_infomation(user: &User){
    println!("添加信息{}成功！",user.infomation);
}
