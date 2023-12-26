// Colors
const colors = {
  backBlack: '\x1b[40m',
  backRed: '\x1b[41m',
  backGreen: '\x1b[42m',
  backYellow: '\x1b[43m',
  backBlue: '\x1b[44m',
  backPurple: '\x1b[45m',
  backCyan: '\x1b[46m',
  backWhite: '\x1b[107m',
  backOrange: '\x1b[48;5;208m',
  backDarkRed: '\x1b[101m',
  backDarkGreen: '\x1b[102m',
  backDarkYellow: '\x1b[103m',
  backDarkBlue: '\x1b[104m',
  backDarkPurple: '\x1b[105m',
  backDarkCyan: '\x1b[106m',
  backDarkGray: '\x1b[100m',
  backDarkOrange: '\x1b[48;5;202m',
  backLightRed: '\x1b[101m',
  backLightGreen: '\x1b[102m',
  backLightYellow: '\x1b[103m',
  backLightBlue: '\x1b[104m',
  backLightPurple: '\x1b[105m',
  backLightCyan: '\x1b[106m',

  foreBlack: '\x1b[30m',
  foreRed: '\x1b[31m',
  foreGreen: '\x1b[32m',
  foreYellow: '\x1b[33m',
  foreBlue: '\x1b[34m',
  forePurple: '\x1b[35m',
  foreCyan: '\x1b[36m',
  foreGray: '\x1b[37m',
  foreWhite: '\x1b[97m',
  foreOrange: '\x1b[38;5;208m',
  foreDarkRed: '\x1b[91m',
  foreDarkGreen: '\x1b[92m',
  foreDarkYellow: '\x1b[93m',
  foreDarkBlue: '\x1b[94m',
  foreDarkPurple: '\x1b[95m',
  foreDarkCyan: '\x1b[96m',
  foreDarkGray: '\x1b[90m',
  foreDarkOrange: '\x1b[38;5;202m',
  foreLightRed: '\x1b[91m',
  foreLightGreen: '\x1b[92m',
  foreLightYellow: '\x1b[93m',
  foreLightBlue: '\x1b[94m',
  foreLightPurple: '\x1b[95m',
  foreLightCyan: '\x1b[96m',
};

// Styles
const styles = {
  reset: '\x1b[0m',
  bold: '\x1b[1m',
  dim: '\x1b[2m',
  underline: '\x1b[4m',
  blink: '\x1b[5m',
  reverse: '\x1b[7m',
  hidden: '\x1b[8m',
  strikethrough: '\x1b[9m',
  magical: '\x1b[6m',
  slowBlink: '\x1b[25m',
  fastBlink: '\x1b[6m',
  italic: '\x1b[3m',
  overline: '\x1b[53m',
  doubleUnderline: '\x1b[21m',
  framed: '\x1b[51m',
  encircled: '\x1b[52m',
  overlined: '\x1b[55m',
  boldFaint: '\x1b[20m',
  positiveImg: '\x1b[27m',
};

// Cursor
const cursor = {
  up: '\x1b[A',
  down: '\x1b[B',
  forward: '\x1b[C',
  backward: '\x1b[D',
  pos: '\x1b[H',
  clear: '\x1b[2J',
  clearForward: '\x1b[0J',
  clearBackward: '\x1b[1J',
  clearLine: '\x1b[2K',
  clearLineForward: '\x1b[0K',
  clearLineBackward: '\x1b[1K',
  savePos: '\x1b[s',
  restorePos: '\x1b[u',
};

module.exports = { colors, styles, cursor };