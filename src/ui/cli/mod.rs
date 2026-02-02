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
    EnterScope,
    Exit,
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
            prompt: "> ".to_string(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.message);
        if let Some(ref info) = self.info {
            println!("{}", info);
        }
        println!("{}", self.prompt);
    }

    pub fn print_help() {
        println!("Available commands:");
        println!("  start_server");
        println!("  stop_server");
        println!("  list_scopes");
        println!("  list_clients");
        println!("  list_leases");
        println!("  list_options");
        println!("  exit");
    }

    pub fn read_input(&mut self) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "start_server" => {
                self.next_command = Command::StartServer;
            }
            "stop_server" => {
                self.next_command = Command::StopServer;
            }
            "list_scopes" => {
                self.next_command = Command::EnterScope;
            }
            "exit" => {
                self.next_command = Command::Exit;
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
            Command::EnterScope => todo!(),
            Command::Exit => {
                self.menu = Menu::Exit;
            }
            Command::Unkown => {
                self.info = Some("Unknown command".to_string());
            }
        }

        self.last_command = self.next_command;
    }

    pub fn menu(&self) -> &Menu {
        &self.menu
    }
}
