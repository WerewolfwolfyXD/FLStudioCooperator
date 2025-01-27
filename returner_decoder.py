import base64
import sys

encoded_data = open(sys.argv[1], "r").read()

encoded_data = base64.b64decode(encoded_data)

decoded_str = encoded_data.decode("utf-8")

p = open(f"{sys.argv[1]}.decoded.txt", "w+", encoding="utf-8")
p.write(decoded_str)
p.flush()
p.close()

print(f"{decoded_str}")

input()
