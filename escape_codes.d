module AnsiEscapeCodes;

// Colors
enum BACK_BLACK = "\x1B[40m";
enum BACK_RED = "\x1B[41m";
enum BACK_GREEN = "\x1B[42m";
enum BACK_YELLOW = "\x1B[43m";
enum BACK_BLUE = "\x1B[44m";
enum BACK_PURPLE = "\x1B[45m";
enum BACK_CYAN = "\x1B[46m";
enum BACK_WHITE = "\x1B[107m";
enum BACK_ORANGE = "\x1B[48;5;208m";

enum FORE_BLACK = "\x1B[30m";
enum FORE_RED = "\x1B[31m";
enum FORE_GREEN = "\x1B[32m";
enum FORE_YELLOW = "\x1B[33m";
enum FORE_BLUE = "\x1B[34m";
enum FORE_PURPLE = "\x1B[35m";
enum FORE_CYAN = "\x1B[36m";
enum FORE_WHITE = "\x1B[97m";
enum FORE_ORANGE = "\x1B[38;5;208m";

// Styles
enum RESET = "\x1B[0m";
enum BOLD = "\x1B[1m";
enum DIM = "\x1B[2m";
enum UNDERLINE = "\x1B[4m";
enum BLINK = "\x1B[5m";
enum REVERSE = "\x1B[7m";
enum HIDDEN = "\x1B[8m";
enum STRIKETHROUGH = "\x1B[9m";
enum MAGICAL = "\x1B[6m";
enum SLOW_BLINK = "\x1B[25m";
enum FAST_BLINK = "\x1B[6m";
enum ITALIC = "\x1B[3m";
enum OVERLINE = "\x1B[53m";
enum DOUBLE_UNDERLINE = "\x1B[21m";
enum FRAMED = "\x1B[51m";
enum ENCIRCLED = "\x1B[52m";
enum OVERLINED = "\x1B[55m";
enum BOLD_FAINT = "\x1B[20m";
enum POSITIVE_IMG = "\x1B[27m";

// Cursor
enum UP = "\x1B[A";
enum DOWN = "\x1B[B";
enum FORWARD = "\x1B[C";
enum BACKWARD = "\x1B[D";
enum POS = "\x1B[H";
enum CLEAR = "\x1B[2J";
enum CLEAR_FORWARD = "\x1B[0J";
enum CLEAR_BACKWARD = "\x1B[1J";
enum CLEAR_LINE = "\x1B[2K";
enum CLEAR_LINE_FORWARD = "\x1B[0K";
enum CLEAR_LINE_BACKWARD = "\x1B[1K";
enum SAVE_POS = "\x1B[s";
enum RESTORE_POS = "\x1B[u";
enum NEXT_LINE = "\x1B[E";
enum PREV_LINE = "\x1B[F";
