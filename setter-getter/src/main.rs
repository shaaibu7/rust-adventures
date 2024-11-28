// This program practices getter and setter
// operations in learning using struct to 
// store, and retrieve data
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
    location: String,
}

impl User {
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
}

fn main() {
    // instantiating a new user
    let mut user: User = User {
        name: "Shaaibu Suleiman".into(),
        age: 21,
        email: "shaaibu@gmail.com".into(),
        location: "Washington DC".into()
    };

     println!("The first user is {:?}", user);

     // Update the details of existing user

     user.set_email("suleiman@gmail.com".into());

     println!("The updated has the following details {:?}", user);
}