PSHARP_TOKENS = {
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
    #
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
            return PSHARP_TOKENS[(self.val.lower())]
        except:
            return PSHARP_TOKENS['error']

