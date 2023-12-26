#ifndef ANSI_COLORS_HPP
#define ANSI_COLORS_HPP

#include <string>

namespace ANSI {

struct Colors {
  struct BackColors {
    static const std::string back_black;
    static const std::string back_red;
    static const std::string back_green;
    static const std::string back_yellow;
    static const std::string back_blue;
    static const std::string back_purple;
    static const std::string back_cyan;
    static const std::string back_white;
    static const std::string back_orange;
    static const std::string back_dark_red;
    static const std::string back_dark_green;
    static const std::string back_dark_yellow;
    static const std::string back_dark_blue;
    static const std::string back_dark_purple;
    static const std::string back_dark_cyan;
    static const std::string back_dark_gray;
    static const std::string back_dark_orange;
    static const std::string back_light_red;
    static const std::string back_light_green;
    static const std::string back_light_yellow;
    static const std::string back_light_blue;
    static const std::string back_light_purple;
    static const std::string back_light_cyan;
  };

  struct ForeColors {
    static const std::string fore_black;
    static const std::string fore_red;
    static const std::string fore_green;
    static const std::string fore_yellow;
    static const std::string fore_blue;
    static const std::string fore_purple;
    static const std::string fore_cyan;
    static const std::string fore_gray;
    static const std::string fore_white;
    static const std::string fore_orange;
    static const std::string fore_dark_red;
    static const std::string fore_dark_green;
    static const std::string fore_dark_yellow;
    static const std::string fore_dark_blue;
    static const std::string fore_dark_purple;
    static const std::string fore_dark_cyan;
    static const std::string fore_dark_gray;
    static const std::string fore_dark_orange;
    static const std::string fore_light_red;
    static const std::string fore_light_green;
    static const std::string fore_light_yellow;
    static const std::string fore_light_blue;
    static const std::string fore_light_purple;
    static const std::string fore_light_cyan;
  };

  struct Styles {
    static const std::string reset;
    static const std::string bold;
    static const std::string dim;
    static const std::string underline;
    static const std::string blink;
    static const std::string reverse;
    static const std::string hidden;
    static const std::string strikethrough;
    static const std::string magical;
    static const std::string slowblink;
    static const std::string fastblink;
    static const std::string italic;
    static const std::string overline;
    static const std::string doubleunderline;
    static const std::string framed;
    static const std::string encircled;
    static const std::string overlined;
    static const std::string boldfaint;
    static const std::string positiveimg;
  };

  struct Cursor {
    static const std::string up;
    static const std::string down;
    static const std::string forward;
    static const std::string backward;
    static const std::string pos;
    static const std::string clear;
    static const std::string clear_forward;
    static const std::string clear_backward;
    static const std::string clear_line;
    static const std::string clear_line_forward;
    static const std::string clear_line_backward;
    static const std::string savepos;
    static const std::string restorepos;
    static const std::string next_line;
    static const std::string prev_line;
  };
};

} // namespace ANSI

#endif // ANSI_COLORS_HPP
