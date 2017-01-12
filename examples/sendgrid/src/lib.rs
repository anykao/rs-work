#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;


use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debut)]
pub struct SGMailV3 {
    From             &Email,
    Subject          String ,
    Personalizations Vec<Personalization>,
    Content          Vec<&Content>,
    Attachments      Vec<&Attachment>,
    TemplateID       String,
    Sections         Hashmap<String, String>,
    Headers          Hashmap<String, String>,
    Categories       Vec<String>,
    CustomArgs       Hashmap<String, String>,
    SendAt           i32,
    BatchID          String,
    IPPoolID         String,
    ReplyTo          *Email,
}

#[derive(Serialize, Deserialize, Debut)]
pub struct Personalization {
    To            Vec<&Email>,
    CC            Vec<&Email>,
    BCC           Vec<&Email>,
    Subject       String,
    Headers       Hashmap<String, String>,
    Substitutions Hashmap<String, String>,
    CustomArgs    Hashmap<String, String>,
    Categories    Vec<String>,
    SendAt        i32,
}

#[derive(Serialize, Deserialize, Debut)]
pub struct Email  {
    Name    String,
    Address String ,
}

#[derive(Serialize, Deserialize, Debut)]
pub struct Content  {
    Type  String,
    Value String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
