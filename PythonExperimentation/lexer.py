from token import *


class Lexer:
    def __init__(self, mode, data=None):
        self.mode = mode
        if mode == 'file':
            self.tokens = self.GetFileData(data)
        elif mode == 'interpret':
            self.tokens = self.GetLineTokens(data)

    def __str__(self):
        return f'{self.tokens}'

    def __repr__(self):
        return f'{self.tokens}'

    def GetFileTokens(self, data):
        return NotImplemented

    def GetLineTokens(self, data):
        tokens = []
        data = data.split()
        for word in data:
            tokens.append(Token(word))
        return tokens


if __name__ == '__main__':
    while True:
        data = input('P# > ')
        lex = Lexer('interpret', data)
        print(lex)
