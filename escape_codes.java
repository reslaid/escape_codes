public class AnsiEscapeCodes {
    // Colors
    public static final String BACK_BLACK = "\u001B[40m";
    public static final String BACK_RED = "\u001B[41m";
    public static final String BACK_GREEN = "\u001B[42m";
    public static final String BACK_YELLOW = "\u001B[43m";
    public static final String BACK_BLUE = "\u001B[44m";
    public static final String BACK_PURPLE = "\u001B[45m";
    public static final String BACK_CYAN = "\u001B[46m";
    public static final String BACK_WHITE = "\u001B[107m";
    public static final String BACK_ORANGE = "\u001B[48;5;208m";

    public static final String FORE_BLACK = "\u001B[30m";
    public static final String FORE_RED = "\u001B[31m";
    public static final String FORE_GREEN = "\u001B[32m";
    public static final String FORE_YELLOW = "\u001B[33m";
    public static final String FORE_BLUE = "\u001B[34m";
    public static final String FORE_PURPLE = "\u001B[35m";
    public static final String FORE_CYAN = "\u001B[36m";
    public static final String FORE_WHITE = "\u001B[97m";
    public static final String FORE_ORANGE = "\u001B[38;5;208m";

    // Styles
    public static final String RESET = "\u001B[0m";
    public static final String BOLD = "\u001B[1m";
    public static final String DIM = "\u001B[2m";
    public static final String UNDERLINE = "\u001B[4m";
    public static final String BLINK = "\u001B[5m";
    public static final String REVERSE = "\u001B[7m";
    public static final String HIDDEN = "\u001B[8m";
    public static final String STRIKETHROUGH = "\u001B[9m";
    public static final String MAGICAL = "\u001B[6m";
    public static final String SLOW_BLINK = "\u001B[25m";
    public static final String FAST_BLINK = "\u001B[6m";
    public static final String ITALIC = "\u001B[3m";
    public static final String OVERLINE = "\u001B[53m";
    public static final String DOUBLE_UNDERLINE = "\u001B[21m";
    public static final String FRAMED = "\u001B[51m";
    public static final String ENCIRCLED = "\u001B[52m";
    public static final String OVERLINED = "\u001B[55m";
    public static final String BOLD_FAINT = "\u001B[20m";
    public static final String POSITIVE_IMG = "\u001B[27m";

    // Cursor
    public static final String UP = "\u001B[A";
    public static final String DOWN = "\u001B[B";
    public static final String FORWARD = "\u001B[C";
    public static final String BACKWARD = "\u001B[D";
    public static final String POS = "\u001B[H";
    public static final String CLEAR = "\u001B[2J";
    public static final String CLEAR_FORWARD = "\u001B[0J";
    public static final String CLEAR_BACKWARD = "\u001B[1J";
    public static final String CLEAR_LINE = "\u001B[2K";
    public static final String CLEAR_LINE_FORWARD = "\u001B[0K";
    public static final String CLEAR_LINE_BACKWARD = "\u001B[1K";
    public static final String SAVE_POS = "\u001B[s";
    public static final String RESTORE_POS = "\u001B[u";
    public static final String NEXT_LINE = "\u001B[E";
    public static final String PREV_LINE = "\u001B[F";
}
