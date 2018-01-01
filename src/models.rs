/// This is the model module

#[derive(Serialize, Deserialize, Queryable)]
struct Message {
    text: String,
}
