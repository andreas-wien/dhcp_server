use std::io::{ self, IsTerminal };

use dhcp_server_lib::ui::cli::{ CLI, Menu };
use dhcp_server_lib::controller::ServerController;

#[tokio::main]
async fn main() {
    let mut controller = ServerController::new();

    if io::stdin().is_terminal() {
        let mut cli = CLI::new();
        loop {
            if matches!(cli.menu(), Menu::Exit) {
                break;
            }

            cli.print();
            cli.read_input();
            cli.execute_command(&mut controller).await;
        }
    } else {
        todo!("implement gui")
    }
}
