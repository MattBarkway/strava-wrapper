macro_rules! define_filter {
    ($name:ident { $($field_name:ident : $field_type:ty),* $(,)? }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            token: String,
            $(pub $field_name: $field_type),*
        }

        impl $name {
            pub fn new(token: String, $($field_name: $field_type),*) -> Self {
                Self {
                    token,
                    $($field_name),*
                }
            }

            pub fn get_token(&self) -> &String {
                &self.token
            }
        }
    };
}

define_filter! {
    OtherFilter {
        other_field: i32,
        additional_field: bool,
    }
}
