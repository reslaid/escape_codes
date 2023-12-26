public static class AnsiEscapeCodes
{
    // Colors
    public const string BACK_BLACK = "\x1B[40m";
    public const string BACK_RED = "\x1B[41m";
    public const string BACK_GREEN = "\x1B[42m";
    public const string BACK_YELLOW = "\x1B[43m";
    public const string BACK_BLUE = "\x1B[44m";
    public const string BACK_PURPLE = "\x1B[45m";
    public const string BACK_CYAN = "\x1B[46m";
    public const string BACK_WHITE = "\x1B[107m";
    public const string BACK_ORANGE = "\x1B[48;5;208m";

    public const string FORE_BLACK = "\x1B[30m";
    public const string FORE_RED = "\x1B[31m";
    public const string FORE_GREEN = "\x1B[32m";
    public const string FORE_YELLOW = "\x1B[33m";
    public const string FORE_BLUE = "\x1B[34m";
    public const string FORE_PURPLE = "\x1B[35m";
    public const string FORE_CYAN = "\x1B[36m";
    public const string FORE_WHITE = "\x1B[97m";
    public const string FORE_ORANGE = "\x1B[38;5;208m";

    // Styles
    public const string RESET = "\x1B[0m";
    public const string BOLD = "\x1B[1m";
    public const string DIM = "\x1B[2m";
    public const string UNDERLINE = "\x1B[4m";
    public const string BLINK = "\x1B[5m";
    public const string REVERSE = "\x1B[7m";
    public const string HIDDEN = "\x1B[8m";
    public const string STRIKETHROUGH = "\x1B[9m";
    public const string MAGICAL = "\x1B[6m";
    public const string SLOW_BLINK = "\x1B[25m";
    public const string FAST_BLINK = "\x1B[6m";
    public const string ITALIC = "\x1B[3m";
    public const string OVERLINE = "\x1B[53m";
    public const string DOUBLE_UNDERLINE = "\x1B[21m";
    public const string FRAMED = "\x1B[51m";
    public const string ENCIRCLED = "\x1B[52m";
    public const string OVERLINED = "\x1B[55m";
    public const string BOLD_FAINT = "\x1B[20m";
    public const string POSITIVE_IMG = "\x1B[27m";

    // Cursor
    public const string UP = "\x1B[A";
    public const string DOWN = "\x1B[B";
    public const string FORWARD = "\x1B[C";
    public const string BACKWARD = "\x1B[D";
    public const string POS = "\x1B[H";
    public const string CLEAR = "\x1B[2J";
    public const string CLEAR_FORWARD = "\x1B[0J";
    public const string CLEAR_BACKWARD = "\x1B[1J";
    public const string CLEAR_LINE = "\x1B[2K";
    public const string CLEAR_LINE_FORWARD = "\x1B[0K";
    public const string CLEAR_LINE_BACKWARD = "\x1B[1K";
    public const string SAVE_POS = "\x1B[s";
    public const string RESTORE_POS = "\x1B[u";
    public const string NEXT_LINE = "\x1B[E";
    public const string PREV_LINE = "\x1B[F";
}
