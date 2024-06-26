import numpy as np

a = np.array([0, 30, 60, 90])

print("angle", a)
print("sine(a)", np.sinc(a * np.pi / 180))

a = np.arange(9, dtype=np.float_).reshape(3,3)
b = np.array([10, 10, 10])

print("a: ", a)
print("b: ", b)
print("a * 2: ", a * b)
print("a + 2: ", a + b)
print("a / 2: ", a / b)
print("average(a)", np.average(a))
print("mean:(b)", np.mean(b))
