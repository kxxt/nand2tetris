import re
import sys

lines = sys.stdin.readlines()[1:-1]

regex = re.compile(r'<(\w+)>([^<]+)</\w+>')

TAG_MAP = {
    'keyword': 'Keyword',
    'symbol': 'Symbol',
    'identifier': 'Identifier',
    'integerConstant': 'IntegerConstant',
    'stringConstant': 'StringConstant',
}

print("vec![")

for line in lines:
    match = regex.match(line)
    tag = match.group(1)
    # DO NOT use strip here.
    # The provided xml is whitespace sensitive.
    value = match.group(2)[1:-1]
    value = re.sub('&amp;', '&', value)
    value = re.sub('&lt;', '<', value)
    value = re.sub('&gt;', '>', value)
    print(f'token!({TAG_MAP[tag]}, "{value}"),')
print("]")
