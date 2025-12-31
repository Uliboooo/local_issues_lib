use local_issues_lib::{
    self, Comment, Issue, Issues,
    user::{User, Users},
};

fn main() {
    let mut issues = Issues::new();
    let new_user = User::new("uliboooo", "uliboooo@protonmail.com");

    let mut users = Users::new();
    users.add_user(new_user);
    let du = users.get(0).unwrap();

    let i1 = Issue::new("test", du.clone());

    issues.add_new_issue(i1);

    println!("{issues:?}\n\n");

    let i2 = Issue::new("sec", du.clone());
    issues.add_new_issue(i2);

    let got_lst = issues.get_list();
    println!("got list: {got_lst:?}\n\n");

    let got_i1 = issues.get_mut(0).unwrap();
    got_i1.close_as_cmp();

    // let got_lst = issues.get_list();
    // println!("got list when cmped i1: {got_lst:?}");

    let new_comment = Comment::new("fix", du.clone());

    got_i1.comment(new_comment);

    println!("{:?}", issues.get_list())
}
