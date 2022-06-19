pub struct Config { // Config struct for the identic program including a list of paths to compare, a flag for comparing content and a flag for comparing name.
    pub paths: Vec<String>,
    pub name: bool,
    pub content: bool,
}
impl Config {
    // Creates a new Config struct from the command line arguments.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let mut name = false;
        let mut content = false;
        let mut paths = Vec::new();
        
        for i in &args[1..]{
            if i.eq("-n"){
                name = true;
            }
            else if i.eq("-c"){
                content = true;
            }
            else{
                paths.push(i.clone())
            }
        }
        Ok(Config {
            paths,
            name,
            content,
        })
    }
}