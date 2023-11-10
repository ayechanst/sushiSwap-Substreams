use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("uniswapv3factory.json", "abis/uniswapv3factory.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("src/abi/uniswapv3factory.rs")
}
