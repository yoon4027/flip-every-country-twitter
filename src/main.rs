use std::{path::Path, str::FromStr, time::Duration};

use fuck_every_country_twit::{parse_config, parse_csv};
use rand::{
    distributions::{Distribution, WeightedIndex},
    rngs::OsRng,
};
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

#[tokio::main]
async fn main() {
    let config = parse_config(Path::new("Config.toml")).await.unwrap();
    let countries_op = parse_csv();

    let countries = match countries_op {
        Some(a) => a,
        None => panic!("Oops its done for"),
    };

    let creds = Oauth1aToken::new(
        config.consumer.key,
        config.consumer.secret,
        config.access.key,
        config.access.secret,
    );

    let twitter = TwitterApi::new(creds);

    let mut weights = vec![1; countries.len()];

    weights[79] = 8;

    let rand_weight = WeightedIndex::new(&weights).unwrap();

    loop {
        let chosen = countries.get(rand_weight.sample(&mut OsRng));

        if let Some(count) = chosen {
            let name = count.name.replace(" ", "");

            let text = match celes::Country::from_str(&name) {
                Ok(a) => format!("Fuck #{} {} ({})", count.name, count.emoji, a.alpha2,),
                Err(_) => format!("Fuck #{} {}", count.name, count.emoji),
            };

            if let Err(why) = twitter.post_tweet().text(text.clone()).send().await {
                println!("Failed to send tweet: {:?}", why);
            }

            println!("Tweeted: {text}");
        } else {
            continue;
        }

        tokio::time::sleep(Duration::from_secs(3600)).await;
    }
}
