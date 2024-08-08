import re
import pprint

code = """
const OR: &'a str = "||";
const AND: &'a str = "&&";
const EQ: &'a str = "==";
const NE: &'a str = "!=";
const LT: &'a str = "<";
const LE: &'a str = "<=";
const GT: &'a str = ">";
const GE: &'a str = ">=";
const ADD: &'a str = "+";
const SUB: &'a str = "-";
const MUL: &'a str = "*";
const DIV: &'a str = "/";
const MOD: &'a str = "%";
const DOT: &'a str = "@";

const ASSIGNMENT: &'a str = "=";
const ADDEQ: &'a str = "+=";
const SUBEQ: &'a str = "-=";
const MULEQ: &'a str = "*=";
const DIVEQ: &'a str = "/=";
const MODEQ: &'a str = "%=";

const POW: &'a str = "**";
const NOT: &'a str = "!";
"""

regex = re.compile(r"const (.*): &'a str = \"(.*)\";")
mo = regex.findall(code)

# pprint.pprint(mo)
# pprint.pprint(sorted(mo, key=lambda a: len(a[1]), reverse=True))
print(f"const LENGTH_ORDER_OPE_LIST:[&'a str;{len(mo)}] = [")
for i, j in sorted(mo, key=lambda a: len(a[1]), reverse=True):
    # print(i, j)
    print(f"Self::{i}, // {j}")

print("];")
