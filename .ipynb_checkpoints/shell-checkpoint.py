import psharp

while True:
    text = input('psharp > ')
    result, error = psharp.run('<stdin>', text)

    if error: print(error.as_string())
    else: print(result)