use chrono::prelude::*;
use dgraph_tonic::{Client, Mutate, Mutation, Operation, Query};
use maplit::hashmap;
use serde::{Deserialize, Serialize};

async fn drop_all(client: &Client) {
    let op = Operation {
        drop_all: true,
        ..Default::default()
    };
    client.alter(op).await.expect("dropped all");
}

async fn set_schema(client: &Client) {
    let schema = r#"
        name: string @index(exact) .
        age: int .
        married: bool .
        loc: geo .
        dob: datetime .
        friend: [uid] @reverse .
    "#
    .into();
    let op = Operation {
        schema,
        ..Default::default()
    };
    client.alter(op).await.expect("set schema");
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    #[serde(rename = "type", alias = "type")]
    t: String,
    coordinates: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Friend {
    name: String,
    age: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct School {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    uid: String,
    name: String,
    age: u8,
    married: bool,
    loc: Location,
    dob: DateTime<Utc>,
    friend: Vec<Friend>,
    school: Vec<School>,
}

#[derive(Debug, Serialize, Deserialize)]
struct All {
    all: Vec<Person>,
}

struct MyMutatedTxn<M: Mutate> {
    mutated_txn: M,
}

impl<M: Mutate> MyMutatedTxn<M> {
    async fn create_data(self) {
        let p = Person {
            uid: "_:alice".into(),
            name: "Alice".into(),
            age: 26,
            married: true,
            loc: Location {
                t: "Point".into(),
                coordinates: vec![1.1, 2.0],
            },
            dob: Utc.ymd(1980, 1, 1).and_hms(23, 0, 0),
            friend: vec![
                Friend {
                    name: "Bob".into(),
                    age: 24,
                },
                Friend {
                    name: "Charlie".into(),
                    age: 29,
                },
            ],
            school: vec![School {
                name: "Crown Public School".into(),
            }],
        };
        let mut mu = Mutation::new();
        mu.set_set_json(&p).expect("JSON");
        let mut txn = self.mutated_txn;
        let response = txn.mutate(mu).await.expect("mutated");
        txn.commit().await.expect("committed");
        println!(
            "Created person named \"Alice\" with uid = {}",
            response.uids.get("alice").to_owned().expect("uid")
        );
        println!();
        println!("All created nodes (map from blank node names to uids):");
        for (uid, key) in response.uids {
            println!("${} => ${}", key, uid);
        }
        println!();
    }
}

struct MyQueryTxn<Q: Query> {
    read_only_txn: Q,
}

impl<Q: Query> MyQueryTxn<Q> {
    async fn query_data(&mut self) {
        let query = r#"
            query all($a: string) {
                all(func: eq(name, $a)) {
                    uid
                    name
                    age
                    married
                    loc
                    dob
                    friend {
                        name
                        age
                    }
                    school {
                        name
                    }
                }
            }
        "#;
        let vars = hashmap! {"$a" => "Alice"};
        let resp = self
            .read_only_txn
            .query_with_vars(query, vars)
            .await
            .expect("resp");
        let ppl: All = resp.try_into().expect("JSON");
        println!("Number of people named \"Alice\": {}", ppl.all.len());
        for person in ppl.all {
            println!("{:#?}", person);
        }
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new(vec!["http://localhost:19080"]).expect("connected client");
    drop_all(&client).await;
    set_schema(&client).await;
    let my_txn = MyMutatedTxn {
        mutated_txn: client.new_mutated_txn(),
    };
    my_txn.create_data().await;
    let mut my_txn = MyQueryTxn {
        read_only_txn: client.new_read_only_txn(),
    };
    my_txn.query_data().await;
    println!("DONE!");
}
