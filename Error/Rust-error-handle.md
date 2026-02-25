## Types of Rust Error Handle:

1. `panic!`
2. `Option<T>`
3. `Result<T, E>`
4. `?`
5. `Custome Error Type`
6. `Box<dyn Error>`
7. `catch_unwind`

### 2. `Option<T>`

<h5> 
Option<T> is just a an built in enum in rust which looks like this,
</h5>

```rust
enum Option<T> {
    Some(T), // Some Value of Type T
    None  // No Value
}
```

<h5>
What problem it solves?
Lets see with an example:
</h5>

```c
int get_salary(id){
    if exist(id) {
        return db_query_salary(id);
    }else{
        return NULL;
    }
}

int main(){
    int salary = get_salary(102); // But the id does not exist returns NULL 
    // Here we forgot to check `NULL` for `salary`
    printf("Total is %d", salary+2000 ); 
    // And we are adding int with NULL which will crash during run time
}
```

<h5>
So when developer forget to check for NULL or None case it crashes during run time on other languages.
</h5>

<h5>
But in rust solve this by Option<T>. How? lets see the example


```rust
fn get_salary(id: u16) -> Option<u32> {
    if exist(id) {
        Some(db_query_salary(id));
    }else{
        None;
    }
}

int main(){
    let salary = get_salary(102); // But the id exists or not it will return Option type  
    println!("Total salary is {}", salary+2000 ); 
    // Here we can't do that because we can't add Option type with u32, which 
    // will show error even before compiling anything, just during writing the code

    // So to make it run we have to unwrap the Option type and to do that
    // we need to check for None. So the compiler forcing use to check for None/NULL 

    match salary {
        Some(sal) => println!("Total Salary: {}", sal+2000),
        None => println!("Id does not exist"),
    }
}
```


Where there is a chance of failure or getting NULL/None we simply return `Option<T>`

For example here on `Option<T>` is `Option<u32>` so when we try to add 

u32 with Option it does not even compile so we need to unwrap this with `match`

and do to this we must need to check for None or NULL which saves us from run time crash
</h5>

