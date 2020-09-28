import psharp

while True:
    text = input('basic > ')
    result, error = psharp.run('<stdin>', text)

    if error: print(error.as_string())
    else: print(result)