pub struct AnsiEscapeCodes;

impl AnsiEscapeCodes {
    // Colors
    pub const BACK_BLACK: &'static str = "\x1B[40m";
    pub const BACK_RED: &'static str = "\x1B[41m";
    pub const BACK_GREEN: &'static str = "\x1B[42m";
    pub const BACK_YELLOW: &'static str = "\x1B[43m";
    pub const BACK_BLUE: &'static str = "\x1B[44m";
    pub const BACK_PURPLE: &'static str = "\x1B[45m";
    pub const BACK_CYAN: &'static str = "\x1B[46m";
    pub const BACK_WHITE: &'static str = "\x1B[107m";
    pub const BACK_ORANGE: &'static str = "\x1B[48;5;208m";

    pub const FORE_BLACK: &'static str = "\x1B[30m";
    pub const FORE_RED: &'static str = "\x1B[31m";
    pub const FORE_GREEN: &'static str = "\x1B[32m";
    pub const FORE_YELLOW: &'static str = "\x1B[33m";
    pub const FORE_BLUE: &'static str = "\x1B[34m";
    pub const FORE_PURPLE: &'static str = "\x1B[35m";
    pub const FORE_CYAN: &'static str = "\x1B[36m";
    pub const FORE_WHITE: &'static str = "\x1B[97m";
    pub const FORE_ORANGE: &'static str = "\x1B[38;5;208m";

    // Styles
    pub const RESET: &'static str = "\x1B[0m";
    pub const BOLD: &'static str = "\x1B[1m";
    pub const DIM: &'static str = "\x1B[2m";
    pub const UNDERLINE: &'static str = "\x1B[4m";
    pub const BLINK: &'static str = "\x1B[5m";
    pub const REVERSE: &'static str = "\x1B[7m";
    pub const HIDDEN: &'static str = "\x1B[8m";
    pub const STRIKETHROUGH: &'static str = "\x1B[9m";
    pub const MAGICAL: &'static str = "\x1B[6m";
    pub const SLOW_BLINK: &'static str = "\x1B[25m";
    pub const FAST_BLINK: &'static str = "\x1B[6m";
    pub const ITALIC: &'static str = "\x1B[3m";
    pub const OVERLINE: &'static str = "\x1B[53m";
    pub const DOUBLE_UNDERLINE: &'static str = "\x1B[21m";
    pub const FRAMED: &'static str = "\x1B[51m";
    pub const ENCIRCLED: &'static str = "\x1B[52m";
    pub const OVERLINED: &'static str = "\x1B[55m";
    pub const BOLD_FAINT: &'static str = "\x1B[20m";
    pub const POSITIVE_IMG: &'static str = "\x1B[27m";

    // Cursor
    pub const UP: &'static str = "\x1B[A";
    pub const DOWN: &'static str = "\x1B[B";
    pub const FORWARD: &'static str = "\x1B[C";
    pub const BACKWARD: &'static str = "\x1B[D";
    pub const POS: &'static str = "\x1B[H";
    pub const CLEAR: &'static str = "\x1B[2J";
    pub const CLEAR_FORWARD: &'static str = "\x1B[0J";
    pub const CLEAR_BACKWARD: &'static str = "\x1B[1J";
    pub const CLEAR_LINE: &'static str = "\x1B[2K";
    pub const CLEAR_LINE_FORWARD: &'static str = "\x1B[0K";
    pub const CLEAR_LINE_BACKWARD: &'static str = "\x1B[1K";
    pub const SAVE_POS: &'static str = "\x1B[s";
    pub const RESTORE_POS: &'static str = "\x1B[u";
    pub const NEXT_LINE: &'static str = "\x1B[E";
    pub const PREV_LINE: &'static str = "\x1B[F";
}
