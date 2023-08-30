use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::time::Duration;
use std::{env, io, thread};

use crate::finished_game::board_contract::BoardContract;
use crate::finished_game::color::Color;
use crate::finished_game::game_state::GameState;
use crate::i18n::ChessTerm;
use crate::square::Square;

struct Game<'a> {
    board: Box<dyn BoardContract + 'a>,
    turn: Color,
    player: Color,
    stream: TcpStream,
    state: GameState,
}

impl<'a> Game<'a> {
    fn new(board: impl BoardContract + 'a, player: Color, stream: TcpStream) -> Self {
        Game {
            board: Box::new(board),
            turn: Color::White,
            state: GameState::Playing,
            player,
            stream,
        }
    }

    fn play(&mut self) {
        self.board.print(None);
        self.print_turn();
        'game: loop {
            if self.board.is_check(self.turn) {
                match self.board.is_checkmate(self.turn) {
                    false => println!("{} konge står i sjakk!", self.turn.print_capitalised()),
                    true => {
                        println!("{} konge er sjakkmatt!", self.turn.print_capitalised());
                        self.state = GameState::Checkmate(self.turn);
                        break 'game;
                    }
                }
            }

            let Some(position) = self.get_piece() else { break; };
            let legal_squares = self.board.get_legal_squares(&position);
            if legal_squares.is_empty() {
                if self.is_your_turn() {
                    println!("Inga lovlege trekk for denne brikka!");
                }
                continue;
            }
            if self.is_your_turn() {
                self.board.print(Some(&legal_squares));
            }

            // maybe change this to normal if else block?
            let Some(position_to_move_to) = self.get_move(&position, legal_squares) else { break };
            match position_to_move_to {
                position_to_move_to if position_to_move_to == position => {
                    if self.is_your_turn() {
                        println!("Du satte brikka tilbake.");
                        self.board.print(None);
                    }
                    continue;
                }
                position_to_move_to
                    if self.board.get_square_color(&position_to_move_to) == Some(self.turn.opposite()) =>
                {
                    let attacking = self.board.get_piece_type(&position).translate();
                    let attacked = self.board.get_piece_type(&position_to_move_to).translate();
                    println!(
                        "{} frå {} fangar {} på {}",
                        attacking,
                        position.as_string().unwrap(),
                        attacked,
                        position_to_move_to.as_string().unwrap()
                    );
                    self.board.move_piece(&position, position_to_move_to);
                }
                position_to_move_to => {
                    self.board.move_piece(&position, position_to_move_to);
                }
            }

            self.board.print(None);
            self.next_turn();
            self.print_turn();
        }
    }

    fn next_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    fn print_turn(&self) {
        println!("{} sin tur", self.turn.print_capitalised())
    }

    fn get_piece(&mut self) -> Option<(u8, u8)> {
        while self.state == GameState::Playing {
            if self.is_your_turn() {
                print!("Vel ei brikke å flytte: ");
                io::stdout().flush().unwrap();
            }
            if let Some(position) = self.select_square() {
                match self.board.get_square_color(&position) {
                    Some(color) if color == self.turn => {
                        return Some(position);
                    }
                    Some(_) if self.is_your_turn() => {
                        println!("Du valde {}, men det er {} sin tur", self.turn.opposite(), self.turn);
                    }
                    None if self.is_your_turn() => {
                        println!("Det er inga brikke i feltet du valde");
                    }
                    _ => {}
                }
            }
        }
        None
    }

    fn get_move(&mut self, position: &(u8, u8), mut legal_squares: HashSet<(u8, u8)>) -> Option<(u8, u8)> {
        while self.state == GameState::Playing {
            if self.is_your_turn() {
                print!("Vel eit felt å flytte til: ");
            }
            // Add the actual piece's own position as a legal move, as this means you unselect it
            legal_squares.insert(*position);
            io::stdout().flush().unwrap();
            match self.select_square() {
                Some(square) if legal_squares.contains(&square) => return Some(square),
                Some(_) => {
                    if self.is_your_turn() {
                        println!("Feltet du valte er ikkje lov å flytte til!")
                    }
                }
                _ => continue,
            }
        }
        None
    }

    fn is_your_turn(&self) -> bool {
        self.turn == self.player
    }

    /// Read chess square name from stdin or TcpStream and return position
    /// For example `"a8" -> (0, 0)`
    fn select_square(&mut self) -> Option<(u8, u8)> {
        let mut square = String::new();
        if self.is_your_turn() {
            io::stdin().read_line(&mut square).unwrap();
            self.stream.write_all(square.as_bytes()).unwrap();
        } else {
            let mut buffer = [0; 512];
            let bytes_read = self.stream.read(&mut buffer).expect("Failed to read from socket.");
            let message = String::from_utf8_lossy(&buffer[..bytes_read]);
            square = message.to_string();
        }

        square.retain(|c| !c.is_ascii_whitespace());

        if square == "x" {
            self.exit_game();
            return None;
        }

        square.as_str().as_u8().ok()
    }

    pub fn exit_game(&mut self) {
        self.state = GameState::Quit;
    }
}

fn listen_for_connection() -> io::Result<TcpStream> {
    let addr = SocketAddr::new(get_local_ip(), 9001);
    let tcp_listener = TcpListener::bind(addr)?;
    println!("Listening for connections on {} on port 9001", get_local_ip());
    let (tcp_stream, _) = tcp_listener.accept()?;
    println!("New connection, {:?}", tcp_stream);
    Ok(tcp_stream)
}

// TODO addr from CLI arg as parameter
fn connect_to_addr() -> io::Result<TcpStream> {
    let socket_addr = SocketAddr::new(get_local_ip(), 9001);
    loop {
        match TcpStream::connect(socket_addr) {
            Ok(stream) => return Ok(stream),
            Err(err) => {
                if err.kind() != io::ErrorKind::ConnectionRefused {
                    return Err(err);
                }
                thread::sleep(Duration::from_secs(1));
                println!("Retrying connection to {}...", socket_addr.ip());
            }
        }
    }
}

fn get_local_ip() -> IpAddr {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    let remote_addr: SocketAddr = "8.8.8.8:80".parse().expect("Failed to parse remote address");
    socket.connect(remote_addr).expect("Failed to connect socket");
    let local_addr = socket.local_addr().expect("Failed to retrieve local address");
    local_addr.ip()
}

enum RunMode {
    Server,
    Client,
}

pub fn main(board: impl BoardContract) {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Must specify --server or --client as second argument");
        return;
    }

    let run_mode = if &args[2][..] == "--server" {
        RunMode::Server
    } else {
        RunMode::Client
    };

    let tcp_stream = match run_mode {
        RunMode::Server => listen_for_connection(),
        RunMode::Client => connect_to_addr(),
    }
    .expect("Failed to obtain TcpStream");

    let mut game = match run_mode {
        RunMode::Server => Game::new(board, Color::White, tcp_stream),
        RunMode::Client => Game::new(board, Color::Black, tcp_stream),
    };

    game.play();
}
