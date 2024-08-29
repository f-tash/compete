use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let  first=n-k;

    let ans=a.iter().cycle().skip(first).take(n).map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    
    println!("{}",ans);
}