from ipynb.fs.full.psharp import *

while True:
    text = input('psharp > ')
    psharp = PSharp(text)
    result = psharp.run()
    
    print(result)