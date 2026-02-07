use crate::controller::ServerController;

pub enum Menu {
    Main,
    Scope,
    Exit,
}

#[derive(Clone, Copy)]
enum Command {
    StartServer,
    StopServer,
    AddScope,
    ListScopes,
    EnterScope,
    Exit,
    Help,
    Unkown,
}

pub struct CLI {
    next_command: Command,
    last_command: Command,
    message: String,
    info: Option<String>,
    prompt: String,
    menu: Menu,
}

impl CLI {
    pub fn new() -> Self {
        CLI {
            menu: Menu::Main,
            next_command: Command::StartServer,
            last_command: Command::StartServer,
            message: "DHCP server command line interface".to_string(),
            info: None,
            prompt: "Enter command (Enter help for available commands)".to_string(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.message);
        if let Some(ref info) = self.info {
            println!("{}", info);
        }
        println!("{}", self.prompt);
    }

    fn print_help(&self) {
        println!("Available commands:");
        println!("  start");
        println!("  stop");
        println!("  list");
        println!("  enter <scope id>");
        println!("  help");
        println!("  exit");
    }

    pub fn read_input(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "start" => {
                self.next_command = Command::StartServer;
            }
            "stop" => {
                self.next_command = Command::StopServer;
            }
            "add" => {
                self.next_command = Command::AddScope;
            }
            "list" => {
                self.next_command = Command::ListScopes;
            }
            "enter" => {
                self.next_command = Command::EnterScope;
            }
            "exit" => {
                self.next_command = Command::Exit;
            }
            "help" => {
                self.next_command = Command::Help;
            }
            _ => {
                self.next_command = Command::Unkown;
            }
        }
    }

    pub async fn execute_command(&mut self, controller: &mut ServerController) {
        match self.next_command {
            Command::StartServer => {
                match controller.start().await {
                    Ok(_) => println!("Server started"),
                    Err(err) => println!("{err}"),
                }
            }
            Command::StopServer => {
                controller.stop().await;
            }
            Command::AddScope => todo!(),
            Command::ListScopes => {
                controller.server().map(async |server| {
                    let server = server.lock().await;
                    let scopes = server.scopes();
                    let mut scopes_string = String::new();
                    scopes.iter().for_each(|scope| scopes_string.push_str(&format!("{}", scope)));
                    self.info = Some(scopes_string);
                });
            }
            Command::EnterScope => todo!(),
            Command::Exit => {
                self.menu = Menu::Exit;
            }
            Command::Unkown => {
                self.info = Some("Unknown command".to_string());
            }
            Command::Help => self.print_help(),
        }

        self.last_command = self.next_command;
    }

    pub fn menu(&self) -> &Menu {
        &self.menu
    }
}
