#[derive(Debug)]
#[derive(PartialEq)]
pub enum Command
{
    Power(bool,i32),    // [Increase/Decrease] power by [number].
    Missiles(bool,i32), // [Increase/Decrease] missiles by [number].
    Shield(bool),       // Turn [On/Off] the shield.
    Try,                // Try calling pepper.
    Invalid             // [anything else]
}


/**
    Adds functionality to Command enums
    Commands can be converted to strings with the as_str method
    
    Command     |     String format
    ---------------------------------------------------------
    Power       |  /Power (increased|decreased) by [0-9]+%/
    Missiles    |  /Missiles (increased|decreased) by [0-9]+/
    Shield      |  /Shield turned (on|off)/
    Try         |  /Call attempt failed/
    Invalid     |  /Not a command/
**/
impl Command {
    pub fn as_str (&self) -> String {
        match self{
            Command::Power(true, num) => return format!("Power increased by {}%", num),
            Command::Power (false, num) => return format!("Power decreased by {}%", num),
            Command::Missiles(true,num) => return format!("Missiles increased by {}", num), 
            Command::Missiles(false,num) => return format!("Missiles decreased by {}", num),
            Command::Shield(true) => return "Shield turned on".to_string(),
            Command::Shield(false) => return "Shield turned off".to_string(),
            Command::Try => return "Call attempt failed".to_string(),
            Command::Invalid => return "Not a command".to_string(),
        }
    }
}

/**
    Complete this method that converts a string to a command 
    We list the format of the input strings below

    Command     |     String format
    ---------------------------------------------
    Power       |  /power (inc|dec) [0-9]+/
    Missiles    |  /(fire|add) [0-9]+ missiles/
    Shield      |  /shield (on|off)/
    Try         |  /try calling Miss Potts/
    Invalid     |  Anything else
**/

pub fn to_command(s: &str) -> Command {
    let mut vec1:Vec<&str> = s.split_whitespace().collect();
    let mut vec = Vec::new();
    
    for elem in vec1 {
        vec.push(Some(elem));
    }

    match vec[0]{
        Some("power") => {
            let boolean;
            match vec[1]{
                Some("inc") => {boolean = true;},
                Some("dec") => {boolean = false;},
                _ =>  {return Command::Invalid},
            }
            match vec[2]{
                Some(num) => {
                        let num2 = num.parse().unwrap();
                        if vec.len() == 3 {return Command::Power(boolean, num2)} else {return Command::Invalid;}
                },
                _ =>  {return Command::Invalid;},
            }
        }
        Some("fire") => {let boolean = false;
            let num = vec[1].unwrap().parse().unwrap();
            match vec[2]{
                Some("missiles") => {if vec.len() == 3 {return Command::Missiles(boolean, num)} else {return Command::Invalid;}},
                _ => return Command::Invalid,}
        }
        Some("add") => {let boolean = true;
            let num = vec[1].unwrap().parse().unwrap();
            match vec[2]{
                Some("missiles") => {if vec.len() == 3 {return Command::Missiles(boolean, num)} else {return Command::Invalid;}},
                _ => return Command::Invalid,}
        }
        Some("shield") => {
            match vec[1]{
                Some("on") => {if vec.len() == 2 {return Command::Shield(true);} else {return Command::Invalid;}},
                Some("off") => {if vec.len() == 2 {return Command::Shield(false);} else {return Command::Invalid;}},
                _ =>  {return Command::Invalid},
            }
        }
        Some("try") => {
            match vec[1]{
                Some("calling") => {
                                    if (vec[2] == Some("Miss")) && 
                                    (vec[3] == Some("Potts")) && 
                                    (vec.len() == 4) 
                                    {return Command::Try;} else {return Command::Invalid;}
                },
                _ => {return Command::Invalid;}
            }
        }
      _ => {return Command::Invalid}
    }
}
