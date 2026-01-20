//! # Tic Tac Toe
//!
//! A simple Tic Tac Toe game built with GPUI, a GPU-accelerated UI framework.
//!
//! ## Features
//! - Two-player gameplay (X and O)
//! - Win detection for rows, columns, and diagonals
//! - Draw detection
//! - Visual feedback with colored cells
//! - Reset button to play again

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, ElementId, MouseButton,
    Window, WindowBounds, WindowOptions,
};

/// Represents a player in the game.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    X,
    O,
}

/// Represents the state of a cell on the board.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    /// The cell is empty and available for play.
    Empty,
    /// The cell is occupied by a player.
    Player(Player),
}

/// The main game state for Tic Tac Toe.
#[derive(Debug)]
struct TicTacToe {
    /// 3x3 game board represented as a 2D array.
    board: [[Cell; 3]; 3],
    /// The player whose turn it is.
    current_player: Player,
    /// Whether the game has ended (win or draw).
    game_over: bool,
    /// The winner of the game, if any.
    winner: Option<Player>,
}

impl TicTacToe {
    /// Creates a new game with an empty board and X as the starting player.
    fn new() -> Self {
        Self {
            board: [[Cell::Empty; 3]; 3],
            current_player: Player::X,
            game_over: false,
            winner: None,
        }
    }

    /// Attempts to make a move at the specified position.
    ///
    /// The move is only made if the game is not over and the cell is empty.
    /// After a valid move, checks for a winner or draw and switches players.
    fn make_move(&mut self, row: usize, col: usize) {
        if self.game_over || self.board[row][col] != Cell::Empty {
            return;
        }

        self.board[row][col] = Cell::Player(self.current_player);

        if self.check_winner(self.current_player) {
            self.game_over = true;
            self.winner = Some(self.current_player);
        } else if self.check_draw() {
            self.game_over = true;
        } else {
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }
    }

    /// Checks if the specified player has won the game.
    ///
    /// Checks all rows, columns, and both diagonals for three in a row.
    fn check_winner(&self, player: Player) -> bool {
        // Check rows
        for row in 0..3 {
            if self.board[row][0] == Cell::Player(player)
                && self.board[row][1] == Cell::Player(player)
                && self.board[row][2] == Cell::Player(player)
            {
                return true;
            }
        }

        // Check columns
        for col in 0..3 {
            if self.board[0][col] == Cell::Player(player)
                && self.board[1][col] == Cell::Player(player)
                && self.board[2][col] == Cell::Player(player)
            {
                return true;
            }
        }

        // Check main diagonal (top-left to bottom-right)
        if self.board[0][0] == Cell::Player(player)
            && self.board[1][1] == Cell::Player(player)
            && self.board[2][2] == Cell::Player(player)
        {
            return true;
        }

        // Check anti-diagonal (top-right to bottom-left)
        if self.board[0][2] == Cell::Player(player)
            && self.board[1][1] == Cell::Player(player)
            && self.board[2][0] == Cell::Player(player)
        {
            return true;
        }

        false
    }

    /// Checks if the game is a draw (all cells filled with no winner).
    fn check_draw(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.board[row][col] == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    /// Resets the game to its initial state.
    fn reset(&mut self) {
        self.board = [[Cell::Empty; 3]; 3];
        self.current_player = Player::X;
        self.game_over = false;
        self.winner = None;
    }
}

impl Render for TicTacToe {
    /// Renders the game UI including the status, board, and reset button.
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Build the game board rows
        let mut rows: Vec<_> = Vec::new();
        for row in 0..3 {
            let mut cells: Vec<_> = Vec::new();
            for col in 0..3 {
                cells.push(self.render_cell(row, col, cx));
            }
            rows.push(div().flex().gap_2().children(cells));
        }

        // Create the reset button (shown only when game is over)
        let reset_button = div()
            .id("reset-button")
            .mt_4()
            .px_4()
            .py_2()
            .bg(rgb(0x4caf50))
            .text_color(rgb(0xffffff))
            .text_lg()
            .cursor_pointer()
            .hover(|el| el.bg(rgb(0x45a049)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _event, _window, _cx| {
                    this.reset();
                }),
            )
            .child("Play Again");

        let game_over = self.game_over;

        // Main container
        div()
            .flex()
            .flex_col()
            .gap_4()
            .bg(rgb(0x2d2d2d))
            .size_full()
            .justify_center()
            .items_center()
            .p_4()
            .child(
                // Status text showing current player or game result
                div()
                    .text_2xl()
                    .text_color(rgb(0xffffff))
                    .child(self.get_status_text()),
            )
            .child(
                // Game board grid
                div().flex().flex_col().gap_2().children(rows),
            )
            .when(game_over, |el| el.child(reset_button))
    }
}

impl TicTacToe {
    /// Renders a single cell of the game board.
    ///
    /// Each cell displays X, O, or is empty, with appropriate coloring
    /// and hover effects for interactive feedback.
    fn render_cell(&self, row: usize, col: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let cell_content = match self.board[row][col] {
            Cell::Empty => "",
            Cell::Player(Player::X) => "X",
            Cell::Player(Player::O) => "O",
        };

        // Color scheme: gray for empty, red for X, blue for O
        let cell_color = match self.board[row][col] {
            Cell::Empty => rgb(0x404040),
            Cell::Player(Player::X) => rgb(0xff6b6b),
            Cell::Player(Player::O) => rgb(0x4dabf7),
        };

        let is_empty = self.board[row][col] == Cell::Empty && !self.game_over;

        div()
            .id(ElementId::Name(format!("cell-{}-{}", row, col).into()))
            .w(px(100.0))
            .h(px(100.0))
            .bg(cell_color)
            .border_1()
            .border_color(rgb(0x000000))
            .flex()
            .justify_center()
            .items_center()
            .text_2xl()
            .text_color(rgb(0xffffff))
            .cursor_pointer()
            .when(is_empty, |el| el.hover(|el| el.bg(rgb(0x505050))))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _event, _window, _cx| {
                    this.make_move(row, col);
                }),
            )
            .child(cell_content)
    }

    /// Returns the status text to display above the board.
    ///
    /// Shows the winner, draw message, or current player's turn.
    fn get_status_text(&self) -> String {
        if self.game_over {
            match self.winner {
                Some(Player::X) => "Player X Wins!".to_string(),
                Some(Player::O) => "Player O Wins!".to_string(),
                None => "It's a Draw!".to_string(),
            }
        } else {
            format!(
                "Current Player: {}",
                match self.current_player {
                    Player::X => "X",
                    Player::O => "O",
                }
            )
        }
    }
}

/// Application entry point.
///
/// Creates a 400x500 window centered on the screen and initializes the game.
fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(400.0), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| TicTacToe::new()),
        )
        .unwrap();
    });
}
