#[cfg(test)]
mod tests {

    macro_rules! generate_match_arms {
        ($data:expr;$($x:expr),* $(,)?)=>{
            match $data{
                $(
                    $x => {
                        println!("{} is {}", stringify!($x), $x);
                    }
                )*

                _ => {
                    println!("default");
                }
            }

        }

    }
    #[test]
    fn test_match() {
        let x = 3;
        generate_match_arms!(x;1,2,3,4,5);
    }
}
