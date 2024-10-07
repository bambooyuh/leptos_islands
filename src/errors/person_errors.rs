use thiserror::Error;

// MOST OF APP / NON LEPTOS SERVER FUNCTION WORLD
// thiserror crate proc macros enable Display on each enum variant and the ability to customize an error message.
// use this for all errors outside of leptos server functions! Because leptos server functions needs pure &str! To display an error message.
#[derive(Error, Debug)]
pub enum MyError {
    #[error("member not found")]
    PersonNotFound,
    #[error("connection timeout")]
    ConnectionTimeout,
    #[error("failed to fetch members")]
    PersonsFetchFailure,
    #[error("failed to update member")]
    PersonUpdateFailure,
    #[error("failed to create member")]
    PersonCreationFailure,
    #[error("failed to delete member")]
    PersonDeleteFailure
}

// a custom label for error messages that are of type String
// an alternative approach could using tuple struct as a wrapper with a String inside


// LEPTOS SERVER FUNCTION WORLD
// In order to return error messages within a Leptos server function it needs to a &str
// the enum MyError with thiserror crate's &str annotation does not work, so the enum variant needs to be converted to pure &str
// which is what happens below.
pub fn create_myerror_message(person_error: MyError) -> String {
    let message = match person_error {
        MyError::PersonNotFound => "member not found".to_string(),
        MyError::ConnectionTimeout => "connection timeout".to_string(),
        MyError::PersonsFetchFailure => "failed to fetch members".to_string(),
        MyError::PersonUpdateFailure => "failed to update member".to_string(),
        MyError::PersonCreationFailure => "failed to create member".to_string(),
        MyError::PersonDeleteFailure => "failed to delete member".to_string(),
    };

    message
}

/*
NOTES:

When you use the thiserror crate, you annotate your error enum variants with #[error("...")]. 
This is a procedural macro that automatically generates implementations of the Display and Error traits for your error enum.

Specifically, it:

Generates the Display implementation: The string provided in the #[error(...)] attribute will be used to generate the error message when the error is printed or displayed.
Implements std::error::Error: This makes the error compatible with Rust’s error handling ecosystem, allowing it to be used with ? and other error-handling constructs.





Question:
alternative approach I would like you feedback on.
What if, instead of just a plain enum variant...the enum MyError variants were tuple structs that held error messages. 
So inside, server functions you could just write PersonNotFound.0 to get access to the &str that serverfn::args() needs?

The downside I think is that using the MyError variants outside of server functions means I'm returning two messages?? 
the this error message and the message wrapped inside the tuple struct?

ChatGPT said:
Your alternative approach of using tuple structs for MyError variants does make sense in certain contexts, 
but as you noted, it comes with trade-offs. Let’s break it down.

1. Tuple Struct Approach:
You could define your MyError enum like this:

#[derive(Debug, Error)]
pub enum MyError {
    #[error("member not found")]
    PersonNotFound(&'static str),
    
    #[error("failed to update member")]
    PersonUpdateFailure(&'static str),
    
    #[error("failed to create member")]
    PersonCreationFailure(&'static str),
}
Now, each variant holds a message, and in your server function, you can directly access the tuple’s contents:

rust
Copy code
Err(ServerFnError::Args(MyError::PersonNotFound.0))
This would be handy since you wouldn’t need to manually convert the MyError to a &str inside the server function.

2. Pros:
Ease of use in server functions: You directly access the error message with .0, so no need for an extra ResponseErrorTrait or conversion method. 
This simplifies your error handling within server functions.
Single source of truth: You store the error message inside the enum variant, so there's no need for external mappings or lookups.
3. Cons/Downsides:
Redundant error messages outside of server functions: Since thiserror is still deriving the Display trait for MyError, 
you might end up with redundant error messages. The one defined by thiserror (via the #[error(...)] attribute) and the one inside the tuple struct will both exist, and you could potentially return two different error messages:

From thiserror: The derived message, e.g., "failed to update member".
From the tuple struct: The string stored in the tuple, e.g., "User couldn't be updated".
For example, using .to_string() on the error outside a server function could still return the formatted message from the thiserror annotation, 
leading to potential duplication.

Clarity and complexity: By storing the message both in the thiserror annotation and in the tuple struct, 
you might create confusion about which message should be used where, especially as the project grows. 
This could lead to inconsistencies in error handling.

4. Potential Resolution:
If you prefer the tuple struct approach but want to avoid redundant messages, 
you could remove the #[error(...)] annotation from thiserror and rely entirely on the tuple struct for the error messages. 
This would centralize the message into one place:

#[derive(Debug)]
pub enum MyError {
    PersonNotFound(&'static str),
    PersonUpdateFailure(&'static str),
    PersonCreationFailure(&'static str),
}
Then, access .0 everywhere for the actual error message. However, this means you'd lose the convenience of thiserror for auto-implementing the Display trait and handling common error scenarios.

Conclusion:
Tuple Struct Approach is more streamlined for server functions where you need direct access to error strings.
However, it introduces complexity and possible redundancy outside of server functions.
Using your current setup with ResponseErrorTrait is likely more maintainable in the long run, 
as it keeps the structured error handling intact and avoids the risk of message duplication.
You could use tuple structs if your error handling is lightweight and mostly focused on server functions. 
Otherwise, the current pattern with ResponseErrorTrait offers better separation of concerns.


*/