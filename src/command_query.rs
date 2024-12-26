use tokio::sync::mpsc;

#[derive(Debug, Clone)]
struct CreateUserCommand {
    name: String,
    email: String,
}

#[derive(Debug, Clone)]
struct UserCreatedEvent {
    user_id: i32,
    name: String,
    email: String,
}

fn save_to_write_database(command: &CreateUserCommand) -> i32 {
    println!("Writing to write database: {:?}", command);
    1
}

fn save_to_read_database(user_id: i32, name: String, email: String) {
    println!(
        "Syncing to read database: user_id={}, name={}, email={}",
        user_id, name, email
    );
}

async fn handle_command(command: CreateUserCommand, sender: mpsc::Sender<UserCreatedEvent>) {
    let user_id = save_to_write_database(&command);
    let event = UserCreatedEvent {
        user_id,
        name: command.name,
        email: command.email,
    };
    sender.send(event).await.unwrap();
}

async fn listen_to_events(mut receiver: mpsc::Receiver<UserCreatedEvent>) {
    while let Some(event) = receiver.recv().await {
        save_to_read_database(event.user_id, event.name, event.email);
    }
}

#[tokio::main]
pub async fn run() {
    let (sender, receiver) = mpsc::channel(32);

    let command = CreateUserCommand {
        name: "John Doe".to_string(),
        email: "john.doe@example.com".to_string(),
    };

    let sender = sender.clone();
    tokio::spawn(async move {
        handle_command(command, sender).await;
    });

    tokio::spawn(async move {
        listen_to_events(receiver).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
