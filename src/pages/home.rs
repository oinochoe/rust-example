// src/pages/home.rs
use yew::prelude::*;

struct Product {
    id: i32,
    name: String,
    description: String,
    image: String,
    price: f64
}

struct State {
    products: Vec<Product>,
}

pub struct Home {
    state: State,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {

        let products: Vec<Product> = vec![
            Product {
                id: 1,
                name: "Yeongmin".to_string(),
                description: "Yegonmin is smilar to irang".to_string(),
                image: "/products/apple.png".to_string(),
                price: 3.65,
            },
            Product {
                id: 2,
                name: "Irang".to_string(),
                description: "Irang is smilar to yeongmin".to_string(),
                image: "/products/banana.png".to_string(),
                price: 7.99,
            }
        ];

        Self {
            state: State {
                products,
            },
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let products: Vec<Html> = self
            .state
            .products
            .iter()
            .map(|product: &Product| {
                html! {
                    <div>
                        <img src={&product.image} />
                        <div>{&product.name}</div>
                        <div>{"$"}{&product.price}</div>
                    </div>
                }
            })
            .collect();

        html! { <span>{products}</span> }
    }
}