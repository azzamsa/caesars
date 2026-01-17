mod schema {
    cynic::use_schema!("tests/schema.graphql");
}

#[cynic::schema_for_derives(file = "tests/schema.graphql", module = "schema")]
pub mod queries {

    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "EncryptArguments")]
    #[allow(dead_code)]
    pub struct EncryptQuery {
        #[arguments(plain: $plain, rotation : $rotation)]
        pub encrypt: Encrypt,
    }

    #[derive(cynic::QueryVariables, Debug)]
    #[allow(dead_code)]
    pub struct EncryptArguments {
        pub plain: String,
        pub rotation: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[allow(dead_code)]
    pub struct Encrypt {
        pub secret: String,
        pub rotation: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "DecryptArguments")]
    #[allow(dead_code)]
    pub struct DecryptQuery {
        #[arguments(secret : $secret, rotation: $rotation)]
        pub decrypt: Decrypt,
    }

    #[derive(cynic::QueryVariables, Debug)]
    #[allow(dead_code)]
    pub struct DecryptArguments {
        pub secret: String,
        pub rotation: i32,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[allow(dead_code)]
    pub struct Decrypt {
        pub plain: String,
        pub rotation: i32,
    }
}
