import lodestone

height = 1.0
width = 1.0
center = (0.0, -width / 2.0)
alpha = 0.0
jr = 1.0
phi = 0.0
point = (width / 2, height * 1.01 / 2)

field = lodestone.rectangle_field(width, height, center, alpha, jr, phi, point)
print(field.x, field.y)
