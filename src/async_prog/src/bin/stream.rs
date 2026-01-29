use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values: Vec<i32> = (1..=10).collect();
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    })
}
