PSHARP_TOKENS = {
# Imports
    'import': 'IMPORT',
    'as': 'ASIMPORT',
# VARIABLES
    '=': 'ASSIGNMENT',
    'var': 'VARIABLE',
# Maths
    '**': 'EXPONENT',
    '%': 'MODULUS',
    '//': 'INTDIVISION',
    '/': 'DIVISION',
    '*': 'MULTIPLICATION',
    '-': 'SUBTRACTION',
    '+': 'ADDITION',
# Bools
    'true': 'TRUE',
    'false': 'FALSE',
# Logical Operands
    '==': 'EQUAL',
    '!=': 'NOTEQUAL',
    '<': 'LESS',
    '>': 'GREATER',
    '<=': 'LESSEQUAL',
    '>=': 'GREATEREQUAL',
    'is': 'EQUAL',
    'is not': 'NOTEQUAL',
    # Types
    'string': 'STRING',
    'int': 'INT',
    'float': 'FLOAT',
    'bool': 'BOOL',
    'List<>': 'LIST<TYPE>',
    'Tuple(type, type)': 'TUPLE<TYPE, TYPE>',
# Functions
    'func': 'FUNCTION',
# Conditionals
    'if': 'IF',
    'elif': 'ELSEIF',
    'else': 'ELSE',
# Iteration
    'for': 'ITERFOR',
    'while': 'ITERWHILE',
# COMMENTS
    '#': 'COMMENT',
    'error': 'ERROR'
}


class Token:
    def __init__(self, val):
        self.val = val
        self.type = self.GetTokenType()

    def __str__(self):
        return f'{self.val}:[{self.type}]'

    def __repr__(self):
        return f'{self.val}:[{self.type}]'

    def GetTokenType(self):
        try:
            val = self.val.lower()
            if val in PSHARP_TOKENS:
                return PSHARP_TOKENS[(val)]
            else:
                if val.isdigit():
                    return PSHARP_TOKENS[('int')]
                elif val.replace('.','',1).isdigit() and val.count('.') < 2:
                    return PSHARP_TOKENS[('float')]
                else:
                    return self.CheckValidStringFormat(val)
        except:
            print(f'{type(val)}{val}')
            return PSHARP_TOKENS['error']
        
    def CheckValidStringFormat(self, val):
        first = val[0]
        last = val[-1]
        if first == "'" or first == '"' and last == "'" and last == '"':
            return PSHARP_TOKENS[('string')]
        else:
            return PSHARP_TOKENS[('var')]
