pub fn bold() -> &'static str {
    "\x1b[1m"
}
pub fn dim() -> &'static str {
    "\x1b[2m"
}
pub fn italic() -> &'static str {
    "\x1b[3m"
}
pub fn underline() -> &'static str {
    "\x1b[4m"
}
pub fn inverse() -> &'static str {
    "\x1b[7m"
}
pub fn strikethrough() -> &'static str {
    "\x1b[9m"
}

pub fn red() -> &'static str {
    "\x1b[31m"
}

pub fn green() -> &'static str {
    "\x1b[32m"
}

pub fn yellow() -> &'static str {
    "\x1b[33m"
}

pub fn blue() -> &'static str {
    "\x1b[34m"
}

pub fn magenta() -> &'static str {
    "\x1b[35m"
}

pub fn cyan() -> &'static str {
    "\x1b[36m"
}

pub fn default() -> &'static str {
    "\x1b[39m"
}

pub fn reset() -> &'static str {
    "\x1b[0m"
}
