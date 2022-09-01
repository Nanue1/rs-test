use dialoguer::{Confirm, FuzzySelect, Editor, Input, MultiSelect, Password, Sort};
use anyhow::Result;
use dialoguer::console::Term;
use dialoguer::theme::ColorfulTheme;

fn main() {
    println!("Hello, world!");
    interact_action()
}

fn interact_action() {
    // confirmation_prompt().unwrap();
    // fuzzy_select().unwrap();
    // editor().unwrap();
    // input().unwrap();
    // multiple_select().unwrap()
    // password().unwrap();
    sort().unwrap();

}

// Confirmation prompts
#[allow(dead_code)]
fn confirmation_prompt() -> Result<()> {
    if Confirm::new().with_prompt("do you want to continue?").interact()?{
        println!("continue.");
    }else {
        println!("no continue");
    }
    return Ok(())
}

#[allow(dead_code)]
fn editor() -> Result<()> {
    if let Some(rv) = Editor::new().edit("Enter a commit message").unwrap(){
        println!("Your message:");
        println!("{}", rv);
    }else {
        println!("Abort!");
    }
    Ok(())
}

// fuzzy select
#[allow(dead_code)]
fn fuzzy_select() -> Result<()> {
    let items = vec!["Item 1", "item 2"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    match selection {
        Some(index) => println!("User selected item : {}", items[index]),
        None => println!("User did not select anything")
    }
    Ok(())
}

// Input validation
#[allow(dead_code)]
fn input() -> Result<()>{
    let input : String = Input::new()
        .with_prompt("Tea or coffee?")
        .with_initial_text("Yes")
        .default("No".into())
        .interact_text()?;
    println!("input :{}",input);
    Ok(())
}

#[allow(dead_code)]
fn multiple_select() -> Result<()>{
    let items = vec!["Option 1", "Option 2"];
    let chosen : Vec<usize> = MultiSelect::new()
        .items(&items)
        .interact()?;
    println!("chosen: {},",chosen[0]);
    Ok(())
}

#[allow(dead_code)]
fn password()-> Result<()> {
    let password = Password::new().with_prompt("New Password")
        .with_confirmation("Confirm password", "Passwords mismatching")
        .interact()?;
    println!("Length of the password is: {}", password.len());
    Ok(())
}

#[allow(dead_code)]
fn sort() -> Result<()> {
    let items_to_order = vec!["Item 1", "Item 2", "Item 3"];
    let ordered = Sort::new()
        .with_prompt("Order the items")
        .items(&items_to_order)
        .interact()?;
    for x in ordered {
        println!("{}", x);
    }
    Ok(())
}