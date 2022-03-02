use std::future::Future;

// Function first parameter is an async function
// The second is the number of retries
// All others arguments are passed to the async function
// The function name is retry
pub async fn run_async<'a, Fut, F, Args, Out>(
    function: Fut,
    retries: u64,
    args: &'a Args,
) -> Result<Out, String>
where
    Fut: Fn(&'a Args) -> F,
    F: Future<Output = Result<Out, String>>,
{
    let mut count = 0;
    let mut error_string: String = String::new();

    while count < retries {
        let executed_func = function(args);

        let result = executed_func.await;
        if result.is_ok() {
            return Ok(result.unwrap());
        } else {
            error_string = result.err().unwrap();
        }

        count += 1;
    }

    Err(format!(
        r#"The function failed after all retries with: "{}""#,
        error_string
    ))
}

// Same but run_sync
// pub fn run_sync<'a, Fut, Args, Out>(
//     function: Fut,
//     retries: u64,
//     args: &'a Args,
// ) -> Result<Out, String>
// where
//     Fut: Fn(&'a Args) -> Result<Out, String>,
// {
//     let mut count = 0;
//     while count < retries {
//         let result = function(args);

//         if result.is_ok() {
//             return Ok(result.unwrap());
//         }

//         count += 1;
//     }

//     Err("The function failed after all retries".to_string())
// }
