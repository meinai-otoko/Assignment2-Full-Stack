use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 9.99,
            description: "Buy this laptop start coding and add misery to your life ".to_string(),
            image: "/catnip.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Iphone".to_string(),
            price: 6.99,
            description: "Buy this Iphone and get addicted to doom scrolling".to_string(),
            image: "/squid.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Laptop".to_string(),
            price: 12.99,
            description: "Buy this laptop please or else my company will be bankrupt.".to_string(),
            image: "/catnip.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Playstation".to_string(),
            price: 11.99,
            description: "Instead of spending money on your girl buy this playsation and have a gaming sesh with your buddies.".to_string(),
            image: "/ocean.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Another Laptop".to_string(),
            price: 8.99,
            description: "This one is definately better than the last one.".to_string(),
            image: "/catnip.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Samsung phone".to_string(),
            price: 14.99,
            description: "Although they are in lot of trouble due to stagnent innovations and shitty chips you need to buy this one.".to_string(),
            image: "/tug.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Another Iphone".to_string(),
            price: 19.99,
            description: "If you buy this stevejobs is gonna kiss your hands in afterlife and tell you a bedtime story.".to_string(),
            image: "/catnip.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Playstaion pro max".to_string(),
            price: 7.99,
            description: "This has additional modded games thats gonna make you forget the pain about that one girl that never texts you back.".to_string(),
            image: "/ocean.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Apple watch".to_string(),
            price: 3.99,
            description: "Buy this along with the iphones and you can watch steve jobs actually innovating something instead of mooching of his colleauges.".to_string(),
            image: "/crabby.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Another Samsung phone".to_string(),
            price: 5.99,
            description: "You need to buy this because they really need your money to make more useless chips, i mean common i can iron my shirt with this phone when i am gaming.".to_string(),
            image: "/tug.jpg".to_string()
        }
    ]
}